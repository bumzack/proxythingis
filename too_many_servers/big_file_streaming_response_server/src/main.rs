use std::convert::Infallible;
use std::env;

use warp::{Filter, hyper};
use warp::hyper::StatusCode;

pub fn proxy() -> impl Filter<Extract=(impl warp::Reply, ), Error=warp::Rejection> + Clone {
    warp::path!("data").and(warp::get()).and_then(stream_data)
}

pub async fn stream_data() -> Result<impl warp::Reply, Infallible> {
    let mut data: Vec<Result<_, std::io::Error>> = vec![];
    for _i in 0..1_000_000 {
        data.push(Ok("Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet. Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet."));
    }
    let stream = futures_util::stream::iter(data);
    let body = hyper::Body::wrap_stream(stream);
    let res = hyper::Response::builder()
        .status(StatusCode::OK)
        .body(body)
        .unwrap();
    Ok(res)
}

#[tokio::main]
async fn main() {
    if env::var_os("RUST_LOG").is_none() {
        env::set_var("RUST_LOG", "response_to_everything=info");
    }
    pretty_env_logger::init();

    warp::serve(proxy()).run(([127, 0, 0, 1], 3070)).await;
}
