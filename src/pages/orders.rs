use dioxus::prelude::*;

pub fn Orders() -> Element {
    rsx! {
        div { class: "orders-container",
            // 顶部导航栏
            nav { class: "nav-bar",
                div { class: "nav-title", "我的订单" }
            }

            // 订单状态标签
            div { class: "order-tabs",
                OrderTab { text: "进行中", active: true }
                OrderTab { text: "已完成", active: false }
                OrderTab { text: "已取消", active: false }
            }

            // 订单列表
            div { class: "orders-list",
                OrderCard {
                    order_id: "2024031501".to_string(),
                    status: "配送中".to_string(),
                    time: "14:30".to_string(),
                    address: "北京市朝阳区xxx街道".to_string(),
                }
                OrderCard {
                    order_id: "2024031502".to_string(),
                    status: "待取货".to_string(),
                    time: "15:00".to_string(),
                    address: "北京市海淀区xxx路".to_string(),
                }
            }
        }
    }
}

#[component]
fn OrderTab(text: &'static str, active: bool) -> Element {
    let class = if active { "order-tab active" } else { "order-tab" };
    rsx! {
        div { class: class,
            span { "{text}" }
        }
    }
}

#[component]
fn OrderCard(order_id: String, status: String, time: String, address: String) -> Element {
    rsx! {
        div { class: "order-card",
            div { class: "order-header",
                span { class: "order-id", "订单号: {order_id}" }
                span { class: "order-status", "{status}" }
            }
            div { class: "order-time", "{time}" }
            div { class: "order-address", "{address}" }
            div { class: "order-actions",
                button { class: "action-btn", "查看详情" }
                button { class: "action-btn primary", "联系客户" }
            }
        }
    }
} 