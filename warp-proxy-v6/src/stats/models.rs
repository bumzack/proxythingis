// https://morioh.com/p/47f04c30ffd7

use chrono::{DateTime, Utc};
use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct ServerSourceStats {
    pub id: i32,
    pub source_id: i32,
    pub hits: i64,
    pub start: DateTime<Utc>,
    pub stop: DateTime<Utc>,
    pub created: DateTime<Utc>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
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

impl Default for ServerTargetStats {
    fn default() -> Self {
        ServerTargetStats {
            id: 0,
            target_id: 0,
            hits: 0,
            avg_ns: 0,
            max_ns: 0,
            min_ns: 99999999,
            start: Default::default(),
            stop: Default::default(),
            created: Default::default(),
        }
    }
}

impl Default for ServerSourceStats {
    fn default() -> Self {
        ServerSourceStats {
            id: 0,
            source_id: 0,
            hits: 0,
            start: Default::default(),
            stop: Default::default(),
            created: Default::default(),
        }
    }
}
