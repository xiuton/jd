use dioxus_router::prelude::Outlet;
use dioxus::prelude::*;
use crate::router::Route;
use crate::components::nav::NavBar;

#[component]
pub fn Layout() -> Element {
    rsx! {
        div { class: "app",
            NavBar {}
            div { class: "main",
                Outlet::<Route> {}
            }
        }
    }
} 