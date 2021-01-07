use warp::{Filter, Reply, http::Response};
use std::convert::Infallible;
use std::net::Ipv4Addr;
use serde::Deserialize;

#[derive(Deserialize)]
struct Config {
    http_host: Ipv4Addr,
    http_port: u16,
}

async fn handler_index() -> Result<impl Reply, Infallible> {
    Ok(Response::new("To jest index routing"))
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let config = match envy::from_env::<Config>() {
        Ok(config) => config,
        Err(error) => panic!("Service started with invalid environment variables {:#?}", error)
    };

    let routes_default = warp::any().map(|| "Websocket server index");

    let route_index = warp::path!("index")
        .and_then(handler_index);
    
    let routes = route_index
        // .or(route_ws_index_http)
        // .or(route_prometheus)
        // .or(route_cos)
        .or(routes_default)
    ;

    log::info!("Start - {}:{}", config.http_host, config.http_port);

    warp::serve(routes)
        .run((config.http_host, config.http_port))
        .await;

}
