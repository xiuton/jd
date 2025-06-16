use dioxus::prelude::*;

#[component]
pub fn TransactionItem() -> Element {
    rsx! {
        div { class: "flex justify-between items-center py-3 border-b border-gray-200",
            div { class: "flex-1",
                div { class: "text-gray-800", "订单收入" }
                div { class: "text-sm text-gray-500", "2024-03-16 12:30" }
            }
            div { class: "text-success", "+15.00" }
        }
    }
} 