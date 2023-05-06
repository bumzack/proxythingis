use chrono::Utc;
use deadpool_postgres::Pool;
use log::info;
use tokio::sync::mpsc::UnboundedSender;
use tokio::sync::oneshot;
use warp::reply::json;
use warp::Reply;

use common::config_manager_models::GetConfigData;

use crate::config_manager::manager::ManagerCommand;
use crate::server::server::Result;
use crate::stats::db::{create_source_stats, create_target_stats};

pub async fn stats_read_handler(
    manager_sender: UnboundedSender<ManagerCommand>,
) -> Result<impl Reply> {
    info!("reading stats");
    let (tx, rx) = oneshot::channel();
    let get_config_data = GetConfigData {
        sender: tx,
        //   reset_start: false,
        whoami: "stats_read_handler".to_string(),
    };
    let cmd = ManagerCommand::GetConfig(get_config_data);
    manager_sender
        .send(cmd)
        .expect("stats_read_handler expected send successful");
    let proxy_config = rx
        .await
        .expect("stats_read_handler expected a valid proxy config");
    // info!("got proxyconfig = {:?}", proxy_config);

    let res = json(&proxy_config);

    Ok(res)
}

pub async fn stats_store_handler(
    pool: Pool,
    manager_sender: UnboundedSender<ManagerCommand>,
) -> Result<impl Reply> {
    info!("saving stats");
    let (tx, rx) = oneshot::channel();
    let get_config_data = GetConfigData {
        sender: tx,
        //   reset_start: true,
        whoami: "stats_store_handler".to_string(),
    };
    let cmd = ManagerCommand::GetConfig(get_config_data);
    manager_sender
        .send(cmd)
        .expect("stats_store_handler expected send successful");
    let mut proxy_config = rx
        .await
        .expect("stats_store_handler expected a valid proxy config");
    proxy_config.stop = Utc::now();

    for source in &proxy_config.server_sources {
        create_source_stats(
            pool.clone(),
            source.id,
            source.stats.hits as i64,
            source.stats.start,
            source.stats.stop,
        )
        .await
        .expect("stats_store_handler expects to be able to write the source stats");
        for target in &source.targets {
            create_target_stats(
                pool.clone(),
                target.id,
                target.stats.hits as i64,
                target.stats.min_ns as i64,
                target.stats.max_ns as i64,
                target.stats.avg_ns as i64,
                source.stats.start,
                source.stats.stop,
            )
            .await
            .expect("stats_store_handler expects to be able to write the target stats");
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

pub async fn stats_reset_handler(
    manager_sender: UnboundedSender<ManagerCommand>,
) -> Result<impl Reply> {
    info!("reset stats");
    let cmd = ManagerCommand::ResetStats;
    manager_sender
        .send(cmd)
        .expect("stats_reset_handler expected send successful");
    let msg = "successfully resetted stats";
    let res = json(&msg);

    Ok(res)
}
