use chrono::Utc;
use deadpool_postgres::Pool;
use log::info;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use tokio::task::JoinHandle;

use common::config_manager_models::{
    GetConfigData, UpdateServerConfigData, UpdateSourceStatsData, UpdateTargetStatsData,
};
use common::models::ProxyConfig;

use crate::proxyserver::db::list_server;

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

pub fn start_config_manager(
    mut proxy_config: ProxyConfig,
    mut manager_receiver: UnboundedReceiver<ManagerCommand>,
) -> JoinHandle<()> {
    tokio::spawn(async move {
        info!("manager thread started");
        while let Some(cmd) = manager_receiver.recv().await {
            match cmd {
                ManagerCommand::GetConfig(c) => {
                    // info!("sending config to {}", &c.whoami);
                    match c.sender.send(proxy_config.clone()) {
                        Ok(()) => {} // info!("send was ok"),
                        Err(_) => panic!("error sending config to task. {}", &c.whoami),
                    }
                    // if c.reset_start {
                    //     proxy_config.start = Utc::now();
                    // }
                }
                ManagerCommand::UpdateSourceStats(source_stats) => {
                    // info!("updating stats for source server {}", source_stats.id);
                    for s in proxy_config.server_sources.iter_mut() {
                        if s.id == source_stats.id {
                            s.stats.hits += 1;
                            // info!(
                            //     "updating source stats hit for id {}, new hits {}",
                            //     s.id, s.stats.hits
                            // );
                        }
                    }
                }
                ManagerCommand::UpdateTargetStats(target_stats) => {
                    // info!("updating stats for target server {}", target_stats.id);
                    for s in proxy_config.server_sources.iter_mut() {
                        for t in s.targets.iter_mut() {
                            if t.id == target_stats.id {
                                // info!(
                                //     "updating target stats hit for id {}, added duration {}",
                                //     t.id, target_stats.duration_nanos
                                // );

                                if t.stats.min_ns > target_stats.duration_nanos {
                                    t.stats.min_ns = target_stats.duration_nanos;
                                }
                                if t.stats.max_ns < target_stats.duration_nanos {
                                    t.stats.max_ns = target_stats.duration_nanos;
                                }
                                let avg = t.stats.avg_ns;

                                // FUNNY BUG: remove casts to u128 -> then this will overflow and crash the tasks
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
    let server = list_server(pool).await.unwrap();

    let config = UpdateServerConfigData {
        server_sources: server,
    };
    let cmd = ManagerCommand::UpdateServerConfig(config);
    manager_sender.send(cmd).unwrap();
}
