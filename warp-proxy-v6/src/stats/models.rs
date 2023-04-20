// https://morioh.com/p/47f04c30ffd7

use chrono::{DateTime, Utc};
use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct ServerSourceStats {
    pub id: i32,
    pub source_id: i32,
    pub hits: i64,
    pub start: DateTime<Utc>,
    pub stop: DateTime<Utc>,
    pub created: DateTime<Utc>,
}

#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct ServerTargetStats {
    pub id: i32,
    pub target_id: i32,
    pub hits: i64,
    pub avg_ns: i64,
    pub max_ns: i64,
    pub min_ns: i64,
    pub start: DateTime<Utc>,
    pub stop: DateTime<Utc>,
    pub created: DateTime<Utc>,
}
