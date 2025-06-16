use dioxus::prelude::*;

#[component]
pub fn OrderCard() -> Element {
    rsx! {
        div { class: "card",
            div { class: "flex justify-between items-center mb-2",
                span { class: "text-lg font-medium", "订单号：123456789" }
                span { class: "text-success", "已完成" }
            }
            div { class: "text-gray-600 mb-2",
                "配送地址：北京市朝阳区xxx街道xxx小区"
            }
            div { class: "text-gray-600 mb-2",
                "配送时间：2024-03-16 12:30"
            }
            div { class: "flex justify-between items-center",
                span { class: "text-primary", "¥15.00" }
                button { class: "btn btn-primary", "查看详情" }
            }
        }
    }
} 