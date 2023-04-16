use futures::TryStreamExt;
use log::info;
use warp::{Buf, Filter, hyper, Rejection, Reply, Stream};
use warp::http::StatusCode;
use warp::path::FullPath;

fn main() {
    info!("Hello, world!");
}

pub fn extract_request_data_filter_body_stream() -> impl Filter<
    Extract=(FullPath, impl Stream<Item=Result<impl Buf, warp::Error>>),
    Error=Rejection,
> + Clone {
    warp::path::full().and(warp::body::stream())
}

pub fn proxy_routes() -> impl Filter<Extract=(impl warp::Reply, ), Error=warp::Rejection> + Clone
{
    warp::any()
        .and(extract_request_data_filter_body_stream())
        .and_then(|_fullpath, body| execute_forward_request(body))
}

pub async fn execute_forward_request(
    body: impl Stream<Item=Result<impl Buf, warp::Error>> + Send + 'static,
) -> Result<impl Reply, Rejection> {
    let body = body.map_ok(|mut buf| buf.copy_to_bytes(buf.remaining()));

    let mut _hyper_request = hyper::http::Request::builder()
        .body(hyper::body::Body::wrap_stream(body))
        .expect("Request::builder() failed");

    let _json = warp::reply::json(&"bla");

    Ok(StatusCode::OK)
}
