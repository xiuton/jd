use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    let mut show_sidebar = use_signal(|| false);

    rsx! {
        div { class: "min-h-screen bg-gray-100 flex flex-col pb-20",
            // 顶部栏
            div { class: "bg-white shadow px-4 py-3 flex items-center justify-between sticky top-0 z-20",
                button {
                    class: "text-2xl text-gray-700 mr-2",
                    onclick: move |_| show_sidebar.set(true),
                    "≡"
                }
                span { class: "font-bold text-lg text-gray-800", "众包配送" }
                button { class: "text-xl text-gray-400", "路线" }
            }
            // 订单Tab
            div { class: "flex bg-white border-b sticky top-[56px] z-10",
                div { class: "flex-1 text-center py-2 border-b-2 border-orange-500 text-orange-500 font-bold", "新任务" }
                div { class: "flex-1 text-center py-2 text-gray-500", "待取货" }
                div { class: "flex-1 text-center py-2 text-gray-500", "配送中" }
            }
            // 订单卡片列表
            div { class: "px-2 mt-2 space-y-3 flex-1 overflow-y-auto pb-32",
                OrderCard {
                    time_left: "35分钟内(16:54前)送达",
                    price: "3.75",
                    shop: "拾阶面包 | 世纪公园店",
                    shop_addr: "管城回族区美景天城(石化路南)",
                    distance: "2.2km",
                    user_addr: "未来路康桥知园西院3号楼一单元20**",
                    user_distance: "1.4km",
                    tag: "1个冲单奖",
                    goods: "食品小吃 · 1公斤",
                    note: None,
                }
                OrderCard {
                    time_left: "26分钟内(16:46前)送达",
                    price: "4.1",
                    shop: "蜜雪冰城（紫南花园店）",
                    shop_addr: "郑州市管城回族区紫东路紫南花园一期商铺(紫东路57-13号)",
                    distance: "2.9km",
                    user_addr: "河南郑州市管城回族区紫荆山南路街道郑州市紫东路121-1号，紫祥烟酒店商行",
                    user_distance: "594m",
                    tag: "1个冲单奖 必达单 畅跑单",
                    goods: "饮料 · 0.5公斤 · 1件",
                    note: Some("门店订单:#84 [JD321680755174] 缺货时电话与我沟通"),
                }
            }
            // 底部导航栏
            div { class: "fixed bottom-0 left-0 right-0 bg-white flex items-center justify-around px-4 py-2 border-t shadow z-20",
                div { class: "flex flex-col items-center text-orange-500", span { class: "text-xl", "📋" } span { class: "text-xs mt-1", "接单" } }
                div { class: "flex flex-col items-center text-gray-400", span { class: "text-xl", "💰" } span { class: "text-xs mt-1", "钱包" } }
                div { class: "flex flex-col items-center text-gray-400", span { class: "text-xl", "👤" } span { class: "text-xs mt-1", "我的" } }
                button { class: "bg-orange-500 text-white px-8 py-2 rounded-full font-bold -mt-8 shadow-lg text-lg", "开工" }
            }
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
        div { class: "flex items-center justify-between px-4 py-2 bg-white shadow",
            button { class: "", onclick: move |_| on_menu_click.call(()), "≡" }
            span { class: "font-bold text-lg", "已收工" }
            button { class: "", "路线" }
        }
    }
}

#[component]
fn OrderTabs() -> Element {
    rsx! {
        div { class: "flex bg-white border-b",
            div { class: "flex-1 text-center py-2 border-b-2 border-orange-500 text-orange-500 font-bold", "新任务" }
            div { class: "flex-1 text-center py-2 text-gray-500", "待取货" }
            div { class: "flex-1 text-center py-2 text-gray-500", "配送中" }
        }
    }
}

#[component]
fn OrderList() -> Element {
    rsx! {
        div { class: "p-2 space-y-4 overflow-y-auto pb-24",
            OrderCard {
                time_left: "35分钟内(16:54前)送达",
                price: "3.75",
                shop: "拾阶面包 | 世纪公园店",
                shop_addr: "管城回族区美景天城(石化路南)",
                distance: "2.2km",
                user_addr: "未来路康桥知园西院3号楼一单元20**",
                user_distance: "1.4km",
                tag: "1个冲单奖",
                goods: "食品小吃 · 1公斤",
                note: None,
            }
            OrderCard {
                time_left: "26分钟内(16:46前)送达",
                price: "4.1",
                shop: "蜜雪冰城（紫南花园店）",
                shop_addr: "郑州市管城回族区紫东路紫南花园一期商铺(紫东路57-13号)",
                distance: "2.9km",
                user_addr: "河南郑州市管城回族区紫荆山南路街道郑州市紫东路121-1号，紫祥烟酒店商行",
                user_distance: "594m",
                tag: "1个冲单奖 必达单 畅跑单",
                goods: "饮料 · 0.5公斤 · 1件",
                note: Some("门店订单:#84 [JD321680755174] 缺货时电话与我沟通"),
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
fn BottomBar() -> Element {
    rsx! {
        div { class: "fixed bottom-0 left-0 right-0 bg-white flex items-center justify-between px-4 py-2 border-t shadow z-10",
            button { class: "text-orange-500", "接单设置" }
            button { class: "bg-orange-500 text-white px-8 py-2 rounded-full font-bold", "开工" }
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