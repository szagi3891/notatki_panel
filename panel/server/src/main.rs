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
use std::net::Ipv4Addr;
use std::sync::Arc;
use axum::{
    http::StatusCode,
    response::Html,
    handler::Handler,
    routing::{get, post},
    extract::Extension,
    Json, Router,
    AddExtensionLayer,
    routing::get_service,
};
use serde::{Deserialize};
use std::net::SocketAddr;
use tower_http::{services::ServeDir, trace::TraceLayer};


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

fn response_with_root(result: Result<String, ErrorProcess>) -> (StatusCode, String) { //Box<dyn IntoResponse> {
    match result {
        Ok(root) => {
            let response = RootResponse {
                root,
            };
            create_response(StatusCode::OK, response)
        },
        Err(err) => {
            err.to_response()
        }
    }
}

async fn handler_index() -> Html<&'static str> {
    Html(
        r##"
            <!DOCTYPE html>
            <html>
                <head>
                    <meta charset="utf-8"/>
                    <style type="text/css">
                        * {
                            box-sizing: border-box;
                        }
                    </style>
                    <script type="module">
                        import { runModule } from "./build/wasm_run.js";
                        runModule("./build/client.wasm");
                    </script>
                </head>
                <body></body>
            </html>
        "##
    )
}

async fn handler_fetch_root(
    Extension(app_state): Extension<Arc<AppState>>,
) -> (StatusCode, String) {
    let result = app_state.git.main_commit().await;
    response_with_root(result)
}

async fn handler_fetch_dir(
    Extension(app_state): Extension<Arc<AppState>>,
    Json(body_request): Json<HandlerFetchDirBody>,
) -> (StatusCode, String) {
    let root = app_state.git.get_from_id(&body_request.id).await;

    let root = match root {
        Ok(root) => root,
        Err(message) => {
            return message.to_response();
        }
    };

    if let Some(git::GitBlob::Tree { list }) = root {
        let mut response: HandlerFetchDirResponse = HandlerFetchDirResponse::new();
        for item in list {
            response.add(item);
        }

        return create_response(StatusCode::OK, response);
    }

    create_response_message(StatusCode::NOT_FOUND, "missing")
}

async fn handler_fetch_node(
    Extension(app_state): Extension<Arc<AppState>>,
    Json(body_request): Json<HandlerFetchNodeBody>,
) -> (StatusCode, String) {

    let hash_id = body_request.hash;

    let data = app_state.git.get_from_id(&hash_id).await;

    let data = match data {
        Ok(data) => data,
        Err(message) => {
            return message.to_response();
        }
    };

    if let Some(git::GitBlob::Blob { content }) = data {
        let content = String::from_utf8(content);

        return match content {
            Ok(content) => {
                let response = HandlerFetchNodeResponse {
                    content
                };

                create_response(StatusCode::OK, response)
            },
            Err(_) => {
                create_response_message(StatusCode::INTERNAL_SERVER_ERROR, "content is not correctly encoded in utf8")
            }
        };
    }

    create_response_message(StatusCode::NOT_FOUND, format!("missing content {}", hash_id))
}

async fn handler_save_content(
    Extension(app_state): Extension<Arc<AppState>>,
    Json(body_request): Json<HandlerSaveContentBody>,
) -> (StatusCode, String) {

    let result = app_state.git.save_content(
        body_request.path,
        body_request.prev_hash,
        body_request.new_content
    ).await;

    response_with_root(result)
}

async fn handler_create_file(
    Extension(app_state): Extension<Arc<AppState>>,
    Json(body_request): Json<HandlerCreateFileBody>,
) -> (StatusCode, String) {
    let result = app_state.git.create_file(
        body_request.path,
        body_request.new_name,
        body_request.new_content
    ).await;

    response_with_root(result)
}

async fn handler_create_dir(
    Extension(app_state): Extension<Arc<AppState>>,
    Json(body_request): Json<HandlerCreateDirBody>,
) -> (StatusCode, String) {
    let result = app_state.git.create_dir(
        body_request.path,
        body_request.dir
    ).await;

    response_with_root(result)
}

async fn handler_rename_item(
    Extension(app_state): Extension<Arc<AppState>>,
    Json(body_request): Json<HandlerRenameItemBody>,
) -> (StatusCode, String) {
    let result = app_state.git.rename_item(
        body_request.path,
        body_request.prev_name,
        body_request.prev_hash,
        body_request.new_name
    ).await;

    response_with_root(result)
}

async fn handler_delete_item(
    Extension(app_state): Extension<Arc<AppState>>,
    Json(body_request): Json<HandlerDeleteItemBody>,
) -> (StatusCode, String) {
    let result = app_state.git.delete_item(
        body_request.path,
        body_request.hash,
    ).await;

    response_with_root(result)
}

async fn error404() -> (StatusCode, String) {
    (StatusCode::NOT_FOUND, "aa".into())
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

    log::info!("Start - {}:{}", config.http_host, config.http_port);

    //TODO - do builda trzeba dodać jakoś te nagłówki przeciw keszowaniu
    // //cache-control: private, no-cache, no-store, must-revalidate, max-age=0
    // headers.insert(
    //     "cache-control", 
    //     HeaderValue::from_static("private, no-cache, no-store, must-revalidate, max-age=0")
    // );

    let app = Router::new()
        .route("/", get(handler_index))
        .nest(
            "/build",
            get_service(ServeDir::new("build")).handle_error(|error: std::io::Error| async move {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Unhandled internal error: {}", error),
                )
            }),
        )

        .route("/fetch_root", get(handler_fetch_root))
        .route("/fetch_tree_item", post(handler_fetch_dir))
        .route("/fetch_node", post(handler_fetch_node))
        .route("/save_content", post(handler_save_content))
        .route("/create_file", post(handler_create_file))
        .route("/create_dir", post(handler_create_dir))
        .route("/rename_item", post(handler_rename_item))
        .route("/delete_item", post(handler_delete_item))
        .fallback(error404.into_service())
        .layer(AddExtensionLayer::new(app_state))
        .layer(TraceLayer::new_for_http())
    ;

    let addr = SocketAddr::from(([0, 0, 0, 0], config.http_port));
    
    //TODO ....
    //.run((config.http_host, config.http_port))

    //tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    // task_synchronize.off();
}
