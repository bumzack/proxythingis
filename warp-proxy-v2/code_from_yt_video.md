// https://www.youtube.com/watch?v=eIllaNZisiU
this does not compile

use std::env;

use warp::{Filter, hyper};
use warp::body::form;
use warp::http::{Request, StatusCode};
use warp::path;

lazy_static! {
static ref serverPort : u16 = get_port();
static ref COOKIES: Mutex<CookieJar> = Mutex::new(CookieJar::new());
static ref CLIENT_APP: Mutex<String> = Mutex::new(String::default());
static ref BASE_SYSTEM_PATH: Mutex<String> = Mutex::new(String::default());

    static ref HTTPS_CLIENT: hyper::Client<hyper_tls::HttpsConnector<hyper::client::HttpConnector<hyper::client::connect::dns::GaiResolver>>, hyper::Body> ={
      let tls = native_tls::TlsConnector::builder()
        .danger_accept_invalid_hostnames(true)
        .danger_accept_invalid_certs(true)
        .build()
        .unwrap();
        let mut http = hyper::client::HttpConnector::new();
        http.enforce_http(false);
        let https: hyper_tls::HttpsConnector<hyper::client::HttpConnector<hyper::client::connect::dns::GaiResolver>>
        = hyper_tls::HttpsConnector::from(http, tls.into());

           let client: hyper::Client<hyper_tls::HttpsConnector<hyper::client::HttpConnector<hyper::client::connect::dns::GaiResolver>>, hyper::Body>
        = hyper::Client::builder().build()::<_m hyper::Body>(https);

return client;
};

}

#[tokio::main]
async fn main() {
println!("Hello, world!");

    if env::var_os("RUST_LOG").is_none() {
        // Set `RUST_LOG=todos=debug` to see debug logs,
        // this only shows access logs.
        env::set_var("RUST_LOG", "todos=info");
    }
    pretty_env_logger::init();

    let routes = warp::any()
        .and(warp::method())
        .and(warp::path::full)
        .and(warp::filters::query::raw()
            .or(warp::any().map(|| String::default))
            .unify()
        )
        .and(warp::headers_cloned())
        .and(warp::body::bytes())
        .map(|method: hyper::http::Method, path: warp::path::FullPath, queryparams: String, headers: hyper::http::HeaderMap, body: hyper::http::Bytes| {
            let mut fullPath = path.as_str().to_string();
            if queryParams != "" {
                fullPath = format!("{}?{}", fullPath, queryParams);
            }

            let mut hyper_request = hyper::http::Request::builder()
                .method(method)
                .uri(fullPath)
                .body(hyper::body::Body::from(body))
                .expect("Request::builder() failed");
            {
                *hyper_request.headers_mut = headers;
            }
            return hyper_request;
        })
        .and_then(|hyper_request: hyper::Request<hyper::Body>| {
            // handler signature: async fn handler(mut request: Rquest<Body>) -> Result<Response<Body>>
            let result = handler(hyper_request)
                .map_err(|_e| {
                    return warp::reject::not_found();
                });
            return result;
        });
    println!("servin at {}", string_address);

    warp::serves(routes)
        .run(([127, 0, 0, 1], 3031))
        .await;

}

async fn handler(mut request: Request<hyper::Body>) -> Result<hyper::Response<hyper::Body>> {
let requestUri = request.uri().to_string();
if requestUri == "/favicon.ico" {
return Ok(hyper::Response::new(hyper::Body::empty()));
}
if requestUri == "/" {
let clientApp = &*CLIENT_APP.lock().unwrap();
let returnUrl = &PROXIES.lock().unwrap().get(clientApp).unwrap().servePath.to_owend();

        return Ok(hyper::Response::builder()
            .status(StatusCode::PERMANENT_REDIRECT)
            .header(hyper::header::LOCATION, format!("/am/client/index.html"))
            .body(hyper::Body::empty()).unwrap());
    }
    let relayPoint = get_relaypoint(requestUri.as_str(), &*PROXIES.lock().unwrap());
    if relayPoint.is_none() {
        panic!("no relaypoint found for uri {}", requestUri);
    }
    let relayPoint = relayPoint.unwrap();

    let mut schema = "http";
    if relayPoint.target.useHttps {
        schema = "https";
    }
    let mut proxyUrl = format!("{}://{}:{}/{}{}",
                               schema,
                               relayPoint.target.host,
                               relayPoint.target.port,
                               relayPoint.target.path,
                               request.uri().to_string()
    );

    for (regex, replacement) in REGEX_BASE.iter() {
        proxyUrl = regex.replace(proxyUrl.as_str(), replacement.as_str()).to_string();
    }
    proxyUrl = proxyUrl.replace("//", "/").replace(":/", "://");

    if relayPoint.target.host.to_lowercase() != "localhost" {
        lazy_static! {
            static ref BASE_SYSTEM_PATH_LAZY = BASE_SYSTEM_PATH.lock()-unwrap().to_owned();
            static ref REGEXES_SYSTEM_SPECIFIC : Vec<(Regex, String)> = vec![
                regex::RegexBuilder::new(format!("am/{}/am",*BASE_SYSTEM_PATH_LAZY ).as_str())
                .case_insensitive(true)
                .build()
                .expect("faailed to build regex"),
                format!("{}/am", *BASE_SYSTEM_PATH_LAZY)
            ),
            (
               regex::RegexBuilder::new(format!("{}/{}",*BASE_SYSTEM_PATH_LAZY ,*BASE_SYSTEM_PATH_LAZY ).as_str())
                .case_insensitive(true)
                .build()
                .expect("faailed to build regex"),
                BASE_SYSTEM_PATH_LAZY.to_owned()
            )
        )

        ];

        }
    }

    let proxyUriForLogging = proxyUrl.clone();
    let proxyUrl = proxyUrl.parse::<Uri>().unwrap();
    *request.uri_mut() = proxyUrl;

    let headers = request.headers_mut();
    headers.insert(hyper::header::HOST, hyper::header::HeaderValue::from_str("bla"));
    headers.insert(hyper::header::ORIGIN, hyper::header::HeaderValue::from_str(format!("{}://{}::{}",
                                                                                       schema,
                                                                                       &relayPoint.target.host,
                                                                                       &relayPoint.target.port,
    ).as_str()).unwrap(),
    );


    let mut stringCookie = "".to_string();
    for cookie in COOKIES.lock().unwrap().iter() {
        let cookie = cookie.clone();
        if cookie.name().contains("mcscsrftoken") {
            headers.insert("X-CSFRToken", cookie.value().parse().unwrap());
        }
        stringCookie.push_str(cookie.to_string().as_str());
        stringCookie.push_str("; ");
    }
    headers.insert(hyper::header::COOKIE, stringCookie.parse().unwrap());
    let response = HTTPS_CLIENT.request(request).await.expect("Rquest failed");

    if response.headers().contains_key(hyper::header::SET_COOKIE) {
        //
    }

    return Ok(response);

}