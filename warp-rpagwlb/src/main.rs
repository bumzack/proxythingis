use std::convert::Infallible;
use std::env;

use warp::{Error, Filter, hyper};
use warp::header::headers_cloned;
use warp::http::{Request, StatusCode};
use warp::hyper::{Body, Response, Uri};
use warp::hyper::body::HttpBody;
use warp::path;

use crate::hyper::body::Bytes;

// lazy_static::lazy_static! {
//     static ref  PROXY_CLIENT: ReverseProxy<RustlsHttpsConnector> = {
//         ReverseProxy::new(
//             hyper::Client::builder().build::<_, hyper::Body>(TrustDnsResolver::default().into_rustls_webpki_https_connector()),
//         )
//     };
// }

#[tokio::main]
async fn main() {
    if env::var_os("RUST_LOG").is_none() {
        // Set `RUST_LOG=todos=debug` to see debug logs,
        // this only shows access logs.
        env::set_var("RUST_LOG", "todos=info");
    }
    //pretty_env_logger::init();

    let routes = warp::any()
        .and(warp::method())
        // .and(warp::path::full)
        // .and(warp::filters::query::raw()
        //     .or(warp::any().map(|| String::default))
        //     .unify()
        // )
        // .and(warp::header::headers_cloned())
        // .and(warp::body::bytes())
        //   .map(|method: hyper::http::Method, path: warp::path::FullPath, queryparams: String, headers: hyper::http::HeaderMap, body: hyper::body::Bytes| {
        .map(|method: hyper::http::Method| {
            // let mut fullPath = path.as_str().to_string();
            // if queryparams != "" {
            //     fullPath = format!("{}?{}", fullPath, queryparams);
            // }
            let method = hyper::http::Method::POST;
            let fullPath = "full_path_ahead";
            let mut hyper_request = hyper::http::Request::builder()
                .method(method)
                .uri(fullPath)
                .body(Body::empty())            //  .body(hyper::body::Body::from(body))
                .expect("Request::builder() failed");
            // {
            //     *hyper_request.headers_mut = headers;
            // }

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
        });
    // println!("serving at {}", string_address);

    warp::serve(routes)
        .run(([127, 0, 0, 1], 3031))
        .await;
}

async fn handler(mut request: Request<hyper::Body>) -> Result<impl warp::Reply, Infallible> {
    let requestUri = request.uri().to_string();
    if requestUri == "/favicon.ico" {
        return Ok(hyper::Response::new(hyper::Body::empty()));
    }
    if requestUri == "/" {
        return Ok(hyper::Response::builder()
            .status(StatusCode::PERMANENT_REDIRECT)
            .header(hyper::header::LOCATION, format!("/am/client/index.html"))
            .body(hyper::Body::empty()).unwrap());
    }
    let schema = "http";
    let host = "localhost";
    let port = "3040";
    let path = "fromloadbalancer";
    let mut proxyUrl = format!("{}://{}:{}/{}{}",
                               schema,
                               host,
                               port,
                               path,
                               request.uri().to_string()
    );

    // let proxyUriForLogging = proxyUrl.clone();
    let proxyUrl = proxyUrl.parse::<Uri>().unwrap();
    *request.uri_mut() = proxyUrl.clone();

    let headers = request.headers_mut();
    headers.insert(hyper::header::HOST, hyper::header::HeaderValue::from_str("bla").unwrap());
    let origin = format!("{}://{}::{}", schema, host, port);
    headers.insert(hyper::header::ORIGIN, hyper::header::HeaderValue::from_str(origin.as_str()).unwrap());

    let http_connector = hyper::client::HttpConnector::new();
    let client = hyper::Client::builder().build(http_connector);

    println!("redirecting to proxyUrl {}", proxyUrl);
    let response = client.request(request).await.expect("Request failed");

    return Ok(response);
}