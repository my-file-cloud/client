use dioxus::prelude::*;
use dioxus::core_macro::{component, rsx};
use dioxus::html::FileData;
use dioxus_router::{use_route, Link, Outlet};
use crate::api_client::ApiClient;
use crate::route::authenticated_app::browse::browse_actions::browse_state::BrowseState;
use crate::route::authenticated_app::browse::browse_actions::create_folder::CreateFolder;
use crate::route::authenticated_app::browse::browse_actions::upload::Upload;
use crate::route::Route;

pub mod browse_state;
pub mod create_folder;
pub mod upload;

#[component]
pub fn BrowseActions() -> Element {
    let client = consume_context::<ApiClient>();
    
    let browse_action = use_action(move |path: String| {
        let client = client.clone();
        
        async move {
            let path = if path.is_empty() {
                path
            } else {
                format!("/{path}")
            };

            client.browse(path).await
        }
    });
    
    let mut browse_state = BrowseState::new(String::new(), browse_action);
    
    use_context_provider(|| browse_state.clone());
    
    use_effect(move || {
        browse_state.refresh();
    });
    
    rsx! {
        div {
            display: "flex",
            align_items: "center",
            border_bottom: "1px solid white",
            
            BrowseNavigationBar {}
            
            CreateFolder {}
            Upload {}
        }
        Outlet::<Route> {}
    }
}

#[component]
fn BrowseNavigationBar() -> Element {
    let subpaths = match use_route::<Route>() {
        Route::Browse { path } => path,
        _ => Vec::new(),
    };

    let deepest_paths: Vec<_> = subpaths.iter()
        .enumerate()
        .rev()
        .take(3)
        .map(|(i, p)| (p, subpaths[..=i].to_vec()) )
        .rev()
        .collect();
    
    let deepest_paths_len = deepest_paths.len();
    
    rsx! {
        nav {
            style: "display: flex; gap: 5px; margin: 10px",
            span {
                 Link { 
                    to: Route::Browse { path: Vec::new() },
                    "mycloud"
                }
            }
            
            {deepest_paths.into_iter().enumerate().map(|(i, (name, relative_path))|{
                rsx! {
                    span { 
                        style: "cursor: default",
                        "/" 
                    }
                    span {
                        style: "cursor: pointer",
                        Link { 
                            to: Route::Browse { path: relative_path }, 
                            if i == 0 && deepest_paths_len > 2 { ".." } else { "{name}" }
                        }
                    }
                }
            })}
        }
    }
}

// TODO: handle file upload simultaneously, after one is done, refresh? or just after every is done? so a user can see a refreshed result faster, although not everything was uploaded yet
async fn handle_files(client: ApiClient, mut browse_state: BrowseState, files: Vec<FileData>) {
    let current_path = browse_state.path.read().clone();

    let files_uploads: Vec<_> = files.into_iter().map(async |file| {
        info!("dbg1");
        
        let parent = match file.path().parent() {
            None => String::new(),
            Some(parent) => parent.to_string_lossy().as_ref().to_string(),
        };

        let upload_path = {
            if parent.is_empty() {
                if current_path.is_empty() {
                    String::new()
                } else {
                    format!("/{current_path}")
                }
            } else {
                if current_path.is_empty() {
                    format!("/{parent}")
                } else {
                    format!("/{current_path}/{parent}")
                }
            }
        };

        info!("dbg2");
        let file = gloo_file::File::new(&file.name(), file.read_bytes().await.unwrap().as_ref());

        info!("dbg3");
        let client = client.clone();

        if let Err(err) = client.upload(upload_path.into(), file).await {
            error!("{err}");
        };
    }).collect();

    futures::future::join_all(files_uploads).await;

    browse_state.refresh();
}