use dioxus::prelude::*;
use dioxus_router::prelude::*;
// use crate::components::layout::Layout;
use crate::pages::{Home, Orders, Wallet, Profile, NotFound, Info};

#[derive(Routable, Clone)]
pub enum Route {
    // #[layout(Layout)]
    #[route("/")]
    Home {},
    #[route("/orders")]
    Orders {},
    #[route("/wallet")]
    Wallet {},
    #[route("/profile")]
    Profile {},
    #[route("/profile/info")]
    Info {},
    #[route("/:..route")]
    NotFound { route: Vec<String> },
}