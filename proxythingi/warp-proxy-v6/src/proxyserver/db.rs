use std::collections::HashMap;
use std::ops::Add;

use chrono::{DateTime, Utc};
use deadpool_postgres::Pool;

use common::models::{
    NewServerSourcePost, NewServerTargetPost, Server2Target, ServerSource, ServerTarget,
};

use crate::db::db::{TABLE_SOURCE, TABLE_SOURCE2TARGET, TABLE_TARGET};
use crate::server::models::MyError::DBQueryError;
use crate::server::server::Result;

pub async fn create_source(pool: Pool, body: NewServerSourcePost) -> Result<ServerSource> {
    let client = pool.get().await.unwrap();
    let query = format!(
        "INSERT INTO {} (description, path_starts_with, method) VALUES ($1, $2, $3) RETURNING *",
        TABLE_SOURCE
    );
    let row = client
        .query_one(
            query.as_str(),
            &[&body.description, &body.path_starts_with, &body.method],
        )
        .await
        .map_err(DBQueryError)?;
    let server_source = ServerSource::from(row);
    Ok(server_source)
}

pub async fn create_target(pool: Pool, body: NewServerTargetPost) -> Result<ServerTarget> {
    let client = pool.get().await.unwrap();
    let query = format!("INSERT INTO {} (description, schema, host, port, path, method, active) VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING *", TABLE_TARGET);
    let row = client
        .query_one(
            query.as_str(),
            &[
                &body.description,
                &body.schema,
                &body.host,
                &body.port,
                &body.path,
                &body.method,
                &body.active,
            ],
        )
        .await
        .map_err(DBQueryError)?;
    let server_target = ServerTarget::from(row);

    // add cross table entry
    let _source_2_target = create_source2target(pool.clone(), body.source, server_target.id)
        .await
        .unwrap();

    Ok(server_target)
}

pub async fn create_source2target(
    pool: Pool,
    source_id: i32,
    target_id: i32,
) -> Result<Server2Target> {
    let client = pool.get().await.unwrap();
    let query = format!(
        "INSERT INTO {} (source_id, target_id) VALUES ($1, $2) RETURNING *",
        TABLE_SOURCE2TARGET
    );
    let row = client
        .query_one(query.as_str(), &[&source_id, &target_id])
        .await
        .map_err(DBQueryError)?;
    let server_source = Server2Target::from(row);
    Ok(server_source)
}

pub async fn list_server(pool: Pool) -> Result<Vec<ServerSource>> {
    let client = pool.get().await.unwrap();
    // source server

    let query1 = format!("SELECT {}.id AS source_id, {}.description AS source_description, {}.path_starts_with AS source_path_starts_with , {}.method AS source_method , {}.created AS source_created, ", TABLE_SOURCE, TABLE_SOURCE, TABLE_SOURCE, TABLE_SOURCE, TABLE_SOURCE);
    let query2 = format!("{}.id AS target_id, {}.description AS target_description,   {}.schema AS target_schema,  {}.host AS target_host,  {}.port AS target_port ,  {}.path AS target_path ,  {}.method AS target_method ,  {}.active AS target_active ,{}.created AS target_created      FROM {} ", TABLE_TARGET, TABLE_TARGET, TABLE_TARGET, TABLE_TARGET, TABLE_TARGET, TABLE_TARGET, TABLE_TARGET, TABLE_TARGET, TABLE_TARGET, TABLE_SOURCE2TARGET);
    let query3 = format!(
        " LEFT JOIN {} ON ({}.source_id = {}.id) ",
        TABLE_SOURCE, TABLE_SOURCE2TARGET, TABLE_SOURCE
    );
    let query4 = format!(
        " LEFT JOIN {} ON ({}.target_id = {}.id) ",
        TABLE_TARGET, TABLE_SOURCE2TARGET, TABLE_TARGET
    );

    let mut map: HashMap<i32, ServerSource> = HashMap::new();

    let query_full = query1.add(&query2).add(&query3).add(&query4);
    // // info!("query   {}", &query_full);
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

        let server_source = ServerSource {
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

        add_to_map(&mut map, &server_source, server_target);
    }

    let sources: Vec<ServerSource> = map.values().cloned().collect();

    Ok(sources)
}

fn add_to_map(
    map: &mut HashMap<i32, ServerSource>,
    server_source: &ServerSource,
    server_target: ServerTarget,
) {
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
    // info!("(de-)activating server {}. val {}", id, val);
    let client = pool.get().await.unwrap();
    let query = format!(
        "UPDATE  {}  SET active= {} WHERE  id = $1 RETURNING *",
        TABLE_TARGET, val
    );

    let _row = client
        .query_one(query.as_str(), &[&id])
        .await
        .map_err(DBQueryError)?;
    Ok(())
}
