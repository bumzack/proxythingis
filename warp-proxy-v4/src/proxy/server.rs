use std::convert::Infallible;
use std::str::FromStr;
use std::time::Instant;

use log::info;
use rand::Rng;
use tokio::sync::mpsc::UnboundedSender;
use tokio::sync::oneshot;
use warp::{hyper, Rejection, Reply};
use warp::http::{HeaderValue, Method, Request, Uri};
use warp::hyper::Body;
use warp::hyper::body::Bytes;

use common::warp_request_filter::{ProxyHeaders, ProxyMethod, ProxyQueryParameters, ProxyUri};

use crate::CLIENT;
use crate::config_manager::manager::{
    GetConfigData, ManagerCommand, ProxyConfig, UpdateSourceStatsData, UpdateTargetStatsData,
};
use crate::proxyserver::models::{ServerSource, ServerTarget};

pub async fn execute_forward_request(
    uri: ProxyUri,
    params: ProxyQueryParameters,
    proxy_method: ProxyMethod,
    headers: ProxyHeaders,
    body: Bytes,
    sender: UnboundedSender<ManagerCommand>,
) -> Result<impl Reply, Rejection> {
    let (tx, rx) = oneshot::channel();
    let get_config_data = GetConfigData {
        sender: tx,
        reset_start: false,
    };
    let cmd = ManagerCommand::GetConfig(get_config_data);
    match sender.send(cmd) {
        Ok(_) => info!("send ok"),
        Err(e) => info!("error sending cmd::GetConfig to manager {}", e),
    };

    let proxy_config = rx
        .await
        .expect("execute_forward_request expected a valid proxy config");

    let source = find_match(&uri, &proxy_config, &proxy_method);
    let target: Option<&ServerTarget> = match source {
        Some(server) => {
            let targets = &server.targets;
            if targets.len() > 0 {
                let mut rng = rand::thread_rng();
                let i = rng.gen_range(0..targets.len());
                if i > targets.len() {
                    info!(
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

    let mut hyper_request = hyper::http::Request::builder()
        .method(m)
        .uri(full_path.clone())
        .body(hyper::body::Body::from(body))
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
            // info!("error from client {}", e);
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
    let proxy_url = format!(
        "{}://{}:{}{}",
        target_schema, target_host, target_port, full_path
    );

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

    let start = Instant::now();
    let mut response = CLIENT.request(request).await.expect("Request failed");
    let duration = start.elapsed();
    let d = format!(
        "duration {} ms, {} µs, {} ns ",
        duration.as_millis(),
        duration.as_micros(),
        duration.as_nanos()
    );
    response
        .headers_mut()
        .insert("x-duration", HeaderValue::from_str(&d).unwrap());
    response.headers_mut().insert(
        "x-provided-by",
        HeaderValue::from_str(target_description).unwrap(),
    );

    let update_target_stats_data = UpdateTargetStatsData {
        id: server_target_idx,
        duration_nanos: duration.as_nanos() as u32,
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
        if uri.as_str().starts_with(&s.path_starts_with)
            && method.as_str().to_ascii_lowercase() == s.method.to_ascii_lowercase()
        {
            return Some(s);
        }
    }
    None
}
