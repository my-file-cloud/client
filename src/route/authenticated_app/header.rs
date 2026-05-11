use dioxus::prelude::*;
use dioxus_router::Link;
use dioxus_style::with_css;
use crate::api_client::ApiClient;
use crate::component::input::button::Button;
use crate::route::Route;

#[with_css(style, "src/route/authenticated_app/header.scss")]
#[component]
pub fn Header() -> Element {
    rsx! {
        header {
            class: style::body,
            
            Link {
                to: Route::Dashboard {}, 
                h1 {
                    style: "margin: 0; padding: 0; color: white; text-decoration: none",
                    "my-file-cloud" 
                }
            }
            
            Button {
                on_click: move |_| {
                    async move {
                        let res = consume_context::<ApiClient>().logout().await;
                        
                        if let Err(err) = res {
                            error!("{}", err.to_string());
                        }
                    }
                },
                "Logout"
            }
        }
            
    }
}