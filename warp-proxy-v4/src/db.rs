// https://morioh.com/p/47f04c30ffd7

use std::collections::HashMap;
use std::env;
use std::ops::Add;

use chrono::{DateTime, Utc};
use deadpool_postgres::{Manager, ManagerConfig, Pool, RecyclingMethod};
use dotenvy::dotenv;
use tokio_postgres::NoTls;
use warp::Rejection;

use crate::models::{NewServerSourcePost, NewServerTargetPost, Server2Target, ServerSource, ServerSourceStats, ServerTarget};
use crate::models::MyError::DBQueryError;

const TABLE_SOURCE: &str = "source";
const TABLE_TARGET: &str = "target";
const TABLE_SOURCE2TARGET: &str = "source2target";
const TABLE_SOURCE_STATS: &str = "source_stats";
const TABLE_TARGET_STATS: &str = "target_stats";

type Result<T> = std::result::Result<T, Rejection>;

pub fn create_pool() -> Pool {
    dotenv().ok();
    let mut pg_config = tokio_postgres::Config::new();

    pg_config.user(env::var("DBUSER").unwrap().as_str());
    pg_config.password(env::var("DBPASSWORD").unwrap().as_str());
    pg_config.host(env::var("DBHOSTNAME").unwrap().as_str());
    pg_config.dbname(env::var("DBNAME").unwrap().as_str());
    let mgr_config = ManagerConfig {
        recycling_method: RecyclingMethod::Fast
    };
    let mgr = Manager::from_config(pg_config, NoTls, mgr_config);
    let pool = Pool::builder(mgr).max_size(16).build().unwrap();
    pool
}

pub async fn create_source(pool: Pool, body: NewServerSourcePost) -> Result<ServerSource> {
    let client = pool.get().await.unwrap();
    let query = format!("INSERT INTO {} (description, path_starts_with, method) VALUES ($1, $2, $3) RETURNING *", TABLE_SOURCE);
    // println!("new server source {:?}", &body);
    // println!("query   {}", &query);
    let row = client
        .query_one(query.as_str(), &[&body.description, &body.path_starts_with, &body.method])
        .await
        .map_err(DBQueryError)?;
    let server_source = ServerSource::from(row);
    Ok(server_source)
}

pub async fn create_target(pool: Pool, body: NewServerTargetPost) -> Result<ServerTarget> {
    let client = pool.get().await.unwrap();
    let query = format!("INSERT INTO {} (description, schema, host, port, path, method, active) VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING *", TABLE_TARGET);
    // println!("new server target {:?}", &body);
    // println!("query   {}", &query);
    let row = client
        .query_one(query.as_str(), &[&body.description, &body.schema, &body.host, &body.port, &body.path, &body.method, &body.active])
        .await
        .map_err(DBQueryError)?;
    let server_target = ServerTarget::from(row);

    // add cross table entry
    let _source_2_target = create_source2target(pool.clone(), body.source, server_target.id).await.unwrap();
    // println!("source_2_target   {:?}", &source_2_target);

    Ok(server_target)
}

pub async fn create_source2target(pool: Pool, source_id: i32, target_id: i32) -> Result<Server2Target> {
    let client = pool.get().await.unwrap();
    let query = format!("INSERT INTO {} (source_id, target_id) VALUES ($1, $2) RETURNING *", TABLE_SOURCE2TARGET);
    // println!("new source -> target  {:?} -> {:?}", source_id, target_id);
    // println!("query   {}", &query);
    let row = client
        .query_one(query.as_str(), &[&source_id, &target_id])
        .await
        .map_err(DBQueryError)?;
    let server_source = Server2Target::from(row);
    Ok(server_source)
}

pub async fn list_server(pool: Pool, active_only: bool) -> Result<Vec<ServerSource>> {
    let client = pool.get().await.unwrap();
    // source server

    let query1 = format!("SELECT {}.id AS source_id, {}.description AS source_description, {}.path_starts_with AS source_path_starts_with , {}.method AS source_method , {}.created AS source_created, ", TABLE_SOURCE, TABLE_SOURCE, TABLE_SOURCE, TABLE_SOURCE, TABLE_SOURCE);
    let query2 = format!("{}.id AS target_id, {}.description AS target_description,   {}.schema AS target_schema,  {}.host AS target_host,  {}.port AS target_port ,  {}.path AS target_path ,  {}.method AS target_method ,  {}.active AS target_active ,{}.created AS target_created      FROM {} ", TABLE_TARGET, TABLE_TARGET, TABLE_TARGET, TABLE_TARGET, TABLE_TARGET, TABLE_TARGET, TABLE_TARGET, TABLE_TARGET, TABLE_TARGET, TABLE_SOURCE2TARGET);
    let query3 = format!(" LEFT JOIN {} ON ({}.source_id = {}.id) ", TABLE_SOURCE, TABLE_SOURCE2TARGET, TABLE_SOURCE);
    let query4 = format!(" LEFT JOIN {} ON ({}.target_id = {}.id) ", TABLE_TARGET, TABLE_SOURCE2TARGET, TABLE_TARGET);

    // println!("query1 {}", &query1);
    // println!("query2 {}", &query2);
    // println!("query3 {}", &query3);
    // println!("query4 {}", &query4);

    let mut map: HashMap<i32, ServerSource> = HashMap::new();

    let query_full = query1.add(&query2).add(&query3).add(&query4);
    // println!("query   {}", &query_full);
    let data = client.query(&query_full, &[]).await.unwrap();
    for row in data {
        let source_id: i32 = row.get("source_id");
        let source_description: &str = row.get("source_description");
        let source_path_starts_with: &str = row.get("source_path_starts_with");
        let source_method: &str = row.get("source_method");
        let source_created: DateTime<Utc> = row.get("source_created");

        let target_id: i32 = row.get("target_id");
        let target_description: &str = row.get("target_description");
        let target_schema: &str = row.get("target_schema");
        let target_host: &str = row.get("target_host");
        let target_port: i32 = row.get("target_port");
        let target_path: &str = row.get("target_path");
        let target_method: &str = row.get("target_method");
        let target_active: bool = row.get("target_active");
        let target_created: DateTime<Utc> = row.get("target_created");


        // println!("found server source: {} {} {} {:?}", source_id, source_description, source_method, source_path_starts_with);
        // println!("\tfound server target: {} {} {} {:?} {} {} {:?}", target_id, target_description, target_schema, target_port, target_path, target_method, target_active);


        let mut server_source = ServerSource {
            id: source_id,
            description: source_description.to_string(),
            path_starts_with: source_path_starts_with.to_string(),
            method: source_method.to_string(),
            created: source_created,
            targets: vec![],
            stats: Default::default(),
        };

        let server_target = ServerTarget {
            id: target_id,
            description: target_description.to_string(),
            schema: target_schema.to_string(),
            host: target_host.to_string(),
            port: target_port,
            path: target_path.to_string(),
            method: target_method.to_string(),
            active: target_active,
            stats: Default::default(),
            created: target_created,
        };

        if active_only {
            if server_target.active {
                add_to_map(&mut map, &mut server_source, server_target);
            }
        } else {
            add_to_map(&mut map, &mut server_source, server_target);
        }
    }

    let sources: Vec<ServerSource> = map.values()
        .cloned()
        .collect();

    Ok(sources)
}

fn add_to_map(map: &mut HashMap<i32, ServerSource>, server_source: &ServerSource, server_target: ServerTarget) {
    if map.contains_key(&server_source.id) {
        let s = map.get_mut(&server_source.id).unwrap();
        s.targets.push(server_target);
    } else {
        let mut server_source = server_source.clone();
        server_source.targets.push(server_target);
        map.insert(server_source.id, server_source);
    }
}

pub async fn activate_server(pool: Pool, id: i32) -> Result<()> {
    change_activate_server(pool, id, true).await
}

pub async fn deactivate_server(pool: Pool, id: i32) -> Result<()> {
    change_activate_server(pool, id, false).await
}

pub async fn change_activate_server(pool: Pool, id: i32, val: bool) -> Result<()> {
    println!("(de-)activating server {}. val {}", id,val);
    let client = pool.get().await.unwrap();
    let query = format!("UPDATE  {}  SET active= {} WHERE  id = $1 RETURNING *", TABLE_TARGET, val);

    let _row = client
        .query_one(query.as_str(), &[&id])
        .await
        .map_err(DBQueryError)?;
    Ok(())
}

pub async fn create_source_stats(pool: Pool, source_id: i32, hits: u32, start: DateTime<Utc>, stop: DateTime<Utc>) -> Result<ServerSourceStats> {
    let client = pool.get().await.unwrap();
    let query = format!("INSERT INTO {} (hits, source_id, start, stop) VALUES ($1, $2, $3, $4) RETURNING *", TABLE_SOURCE_STATS);
    let row = client
        .query_one(query.as_str(), &[&hits, &source_id, &start, &stop])
        .await
        .map_err(DBQueryError)?;
    let server_source_stats = ServerSourceStats::from(row);

    Ok(server_source_stats)
}

pub async fn create_target_stats(pool: Pool, target_id: i32, hits: u32, min_ns: u32, max_ns: u32, avg_ns: u32, start: DateTime<Utc>, stop: DateTime<Utc>) -> Result<ServerSourceStats> {
    let client = pool.get().await.unwrap();
    let query = format!("INSERT INTO {} (hits, target_id, start, stop, min_ns, max_ns, avg_ns) VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING *", TABLE_TARGET_STATS);
    let row = client
        .query_one(query.as_str(), &[&hits, &target_id, &start, &stop, &min_ns, &max_ns, &avg_ns])
        .await
        .map_err(DBQueryError)?;
    let server_source_stats = ServerSourceStats::from(row);

    Ok(server_source_stats)
}
