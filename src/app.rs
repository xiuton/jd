use dioxus::prelude::*;
use crate::router::Route;

pub fn app() -> Element {
    rsx! {
        Router::<Route> {}
    }
}