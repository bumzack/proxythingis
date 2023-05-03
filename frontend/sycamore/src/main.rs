use chrono::{DateTime, Utc};
use reqwasm::http::Request;
use serde::{Deserialize, Serialize};
use sycamore::prelude::*;
use sycamore::suspense::Suspense;

// API that counts visits to the web-page
const API_BASE_URL: &str = "http://localhost:3036/proxythingi/server";

#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct ServerSource {
    pub id: i32,
    pub description: String,
    pub path_starts_with: String,
    pub method: String,
    pub created: DateTime<Utc>,
    pub targets: Vec<ServerTarget>,
    //      pub stats: ServerSourceStats,
}

#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct ServerTarget {
    pub id: i32,
    pub description: String,
    pub schema: String,
    pub host: String,
    pub port: i32,
    pub path: String,
    pub method: String,
    //      pub stats: ServerTargetStats,
    pub active: bool,
    pub created: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
struct Visits {
    value: u64,
}

async fn fetch_visits(_id: &str) -> Result<Vec<ServerSource>, reqwasm::Error> {
    let url = format!("{}", API_BASE_URL);
    let resp = Request::get(&url).send().await?;

    let body = resp.json::<Vec<ServerSource>>().await?;
    Ok(body)
}

#[component]
async fn VisitsCount<G: Html>(cx: Scope<'_>) -> View<G> {
    let id = "sycamore-visits-counter";
    let visits = fetch_visits(id).await.unwrap_or_default();

    // view! { cx,
    //         (
    //             let templates = visits.iter().cloned().map(|post| {
    //                 let PostData { id, description } = post;
    //                 view! { cx,
    //                     li {
    //                         div()   {  description }
    //                     }
    //                 }
    //             }).collect();
    //             let templates = View::new_fragment(templates);
    //             view! { cx,
    //                 ul {
    //                     (templates)
    //                 }
    //             }
    //         )
    //     }

    struct PostData {
        id: String,
        description: String,
    }

    let post_list = Some(visits);

    view! { cx,
        (if let Some(post_list) = &post_list   {
            let templates = post_list.iter().cloned().map(|post| {
                let ServerSource { id, description, .. } = post;
               let x =  view! { cx,
                    li {
                        div{ (id)       (description) }
                    }
                };
               x
            }).collect();
            let templates = View::new_fragment(templates);
            view! { cx,
                ul {
                    (templates)
                }
            }
        }
        else {
            view! { cx,
                "Loading..."
            }
        })
    }
}

#[component]
fn App<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        div {
            p { "Page Visit Counter" }
            Suspense(fallback=view! { cx, "Loading..." }) {
                VisitsCount {}
            }
        }
    }
}

fn main() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Debug).unwrap();

    sycamore::render(|cx| view! { cx, App {} });
}
