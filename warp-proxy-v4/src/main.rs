extern crate lazy_static;

use std::convert::Infallible;
use std::env;
use std::str::FromStr;

use rand::Rng;
use tokio::sync::{mpsc, oneshot};
use tokio::sync::mpsc::UnboundedSender;
use tokio::time::Instant;
use tracing_subscriber::fmt::format::FmtSpan;
use warp::{Filter, hyper, method, Rejection, Reply};
use warp::http::{HeaderValue, Method, Request};
use warp::hyper::{Body, Uri};
use warp::hyper::body::Bytes;

use common::warp_request_filter::{extract_request_data_filter, ProxyHeaders, ProxyMethod, ProxyQueryParameters, ProxyUri};

use crate::config_manager::{GetConfigData, ManagerCommand, ProxyConfig, start_config_manager, UpdateSourceStatsData, UpdateTargetStatsData};
use crate::db::{create_pool, list_server};
use crate::hyper::Client;
use crate::hyper::client::HttpConnector;
use crate::models::{ServerSource, ServerTarget};
use crate::server::{activate_server_handler, create_source_handler, create_target_handler, deactivate_server_handler, list_servers_handler, stats_read_handler, stats_reset_handler, stats_store_handler, with_db, with_sender};

mod db;
mod models;
mod server;
mod config_manager;

// gotta give credit where credit is due and stuff
lazy_static::lazy_static! {
    static ref CLIENT: Client<HttpConnector> = {
         let http_connector = hyper::client::HttpConnector::new();
         let client = hyper::Client::builder().build(http_connector);
         return client;
    };
}


// #[tokio::main(worker_threads = 2)]
#[tokio::main]
async fn main() {
    let _result = dotenvy::from_filename("/Users/bumzack/stoff/rust/proxythingis/warp-proxy-v4/.env");

    if env::var_os("RUST_LOG").is_none() {
        // Set `RUST_LOG=todos=debug` to see debug logs,
        // this only shows access logs.
        env::set_var("RUST_LOG", "todos=info");
    }
    let pool = create_pool();


    let servers = list_server(pool.clone(), true).await.expect("loading the servers config should work");
    let proxy_config = ProxyConfig {
        server_sources: servers
    };

    let (manager_sender, manager_receiver) = mpsc::unbounded_channel();

    let _handle_config_manager = start_config_manager(proxy_config, manager_receiver);


    //pretty_env_logger::init();
    let filter = std::env::var("RUST_LOG").unwrap_or_else(|_| "tracing=info,warp=debug".to_owned());
    tracing_subscriber::fmt()
        // Use the filter we built above to determine which traces to record.
        .with_env_filter(filter)
        // Record an event when each span closes. This can be used to time our
        // routes' durations!
        .with_span_events(FmtSpan::CLOSE)
        .init();

    let stats = warp::path!("proxythingi" / "stats");
    let stats_read = stats
        .and(warp::get())
        .and(with_sender(manager_sender.clone()))
        .and_then(|sender| {
            stats_read_handler(sender)
        });

    let stats_store = stats
        .and(warp::post())
        .and(with_db(pool.clone()))
        .and(with_sender(manager_sender.clone()))
        .and_then(|pool, sender| {
            stats_store_handler(pool, sender)
        });

    let stats_reset = stats
        .and(warp::delete())
        .and(with_sender(manager_sender.clone()))
        .and_then(|sender| {
            stats_reset_handler(sender)
        });

    let stats_routes = stats_read
        .or(stats_store)
        .or(stats_reset);

    let server_source = warp::path!("proxythingi"  / "server" / "source");
    let server_source_create = server_source
        .and(warp::post())
        .and(warp::body::json())
        .and(with_db(pool.clone()))
        .and(with_sender(manager_sender.clone()))
        .and_then(|body, pool, sender| {
            create_source_handler(pool, body, sender)
        });

    let server_target = warp::path!("proxythingi" / "server" / "target");
    let server_target_create = server_target
        .and(warp::post())
        .and(warp::body::json())
        .and(with_db(pool.clone()))
        .and(with_sender(manager_sender.clone()))
        .and_then(|body, pool, sender| {
            create_target_handler(pool, body, sender)
        });

    let server = warp::path!("proxythingi" / "server");
    let server_list = server
        .and(warp::get())
        .and(with_db(pool.clone()))
        .and_then(list_servers_handler);

    let server = warp::path!("proxythingi" / "server" / "activate" / i32 );
    let server_activate = server
        .and(warp::get())
        .and(with_db(pool.clone()))
        .and_then(|id, pool| activate_server_handler(pool, id));

    let server = warp::path!("proxythingi" / "server" / "deactivate" / i32 );
    let server_deactivate = server
        .and(warp::get())
        .and(with_db(pool.clone()))
        .and_then(|id, pool| deactivate_server_handler(pool, id));

    let server_routes = server_source_create
        .or(server_target_create)
        .or(server_list)
        .or(server_activate)
        .or(server_deactivate)
        .or(stats_routes);

    let routes_proxy = warp::any()
        .and(extract_request_data_filter())
        .and(with_sender(manager_sender.clone()))
        .and_then(|uri: ProxyUri, params: ProxyQueryParameters, proxy_method: ProxyMethod, headers: ProxyHeaders, body: Bytes, sender: UnboundedSender<ManagerCommand>| {
            execute_forward_request(uri, params, proxy_method, headers, body, sender)
        });

    let routes = server_routes
        .or(routes_proxy);

    warp::serve(routes)
        .run(([127, 0, 0, 1], 3034))
        .await;
}

async fn execute_forward_request(uri: ProxyUri, params: ProxyQueryParameters, proxy_method: ProxyMethod, headers: ProxyHeaders, body: Bytes, sender: UnboundedSender<ManagerCommand>) -> Result<impl Reply, Rejection> {
    let (tx, rx) = oneshot::channel();
    let get_config_data = GetConfigData {
        sender: tx,
    };
    let cmd = ManagerCommand::GetConfig(get_config_data);
    sender.send(cmd).expect("execute_forward_request expected send successful");
    let proxy_config = rx.await.expect("execute_forward_request expected a valid proxy config");
    // println!("got a config!!!! {:?}", proxy_config);


    // println!("uri  {:?}", &uri);
    // match &params {
    //     Some(p) => println!("params  {:?}", p),
    //     None => println!("no params provided"),
    // }
    // println!("params  {:?}", &params);
    // println!("proxy_method  {:?}", &proxy_method);
    // println!("headers  {:?}", &headers);

    // is there a match for the uri in the config
    let source = find_match(&uri, &proxy_config, &proxy_method);
    let target: Option<&ServerTarget> = match source {
        Some(server) => {
            let targets = &server.targets;
            if targets.len() > 0 {
                let mut rng = rand::thread_rng();
                let i = rng.gen_range(0..targets.len());
                if i < 0 || i > 2 {
                    println!("random number WRONG between {} and {}: {}", 0, targets.len(), i);
                }
                let t: &ServerTarget = targets.get(i as usize).expect("cant unwrap target server");
                Some(t)
            } else {
                None
            }
        }
        None => {
            None
        }
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
    // println!("path_to_pass_on       {:?}", &path_to_pass_on);
    // println!("target_host           {:?}", &target_host);
    // println!("target_port           {:?}", &target_port);
    // println!("target_method         {:?}", &target_method);
    // println!("target_schema         {:?}", &target_schema);
    // println!("target_path           {:?}", &target_path);
    // println!("full_path  {:?}", &full_path);

    // println!("final path {:?}", &full_path);
    // println!("body empty {:?}", &body.is_empty());

    let m = Method::from_str(target_method).expect("cant determine method from str");

    let mut hyper_request = hyper::http::Request::builder()
        .method(m)
        .uri(full_path.clone())
        .body(hyper::body::Body::from(body))
        .expect("Request::builder() failed");
    {
        *hyper_request.headers_mut() = headers.clone();
    }

    let update_source_stats_data = UpdateSourceStatsData {
        id: 1,
    };
    let cmd = ManagerCommand::UpdateSourceStats(update_source_stats_data);
    sender.send(cmd).expect("expect the send with command UpdateSourceStats to work");

    let result = handler(hyper_request, sender, target.id, target_port, target_host, full_path, target_schema);

    let res = match result.await {
        Ok(response) => Ok(response),
        Err(e) => {
            // println!("error from client {}", e);
            Err(warp::reject::not_found())
        }
    };
    res
}

fn find_match<'a>(uri: &ProxyUri, proxy_config: &'a ProxyConfig, method: &Method) -> Option<&'a ServerSource> {
    for s in &proxy_config.server_sources {
        if uri.as_str().starts_with(&s.path_starts_with) && method.as_str().to_ascii_lowercase() == s.method.to_ascii_lowercase() {
            return Some(s);
        }
    }
    None
}

async fn handler(mut request: Request<Body>, sender: UnboundedSender<ManagerCommand>, server_target_idx: i32, target_port: &i32, target_host: &String, full_path: String, target_schema: &String) -> Result<impl warp::Reply, Infallible> {
    // println!("full_path                         {:?}", &full_path);
    // println!("target_host                       {:?}", &target_host);
    // println!("target_port                       {:?}", &target_port);
    // println!("target_method                     {:?}", &target_method);
    // println!("target_schema                     {:?}", &target_schema);
    // println!("request.uri().to_string()         {:?}", &request.uri().to_string());


    let proxy_url = format!("{}://{}:{}{}",
                            target_schema,
                            target_host,
                            target_port,
                            full_path
    );
    // println!("proxy_url         {:?}", &proxy_url);

    // let proxyUriForLogging = proxyUrl.clone();
    let proxy_url = proxy_url.parse::<Uri>().unwrap();
    *request.uri_mut() = proxy_url.clone();

    let headers = request.headers_mut();
    headers.insert(hyper::header::HOST, hyper::header::HeaderValue::from_str("bla").unwrap());
    let origin = format!("{}://{}::{}", target_schema, target_host, target_port);
    headers.insert(hyper::header::ORIGIN, hyper::header::HeaderValue::from_str(origin.as_str()).unwrap());
    //
    // let http_connector = hyper::client::HttpConnector::new();
    // let client = hyper::Client::builder().build(http_connector);

    let start = Instant::now();
    //println!("request uri {}", request.uri().to_string());
    let mut response = CLIENT.request(request).await.expect("Request failed");
    let duration = start.elapsed();
    let d = format!("duration {} ms, {} µs, {} ns ", duration.as_millis(), duration.as_micros(), duration.as_nanos());
    // println!("{} ", &d);
    response.headers_mut().insert("x-duration", HeaderValue::from_str(&d).unwrap());

    let update_target_stats_data = UpdateTargetStatsData {
        id: server_target_idx,
        duration_nanos: duration.as_nanos(),
    };
    let cmd = ManagerCommand::UpdateTargetStats(update_target_stats_data);
    sender.send(cmd).expect("expect the send with command UpdateTargetStats to work");

    Ok(response)
}
