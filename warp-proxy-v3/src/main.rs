extern crate lazy_static;

use std::convert::Infallible;
use std::env;
use std::future::Future;

use log::{error, info};
use tokio::time::Instant;
use warp::http::{HeaderValue, Request};
use warp::hyper::body::Bytes;
use warp::hyper::{Body, Uri};
use warp::{hyper, Filter, Rejection, Reply};

use common::warp_request_filter::{
    extract_request_data_filter, ProxyHeaders, ProxyMethod, ProxyQueryParameters, ProxyUri,
};

use crate::hyper::client::HttpConnector;
use crate::hyper::Client;

// gotta give credit where credit is due and stuff
lazy_static::lazy_static! {
    static ref  CLIENT: Client<HttpConnector> = {
         let http_connector = hyper::client::HttpConnector::new();
         let client = hyper::Client::builder().build(http_connector);
         return client;
    };
}

#[tokio::main]
async fn main() {
    if env::var_os("RUST_LOG").is_none() {
        // Set `RUST_LOG=todos=debug` to see debug logs,
        // this only shows access logs.
        env::set_var("RUST_LOG", "todos=info");
    }
    pretty_env_logger::init();

    let routes = warp::any()
        .and(extract_request_data_filter())
        .map(
            |uri: ProxyUri,
             params: ProxyQueryParameters,
             proxy_method: ProxyMethod,
             headers: ProxyHeaders,
             body: Bytes| {
                compose_forward_request(&uri, &params, &proxy_method, &headers, body)
            },
        )
        .and_then(|hyper_request: Request<Body>| execute_forward_request(hyper_request));

    warp::serve(routes).run(([127, 0, 0, 1], 3033)).await;
}

fn execute_forward_request(
    hyper_request: Request<Body>,
) -> impl Future<Output = Result<impl Reply + Sized, Rejection>> {
    let result = handler(hyper_request);

    async move {
        let res = match result.await {
            Ok(response) => Ok(response),
            Err(e) => {
                error!("error from client {}", e);
                Err(warp::reject::not_found())
            }
        };
        res
    }
}

fn compose_forward_request(
    uri: &ProxyUri,
    params: &ProxyQueryParameters,
    proxy_method: &ProxyMethod,
    headers: &ProxyHeaders,
    body: Bytes,
) -> Request<Body> {
    info!("uri  {:?}", &uri);
    match &params {
        Some(p) => info!("params  {:?}", p),
        None => info!("no params provided"),
    }
    info!("params  {:?}", &params);
    info!("proxy_method  {:?}", &proxy_method);
    info!("headers  {:?}", &headers);

    let method = hyper::http::Method::POST;
    let path = "full_path_ahead";

    let full_path = match &params {
        Some(p) => format!("/{}?{}", path, p),
        None => path.to_string(),
    };

    info!("final path {:?}", &full_path);
    info!("body empty {:?}", &body.is_empty());

    let mut hyper_request = hyper::http::Request::builder()
        .method(method)
        .uri(full_path)
        .body(hyper::body::Body::from(body))
        // .body(hyper::body::Body::empty())
        .expect("Request::builder() failed");
    {
        *hyper_request.headers_mut() = headers.clone();
    }

    hyper_request
}

async fn handler(mut request: Request<Body>) -> Result<impl warp::Reply, Infallible> {
    let schema = "http";
    let host = "localhost";
    let port = "3040";
    let path = "fromloadbalancer";
    let proxy_url = format!(
        "{}://{}:{}/{}{}",
        schema,
        host,
        port,
        path,
        request.uri().to_string()
    );

    // let proxyUriForLogging = proxyUrl.clone();
    let proxy_url = proxy_url.parse::<Uri>().unwrap();
    *request.uri_mut() = proxy_url.clone();

    let headers = request.headers_mut();
    headers.insert(
        hyper::header::HOST,
        hyper::header::HeaderValue::from_str("bla").unwrap(),
    );
    let origin = format!("{}://{}::{}", schema, host, port);
    headers.insert(
        hyper::header::ORIGIN,
        hyper::header::HeaderValue::from_str(origin.as_str()).unwrap(),
    );
    //
    // let http_connector = hyper::client::HttpConnector::new();
    // let client = hyper::Client::builder().build(http_connector);
    info!("redirecting to proxyUrl {}", proxy_url);

    let start = Instant::now();
    let mut response = CLIENT.request(request).await.expect("Request failed");
    let duration = start.elapsed();
    let d = format!(
        "duration {} ms,{} µs  {} ns ",
        duration.as_millis(),
        duration.as_micros(),
        duration.as_nanos()
    );
    info!("{} ", &d);
    response
        .headers_mut()
        .insert("x-duration", HeaderValue::from_str(&d).unwrap());
    Ok(response)
}
