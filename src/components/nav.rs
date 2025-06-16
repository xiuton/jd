use dioxus::prelude::*;

#[component]
pub fn NavBar() -> Element {
    rsx! {
        nav { class: "fixed bottom-0 left-0 right-0 bg-white border-t border-gray-200 z-1",
            div { class: "flex justify-around items-center h-16",
                a { href: "/", class: "flex flex-col items-center text-gray-600",
                    div { class: "text-xl mb-1", "🏠" }
                    span { class: "text-xs", "首页" }
                }
                a { href: "/orders", class: "flex flex-col items-center text-gray-600",
                    div { class: "text-xl mb-1", "📦" }
                    span { class: "text-xs", "订单" }
                }
                a { href: "/wallet", class: "flex flex-col items-center text-gray-600",
                    div { class: "text-xl mb-1", "💰" }
                    span { class: "text-xs", "钱包" }
                }
                a { href: "/profile", class: "flex flex-col items-center text-gray-600",
                    div { class: "text-xl mb-1", "👤" }
                    span { class: "text-xs", "我的" }
                }
            }
        }
    }
} 