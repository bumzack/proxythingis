// https://morioh.com/p/47f04c30ffd7

use std::env;

use deadpool_postgres::Pool;
use warp::Filter;

use crate::db::{create_pool, with_db};
use crate::server::{create_person_handler, handle_rejection, health_handler, list_person_handler};

mod db;
mod server;
mod models;


#[tokio::main]
async fn main() {

    // TODO WTF why why ...
    let result = dotenvy::from_filename("/Users/bumzack/stoff/rust/proxythingis/too_many_servers/warp_tokio_postgres_person_server/.env");
    match &result {
        Ok(p) => println!("path to .env {:?}", &p),
        Err(e) => println!("error {:?}", e),
    }

    if env::var_os("RUST_LOG").is_none() {
        // Set `RUST_LOG=todos=debug` to see debug logs,
        // this only shows access logs.
        env::set_var("RUST_LOG", "response_to_everything=info");
    }

    let pool = create_pool();
    let limit = 20;
    let health_route = warp::path!("health")
        .and(with_db(pool.clone()))
        .and_then(health_handler);

    let person = warp::path("person");
    let person_create = person
        .and(warp::post())
        .and(warp::body::json())
        .and(with_db(pool.clone()))
        .and_then(create_person_handler);

    let person_list = person
        .and(warp::get())
        .and(with_db(pool.clone()))
        .and_then(move |pool: Pool| list_person_handler(pool, limit));

    let person_routes = person_create
        .or(person_list);

    let routes = health_route
        .or(person_routes)
        .with(warp::cors().allow_any_origin())
        .recover(handle_rejection);

    warp::serve(routes).run(([127, 0, 0, 1], 3050)).await;
}

