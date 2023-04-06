extern crate lazy_static;

use std::convert::Infallible;
use std::env;

use log::error;
use log::info;
use warp::{Filter, hyper};
use warp::http::{Request, StatusCode};
use warp::hyper::{Body, Uri};
use warp::hyper::body::Bytes;

use common::warp_request_filter::{
    extract_request_data_filter, ProxyHeaders, ProxyMethod, ProxyQueryParameters, ProxyUri,
};

use crate::hyper::Client;
use crate::hyper::client::HttpConnector;

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
    let filter = std::env::var("RUST_LOG").unwrap_or_else(|_| "tracing=info,warp=debug".to_owned());

    let routes = warp::any()
        .and(extract_request_data_filter())
        .map(
            |uri: ProxyUri,
             params: ProxyQueryParameters,
             proxy_method: ProxyMethod,
             headers: ProxyHeaders,
             body: Bytes| {
                info!("uri  {:?}", &uri);
                match &params {
                    Some(p) => info!("params  {:?}", p),
                    None => error!("no params provided"),
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

                let mut hyper_request = hyper::http::Request::builder()
                    .method(method)
                    .uri(full_path)
                    .body(hyper::body::Body::from(body))
                    .expect("Request::builder() failed");
                {
                    *hyper_request.headers_mut() = headers;
                }

                return hyper_request;
            },
        )
        .and_then(|hyper_request: Request<Body>| {
            // handler signature: async fn handler(mut request: Request<Body>) -> Result<Response<Body>>
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
        });

    warp::serve(routes).run(([127, 0, 0, 1], 3032)).await;
}

async fn handler(mut request: Request<Body>) -> Result<impl warp::Reply, Infallible> {
    let request_uri = request.uri().to_string();

    if request_uri == "/" {
        return Ok(hyper::Response::builder()
            .status(StatusCode::PERMANENT_REDIRECT)
            .header(hyper::header::LOCATION, format!("/am/client/index.html"))
            .body(hyper::Body::empty())
            .unwrap());
    }
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
