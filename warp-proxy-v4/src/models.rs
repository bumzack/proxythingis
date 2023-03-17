// https://morioh.com/p/47f04c30ffd7

use chrono::{DateTime, Utc};
use serde_derive::Deserialize;
use serde_derive::Serialize;
use thiserror::Error;
use tokio_postgres::Row;
use warp::reject;

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
pub struct ServerSourceStats {
    pub hits: u128,
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
pub struct ServerTargetStats {
    pub hits: u128,
    pub avg_ns: u128,
    pub max_ns: u128,
    pub min_ns: u128,
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

impl From<Row> for ServerSource {
    fn from(value: Row) -> Self {
        ServerSource {
            id: value.get(0),
            description: value.get(1),
            path_starts_with: value.get(2),
            method: value.get(3),
            created: value.get(4),
            targets: vec![],
            stats: ServerSourceStats::default(),
        }
    }
}

impl From<Row> for ServerTarget {
    fn from(value: Row) -> Self {
        ServerTarget {
            id: value.get(0),
            description: value.get(1),
            schema: value.get(2),
            host: value.get(3),
            port: value.get(4),
            method: value.get(5),
            path: value.get(6),
            active: value.get(7),
            stats: ServerTargetStats::default(),
            created: Default::default(),
        }
    }
}

impl From<Row> for Server2Target {
    fn from(value: Row) -> Self {
        Server2Target {
            id: value.get(0),
            source_id: value.get(1),
            target_id: value.get(2),
        }
    }
}


impl Default for ServerTargetStats {
    fn default() -> Self {
        ServerTargetStats {
            hits: 0,
            avg_ns: 0,
            max_ns: 0,
            min_ns: 99999999,
        }
    }
}

impl Default for ServerSourceStats {
    fn default() -> Self {
        ServerSourceStats {
            hits: 0,
        }
    }
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub message: String,
}


#[derive(Error, Debug)]
pub enum MyError {
    #[error("error executing DB query: {0}")]
    DBQueryError(#[from] tokio_postgres::Error),
    // #[error("error creating table: {0}")]
    // DBInitError(tokio_postgres::Error),
    // #[error("error reading file: {0}")]
    // ReadFileError(#[from] std::io::Error),
}

impl warp::reject::Reject for MyError {}

// TODO: hihihii
#[derive(Debug)]
pub struct DivideByZero;

impl reject::Reject for DivideByZero {}



