use std::ops::Add;

use dioxus::core::Element;
use dioxus::core_macro::{component, rsx};
use dioxus::prelude::*;
use dioxus_material_icons::{MaterialIcon, MaterialIconColor};
use dioxus_router::{use_navigator, use_route};
use my_file_cloud_api::route::browse::{StorageContentDTO, StorageContentTypeDTO};
use crate::api_client::ApiClient;
use crate::component::input::button::{Button, ButtonLabel};
use crate::route::authenticated_app::browse::browse_actions::browse_state::BrowseState;
use crate::route::Route;

#[component]
pub fn FileView(path: ReadSignal<Vec<String>>, content: String) -> Element {
    rsx! {
        textarea {
            background_color: "#181818",
            readonly: "value",
            value: content,
            
            flex: 1,
        }
    }
}

#[component]
pub fn DirectoryView(path: ReadSignal<Vec<String>>, items: Vec<StorageContentDTO>) -> Element {
    let nav = use_navigator();
    let route: Route = use_route();
    let browse_state = consume_context::<BrowseState>();
    let client = consume_context::<ApiClient>();
    
    let mut selected_item = use_signal::<Option<String>>(|| None);
    
    rsx! {
        div {
            style: "display: flex; flex-direction: column",
            flex: 1,
            
            {items.into_iter().map(|item| {
                let client = client.clone();
                
                let mut browse_state = browse_state.clone();
                let route = route.to_string();
                
                let file_path = if path().is_empty() {
                    item.name.clone()
                } else {
                    format!("{}/{}", path().join("/"), item.name.clone())
                };
                
                let download_path = format!("http://localhost:3000/download/{file_path}");
                
                let is_selected = selected_item().is_some_and(|item| item == file_path);
                
                let mut delete_action = use_action(move |path: String| {
                let client = client.clone();
                    
                    async move {
                        let path = format!("/{}", path);
                    
                        client.delete(path).await
                    }
                });
                
                use_effect(move || {
                    if let Some(Ok(_)) = delete_action.value() {
                        browse_state.refresh();
                    }
                });
                
                
                if delete_action.pending() {
                    return rsx! {
                        div {
                            key: "{file_path}",
                            "Deleting"
                        }
                    }
                }
                
                let onclick_file_path = file_path.clone();
                let ondelete_file_path = file_path.clone();
                rsx! {
                    div {
                        display: "flex",
                        justify_content: "space-between",
                        align_items: "center",
                        background_color: if is_selected {
                            "red"
                        } else {
                            "green"
                        },
                        key: "{file_path}",
                        onclick: move |_| {
                            if !is_selected {
                                selected_item.set(Some(onclick_file_path.clone()));
                            } else {
                                selected_item.set(None);
                            }
                        },
                        ondoubleclick: move |_| {
                            nav.push({
                                let mut route = route.clone();
                                if !route.ends_with("/") {
                                    route = route.add("/");
                                }
                                selected_item.set(None);
                                
                                route.add(&item.name)
                            });
                        },
                        style: "display: flex; cursor: pointer",
                        div {
                            display: "flex",
                            align_items: "center",
                            MaterialIcon {
                                name: match item.storage_content_type {
                                    StorageContentTypeDTO::File => "insert_drive_file",
                                    StorageContentTypeDTO::Folder => "folder",
                                },
                                size: 16,
                                color: MaterialIconColor::Light,
                            }
                            span {
                                "{item.name}"
                            }
                        }
                        if is_selected {
                            div {
                                display: "flex",
                                align_items: "center",
                                a {
                                    id: "download_{download_path}",
                                    href: download_path,
                                    
                                    ButtonLabel {
                                        r#for: "download_{download_path}",
                                        
                                        MaterialIcon {
                                            name: "download",
                                            size: 16,
                                            color: MaterialIconColor::Light,
                                        }
                                    }
                                }
                                Button {
                                    on_click: move |_| {
                                        let a= delete_action.call(ondelete_file_path.clone());
                                        
                                        async move {
                                            a.await;
                                            
                                        }
                                    },
                                    MaterialIcon {
                                        name: "delete",
                                        size: 16,
                                        color: MaterialIconColor::Light,
                                    }
                                }
                            }
                        }
                    }
                }
            })}
        }
    }
}
