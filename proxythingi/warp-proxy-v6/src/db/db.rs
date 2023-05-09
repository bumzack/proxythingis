use std::env;

use deadpool_postgres::{Manager, ManagerConfig, Pool, RecyclingMethod};
use dotenvy::dotenv;
use tokio_postgres::NoTls;

pub const TABLE_SOURCE: &str = "source";
pub const TABLE_TARGET: &str = "target";
pub const TABLE_SOURCE2TARGET: &str = "source2target";
pub const TABLE_SOURCE_STATS: &str = "source_stats";
pub const TABLE_TARGET_STATS: &str = "target_stats";

pub fn create_pool() -> Pool {
    dotenv().ok();
    let mut pg_config = tokio_postgres::Config::new();

    pg_config.user(env::var("DBUSER").unwrap().as_str());
    pg_config.password(env::var("DBPASSWORD").unwrap().as_str());
    pg_config.host(env::var("DBHOSTNAME").unwrap().as_str());
    pg_config.dbname(env::var("DBNAME").unwrap().as_str());
    let mgr_config = ManagerConfig {
        recycling_method: RecyclingMethod::Fast,
    };
    let mgr = Manager::from_config(pg_config, NoTls, mgr_config);
    let pool = Pool::builder(mgr).max_size(16).build().unwrap();
    pool
}
