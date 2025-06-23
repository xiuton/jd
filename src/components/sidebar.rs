use dioxus::prelude::*;

#[component]
pub fn SidebarDrawer(
    on_close: EventHandler<()>,
    full_name: String,
    today_income: String,
    today_order_count: String,
    today_trip_count: String,
) -> Element {
    let mut show_delivery_type_modal = use_signal(|| false);
    let mut selected_delivery_type = use_signal(|| "众包配送".to_string());
    let mut current_slide = use_signal(|| 0);
    let mut show_full_name = use_signal(|| false);
    let slides = vec![
        ("小队6.16~6.22指数分活动", "活动日期 06-16 - 06-22", "🎁"),
        ("夏季防暑补贴", "高温下发,注意查收", "☀️"),
        ("新人冲单奖励", "额外奖励,等你来拿", "🚀"),
    ];
    let slides_len = slides.len();
    let masked_name = if let Some(first) = full_name.chars().next() {
        format!("{}*", first)
    } else {
        "*".to_string()
    };
    let display_name = if *show_full_name.read() {
        full_name.clone()
    } else {
        masked_name.clone()
    };

    use_future(move || async move {
        loop {
            gloo_timers::future::sleep(std::time::Duration::from_secs(3)).await;
            current_slide.with_mut(|i| *i = (*i + 1) % slides_len);
        }
    });

    rsx! {
        div { class: "fixed inset-0 z-50 flex",
            // 遮罩
            div {
                class: "fixed inset-0 bg-black/50",
                onclick: move |_| on_close.call(())
            }
            // 侧边栏内容
            div {
                class: "relative w-[90%] max-w-[350px] bg-white h-full shadow-lg flex flex-col",
                // 顶部固定区域
                div {
                    class: "bg-white px-2",
                    // 新的头部
                    div {
                        class: "flex justify-between items-center py-4",
                        button {
                            class: "flex items-center text-lg",
                            onclick: move |_| show_delivery_type_modal.toggle(),
                            span { "{selected_delivery_type}" }
                            svg {
                                class: "w-6 h-6 inline-block ml-1 align-middle",
                                view_box: "0 0 24 24",
                                circle { cx: "12", cy: "12", r: "11", fill: "black" }
                                // Arrow pointing right
                                path {
                                    d: "M8 9.5 H 14.5 M 12.5 7 L 15 9.5 L 12.5 12",
                                    stroke: "white",
                                    stroke_width: "1.5",
                                    fill: "none",
                                    stroke_linecap: "round",
                                    stroke_linejoin: "round"
                                }
                                // Arrow pointing left
                                path {
                                    d: "M16 14.5 H 9.5 M 11.5 17 L 9 14.5 L 11.5 12",
                                    stroke: "white",
                                    stroke_width: "1.5",
                                    fill: "none",
                                    stroke_linecap: "round",
                                    stroke_linejoin: "round"
                                }
                            }
                        }
                        button {
                            class: "text-gray-600",
                            // 设置图标
                            svg {
                                class: "w-6 h-6",
                                view_box: "0 0 24 24",
                                fill: "none",
                                stroke: "currentColor",
                                stroke_width: "1.5",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                 path { d: "M19.4 7.34L12 3.13 4.6 7.34v8.32L12 20.87l7.4-4.21V7.34z" }
                                 circle { cx: "12", cy: "12", r: "4" }
                            }
                        }
                    }
                }

                // 个人信息，仅在主视图显示
                if !*show_delivery_type_modal.read() {
                    div { class: "flex items-center justify-between gap-2 mb-3 px-2",
                        // Left part: avatar + name + eye
                        div { class: "flex items-center gap-2",
                            img {
                                class: "w-10 h-10 rounded-full",
                                src: "/public/images/avatar.jpg",
                                alt: "User Avatar"
                            }
                            div {
                                class: "flex items-center text-lg text-gray-800",
                                span {
                                    "{display_name}"
                                }
                                // 右侧直角箭头
                                svg {
                                    class: "w-5 h-5 mx-1",
                                    xmlns: "http://www.w3.org/2000/svg",
                                    fill: "none",
                                    view_box: "0 0 24 24",
                                    stroke: "currentColor",
                                    stroke_width: "2.2",
                                    stroke_linecap: "round",
                                    stroke_linejoin: "round",
                                    path { d: "M9 6l6 6-6 6" }
                                }
                                button {
                                    class: "flex items-center",
                                    onclick: move |_| show_full_name.toggle(),
                                    if *show_full_name.read() {
                                        // Open eye icon
                                        svg {
                                            class: "w-6 h-6",
                                            xmlns: "http://www.w3.org/2000/svg",
                                            fill: "none",
                                            view_box: "0 0 24 24",
                                            stroke_width: "1",
                                            stroke: "currentColor",
                                            path {
                                                stroke_linecap: "round",
                                                stroke_linejoin: "round",
                                                d: "M2.036 12.322a1.012 1.012 0 010-.639C3.423 7.51 7.36 4.5 12 4.5c4.638 0 8.573 3.007 9.963 7.178.07.207.07.431 0 .639C20.577 16.49 16.64 19.5 12 19.5c-4.638 0-8.573-3.007-9.963-7.178z"
                                            }
                                            path {
                                                stroke_linecap: "round",
                                                stroke_linejoin: "round",
                                                d: "M15 12a3 3 0 11-6 0 3 3 0 016 0z"
                                            }
                                        }
                                    } else {
                                        // Closed eye icon (standard eye-slash)
                                        svg {
                                            class: "w-6 h-6",
                                            xmlns: "http://www.w3.org/2000/svg",
                                            fill: "none",
                                            view_box: "0 0 24 24",
                                            stroke_width: "1",
                                            stroke: "currentColor",
                                            path {
                                                stroke_linecap: "round",
                                                stroke_linejoin: "round",
                                                d: "M3.98 8.223A10.477 10.477 0 001.934 12C3.226 16.338 7.244 19.5 12 19.5c.993 0 1.953-.138 2.863-.395M6.228 6.228A10.45 10.45 0 0112 4.5c4.756 0 8.774 3.162 10.065 7.498a10.523 10.523 0 01-4.293 5.774M6.228 6.228L3 3m3.228 3.228l3.65 3.65m7.894 7.894L21 21m-3.228-3.228l-3.65-3.65m0 0a3 3 0 10-4.243-4.243m4.243 4.243L6.228 6.228"
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        // Right part: badge
                        img {
                            class: "w-[85px]",
                            src: "/public/images/wangzhe.png"
                        }
                    }
                }

                // Flex container for the two main sections
                if *show_delivery_type_modal.read() {
                    DeliveryTypeSelection {
                        selected_type: selected_delivery_type,
                        on_select: move |new_type| {
                            selected_delivery_type.set(new_type);
                            show_delivery_type_modal.set(false);
                        }
                    }
                } else {
                    div {
                        class: "flex-1 flex flex-col overflow-y-hidden",

                        // 主体内容区域
                        div {
                            class: "flex-1 overflow-y-auto",
                            // 白色背景区域
                            div {
                                class: "bg-white px-2",
                                // 今日收入/单量
                                div {
                                    class: "flex gap-2 border-[1px] border-[#ffcdcd] rounded-md p-4",
                                    style: "background: linear-gradient(to right, #fee6e6, #fef3f3);",
                                    div { class: "flex-1 flex flex-col gap-2",
                                        span { class: "text-xs", "今日收入(元)" }
                                        span { class: "text-xl font-bold text-[#960001]", "{today_income}" }
                                        span { class: "text-xs text-[#703e3d]", "我的钱包 >" }
                                    }
                                    div { class: "flex-1 flex flex-col gap-2",
                                        span { class: "text-xs", "今日完单量(单)" }
                                        span { class: "text-xl font-bold text-[#960001]", "{today_order_count}", i { class: "text-xs text-[#703e3d] not-italic", "(含{today_trip_count}趟)" } }
                                        span { class: "text-xs text-[#703e3d]", "订单统计 >" }
                                    }
                                }
                                // 违规申诉 我的评价
                                div {
                                    class: "flex justify-around items-center bg-white p-2 rounded-lg text-sm text-gray-700 shadow-sm",
                                    a {
                                        href: "#",
                                        class: "flex items-center gap-2",
                                        svg {
                                            xmlns: "http://www.w3.org/2000/svg",
                                            class: "h-5 w-5 text-orange-500",
                                            fill: "none",
                                            view_box: "0 0 24 24",
                                            stroke: "currentColor",
                                            stroke_width: "1.5",
                                            path {
                                                stroke_linecap: "round",
                                                stroke_linejoin: "round",
                                                d: "M9 12.75L11.25 15 15 9.75m-3-7.036A11.959 11.959 0 013.598 6 11.99 11.99 0 003 9.749c0 5.592 3.824 10.29 9 11.622 5.176-1.332 9-6.03 9-11.622 0-1.31-.21-2.571-.598-3.751h-.152c-3.196 0-6.1-1.248-8.25-3.286zm0 13.036h.008v.008h-.008v-.008z"
                                            }
                                        }
                                        span { "违规申诉" }
                                    }
                                    div { class: "h-6 border-l border-gray-200" }
                                    a {
                                        href: "#",
                                        class: "flex items-center gap-2",
                                        svg {
                                            xmlns: "http://www.w3.org/2000/svg",
                                            class: "h-5 w-5 text-blue-500",
                                            fill: "none",
                                            view_box: "0 0 24 24",
                                            stroke: "currentColor",
                                            stroke_width: "1.5",
                                            path {
                                                stroke_linecap: "round",
                                                stroke_linejoin: "round",
                                                d: "M19.5 14.25v-2.625a3.375 3.375 0 00-3.375-3.375h-1.5A1.125 1.125 0 0113.5 7.125v-1.5a3.375 3.375 0 00-3.375-3.375H8.25m0 12.75h7.5m-7.5 3H12M10.5 2.25H5.625c-.621 0-1.125.504-1.125 1.125v17.25c0 .621.504 1.125 1.125 1.125h12.75c.621 0 1.125-.504 1.125-1.125V11.25a9 9 0 00-9-9z"
                                            }
                                        }
                                        span { "我的评价" }
                                    }
                                }
                            }

                            // 特殊背景区域
                            div {
                                class: "flex flex-col gap-2 bg-[#f8f8f8] p-2",
                                // 奖励 / 活动
                                div {
                                    class: "flex items-center gap-2",
                                    div {
                                        class: "flex flex-col bg-white rounded-md p-2",
                                        span { "奖励活动" }
                                        span { 
                                            class: "text-xs text-[#8c0304] rounded-full px-2 py-1",
                                            style: "background: linear-gradient(to right, #ffd9d8, #fef3f3);",
                                            "3个进行中"
                                        }
                                    }
                                    // 轮播效果
                                    div {
                                        class: "relative flex-1 flex flex-col bg-white rounded-md p-2 overflow-hidden",
                                        {
                                            slides.iter().enumerate()
                                                .find(|(i, _)| *i == *current_slide.read())
                                                .map(|(i, (title, subtitle, icon))| rsx! {
                                                    div {
                                                        key: "{i}",
                                                        class: "flex items-center justify-between",
                                                        div {
                                                            class: "text-sm",
                                                            p { class:"font-semibold truncate", "{title}"},
                                                            p { class:"text-xs text-gray-500", "{subtitle}"}
                                                        }
                                                        span { class: "text-2xl", "{icon}" }
                                                    }
                                                })
                                        }
                                        // 指示器
                                        div {
                                            class: "absolute bottom-1 left-0 right-0 flex justify-center space-x-1.5",
                                            for i in 0..slides.len() {
                                                button {
                                                    onclick: move |_| current_slide.set(i),
                                                    class: if i == *current_slide.read() { "w-2 h-1 bg-orange-500 rounded-full" } else { "w-1 h-1 bg-gray-300 rounded-full" },
                                                }
                                            }
                                        }
                                    }
                                }
                                // 进阶跑单
                                div {
                                    class: "flex flex-col gap-2 bg-white rounded-md p-4",
                                    p { class: "text-sm font-semibold", "进阶跑单" }
                                    div {
                                        class: "grid grid-cols-4 gap-2 pt-2",
                                        // 全职报名
                                        a { href: "#", class: "flex flex-col items-center gap-2 text-center",
                                            div { class: "w-10 h-10 bg-orange-100 rounded-full flex items-center justify-center",
                                                svg {
                                                    class: "w-6 h-6 text-orange-500",
                                                    xmlns: "http://www.w3.org/2000/svg", fill: "none", view_box: "0 0 24 24", stroke_width: "1.5", stroke: "currentColor",
                                                    path { stroke_linecap: "round", stroke_linejoin: "round", d: "M19 7.5v3m0 0v3m0-3h3m-3 0h-3m-2.25-4.125a3.375 3.375 0 11-6.75 0 3.375 3.375 0 016.75 0zM4 19.235v-.11a6.375 6.375 0 0112.75 0v.109A12.318 12.318 0 0110.5 22.5a12.318 12.318 0 01-6.5-3.265z" }
                                                }
                                            }
                                            span { class: "text-xs", "全职报名" }
                                        }
                                        // 骑士小队
                                        a { href: "#", class: "flex flex-col items-center gap-2 text-center",
                                            div { class: "w-10 h-10 bg-red-100 rounded-full flex items-center justify-center",
                                                svg {
                                                    class: "w-6 h-6 text-red-500",
                                                    xmlns: "http://www.w3.org/2000/svg", fill: "currentColor", view_box: "0 0 24 24",
                                                    path { d: "M4 15.5v-11a.5.5 0 01.5-.5H15a.5.5 0 01.5.5v6.5a.5.5 0 001 0V4a1.5 1.5 0 00-1.5-1.5H4.5A1.5 1.5 0 003 4v11.5a.5.5 0 001 0zM5.354 5.146a.5.5 0 10-.708.708L7.293 8.5l-2.647 2.646a.5.5 0 00.708.708L8 9.207l2.646 2.647a.5.5 0 00.708-.708L8.707 8.5l2.647-2.646a.5.5 0 00-.708-.708L8 7.793 5.354 5.146z" }
                                                }
                                            }
                                            span { class: "text-xs", "骑士小队" }
                                        }
                                        // 热门岗位
                                        a { href: "#", class: "flex flex-col items-center gap-2 text-center",
                                            div { class: "w-10 h-10 bg-blue-100 rounded-full flex items-center justify-center",
                                                svg {
                                                    class: "w-8 h-8 text-blue-500",
                                                    xmlns: "http://www.w3.org/2000/svg", fill: "none", view_box: "0 0 48 48", stroke_width: "3", stroke: "currentColor",
                                                    path { d: "M17 12c0-2.21 1.79-4 4-4s4 1.79 4 4-1.79 4-4 4-4-1.79-4-4zm-3 8c0-1.1.9-2 2-2h8c1.1 0 2 .9 2 2v16h-4v-8h-4v8H14V20zm20-8c-5.52 0-10 4.48-10 10s4.48 10 10 10 10-4.48 10-10-4.48-10-10-10zm-1 14h-3v-8h3v8zm0-10h-3v-2h3v2z", fill: "currentColor" }
                                                }
                                            }
                                            span { class: "text-xs", "热门岗位" }
                                        }
                                    }
                                }
                                // 工作必备
                                div {
                                    class: "flex flex-col gap-2 bg-white rounded-md p-4",
                                    p { class: "text-sm font-semibold", "工作必备" }
                                    div {
                                        class: "grid grid-cols-4 gap-y-4 pt-2",
                                        // 骑士学院
                                        a { href: "#", class: "flex flex-col items-center gap-2 text-center relative",
                                            div { class: "w-10 h-10 flex items-center justify-center",
                                                svg {
                                                    class: "w-8 h-8 text-gray-700",
                                                    xmlns: "http://www.w3.org/2000/svg", fill: "none", view_box: "0 0 24 24", stroke_width: "1.5", stroke: "currentColor",
                                                    path { stroke_linecap: "round", stroke_linejoin: "round", d: "M12 6.042A8.967 8.967 0 006 3.75c-1.052 0-2.062.18-3 .512v14.25A8.987 8.987 0 016 18c2.305 0 4.408.867 6 2.292m0-14.25a8.966 8.966 0 016-2.292c1.052 0 2.062.18 3 .512v14.25A8.987 8.987 0 0018 18a8.967 8.967 0 00-6 2.292m0-14.25v14.25" }
                                                }
                                            }
                                            span { class: "text-xs", "骑士学院" }
                                            span { class: "absolute -top-1 -right-1 bg-red-500 text-white text-[10px] px-1.5 py-0.5 rounded-full", "更新" }
                                        }
                                        // 小达直播
                                        a { href: "#", class: "flex flex-col items-center gap-2 text-center relative",
                                            div { class: "w-10 h-10 flex items-center justify-center",
                                                svg {
                                                    class: "w-8 h-8 text-gray-700",
                                                    xmlns: "http://www.w3.org/2000/svg", fill: "none", view_box: "0 0 24 24", stroke_width: "1.5", stroke: "currentColor",
                                                    path { stroke_linecap: "round", stroke_linejoin: "round", d: "M21 12a9 9 0 11-18 0 9 9 0 0118 0z" }
                                                    path { stroke_linecap: "round", stroke_linejoin: "round", d: "M15.91 11.672a.375.375 0 010 .656l-5.603 3.113a.375.375 0 01-.557-.328V8.887c0-.286.307-.466.557-.327l5.603 3.112z" }
                                                }
                                            }
                                            span { class: "text-xs", "小达直播" }
                                            span { class: "absolute -top-1 -right-1 bg-red-500 text-white text-[10px] px-1.5 py-0.5 rounded-full", "Hot" }
                                        }
                                        // 骑士装备
                                        a { href: "#", class: "flex flex-col items-center gap-2 text-center relative",
                                            div { class: "w-10 h-10 flex items-center justify-center",
                                                svg {
                                                    class: "w-8 h-8 text-gray-700",
                                                    xmlns: "http://www.w3.org/2000/svg", fill: "none", view_box: "0 0 24 24", stroke_width: "1.5", stroke: "currentColor",
                                                    path { stroke_linecap: "round", stroke_linejoin: "round", d: "M21 12a9 9 0 11-18 0 9 9 0 0118 0z" }
                                                    path { stroke_linecap: "round", stroke_linejoin: "round", d: "M15.91 11.672a.375.375 0 010 .656l-5.603 3.113a.375.375 0 01-.557-.328V8.887c0-.286.307-.466.557-.327l5.603 3.112z" }
                                                }
                                            }
                                            span { class: "text-xs", "骑士装备" }
                                            span { class: "absolute -top-1 -right-1 bg-orange-500 text-white text-[10px] px-1.5 py-0.5 rounded-md", "优先派" }
                                        }
                                        // 我的保险
                                        a { href: "#", class: "flex flex-col items-center gap-2 text-center",
                                            div { class: "w-10 h-10 flex items-center justify-center",
                                                svg {
                                                    class: "w-8 h-8 text-gray-700",
                                                    xmlns: "http://www.w3.org/2000/svg", fill: "none", view_box: "0 0 24 24", stroke_width: "1.5", stroke: "currentColor",
                                                    path { stroke_linecap: "round", stroke_linejoin: "round", d: "M9 12.75L11.25 15 15 9.75M21 12a9 9 0 11-18 0 9 9 0 0118 0z" }
                                                }
                                            }
                                            span { class: "text-xs", "我的保险" }
                                        }
                                        // 我的卡券
                                        a { href: "#", class: "flex flex-col items-center gap-2 text-center",
                                            div { class: "w-10 h-10 flex items-center justify-center",
                                                svg {
                                                    class: "w-8 h-8 text-gray-700",
                                                    xmlns: "http://www.w3.org/2000/svg", fill: "none", view_box: "0 0 24 24", stroke_width: "1.5", stroke: "currentColor",
                                                    path { stroke_linecap: "round", stroke_linejoin: "round", d: "M2.25 8.25h19.5M2.25 9h19.5m-16.5 5.25h6m-6 2.25h6m3-3.75l3 3m0 0l3-3m-3 3V1.5m-9 5.25h16.5a1.5 1.5 0 011.5 1.5v9a1.5 1.5 0 01-1.5 1.5H2.25a1.5 1.5 0 01-1.5-1.5v-9a1.5 1.5 0 011.5-1.5z" }
                                                }
                                            }
                                            span { class: "text-xs", "我的卡券" }
                                        }
                                        // 我的二维码
                                        a { href: "#", class: "flex flex-col items-center gap-2 text-center",
                                            div { class: "w-10 h-10 flex items-center justify-center",
                                                svg {
                                                    class: "w-8 h-8 text-gray-700",
                                                    xmlns: "http://www.w3.org/2000/svg", fill: "none", view_box: "0 0 24 24", stroke_width: "1.5", stroke: "currentColor",
                                                    path { stroke_linecap: "round", stroke_linejoin: "round", d: "M3.75 4.875c0-.621.504-1.125 1.125-1.125h4.5c.621 0 1.125.504 1.125 1.125v4.5c0 .621-.504 1.125-1.125 1.125h-4.5A1.125 1.125 0 013.75 9.375v-4.5zM3.75 14.625c0-.621.504-1.125 1.125-1.125h4.5c.621 0 1.125.504 1.125 1.125v4.5c0 .621-.504 1.125-1.125 1.125h-4.5a1.125 1.125 0 01-1.125-1.125v-4.5zM13.5 4.875c0-.621.504-1.125 1.125-1.125h4.5c.621 0 1.125.504 1.125 1.125v4.5c0 .621-.504 1.125-1.125 1.125h-4.5a1.125 1.125 0 01-1.125-1.125v-4.5z" }
                                                    path { stroke_linecap: "round", stroke_linejoin: "round", d: "M13.5 15.375a.75.75 0 01.75-.75h4.5a.75.75 0 01.75.75v4.5a.75.75 0 01-.75.75h-4.5a.75.75 0 01-.75-.75v-4.5z" }
                                                }
                                            }
                                            span { class: "text-xs", "我的二维码" }
                                        }
                                        // 联名装绑定
                                        a { href: "#", class: "flex flex-col items-center gap-2 text-center",
                                            div { class: "w-10 h-10 flex items-center justify-center",
                                                svg {
                                                    class: "w-8 h-8 text-gray-700",
                                                    xmlns: "http://www.w3.org/2000/svg", fill: "none", view_box: "0 0 24 24", stroke_width: "1.5", stroke: "currentColor",
                                                    path { stroke_linecap: "round", stroke_linejoin: "round", d: "M9 12.75L11.25 15 15 9.75M21 12a9 9 0 11-18 0 9 9 0 0118 0z" }
                                                }
                                            }
                                            span { class: "text-xs", "联名装绑定" }
                                        }
                                    }
                                }
                                // 达达商城 / 骑士关怀
                                div {
                                    class: "flex items-center gap-2",
                                    div {
                                        class: "flex-1 flex flex-col gap-2 bg-white rounded-md p-2 relative",
                                        span { class: "text-sm", "达达商城" }
                                        i { class: "text-xs not-italic", "穿装备享权益" }
                                        // 购物袋
                                        svg {
                                            xmlns: "http://www.w3.org/2000/svg",
                                            class: "h-5 w-5 absolute bottom-0 right-0",
                                            fill: "none",
                                            view_box: "0 0 24 24",
                                            stroke: "currentColor",
                                            stroke_width: "1.5",
                                            path {
                                                stroke_linecap: "round",
                                                stroke_linejoin: "round",
                                                d: "M16 11V7a4 4 0 00-8 0v4M5 9h14l1 12H4L5 9z"
                                            }
                                        }
                                    }
                                    div {
                                        class: "flex-1 flex flex-col gap-2 bg-white rounded-md p-2 relative",
                                        span { class: "text-sm", "骑士关怀" }
                                        i { class: "text-xs not-italic", "暖心常伴" }
                                        // 爱心
                                        svg {
                                            xmlns: "http://www.w3.org/2000/svg",
                                            class: "h-5 w-5 absolute bottom-0 right-0",
                                            fill: "none",
                                            view_box: "0 0 24 24",
                                            stroke: "currentColor",
                                            stroke_width: "1.5",
                                            path {
                                                stroke_linecap: "round",
                                                stroke_linejoin: "round",
                                                d: "M12 6.75a.75.75 0 110-1.5.75.75 0 010 1.5zM12 12.75a.75.75 0 110-1.5.75.75 0 010 1.5zM12 18.75a.75.75 0 110-1.5.75.75 0 010 1.5z"
                                            }
                                        }
                                    }
                                }
                                // 更多功能
                                div {
                                    class: "flex flex-col gap-2 bg-white rounded-md p-4",
                                    p { class: "text-sm font-semibold", "更多功能" }
                                    div {
                                        class: "grid grid-cols-4 gap-y-4 pt-2",
                                        // 超级会员
                                        a { href: "#", class: "flex flex-col items-center gap-2 text-center",
                                            div { class: "w-10 h-10 flex items-center justify-center",
                                                svg {
                                                    class: "w-8 h-8 text-yellow-500",
                                                    xmlns: "http://www.w3.org/2000/svg", fill: "currentColor", view_box: "0 0 24 24",
                                                    path { d: "M5.164 19.343a1.5 1.5 0 01-2.435-1.57l1.71-4.932a1.5 1.5 0 011.458-1.091h1.305a4.502 4.502 0 014.288 3.033l1.33 3.324a1.5 1.5 0 01-1.332 2.164H5.164zm13.78-7.252a1.5 1.5 0 01-1.458-1.091l-1.71-4.932a1.5 1.5 0 012.435-1.57l.11.11 3.42 3.42a1.5 1.5 0 010 2.12l-3.42 3.42-.11.11zM11.953 3.55a1.5 1.5 0 012.094 0l2.667 2.667a1.5 1.5 0 010 2.094L14.047 11a1.5 1.5 0 01-2.094 0L9.286 8.333a1.5 1.5 0 010-2.094L11.953 3.55z" }
                                                }
                                            }
                                            span { class: "text-xs", "超级会员" }
                                        }
                                        // 邀请有礼
                                        a { href: "#", class: "flex flex-col items-center gap-2 text-center relative",
                                            div { class: "w-10 h-10 flex items-center justify-center",
                                                svg {
                                                    class: "w-8 h-8 text-red-500",
                                                    xmlns: "http://www.w3.org/2000/svg", fill: "none", view_box: "0 0 24 24", stroke_width: "1.5", stroke: "currentColor",
                                                    path { stroke_linecap: "round", stroke_linejoin: "round", d: "M21 11.25v8.25a2.25 2.25 0 01-2.25 2.25H5.25A2.25 2.25 0 013 19.5v-8.25M12 4.875A2.625 2.625 0 1012 10.125A2.625 2.625 0 0012 4.875z" }
                                                    path { stroke_linecap: "round", stroke_linejoin: "round", d: "M12 12.75c-3.14 0-6.02.88-8.25 2.457m16.5 0A18.72 18.72 0 0012 12.75" }
                                                }
                                            }
                                            span { class: "text-xs", "邀请有礼" }
                                            span { class: "absolute -top-1 right-0 bg-gradient-to-r from-red-500 to-orange-400 text-white text-[10px] px-1.5 py-0.5 rounded-md", "领佣金" }
                                        }
                                        // 残障关怀
                                        a { href: "#", class: "flex flex-col items-center gap-2 text-center",
                                            div { class: "w-10 h-10 flex items-center justify-center",
                                                svg {
                                                    class: "w-8 h-8 text-gray-700",
                                                    xmlns: "http://www.w3.org/2000/svg", fill: "none", view_box: "0 0 24 24", stroke_width: "1.5", stroke: "currentColor",
                                                    path { stroke_linecap: "round", stroke_linejoin: "round", d: "M15.182 15.182a4.5 4.5 0 01-6.364 0M21 12a9 9 0 11-18 0 9 9 0 0118 0zM9.75 9.75c0 .414-.168.75-.375.75S9 10.164 9 9.75s.168-.75.375-.75.375.336.375.75zm4.5 0c0 .414-.168.75-.375.75S13.5 10.164 13.5 9.75s.168-.75.375-.75.375.336.375.75z" }
                                                }
                                            }
                                            span { class: "text-xs", "残障关怀" }
                                        }
                                        // 意见反馈
                                        a { href: "#", class: "flex flex-col items-center gap-2 text-center",
                                            div { class: "w-10 h-10 flex items-center justify-center",
                                                svg {
                                                    class: "w-8 h-8 text-gray-700",
                                                    xmlns: "http://www.w3.org/2000/svg", fill: "none", view_box: "0 0 24 24", stroke_width: "1.5", stroke: "currentColor",
                                                    path { stroke_linecap: "round", stroke_linejoin: "round", d: "M8.625 12a.375.375 0 11-.75 0 .375.375 0 01.75 0zm4.5 0a.375.375 0 11-.75 0 .375.375 0 01.75 0zm4.5 0a.375.375 0 11-.75 0 .375.375 0 01.75 0z" }
                                                    path { stroke_linecap: "round", stroke_linejoin: "round", d: "M12 20.25c4.97 0 9-3.694 9-8.25s-4.03-8.25-9-8.25S3 7.006 3 11.5c0 2.252.898 4.33 2.38 5.897L4.5 21l2.809-1.405A9.025 9.025 0 0012 20.25z" }
                                                }
                                            }
                                            span { class: "text-xs", "意见反馈" }
                                        }
                                        // 骑士话卡
                                        a { href: "#", class: "flex flex-col items-center gap-2 text-center relative",
                                            div { class: "w-10 h-10 flex items-center justify-center",
                                                svg {
                                                    class: "w-8 h-8 text-gray-700",
                                                    xmlns: "http://www.w3.org/2000/svg", fill: "none", view_box: "0 0 24 24", stroke_width: "1.5", stroke: "currentColor",
                                                    path { stroke_linecap: "round", stroke_linejoin: "round", d: "M10.5 1.5H8.25A2.25 2.25 0 006 3.75v16.5a2.25 2.25 0 002.25 2.25h7.5A2.25 2.25 0 0018 20.25V3.75a2.25 2.25 0 00-2.25-2.25H13.5m-3 0V3h3V1.5m-3 0h3m-3 18.75h3" }
                                                }
                                            }
                                            span { class: "text-xs", "骑士话卡" }
                                            span { class: "absolute top-0 right-0 bg-orange-200 text-orange-600 border border-orange-400 text-[10px] px-1 py-0.5 rounded-md", "0元领" }
                                        }
                                        // 租车换电
                                        a { href: "#", class: "flex flex-col items-center gap-2 text-center",
                                            div { class: "w-10 h-10 flex items-center justify-center",
                                                svg {
                                                    class: "w-8 h-8 text-gray-700",
                                                    xmlns: "http://www.w3.org/2000/svg", fill: "none", view_box: "0 0 24 24", stroke_width: "1.5", stroke: "currentColor",
                                                    path { stroke_linecap: "round", stroke_linejoin: "round", d: "M15 10.5a3 3 0 11-6 0 3 3 0 016 0z" }
                                                    path { stroke_linecap: "round", stroke_linejoin: "round", d: "M19.5 10.5c0 7.142-7.5 11.25-7.5 11.25S4.5 17.642 4.5 10.5a7.5 7.5 0 1115 0z" }
                                                    path { stroke_linecap: "round", stroke_linejoin: "round", d: "M12 10.5v-2.25m-1.5.075L12 6l1.5 2.25-1.5.075z" }
                                                }
                                            }
                                            span { class: "text-xs", "租车换电" }
                                        }
                                        // 落地配报名
                                        a { href: "#", class: "flex flex-col items-center gap-2 text-center",
                                            div { class: "w-10 h-10 flex items-center justify-center",
                                                svg {
                                                    class: "w-8 h-8 text-gray-700",
                                                    xmlns: "http://www.w3.org/2000/svg", fill: "none", view_box: "0 0 24 24", stroke_width: "1.5", stroke: "currentColor",
                                                    path { stroke_linecap: "round", stroke_linejoin: "round", d: "M9 12.75L11.25 15 15 9.75M21 12a9 9 0 11-18 0 9 9 0 0118 0z" }
                                                }
                                            }
                                            span { class: "text-xs", "落地配报名" }
                                        }
                                        // 拣货报名
                                        a { href: "#", class: "flex flex-col items-center gap-2 text-center",
                                            div { class: "w-10 h-10 flex items-center justify-center",
                                                svg {
                                                    class: "w-8 h-8 text-gray-700",
                                                    xmlns: "http://www.w3.org/2000/svg", fill: "none", view_box: "0 0 24 24", stroke_width: "1.5", stroke: "currentColor",
                                                    path { stroke_linecap: "round", stroke_linejoin: "round", d: "M3 3v1.5M3 21v-6m0 0l2.77-.693a9 9 0 016.208.682l.108.054a9 9 0 006.086.71l3.114-.732a4.5 4.5 0 014.136 3.894m-17.552 0c.035.146.06.292.086.439L4.5 21m0-6.833l.428-1.597a6.75 6.75 0 0111.48-1.45l.158.314A6.75 6.75 0 0119.5 12.5" }
                                                }
                                            }
                                            span { class: "text-xs", "拣货报名" }
                                        }
                                        // 配送商招募
                                        a { href: "#", class: "flex flex-col items-center gap-2 text-center",
                                            div { class: "w-10 h-10 flex items-center justify-center",
                                                svg {
                                                    class: "w-8 h-8 text-gray-700",
                                                    xmlns: "http://www.w3.org/2000/svg", fill: "none", view_box: "0 0 24 24", stroke_width: "1.5", stroke: "currentColor",
                                                    path { stroke_linecap: "round", stroke_linejoin: "round", d: "M12 4.5v15m7.5-7.5h-15" }
                                                }
                                            }
                                            span { class: "text-xs", "配送商招募" }
                                        }
                                        // 有奖寻线索
                                        a { href: "#", class: "flex flex-col items-center gap-2 text-center",
                                            div { class: "w-10 h-10 flex items-center justify-center",
                                                svg {
                                                    class: "w-8 h-8 text-gray-700",
                                                    xmlns: "http://www.w3.org/2000/svg", fill: "none", view_box: "0 0 24 24", stroke_width: "1.5", stroke: "currentColor",
                                                    path { stroke_linecap: "round", stroke_linejoin: "round", d: "M9 12.75L11.25 15 15 9.75M21 12a9 9 0 11-18 0 9 9 0 0118 0z" }
                                                }
                                            }
                                            span { class: "text-xs", "有奖寻线索" }
                                        }
                                        // 我要举报
                                        a { href: "#", class: "flex flex-col items-center gap-2 text-center",
                                            div { class: "w-10 h-10 flex items-center justify-center",
                                                svg {
                                                    class: "w-8 h-8 text-gray-700",
                                                    xmlns: "http://www.w3.org/2000/svg", fill: "none", view_box: "0 0 24 24", stroke_width: "1.5", stroke: "currentColor",
                                                    path { stroke_linecap: "round", stroke_linejoin: "round", d: "M16.862 4.487l1.687-1.688a1.875 1.875 0 112.652 2.652L10.582 16.07a4.5 4.5 0 01-1.897 1.13L6 18l.8-2.685a4.5 4.5 0 011.13-1.897l8.932-8.931zm0 0L19.5 7.125M18 14v4.75A2.25 2.25 0 0115.75 21H5.25A2.25 2.25 0 013 18.75V8.25A2.25 2.25 0 015.25 6H10" }
                                                }
                                            }
                                            span { class: "text-xs", "我要举报" }
                                        }
                                    }
                                }
                            }
                        }

                        // 服务中心
                        div { 
                            class: "p-2",
                            button {
                                class: "w-full flex items-center justify-center gap-2 p-3 border border-gray-200 rounded-lg text-gray-700",
                                svg {
                                    xmlns: "http://www.w3.org/2000/svg",
                                    class: "h-5 w-5",
                                    fill: "none",
                                    view_box: "0 0 24 24",
                                    stroke: "currentColor",
                                    stroke_width: "1.5",
                                    path {
                                        stroke_linecap: "round",
                                        stroke_linejoin: "round",
                                        d: "M3.75 6A2.25 2.25 0 016 3.75h2.25A2.25 2.25 0 0110.5 6v2.25a2.25 2.25 0 01-2.25 2.25H6A2.25 2.25 0 013.75 8.25V6zM3.75 15.75A2.25 2.25 0 016 13.5h2.25a2.25 2.25 0 012.25 2.25V18a2.25 2.25 0 01-2.25 2.25H6A2.25 2.25 0 013.75 18v-2.25zM13.5 6a2.25 2.25 0 012.25-2.25H18A2.25 2.25 0 0120.25 6v2.25A2.25 2.25 0 0118 10.5h-2.25a2.25 2.25 0 01-2.25-2.25V6zM13.5 15.75a2.25 2.25 0 012.25-2.25H18a2.25 2.25 0 012.25 2.25V18a2.25 2.25 0 01-2.25 2.25H13.5A2.25 2.25 0 0111.25 18v-2.25z"
                                    }
                                }
                                span { "服务中心" }
                            }
                        }
                    }
                }
            }
        }
    }
}
#[component]
fn DeliveryTypeSelection(selected_type: Signal<String>, on_select: EventHandler<String>) -> Element {
    let delivery_types = vec![
        ("众包配送", "同城商超、外卖、个人等订单即时配送"),
        ("驻店配送", "专业即时配送，按时出勤收入有保障"),
        ("落地配揽派", "为京东物流等平台派件、揽件"),
        ("超市拣货", "在周边超市/仓库进行拣货打包"),
        ("大件配送", "为京东及商超提供四轮配送服务"),
        ("智配服务", "向企业客户提供骑士、订单及物流配送管理系统"),
    ];

    rsx! {
        div {
            class: "bg-white flex-1 overflow-y-auto -mx-2",
            ul {
                class: "divide-y divide-gray-100",
                {delivery_types.into_iter().map(|(title, description)| {
                    let is_selected = *selected_type.read() == title;
                    rsx! {
                        li {
                            class: "p-4 flex justify-between items-center cursor-pointer hover:bg-gray-50",
                            onclick: move |_| on_select.call(title.to_string()),
                            div {
                                class: "flex flex-col",
                                p {
                                    class: "font-semibold text-gray-800",
                                    "{title}"
                                }
                                p {
                                    class: "text-sm text-gray-500",
                                    "{description}"
                                }
                            }
                            if is_selected {
                                // Checkmark Icon
                                svg {
                                    class: "w-6 h-6 text-orange-500",
                                    view_box: "0 0 24 24",
                                    fill: "none",
                                    stroke: "currentColor",
                                    stroke_width: "2.5",
                                    stroke_linecap: "round",
                                    stroke_linejoin: "round",
                                    path { d: "M20 6L9 17l-5-5" }
                                }
                            }
                        }
                    }
                })}
            }
        }
    }
} 
