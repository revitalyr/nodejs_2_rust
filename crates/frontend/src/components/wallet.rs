use leptos::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use web_sys::{console, window};
use gloo_net::http::Request;
use gloo_storage::{LocalStorage, Storage};
use js_sys::Promise;
use wasm_bindgen_futures::spawn_local;
use std::rc::Rc;
use ethereum_boilerplate_shared::{SUPPORTED_NETWORKS, NetworkInfo};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WalletInfo {
    pub address: String,
    pub chain_id: u64,
    pub balance: String,
}

#[component]
pub fn WalletConnector() -> impl IntoView {
    let (wallet_connected, set_wallet_connected) = use_context::<ReadSignal<bool>>()
        .expect("wallet_connected context provided");
    let (current_address, set_current_address) = use_context::<WriteSignal<String>>()
        .expect("current_address context provided");

    let connect_wallet = move |_| {
        spawn_local(async move {
            match connect_to_wallet().await {
                Ok(address) => {
                    set_wallet_connected.set(true);
                    set_current_address.set(address.clone());
                    console::log_1(&format!("Wallet connected: {}", address));
                }
                Err(e) => {
                    console::error_1(&format!("Failed to connect wallet: {}", e));
                }
            }
        });
    };

    let disconnect_wallet = move |_| {
        let storage = LocalStorage::new();
        storage.delete("wallet_address");
        storage.delete("wallet_connected");
        set_wallet_connected.set(false);
        set_current_address.set(String::new());
        console::log_1("Wallet disconnected");
    };

    view! {
        <div class="wallet-connector">
            {move || {
                if wallet_connected.get() {
                    view! {
                        <div class="wallet-connected">
                            <div class="wallet-info">
                                <span class="wallet-address">
                                    {current_address.get()}
                                </span>
                                <span class="connection-indicator connected">"🟢"</span>
                            </div>
                            <button 
                                class="disconnect-button"
                                on:click=disconnect_wallet
                            >
                                "Disconnect"
                            </button>
                        </div>
                    }
                } else {
                    view! {
                        <button 
                            class="connect-button"
                            on:click=connect_wallet
                        >
                            "🔗 Connect Wallet"
                        </button>
                    }
                }
            }}
        </div>
    }
}

#[component]
pub fn WalletInfo() -> impl IntoView {
    let (current_address, _) = use_context::<WriteSignal<String>>()
        .expect("current_address context provided");
    let (wallet_info, set_wallet_info) = create_signal(Option::<WalletInfo>::None);

    let address = current_address.get();
    
    let load_wallet_info = move || {
        if !address.is_empty() {
            spawn_local(async move {
                match get_wallet_info(&address).await {
                    Ok(info) => {
                        set_wallet_info.set(Some(info));
                    }
                    Err(e) => {
                        console::error_1(&format!("Failed to get wallet info: {}", e));
                    }
                }
            });
        }
    };

    // Load wallet info when address changes
    leptos::create_effect(move |_| {
        load_wallet_info();
    });

    view! {
        <div class="wallet-info-card">
            <h2>"Wallet Information"</h2>
            {move || {
                if let Some(info) = wallet_info.get() {
                    view! {
                        <div class="info-grid">
                            <div class="info-item">
                                <label>"Address:"</label>
                                <span class="address">{info.address}</span>
                            </div>
                            <div class="info-item">
                                <label>"Chain ID:"</label>
                                <span>{info.chain_id}</span>
                            </div>
                            <div class="info-item">
                                <label>"ETH Balance:"</label>
                                <span>{info.balance}</span>
                            </div>
                        </div>
                    }
                } else {
                    view! {
                        <div class="connect-prompt">
                            <p>"Please connect your wallet to view information"</p>
                        </div>
                    }
                }
            }}
        </div>
    }
}

// Wallet connection functions
async fn connect_to_wallet() -> Result<String, String> {
    // In a real implementation, this would connect to MetaMask or other wallets
    // For now, we'll simulate with a mock implementation
    
    let window = window().ok_or("Failed to get window")?;
    let storage = LocalStorage::new();
    
    // Simulate wallet connection
    let mock_address = "0x742d35Cc6634C0532925a3b8D4C9db96C4b4d8b";
    
    storage.set("wallet_address", mock_address);
    storage.set("wallet_connected", "true");
    
    Ok(mock_address.to_string())
}

async fn get_wallet_info(address: &str) -> Result<WalletInfo, String> {
    // Simulate API call to get wallet info
    Ok(WalletInfo {
        address: address.to_string(),
        chain_id: 1,
        balance: "1.2345".to_string(),
    })
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ethereum)]
    type Ethereum;

    #[wasm_bindgen(method, js_name = request)]
    async fn request(args: JsValue) -> JsValue;

    #[wasm_bindgen(method, getter)]
    fn selectedAddress() -> Option<String>;
    
    #[wasm_bindgen(method, getter)]
    fn chainId() -> Option<String>;
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = window)]
    type Window;

    #[wasm_bindgen(method, getter)]
    fn ethereum() -> Option<Ethereum>;
}
