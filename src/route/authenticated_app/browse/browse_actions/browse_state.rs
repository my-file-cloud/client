use dioxus::hooks::{use_signal, Action};
use dioxus::prelude::*;
use my_file_cloud_api::route::browse::BrowseResponseDTO;

#[derive(Clone)]
pub struct BrowseState {
    /// path is in format "path/to/something"
    pub path: Signal<String>,
    browse_action: Action<(String,), BrowseResponseDTO>,
}
impl BrowseState {
    pub fn new(path: String, browse_action: Action<(String,), BrowseResponseDTO>) -> Self {
        let path_signal = use_signal(|| path.clone());

        let state = Self {
            path: path_signal,
            browse_action,
        };

        state
    }

    pub fn update_path(&mut self, new_path: String) {
        self.path.set(new_path.clone());
        self.browse_action.call(new_path);
    }

    pub fn refresh(&mut self) {
        self.browse_action.call(self.path.cloned());
    }

    pub fn value(&self) -> Option<dioxus::Result<ReadSignal<BrowseResponseDTO>>> {
        self.browse_action.value()
    }
}