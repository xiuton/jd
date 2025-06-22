use dioxus::prelude::*;

#[component]
pub fn SidebarDrawer(on_close: EventHandler<()>) -> Element {
    let mut show_delivery_type_modal = use_signal(|| false);
    let mut selected_delivery_type = use_signal(|| "众包配送".to_string());
    let mut current_slide = use_signal(|| 0);
    let slides = vec![
        ("小队6.16~6.22指数分活动", "活动日期 06-16 - 06-22", "🎁"),
        ("夏季防暑补贴", "高温下发,注意查收", "☀️"),
        ("新人冲单奖励", "额外奖励,等你来拿", "🚀"),
    ];
    let slides_len = slides.len();

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
                class: "relative w-[90%] max-w-[350px] bg-white h-full shadow-lg flex flex-col px-2",
                // 顶部固定区域
                div {
                    class: "sticky top-0 z-10 bg-white",
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
                    div { class: "flex items-center gap-2 mb-3",
                        div { class: "w-10 h-10 bg-orange-200 rounded-full flex items-center justify-center text-2xl font-bold",
                            // 头像
                            img {
                                class: "w-10 h-10 rounded-full",
                                src: "/public/images/avatar.jpg",
                            }
                        }
                        div {
                            class: "flex-1 flex justify-between items-center",
                            div { class: "text-lg text-gray-800", "李*" }
                            div { class: "flex items-center gap-1 mt-1",
                                span { class: "bg-[#281f34] text-xs text-white px-2 py-0.5 rounded-md", "王者骑士" }
                            }
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
                            div {
                                class: "bg-white",
                                // 今日收入/单量
                                div { class: "flex gap-2 border-[1px] border-[#cd5959] bg-[#fee6e6] rounded-md p-4",
                                    div { class: "flex-1 flex flex-col gap-2",
                                        span { class: "text-xs", "今日收入(元)" }
                                        span { class: "text-xl font-bold text-[#960001]", "138.70" }
                                        span { class: "text-xs text-[#703e3d]", "我的钱包 >" }
                                    }
                                    div { class: "flex-1 flex flex-col gap-2",
                                        span { class: "text-xs", "今日完单量(单)" }
                                        span { class: "text-xl font-bold text-[#960001]", "16", i { class: "text-xs text-[#703e3d]", "(含2趟)" } }
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
                                            class: "text-xs text-[#703e3d] bg-gradient-to-r from-[#ffd9d8] to-[#fef3f3] rounded-full px-2 py-1",
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
                                                            p { class:"font-semibold", "{title}"},
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
                                    class: "flex flex-col gap-2 bg-white rounded-md p-2",
                                    p { class: "text-sm", "进阶跑单" }
                                    div {
                                        class: "flex items-center gap-2",
                                        div {
                                            class: "flex flex-col items-center gap-2",
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
                                                    d: "M12 4.5v15m7.5-7.5h-15"
                                                }
                                            }
                                            span { class: "text-xs", "全职报名" }
                                        }
                                        div {
                                            class: "flex flex-col items-center gap-2",
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
                                                    d: "M12 4.5v15m7.5-7.5h-15"
                                                }
                                            }
                                            span { class: "text-xs", "骑士小队" }
                                        }
                                        div {
                                            class: "flex flex-col items-center gap-2",
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
                                                    d: "M12 4.5v15m7.5-7.5h-15"
                                                }
                                            }
                                            span { class: "text-xs", "热门岗位" }
                                        }
                                    }
                                }
                                // 工作必备
                                div {
                                    class: "flex flex-col gap-2 bg-white rounded-md p-2",
                                    p { class: "text-sm", "工作必备" }
                                    div {
                                        class: "grid grid-cols-4 gap-2",
                                        div {
                                            class: "flex flex-col items-center gap-2",
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
                                                    d: "M12 4.5v15m7.5-7.5h-15"
                                                }
                                            }
                                            span { class: "text-xs", "骑士学院" }
                                        }
                                        div {
                                            class: "flex flex-col items-center gap-2",
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
                                                    d: "M12 4.5v15m7.5-7.5h-15"
                                                }
                                            }
                                            span { class: "text-xs", "小达直播" }
                                        }
                                        div {
                                            class: "flex flex-col items-center gap-2",
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
                                                    d: "M12 4.5v15m7.5-7.5h-15"
                                                }
                                            }
                                            span { class: "text-xs", "骑士装备" }
                                        }
                                        div {
                                            class: "flex flex-col items-center gap-2",
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
                                                    d: "M12 4.5v15m7.5-7.5h-15"
                                                }
                                            }
                                            span { class: "text-xs", "我的保险" }
                                        }
                                        div {
                                            class: "flex flex-col items-center gap-2",
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
                                                    d: "M12 4.5v15m7.5-7.5h-15"
                                                }
                                            }
                                            span { class: "text-xs", "我的卡券" }
                                        }
                                        div {
                                            class: "flex flex-col items-center gap-2",
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
                                                    d: "M12 4.5v15m7.5-7.5h-15"
                                                }
                                            }
                                            span { class: "text-xs", "我的二维码" }
                                        }
                                        div {
                                            class: "flex flex-col items-center gap-2",
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
                                                    d: "M12 4.5v15m7.5-7.5h-15"
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
                                        i { class: "text-xs", "穿装备享权益" }
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
                                        i { class: "text-xs", "暖心常伴" }
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
                                    class: "flex flex-col gap-2 bg-white rounded-md p-2",
                                    p { class: "text-sm", "更多功能" }
                                    div {
                                        class: "grid grid-cols-4 gap-2",
                                        div {
                                            class: "flex flex-col items-center gap-2",
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
                                                    d: "M12 4.5v15m7.5-7.5h-15"
                                                }
                                            }
                                            span { class: "text-xs", "超级会员" }
                                        }
                                        div {
                                            class: "flex flex-col items-center gap-2",
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
                                                    d: "M12 4.5v15m7.5-7.5h-15"
                                                }
                                            }
                                            span { class: "text-xs", "邀请有礼" }
                                        }
                                        div {
                                            class: "flex flex-col items-center gap-2",
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
                                                    d: "M12 4.5v15m7.5-7.5h-15"
                                                }
                                            }
                                            span { class: "text-xs", "残障关注" }
                                        }
                                        div {
                                            class: "flex flex-col items-center gap-2",
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
                                                    d: "M12 4.5v15m7.5-7.5h-15"
                                                }
                                            }
                                            span { class: "text-xs", "意见反馈" }
                                        }
                                        div {
                                            class: "flex flex-col items-center gap-2",
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
                                                    d: "M12 4.5v15m7.5-7.5h-15"
                                                }
                                            }
                                            span { class: "text-xs", "骑士话卡" }
                                        }
                                        div {
                                            class: "flex flex-col items-center gap-2",
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
                                                    d: "M12 4.5v15m7.5-7.5h-15"
                                                }
                                            }
                                            span { class: "text-xs", "租车租电" }
                                        }
                                        div {
                                            class: "flex flex-col items-center gap-2",
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
                                                    d: "M12 4.5v15m7.5-7.5h-15"
                                                }
                                            }
                                            span { class: "text-xs", "落地配报名" }
                                        }
                                        div {
                                            class: "flex flex-col items-center gap-2",
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
                                                    d: "M12 4.5v15m7.5-7.5h-15"
                                                }
                                            }
                                            span { class: "text-xs", "拣货报名" }
                                        }
                                        div {
                                            class: "flex flex-col items-center gap-2",
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
                                                    d: "M12 4.5v15m7.5-7.5h-15"
                                                }
                                            }
                                            span { class: "text-xs", "配送商招募" }
                                        }
                                        div {
                                            class: "flex flex-col items-center gap-2",
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
                                                    d: "M12 4.5v15m7.5-7.5h-15"
                                                }
                                            }
                                            span { class: "text-xs", "有奖寻线索" }
                                        }
                                        div {
                                            class: "flex flex-col items-center gap-2",
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
                                                    d: "M12 4.5v15m7.5-7.5h-15"
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
