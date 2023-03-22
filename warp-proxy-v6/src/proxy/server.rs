use std::convert::Infallible;
use std::str::FromStr;
use std::time::Instant;

use futures_util::TryStreamExt;
use rand::Rng;
use tokio::sync::mpsc::UnboundedSender;
use tokio::sync::oneshot;
use warp::http::{HeaderValue, Method, Request, Uri};
use warp::hyper::Body;
use warp::{hyper, Buf, Rejection, Reply, Stream};

use common::warp_request_filter::{ProxyHeaders, ProxyMethod, ProxyQueryParameters, ProxyUri};

use crate::config_manager::manager::{
    GetConfigData, ManagerCommand, ProxyConfig, UpdateSourceStatsData, UpdateTargetStatsData,
};
use crate::proxyserver::models::{ServerSource, ServerTarget};
use crate::CLIENT;

pub async fn execute_forward_request(
    uri: ProxyUri,
    params: ProxyQueryParameters,
    proxy_method: ProxyMethod,
    headers: ProxyHeaders,
    body: impl Stream<Item = Result<impl Buf, warp::Error>> + Send + 'static,
    sender: UnboundedSender<ManagerCommand>,
) -> Result<impl Reply, Rejection> {
    let (tx, rx) = oneshot::channel();
    let get_config_data = GetConfigData {
        sender: tx,
        reset_start: false,
    };
    let cmd = ManagerCommand::GetConfig(get_config_data);
    match sender.send(cmd) {
        Ok(_) => println!("send ok"),
        Err(e) => println!("error sending cmd::GetConfig to manager {}", e),
    };

    let proxy_config = rx
        .await
        .expect("execute_forward_request expected a valid proxy config");

    // is there a match for the uri in the config
    let source = find_match(&uri, &proxy_config, &proxy_method);
    let target: Option<&ServerTarget> = match source {
        Some(server) => {
            let targets = &server.targets;
            if targets.len() > 0 {
                let mut rng = rand::thread_rng();
                let i = rng.gen_range(0..targets.len());
                if i > targets.len() {
                    println!(
                        "random number WRONG between {} and {}: {}",
                        0,
                        targets.len(),
                        i
                    );
                }
                let t = targets.get(i as usize).expect("cant unwrap target server");
                Some(t)
            } else {
                None
            }
        }
        None => None,
    };
    if target.is_none() {
        return Err(warp::reject::not_found());
    }
    let source = source.unwrap();

    let x = uri.as_str();
    let string = x.to_ascii_lowercase();
    let (_, path_to_pass_on) = string.split_at(source.path_starts_with.len());
    let target = target.unwrap();
    let target_path = &target.path;
    let target_host = &target.host;
    let target_port = &target.port;
    let target_method = &target.method;
    let target_schema = &target.schema;

    let full_path = match &params {
        Some(p) => format!("{}{}?{}", target_path, path_to_pass_on, p),
        None => format!("{}{}", target_path, path_to_pass_on),
    };

    let m = Method::from_str(target_method).expect("cant determine method from str");

    let body = body.map_ok(|mut buf| buf.copy_to_bytes(buf.remaining()));

    let mut hyper_request = hyper::http::Request::builder()
        .method(m)
        .uri(full_path.clone())
        .body(hyper::body::Body::wrap_stream(body))
        .expect("Request::builder() failed");
    {
        *hyper_request.headers_mut() = headers.clone();
    }

    let update_source_stats_data = UpdateSourceStatsData { id: 1 };
    let cmd = ManagerCommand::UpdateSourceStats(update_source_stats_data);
    sender
        .send(cmd)
        .expect("expect the send with command UpdateSourceStats to work");

    let result = handler(
        hyper_request,
        sender,
        target.id,
        target_port,
        target_host,
        full_path,
        target_schema,
        &target.description,
    );

    let res = match result.await {
        Ok(response) => Ok(response),
        Err(_e) => {
            // println!("error from client {}", e);
            Err(warp::reject::not_found())
        }
    };
    res
}

async fn handler(
    mut request: Request<Body>,
    sender: UnboundedSender<ManagerCommand>,
    server_target_idx: i32,
    target_port: &i32,
    target_host: &String,
    full_path: String,
    target_schema: &String,
    target_description: &String,
) -> Result<impl warp::Reply, Infallible> {
    // println!("full_path                         {:?}", &full_path);
    // println!("target_host                       {:?}", &target_host);
    // println!("target_port                       {:?}", &target_port);
    // println!("target_method                     {:?}", &target_method);
    // println!("target_schema                     {:?}", &target_schema);
    // println!("request.uri().to_string()         {:?}", &request.uri().to_string());

    let proxy_url = format!(
        "{}://{}:{}{}",
        target_schema, target_host, target_port, full_path
    );
    // println!("proxy_url         {:?}", &proxy_url);

    // let proxyUriForLogging = proxyUrl.clone();
    let proxy_url = proxy_url.parse::<Uri>().unwrap();
    *request.uri_mut() = proxy_url.clone();

    let headers = request.headers_mut();
    headers.insert(
        hyper::header::HOST,
        hyper::header::HeaderValue::from_str("bla").unwrap(),
    );
    let origin = format!("{}://{}::{}", target_schema, target_host, target_port);
    headers.insert(
        hyper::header::ORIGIN,
        hyper::header::HeaderValue::from_str(origin.as_str()).unwrap(),
    );
    //
    // let http_connector = hyper::client::HttpConnector::new();
    // let client = hyper::Client::builder().build(http_connector);

    let start = Instant::now();
    //println!("request uri {}", request.uri().to_string());
    let mut response = CLIENT.request(request).await.expect("Request failed");
    let duration = start.elapsed();
    let d = format!(
        "duration {} ms, {} µs, {} ns ",
        duration.as_millis(),
        duration.as_micros(),
        duration.as_nanos()
    );
    // println!("{} ", &d);
    response
        .headers_mut()
        .insert("x-duration", HeaderValue::from_str(&d).unwrap());
    response.headers_mut().insert(
        "access-control-allow-origin",
        HeaderValue::from_str(&"http://localhost:4011").unwrap(),
    );
    response.headers_mut().insert(
        "x-provided-by",
        HeaderValue::from_str(target_description).unwrap(),
    );

    let update_target_stats_data = UpdateTargetStatsData {
        id: server_target_idx,
        duration_nanos: duration.as_nanos() as i64,
    };
    let cmd = ManagerCommand::UpdateTargetStats(update_target_stats_data);
    sender
        .send(cmd)
        .expect("expect the send with command UpdateTargetStats to work");

    Ok(response)
}

fn find_match<'a>(
    uri: &ProxyUri,
    proxy_config: &'a ProxyConfig,
    method: &Method,
) -> Option<&'a ServerSource> {
    for s in &proxy_config.server_sources {
        // println!("searching for request uri {}, method {}    comparing with config  path_starts_with  {} and method {}",
        //          &uri.as_str(), &method,&s.path_starts_with, &s.method);
        if uri.as_str().starts_with(&s.path_starts_with)
            && method.as_str().to_ascii_lowercase() == s.method.to_ascii_lowercase()
        {
            return Some(s);
        }
    }
    None
}
