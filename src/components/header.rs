use dioxus::prelude::*;
use std::cell::RefCell;

thread_local! {
    static LAST_CONFIRM_DATE: RefCell<Option<String>> = RefCell::new(None);
}

#[component]
fn WorkingIcon() -> Element {
    rsx! {
        circle { 
            cx: "12",
            cy: "12",
            r: "10",
            fill: "rgb(34, 197, 94)" // Green color
        }
        path { 
            d: "M8 12l3 3 6-6",
            stroke: "white",
            stroke_width: "2",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            fill: "none"
        }
    }
}

#[component]
fn NotWorkingIcon() -> Element {
    rsx! {
        circle { 
            cx: "12",
            cy: "12",
            r: "10",
            fill: "rgb(234, 67, 53)" // Red color
        }
        path { 
            d: "M8 12h8",
            stroke: "white",
            stroke_width: "2",
            stroke_linecap: "round"
        }
    }
}

#[component]
pub fn HeaderBar(
    on_menu_click: EventHandler<()>,
    is_working: Signal<bool>,
    active_tab: Signal<i32>,
) -> Element {
    let mut show_work_status_dialog = use_signal(|| false);
    let mut show_confirm = use_signal(|| false);

    // 复用BottomBar弹窗内容
    let confirm_popup = || rsx! {
        div {
            class: "fixed inset-0 bg-black/40 z-40 flex items-center justify-center",
            div {
                class: "bg-white rounded-xl shadow-lg p-6 w-80 max-w-full flex flex-col items-center z-50",
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
    };

    rsx! {
        if *show_confirm.read() {
            {confirm_popup()}
        }
        // Header bar
        header {
            class: "sticky top-0 left-0 right-0 bg-black text-white z-20",
            div {
                class: "h-[52px] flex items-center justify-between px-4",
                div {
                    class: "flex items-center relative gap-2",
                    // Menu button
                    button { 
                        class: "flex items-center justify-center w-7 h-7 border border-slate-600 rounded-full bg-white/10",
                        onclick: move |_| on_menu_click.call(()),
                        // Hamburger menu icon
                        svg {
                            class: "w-[18px] h-[18px]",
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            path { d: "M4 6h16M4 12h16M4 18h16" }
                        }
                    }
                    // Work status button
                    div {
                        class: "flex items-center relative",
                        button {
                            class: "flex items-center border border-slate-600 rounded-full px-[5px] py-[3px] bg-white/10 relative z-10",
                            onclick: move |_| show_work_status_dialog.set(true),
                            // Status icon (red circle with dash or green checkmark)
                            svg {
                                class: "w-5 h-5 mr-1",
                                view_box: "0 0 24 24",
                                if *is_working.read() {
                                    WorkingIcon {}
                                } else {
                                    NotWorkingIcon {}
                                }
                            }
                            span { 
                                class: "text-[15px] leading-none text-white", 
                                if *is_working.read() { "已开工" } else { "已收工" }
                            }
                            // Dropdown arrow icon
                            svg {
                                class: "w-4 h-4 ml-1",
                                view_box: "0 0 24 24",
                                path { 
                                    d: "M12 16L6 10H18L12 16Z",
                                    fill: "white"
                                }
                            }
                        }
                        // 气泡确认框
                        if *show_work_status_dialog.read() {
                            // 蒙层，点击关闭气泡
                            div {
                                class: "fixed inset-0 z-0 bg-black/50 backdrop-blur-sm",
                                onclick: move |_| show_work_status_dialog.set(false),
                            }
                            div {
                                class: "w-full absolute top-[110%] inset-x-0 mx-auto bg-white rounded-lg shadow-lg px-4 py-2 flex items-center z-20 min-w-[80px] min-h-[40px]",
                                onclick: move |_| {
                                    if !*is_working.read() {
                                        // 当前是"收工"，点击"开工"时弹二次确认
                                        show_confirm.set(true);
                                    } else {
                                        // 当前是"开工"，点击"收工"直接切换
                                        is_working.set(false);
                                    }
                                    show_work_status_dialog.set(false);
                                },
                                // 小三角
                                div {
                                    class: "absolute -top-2 inset-x-0 mx-auto w-4 h-4 overflow-hidden",
                                    svg {
                                        class: "w-4 h-4",
                                        view_box: "0 0 16 8",
                                        path { d: "M0 8L8 0L16 8Z", fill: "white", stroke: "#e5e7eb", stroke_width: "1" }
                                    }
                                }
                                // 气泡内容
                                svg {
                                    class: "w-5 h-5 mr-2",
                                    view_box: "0 0 24 24",
                                    if *is_working.read() {
                                        NotWorkingIcon {}
                                    } else {
                                        WorkingIcon {}
                                    }
                                }
                                span {
                                    class: "text-base text-black",
                                    if *is_working.read() { "收工" } else { "开工" }
                                }
                            }
                        }
                    }
                }
                div { class: "flex items-center space-x-4",
                    button {
                        class: "flex items-center border border-slate-600 rounded-full bg-white/10 px-[5px] py-[3px] text-[14px]",
                        // 设计图风格路线图标：矩形地图框+定位点
                        svg {
                            class: "w-5 h-5",
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            // 地图矩形框
                            rect { x: "3", y: "5", width: "18", height: "14", rx: "3", stroke: "currentColor", fill: "none" }
                            // 地图定位点
                            path { d: "M12 10.5C12 9.119 13.119 8 14.5 8C15.881 8 17 9.119 17 10.5C17 12.5 14.5 15 14.5 15C14.5 15 12 12.5 12 10.5Z", stroke: "currentColor", fill: "none" }
                            circle { cx: "14.5", cy: "10.5", r: "1", fill: "currentColor", stroke: "none" }
                        }
                        span { class: "ml-1 leading-none", "路线" }
                    }
                    button {
                        class: "flex items-center justify-center w-7 h-7 border border-slate-600 rounded-full bg-white/10",
                        // Bell icon
                        svg {
                            class: "w-[18px] h-[18px]",
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
            // 订单Tab
            OrderTabs {
                active: *active_tab.read(),
                on_change: move |index| active_tab.set(index)
            }
        }
    }
}

#[component]
pub fn OrderTabs(active: i32, on_change: EventHandler<i32>) -> Element {
    let tabs = vec![
        ("新任务", "border-white text-white", "text-gray-500"),
        ("待取货", "border-white text-white", "text-gray-500"),
        ("配送中", "border-white text-white", "text-gray-500"),
    ];

    rsx! {
        div { class: "flex bg-black",
            {tabs.into_iter().enumerate().map(|(index, (text, active_class, inactive_class))| {
                let is_active = active == index as i32;
                let class_name = if is_active {
                    format!("text-center text-sm px-4 py-2 cursor-pointer relative after:content-[''] after:absolute after:bottom-0 after:inset-x-0 after:mx-auto after:w-[30px] after:h-[4px] after:bg-white after:rounded-full {}", active_class)
                } else {
                    format!("text-center text-sm px-4 py-2 cursor-pointer relative {}", inactive_class)
                };
                rsx! {
                    div {
                        class: class_name,
                        onclick: move |_| on_change.call(index as i32),
                        span { "{text}" }
                        // 倒三角svg
                        svg {
                            class: if is_active { "inline w-4 h-4 align-middle text-white" } else { "inline w-4 h-4 align-middle text-gray-500" },
                            view_box: "0 0 24 24",
                            path { 
                                d: "M12 16L6 10H18L12 16Z",
                                fill: "currentColor"
                            }
                        }
                    }
                }
            })}
        }
    }
} 