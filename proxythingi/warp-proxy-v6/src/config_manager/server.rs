use std::convert::Infallible;

use tokio::sync::mpsc::UnboundedSender;
use warp::Filter;

use crate::config_manager::manager::ManagerCommand;

pub fn with_sender(
    manager_sender: UnboundedSender<ManagerCommand>,
) -> impl Filter<Extract=(UnboundedSender<ManagerCommand>, ), Error=Infallible> + Clone {
    warp::any().map(move || manager_sender.clone())
}
