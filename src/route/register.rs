use dioxus::prelude::*;
use dioxus_router::{use_navigator, Link};
use my_file_cloud_api::route::auth::register::RegisterBody;
use crate::{ApiClient, Route};
use crate::component::input::button::Button;
use crate::component::input::password::PasswordInput;
use crate::component::input::text::TextInput;
use dioxus_style::with_css;

#[with_css(style, "src/route/login_register.scss")]
#[component]
pub fn Register() -> Element {
    let nav = use_navigator();
    
    let mut register_form = use_signal(|| RegisterBody{
        username: String::from("admin"),
        password: String::from("password"),
    });

    let mut error_msg = use_signal::<Option<String>>(|| None);
    
    rsx! {
        main {
            class: style::container,
            
            div {
                class: style::body,
                
                h1 {
                    margin: "5px",
                    padding: "5px",
                    "Register " 
                }
                
                if let Some(msg) = error_msg() {
                    p { "Error: {msg}" }
                }
                
                form {
                    display: "flex",
                    flex_direction: "column",
                    gap: "20px",
                    
                    div {
                        display: "flex",
                        flex_direction: "column",
                        
                        TextInput {
                            placeholder: "Name",
                            value: register_form().username,
                            on_input: move |e: Event<FormData>| {
                                register_form.write().username = e.value()
                            }
                        }
                        
                        PasswordInput {
                            placeholder: "Password",
                            value: register_form().password,
                            on_input: move |e: Event<FormData>| {
                                register_form.write().password = e.value()
                            }
                        }
                    }
                    
                    Button {
                        on_click: move |e: Event<MouseData>| {
                            e.prevent_default();
                            
                            async move {
                                match consume_context::<ApiClient>().register(register_form.read().clone()).await {
                                    Ok(_) => {
                                        nav.push(Route::Login { redirect: None });
                                    },
                                    Err(err) => error_msg.set(Some(err.to_string())),
                                }
                            }
                        },
                        "Register"
                    }
                    nav {
                        span { "Already have an account? " }
                        Link { 
                            color: "#0AE",
                            to: Route::Login { redirect: None }, 
                            "Login"
                        }
                    }
                }
            }
            
        }
    }
}