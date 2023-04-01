extern crate lazy_static;
#[macro_use]
extern crate log;

use std::env;

use tokio::sync::mpsc;
use warp::Filter;
use warp::hyper::Client;
use warp::hyper::client::HttpConnector;

use crate::config_manager::manager::{ProxyConfig, start_config_manager};
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
    pretty_env_logger::init();

    let _result =
        dotenvy::from_filename("/Users/bumzack/stoff/rust/proxythingis/warp-proxy-v4/.env");

    if env::var_os("RUST_LOG").is_none() {
        // Set `RUST_LOG=todos=debug` to see debug logs,
        // this only shows access logs.
        env::set_var("RUST_LOG", "todos=info");
    }
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


    // let cors = warp::cors()
    //     .allow_any_origin()
    //     .allow_headers(vec![
    //         "User-Agent",
    //         "Sec-Fetch-Mode",
    //         "Referer",
    //         "Origin",
    //         "Access-Control-Request-Method",
    //         "Access-Control-Request-Headers",
    //     ])
    //     .allow_methods(vec!["POST", "GET"]);

    let stats_routes = stats_routes(&pool, &manager_sender);
    let server_routes = server_routes(pool, &manager_sender);
    let proxy_routes = proxy_routes(manager_sender);

    let routes = stats_routes.or(server_routes).or(proxy_routes);

    warp::serve(routes).run(([127, 0, 0, 1], 3036)).await;
}
