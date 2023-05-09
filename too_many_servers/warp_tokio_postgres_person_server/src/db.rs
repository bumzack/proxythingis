// https://morioh.com/p/47f04c30ffd7

use std::convert::Infallible;
use std::env;

use chrono::{DateTime, Utc};
use deadpool_postgres::{Manager, ManagerConfig, Pool, RecyclingMethod};
use dotenvy::dotenv;
use log::{error, info};
use tokio_postgres::{NoTls, Row};
use warp::http::StatusCode;
use warp::Filter;
use warp::Rejection;

use crate::models::MyError::DBQueryError;
use crate::models::{DivideByZero, InternalError, Person, PersonRequest};

type Result<T> = std::result::Result<T, Rejection>;

const TABLE: &str = "person";

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

pub fn with_db(pool: Pool) -> impl Filter<Extract = (Pool,), Error = Infallible> + Clone {
    warp::any().map(move || pool.clone())
}

pub async fn create_person(pool: Pool, body: PersonRequest) -> Result<Person> {
    let client = pool.get().await.unwrap();
    let query = format!(
        "INSERT INTO {} (firstname, lastname) VALUES ($1, $2) RETURNING *",
        TABLE
    );
    // // info!("person {:?}", &body);
    // // info!("query   {}", &query);
    let row = client
        .query_one(query.as_str(), &[&body.firstname, &body.lastname])
        .await
        .map_err(DBQueryError)?;
    let p = Person::from(row);

    Ok(p)
}

pub async fn list_person(pool: Pool, limit: u32) -> Result<Vec<Person>> {
    let mut persons = vec![];
    let client = pool.get().await;
    if client.is_err() {
        let e = client.as_ref().err();
        let e = e.unwrap();
        let msg = format!("error getting client {:?} ", e);
        error!("err {}", &msg);
        let res = warp::reject::custom(InternalError);
        return Err(res);
    }
    let client = client.unwrap();

    let query = format!(
        "SELECT id, firstname, lastname, created FROM {} ORDER BY lastname DESC LIMIT {}",
        TABLE, limit
    );
    // // info!("query   {}", &query);
    let data = client.query(&query, &[]).await;
    if data.is_err() {
        let e = data.as_ref().err();
        let e = e.unwrap();
        let msg = format!("error reading from DB  {:?} ", e);
        error!("err {}", &msg);
        let res = warp::reject::custom(InternalError);
        return Err(res);
    }
    let data = data.unwrap();

    for row in data {
        let id: i32 = row.get(0);
        let firstname: &str = row.get(1);
        let lastname: &str = row.get(2);
        let created: DateTime<Utc> = row.get(3);

        // // info!("found person: {} {} {} {:?}", id, firstname, lastname, created);
        let p = Person {
            id,
            firstname: firstname.to_string(),
            lastname: lastname.to_string(),
            created,
        };
        persons.push(p);
    }
    // info!("found {} persons", persons.len());
    Ok(persons)
}

impl From<Row> for Person {
    fn from(row: Row) -> Self {
        let id: i32 = row.get(0);
        let firstname: String = row.get(1);
        let lastname: String = row.get(2);
        let created: DateTime<Utc> = row.get(3);
        Self {
            id,
            firstname,
            lastname,
            created,
        }
    }
}
