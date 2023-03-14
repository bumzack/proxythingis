use std::env;

use warp::{  Filter  };


#[tokio::main]
async fn main() {
    if env::var_os("RUST_LOG").is_none() {
        // Set `RUST_LOG=todos=debug` to see debug logs,
        // this only shows access logs.
        env::set_var("RUST_LOG", "response_to_everything=info");
    }

    let routes = warp::any()
        .map(warp::reply);

    warp::serve(routes)
        .run(([127, 0, 0, 1], 3040))
        .await;
}