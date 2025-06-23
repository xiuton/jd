use dioxus::prelude::*;
use dioxus_router::prelude::*;
use crate::router::Route;

pub fn Profile() -> Element {
    let nav = use_navigator();
    rsx! {
        div { class: "min-h-screen bg-[#fff6f6] flex flex-col",
            // 顶部导航栏
            div { class: "flex items-center h-14 px-4 border-b border-gray-100 relative bg-gradient-to-b from-[#ffe6e6] to-[#fff6f6]",
                a { href: "/", class: "absolute left-4 top-1/2 -translate-y-1/2 flex items-center", 
                    svg {
                        class: "w-6 h-6 text-gray-500",
                        xmlns: "http://www.w3.org/2000/svg",
                        fill: "none",
                        view_box: "0 0 24 24",
                        stroke: "currentColor",
                        stroke_width: "2.2",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        path { d: "M15 19l-7-7 7-7" }
                    }
                }
                div { class: "flex-1 text-center text-lg font-semibold", "个人中心" }
            }
            // 用户信息卡片
            div { class: "flex items-center gap-3 px-4 py-6 border-b border-gray-100 cursor-pointer", onclick: move |_| { nav.push(Route::Info {}); },
                img { class: "w-16 h-16 rounded-full", src: "/public/images/avatar.jpg", alt: "用户头像" }
                div { class: "flex flex-col gap-1 flex-1",
                    div { class: "flex items-center gap-2 text-lg font-semibold text-gray-800",
                        span { "李*" }
                        // 闭眼icon
                        svg {
                            class: "w-6 h-6 text-gray-400",
                            xmlns: "http://www.w3.org/2000/svg",
                            fill: "none",
                            view_box: "0 0 24 24",
                            stroke: "currentColor",
                            stroke_width: "1.5",
                            path {
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                d: "M3.98 8.223A10.477 10.477 0 001.934 12C3.226 16.338 7.244 19.5 12 19.5c.993 0 1.953-.138 2.863-.395M6.228 6.228A10.45 10.45 0 0112 4.5c4.756 0 8.774 3.162 10.065 7.498a10.523 10.523 0 01-4.293 5.774M6.228 6.228L3 3m3.228 3.228l3.65 3.65m7.894 7.894L21 21m-3.228-3.228l-3.65-3.65m0 0a3 3 0 10-4.243-4.243m4.243 4.243L6.228 6.228"
                            }
                        }
                    }
                    div { class: "flex gap-2 mt-1",
                        span { class: "bg-orange-400 text-white text-xs px-2 py-0.5 rounded-full", "优质骑士" }
                        span { class: "bg-cyan-400 text-white text-xs px-2 py-0.5 rounded-full", "信用骑士" }
                    }
                }
                svg { class: "w-5 h-5 text-gray-400", fill: "none", view_box: "0 0 24 24", stroke: "currentColor", stroke_width: "2.2", stroke_linecap: "round", stroke_linejoin: "round", path { d: "M9 6l6 6-6 6" } }
            }
            // 王者骑士卡片
            div { class: "bg-[#281f34] rounded-xl mx-4 mt-4 p-4 flex items-center gap-4 text-white",
                div { class: "flex-1",
                    div { class: "text-xl font-bold", "王者骑士" }
                    div { class: "text-xs mt-1", "生效日期：2025/6/16-2025/6/22" }
                    div { class: "text-lg mt-2 font-bold", "730 等级分" }
                }
                img { class: "w-20 h-20", src: "/public/images/wangzhe.png" }
            }
            // 权益
            div { class: "bg-white rounded-xl mx-4 mt-4 p-4",
                div { class: "flex justify-between items-center mb-2",
                    div { class: "font-semibold text-base text-gray-800", "可享权益" }
                    a { href: "#", class: "text-sm text-gray-400", "查看全部权益 >" }
                }
                div { class: "flex gap-4 mt-2",
                    div { class: "flex flex-col items-center flex-1",
                        img { class: "w-8 h-8 mb-1", src: "/public/images/benefit1.png" }
                        div { class: "text-xs text-gray-800", "+2单" }
                        div { class: "text-xs text-gray-400", "背单权限" }
                    }
                    div { class: "flex flex-col items-center flex-1",
                        img { class: "w-8 h-8 mb-1", src: "/public/images/benefit2.png" }
                        div { class: "text-xs text-gray-800", "2单/天" }
                        div { class: "text-xs text-gray-400", "一分钟免责取消" }
                    }
                    div { class: "flex flex-col items-center flex-1",
                        img { class: "w-8 h-8 mb-1", src: "/public/images/benefit3.png" }
                        div { class: "text-xs text-gray-800", "优先享受" }
                        div { class: "text-xs text-gray-400", "人文关怀" }
                    }
                }
            }
            // 服务质量
            div { class: "bg-white rounded-xl mx-4 mt-4 p-4",
                div { class: "font-semibold text-base text-gray-800 mb-2", "我的服务质量" }
                div { class: "flex gap-4",
                    div { class: "flex flex-col items-center flex-1",
                        img { class: "w-8 h-8 mb-1", src: "/public/images/star.png" }
                        div { class: "text-lg font-bold text-gray-800", "132" }
                        div { class: "text-xs text-gray-400", "推荐指数" }
                    }
                    div { class: "flex flex-col items-center flex-1",
                        img { class: "w-8 h-8 mb-1", src: "/public/images/service.png" }
                        div { class: "text-lg font-bold text-gray-800", "96" }
                        div { class: "text-xs text-gray-400", "服务分" }
                    }
                    div { class: "flex flex-col items-center flex-1",
                        img { class: "w-8 h-8 mb-1", src: "/public/images/benefit1.png" }
                        div { class: "text-lg font-bold text-gray-800", "8" }
                        div { class: "text-xs text-gray-400", "背单权限" }
                    }
                }
            }
            // 其他信息
            div { class: "bg-white rounded-xl mx-4 mt-4 p-4 mb-4",
                div { class: "font-semibold text-base text-gray-800 mb-2", "其他信息" }
                div { class: "divide-y divide-gray-100 text-base text-gray-800",
                    div { class: "flex items-center h-12", span { class: "flex-1", "健康证" } span { class: "text-gray-400 text-sm", "" } }
                    div { class: "flex items-center h-12", span { class: "flex-1", "车辆认证" } span { class: "text-blue-500 text-sm", "已认证 当前城市证件" } a { href: "#", class: "text-gray-400 text-xs ml-2", "去管理" } }
                    div { class: "flex items-center h-12", span { class: "flex-1", "装备餐箱" } span { class: "text-gray-400 text-sm", "" } }
                }
            }
        }
    }
}

#[component]
fn StatItem(label: &'static str, value: &'static str) -> Element {
    rsx! {
        div { class: "stat-item",
            div { class: "stat-value", "{value}" }
            div { class: "stat-label", "{label}" }
        }
    }
}

#[component]
fn FeatureItem(icon: &'static str, title: &'static str, description: &'static str) -> Element {
    rsx! {
        div { class: "feature-item",
            span { class: "feature-icon", "{icon}" }
            div { class: "feature-content",
                h3 { "{title}" }
                p { "{description}" }
            }
            span { class: "feature-arrow", ">" }
        }
    }
} 