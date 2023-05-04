use chrono::{DateTime, Utc};
use deadpool_postgres::Pool;
use log::info;
use serde::Serialize;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use tokio::task::JoinHandle;

use crate::proxyserver::db::list_server;
use crate::proxyserver::models::ServerSource;

#[derive(Debug)]
pub enum ManagerCommand {
    GetConfig(GetConfigData),
    UpdateSourceStats(UpdateSourceStatsData),
    UpdateTargetStats(UpdateTargetStatsData),
    UpdateServerConfig(UpdateServerConfigData),
    ResetStats,
}

// #[derive(Debug)]
// pub struct ResetStatsData {
//     pub(crate) sender: tokio::sync::oneshot::Sender<ProxyConfig>,
// }

#[derive(Debug)]
pub struct GetConfigData {
    pub(crate) sender: tokio::sync::oneshot::Sender<ProxyConfig>,
    // pub(crate) reset_start: bool,
}

#[derive(Debug, Clone)]
pub struct UpdateSourceStatsData {
    pub(crate) id: i32,
}

#[derive(Debug, Clone)]
pub struct UpdateTargetStatsData {
    pub(crate) id: i32,
    pub(crate) duration_nanos: u32,
}

#[derive(Debug, Clone, Serialize)]
pub struct ProxyConfig {
    pub(crate) server_sources: Vec<ServerSource>,
    pub(crate) start: DateTime<Utc>,
    pub(crate) stop: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize)]
pub struct UpdateServerConfigData {
    pub(crate) server_sources: Vec<ServerSource>,
}

pub fn start_config_manager(
    mut proxy_config: ProxyConfig,
    mut manager_receiver: UnboundedReceiver<ManagerCommand>,
) -> JoinHandle<()> {
    tokio::spawn(async move {
        info!("manager thread started");
        while let Some(cmd) = manager_receiver.recv().await {
            match cmd {
                ManagerCommand::GetConfig(c) => {
                    // info!("sending config");
                    c.sender
                        .send(proxy_config.clone())
                        .expect("start_config_manager  ManagerCommand::GetConfig should succeed");
                    // if c.reset_start {
                    //     proxy_config.start = Utc::now();
                    // }
                }
                ManagerCommand::UpdateSourceStats(source_stats) => {
                    // info!("updating stats for source server {}", source_stats.id);
                    for s in proxy_config.server_sources.iter_mut() {
                        if s.id == source_stats.id {
                            s.stats.hits += 1;
                        }
                    }
                }
                ManagerCommand::UpdateTargetStats(target_stats) => {
                    // info!("updating stats for target server {}", target_stats.id);
                    for s in proxy_config.server_sources.iter_mut() {
                        for t in s.targets.iter_mut() {
                            if t.id == target_stats.id {
                                if t.stats.min_ns > target_stats.duration_nanos {
                                    t.stats.min_ns = target_stats.duration_nanos;
                                }
                                if t.stats.max_ns < target_stats.duration_nanos {
                                    t.stats.max_ns = target_stats.duration_nanos;
                                }
                                let avg = t.stats.avg_ns;
                                let old_n = t.stats.hits;
                                let sum = avg * old_n;
                                let new_avg = (sum + target_stats.duration_nanos) / (old_n + 1);

                                t.stats.hits += 1;
                                t.stats.avg_ns = new_avg;
                            }
                        }
                    }
                }
                ManagerCommand::UpdateServerConfig(new_config) => {
                    // info!("got a new config");
                    proxy_config.server_sources = new_config.server_sources;
                }
                ManagerCommand::ResetStats => {
                    // info!("reset config. stats_started was {}", stats_started);
                    for s in proxy_config.server_sources.iter_mut() {
                        s.stats.hits = 0;
                        for t in s.targets.iter_mut() {
                            t.stats.hits = 0;
                            t.stats.avg_ns = 0;
                            t.stats.min_ns = 999999999;
                            t.stats.max_ns = 0;
                        }
                    }
                    proxy_config.start = Utc::now();
                    proxy_config.stop = Utc::now();
                }
            }
        }
    })
}

pub async fn send_config(pool: Pool, manager_sender: UnboundedSender<ManagerCommand>) {
    let server = list_server(pool, true).await.unwrap();

    let config = UpdateServerConfigData {
        server_sources: server,
    };
    let cmd = ManagerCommand::UpdateServerConfig(config);
    manager_sender.send(cmd).unwrap();
}
