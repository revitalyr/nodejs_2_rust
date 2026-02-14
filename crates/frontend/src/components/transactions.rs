use leptos::*;
use serde::{Deserialize, Serialize};
use std::rc::Rc;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Transaction {
    pub hash: String,
    pub from: String,
    pub to: Option<String>,
    pub value: String,
    pub gas_used: String,
    pub gas_price: Option<String>,
    pub block_number: u64,
    pub timestamp: Option<String>,
    pub status: TransactionStatus,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum TransactionStatus {
    Success,
    Pending,
    Failed,
}

#[component]
pub fn TransactionHistory() -> impl IntoView {
    let (current_address, _) = use_context::<WriteSignal<String>>()
        .expect("current_address context provided");
    let (transactions, set_transactions) = create_signal(Vec::<Transaction>::new);
    let (loading, set_loading) = create_signal(false);
    let (page, set_page) = create_signal(1u32);

    let address = current_address.get();
    
    let load_transactions = move || {
        if !address.is_empty() {
            set_loading.set(true);
            let current_page = page.get();
            spawn_local(async move {
                match get_transactions(&address, current_page).await {
                    Ok(tx_list) => {
                        set_transactions.set(tx_list);
                        set_loading.set(false);
                    }
                    Err(e) => {
                        web_sys::console::error_1(&format!("Failed to get transactions: {}", e));
                        set_loading.set(false);
                    }
                }
            });
        }
    };

    leptos::create_effect(move |_| {
        load_transactions();
    });

    let load_next_page = move |_| {
        set_page.update(|p| p + 1);
    };

    let load_prev_page = move |_| {
        set_page.update(|p| if p > 1 { p - 1 } else { p });
    };

    view! {
        <div class="transaction-history-card">
            <h3>"📜 Transaction History"</h3>
            
            {move || {
                if loading.get() {
                    view! {
                        <div class="loading-container">
                            <div class="spinner"></div>
                            <p>"Loading transactions..."</p>
                        </div>
                    }
                } else {
                    let tx_list = transactions.get();
                    if tx_list.is_empty() {
                        view! {
                            <div class="empty-state">
                                <p>"No transactions found for this wallet"</p>
                            </div>
                        }
                    } else {
                        view! {
                            <div class="transaction-controls">
                                <button 
                                    class="pagination-button"
                                    on:click=load_prev_page
                                    disabled={page.get() == 1}
                                >
                                    "← Previous"
                                </button>
                                <span class="page-info">
                                    "Page "{page.get()}
                                </span>
                                <button 
                                    class="pagination-button"
                                    on:click=load_next_page
                                >
                                    "Next →"
                                </button>
                            </div>
                            
                            <div class="transaction-list">
                                {tx_list.iter().map(|tx| {
                                    view! {
                                        <TransactionItem transaction={tx.clone()} key={tx.hash.clone()}/>
                                    }
                                }).collect_view()}
                            </div>
                        }
                    }
                }
            }}
        </div>
    }
}

#[component]
fn TransactionItem(transaction: Transaction) -> impl IntoView {
    let status_class = move || {
        match transaction.status {
            TransactionStatus::Success => "success",
            TransactionStatus::Pending => "pending",
            TransactionStatus::Failed => "failed",
        }
    };

    let status_icon = move || {
        match transaction.status {
            TransactionStatus::Success => "✅",
            TransactionStatus::Pending => "⏳",
            TransactionStatus::Failed => "❌",
        }
    };

    view! {
        <div class="transaction-item" class={status_class}>
            <div class="transaction-header">
                <div class="transaction-hash">
                    <span class="hash-label">"Hash:"</span>
                    <code>{transaction.hash.clone()}</code>
                </div>
                <div class="transaction-status">
                    <span class="status-icon">{status_icon}</span>
                    <span class="status-text">
                        {format!("{:?}", transaction.status)}
                    </span>
                </div>
            </div>
            
            <div class="transaction-details">
                <div class="detail-row">
                    <span>"From:"</span>
                    <code class="address">{transaction.from.clone()}</code>
                </div>
                
                {move || {
                    if let Some(to) = &transaction.to {
                        view! {
                            <div class="detail-row">
                                <span>"To:"</span>
                                <code class="address">{to.clone()}</code>
                            </div>
                        }
                    } else {
                        view! {
                            <div class="detail-row">
                                <span>"To:"</span>
                                <span>"Contract Creation"</span>
                            </div>
                        }
                    }
                }}
                
                <div class="detail-row">
                    <span>"Value:"</span>
                    <strong class="value">{transaction.value.clone()}</strong>
                    <span class="eth-label">"ETH"</span>
                </div>
                
                <div class="detail-row">
                    <span>"Gas Used:"</span>
                    <span>{transaction.gas_used.clone()}</span>
                </div>
                
                {move || {
                    if let Some(gas_price) = &transaction.gas_price {
                        view! {
                            <div class="detail-row">
                                <span>"Gas Price:"</span>
                                <span>{gas_price.clone()}</span>
                            </div>
                        }
                    } else {
                        view! {
                            <div class="detail-row">
                                <span>"Gas Price:"</span>
                                <span>"N/A"</span>
                            </div>
                        }
                    }
                }}
                
                <div class="detail-row">
                    <span>"Block:"</span>
                    <span>{transaction.block_number}</span>
                </div>
                
                {move || {
                    if let Some(timestamp) = &transaction.timestamp {
                        view! {
                            <div class="detail-row">
                                <span>"Time:"</span>
                                <span>{timestamp.clone()}</span>
                            </div>
                        }
                    } else {
                        view! {
                            <div class="detail-row">
                                <span>"Time:"</span>
                                <span>"N/A"</span>
                            </div>
                        }
                    }
                }}
            </div>
            
            <div class="transaction-actions">
                <a 
                    href={format!("https://etherscan.io/tx/{}", transaction.hash)}
                    target="_blank"
                    class="etherscan-link"
                >
                    "View on Etherscan →"
                </a>
            </div>
        </div>
    }
}

#[component]
pub fn RecentTransactions() -> impl IntoView {
    let (current_address, _) = use_context::<WriteSignal<String>>()
        .expect("current_address context provided");
    let (recent_txs, set_recent_txs) = create_signal(Vec::<Transaction>::new);

    let address = current_address.get();
    
    let load_recent = move || {
        if !address.is_empty() {
            spawn_local(async move {
                match get_recent_transactions(&address).await {
                    Ok(tx_list) => {
                        set_recent_txs.set(tx_list);
                    }
                    Err(e) => {
                        web_sys::console::error_1(&format!("Failed to get recent transactions: {}", e));
                    }
                }
            });
        }
    };

    leptos::create_effect(move |_| {
        load_recent();
    });

    view! {
        <div class="recent-transactions-card">
            <h3>"🔄 Recent Activity"</h3>
            {move || {
                let tx_list = recent_txs.get();
                if tx_list.is_empty() {
                    view! {
                        <div class="empty-state">
                            <p>"No recent activity"</p>
                        </div>
                    }
                } else {
                    view! {
                        <div class="recent-list">
                            {tx_list.iter().take(5).map(|tx| {
                                view! {
                                    <div class="recent-item" key={tx.hash.clone()}>
                                        <div class="recent-status">
                                            {match tx.status {
                                                TransactionStatus::Success => "✅",
                                                TransactionStatus::Pending => "⏳",
                                                TransactionStatus::Failed => "❌",
                                            }}
                                        </div>
                                        <div class="recent-details">
                                            <div class="recent-value">
                                                <strong>{tx.value.clone()}</strong>
                                                <span>" ETH"</span>
                                            </div>
                                            <div class="recent-time">
                                                {tx.timestamp.clone().unwrap_or_else(|| "Unknown".to_string())}
                                            </div>
                                        </div>
                                    </div>
                                }
                            }).collect_view()}
                        </div>
                        
                        <a href="/transactions" class="view-all-link">
                            "View All Transactions →"
                        </a>
                    }
                }
            }}
        </div>
    }
}

// API functions (mock implementations)
async fn get_transactions(address: &str, page: u32) -> Result<Vec<Transaction>, String> {
    // Mock implementation - in real app, this would call backend API
    let mock_transactions = vec![
        Transaction {
            hash: "0x1234567890abcdef1234567890abcdef1234567890abcdef".to_string(),
            from: address.to_string(),
            to: Some("0x9876543210fedcba9876543210fedcba9876543210fedcba".to_string()),
            value: "0.123456".to_string(),
            gas_used: "21000".to_string(),
            gas_price: Some("20".to_string()),
            block_number: 18500000,
            timestamp: Some("2024-01-15 10:30:00".to_string()),
            status: TransactionStatus::Success,
        },
        Transaction {
            hash: "0xabcdef1234567890abcdef1234567890abcdef1234567890".to_string(),
            from: "0x9876543210fedcba9876543210fedcba9876543210fedcba".to_string(),
            to: Some(address.to_string()),
            value: "0.054321".to_string(),
            gas_used: "21000".to_string(),
            gas_price: Some("25".to_string()),
            block_number: 18499950,
            timestamp: Some("2024-01-15 09:45:00".to_string()),
            status: TransactionStatus::Success,
        },
    ];

    let start_index = ((page - 1) * 10) as usize;
    let end_index = (start_index + 10).min(mock_transactions.len());
    
    Ok(mock_transactions[start_index..end_index].to_vec())
}

async fn get_recent_transactions(address: &str) -> Result<Vec<Transaction>, String> {
    // Mock implementation - return last 5 transactions
    get_transactions(address, 1).await
}
