use deadpool_postgres::Pool;
use tokio::sync::mpsc::UnboundedSender;
use warp::Filter;

use crate::config_manager::manager::ManagerCommand;
use crate::config_manager::server::with_sender;
use crate::db::server::with_db;
use crate::proxyserver::server::{
    activate_server_handler, create_source_handler, create_target_handler,
    deactivate_server_handler, list_servers_handler,
};

pub fn server_routes(
    pool: Pool,
    manager_sender: &UnboundedSender<ManagerCommand>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    let server_source = warp::path!("proxythingi" / "server" / "source");
    let server_source_create = server_source
        .and(warp::post())
        .and(warp::body::json())
        .and(with_db(pool.clone()))
        .and(with_sender(manager_sender.clone()))
        .and_then(|body, pool, sender| create_source_handler(pool, body, sender));

    let server_target = warp::path!("proxythingi" / "server" / "target");
    let server_target_create = server_target
        .and(warp::post())
        .and(warp::body::json())
        .and(with_db(pool.clone()))
        .and(with_sender(manager_sender.clone()))
        .and_then(|body, pool, sender| create_target_handler(pool, body, sender));

    let server = warp::path!("proxythingi" / "server");
    let server_list = server
        .and(warp::get())
        .and(with_db(pool.clone()))
        .and_then(list_servers_handler);

    let server = warp::path!("proxythingi" / "server" / "activate" / i32);
    let server_activate = server
        .and(warp::get())
        .and(with_db(pool.clone()))
        .and(with_sender(manager_sender.clone()))
        .and_then(|id, pool, sender| activate_server_handler(pool, id, sender));

    let server = warp::path!("proxythingi" / "server" / "deactivate" / i32);
    let server_deactivate = server
        .and(warp::get())
        .and(with_db(pool.clone()))
        .and(with_sender(manager_sender.clone()))
        .and_then(|id, pool, sender| deactivate_server_handler(pool, id, sender));

    server_source_create
        .or(server_target_create)
        .or(server_list)
        .or(server_activate)
        .or(server_deactivate)
}
