use dioxus::prelude::*;
use dioxus::core::{consume_context, Element};
use dioxus::core_macro::{component, rsx};
use dioxus_router::{use_navigator, Link};
use crate::api_client::ApiClient;
use crate::route::Route;

#[component]
pub fn Index() -> Element {
    let nav = use_navigator();
    let client = consume_context::<ApiClient>();
    
    let redirect = if *client.is_authenticated.read() {
        Route::Dashboard {}
    } else {
        Route::Login { redirect: None }
    };
    
    nav.push(redirect);
    
    rsx! {
        nav {
            style: "display: flex; gap: 5px",
            Link { to: Route::Login { redirect: None }, "Login"}
            Link { to: Route::Register {}, "Register"}
        }
        h1 { "my-file-cloud" }
    }
}
