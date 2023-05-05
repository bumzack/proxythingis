use chrono::{DateTime, Utc};
use serde::Serialize;

use crate::models::ServerSource;

#[derive(Debug)]
pub struct GetConfigData {
    pub sender: tokio::sync::oneshot::Sender<ProxyConfig>,
    //    pub(crate) reset_start: bool,
}

#[derive(Debug, Clone)]
pub struct UpdateSourceStatsData {
    pub id: i32,
}

#[derive(Debug, Clone)]
pub struct UpdateTargetStatsData {
    pub id: i32,
    pub duration_nanos: i64,
}

#[derive(Debug, Clone, Serialize)]
pub struct ProxyConfig {
    pub server_sources: Vec<ServerSource>,
    pub start: DateTime<Utc>,
    pub stop: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize)]
pub struct UpdateServerConfigData {
    pub server_sources: Vec<ServerSource>,
}
