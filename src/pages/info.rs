use dioxus::prelude::*;

pub fn Info() -> Element {
    rsx! {
        div { class: "min-h-screen bg-white flex flex-col",
            // 顶部导航栏
            div { class: "flex items-center h-14 px-4 border-b border-gray-100 relative",
                // 返回按钮
                a { href: "/profile", class: "absolute left-4 top-1/2 -translate-y-1/2 flex items-center", 
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
                div { class: "flex-1 text-center text-lg font-semibold", "我的信息" }
            }
            // 头像和姓名
            div { class: "flex items-center justify-between px-4 py-6 border-b border-gray-100",
                div { class: "flex items-center gap-3",
                    img { class: "w-14 h-14 rounded-full", src: "/public/images/avatar.jpg", alt: "头像" }
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
                }
            }
            // 信息列表
            div { class: "divide-y divide-gray-100 text-base text-gray-800",
                div { class: "flex items-center px-4 h-14", span { class: "flex-1", "姓名" } span { "李*" } }
                div { class: "flex items-center px-4 h-14", span { class: "flex-1", "身份证号码" } span { "4** ************ 0" } }
                div { class: "flex items-center px-4 h-14", span { class: "flex-1", "登陆手机号" } span { "195****0233" } }
                div { class: "flex items-center px-4 h-14", span { class: "flex-1", "紧急联系人" } span { class: "text-gray-400", "" } }
            }
        }
    }
} 