use crate::config_manager::manager::ManagerCommand;
use crate::config_manager::server::with_sender;
use crate::db::server::with_db;
use crate::stats::server::{stats_read_handler, stats_reset_handler, stats_store_handler};
use deadpool_postgres::Pool;
use tokio::sync::mpsc::UnboundedSender;
use warp::Filter;

pub fn stats_routes(
    pool: &Pool,
    manager_sender: &UnboundedSender<ManagerCommand>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    let stats = warp::path!("proxythingi" / "stats");
    let stats_read = stats
        .and(warp::get())
        .and(with_sender(manager_sender.clone()))
        .and_then(|sender| stats_read_handler(sender));

    let stats_store = stats
        .and(warp::post())
        .and(with_db(pool.clone()))
        .and(with_sender(manager_sender.clone()))
        .and_then(|pool, sender| stats_store_handler(pool, sender));

    let stats_reset = stats
        .and(warp::delete())
        .and(with_sender(manager_sender.clone()))
        .and_then(|sender| stats_reset_handler(sender));

    stats_read.or(stats_store).or(stats_reset)
}
