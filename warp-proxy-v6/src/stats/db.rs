use chrono::{DateTime, Utc};
use deadpool_postgres::Pool;
use tokio_postgres::Row;

use crate::db::db::{TABLE_SOURCE_STATS, TABLE_TARGET_STATS};
use crate::server::models::MyError::DBQueryError;
use crate::server::server::Result;
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

pub async fn create_source_stats(
    pool: Pool,
    source_id: i32,
    hits: i64,
    start: DateTime<Utc>,
    stop: DateTime<Utc>,
) -> Result<ServerSourceStats> {
    info!("inserting stats into DB 'create_source_stats'");
    let client = pool.get().await.unwrap();
    let query = format!(
        "INSERT INTO {} (hits, source_id, start, stop) VALUES ($1, $2, $3, $4) RETURNING *",
        TABLE_SOURCE_STATS
    );
    let row = client
        .query_one(query.as_str(), &[&hits, &source_id, &start, &stop])
        .await
        .map_err(DBQueryError)?;
    let server_source_stats = ServerSourceStats::from(row);

    Ok(server_source_stats)
}

pub async fn create_target_stats(
    pool: Pool,
    target_id: i32,
    hits: i64,
    min_ns: i64,
    max_ns: i64,
    avg_ns: i64,
    start: DateTime<Utc>,
    stop: DateTime<Utc>,
) -> Result<ServerTargetStats> {
    info!("inserting stats into DB 'create_target_stats'");
    let client = pool.get().await.unwrap();
    let query = format!("INSERT INTO {} (hits, target_id, start, stop, min_ns, max_ns, avg_ns) VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING *", TABLE_TARGET_STATS);
    let row = client
        .query_one(
            query.as_str(),
            &[&hits, &target_id, &start, &stop, &min_ns, &max_ns, &avg_ns],
        )
        .await
        .map_err(DBQueryError)?;
    let server_source_stats = ServerTargetStats::from(row);

    Ok(server_source_stats)
}
