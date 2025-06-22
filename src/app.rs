use dioxus::prelude::*;
use crate::router::Route;
use dioxus_router::prelude::Router;

pub fn app() -> Element {
    rsx! {
        Router::<Route> {}
    }
}