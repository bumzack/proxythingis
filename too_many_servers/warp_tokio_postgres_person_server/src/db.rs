// https://morioh.com/p/47f04c30ffd7

use std::{env, fs};
use std::convert::Infallible;
use std::str::FromStr;
use std::time::Duration;

use chrono::{DateTime, Utc};
use deadpool_postgres::{Manager, ManagerConfig, Pool, RecyclingMethod};
use dotenvy::dotenv;
use serde_derive::Deserialize;
use serde_derive::Serialize;
use thiserror::Error;
use tokio_postgres::{Client, Config, Connection, NoTls, Row};
use warp::{reject, Rejection, Reply};
use warp::Filter;
use warp::http::StatusCode;
use warp::reply::json;

use crate::models::{Person, PersonRequest};
use crate::models::MyError::DBQueryError;

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

pub fn with_db(pool: Pool) -> impl Filter<Extract=(Pool, ), Error=Infallible> + Clone {
    warp::any().map(move || pool.clone())
}

const TABLE: &str = "person";

pub async fn create_person(pool: Pool, body: PersonRequest) -> Result<Person> {
    let mut client = pool.get().await.unwrap();
    let query = format!("INSERT INTO {} (firstname, lastname) VALUES ($1, $2) RETURNING *", TABLE);
    println!("person {:?}", &body);
    println!("query   {}", &query);
    let row = client
        .query_one(query.as_str(), &[&body.firstname, &body.lastname])
        .await
        .map_err(DBQueryError)?;
    let p = Person::from(row);

    Ok(p)
}


pub async fn list_person(pool: Pool, limit:u32) -> Result<Vec<Person>> {
   let mut persons = vec![];
    let mut client = pool.get().await.unwrap();
    let query = format!("SELECT id, firstname, lastname, created FROM {} ORDER BY lastname DESC LIMIT {}", TABLE, limit);
    println!("query   {}", &query);
    let data = client.query(&query, &[]).await.unwrap();
    for row in data {
        let id: i32 = row.get(0);
        let firstname: &str = row.get(1);
        let lastname: &str = row.get(2);
        let created: DateTime<Utc> = row.get(3);

        println!("found person: {} {} {} {:?}", id, firstname, lastname, created);
        let p = Person {
            id,
            firstname: firstname.to_string(),
            lastname: lastname.to_string(),
            created,
        };
        persons.push(p);
    }
    Ok(persons)
}

impl From<Row> for Person {
    fn from(row: Row) -> Self {
        let id: i32 = row.get(0);
        let firstname: String = row.get(1);
        let lastname: String = row.get(2);
        let created: chrono::DateTime<chrono::offset::Utc> = row.get(3);
        Self {
            id,
            firstname,
            lastname,
            created,
        }
    }
}
