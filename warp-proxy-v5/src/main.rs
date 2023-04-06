extern crate lazy_static;

use std::env;

use tokio::sync::mpsc;
use tracing_subscriber::fmt::format::FmtSpan;
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

    //pretty_env_logger::init();
    let filter = std::env::var("RUST_LOG").unwrap_or_else(|_| "tracing=info,warp=debug".to_owned());
    tracing_subscriber::fmt()
        // Use the filter we built above to determine which traces to record.
        .with_env_filter(filter)
        // Record an event when each span closes. This can be used to time our
        // routes' durations!
        .with_span_events(FmtSpan::CLOSE)
        .init();

    let stats_routes = stats_routes(&pool, &manager_sender);
    let server_routes = server_routes(pool, &manager_sender);
    let proxy_routes = proxy_routes(manager_sender);

    let routes = stats_routes.or(server_routes).or(proxy_routes);

    warp::serve(routes).run(([127, 0, 0, 1], 3035)).await;
}
