use chrono::{DateTime, Utc};
use serde::Deserialize;
use serde::Serialize;

#[derive(Deserialize, Serialize, Clone, Debug, Default, PartialEq)]
// #[cfg_attr(feature = "sycamore_support", derive(Props))]
pub struct ServerSource {
    pub id: i32,
    pub description: String,
    pub path_starts_with: String,
    pub method: String,
    pub created: DateTime<Utc>,
    pub targets: Vec<ServerTarget>,
    pub stats: ServerSourceStats,
}

#[derive(Deserialize, Serialize, Clone, Debug, Default, PartialEq)]
// #[cfg_attr(feature = "sycamore_support", derive(Props))]
pub struct ServerTarget {
    pub id: i32,
    pub description: String,
    pub schema: String,
    pub host: String,
    pub port: i32,
    pub path: String,
    pub method: String,
    pub stats: ServerTargetStats,
    pub active: bool,
    pub created: DateTime<Utc>,
}

#[derive(Deserialize, Serialize, Clone, Debug, Default, PartialEq)]
pub struct Server2Target {
    pub id: i32,
    pub source_id: i32,
    pub target_id: i32,
}

#[derive(Deserialize, Serialize, Clone, Debug, Default, PartialEq)]
pub struct NewServerSourcePost {
    pub description: String,
    pub path_starts_with: String,
    pub method: String,
}

#[derive(Deserialize, Serialize, Clone, Debug, Default, PartialEq)]
pub struct NewServerSource<'a> {
    pub description: &'a str,
    pub path_starts_with: &'a str,
    pub method: &'a str,
}

#[derive(Deserialize, Serialize, Clone, Debug, Default, PartialEq)]
pub struct NewServerTargetPost {
    pub description: String,
    pub schema: String,
    pub host: String,
    pub port: i32,
    pub path: String,
    pub method: String,
    pub source: i32,
    pub active: bool,
}

#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct NewServerTarget<'a> {
    pub description: &'a str,
    pub schema: &'a str,
    pub host: &'a str,
    pub port: i32,
    pub path: &'a str,
    pub method: &'a str,
    pub source: i32,
    pub active: bool,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub message: String,
}

#[derive(Deserialize, Serialize, Default, Clone, Debug, PartialEq)]
// #[cfg_attr(feature = "sycamore_support", derive(Props))]
pub struct ServerSourceStats {
    pub id: i32,
    pub source_id: i32,
    pub hits: i128,
    pub start: DateTime<Utc>,
    pub stop: DateTime<Utc>,
    pub created: DateTime<Utc>,
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
// #[cfg_attr(feature = "sycamore_support", derive(Props))]
pub struct ServerTargetStats {
    pub id: i32,
    pub target_id: i32,
    pub hits: i128,
    pub avg_ns: i128,
    pub max_ns: i128,
    pub min_ns: i128,
    pub start: DateTime<Utc>,
    pub stop: DateTime<Utc>,
    pub created: DateTime<Utc>,
}

impl Default for ServerTargetStats {
    fn default() -> Self {
        ServerTargetStats {
            id: 0,
            target_id: 0,
            hits: 0,
            avg_ns: 0,
            max_ns: 0,
            min_ns: 99999999999,
            start: chrono::Utc::now(),
            stop: chrono::Utc::now(),
            created: chrono::Utc::now(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ProxyConfig {
    pub server_sources: Vec<ServerSource>,
    pub start: DateTime<Utc>,
    pub stop: DateTime<Utc>,
}
