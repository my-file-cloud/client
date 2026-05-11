use dioxus::prelude::*;
use dioxus_material_icons::MaterialIconStylesheet;
use dioxus_router::{Router};
use crate::api_client::ApiClient;
use crate::route::Route;

pub static STYLES: Asset = asset!("/assets/main.css");
pub static RESET: Asset = asset!("/assets/reset.css");

fn main() {launch(App); }

pub mod component;
mod route;
mod api_client;

#[component]
fn App() -> Element {
    let client = ApiClient::new(String::from("http://localhost:3000"));
    use_context_provider(|| client);
    
    rsx! {
        link { href: STYLES, rel: "stylesheet" }
        link { href: RESET, rel: "stylesheet" }
        
        MaterialIconStylesheet {}
        Router::<Route> {}
    }
}