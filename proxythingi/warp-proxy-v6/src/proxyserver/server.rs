use deadpool_postgres::Pool;
use tokio::sync::mpsc::UnboundedSender;
use warp::reply::json;
use warp::{reject, Reply};

use common::models::{NewServerSourcePost, NewServerTargetPost};

use crate::config_manager::manager::{send_config, ManagerCommand};
use crate::proxyserver::db::{
    activate_server, create_source, create_target, deactivate_server, list_server,
};
use crate::server::models::DivideByZero;
use crate::server::server_impl::Result;

pub async fn create_source_handler(
    pool: Pool,
    body: NewServerSourcePost,
    manager_sender: UnboundedSender<ManagerCommand>,
) -> Result<impl Reply> {
    let res = json(
        &create_source(pool.clone(), body)
            .await
            // TODO fix CustomError
            .map_err(|_e| {
                // info!("error rejection {:?}", e);
                reject::custom(DivideByZero)
            })?,
    );
    send_config(pool, manager_sender).await;

    Ok(res)
}

pub async fn create_target_handler(
    pool: Pool,
    body: NewServerTargetPost,
    manager_sender: UnboundedSender<ManagerCommand>,
) -> Result<impl Reply> {
    let res = json(
        &create_target(pool.clone(), body)
            .await
            // TODO fix CustomError
            .map_err(|_e| {
                // info!("error rejection {:?}", e);
                reject::custom(DivideByZero)
            })?,
    );
    send_config(pool, manager_sender).await;
    Ok(res)
}

pub async fn list_servers_handler(pool: Pool) -> Result<impl Reply> {
    let data = list_server(pool)
        .await
        // TODO fix CustomError
        .map_err(|_e| {
            // info!("error rejection {:?}", e);
            reject::custom(DivideByZero)
        })?;
    Ok(json(&data))
}

pub async fn activate_server_handler(
    pool: Pool,
    id: i32,
    manager_sender: UnboundedSender<ManagerCommand>,
) -> Result<impl Reply> {
    activate_server(pool.clone(), id)
        .await
        // TODO fix CustomError
        .map_err(|_e| {
            // info!("error rejection {:?}", e);
            reject::custom(DivideByZero)
        })?;
    send_config(pool, manager_sender).await;

    Ok("server activated")
}

pub async fn deactivate_server_handler(
    pool: Pool,
    id: i32,
    manager_sender: UnboundedSender<ManagerCommand>,
) -> Result<impl Reply> {
    deactivate_server(pool.clone(), id)
        .await
        // TODO fix CustomError
        .map_err(|_e| {
            // info!("error rejection {:?}", e);
            reject::custom(DivideByZero)
        })?;
    send_config(pool, manager_sender).await;
    Ok("server deactivated")
}
