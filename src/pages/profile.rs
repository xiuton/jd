use dioxus::prelude::*;

pub fn Profile() -> Element {
    rsx! {
        div { class: "profile-container",
            // 顶部导航栏
            nav { class: "nav-bar",
                div { class: "nav-title", "个人中心" }
            }

            // 用户信息卡片
            div { class: "user-card",
                div { class: "user-avatar",
                    img { src: "https://via.placeholder.com/80", alt: "用户头像" }
                }
                div { class: "user-info",
                    h2 { "张三" }
                    p { "骑手ID: 12345678" }
                }
                button { class: "edit-profile-btn", "编辑资料" }
            }

            // 统计信息
            div { class: "stats-grid",
                StatItem { label: "总订单", value: "328" }
                StatItem { label: "好评率", value: "98%" }
                StatItem { label: "在线时长", value: "256h" }
                StatItem { label: "等级", value: "Lv.5" }
            }

            // 功能列表
            div { class: "feature-list",
                FeatureItem { icon: "📋", title: "工作统计", description: "查看详细工作数据" }
                FeatureItem { icon: "🏆", title: "成就中心", description: "查看获得的成就" }
                FeatureItem { icon: "📱", title: "设备管理", description: "管理登录设备" }
                FeatureItem { icon: "⚙️", title: "设置", description: "应用设置" }
            }

            // 退出登录按钮
            button { class: "logout-btn", "退出登录" }
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