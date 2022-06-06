#![allow(clippy::needless_lifetimes)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::module_inception)]
#![allow(clippy::wrong_self_convention)]
#![allow(clippy::len_zero)]
mod models;

use models::{
    RootResponse,
    HandlerCreateDirBody,
    HandlerCreateFileBody,
    HandlerDeleteItemBody,
    HandlerFetchDirBody,
    HandlerFetchDirResponse,
    HandlerFetchNodeBody,
    HandlerFetchNodeResponse,
    HandlerRenameItemBody,
    HandlerSaveContentBody, HandlerMoveItemBody,
};
use poem::{
    Body,
    endpoint::StaticFilesEndpoint,
    listener::TcpListener,
    Server,
    Route
};
use poem_openapi::{
    OpenApiService,
    OpenApi,
    param::Path,
    payload::{
        // PlainText,
        Html, PlainText,
        Binary
    }
};
use utils::{
    ErrorProcess, ApiResponseHttp,
};
use poem_openapi::response::StaticFileResponse;

use std::net::Ipv4Addr;
use serde::{Deserialize};
use std::net::SocketAddr;
use poem_openapi::payload::Json;


mod git;
mod utils;
// mod sync;

// use sync::start_sync;

use git::{Git, GitBlob};

#[derive(Deserialize)]
struct Config {
    http_host: Ipv4Addr,
    http_port: u16,
    git_repo: String,
}

#[derive(Clone)]
struct App {
    git: Git,
}

#[OpenApi]
impl App {
    pub fn new(git: Git) -> App {
        App {
            git
        }
    }

    #[oai(method = "get", path = "/")]
    async fn handler_index(&self) -> Html<&str> {
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

    #[oai(method = "get", path = "/fetch_root")]
    async fn handler_fetch_root(&self) -> ApiResponseHttp<RootResponse> {
        let result = self.git.main_commit().await;
        response_with_root(result)
    }

    #[oai(method = "post", path = "/fetch_tree_item")]
    async fn handler_fetch_dir(&self, json: Json<HandlerFetchDirBody>) -> ApiResponseHttp<HandlerFetchDirResponse> {
        let Json(body_request) = json;

        let root = self.git.get_from_id(&body_request.id).await;

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

            return ApiResponseHttp::ok(response);
        }

        ApiResponseHttp::not_found("missing")
    }

    #[oai(method = "post", path = "/fetch_node")]
    async fn handler_fetch_node(&self, json: Json<HandlerFetchNodeBody>) -> ApiResponseHttp<HandlerFetchNodeResponse> {
        let Json(body_request) = json;

        let hash_id = body_request.hash;

        let data = self.git.get_from_id(&hash_id).await;

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

                    ApiResponseHttp::ok(response)
                },
                Err(_) => {
                    ApiResponseHttp::internal("content is not correctly encoded in utf8")
                }
            };
        }

        ApiResponseHttp::not_found(format!("missing content {}", hash_id))
    }

    #[oai(method = "post", path = "/save_content")]
    async fn handler_save_content(&self, json: Json<HandlerSaveContentBody>) -> ApiResponseHttp<RootResponse> {
        let Json(body_request) = json;
        let result = self.git.save_content(
            body_request.path,
            body_request.prev_hash,
            body_request.new_content
        ).await;

        response_with_root(result)
    }

    #[oai(method = "post", path = "/create_file")]
    async fn handler_create_file(&self, json: Json<HandlerCreateFileBody>) -> ApiResponseHttp<RootResponse> {
        let Json(body_request) = json;
        let result = self.git.create_file(
            body_request.path,
            body_request.new_name,
            body_request.new_content
        ).await;

        response_with_root(result)
    }

    #[oai(method = "post", path = "/create_dir")]
    async fn handler_create_dir(&self, json: Json<HandlerCreateDirBody>) -> ApiResponseHttp<RootResponse> {
        let Json(body_request) = json;
        let result = self.git.create_dir(
            body_request.path,
            body_request.dir
        ).await;

        response_with_root(result)
    }

    #[oai(method = "post", path = "/rename_item")]
    async fn handler_rename_item(&self, json: Json<HandlerRenameItemBody>) -> ApiResponseHttp<RootResponse> {
        let Json(body_request) = json;
        let result = self.git.rename_item(
            body_request.path,
            body_request.prev_name,
            body_request.prev_hash,
            body_request.new_name
        ).await;

        response_with_root(result)
    }

    #[oai(method = "post", path = "/delete_item")]
    async fn handler_delete_item(&self, json: Json<HandlerDeleteItemBody>) -> ApiResponseHttp<RootResponse> {
        let Json(body_request) = json;
        let result = self.git.delete_item(
            body_request.path,
            body_request.hash,
        ).await;

        response_with_root(result)
    }

    #[oai(method = "post", path = "/move_item")]
    async fn handler_move_item(&self, json: Json<HandlerMoveItemBody>) -> ApiResponseHttp<RootResponse> {
        let Json(body_request) = json;
        
        let result = self.git.move_item(
            body_request.path,
            body_request.hash,
            body_request.new_path,
        ).await;

        response_with_root(result)
    }

    //meta - określa jakiego content type się spodziewamy 
    //https://docs.rs/poem-openapi/1.3.29/poem_openapi/response/enum.StaticFileResponse.html
    //https://github.com/poem-web/poem/blob/master/poem-openapi/src/docs/response_content.md

    #[oai(method = "get", path = "/image/:id/:meta")]
    async fn handler_get_image(&self, id: Path<String>, meta: Path<String>) -> StaticFileResponse {
        let Path(id) = id;
        let Path(meta) = meta;

        let data = self.git.get_from_id(&id).await;

        let data = match data {
            Ok(content) => content,
            Err(err) => {
                let message = match err.to_string() {
                    (false, message) => format!("User error: {message}"),
                    (true, message) => format!("Internal error: {message}"),
                };

                return StaticFileResponse::InternalServerError(PlainText(message));
            }
        };

        let data = match data {
            Some(data) => data,
            None => {
                return StaticFileResponse::NotFound;
            }
        };

        let content = match data {
            GitBlob::Blob { content } => content,
            GitBlob::Tree { .. } => {
                return StaticFileResponse::NotFound;
            }
        };

        let header = match meta.as_str() {
            "webp" => Some("Content-Type: image/webp"),
            "png" => Some("Content-Type: image/png"),
            "jpg" => Some("Content-Type: image/jpeg"),
            "jpeg" => Some("Content-Type: image/jpeg"),
            _ => None
        };

        match header {
            Some(header) => {
                StaticFileResponse::Ok(
                    Binary(Body::from_vec(content)),
                    None,
                    None,
                    Some(String::from(header))
                )
            },
            None => StaticFileResponse::NotFound
        }
    }
}

fn response_with_root(result: Result<String, ErrorProcess>) -> ApiResponseHttp<RootResponse> {
    match result {
        Ok(root) => {
            let response = RootResponse {
                root,
            };
            ApiResponseHttp::ok(response)
        },
        Err(err) => {
            ApiResponseHttp::from(Err(err))
        }
    }
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


    let app = App::new(git.clone());

    //chwilowo wyłączamy synchronizację
    // let task_synchronize = start_sync(config.git_repo.clone()).await;

    log::info!("Start - {}:{}", config.http_host, config.http_port);

    let api_service = OpenApiService::new(
        app,
        "Server",
        "1.0",
    )
    .server("http://localhost:3000/api");

    let ui = api_service.swagger_ui();


    let addr = SocketAddr::from(([0, 0, 0, 0], config.http_port));
    
    Server::new(TcpListener::bind(addr)) //"127.0.0.1:3000"))
        .run(Route::new()
            .nest("/swagger", ui)
            .nest("/build", StaticFilesEndpoint::new("./build").show_files_listing())
            .nest_no_strip("/", api_service)
        )
        .await.unwrap();
}
