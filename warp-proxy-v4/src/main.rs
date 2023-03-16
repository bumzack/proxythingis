extern crate lazy_static;

use std::convert::Infallible;
use std::env;
use std::future::Future;

use tokio::time::Instant;
use tracing_subscriber::fmt::format::FmtSpan;
use warp::{Filter, hyper, Rejection, Reply};
use warp::http::{HeaderValue, Request};
use warp::hyper::{Body, Uri};
use warp::hyper::body::Bytes;

use common::warp_request_filter::{extract_request_data_filter, ProxyHeaders, ProxyMethod, ProxyQueryParameters, ProxyUri};

use crate::db::{create_pool, with_db};
use crate::hyper::Client;
use crate::hyper::client::HttpConnector;
use crate::server::{create_source_handler, create_target_handler, list_servers_handler};

mod db;
mod models;
mod server;


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
    let result = dotenvy::from_filename("/Users/bumzack/stoff/rust/proxythingis/warp-proxy-v4/.env");

    if env::var_os("RUST_LOG").is_none() {
        // Set `RUST_LOG=todos=debug` to see debug logs,
        // this only shows access logs.
        env::set_var("RUST_LOG", "todos=info");
    }
    let pool = create_pool();

    //pretty_env_logger::init();
    let filter = std::env::var("RUST_LOG").unwrap_or_else(|_| "tracing=info,warp=debug".to_owned());
    tracing_subscriber::fmt()
        // Use the filter we built above to determine which traces to record.
        .with_env_filter(filter)
        // Record an event when each span closes. This can be used to time our
        // routes' durations!
        .with_span_events(FmtSpan::CLOSE)
        .init();

    let server_source = warp::path!("server" / "source");
    let server_source_create = server_source
        .and(warp::post())
        .and(warp::body::json())
        .and(with_db(pool.clone()))
        .and_then(create_source_handler);

    let server_target = warp::path!("server" / "target");
    let server_target_create = server_target
        .and(warp::post())
        .and(warp::body::json())
        .and(with_db(pool.clone()))
        .and_then(create_target_handler);

    let server = warp::path("server");
    let server_list = server
        .and(warp::get())
        .and(with_db(pool.clone()))
        .and_then(list_servers_handler);

    let server_routes = server_source_create
        .or(server_target_create)
        .or(server_list);

    let routes_proxy = warp::any()
        .and(extract_request_data_filter())
        .map(|uri: ProxyUri, params: ProxyQueryParameters, proxy_method: ProxyMethod, headers: ProxyHeaders, body: Bytes| {
            compose_forward_request(&uri, &params, &proxy_method, &headers, body)
        })
        .and_then(|hyper_request: Request<Body>| {
            execute_forward_request(hyper_request)
        });
    // .with(warp::trace(|info| {
    //     // Construct our own custom span for this route.
    //     tracing::info_span!("goodbye", req.path = ?info.path())
    // }))
    // .with(warp::trace::named("hello"))
    // .with(warp::trace::request());

    let routes = server_routes.or(routes_proxy);
    warp::serve(routes)
        .run(([127, 0, 0, 1], 3034))
        .await;
}

fn execute_forward_request(hyper_request: Request<Body>) -> impl Future<Output=Result<impl Reply + Sized, Rejection>> {
    let result = handler(hyper_request);

    async move {
        let res = match result.await {
            Ok(response) => Ok(response),
            Err(e) => {
                println!("error from client {}", e);
                Err(warp::reject::not_found())
            }
        };
        res
    }
}

fn compose_forward_request(uri: &ProxyUri, params: &ProxyQueryParameters, proxy_method: &ProxyMethod, headers: &ProxyHeaders, body: Bytes) -> Request<Body> {
    println!("uri  {:?}", &uri);
    match &params {
        Some(p) => println!("params  {:?}", p),
        None => println!("no params provided"),
    }
    println!("params  {:?}", &params);
    println!("proxy_method  {:?}", &proxy_method);
    println!("headers  {:?}", &headers);

    let method = hyper::http::Method::POST;
    let path = "full_path_ahead";

    let full_path = match &params {
        Some(p) => format!("/{}?{}", path, p),
        None => path.to_string(),
    };

    println!("final path {:?}", &full_path);
    println!("body empty {:?}", &body.is_empty());


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
    let proxy_url = format!("{}://{}:{}/{}{}",
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
    headers.insert(hyper::header::HOST, hyper::header::HeaderValue::from_str("bla").unwrap());
    let origin = format!("{}://{}::{}", schema, host, port);
    headers.insert(hyper::header::ORIGIN, hyper::header::HeaderValue::from_str(origin.as_str()).unwrap());
    //
    // let http_connector = hyper::client::HttpConnector::new();
    // let client = hyper::Client::builder().build(http_connector);
    println!("redirecting to proxyUrl {}", proxy_url);

    let start = Instant::now();
    let mut response = CLIENT.request(request).await.expect("Request failed");
    let duration = start.elapsed();
    let d = format!("duration {} ms, {} µs, {} ns ", duration.as_millis(), duration.as_micros(), duration.as_nanos());
    println!("{} ", &d);
    response.headers_mut().insert("x-duration", HeaderValue::from_str(&d).unwrap());
    Ok(response)
}
