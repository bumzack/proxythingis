use tokio_postgres::Row;

use crate::models::{
    Server2Target, ServerSource, ServerSourceStats, ServerTarget, ServerTargetStats,
};

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

impl From<Row> for ServerSourceStats {
    fn from(value: Row) -> Self {
        let hits: i64 = value.get("hits");
        ServerSourceStats {
            id: value.get("id"),
            source_id: value.get("source_id"),
            hits: hits as i128,
            start: value.get("start"),
            stop: value.get("stop"),
            created: value.get("created"),
        }
    }
}

impl From<Row> for ServerTargetStats {
    fn from(value: Row) -> Self {
        let hits: i64 = value.get("hits");
        let avg_ns: i64 = value.get("avg_ns");
        let max_ns: i64 = value.get("max_ns");
        let min_ns: i64 = value.get("min_ns");
        ServerTargetStats {
            id: value.get("id"),
            target_id: value.get("target_id"),
            hits: hits as i128,
            avg_ns: avg_ns as i128,
            max_ns: max_ns as i128,
            min_ns: min_ns as i128,
            start: value.get("start"),
            stop: value.get("stop"),
            created: value.get("created"),
        }
    }
}
