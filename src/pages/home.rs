use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    let mut show_sidebar = use_signal(|| false);
    let mut active_tab = use_signal(|| 0);

    rsx! {
        div { class: "min-h-screen bg-gray-100 flex flex-col pb-20",
            // 顶部栏
            HeaderBar { on_menu_click: move |_| show_sidebar.set(true) }
            // 订单Tab
            OrderTabs { 
                active: *active_tab.read(),
                on_change: move |index| active_tab.set(index)
            }
            // 订单列表
            OrderList { active_tab: *active_tab.read() }
            // 底部导航栏
            BottomBar {}
            // 侧边栏抽屉
            if *show_sidebar.read() {
                SidebarDrawer { on_close: move |_| show_sidebar.set(false) }
            }
        }
    }
}

#[component]
fn HeaderBar(on_menu_click: EventHandler<()>) -> Element {
    rsx! {
        div { class: "bg-black px-4 py-3 flex items-center justify-between sticky top-0 z-20",
            div {
                class: "flex items-center",
                button {
                    class: "w-7 h-7 border border-slate-600 bg-black bg-opacity-50 text-white rounded-full flex items-center justify-center text-xl",
                    onclick: move |_| on_menu_click.call(()),
                    svg {
                        class: "w-5 h-5",
                        view_box: "0 0 24 24",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        path { d: "M4 6h16" }
                        path { d: "M4 12h16" }
                        path { d: "M4 18h16" }
                    }
                }
                div {
                    class: "flex items-center ml-2 border border-slate-600 rounded-full px-[5px] py-[3px] bg-black bg-opacity-50",
                    // 红色圆点图标带白色横杠
                    svg {
                        class: "w-5 h-5 mr-1",
                        view_box: "0 0 24 24",
                        circle { 
                            cx: "12",
                            cy: "12",
                            r: "10",
                            fill: "rgb(234, 67, 53)"
                        }
                        path { 
                            d: "M8 12h8",
                            stroke: "white",
                            stroke_width: "2",
                            stroke_linecap: "round"
                        }
                    }
                    span { 
                        class: "text-[15px] leading-none text-white", 
                        "已收工" 
                    }
                    // 倒三角图标
                    svg {
                        class: "w-4 h-4 ml-1",
                        view_box: "0 0 24 24",
                        path { 
                            d: "M12 16L6 10H18L12 16Z",
                            fill: "white"
                        }
                    }
                }
            }
            div {
                class: "flex items-center space-x-4",
                button {
                    class: "flex items-center text-gray-400",
                    // 地图路线图标
                    svg {
                        class: "w-5 h-5",
                        view_box: "0 0 24 24",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        path { d: "M3 7l6 6 4-4 8 8" }
                        path { d: "M14 3l7 0 0 7" }
                    }
                    span { class: "ml-1", "路线" }
                }
                button {
                    class: "flex items-center text-gray-400",
                    // 铃铛图标
                    svg {
                        class: "w-5 h-5",
                        view_box: "0 0 24 24",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        path { d: "M18 8A6 6 0 0 0 6 8c0 7-3 9-3 9h18s-3-2-3-9" }
                        path { d: "M13.73 21a2 2 0 0 1-3.46 0" }
                    }
                }
            }
        }
    }
}

#[component]
fn OrderTabs(active: i32, on_change: EventHandler<i32>) -> Element {
    let tabs = vec![
        ("新任务", "border-orange-500 text-orange-500", "text-gray-500"),
        ("待取货", "border-orange-500 text-orange-500", "text-gray-500"),
        ("配送中", "border-orange-500 text-orange-500", "text-gray-500"),
    ];

    rsx! {
        div { class: "flex bg-black border-b sticky top-[52px] z-10",
            {tabs.into_iter().enumerate().map(|(index, (text, active_class, inactive_class))| {
                let is_active = active == index as i32;
                let class_name = if is_active {
                    format!("flex-1 text-center py-2 cursor-pointer border-b-2 font-bold {}", active_class)
                } else {
                    format!("flex-1 text-center py-2 cursor-pointer {}", inactive_class)
                };
                rsx! {
                    div {
                        class: class_name,
                        onclick: move |_| on_change.call(index as i32),
                        "{text}"
                    }
                }
            })}
        }
    }
}

#[component]
fn OrderList(active_tab: i32) -> Element {
    // 根据不同的 tab 显示不同的订单数据
    let orders = match active_tab {
        0 => vec![ // 新任务
            (
                "35分钟内(16:54前)送达",
                "3.75",
                "拾阶面包 | 世纪公园店",
                "管城回族区美景天城(石化路南)",
                "2.2km",
                "未来路康桥知园西院3号楼一单元20**",
                "1.4km",
                "1个冲单奖",
                "食品小吃 · 1公斤",
                None,
            ),
            (
                "26分钟内(16:46前)送达",
                "4.1",
                "蜜雪冰城（紫南花园店）",
                "郑州市管城回族区紫东路紫南花园一期商铺(紫东路57-13号)",
                "2.9km",
                "河南郑州市管城回族区紫荆山南路街道郑州市紫东路121-1号，紫祥烟酒店商行",
                "594m",
                "1个冲单奖 必达单 畅跑单",
                "饮料 · 0.5公斤 · 1件",
                Some("门店订单:#84 [JD321680755174] 缺货时电话与我沟通"),
            ),
        ],
        1 => vec![ // 待取货
            (
                "15分钟内取货",
                "5.0",
                "肯德基 | 商都路店",
                "金水区商都路与花园路交叉口",
                "1.5km",
                "金水区国基路24号院",
                "800m",
                "必达单",
                "快餐 · 2件",
                None,
            ),
        ],
        2 => vec![ // 配送中
            (
                "10分钟内送达",
                "6.5",
                "麦当劳 | 文化路店",
                "金水区文化路与农业路交叉口",
                "0.5km",
                "文化路89号",
                "300m",
                "超时预警",
                "快餐 · 3件",
                Some("顾客等急了，请尽快送达"),
            ),
        ],
        _ => vec![],
    };

    rsx! {
        div { class: "px-2 mt-2 space-y-3 flex-1 overflow-y-auto",
            {orders.into_iter().map(|(time_left, price, shop, shop_addr, distance, user_addr, user_distance, tag, goods, note)| {
                rsx! {
                    OrderCard {
                        time_left: time_left,
                        price: price,
                        shop: shop,
                        shop_addr: shop_addr,
                        distance: distance,
                        user_addr: user_addr,
                        user_distance: user_distance,
                        tag: tag,
                        goods: goods,
                        note: note,
                    }
                }
            })}
        }
    }
}

#[component]
fn BottomBar() -> Element {
    rsx! {
        div { class: "fixed bottom-0 left-0 right-0 bg-white flex items-center justify-around px-4 py-2 border-t shadow z-20",
            div { class: "flex flex-col items-center text-orange-500", span { class: "text-xl", "📋" } span { class: "text-xs mt-1", "接单" } }
            div { class: "flex flex-col items-center text-gray-400", span { class: "text-xl", "💰" } span { class: "text-xs mt-1", "钱包" } }
            div { class: "flex flex-col items-center text-gray-400", span { class: "text-xl", "👤" } span { class: "text-xs mt-1", "我的" } }
            button { class: "bg-orange-500 text-white px-8 py-2 rounded-full font-bold -mt-8 shadow-lg text-lg", "开工" }
        }
    }
}

#[component]
fn OrderCard(
    time_left: &'static str,
    price: &'static str,
    shop: &'static str,
    shop_addr: &'static str,
    distance: &'static str,
    user_addr: &'static str,
    user_distance: &'static str,
    tag: &'static str,
    goods: &'static str,
    note: Option<&'static str>,
) -> Element {
    rsx! {
        div { class: "bg-white rounded-lg shadow p-4 space-y-2",
            div { class: "flex justify-between items-center",
                span { class: "text-orange-500 text-sm font-bold", "{time_left}" }
                span { class: "text-red-500 text-lg font-bold", "¥{price}" }
            }
            div { class: "flex text-xs text-gray-500 space-x-4",
                span { "{distance}" }
                span { "{user_distance}" }
            }
            div { class: "font-bold", "{shop}" }
            div { class: "text-xs text-gray-500", "{shop_addr}" }
            div { class: "text-xs text-gray-500", "{user_addr}" }
            div { class: "flex flex-wrap gap-1 mt-1",
                {tag.split(' ').map(|t| rsx! {
                    span { class: "bg-orange-100 text-orange-500 px-2 py-0.5 rounded text-xs", "{t}" }
                })}
            }
            div { class: "text-xs text-gray-500", "{goods}" }
            if let Some(n) = note {
                div { class: "bg-yellow-100 text-yellow-800 p-1 rounded text-xs mt-1", "{n}" }
            }
            button { class: "w-full mt-2 bg-orange-400 text-white py-2 rounded font-bold", "接单" }
        }
    }
}

#[component]
fn SidebarDrawer(on_close: EventHandler<()>) -> Element {
    rsx! {
        div { class: "fixed inset-0 z-50 flex",
            // 遮罩
            div {
                class: "fixed inset-0 bg-black bg-opacity-30",
                onclick: move |_| on_close.call(())
            }
            // 侧边栏内容
            div {
                class: "relative w-4/5 max-w-xs bg-white h-full shadow-lg p-4 overflow-y-auto flex flex-col",
                    // 个人信息
                    div { class: "flex items-center space-x-3 mb-4 mt-2",
                        div { class: "w-12 h-12 bg-orange-200 rounded-full flex items-center justify-center text-2xl font-bold", "李*" }
                        div {
                            div { class: "font-bold text-lg text-gray-800", "李*" }
                            div { class: "text-xs text-yellow-600 flex items-center gap-1 mt-1", 
                                span { class: "bg-yellow-100 px-2 py-0.5 rounded-full font-bold", "王者骑士" }
                            }
                        }
                    }
                    // 今日收入/单量
                    div { class: "flex gap-2 mb-4",
                        div { class: "flex-1 bg-red-50 rounded-xl p-3 flex flex-col items-center shadow",
                            span { class: "text-xs text-gray-500 mb-1", "今日收入(元)" }
                            span { class: "text-xl font-bold text-red-500", "138.70" }
                            span { class: "text-xs text-orange-500 mt-1", "我的钱包 >" }
                        }
                        div { class: "flex-1 bg-red-50 rounded-xl p-3 flex flex-col items-center shadow",
                            span { class: "text-xs text-gray-500 mb-1", "今日完单量(单)" }
                            span { class: "text-xl font-bold text-red-500", "16(含2趟)" }
                            span { class: "text-xs text-orange-500 mt-1", "订单统计 >" }
                        }
                    }
                    // 奖励活动
                    div { class: "bg-orange-100 rounded-lg flex items-center p-3 gap-3 shadow text-xs text-orange-700 mb-4 relative",
                        span { class: "bg-orange-400 text-white px-2 py-0.5 rounded-full text-xs absolute left-2 -top-2", "3个进行中" }
                        span { class: "text-lg", "🎁" }
                        span { class: "flex-1", "奖励活动 小队6.16~6.22指数..." }
                        span { class: "text-orange-400 font-bold", ">" }
                    }
                    // 快捷入口/功能区
                    div { class: "grid grid-cols-4 gap-3 mb-4 text-center text-xs",
                        div { class: "relative flex flex-col items-center", span { class: "text-2xl mb-1", "📝" } span { "全职报名" } }
                        div { class: "relative flex flex-col items-center", span { class: "text-2xl mb-1", "🚩" } span { "骑士小队" } }
                        div { class: "relative flex flex-col items-center", span { class: "text-2xl mb-1", "🔥" } span { "热门岗位" } }
                        div { class: "relative flex flex-col items-center", span { class: "text-2xl mb-1", "🎓" } span { "骑士学院" } }
                        div { class: "relative flex flex-col items-center", span { class: "text-2xl mb-1", "🎥" } span { "小达直播" } span { class: "absolute top-0 right-2 bg-red-500 text-white text-[10px] px-1 rounded-full", "Hot" } }
                        div { class: "relative flex flex-col items-center", span { class: "text-2xl mb-1", "🛡️" } span { "骑士装备" } }
                        div { class: "relative flex flex-col items-center", span { class: "text-2xl mb-1", "💼" } span { "我的保险" } }
                        div { class: "relative flex flex-col items-center", span { class: "text-2xl mb-1", "🎫" } span { "我的卡券" } }
                        div { class: "relative flex flex-col items-center", span { class: "text-2xl mb-1", "🔳" } span { "我的二维码" } }
                        div { class: "relative flex flex-col items-center", span { class: "text-2xl mb-1", "👕" } span { "联名装绑定" } }
                    }
                    // 其它功能
                    div { class: "grid grid-cols-3 gap-2 mb-4 text-center text-xs",
                        div { class: "flex flex-col items-center", span { class: "text-xl mb-1", "🛒" } span { "达达商城" } }
                        div { class: "flex flex-col items-center", span { class: "text-xl mb-1", "🤝" } span { "骑士关怀" } }
                        div { class: "flex flex-col items-center", span { class: "text-xl mb-1", "➕" } span { "更多功能" } }
                    }
                    // 服务中心
                    div { class: "mt-4 text-center text-gray-400 text-xs", "服务中心" }
            }
        }
    }
}