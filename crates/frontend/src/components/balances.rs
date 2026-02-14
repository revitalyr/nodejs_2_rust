//! Balances component with fixed Leptos 0.8.14 syntax

use leptos::*;
use crate::models::*;

#[component]
pub fn BalancesComponent() -> impl IntoView {
    let (balances, set_balances) = create_signal(Vec::<TokenBalance>::new());
    let (nfts, set_nfts) = create_signal(Vec::<NFTBalance>::new());
    let (loading, set_loading) = create_signal(false);

    // Mock data
    let mock_balances = vec![
        TokenBalance {
            symbol: "ETH".to_string(),
            name: "Ethereum".to_string(),
            balance: "2.345".to_string(),
            usd_value: "$4,690.00".to_string(),
        },
        TokenBalance {
            symbol: "USDC".to_string(),
            name: "USD Coin".to_string(),
            balance: "1,234.56".to_string(),
            usd_value: "$1,234.56".to_string(),
        },
    ];

    let mock_nfts = vec![
        NFTBalance {
            token_address: "0x1234567890123456789012345678901234".to_string(),
            token_id: "1".to_string(),
            name: Some("Cool NFT #1".to_string()),
            symbol: Some("NFT".to_string()),
            image_url: Some("https://via.placeholder.com/150".to_string()),
        },
    ];

    // Load mock data
    set_balances.set(mock_balances);
    set_nfts.set(mock_nfts);
    set_loading.set(false);

    view! {
        <div class="balances-container">
            // Token Balances Section
            <div class="token-balance-card">
                <h3>"💰 Token Balances"</h3>
                {move || {
                    if loading.get() {
                        view! {
                            <div class="loading-container">
                                <div class="spinner"></div>
                                <p>"Loading balances..."</p>
                            </div>
                        }
                    } else {
                        let balance_list = balances.get();
                        if balance_list.is_empty() {
                            view! {
                                <div class="empty-state">
                                    <p>"No token balances found"</p>
                                </div>
                            }
                        } else {
                            view! {
                                <div class="token-list">
                                    {balance_list.into_iter().map(|token| {
                                        view! {
                                            <div class="token-item">
                                                <div class="token-info">
                                                    <span class="token-symbol">{token.symbol}</span>
                                                    <span class="token-name">{token.name}</span>
                                                </div>
                                                <div class="token-values">
                                                    <span class="token-balance">{token.balance}</span>
                                                    <span class="token-usd-value">{token.usd_value}</span>
                                                </div>
                                            </div>
                                        }
                                    }).collect_view()}
                                </div>
                            }
                        }
                    }
                }}
            </div>

            // NFT Balances Section
            <div class="nft-balance-card">
                <h3>"🎨 NFT Collection"</h3>
                {move || {
                    if loading.get() {
                        view! {
                            <div class="loading-container">
                                <div class="spinner"></div>
                                <p>"Loading NFTs..."</p>
                            </div>
                        }
                    } else {
                        let nft_list = nfts.get();
                        if nft_list.is_empty() {
                            view! {
                                <div class="empty-state">
                                    <p>"No NFTs found in this wallet"</p>
                                </div>
                            }
                        } else {
                            view! {
                                <div class="nft-grid">
                                    {nft_list.into_iter().map(|nft| {
                                        view! {
                                            <div class="nft-card" key={format!("{}-{}", nft.token_address, nft.token_id)}>
                                                <img 
                                                    src={nft.image_url.clone()} 
                                                    alt={nft.name.clone()} 
                                                    class="nft-image"
                                                />
                                                <div class="nft-placeholder">
                                                    <div class="nft-info">
                                                        <h4>{nft.name.clone().unwrap_or_else(|| "Unknown NFT".to_string())}</h4>
                                                        <p class="nft-symbol">{nft.symbol.clone().unwrap_or_else(|| "NFT".to_string())}</p>
                                                        <p class="nft-id">"ID: {nft.token_id}"</p>
                                                    </div>
                                                </div>
                                            </div>
                                        }
                                    }).collect_view()}
                                </div>
                            }
                        }
                    }
                }}
            </div>
        </div>
    }
}
