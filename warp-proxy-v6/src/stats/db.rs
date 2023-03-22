use tokio_postgres::Row;
use crate::stats::models::{ServerSourceStats, ServerTargetStats};

impl From<Row> for ServerSourceStats {
    fn from(value: Row) -> Self {
        ServerSourceStats {
            id: value.get("id"),
            source_id: value.get("source_id"),
            hits: value.get("hits"),
            start: value.get("start"),
            stop: value.get("stop"),
            created: value.get("created"),
        }
    }
}

impl From<Row> for ServerTargetStats {
    fn from(value: Row) -> Self {
        ServerTargetStats {
            id: value.get("id"),
            target_id: value.get("target_id"),
            hits: value.get("hits"),
            avg_ns: value.get("avg_ns"),
            max_ns: value.get("max_ns"),
            min_ns: value.get("min_ns"),
            start: value.get("start"),
            stop: value.get("stop"),
            created: value.get("created"),
        }
    }
}

