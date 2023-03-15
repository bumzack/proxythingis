#[macro_use]
extern crate lazy_static;

use std::convert::Infallible;
use std::env;

use tracing_subscriber::fmt::format::FmtSpan;
use warp::{Filter, hyper};
use warp::http::{Request, StatusCode};
use warp::hyper::{Body, Uri};
use warp::hyper::body::Bytes;

use common::warp_request_filter::{extract_request_data_filter, ProxyHeaders, ProxyMethod, ProxyQueryParameters, ProxyUri, string_filter};

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
    //pretty_env_logger::init();
    let filter = std::env::var("RUST_LOG").unwrap_or_else(|_| "tracing=info,warp=debug".to_owned());
    tracing_subscriber::fmt()
        // Use the filter we built above to determine which traces to record.
        .with_env_filter(filter)
        // Record an event when each span closes. This can be used to time our
        // routes' durations!
        .with_span_events(FmtSpan::CLOSE)
        .init();


    let routes = warp::any()
        .and(extract_request_data_filter())
        .map(|uri: ProxyUri, params: ProxyQueryParameters, proxy_method: ProxyMethod, headers: ProxyHeaders, body: Bytes| {
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

            let mut hyper_request = hyper::http::Request::builder()
                .method(method)
                .uri(full_path)


                .body(hyper::body::Body::from(body))

                // .body(hyper::body::Body::empty())
                .expect("Request::builder() failed");
            {
                *hyper_request.headers_mut() = headers;
            }

            return hyper_request;
        })
        .and_then(|hyper_request: Request<Body>| {
            // handler signature: async fn handler(mut request: Request<Body>) -> Result<Response<Body>>
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
        })
        .with(warp::trace(|info| {
            // Construct our own custom span for this route.
            tracing::info_span!("goodbye", req.path = ?info.path())
        }))
        .with(warp::trace::named("hello"))
        .with(warp::trace::request());

    warp::serve(routes)
        .run(([127, 0, 0, 1], 3031))
        .await;
}

async fn handler(mut request: Request<Body>) -> Result<impl warp::Reply, Infallible> {
    let request_uri = request.uri().to_string();

    if request_uri == "/" {
        return Ok(hyper::Response::builder()
            .status(StatusCode::PERMANENT_REDIRECT)
            .header(hyper::header::LOCATION, format!("/am/client/index.html"))
            .body(hyper::Body::empty()).unwrap());
    }
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
    let response = CLIENT.request(request).await.expect("Request failed");

    return Ok(response);
}
