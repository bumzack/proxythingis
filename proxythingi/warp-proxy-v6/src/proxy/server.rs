use std::convert::Infallible;
use std::str::FromStr;
use std::time::Instant;

use futures_util::TryStreamExt;
use log::{error, info};
use rand::Rng;
use tokio::sync::mpsc::UnboundedSender;
use tokio::sync::oneshot;
use uuid::Uuid;
use warp::http::{HeaderValue, Method, Request, Response, StatusCode, Uri};
use warp::hyper::Body;
use warp::{hyper, Buf, Rejection, Reply, Stream};

use common::config_manager_models::{GetConfigData, UpdateSourceStatsData, UpdateTargetStatsData};
use common::models::{ProxyConfig, ServerSource, ServerTarget};
use common::warp_server::warp_request_filter::{
    ProxyHeaders, ProxyMethod, ProxyQueryParameters, ProxyUri, HEADER_X_INITIATED_BY,
    HEADER_X_PROCESSED_BY, HEADER_X_UUID,
};

use crate::config_manager::manager::ManagerCommand;
use crate::CLIENT;

pub async fn execute_forward_request(
    uri: ProxyUri,
    params: ProxyQueryParameters,
    proxy_method: ProxyMethod,
    headers: ProxyHeaders,
    body: impl Stream<Item = Result<impl Buf, warp::Error>> + Send + 'static,
    sender: UnboundedSender<ManagerCommand>,
) -> Result<impl Reply, Rejection> {
    let start_total = Instant::now();
    let mut x_inititated_by = false;

    let (tx, rx) = oneshot::channel();
    let get_config_data = GetConfigData {
        sender: tx,
        whoami: "execute_forward_request".to_string(),
        //   reset_start: false,
    };
    let cmd = ManagerCommand::GetConfig(get_config_data);
    match sender.send(cmd) {
        Ok(_) => {} // info!("send ok"),
        Err(e) => error!("error sending cmd::GetConfig to manager {}", e),
    };

    let proxy_config = rx.await;

    if proxy_config.is_err() {
        let e = proxy_config.as_ref().err();
        error!(
            "error retrieving config while handling url {}. error {:?}",
            uri.as_str(),
            e.unwrap()
        )
    }
    let proxy_config = proxy_config.unwrap();

    // is there a match for the uri in the config
    let source = find_match(&uri, &proxy_config, &proxy_method);
    let target: Option<&ServerTarget> = match source {
        Some(server) => {
            info!(
                "found a matching source server for uri: {}, method  {} ",
                &uri.as_str(),
                &proxy_method.as_str()
            );
            let targets = &server.targets;
            if !targets.is_empty() {
                let mut rng = rand::thread_rng();
                let i = rng.gen_range(0..targets.len());
                if i > targets.len() {
                    // info!(
                    //     "random number WRONG between {} and {}: {}",
                    //     0,
                    //     targets.len(),
                    //     i
                    // );
                }
                let t = targets.get(i as usize).expect("cant unwrap target server");
                // info!("found a matching target server for uri: {}, method  {} //  target server: host: {} // port {} // method {} // path {}", &uri.as_str(), &proxy_method.as_str() ,t.host, t.port, t.method, t.path);
                Some(t)
            } else {
                error!(
                    "NOT found a matching target server for uri: {}, method  {} ",
                    &uri.as_str(),
                    &proxy_method.as_str()
                );
                None
            }
        }
        None => {
            error!(
                "NOT found a matching source server for uri: {}, method  {} ",
                &uri.as_str(),
                &proxy_method.as_str()
            );
            None
        }
    };
    if target.is_none() {
        return Err(warp::reject::not_found());
    }
    let source = source.expect("source ser ver should exist");

    let tmp = headers.clone();
    tmp.into_iter().for_each(|h| {
        if h.0.is_some() {
            let name = h.0.expect("header .0 should exist");
            info!("header: {:?} -> {:?} ", name, &h.1);
        }
    });

    let x = uri.as_str();
    let string = x.to_ascii_lowercase();
    let (_, path_to_pass_on) = string.split_at(source.path_starts_with.len());
    let target = target.unwrap();
    let target_path = &target.path;
    let target_host = &target.host;
    let target_port = &target.port;

    let target_method = match &target.method.eq("*") {
        true => proxy_method.as_str(),
        false => &target.method,
    };

    let target_schema = &target.schema;
    let full_path = match &params {
        Some(p) => format!("{}{}?{}", target_path, path_to_pass_on, p),
        None => format!("{}{}", target_path, path_to_pass_on),
    };

    info!("final request params taking into consideration a wildcard for the method. uri: {}, method  {} //  target server: host: {} // port {} // method {} // path {} // fullpath {}", &uri.as_str(), &proxy_method.as_str() ,target_host, target_port, target_method, &target_path, &full_path);

    let m = Method::from_str(target_method);
    if m.is_err() {
        let e = m.as_ref().err();
        error!(
            "error getting method from string {}. error {:?}",
            uri.as_str(),
            e.unwrap()
        )
    }
    let m = m.unwrap();

    let body = body.map_ok(|mut buf| buf.copy_to_bytes(buf.remaining()));

    let hyper_request = hyper::http::Request::builder()
        .method(m)
        .uri(full_path.clone())
        .body(Body::wrap_stream(body));

    if hyper_request.is_err() {
        let e = hyper_request.as_ref().err();
        error!(
            "error building request {}. error {:?}",
            uri.as_str(),
            e.unwrap()
        )
    }
    let mut hyper_request = hyper_request.unwrap();

    {
        *hyper_request.headers_mut() = headers.clone();
        // start_total
        if !headers.contains_key(HEADER_X_INITIATED_BY) {
            // hyper_request.headers_mut().insert(HEADER_X_INITIATED_BY, "proxythingi".parse().unwrap());
            x_inititated_by = true;
        }
    }

    let update_source_stats_data = UpdateSourceStatsData { id: 1 };
    let cmd = ManagerCommand::UpdateSourceStats(update_source_stats_data);
    match sender.send(cmd) {
        Ok(_) => {}
        Err(e) => error!("error sending update stats {:?}", e),
    }

    let result = handler(
        hyper_request,
        sender,
        target.id,
        target_port,
        target_host,
        full_path,
        target_schema,
        &target.description,
        x_inititated_by,
        start_total,
    );

    match result.await {
        Ok(response) => {
            info!(
                "forwarded request successfully handled for  source uri: {}, method  {}    ",
                &uri.as_str(),
                &proxy_method.as_str()
            );

            Ok(response)
        }
        Err(e) => {
            error!("forwarded request returned an error  for  source uri: {}, method  {}   ==>  error {:?} ",&uri.as_str(),&proxy_method.as_str(),e);
            Err(warp::reject::not_found())
        }
    }
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
    x_inititated_by: bool,
    start_total: Instant,
) -> Result<impl Reply, Infallible> {
    // info!("handler full_path                         {:?}", &full_path);
    // // info!(
    //     "handler target_host                       {:?}",
    //     &target_host
    // );
    // // info!(
    //     "handler target_port                       {:?}",
    //     &target_port
    // );
    // // info!(
    //     "handler target_schema                     {:?}",
    //     &target_schema
    // );
    // // info!(
    //     "handler request.uri().to_string()         {:?}",
    //     &request.uri().to_string()
    // );

    info!("full_path                         {:?}", &full_path);
    info!("target_host                       {:?}", &target_host);
    info!("target_port                       {:?}", &target_port);
    info!("target_method                     {:?}", &target_method);
    info!("target_schema                     {:?}", &target_schema);
    info!(
        "request.uri().to_string()         {:?}",
        &request.uri().to_string()
    );

    let proxy_url = format!(
        "{}://{}:{}{}",
        target_schema, target_host, target_port, full_path
    );

    let proxy_url = proxy_url.parse::<Uri>();
    if proxy_url.is_err() {
        let e = proxy_url.as_ref().err();
        error!(
            "error parsing URI from string {:?}. error {:?}",
            &proxy_url,
            e.unwrap()
        )
    }
    let proxy_url = proxy_url.unwrap();

    *request.uri_mut() = proxy_url.clone();

    let headers = request.headers_mut();
    // headers.insert(hyper::header::HOST, HeaderValue::from_str("bla").unwrap());
    let origin = format!("{}://{}::{}", target_schema, target_host, target_port);

    let header_value = HeaderValue::from_str(origin.as_str());

    if header_value.is_err() {
        let e = header_value.as_ref().err();
        error!(
            "error parsing header_value from string {:?}. error {:?}",
            &origin,
            e.unwrap()
        )
    }
    let header_value = header_value.unwrap();

    headers.insert(hyper::header::ORIGIN, header_value);
    //
    // let http_connector = hyper::client::HttpConnector::new();
    // let client = hyper::Client::builder().build(http_connector);

    let start = Instant::now();
    // info!("request uri {}", request.uri().to_string());
    let u = request.uri().to_string();
    let response = CLIENT.request(request).await;

    if response.is_err() {
        let e = response.as_ref().err();
        let msg = format!(
            "error response from request to url {} error {:?}",
            &proxy_url,
            e.unwrap()
        );
        error!("{}", &msg);
        let r2 = warp::reply::with_status::<String>(msg.into(), StatusCode::INTERNAL_SERVER_ERROR);
        let r2 = r2.into_response();
        return Ok(r2);
    }
    let mut response = response.unwrap();

    info!(
        "response status {}       for request uri {}   ",
        &response.status(),
        &u
    );
    info!(
        "response headers {:?}    for request uri {} ",
        &response.headers(),
        &u
    );

    let duration = start.elapsed();
    let d = format!(
        "duration {} ms, {} µs, {} ns ",
        duration.as_millis(),
        duration.as_micros(),
        duration.as_nanos()
    );
    info!("{} ", &d);
    response.headers_mut().insert(
        "x-duration",
        HeaderValue::from_str(&d).expect("add header should work"),
    );

    // response.headers_mut().insert(
    //     "access-control-allow-origin",
    //     HeaderValue::from_str(&"http://localhost:4011").unwrap(),
    // );

    response.headers_mut().insert(
        "x-provided-by",
        HeaderValue::from_str(target_description).unwrap(),
    );

    add_tracing_headers(x_inititated_by, start_total, &mut response);

    let update_target_stats_data = UpdateTargetStatsData {
        id: server_target_idx,
        duration_nanos: duration.as_nanos() as i128,
    };
    let cmd = ManagerCommand::UpdateTargetStats(update_target_stats_data);
    match sender.send(cmd) {
        Ok(_) => {}
        Err(e) => error!("error sending update stats {}", e),
    }

    Ok(response)
}

fn add_tracing_headers(x_initiated_by: bool, start_total: Instant, response: &mut Response<Body>) {
    let duration_total = start_total.elapsed();

    if x_initiated_by {
        //  println!("adding new X-initiated-by header");
        response.headers_mut().insert(
            HEADER_X_INITIATED_BY,
            HeaderValue::from_str("proxythingi").expect("initiated by should be a haeder value"),
        );
        let id = Uuid::new_v4();

        response.headers_mut().insert(
            HEADER_X_UUID,
            HeaderValue::from_str(&id.to_string()).expect("uuid  should be a haeder value"),
        );
    }
    let x_processed_by = response.headers().get(HEADER_X_PROCESSED_BY);
    let new_x_processed_by = match x_processed_by {
        Some(h) => {
            format!(
                "{} || proxythingi: dur {:?} μs",
                h.to_str().expect("h should be a str"),
                duration_total.as_micros()
            )
        }
        None => {
            format!(" proxythingi: dur {:?}", duration_total.as_micros())
        }
    };

    info!(
        "adding proxythingi to  X-processed-by header.     new header '{}'",
        &new_x_processed_by
    );

    response.headers_mut().insert(
        HEADER_X_PROCESSED_BY,
        HeaderValue::from_str(&new_x_processed_by)
            .expect("insert new_x_processed_by should succeed"),
    );
}

fn find_match<'a>(
    uri: &ProxyUri,
    proxy_config: &'a ProxyConfig,
    method: &Method,
) -> Option<&'a ServerSource> {
    // nice, funny demo
    // for s in &proxy_config.server_sources {
    //     // // info!("searching for request uri {}, method {}    comparing with config  path_starts_with  {} and method {}",
    //     //          &uri.as_str(), &method,&s.path_starts_with, &s.method);
    //     if uri.as_str().starts_with(&s.path_starts_with)
    //         && method.as_str().to_ascii_lowercase() == s.method.to_ascii_lowercase()
    //     {
    //         return Some(s);
    //     }
    // }
    proxy_config.server_sources.iter().find(|&s| {
        uri.as_str().starts_with(&s.path_starts_with) && method_matches_or_wildcard(method, s)
    })
}

fn method_matches_or_wildcard(method: &Method, s: &ServerSource) -> bool {
    s.method.eq("*") || method.as_str().to_ascii_lowercase() == s.method.to_ascii_lowercase()
}
