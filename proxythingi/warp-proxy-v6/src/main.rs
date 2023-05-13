use log::{info, LevelFilter};
use pretty_env_logger::env_logger::{Builder, Target};
use tokio::sync::mpsc;
use warp::Filter;
use warp::hyper::Client;
use warp::hyper::client::HttpConnector;

use common::models::ProxyConfig;

use crate::config_manager::manager::start_config_manager;
use crate::db::db::create_pool;
use crate::proxy::route::proxy_routes;
use crate::proxyserver::db::list_server;
use crate::proxyserver::route::server_routes;
use crate::stats::route::stats_routes;

mod config_manager;
mod db;
mod proxy;
mod proxyserver;
mod server;
mod stats;

// gotta give credit where credit is due and stuff
lazy_static::lazy_static! {
    static ref CLIENT: warp::hyper::Client<HttpConnector> = {
         let http_connector = HttpConnector::new();
         let client = Client::builder().build(http_connector);
         return client;
    };
}

// #[tokio::main(worker_threads = 2)]
#[tokio::main]
async fn main() {
    let _result = dotenvy::from_filename(
        "/Users/bumzack/stoff/rust/proxythingis/proxythingi/warp-proxy-v6/.env",
    );

    let mut builder = Builder::new();
    builder.target(Target::Stdout);
    builder.filter_level(LevelFilter::Info);
    builder.init();
    info!("builder={:?}", builder);

    let pool = create_pool();

    let servers = list_server(pool.clone(), true)
        .await
        .expect("loading the servers config should work");

    let proxy_config = ProxyConfig {
        server_sources: servers,
        start: chrono::Utc::now(),
        stop: chrono::Utc::now(),
    };

    let (manager_sender, manager_receiver) = mpsc::unbounded_channel();

    let _handle_config_manager = start_config_manager(proxy_config, manager_receiver);

    let stats_routes = stats_routes(&pool, &manager_sender);
    let server_routes = server_routes(pool, &manager_sender);
    let proxy_routes = proxy_routes(manager_sender);

    let cors = warp::cors()
        .allow_any_origin()
        .expose_headers(vec![
            "x-duration",
            "x-provided-by",
            "x-initiated-by",
            "x-processed-by",
        ])
        .allow_headers(vec![
            "User-Agent",
            "Sec-Fetch-Mode",
            "Referer",
            "Origin",
            "content-type",
            "Access-Control-Request-Method",
            "Access-Control-Request-Headers",
            "Access-Control-Allow-Headers",
            "Access-Control-Allow-Methods",
            "Access-Control-Allow-Origin",
            "Access-Control-Expose-Headers",
            "Access-Control-Request-Headers",
            "Access-Control-Request-Methods",
            "Accept-Encoding",
            "Accept-Language",
            "Accept-Post",
            "Access-Control-Allow-Credentials",
            "keep-alive",
            "x-duration",
            "x-provided-by",
            "x-initiated-by",
            "x-processed-by",
        ])
        .allow_methods(vec!["POST", "GET", "OPTIONS", "PUT", "DELETE", "HEAD"]);

    let routes = stats_routes.or(server_routes).or(proxy_routes).with(cors);

    warp::serve(routes).run(([127, 0, 0, 1], 3036)).await;
}
