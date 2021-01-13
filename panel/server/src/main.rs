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

fn inject_state<T: Clone + Sized + Send>(state: T) -> impl Filter<Extract = (T,), Error = Infallible> + Clone {
    warp::any().map(move || state.clone())
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let config = match envy::from_env::<Config>() {
        Ok(config) => config,
        Err(error) => panic!("Service started with invalid environment variables {:#?}", error)
    };

    let git_db = GitDB::new(config.git_notes);
    let _app_state = AppState::new(git_db);

    let task_synchronize = start_sync(config.git_sync);

    let routes_default = warp::any().map(|| "Websocket server index");

    let route_build = warp::path("build").and(warp::fs::dir("build"));

    let route_index = warp::path::end()
        .and_then(handler_index);
    
    let routes = route_index
        .or(route_build)
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
