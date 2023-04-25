use log::info;
use tokio::sync::mpsc::UnboundedSender;
use warp::Filter;

use common::warp_request_filter::{
    extract_request_data_filter_body_stream, ProxyHeaders, ProxyMethod, ProxyQueryParameters,
    ProxyUri,
};

use crate::config_manager::manager::ManagerCommand;
use crate::config_manager::server::with_sender;
use crate::proxy::server::execute_forward_request;

pub fn proxy_routes(
    manager_sender: UnboundedSender<ManagerCommand>,
) -> impl Filter<Extract=(impl warp::Reply, ), Error=warp::Rejection> + Clone {
    warp::any()
        .and(with_sender(manager_sender.clone()))
        .and(extract_request_data_filter_body_stream())
        // body: impl Stream<Item=Result<impl Buf, warp::Error>> + Send + 'static
        .and_then(
            |sender: UnboundedSender<ManagerCommand>,
             uri: ProxyUri,
             params: ProxyQueryParameters,
             proxy_method: ProxyMethod,
             headers: ProxyHeaders,
             body| {
                info!(
                    "some route matched and will be forwarded url. matched uri {}, method {}",
                    uri.as_str(),
                    &proxy_method.as_str()
                );

                execute_forward_request(uri, params, proxy_method, headers, body, sender)
            },
        )
}
