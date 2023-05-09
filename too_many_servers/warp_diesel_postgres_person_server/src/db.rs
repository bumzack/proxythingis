use std::convert::Infallible;
use std::env;

use diesel::r2d2::ConnectionManager;
use diesel::{PgConnection, RunQueryDsl};
use log::error;
use log::info;
use r2d2::Pool;
use warp::http::StatusCode;
use warp::Filter;

use crate::models::{ErrorMessage, NewPerson, Person};

pub fn read_persons(pool: Pool<ConnectionManager<PgConnection>>) -> Vec<Person> {
    use crate::schema::person::dsl::*;

    let connection = &mut pool.get().unwrap();

    let results = person
        .load::<Person>(connection)
        .expect("Error loading persons");

    // info!("Displaying {} persons", results.len());
    // for p in &results {
    //     info!("id {}:  {} {}, created at {}", p.id, p.firstname, p.lastname, p.created);
    // }
    results
}

pub fn create_person(
    pool: Pool<ConnectionManager<PgConnection>>,
    firstname: &str,
    lastname: &str,
) -> Result<impl warp::Reply, Infallible> {
    use crate::schema::person;
    let connection = &mut pool.get().unwrap();

    let new_person = NewPerson {
        firstname,
        lastname,
    };

    let res = diesel::insert_into(person::table)
        .values(&new_person)
        .execute(connection);

    match res {
        Ok(new_id) => {
            let message = format!("created with id {}", new_id);
            let code = StatusCode::CREATED;
            let json = warp::reply::json(&ErrorMessage {
                code: code.as_u16(),
                message: message.into(),
            });
            Ok(warp::reply::with_status(json, code))
        }
        Err(e) => {
            let message = format!(
                "an error occurred inserting a new person which we are ignoring '{}'",
                e
            );
            let code = StatusCode::INTERNAL_SERVER_ERROR;

            let json = warp::reply::json(&ErrorMessage {
                code: code.as_u16(),
                message: message.into(),
            });

            Ok(warp::reply::with_status(json, code))
        }
    }
}

fn database_url_for_env() -> String {
    // TODO
    // WTF why why ...
    let result = dotenvy::from_filename(
        "/Users/bumzack/stoff/rust/proxythingis/too_many_servers/warp_diesel_postgres_person_server/.env",
    );
    match &result {
        Ok(p) => info!("path to .env {:?}", &p),
        Err(e) => error!("error {:?}", e),
    }

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    info!("DATABASE URL {}", database_url);
    database_url
}

pub fn get_connection_pool() -> Pool<ConnectionManager<PgConnection>> {
    let url = database_url_for_env();
    let manager = ConnectionManager::<PgConnection>::new(url);
    // Refer to the `r2d2` documentation for more methods to use
    // when building a connection pool
    Pool::builder()
        .test_on_check_out(true)
        .build(manager)
        .expect("Could not build connection pool")
}

pub fn with_db(
    db: Pool<ConnectionManager<PgConnection>>,
) -> impl Filter<Extract = (Pool<ConnectionManager<PgConnection>>,), Error = std::convert::Infallible>
       + Clone {
    let connection = pool.get();

    if connection.is_err() {
        let e = connection.as_ref().err();
        error!("error getting connection for DB.  err {}", e.unwrap());
        return vec![];
    }
    let mut conn = connection.unwrap();
    let results = person.load::<Person>(&mut conn);
    if results.is_err() {
        let e = results.as_ref().err();
        error!("error reading persons from DB.  err {}", e.unwrap());
        return vec![];
    }
    results.unwrap()

    // // info!("Displaying {} persons", results.len());
    //     // info!("id {}:  {} {}, created at {}", p.id, p.firstname, p.lastname, p.created);
        Ok(p) => {} // info!("path to .env {:?}", &p),
    // info!("DATABASE URL {}", database_url);
) -> impl Filter<Extract=(Pool<ConnectionManager<PgConnection>>, ), Error=Infallible> + Clone {
    warp::any().map(move || db.clone())
}
