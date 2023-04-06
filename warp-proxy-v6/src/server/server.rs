use deadpool_postgres::Pool;
use warp::http::StatusCode;
use warp::{reject, Rejection, Reply};

use crate::server::models::MyError::DBQueryError;
use crate::server::models::{DivideByZero, MyError};

pub type Result<T> = std::result::Result<T, Rejection>;

pub async fn health_handler(pool: Pool) -> std::result::Result<impl Reply, Rejection> {
    let client = pool.get().await.unwrap();

    info!("hello from healthhandler");
    client
        .execute("SELECT 1", &[])
        .await
        .map_err(|e| reject::custom(DBQueryError(e)))?;
    Ok(StatusCode::OK)
}

impl warp::reject::Reject for MyError {}

impl reject::Reject for DivideByZero {}

// pub async fn handle_rejection(err: Rejection) -> std::result::Result<impl Reply, Infallible> {
//     let code;
//     let message;
//
//     if err.is_not_found() {
//         code = StatusCode::NOT_FOUND;
//         message = "Not Found";
//     } else if let Some(_) = err.find::<warp::filters::body::BodyDeserializeError>() {
//         code = StatusCode::BAD_REQUEST;
//         message = "Invalid Body";
//     } else if let Some(e) = err.find::<MyError>() {
//         match e {
//             DBQueryError(_) => {
//                 code = StatusCode::BAD_REQUEST;
//                 message = "Could not Execute request";
//             }
//             // _ => {
//             //     eprintln!("unhandled application error: {:?}", err);
//             //     code = StatusCode::INTERNAL_SERVER_ERROR;
//             //     message = "Internal Server Error";
//             // }
//         }
//     } else if let Some(_) = err.find::<warp::reject::MethodNotAllowed>() {
//         code = StatusCode::METHOD_NOT_ALLOWED;
//         message = "Method Not Allowed";
//     } else {
//         eprintln!("unhandled error: {:?}", err);
//         code = StatusCode::INTERNAL_SERVER_ERROR;
//         message = "Internal Server Error";
//     }
//
//     let json = json(&ErrorResponse {
//         message: message.into(),
//     });
//
//     Ok(warp::reply::with_status(json, code))
// }
