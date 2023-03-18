use std::convert::Infallible;

use chrono::Utc;
use deadpool_postgres::Pool;
use tokio::sync::mpsc::UnboundedSender;
use tokio::sync::oneshot;
use warp::{Filter, reject, Rejection, Reply};
use warp::reply::json;

use crate::config_manager::{GetConfigData, ManagerCommand, UpdateServerConfigData};
use crate::db::{activate_server, create_source, create_source_stats, create_target, create_target_stats, deactivate_server, list_server};
use crate::models::{DivideByZero, NewServerSourcePost, NewServerTargetPost};

pub async fn create_source_handler(pool: Pool, body: NewServerSourcePost, manager_sender: UnboundedSender<ManagerCommand>) -> Result<impl Reply> {
    let res = json(&create_source(pool.clone(), body)
        .await
        // TODO fix CustomError
        .map_err(|e| {
            println!("error rejection {:?}", e);
            reject::custom(DivideByZero)
        })?,
    );
    send_config(pool, manager_sender).await;

    Ok(res)
}


pub fn with_sender(manager_sender: UnboundedSender<ManagerCommand>) -> impl Filter<Extract=(UnboundedSender<ManagerCommand>, ), Error=Infallible> + Clone {
    warp::any().map(move || manager_sender.clone())
}

pub fn with_db(pool: Pool) -> impl Filter<Extract=(Pool, ), Error=Infallible> + Clone {
    warp::any().map(move || pool.clone())
}

async fn send_config(pool: Pool, manager_sender: UnboundedSender<ManagerCommand>) {
    let server = list_server(pool, true).await.unwrap();

    let config = UpdateServerConfigData {
        server_sources: server,
    };
    let cmd = ManagerCommand::UpdateServerConfig(config);
    manager_sender.send(cmd).unwrap();
}

pub async fn create_target_handler(pool: Pool, body: NewServerTargetPost, manager_sender: UnboundedSender<ManagerCommand>) -> Result<impl Reply> {
    let res = json(&create_target(pool.clone(), body)
        .await
        // TODO fix CustomError
        .map_err(|e| {
            println!("error rejection {:?}", e);
            reject::custom(DivideByZero)
        })?,
    );
    send_config(pool, manager_sender).await;
    Ok(res)
}

pub async fn list_servers_handler(pool: Pool) -> Result<impl Reply> {
    let data = list_server(pool, false)
        .await
        // TODO fix CustomError
        .map_err(|e| {
            println!("error rejection {:?}", e);
            reject::custom(DivideByZero)
        })?;
    Ok(json(&data))
}

pub async fn activate_server_handler(pool: Pool, id: i32, manager_sender: UnboundedSender<ManagerCommand>) -> Result<impl Reply> {
    let _ = activate_server(pool.clone(), id)
        .await
        // TODO fix CustomError
        .map_err(|e| {
            println!("error rejection {:?}", e);
            reject::custom(DivideByZero)
        })?;
    send_config(pool, manager_sender).await;

    Ok("server activated")
}

pub async fn deactivate_server_handler(pool: Pool, id: i32, manager_sender: UnboundedSender<ManagerCommand>) -> Result<impl Reply> {
    let _ = deactivate_server(pool.clone(), id)
        .await
        // TODO fix CustomError
        .map_err(|e| {
            println!("error rejection {:?}", e);
            reject::custom(DivideByZero)
        })?;
    send_config(pool, manager_sender).await;
    Ok("server deactivated")
}

// pub async fn health_handler(pool: Pool) -> std::result::Result<impl Reply, Rejection> {
//     let client = pool.get().await.unwrap();
//
//     println!("hello from healthhandler");
//     client
//         .execute("SELECT 1", &[])
//         .await
//         .map_err(|e| reject::custom(DBQueryError(e)))?;
//     Ok(StatusCode::OK)
// }

type Result<T> = std::result::Result<T, Rejection>;


// pub async fn handle_rejection(err: Rejection) -> std::result::Result<impl Reply, Infallible> {
//     let code;
//     let message;
//
//     if err.is_not_found() {
//         code = StatusCode::NOT_FOUND;
//         message = "Not Found";
//     } else if let Some(_) = err.find::<warp::filters::body::BodyDeserializeError>() {
//         code = StatusCode::BAD_REQUEST;
//         message = "Invalid Body";
//     } else if let Some(e) = err.find::<MyError>() {
//         match e {
//             DBQueryError(_) => {
//                 code = StatusCode::BAD_REQUEST;
//                 message = "Could not Execute request";
//             }
//             // _ => {
//             //     eprintln!("unhandled application error: {:?}", err);
//             //     code = StatusCode::INTERNAL_SERVER_ERROR;
//             //     message = "Internal Server Error";
//             // }
//         }
//     } else if let Some(_) = err.find::<warp::reject::MethodNotAllowed>() {
//         code = StatusCode::METHOD_NOT_ALLOWED;
//         message = "Method Not Allowed";
//     } else {
//         eprintln!("unhandled error: {:?}", err);
//         code = StatusCode::INTERNAL_SERVER_ERROR;
//         message = "Internal Server Error";
//     }
//
//     let json = json(&ErrorResponse {
//         message: message.into(),
//     });
//
//     Ok(warp::reply::with_status(json, code))
// }

pub async fn stats_read_handler(manager_sender: UnboundedSender<ManagerCommand>) -> Result<impl Reply> {
    let (tx, rx) = oneshot::channel();
    let get_config_data = GetConfigData {
        sender: tx,
        reset_start: false,
    };
    let cmd = ManagerCommand::GetConfig(get_config_data);
    manager_sender.send(cmd).expect("stats_read_handler expected send successful");
    let proxy_config = rx.await.expect("stats_read_handler expected a valid proxy config");
    // println!("got proxyconfig = {:?}", proxy_config);

    let res = json(&proxy_config);

    Ok(res)
}

pub async fn stats_store_handler(pool: Pool, manager_sender: UnboundedSender<ManagerCommand>) -> Result<impl Reply> {
    let (tx, rx) = oneshot::channel();
    let get_config_data = GetConfigData {
        sender: tx,
        reset_start:true,
    };
    let cmd = ManagerCommand::GetConfig(get_config_data);
    manager_sender.send(cmd).expect("stats_store_handler expected send successful");
    let mut proxy_config = rx.await.expect("stats_store_handler expected a valid proxy config");
    proxy_config.stop = Utc::now();

    for source in &proxy_config.server_sources {
        create_source_stats(pool.clone(), source.id, source.stats.hits, source.stats.start, source.stats.stop).await.expect("stats_store_handler expects to be able to write the source stats");
        for target in &source.targets {
            create_target_stats(pool.clone(), target.id, target.stats.hits, target.stats.min_ns, target.stats.max_ns, target.stats.avg_ns, source.stats.start, source.stats.stop).await.expect("stats_store_handler expects to be able to write the target stats");
        }
    }

    // proxy_config.server_sources.iter().for_each(async |source| {
    //     source.targets.iter().for_each(async |target| {
    //         create_target_stats(pool.clone(), target.id, target.stats.hits, target.stats.min_ns, target.stats.max_ns, target.stats.avg_ns, source.stats.start, source.stats.stop).await.expect("stats_store_handler expects to be able to write the target stats");
    //     })
    // });
    let res = json(&proxy_config);

    Ok(res)
}

pub async fn stats_reset_handler(manager_sender: UnboundedSender<ManagerCommand>) -> Result<impl Reply> {
    let cmd = ManagerCommand::ResetStats;
    manager_sender.send(cmd).expect("stats_reset_handler expected send successful");
    let msg = "successfully reseted stats";
    let res = json(&msg);

    Ok(res)
}

