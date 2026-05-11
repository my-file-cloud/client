use dioxus::prelude::*;
use dioxus::core::Element;
use dioxus::core_macro::{rsx};
use dioxus_material_icons::{MaterialIcon, MaterialIconColor};
use dioxus_style::with_css;
use crate::api_client::ApiClient;
use crate::component::input::button::Button;
use crate::component::input::text::TextInput;
use crate::component::modal::Modal;
use crate::route::authenticated_app::browse::browse_actions::browse_state::BrowseState;

#[with_css(style, "src/route/authenticated_app/browse/browse_actions/create_folder.scss")]
#[component]
pub fn CreateFolder() -> Element {
    let mut form = use_signal::<Option<String>>(|| None);
    let client = consume_context::<ApiClient>();
    let browse_state = consume_context::<BrowseState>();
    
    rsx! {
        div {
            Button {
                on_click: move |_| {
                    form.set(Some(String::from("New Folder")));
                },
                MaterialIcon {
                    name: "create_new_folder",
                    color: MaterialIconColor::Light,
                    size: 16,
                }
            }
            
            if let Some(name) = form() {
                Modal {
                    div {
                        class: style::modal,
                        
                        h2 {
                            "Create Folder"
                        }
                        
                        TextInput {
                            on_input: move |e: Event<FormData>| {
                                form.set(Some(e.value()));
                            },
                            value: name.clone()
                        }
                        
                        div {
                            class: style::bottom,
                            
                            Button {
                                on_click: move |_| {
                                    let client = client.clone();
                                    
                                    let path = {
                                        let mut base = browse_state.path.cloned();
                                        if !base.is_empty() {
                                            base.insert(0, '/');
                                        }
                                        let path = format!("{base}/{name}");
                                        
                                        path
                                    };
                                    async move {
                                        let res = client.create_directory(path).await;
                                        let mut browse_state = consume_context::<BrowseState>();
                                        
                                        match res {
                                            Ok(_) => { 
                                                browse_state.refresh();
                                                form.set(None);
                                            },
                                            Err(err) => { error!("{err}"); },
                                        }
                                    }
                                },
                                "Create"
                            }
                            Button {
                                on_click: move |_| {
                                    form.set(None);
                                },
                                "Cancel"
                            }
                        }
                    }
                }
            }
        }
    }
}