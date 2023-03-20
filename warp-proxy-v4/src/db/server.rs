use std::convert::Infallible;

use deadpool_postgres::Pool;
use warp::Filter;

pub fn with_db(pool: Pool) -> impl Filter<Extract=(Pool, ), Error=Infallible> + Clone {
    warp::any().map(move || pool.clone())
}

