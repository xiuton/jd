use dioxus::prelude::*;

pub fn Wallet() -> Element {
    rsx! {
        div { class: "wallet-container",
            // 顶部导航栏
            nav { class: "nav-bar",
                div { class: "nav-title", "我的钱包" }
            }

            // 余额卡片
            div { class: "balance-card",
                div { class: "balance-title", "账户余额" }
                div { class: "balance-amount", "¥ 1,234.56" }
                div { class: "balance-actions",
                    button { class: "balance-btn", "提现" }
                    button { class: "balance-btn", "明细" }
                }
            }

            // 收入统计
            div { class: "income-stats",
                div { class: "stat-item",
                    div { class: "stat-value", "¥ 328" }
                    div { class: "stat-label", "今日收入" }
                }
                div { class: "stat-item",
                    div { class: "stat-value", "¥ 2,156" }
                    div { class: "stat-label", "本周收入" }
                }
                div { class: "stat-item",
                    div { class: "stat-value", "¥ 8,432" }
                    div { class: "stat-label", "本月收入" }
                }
            }

            // 交易记录
            div { class: "transaction-list",
                h3 { "交易记录" }
                TransactionItem {
                    type_: "收入".to_string(),
                    amount: "+¥15.00".to_string(),
                    time: "2024-03-15 14:30".to_string(),
                    description: "配送订单".to_string(),
                }
                TransactionItem {
                    type_: "提现".to_string(),
                    amount: "-¥100.00".to_string(),
                    time: "2024-03-14 16:45".to_string(),
                    description: "提现到银行卡".to_string(),
                }
            }
        }
    }
}

#[component]
fn TransactionItem(type_: String, amount: String, time: String, description: String) -> Element {
    let class = if amount.starts_with('+') { "transaction-amount income" } else { "transaction-amount expense" };
    rsx! {
        div { class: "transaction-item",
            div { class: "transaction-info",
                div { class: "transaction-type", "{type_}" }
                div { class: "transaction-time", "{time}" }
                div { class: "transaction-desc", "{description}" }
            }
            div { class: class,
                "{amount}"
            }
        }
    }
} 