use reqwasm::http::Request;
use sycamore::flow::Keyed;
use sycamore::prelude::*;
use sycamore::web::html::svg_tags::builder::view;

use common::models::{ProxyConfig, ServerSource};

// API that counts visits to the web-page
const API_BASE_URL: &str = "http://localhost:3036/proxythingi/stats";
const API_BASE_URL_: &str = "http:///proxy.proxythingi.at/proxythingi/server";

async fn fetch_server_sources() -> Result<Vec<ServerSource>, reqwasm::Error> {
    let url = API_BASE_URL.to_string();
    let resp = Request::get(&url).send().await?;

    let proxy_config = resp.json::<ProxyConfig>().await?;
    let body = proxy_config.server_sources;
    Ok(body)
}

#[component]
async fn LeftNavItems<G: Html>(cx: Scope<'_>) -> View<G> {
    // wtf   ¯\_(ツ)_/¯

    let app_state = use_context::<Signal<AppState>>(cx);
    let mut server = app_state.get().server.clone();

    let s = server
        .drain(..)
        .map(|s| s.get().as_ref().clone())
        .collect::<Vec<ServerSource>>();

    let iter = create_signal(cx, s);

    view! { cx,
        div {
            Keyed (
                iterable = iter,
                view =| cx, ServerSource {id, description, .. }  | view! { cx,
                    a(class="list-group-item list-group-item-action", href=format!("#list-item-{}",id)) {
                            ( description) "ID: " (id)
                    }
                },
                key =|server_source|  server_source.id,
            )
        }
    }
}

#[component]
async fn Header<G: Html>(cx: Scope<'_>) -> View<G> {
    view! { cx,
        header(class = "py-3 mb-3 border-bottom") {
            div(class = "container-fluid d-grid gap-3 align-items-center", style ="rid-template-columns: 1fr 2fr;") {
                span(class="navbar-brand mb-0 h1") {
                    "ProxyThingi"
                }
            }
        }
    }
}

#[component]
async fn ServerTargetStatsComp<G: Html>(cx: Scope<'_>) -> View<G> {
    view! {cx,
        h4 {
            "Target Stats"
        }
        br {}
        table(class="table table-target-stats") {
            thead{
                tr{
                    th (scope="col") {
                        "Hits"
                    }
                    th (scope="col") {
                        "Min dur"
                    }
                    th (scope="col") {
                        "Max dur"
                    }
                    th (scope="col") {
                        "Avg dur"
                    }
                    th (scope="col") {
                        "Start"
                    }
                    th (scope="col") {
                        "End"
                    }
                }
            }
            tbody{
                tr {
                    td (scope="col") {
                        "23"
                    }
                    td (scope="col") {
                        "1 ms"
                    }
                    td (scope="col") {
                        "2 ms"
                    }
                    td (scope="col") {
                        "20230412-121203"
                    }
                    td (scope="col") {
                        "20230412-121203"
                    }
                }
            }
        }
    }
}

#[component]
async fn ServerTargetsComp<G: Html>(cx: Scope<'_>) -> View<G> {
    view! {cx,
        h2 {
            "Targets"
        }
        div(class="targets") {
             div(class="accordion", id="accordionExample") {
                div(class = "accordion-item") {
                    h2(class="accordion-header") {
                        button(aria-controls="collapseOne",  aria-expanded="true", class="accordion-button", data-bs-target="#collapseOne1",  data-bs-toggle="collapse", type="button") {
                            "Target: Article Search Rust MicroServices - Solr (id: 1)"
                        }
                    }
                    div(class="accordion-collapse collapse", data-bs-parent="#accordionExample" ,     id="collapseOne1") {
                        div(class="accordion-body") {
                            div(class="card") {
                                div(class="card-body") {
                                    p {
                                        strong{
                                            "Target Config"
                                        }
                                    }
                                    table(class="table table-targets") {
                                        thead {
                                            tr {
                                                th (scope="col") {
                                                    "id"
                                                }
                                                th (scope="col") {
                                                    "description"
                                                }
                                                th (scope="col") {
                                                    "Path"
                                                }
                                                th (scope="col") {
                                                    "Method"
                                                }
                                                th (scope="col") {
                                                    "Server"
                                                }
                                                th (scope="col") {
                                                    "Port"
                                                }
                                                th (scope="col") {

                                                }
                                                th (scope="col") {

                                                }
                                            }
                                        }
                                        tbody {
                                            tr {
                                                td  {
                                                   "1"
                                                }
                                                td  {
                                                   "Rust Meilisearch"
                                                }
                                                td  {
                                                   "/api/v1/meili/search"
                                                }
                                                td  {
                                                   "*"
                                                }
                                                td  {
                                                   "locahost"
                                                }
                                                td  {
                                                   "18901"
                                                }
                                                td  {
                                                    button(class="btn btn-success", type="button") {
                                                        svg(class="bi bi-file-play", fill="currentColor" , height="16", viewBox="0 0 16 16",width="16", xmlns="http://www.w3.org/2000/svg") {
                                                            path(d="M6 10.117V5.883a.5.5 0 0 1 .757-.429l3.528 2.117a.5.5 0 0 1 0 .858l-3.528 2.117a.5.5 0 0 1-.757-.43z")
                                                            path(d="M4 0a2 2 0 0 0-2 2v12a2 2 0 0 0 2 2h8a2 2 0 0 0 2-2V2a2 2 0 0 0-2-2H4zm0 1h8a1 1 0 0 1 1 1v12a1 1 0 0 1-1 1H4a1 1 0 0 1-1-1V2a1 1 0 0 1 1-1z")
                                                        }
                                                    }
                                                }
                                                td  {
                                                    button(class="btn btn-btn-danger", type="button") {
                                                        svg(class="bi bi-pause-btn", fill="currentColor" , height="16", viewBox="0 0 16 16",width="16", xmlns="http://www.w3.org/2000/svg") {
                                                            path(d="M6.25 5C5.56 5 5 5.56 5 6.25v3.5a1.25 1.25 0 1 0 2.5 0v-3.5C7.5 5.56 6.94 5 6.25 5zm3.5 0c-.69 0-1.25.56-1.25 1.25v3.5a1.25 1.25 0 1 0 2.5 0v-3.5C11 5.56 10.44 5 9.75 5z")
                                                            path(d="M0 4a2 2 0 0 1 2-2h12a2 2 0 0 1 2 2v8a2 2 0 0 1-2 2H2a2 2 0 0 1-2-2V4zm15 0a1 1 0 0 0-1-1H2a1 1 0 0 0-1 1v8a1 1 0 0 0 1 1h12a1 1 0 0 0 1-1V4z")
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    ServerTargetStatsComp
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component(inline_props)]
async fn ServerSourceStatsComp<G: Html>(cx: Scope<'_>, source: RcSignal<ServerSource>) -> View<G> {
    let source2 = create_ref(cx, source.clone());
    let stats = || source2.get().stats.clone();
    let stats = stats();

    view! {cx,

        h4 {
            "Source Stats"
        }
        br {}
        table(class="table table-source-targets") {
            thead{
                tr{
                    th (scope="col") {
                        "id"
                    }
                    th (scope="col") {
                        "Hits"
                    }
                    th (scope="col") {
                        "Start"
                    }
                th (scope="col") {
                        "End"
                    }
                }
            }
            tbody{
                tr{
                    td (scope="col") {
                        (stats.id)
                    }
                     td (scope="col") {
                        (stats.hits)
                    }
                    td (scope="col") {
                         (format!("{:?}",stats.start))
                    }
                    td (scope="col") {
                         (format!("{:?}",stats.stop))
                    }
                }
            }
        }
    }
}

#[component(inline_props)]
async fn ServerSourceEntryComp<G: Html>(cx: Scope<'_>, source: RcSignal<ServerSource>) -> View<G> {
    let source2 = create_ref(cx, source.clone());
    let method = || source2.get().method.clone();
    let path = || source2.get().path_starts_with.clone();
    let description = || source2.get().description.clone();

    let method = method();
    let path = path();
    let description = description();

    view! {cx,
        div(class="card-header") {
            h1 {
                (description)
            }
        }
        div(class="card-body") {
            h2 {
                "Source Config"
            }
            br {}
            table(class="table") {
                thead{
                    tr{
                        th (scope="col") {
                            "URL Path"
                        }
                        th (scope="col") {
                            "Method"
                        }
                    }
                }
                tbody{
                    tr{
                        td (scope="col") {
                           (path)
                        }
                        td (scope="col") {
                            (method)
                        }
                    }
                }
            }
            ServerSourceStatsComp(source=source)
            ServerTargetsComp
        }
    }
}

#[component]
async fn ServerSourceComp<G: Html>(cx: Scope<'_>) -> View<G> {
    let app_state = use_context::<Signal<AppState>>(cx);
    let server = app_state.get().server.clone();

    let iter: &Signal<Vec<RcSignal<ServerSource>>> = create_signal(cx, server);

    view! {cx,
    div {
        Keyed (
            iterable = iter,
            view =| cx,  server_source| view! { cx,
                       ServerSourceEntryComp(source=server_source)
                    },
            key =|server_source| server_source.get().id,
            )
        }

    }
}

#[component]
async fn MainContent<G: Html>(cx: Scope<'_>) -> View<G> {
    let app_state = use_context::<Signal<AppState>>(cx);
    let server = app_state.get().server.clone();

    let iter = create_signal(cx, server);

    // let sources = create_memo(cx, || app_state.server.iter().cloned().collect::<Vec<_>>());

    view! { cx,
        div(class = "container-fluid") {
            div(class = "row") {
                div(class = "col-2") {
                    div(class = "list-group", id="list-example") {
                        LeftNavItems
                    }
                }
                div(class="col"){
                    div(class="d-flex justify-content-between flex-wrap flex-md-nowrap align-items-center pt-3 pb-2 mb-3 border-bottom") {
                        h1(class="h1"){
                            "h1"
                        }
                        div(class="btn-toolbar mb-2 mb-md-0"){
                            div(class="btn-group me-2"){
                                button(class="btn btn-sm btn-outline-secondary", type="button"){
                                    "Save Stats"
                                }
                            }
                        }
                    }
                    div(class ="scrollspy-example", data-bs-smooth-scroll="true", data-bs-spy="scroll",data-bs-target="#list-example", tabindex="0") {
                        div(class="card mb-4", id="list-item-1") {

                            ServerSourceComp

                        }
                    }
                }
            }
        }
    }
}

pub struct AppState {
    server: Vec<RcSignal<ServerSource>>,
}

#[component]
async fn App<G: Html>(cx: Scope<'_>) -> View<G> {
    let mut server_sources = fetch_server_sources().await.unwrap_or_default();
    server_sources.sort_by(|s, t| s.id.cmp(&t.id));

    let server_source = server_sources
        .drain(..)
        .map(|source| create_rc_signal(source))
        .collect::<Vec<RcSignal<ServerSource>>>();

    let app_state = AppState {
        server: server_source,
    };

    let app_state = create_signal(cx, app_state);

    create_effect(cx, || {
        println!("sources changed - new source_server vec");
        app_state
            .get()
            .server
            .iter()
            .for_each(|s| println!("source {:?}", s));
    });

    provide_context_ref(cx, app_state);

    let server_stats_vec = create_memo(cx, || {
        // TODO load stuff ?!
        println!("hi from create_memo server_stats_vec");
    });

    view! { cx,
        main {
            Header
            MainContent
        }

    }
}

// #[component]
// fn AppOld<G: Html>(cx: Scope) -> View<G> {
//     view! { cx,
//         div {
//             p { "Page Visit Counter" }
//             Suspense(fallback=view! { cx, "Loading..." }) {
//                 VisitsCount {}
//             }
//         }
//     }
// }

fn main() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Debug).unwrap();

    sycamore::render(|cx| view! { cx, App {} });
}
