use std::future::Future;

use futures::TryStreamExt;
use warp::http::StatusCode;
use warp::path::FullPath;
use warp::{hyper, Buf, Filter, Rejection, Reply, Stream};

fn main() {
    println!("Hello, world!");
}

pub fn extract_request_data_filter_body_stream() -> impl Filter<
    Extract = (FullPath, impl Stream<Item = Result<impl Buf, warp::Error>>),
    Error = warp::Rejection,
> + Clone {
    warp::path::full().and(warp::body::stream())
}

pub fn proxy_routes() -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone
{
    warp::any()
        .and(extract_request_data_filter_body_stream())
        .and_then(|fullpath, body| execute_forward_request(body))
}

pub async fn execute_forward_request(
    body: impl Stream<Item = Result<impl Buf, warp::Error>> + Send + 'static,
) -> std::result::Result<impl Reply, Rejection> {
    let body = body.map_ok(|mut buf| buf.copy_to_bytes(buf.remaining()));

    let mut _hyper_request = hyper::http::Request::builder()
        .body(hyper::body::Body::wrap_stream(body))
        .expect("Request::builder() failed");

    let json = warp::reply::json(&"bla");

    Ok(StatusCode::OK)
}
