use std::convert::Infallible;

use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use r2d2::Pool;
use serde::Serialize;
use warp::{Rejection, Reply};
use warp::http::StatusCode;
use warp::reply::json;

use crate::db::{create_person, read_persons};
use crate::models::NewPersonPost;

pub async fn create_person_handler(
    body: NewPersonPost,
    pool: Pool<ConnectionManager<PgConnection>>,
) -> Result<impl Reply, Infallible> {
    create_person(pool, &body.firstname, &body.lastname)
}

pub async fn list_person_handler(
    pool: Pool<ConnectionManager<PgConnection>>,
) -> Result<impl Reply, Infallible> {
    Ok(json(&read_persons(pool)))
}

pub async fn health_handler(
    pool: Pool<ConnectionManager<PgConnection>>,
) -> Result<impl Reply, Rejection> {
    let _client = pool.get().unwrap();

    // info!("hello from healthhandler");

    // TODO
    // client
    //     .execute("SELECT 1", &[])
    //     .map_err(|e| reject::custom(DBQueryError(e)))?;
    Ok(StatusCode::OK)
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub message: String,
}
