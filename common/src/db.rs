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
