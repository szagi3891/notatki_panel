#![feature(try_trait_v2)]
#![allow(clippy::needless_lifetimes)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::module_inception)]
#![allow(clippy::wrong_self_convention)]
#![allow(clippy::len_zero)]
mod models;

use poem::{
    endpoint::StaticFilesEndpoint,
    listener::TcpListener,
    Server,
    Route,
};
use poem_openapi::{
    OpenApiService,
};

use std::{net::Ipv4Addr, sync::Arc};
use serde::{Deserialize};
use std::net::SocketAddr;
use tokio::sync::Notify;

mod sync;
mod git;
mod utils;
mod static_response;
mod api;

use git::{Git};

use crate::{api::Api, sync::start_sync};

#[derive(Deserialize)]
struct Config {
    http_host: Ipv4Addr,
    http_port: u16,
    git_repo: String,
}


#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let config = match envy::from_env::<Config>() {
        Ok(config) => config,
        Err(error) => panic!("Service started with invalid environment variables {error:#?}")
    };

    let notify = Arc::new(Notify::new());

    let task_synchronize = start_sync(notify.clone(), config.git_repo.clone()).await;

    println!("start git test: {}", &config.git_repo);
    let git = Git::new(notify, config.git_repo.clone(), "master".into()).unwrap();

    let api = Api::new(git.clone());


    log::info!("Start - {}:{}", config.http_host, config.http_port);

    let api_service = OpenApiService::new(
            api,
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

    task_synchronize.off();
}

