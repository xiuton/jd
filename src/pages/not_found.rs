use dioxus::prelude::*;

#[component]
pub fn NotFound(route: Vec<String>) -> Element {
    rsx! {
        div { class: "not-found",
            div { class: "not-found-content",
                // 404 Icon
                svg {
                    class: "not-found-icon",
                    xmlns: "http://www.w3.org/2000/svg",
                    view_box: "0 0 24 24",
                    fill: "none",
                    stroke: "currentColor",
                    stroke_width: "1.5",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                    path {
                        d: "M9.75 9.75l4.5 4.5m0-4.5l-4.5 4.5M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
                    }
                }
                h1 { 
                    span { "🤔 " }
                    "404 - 页面未找到"
                }
                p { class: "not-found-path", "请求的页面不存在: ",
                    span { class: "not-found-path-text", "/" }
                }
            }
        }
    }
} 