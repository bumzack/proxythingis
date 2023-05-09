use serde::Serialize;

use crate::models::{ProxyConfig, ServerSource};

#[derive(Debug)]
pub struct GetConfigData {
    pub sender: tokio::sync::oneshot::Sender<ProxyConfig>,
    pub whoami: String,
    //    pub(crate) reset_start: bool,
}

#[derive(Debug, Clone)]
pub struct UpdateSourceStatsData {
    pub id: i32,
}

#[derive(Debug, Clone)]
pub struct UpdateTargetStatsData {
    pub id: i32,
    pub duration_nanos: i128,
}

#[derive(Debug, Clone, Serialize)]
pub struct UpdateServerConfigData {
    pub server_sources: Vec<ServerSource>,
}
