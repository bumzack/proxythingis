use chrono::{DateTime, Utc};
use serde_derive::Deserialize;
use serde_derive::Serialize;
use thiserror::Error;
use warp::reject;

#[derive(Deserialize, Clone)]
pub struct Person {
    pub id: i32,
    pub firstname: String,
    pub lastname: String,
    pub created: DateTime<Utc>,
}

#[derive(Deserialize, Debug)]
pub struct PersonRequest {
    pub firstname: String,
    pub lastname: String,
}

#[derive(Serialize)]
pub struct PersonResponse {
    pub id: i32,
    pub firstname: String,
    pub lastname: String,
}

impl PersonResponse {
    pub fn of(p: Person) -> PersonResponse {
        PersonResponse {
            id: p.id,
            firstname: p.firstname,
            lastname: p.lastname,
        }
    }
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub message: String,
}

#[derive(Error, Debug)]
pub enum MyError {
    #[error("error executing DB query: {0}")]
    DBQueryError(#[from] tokio_postgres::Error),
    // #[error("error creating table: {0}")]
    // DBInitError(tokio_postgres::Error),
    #[error("error reading file: {0}")]
    ReadFileError(#[from] std::io::Error),
}

impl warp::reject::Reject for MyError {}

// TODO: hihihii
#[derive(Debug)]
pub struct DivideByZero;

impl reject::Reject for DivideByZero {}



// TODO: hihihii
#[derive(Debug)]
pub struct InternalError;

impl reject::Reject for InternalError {}
