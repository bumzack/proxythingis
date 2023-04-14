use log::LevelFilter;
use pretty_env_logger::env_logger::Builder;
use warp::Filter;

use crate::db::{get_connection_pool, with_db};
use crate::server::{create_person_handler, health_handler, list_person_handler};

mod db;
mod models;
mod schema;
mod server;

#[tokio::main]
async fn main() {
    Builder::new().filter_level(LevelFilter::Info).init();

    let pool = get_connection_pool();
    let health_route = warp::path!("health")
        .and(with_db(pool.clone()))
        .and_then(health_handler);

    let person = warp::path!("api" / "person");
    let person_create = person
        .and(warp::post())
        .and(warp::body::json())
        .and(with_db(pool.clone()))
        .and_then(create_person_handler);

    let person_list = person
        .and(warp::get())
        .and(with_db(pool.clone()))
        .and_then(list_person_handler);

    let person_routes = person_create.or(person_list);

    let routes = health_route
        .or(person_routes)
        .with(warp::cors().allow_any_origin());
    // .recover(handle_rejection);

    warp::serve(routes).run(([127, 0, 0, 1], 3060)).await;
}
