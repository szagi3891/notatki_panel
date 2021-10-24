#![allow(clippy::needless_lifetimes)]
#![allow(clippy::ptr_arg)]

use common::{
    RootResponse,
    HandlerCreateDirBody,
    HandlerCreateFileBody,
    HandlerDeleteItemBody,
    HandlerFetchDirBody,
    HandlerFetchDirResponse,
    HandlerFetchNodeBody,
    HandlerFetchNodeResponse,
    HandlerRenameItemBody,
    HandlerSaveContentBody,
};
use utils::{ErrorProcess, create_response, create_response_message};
use warp::{Filter, Reply, http::Response};
use std::convert::Infallible;
use std::net::Ipv4Addr;
use serde::Deserialize;
use std::sync::Arc;

mod git;
mod utils;
mod sync;

// use sync::start_sync;

use git::Git;

#[derive(Deserialize)]
struct Config {
    http_host: Ipv4Addr,
    http_port: u16,
    git_repo: String,
}

struct AppState {
    git: Git,
}

impl AppState {
    pub fn new(git: Git) -> Arc<AppState> {
        Arc::new(AppState {
            git
        })
    }
}

fn inject_state<T: Clone + Sized + Send>(state: T) -> impl Filter<Extract = (T,), Error = Infallible> + Clone {
    warp::any().map(move || state.clone())
}

fn response_with_root(result: Result<String, ErrorProcess>) -> Result<impl warp::Reply, Infallible> {
    match result {
        Ok(root) => {
            let response = RootResponse {
                root,
            };
            Ok(create_response(200, response))
        },
        Err(err) => {
            Ok(err.to_response())
        }
    }
}

async fn handler_index() -> Result<impl Reply, Infallible> {
    Ok(Response::new(r##"<!DOCTYPE html>
    <html>
        <head>
            <meta charset="utf-8"/>
            <script type="module">
                import init from "/build/app.js";
                init("/build/app_bg.wasm");
            </script>
        </head>
        <body></body>
    </html>
"##))
}

async fn handler_fetch_root(app_state: Arc<AppState>) -> Result<impl warp::Reply, Infallible> {
    let result = app_state.git.main_commit().await;
    response_with_root(result)
}

async fn handler_fetch_dir(app_state: Arc<AppState>, body_request: HandlerFetchDirBody) -> Result<Response<String>, Infallible> {
    let root = app_state.git.get_from_id(&body_request.id).await;

    let root = match root {
        Ok(root) => root,
        Err(message) => {
            return Ok(message.to_response());
        }
    };

    if let Some(git::GitBlob::Tree { list }) = root {
        let mut response: HandlerFetchDirResponse = HandlerFetchDirResponse::new();
        for item in list {
            response.add(item);
        }

        return Ok(create_response(200, response));
    }

    Ok(create_response_message(404, "missing"))
}

async fn handler_fetch_node(app_state: Arc<AppState>, body_request: HandlerFetchNodeBody) -> Result<impl warp::Reply, Infallible> {

    let hash_id = body_request.hash;

    let data = app_state.git.get_from_id(&hash_id).await;

    let data = match data {
        Ok(data) => data,
        Err(message) => {
            return Ok(message.to_response());
        }
    };

    if let Some(git::GitBlob::Blob { content }) = data {
        let content = String::from_utf8(content);

        return match content {
            Ok(content) => {
                let response = HandlerFetchNodeResponse {
                    content
                };

                Ok(create_response(200, response))
            },
            Err(_) => {
                Ok(create_response_message(500, "content is not correctly encoded in utf8"))
            }
        };
    }

    Ok(create_response_message(404, format!("missing content {}", hash_id)))
}

async fn handler_save_content(app_state: Arc<AppState>, body_request: HandlerSaveContentBody) -> Result<impl warp::Reply, Infallible> {

    let result = app_state.git.save_content(
        body_request.path,
        body_request.prev_hash,
        body_request.new_content
    ).await;

    response_with_root(result)
}

async fn handler_create_file(app_state: Arc<AppState>, body_request: HandlerCreateFileBody) -> Result<impl warp::Reply, Infallible> {
    let result = app_state.git.create_file(
        body_request.path,
        body_request.new_name,
        body_request.new_content
    ).await;

    response_with_root(result)
}

async fn handler_create_dir(app_state: Arc<AppState>, body_request: HandlerCreateDirBody) -> Result<impl warp::Reply, Infallible> {
    let result = app_state.git.create_dir(
        body_request.path,
        body_request.dir
    ).await;

    response_with_root(result)
}

async fn handler_rename_item(app_state: Arc<AppState>, body_request: HandlerRenameItemBody) -> Result<impl warp::Reply, Infallible> {
    let result = app_state.git.rename_item(
        body_request.path,
        body_request.prev_name,
        body_request.prev_hash,
        body_request.new_name
    ).await;

    response_with_root(result)
}

async fn handler_delete_item(app_state: Arc<AppState>, body_request: HandlerDeleteItemBody) -> Result<impl warp::Reply, Infallible> {
    let result = app_state.git.delete_item(
        body_request.path,
        body_request.hash,
    ).await;

    response_with_root(result)
}


#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let config = match envy::from_env::<Config>() {
        Ok(config) => config,
        Err(error) => panic!("Service started with invalid environment variables {:#?}", error)
    };

    println!("start git test: {}", &config.git_repo);
    let git = Git::new(config.git_repo, "master".into()).unwrap();


    let app_state = AppState::new(git.clone());

    //chwilowo wyłączamy synchronizację
    // let task_synchronize = start_sync(config.git_repo.clone()).await;

    let route_index =
        warp::path::end()
        .and_then(handler_index);

    let route_build = {
        use warp::http::header::{HeaderMap, HeaderValue};
        let mut headers = HeaderMap::new();

        //cache-control: private, no-cache, no-store, must-revalidate, max-age=0
        headers.insert(
            "cache-control", 
            HeaderValue::from_static("private, no-cache, no-store, must-revalidate, max-age=0")
        );

        warp::path("build")
            .and(warp::fs::dir("build"))
            .with(warp::reply::with::headers(headers))
    };

    let filter_fetch_root =
        warp::path!("fetch_root")
        .and(warp::get())
        .and(inject_state(app_state.clone()))
        .and_then(handler_fetch_root);


    let filter_fetch_dir =
        warp::path!("fetch_tree_item")
        .and(warp::post())
        .and(inject_state(app_state.clone()))
        .and(warp::body::json())
        .and_then(handler_fetch_dir);


    let filter_fetch_node =
        warp::path!("fetch_node")
        .and(warp::post())
        .and(inject_state(app_state.clone()))
        .and(warp::body::json())
        .and_then(handler_fetch_node);

    let filter_save_content =
        warp::path!("save_content")
        .and(warp::post())
        .and(inject_state(app_state.clone()))
        .and(warp::body::json())
        .and_then(handler_save_content);

    let filter_create_file =
        warp::path!("create_file")
        .and(warp::post())
        .and(inject_state(app_state.clone()))
        .and(warp::body::json())
        .and_then(handler_create_file);
    
    let filter_create_dir =
        warp::path!("create_dir")
        .and(warp::post())
        .and(inject_state(app_state.clone()))
        .and(warp::body::json())
        .and_then(handler_create_dir);
    
    let filter_rename_item =
        warp::path!("rename_item")
        .and(warp::post())
        .and(inject_state(app_state.clone()))
        .and(warp::body::json())
        .and_then(handler_rename_item);

    let filter_delete_item = 
        warp::path!("delete_item")
        .and(warp::post())
        .and(inject_state(app_state.clone()))
        .and(warp::body::json())
        .and_then(handler_delete_item);

    let routes_default =
        warp::any()
        .map(|| {
            warp::http::Response::builder()
                .status(404)
                .body("error 404")
        });

    let routes =
        route_index
        .or(route_build)
        .or(filter_fetch_root)
        .or(filter_fetch_dir)
        .or(filter_fetch_node)
        .or(filter_save_content)
        .or(filter_create_file)
        .or(filter_create_dir)
        .or(filter_rename_item)
        .or(filter_delete_item)
        .or(routes_default)
    ;

    log::info!("Start - {}:{}", config.http_host, config.http_port);

    let logger = warp::filters::log::custom(|info| {
        log::info!(
            "Request {} {} {} - elapsed: {:?}",
            info.status(),
            info.method(),
            info.path(),
            info.elapsed()
        );
    });

    warp::serve(routes.with(logger))
        .run((config.http_host, config.http_port))
        .await;

    // task_synchronize.off();
}
