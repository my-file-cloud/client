use dioxus::prelude::*;
use dioxus_material_icons::MaterialIconStylesheet;
use dioxus_router::{Router};
use crate::api_client::ApiClient;
use crate::route::Route;

pub static STYLES: Asset = asset!("/assets/main.css");
pub static RESET: Asset = asset!("/assets/reset.css");

fn main() { launch(App); }

fn api_base_url() -> String {
    #[cfg(target_arch = "wasm32")]
    {
        let host = web_sys::window()
            .and_then(|w| w.location().hostname().ok())
            .unwrap_or_default();

        if host == "localhost" || host == "127.0.0.1" {
            "http://localhost:3000".to_string()
        } else {
            format!("https://api.{host}")
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        std::env::var("API_BASE_URL")
            .expect("API_BASE_URL must be set")
    }
}

pub mod component;
mod route;
mod api_client;

#[component]
fn App() -> Element {
    let client = ApiClient::new(api_base_url());
    use_context_provider(|| client);
    
    rsx! {
        link { href: STYLES, rel: "stylesheet" }
        link { href: RESET, rel: "stylesheet" }
        
        MaterialIconStylesheet {}
        Router::<Route> {}
    }
}
