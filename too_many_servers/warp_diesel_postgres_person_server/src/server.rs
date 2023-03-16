// https://morioh.com/p/47f04c30ffd7

use std::{env, fs};
use std::convert::Infallible;

use deadpool_postgres::Pool;
use serde::Serialize;
use tokio_postgres::NoTls;
use warp::{reject, Rejection, Reply};
use warp::http::StatusCode;
use warp::reply::json;

use crate::db;
use crate::db::{create_person, list_person};
use crate::models::{DivideByZero, ErrorResponse, MyError, PersonRequest, PersonResponse};
use crate::models::MyError::DBQueryError;

pub async fn create_person_handler(body: PersonRequest, pool: Pool) -> Result<impl Reply> {
    Ok(json(&PersonResponse::of(
        create_person(pool, body)
            .await
            // TODO fix CustomError
            .map_err(|e| {
                println!("error rejection {:?}", e);
                reject::custom(DivideByZero)
            })?,
    )))
}


pub async fn list_person_handler(pool: Pool, limit: u32) -> Result<impl Reply> {
    let data = list_person(pool, limit)
        .await
        // TODO fix CustomError
        .map_err(|e| {
            println!("error rejection {:?}", e);
            reject::custom(DivideByZero)
        })?;

    let res: Vec<PersonResponse> = data.iter().map(|p| {
        PersonResponse::of(p.clone())
    }).collect();
    Ok(json(&res))
}


pub async fn health_handler(pool: Pool) -> std::result::Result<impl Reply, Rejection> {
    let mut client = pool.get().await.unwrap();

    println!("hello from healthhandler");
    client
        .execute("SELECT 1", &[])
        .await
        .map_err(|e| reject::custom(DBQueryError(e)))?;
    Ok(StatusCode::OK)
}

type Result<T> = std::result::Result<T, Rejection>;


pub async fn handle_rejection(err: Rejection) -> std::result::Result<impl Reply, Infallible> {
    let code;
    let message;

    if err.is_not_found() {
        code = StatusCode::NOT_FOUND;
        message = "Not Found";
    } else if let Some(_) = err.find::<warp::filters::body::BodyDeserializeError>() {
        code = StatusCode::BAD_REQUEST;
        message = "Invalid Body";
    } else if let Some(e) = err.find::<MyError>() {
        match e {
            DBQueryError(_) => {
                code = StatusCode::BAD_REQUEST;
                message = "Could not Execute request";
            }
            _ => {
                eprintln!("unhandled application error: {:?}", err);
                code = StatusCode::INTERNAL_SERVER_ERROR;
                message = "Internal Server Error";
            }
        }
    } else if let Some(_) = err.find::<warp::reject::MethodNotAllowed>() {
        code = StatusCode::METHOD_NOT_ALLOWED;
        message = "Method Not Allowed";
    } else {
        eprintln!("unhandled error: {:?}", err);
        code = StatusCode::INTERNAL_SERVER_ERROR;
        message = "Internal Server Error";
    }

    let json = json(&ErrorResponse {
        message: message.into(),
    });

    Ok(warp::reply::with_status(json, code))
}



