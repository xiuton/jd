use dioxus::prelude::*;
use crate::components::HeaderBar;

// 全局信号用于共享工作状态
thread_local! {
    static IS_WORKING: Signal<bool> = Signal::new(false);
}

pub fn use_is_working() -> Signal<bool> {
    IS_WORKING.with(|s| s.clone())
}

#[component]
pub fn Home() -> Element {
    let mut show_sidebar = use_signal(|| false);
    let mut active_tab = use_signal(|| 0);
    let is_working = use_is_working();

    rsx! {
        div { class: "min-h-screen bg-[#f3f3f3] flex flex-col pb-20",
            // 顶部栏
            HeaderBar { on_menu_click: move |_| show_sidebar.set(true), is_working: is_working }
            // 订单Tab
            OrderTabs { 
                active: *active_tab.read(),
                on_change: move |index| active_tab.set(index)
            }
            // 订单列表
            OrderList { active_tab: *active_tab.read() }
            // 底部导航栏
            BottomBar { is_working: is_working }
            // 侧边栏抽屉
            if *show_sidebar.read() {
                SidebarDrawer { on_close: move |_| show_sidebar.set(false) }
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
        div { class: "flex bg-black sticky top-[52px] z-10",
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
fn BottomBar(is_working: Signal<bool>) -> Element {
    let mut show_confirm = use_signal(|| false);
    rsx! {
        // 弹窗遮罩和内容
        if *show_confirm.read() {
            // 遮罩
            div {
                class: "fixed inset-0 bg-black bg-opacity-40 z-40 flex items-center justify-center",
                // 点击遮罩不关闭弹窗，必须点按钮
                // 内容
                div {
                    class: "bg-white rounded-xl shadow-lg p-6 w-80 max-w-full flex flex-col items-center z-50",
                    // 蓝色 info 图标
                    div {
                        class: "mb-3",
                        svg {
                            class: "w-12 h-12 text-blue-400 mx-auto",
                            view_box: "0 0 48 48",
                            fill: "none",
                            circle { cx: "24", cy: "24", r: "24", fill: "#E6F0FA" }
                            path { d: "M24 14a2 2 0 1 1 0 4 2 2 0 0 1 0-4zm-2 8a2 2 0 0 1 2-2h0a2 2 0 0 1 2 2v10a2 2 0 0 1-2 2h0a2 2 0 0 1-2-2V22z", fill: "#36A3F7" }
                        }
                    }
                    div { class: "text-lg font-bold mb-2 text-gray-800", "请确认开工" }
                    div { class: "text-gray-500 text-sm mb-5 text-center", "请确认已做好接单准备，开始工作后可以接单。" }
                    div { class: "flex w-full gap-3 mt-2",
                        button {
                            class: "flex-1 bg-orange-500 text-white rounded-lg py-2 font-bold text-base active:scale-95 transition-all",
                            onclick: move |_| {
                                is_working.set(true);
                                show_confirm.set(false);
                            },
                            "确认"
                        }
                        button {
                            class: "flex-1 border border-gray-300 text-gray-700 rounded-lg py-2 font-bold text-base active:scale-95 transition-all",
                            onclick: move |_| show_confirm.set(false),
                            "取消"
                        }
                    }
                }
            }
        }
        div { 
            class: "fixed bottom-0 left-0 right-0 bg-white flex items-center justify-between px-2 py-2 border-t shadow z-20",
            // 左侧"接单设置"
            button {
                class: "flex flex-col items-center justify-center text-gray-600 px-2 py-1",
                // 滑块图标
                svg {
                    class: "w-6 h-6 mb-1",
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
                span { class: "text-xs", "接单设置" }
            }
            // 右侧按钮
            if *is_working.read() {
                // 刷新按钮
                button {
                    class: "flex items-center justify-center border border-gray-300 text-gray-700 text-lg font-bold rounded-xl flex-1 h-12 ml-2 bg-white shadow transition-all duration-150 active:scale-95",
                    // 刷新(旋转箭头)图标
                    svg {
                        class: "w-5 h-5 mr-2 animate-spin-slow",
                        view_box: "0 0 24 24",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        path { d: "M4 4v6h6" }
                        path { d: "M20 20v-6h-6" }
                        path { d: "M5 19A9 9 0 0 1 19 5" }
                    }
                    span { "刷新" }
                }
            } else {
                // 开工按钮
                button {
                    class: "flex items-center justify-center bg-orange-500 text-white text-lg font-bold rounded-xl flex-1 h-12 ml-2 shadow transition-all duration-150 active:scale-95",
                    onclick: move |_| show_confirm.set(true),
                    // 向上箭头图标
                    svg {
                        class: "w-5 h-5 mr-2",
                        view_box: "0 0 24 24",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        path { d: "M12 19V5" }
                        path { d: "M5 12l7-7 7 7" }
                    }
                    span { "开工" }
                }
            }
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