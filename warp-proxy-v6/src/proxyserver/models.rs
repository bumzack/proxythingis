use chrono::{DateTime, Utc};
use serde_derive::Deserialize;
use serde_derive::Serialize;

use crate::stats::models::{ServerSourceStats, ServerTargetStats};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct ServerSource {
    pub id: i32,
    pub description: String,
    pub path_starts_with: String,
    pub method: String,
    pub created: DateTime<Utc>,
    pub targets: Vec<ServerTarget>,
    pub stats: ServerSourceStats,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
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

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Server2Target {
    pub id: i32,
    pub source_id: i32,
    pub target_id: i32,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct NewServerSourcePost {
    pub description: String,
    pub path_starts_with: String,
    pub method: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct NewServerSource<'a> {
    pub description: &'a str,
    pub path_starts_with: &'a str,
    pub method: &'a str,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
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

#[derive(Deserialize, Serialize, Debug)]
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
