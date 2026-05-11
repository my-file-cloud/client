use dioxus::prelude::*;
use my_file_cloud_api::route::browse::{BrowseResponseDTO};
use crate::route::authenticated_app::browse::browse_actions::browse_state::BrowseState;
use crate::route::authenticated_app::browse::view::{DirectoryView, FileView};

mod view;
pub mod browse_actions;

#[component]
pub fn Browse(path: ReadSignal<Vec<String>>) -> Element {
    let browse_state = consume_context::<BrowseState>();
    
    use_effect(move || {
        let mut browse_state = browse_state.clone();
        let path = path.cloned();

        browse_state.update_path(path.join("/"));
    });
    
    let browse_state = consume_context::<BrowseState>();
    
    rsx! {
        match browse_state.value() {
            Some(Ok(browse_result)) => {
                match browse_result.read().cloned() {
                    BrowseResponseDTO::File(content) => rsx! { FileView {
                        path,
                        content: content,
                    } },
                    BrowseResponseDTO::Directory(items) => rsx! { DirectoryView {
                        path,
                        items: items,
                    } },
                }
            },
            Some(Err(err)) => rsx! { "Error: {err}" },
            None => rsx! { "loading..." }
        }
    }
}


