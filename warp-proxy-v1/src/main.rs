extern crate lazy_static;

use std::convert::Infallible;
use std::env;

use log::info;
use warp::http::Request;
use warp::hyper::{Body, Uri};
use warp::{hyper, Filter};

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
        .map(|| {
            let method = hyper::http::Method::GET;
            let full_path = "full_path_ahead";
            let hyper_request = hyper::http::Request::builder()
                .method(method)
                .uri(full_path)
                .body(Body::empty())
                .expect("Request::builder() failed");

            return hyper_request;
        })
        .and_then(|hyper_request: Request<Body>| {
            // handler signature: async fn handler(mut request: Request<Body>) -> Result<Response<Body>>
            let result = handler(hyper_request);

            async move {
                let res = match result.await {
                    Ok(response) => Ok(response),
                    Err(e) => {
                        info!("error from client {}", e);
                        Err(warp::reject::not_found())
                    }
                };
                res
            }
        });
    // info!("serving at {}", string_address);

    warp::serve(routes).run(([127, 0, 0, 1], 3031)).await;
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
    let response = CLIENT.request(request).await.expect("Request failed");

    return Ok(response);
}
