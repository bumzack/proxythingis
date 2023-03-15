// https://morioh.com/p/47f04c30ffd7

use std::{env, fs};
use std::convert::Infallible;
use std::str::FromStr;
use std::time::Duration;
use chrono::{DateTime, Utc};

use serde_derive::Deserialize;
use serde_derive::Serialize;
use tokio_postgres::{Config, Connection, NoTls, Row};
use warp::{reject, Rejection, Reply};
use warp::Filter;
use warp::http::StatusCode;
use warp::reply::json;

pub async fn get_db_con(db_pool: &DBPool) -> Result<DBCon> {
    db_pool.get().await.map_err(DBPoolError)
}

pub async fn init_db(db_pool: &DBPool) -> Result<()> {
    let init_file = fs::read_to_string(INIT_SQL)?;
    let con = get_db_con(db_pool).await?;
    con
        .batch_execute(init_file.as_str())
        .await
        .map_err(DBInitError)?;
    Ok(())
}


fn with_db(db_pool: DBPool) -> impl Filter<Extract=(DBPool, ), Error=Infallible> + Clone {
    warp::any().map(move || db_pool.clone())
}

const TABLE: &str = "person";

pub async fn create_person(db_pool: &DBPool, body: PersonRequest) -> Result<Person> {
    let con = get_db_con(db_pool).await?;
    let query = format!("INSERT INTO {} (firstname, lastname) VALUES ($1, $2) RETURNING *", TABLE);
    let row = con
        .query_one(query.as_str(), &[&body.firstname, body.lastname])
        .await
        .map_err(DBQueryError)?;
    let p = row.into_iter().map(|r| Person::from(r)).collect();

    Ok(p)
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


#[derive(Deserialize)]
pub struct Person {
    pub id: i32,
    pub firstname: String,
    pub lastname: String,
    pub created: DateTime<Utc>,
}

#[derive(Deserialize)]
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

pub async fn create_person_handler(body: PersonRequest, db_pool: DBPool) -> Result<impl Reply> {
    Ok(json(&TodoResponse::of(
        create_person(&db_pool, body)
            .await
            .map_err(|e| reject::custom(e))?,
    )))
}


#[tokio::main]
async fn main() -> Result<()> {
    if env::var_os("RUST_LOG").is_none() {
        // Set `RUST_LOG=todos=debug` to see debug logs,
        // this only shows access logs.
        env::set_var("RUST_LOG", "response_to_everything=info");
    }

    let (client, connection) =
        tokio_postgres::connect("host=localhost user=postgres", NoTls).await?;

    // The connection object performs the actual communication with the database,
    // so spawn it off to run on its own.
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });



    persons = vec![];
    for row in client.query("SELECT id, firstname, lastname, created FROM person ORDER BY lastname DESC", &[])? {
        let id: i32 = row.get(0);
        let firstname: &str = row.get(1);
        let lastname: &str = row.get(2);
        let created: DateTime<Utc> = row.get(1);

        println!("found person: {} {} {} {:?}", id, firstname, lastname, created);
        let p = Person {
            id,
            firstname: firstname.to_string(),
            lastname: lastname.to_string(),
            created,
        };
        persons.push(p);
    }


    let health_route = warp::path!("health")
        .and(with_db(db_pool.clone()))
        .and_then(health_handler);

    let routes = health_route
        .with(warp::cors().allow_any_origin());

    let person = warp::path("person");
    let person_routes = person
        .and(warp::post())
        .and(warp::body::json())
        .and(with_db(db_pool.clone()))
        .and_then(create_person_handler);

    let routes = health_route
        .or(person_routes)
        .with(warp::cors().allow_any_origin())
        .recover(handle_rejection);

    warp::serve(routes).run(([127, 0, 0, 1], 3041)).await;
    Ok(())

    //
    // let routes = warp::any()
    //     .map(warp::reply);
    //
    // warp::serve(routes)
    //     .run(([127, 0, 0, 1], 3040))
    //     .await;

    // // Connect to the database.
    // let (client, connection) =
    //     tokio_postgres::connect("host=localhost user=bumzack", NoTls).await?;
    //
    // // The connection object performs the actual communication with the database,
    // // so spawn it off to run on its own.
    // tokio::spawn(async move {
    //     if let Err(e) = connection.await {
    //         eprintln!("connection error: {}", e);
    //     }
    // });
    //
    // // Now we can execute a simple statement that just returns its parameter.
    // let rows = client
    //     .query("SELECT $1::TEXT", &[&"hello world"])
    //     .await?;
    //
    // // And then check that we got back the same string we sent over.
    // let value: &str = rows[0].get(0);
    // assert_eq!(value, "hello world");
    //
    // Ok(())
}

pub async fn health_handler(db_pool: DBPool) -> std::result::Result<impl Reply, Rejection> {
    let db = db::get_db_con(&db_pool)
        .await
        .map_err(|e| reject::custom(e))?;

    db.execute("SELECT 1", &[])
        .await
        .map_err(|e| reject::custom(DBQueryError(e)))?;
    Ok(StatusCode::OK)
}

type Result<T> = std::result::Result<T, warp::Rejection>;


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
            MyError::DBQueryError(_) => {
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

    let json = warp::reply::json(&ErrorResponse {
        message: message.into(),
    });

    Ok(warp::reply::with_status(json, code))
}

