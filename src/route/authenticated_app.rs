use dioxus::prelude::*;
use dioxus_router::{use_navigator, use_route, Outlet};
use dioxus_style::with_css;
use crate::api_client::ApiClient;
use crate::route::authenticated_app::header::Header;
use crate::route::Route;

pub mod header;
pub mod dashboard;
pub mod browse;

#[with_css(style, "src/route/authenticated_app.scss")]
#[component]
pub fn AuthenticatedApp() -> Element {
    let nav = use_navigator();
    let current_route = use_route::<Route>();
    let client = consume_context::<ApiClient>();
    
    use_effect(move || {
        if !(client.is_authenticated)() {
            let redirect_to = current_route.to_string();
            nav.push(Route::Login {
                redirect: Some(redirect_to)
            });
        }
    });

    rsx! {
        div {
            class: style::authenticated_app,
            
            Header {}
            
            main {
                class: style::main,
                
                Outlet::<Route> {}
            }
        }
    }
}