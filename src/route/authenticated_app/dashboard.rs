use dioxus::prelude::*;
use dioxus_router::Link;
use crate::api_client::ApiClient;
use crate::route::Route;

#[component]
pub fn Dashboard() -> Element {
    let client = consume_context::<ApiClient>();

    let dashboard_data = use_resource(move || {
        let client = client.clone();
        async move {
            client.dashboard().await
        }
    });
    
    match dashboard_data.value().read().as_ref() {
        Some(Ok(res)) => {
            rsx! {
                h1 { "Welcome {res.name}" }
                nav {
                    Link { to: Route::Browse { path: Vec::new() }, "Browse"}
                }  
            }
        },
        Some(Err(err)) => {
            rsx! {
                "Error: {err}"
            }
        },
        None => {
            rsx! {
                "loading"
            }
        },
    }
}