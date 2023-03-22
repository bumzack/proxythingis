use tokio::sync::mpsc::UnboundedSender;
use warp::hyper::body::Bytes;
use warp::Filter;

use common::warp_request_filter::{
    extract_request_data_filter, ProxyHeaders, ProxyMethod, ProxyQueryParameters, ProxyUri,
};

use crate::config_manager::manager::ManagerCommand;
use crate::config_manager::server::with_sender;
use crate::proxy::server::execute_forward_request;

pub fn proxy_routes(
    manager_sender: UnboundedSender<ManagerCommand>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::any()
        .and(extract_request_data_filter())
        .and(with_sender(manager_sender.clone()))
        .and_then(
            |uri: ProxyUri,
             params: ProxyQueryParameters,
             proxy_method: ProxyMethod,
             headers: ProxyHeaders,
             body: Bytes,
             sender: UnboundedSender<ManagerCommand>| {
                execute_forward_request(uri, params, proxy_method, headers, body, sender)
            },
        )
}
