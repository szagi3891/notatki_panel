#![feature(async_closure)]

use common::{PostParamsCreateDir, PostParamsFetchNodePost};
use warp::{Filter, Reply, http::Response};
use std::convert::Infallible;
use std::net::Ipv4Addr;
use serde::Deserialize;
use std::sync::Arc;

mod gitdb;
mod utils;
mod sync;

use sync::start_sync;

use gitdb::GitDB;

#[derive(Deserialize)]
struct Config {
    http_host: Ipv4Addr,
    http_port: u16,
    git_sync: String,
    git_notes: String,
}

struct AppState {
    git: GitDB,
}

impl AppState {
    pub fn new(git: GitDB) -> Arc<AppState> {
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
                init();
            </script>
        </head>
        <body></body>
    </html>
"##))
}

async fn handler_fetch_node(app_state: Arc<AppState>, body_request: PostParamsFetchNodePost) -> Result<impl warp::Reply, Infallible> {
    let node_id = body_request.node_id;

    let data = app_state.git.get(node_id).await;

    match data {
        Ok(data) => {
            let a = serde_json::to_string(&data).unwrap();

            let response = Response::builder()
                .status(200)
                .body(a);
            Ok(response)
        },
        Err(err) => {
            let response = Response::builder()
                .status(500)
                .body(format!("error = {:?}", err));

            Ok(response)
        }
    }
}

async fn handler_create_dir(app_state: Arc<AppState>, body: PostParamsCreateDir) -> Result<impl warp::Reply, Infallible> {
    let new_id = app_state.git.create_dir(body.parent_node, body.name).await;

    match new_id {
        Ok(new_id) => {
            let message = serde_json::to_string(&new_id).unwrap();

            let response = Response::builder()
                .status(200)
                .body(message);
            Ok(response)
        },
        Err(err) => {
            let response = Response::builder()
                .status(500)
                .body(format!("error dir create = {:?}", err));

            Ok(response)
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

    let git_db = GitDB::new(config.git_notes);
    let app_state = AppState::new(git_db);

    let task_synchronize = start_sync(config.git_sync).await;

    app_state.git.check_root().await;           //sprawdź czy istnieje węzeł główny

    let route_index =
        warp::path::end()
        .and_then(handler_index);

    let route_build =
        warp::path("build")
        .and(warp::fs::dir("build"));

    let filter_fetch_node =
        warp::path!("fetch_node")
        .and(warp::post())
        .and(inject_state(app_state.clone()))
        .and(warp::body::json())
        .and_then(handler_fetch_node);

    let filter_create_dir =
        warp::path!("create_dir")
        .and(warp::post())
        .and(inject_state(app_state.clone()))
        .and(warp::body::json())
        .and_then(handler_create_dir);
    
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
        .or(filter_fetch_node)
        .or(filter_create_dir)
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

    task_synchronize.off();
}
