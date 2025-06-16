use dioxus::prelude::*;

#[component]
pub fn ProfileCard() -> Element {
    rsx! {
        div { class: "card",
            div { class: "flex items-center mb-4",
                div { class: "w-16 h-16 rounded-full bg-gray-200 mr-4" }
                div {
                    div { class: "text-lg font-medium", "张三" }
                    div { class: "text-gray-500", "骑手ID：123456" }
                }
            }
            div { class: "grid grid-cols-3 gap-4 text-center",
                div {
                    div { class: "text-lg font-medium", "128" }
                    div { class: "text-gray-500", "完成订单" }
                }
                div {
                    div { class: "text-lg font-medium", "4.9" }
                    div { class: "text-gray-500", "评分" }
                }
                div {
                    div { class: "text-lg font-medium", "¥1,280" }
                    div { class: "text-gray-500", "总收入" }
                }
            }
        }
    }
} 