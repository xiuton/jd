use dioxus::prelude::*;
use crate::components::{HeaderBar, SidebarDrawer};

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
    let active_tab = use_signal(|| 0);
    let is_working = use_is_working();

    rsx! {
        div { class: "min-h-screen bg-[#f2f2f2] flex flex-col pb-20",
            // 顶部栏
            HeaderBar { on_menu_click: move |_| show_sidebar.set(true), is_working: is_working, active_tab: active_tab }
            // 订单列表
            OrderList { active_tab: *active_tab.read() }
            // 底部导航栏
            BottomBar { is_working: is_working }
            // 侧边栏抽屉
            if *show_sidebar.read() {
                SidebarDrawer {
                    on_close: move |_| show_sidebar.set(false),
                    full_name: "蔡徐坤".to_string(),
                    today_income: "643.70".to_string(),
                    today_order_count: "79".to_string(),
                    today_trip_count: "5".to_string(),
                }
            }
        }
    }
}

#[component]
fn OrderList(active_tab: i32) -> Element {
    let is_working = use_is_working();
    // 根据不同的 tab 显示不同的订单数据
    let orders = match active_tab {
        0 => vec![ // 新任务
            (
                "35分钟内",
                "16:54前",
                "3.75",
                "1个冲单奖",
                "2200",
                "拾阶面包 | 世纪公园店",
                "管城回族区美景天城(石化路南)",
                "1400",
                "未来路康桥知园西院3号楼一单元20**",
                "畅跑单",
                "食品小吃 · 1公斤",
                None,
            ),
            (
                "26分钟内",
                "16:46前",
                "4.1",
                "1个冲单奖",
                "2900",
                "蜜雪冰城（紫南花园店）",
                "郑州市管城回族区紫东路紫南花园一期商铺(紫东路57-13号)",
                "594",
                "河南郑州市管城回族区紫荆山南路街道郑州市紫东路121-1号，紫祥烟酒店商行",
                "必达单 畅跑单",
                "饮料 · 0.5公斤 · 1件",
                Some("门店订单:#84 [JD321680755174] 缺货时电话与我沟通"),
            ),
        ],
        1 => vec![ // 待取货
            (
                "35分钟内",
                "16:54前",
                "3.75",
                "1个冲单奖",
                "2200",
                "拾阶面包 | 世纪公园店",
                "管城回族区美景天城(石化路南)",
                "1400",
                "未来路康桥知园西院3号楼一单元20**",
                "畅跑单",
                "食品小吃 · 1公斤",
                None,
            ),
        ],
        2 => vec![ // 配送中
            (
                "26分钟内",
                "16:46前",
                "4.1",
                "1个冲单奖",
                "2900",
                "蜜雪冰城（紫南花园店）",
                "郑州市管城回族区紫东路紫南花园一期商铺(紫东路57-13号)",
                "594",
                "河南郑州市管城回族区紫荆山南路街道郑州市紫东路121-1号，紫祥烟酒店商行",
                "必达单",
                "饮料 · 0.5公斤 · 1件",
                Some("门店订单:#84 [JD321680755174] 缺货时电话与我沟通"),
            ),
        ],
        _ => vec![],
    };

    rsx! {
        div { class: "px-2 mt-2 space-y-2 flex-1 overflow-y-auto",
            {orders.into_iter().map(|(time, to_time, price, award, start_distance, start_name, start_addr, shop_distance, shop_name, tag, goods, note)| {
                rsx! {
                    OrderCard {
                        time: time,
                        to_time: to_time,
                        price: price,
                        award: award,
                        start_distance: start_distance,
                        start_name: start_name,
                        start_addr: start_addr,
                        shop_distance: shop_distance,
                        shop_name: shop_name,
                        tag: tag,
                        goods: goods,
                        note: note,
                        active_tab: active_tab,
                        is_working: is_working,
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
                class: "fixed inset-0 bg-black/40 z-40 flex items-center justify-center",
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
            class: "fixed bottom-0 left-0 right-0 bg-white flex items-center justify-between px-2 py-2 border-t border-[#e5e7eb] shadow z-20",
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
                    // 双弯曲箭头图标
                    svg {
                        class: "w-5 h-5 mr-2",
                        view_box: "0 0 24 24",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        // 第一个弯曲箭头
                        path { d: "M3 12a9 9 0 0 1 9-9 9.75 9.75 0 0 1 6.74 2.74L21 8" }
                        path { d: "M21 3v5h-5" }
                        // 第二个弯曲箭头
                        path { d: "M21 12a9 9 0 0 1-9 9 9.75 9.75 0 0 1-6.74-2.74L3 16" }
                        path { d: "M3 21v-5h5" }
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
    time: &'static str,
    to_time: &'static str,
    price: &'static str,
    award: &'static str,
    start_distance: &'static str,
    start_name: &'static str,
    start_addr: &'static str,
    shop_distance: &'static str,
    shop_name: &'static str,
    tag: &'static str,
    goods: &'static str,
    note: Option<&'static str>,
    active_tab: i32,
    is_working: Signal<bool>,
) -> Element {
    let mut arrived = use_signal(|| false);
    let disabled = !*is_working.read();
    let btn_disabled = if disabled { "opacity-50 cursor-not-allowed" } else { "" };
    rsx! {
        div { class: "bg-white rounded-xl p-3 flex flex-col gap-y-1.5",
            div { class: "flex justify-between items-center",
                div { class: "flex",
                    span { class: "text-orange-500 text-sm", "{time}" }
                    span { class: "text-gray-500 text-sm", "({to_time})送达" }
                }
                div { class: "flex items-baseline",
                    i { class: "text-[red] text-xs font-bold not-italic", "¥" }
                    span { class: "text-[red] text-lg font-bold", "{price}" }
                }
            }
            div {
                class: "flex justify-between items-center",
                span {
                    class: "bg-red-100 text-red-500 border border-red-500 px-1 py-0.1 rounded text-[10px] flex items-center",
                    svg {
                        class: "w-3 h-4 mr-1",
                        view_box: "0 0 24 24",
                        preserve_aspect_ratio: "none",
                        defs {
                            linearGradient {
                                id: "redPacketGradient",
                                x1: "0%", y1: "0%", x2: "100%", y2: "100%",
                                stop { offset: "0%", stop_color: "#ff7e5f" }
                                stop { offset: "100%", stop_color: "#feb47b" }
                            }
                        }
                        rect {
                            x: "3", y: "3",
                            width: "18", height: "18",
                            rx: "2",
                            fill: "url(#redPacketGradient)"
                        }
                        path {
                            d: "M3 12 C 8 10, 16 14, 21 12",
                            stroke: "#FFC107",
                            stroke_width: "1.5",
                            fill: "none",
                        }
                        circle {
                            cx: "12", cy: "12", r: "2.5",
                            fill: "#FFC107",
                        }
                    }
                    "{award}"
                }
            }
            div {
                class: "flex flex-col gap-y-2 mt-2",
                div {
                    class: "relative",
                    div {
                        class: "absolute left-0 top-[-9px] w-[30px] h-[calc(100%+50px)] bg-[#f8f8f8] rounded-full",
                    }
                    div {
                        class: "absolute top-[29px] left-[15px] w-[1px] h-[calc(100%-25px)] bg-[#a6a6a6]",
                    }
                    if start_distance.parse::<i32>().unwrap_or(0) >= 1000 {
                        div {
                            class: "absolute left-0 top-0 text-center w-[30px]",
                            span { class: "font-semibold block text-[12px] leading-none", {format!("{:.1}", start_distance.parse::<f64>().unwrap_or(0.0) / 1000.0)} }
                            span { class: "text-xs font-thin block text-gray-500 leading-none", "km" }
                        }
                    } else {
                        div {
                            class: "absolute left-0 top-0 text-center w-[30px]",
                            span { class: "font-semibold block text-[12px] leading-none", {format!("{}", start_distance)} }
                            span { class: "text-xs font-thin block text-gray-500 leading-none", "m" }
                        }
                    }
                    div {
                        class: "ml-[40px]",
                        div { class: "text-lg", "{start_name}" }
                        div { class: "text-xs text-gray-500", "{start_addr}" }
                    }
                }
                div {
                    class: "relative",
                    if shop_distance.parse::<i32>().unwrap_or(0) >= 1000 {
                        div {
                            class: "absolute left-0 top-0 text-center w-[30px]",
                            span { class: "font-semibold block text-[12px] leading-none", {format!("{:.1}", shop_distance.parse::<f64>().unwrap_or(0.0) / 1000.0)} }
                            span { class: "text-xs font-thin block text-gray-500 leading-none", "km" }
                        }
                    } else {
                        div {
                            class: "absolute left-0 top-0 text-center w-[30px]",
                            span { class: "font-semibold block text-[12px] leading-none", {format!("{}", shop_distance)} }
                            span { class: "text-xs font-thin block text-gray-500 leading-none", "m" }
                        }
                    }
                    div { class: "text-lg ml-[40px]", "{shop_name}" }
                }
            }
            div {
                class: "flex flex-col justify-between gap-y-1 ml-[40px]",
                div { class: "flex flex-wrap gap-1",
                    {tag.split(' ').map(|t| rsx! {
                        if t == "必达单" {
                            span { class: "bg-orange-100 text-orange-500 border border-orange-500 px-2 py-0.5 rounded text-xs", "{t}" }
                        } else if t == "畅跑单" {
                            span { class: "bg-blue-100 text-blue-500 border border-blue-500 px-2 py-0.5 rounded text-xs", "{t}" }
                        } else {
                            span { class: "bg-gray-100 text-gray-500 border border-gray-500 px-2 py-0.5 rounded text-xs", "{t}" }
                        }
                    })}
                }
                div { class: "text-xs text-gray-500 bg-gray-100 py-2 px-3 mt-1 rounded", "{goods}" }
                if let Some(n) = note {
                    div { class: "bg-yellow-100 text-yellow-800 py-2 px-3 rounded text-xs mt-1", "备注：{n}" }
                }
            }
            if active_tab == 0 {
                button {
                    class: format!("w-full mt-2 bg-orange-400 text-white py-2 rounded font-bold {}", btn_disabled),
                    disabled: disabled,
                    "接单"
                }
            } else if active_tab == 1 {
                if !*arrived.read() {
                    button {
                        class: format!("w-full mt-2 bg-orange-400 text-white py-2 rounded font-bold {}", btn_disabled),
                        disabled: disabled,
                        onclick: move |_| {
                            if !disabled {
                                arrived.set(true);
                            }
                        },
                        "确认到店"
                    }
                } else {
                    button {
                        class: format!("w-full mt-2 bg-green-500 text-white py-2 rounded font-bold {}", btn_disabled),
                        disabled: disabled,
                        "确认取货"
                    }
                }
            } else if active_tab == 2 {
                button {
                    class: format!("w-full mt-2 bg-green-500 text-white py-2 rounded font-bold {}", btn_disabled),
                    disabled: disabled,
                    "送达"
                }
            }
        }
    }
}