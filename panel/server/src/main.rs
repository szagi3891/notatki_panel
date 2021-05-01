use common::{HandlerFetchDirBody, HandlerFetchDirResponse, HandlerFetchNodeResponse, HandlerFetchRootResponse};
use utils::{create_response, create_response_message};
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
    let root = app_state.git.get_main_commit().await;

    let body = HandlerFetchRootResponse {
        root,
    };

    Ok(create_response(200, body))
}

async fn handler_fetch_dir(app_state: Arc<AppState>, body_request: HandlerFetchDirBody) -> Result<Response<String>, Infallible> {
    let root = app_state.git.get_from_id(&body_request.id).await;

    if let Some(git::GitBlob::Tree { list }) = root {
        let mut response: HandlerFetchDirResponse = HandlerFetchDirResponse::new();
        for item in list {
            response.add(item);
        }

        return Ok(create_response(200, response));
    }

    Ok(create_response_message(404, "missing"))
}

async fn handler_fetch_node(hash_id: String, app_state: Arc<AppState>) -> Result<impl warp::Reply, Infallible> {

    let data = app_state.git.get_from_id(&hash_id).await;

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


// async fn handler_create_dir(app_state: Arc<AppState>, body: PostParamsCreateDir) -> Result<impl warp::Reply, Infallible> {
//     let new_id = app_state.git.create_dir(body.parent_node, body.name).await;

//     match new_id {
//         Ok(new_id) => {
//             let message = serde_json::to_string(&new_id).unwrap();

//             let response = Response::builder()
//                 .status(200)
//                 .body(message);
//             Ok(response)
//         },
//         Err(err) => {
//             let response = Response::builder()
//                 .status(500)
//                 .body(format!("error dir create = {:?}", err));

//             Ok(response)
//         }
//     }
// }

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let config = match envy::from_env::<Config>() {
        Ok(config) => config,
        Err(error) => panic!("Service started with invalid environment variables {:#?}", error)
    };

    println!("start git test: {}", &config.git_repo);
    let git = Git::new(config.git_repo, "master".into());

    let app_state = AppState::new(git.clone());

    //chwilowo wyłączamy synchronizację
    // let task_synchronize = start_sync(config.git_repo.clone()).await;

    let route_index =
        warp::path::end()
        .and_then(handler_index);

    let route_build =
        warp::path("build")
        .and(warp::fs::dir("build"));

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
        warp::path!("fetch_node" / String)
        .and(warp::get())
        .and(inject_state(app_state.clone()))
        .and_then(handler_fetch_node);

    // let filter_create_dir =
    //     warp::path!("create_dir")
    //     .and(warp::post())
    //     .and(inject_state(app_state.clone()))
    //     .and(warp::body::json())
    //     .and_then(handler_create_dir);
    
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
        // .or(filter_create_dir)
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
