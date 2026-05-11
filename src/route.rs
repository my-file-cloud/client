use crate::route::authenticated_app::browse::browse_actions::BrowseActions;
use crate::route::authenticated_app::browse::Browse;
use crate::route::authenticated_app::AuthenticatedApp;
use crate::route::authenticated_app::dashboard::Dashboard;
use crate::route::index::Index;
use crate::route::login::Login;
use crate::route::register::Register;
use dioxus::prelude::*;
use dioxus_router::{Routable};

mod index;
mod login;
mod register;
mod authenticated_app;

#[derive(Routable, Clone)]
#[rustfmt::skip]
pub enum Route {
    #[route("/")]
    Index {},

    #[route("/login?:redirect")]
    Login {
        redirect: Option<String>,
    },
    
    #[route("/register")]
    Register {},
    
    #[nest("/app")]
    #[layout(AuthenticatedApp)]
        #[route("/")]
        Dashboard {},
    
        #[nest("/browse")]
        #[layout(BrowseActions)]
            #[route("/:..path")]
            Browse { path: Vec<String> },
}
