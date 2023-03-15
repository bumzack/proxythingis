use std::env;

use warp::Filter;
use warp::http::HeaderName;
use warp::hyper::{Method as RequestMethod, Uri};
use warp::hyper::body::Bytes;

use common::warp_request_filter::{extract_request_data_filter_body_as_string, ProxyHeaders, ProxyMethod, ProxyQueryParameters, ProxyUri};
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize)]
struct ResponseSummary {
    method: String,
    path: String,
    query_params: String,
    headers: Vec<String>,
    body: String,
}


#[tokio::main]
async fn main() {
    if env::var_os("RUST_LOG").is_none() {
        env::set_var("RUST_LOG", "response_to_everything=info");
    }

    let routes = warp::any()
        .and(extract_request_data_filter_body_as_string())
        .map(|uri: ProxyUri, params: ProxyQueryParameters, proxy_method: ProxyMethod, headers: ProxyHeaders, body_string: String| {
            println!("uri  {:?}", &uri);
            println!("params  {:?}", &params);
            println!("proxy_method  {:?}", &proxy_method);
            println!("headers  {:?}", &headers);
            println!("body as string {:?}", &body_string);

            let headers: Vec<String> = headers.into_iter()
                .map(|h| {
                    let key = match h.0 {
                        Some(n) => n.to_string(),
                        None => "n/a".to_string(),
                    };
                    let value = match h.1.to_str() {
                        Ok(n) => {
                            n.to_string()
                        }
                        Err(e) => {
                            println!("error2 {}", e);
                            "n/a".to_string()
                        }
                    };
                    format!("{} -> {}", &key, &value)
                })
                .collect();

            let path = format!("{:?}", uri);
            let params = match params {
                Some(p) => p,
                None => "(no query params) ".to_string()
            };

            let res = ResponseSummary {
                method: proxy_method.to_string(),
                path,
                query_params: params,
                headers,
                body: body_string,
            };
            warp::reply::json(&res)
        });

    warp::serve(routes)
        .run(([127, 0, 0, 1], 3040))
        .await;
}
