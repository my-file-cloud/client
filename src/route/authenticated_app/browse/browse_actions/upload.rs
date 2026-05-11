use dioxus::prelude::*;
use dioxus_material_icons::{MaterialIcon, MaterialIconColor};
use crate::api_client::ApiClient;
use crate::component::input::button::{ButtonLabel};
use crate::route::authenticated_app::browse::browse_actions::browse_state::BrowseState;
use crate::route::authenticated_app::browse::browse_actions::handle_files;

#[component]
pub fn Upload() -> Element {
    let client = consume_context::<ApiClient>();
    let browse_state = consume_context::<BrowseState>();
    
    rsx! {
        // style: "display: flex; flex-direction: column; padding: 5px; border: 2px solid black; width: fit-content",
        // ondragover: move |evt| evt.prevent_default(),
        // ondrop: {
        //     let client = client.clone();
        //     let browse_state = browse_state.clone();
        //     move |evt| {
        //         evt.prevent_default();
        // 
        //         let client = client.clone();
        //         let browse_state = browse_state.clone();     
        //         let files = evt.files();
        //         
        //         spawn(async move {
        //             handle_files(client, browse_state, files).await;
        //         });
        //     }
        // },
        div {
            input {
                id: "file-upload",
                display: "none",
                
                type: "file",
                multiple: true,
                onchange: {
                    let client = client.clone();    
                    let browse_state = browse_state.clone();  
                    move |evt| {
                        evt.prevent_default();
                        
                        let client = client.clone();
                        let browse_state = browse_state.clone();  
                        let files = evt.files();
                        
                        spawn(async move {
                            handle_files(client, browse_state, files).await;
                        });
                    }
                },
                "Upload File"
            }
            
            ButtonLabel {
                r#for: "file-upload",
                MaterialIcon {
                    name: "upload_file",
                    color: MaterialIconColor::Light,
                    size: 16,
                }
            }
        }
        div {
            input {
                id: "directory-upload",
                display: "none",
                    
                type: "file",
                directory: true,
                onchange: move |evt| {
                    evt.prevent_default();
                    
                    let client = client.clone();
                    let browse_state = browse_state.clone();  
                    let files = evt.files();
                    
                    spawn(async move {
                         handle_files(client, browse_state, files).await;
                    });
                },
                "Upload directory"
            }
            
            ButtonLabel {
                r#for: "directory-upload",
                MaterialIcon {
                    name: "drive_folder_upload",
                    color: MaterialIconColor::Light,
                    size: 16,
                }
            }
        }
    }
}
