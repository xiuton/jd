use dioxus::prelude::*;

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
pub fn HeaderBar(on_menu_click: EventHandler<()>) -> Element {
    let mut show_work_status_dialog = use_signal(|| false);
    let mut is_working = use_signal(|| false);

    rsx! {
        // Header bar
        header { 
            class: "fixed top-0 left-0 right-0 bg-black text-white h-[52px] flex items-center justify-between px-4 z-20",
            div {
                class: "flex items-center",
                button { 
                    class: "flex items-center",
                    onclick: move |_| on_menu_click.call(()),
                    // Hamburger menu icon
                    svg {
                        class: "w-6 h-6",
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
                button {
                // Menu button
                    class: "flex items-center ml-2 border border-slate-600 rounded-full px-[5px] py-[3px] bg-black bg-opacity-50",
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
            }
            div { class: "flex items-center space-x-4",
                button {
                    class: "flex items-center text-gray-400",
                    // Route icon
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
                    // Bell icon
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

        // Work status dialog
        if *show_work_status_dialog.read() {
            div { 
                class: "fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-30",
                onclick: move |_| show_work_status_dialog.set(false),
                // Dialog content
                div { 
                    class: "bg-white rounded-lg p-6 w-72 flex flex-col items-center",
                    onclick: move |e| e.stop_propagation(),
                    // Status icon
                    div { 
                        class: "mb-4",
                        svg {
                            class: "w-16 h-16",
                            view_box: "0 0 24 24",
                            if *is_working.read() {
                                WorkingIcon {}
                            } else {
                                NotWorkingIcon {}
                            }
                        }
                    }
                    // Status text
                    h3 { 
                        class: "text-xl font-bold mb-6",
                        if *is_working.read() { "确认收工?" } else { "开工" }
                    }
                    // Action button
                    button { 
                        class: "w-full py-3 rounded-lg text-white font-bold",
                        class: if *is_working.read() { "bg-red-500" } else { "bg-green-500" },
                        onclick: move |_| {
                            let current_status = *is_working.read();
                            is_working.set(!current_status);
                            show_work_status_dialog.set(false);
                        },
                        if *is_working.read() { "收工" } else { "开工" }
                    }
                }
            }
        }
    }
} 