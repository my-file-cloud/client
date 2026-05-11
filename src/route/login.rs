use dioxus::core::Element;
use dioxus::core_macro::rsx;
use dioxus::prelude::*;
use dioxus_router::{use_navigator, Link};
use dioxus_style::with_css;
use my_file_cloud_api::route::auth::login::LoginBody;
use crate::{ApiClient, Route};
use crate::component::input::button::Button;
use crate::component::input::password::PasswordInput;
use crate::component::input::text::TextInput;

#[with_css(style, "src/route/login_register.scss")]
#[component]
pub fn Login(redirect: Option<String>) -> Element {
    let nav = use_navigator();
    let client = consume_context::<ApiClient>();
    
    let refresh = use_future(move || {
        let mut client = client.clone();
        let redirect = redirect.clone();

        async move {
            if *client.is_authenticated.read() {
                return
            }
            
            let nav = nav.clone();

            if let Ok(()) = client.refresh().await {
                if let Some(path) = redirect {
                    nav.push(path);
                } else {
                    nav.push(Route::Dashboard {});
                }
            };
        }
    });

    let mut login_form = use_signal(|| LoginBody{
        username: String::from("admin"),
        password: String::from("password"),
    });

    let nav = use_navigator();
    let mut error_msg = use_signal::<Option<String>>(|| None);

    rsx! {
        main {
            class: style::container,
            
            div {
                class: style::body,
                
                h1 {
                    margin: "5px",
                    padding: "5px",
                    "Login"
                }
                
                match *refresh.state().read() {
                    UseFutureState::Pending => rsx! { "Refreshing Session..." },
                    _ => rsx! {
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
                                    value: login_form().username,
                                    on_input: move |e: Event<FormData>| {
                                        login_form.write().username = e.value()
                                    }
                                }
                                
                                PasswordInput {
                                    placeholder: "Password",
                                    value: login_form().password,
                                    on_input: move |e: Event<FormData>| {
                                        login_form.write().password = e.value()
                                    }
                                }
                            }
                            Button {
                                on_click: move |e: Event<MouseData>| {
                                    e.prevent_default();
                                    
                                    async move {
                                        match consume_context::<ApiClient>().login(login_form.read().clone()).await {
                                            Ok(()) => { nav.push(Route::Dashboard {}); },
                                            Err(err) => error_msg.set(Some(err.to_string())),
                                        }
                                    }
                                },
                                "Login"
                            }
                            p {
                                span { "New here? " }
                                Link {
                                    color: "#0AE",
                                    to: Route::Register {}, 
                                    "Register" 
                                }
                                span { " a new account!" }
                            }
                        } 
                    },
                }
            }
        }
    }
}