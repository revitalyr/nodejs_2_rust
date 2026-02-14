use leptos::*;
use serde::{Deserialize, Serialize};

// --- Data Models ---

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ContractInfo {
    pub name: String,
    pub symbol: String,
    pub address: String,
    pub total_supply: String,
    pub decimals: u8,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct NFTContractInfo {
    pub address: String,
    pub name: String,
    pub symbol: String,
    pub total_minted: u64,
    pub max_supply: u64,
}

// --- Mock APIs (wallet/node interaction simulation) ---

async fn get_contract_info(address: String) -> Result<ContractInfo, String> {
    if address.is_empty() { return Err("Invalid address".into()); }
    Ok(ContractInfo {
        name: "Mock Token".to_string(),
        symbol: "MTK".to_string(),
        address,
        total_supply: "1,000,000".to_string(),
        decimals: 18,
    })
}

async fn mint_tokens(amount: String) -> Result<String, String> {
    Ok(format!("Successfully minted {} tokens", amount))
}

async fn get_nft_contracts(_user_addr: String) -> Result<Vec<NFTContractInfo>, String> {
    Ok(vec![
        NFTContractInfo {
            address: "0x123...".to_string(),
            name: "Alpha Collection".to_string(),
            symbol: "ALP".to_string(),
            total_minted: 42,
            max_supply: 100,
        },
    ])
}

// --- Component: Contract Interface (ERC-20) ---

#[component]
pub fn ContractInterface(address: String) -> impl IntoView {
    // Use Resource for automatic data loading when address changes
    let addr = store_value(address);
    let contract_res = create_resource(
        move || addr.get_value(),
        |a| async move { get_contract_info(a).await }
    );

    // Form states
    let (mint_amount, set_mint_amount) = create_signal(String::new());
    let (status, set_status) = create_signal(Option::<String>::None);
    let (loading, set_loading) = create_signal(false);

    let handle_mint = move |_| {
        set_loading.set(true);
        let amount = mint_amount.get();
        spawn_local(async move {
            let res = mint_tokens(amount).await;
            set_status.set(Some(res.unwrap_or_else(|e| e)));
            set_loading.set(false);
        });
    };

    view! {
        <div class="contract-card p-4 border rounded-lg bg-gray-50 dark:bg-gray-800">
            <Transition fallback=move || view! { <p>"Loading contract..."</p> }>
                {move || contract_res.get().map(|res| match res {
                    Ok(info) => view! {
                        <div class="space-y-4">
                            <header>
                                <h4 class="text-xl font-bold">{info.name} " (" {info.symbol} ")"</h4>
                                <code class="text-sm text-blue-500">{info.address}</code>
                            </header>

                            <div class="action-box p-3 border rounded">
                                <label class="block text-sm font-medium mb-1">"Mint New Tokens"</label>
                                <div class="flex gap-2">
                                    <input type="number" 
                                        class="border p-1 flex-grow rounded"
                                        on:input=move |ev| set_mint_amount.set(event_target_value(&ev))
                                        prop:value=mint_amount
                                    />
                                    <button 
                                        class="bg-blue-600 text-white px-4 py-1 rounded disabled:opacity-50"
                                        on:click=handle_mint
                                        disabled=loading
                                    >
                                        {move || if loading.get() { "Processing..." } else { "Mint" }}
                                    </button>
                                </div>
                            </div>
                        </div>
                    }.into_view(),
                    Err(e) => view! { <p class="text-red-500">"Error: " {e}</p> }.into_view()
                })}
            </Transition>

            {move || status.get().map(|s| view! { 
                <div class="mt-4 p-2 bg-green-100 text-green-800 rounded"> {s} </div> 
            })}
        </div>
    }
}

// --- Component: NFT Minter ---

#[component]
pub fn NFTMinter() -> impl IntoView {
    // IMPORTANT: Usually ReadSignal for address is taken from context
    let user_address = use_context::<ReadSignal<String>>()
        .expect("User address context not found");

    let nft_resource = create_resource(
        move || user_address.get(),
        |addr| async move { get_nft_contracts(addr).await }
    );

    let (selected_addr, set_selected_addr) = create_signal(String::new());

    view! {
        <div class="nft-minter p-6 space-y-6">
            <h3 class="text-2xl font-bold">"🎨 NFT Minter"</h3>

            <Transition fallback=move || view! { <p>"Scanning for contracts..."</p> }>
                {move || nft_resource.get().map(|res| match res {
                    Ok(contracts) => view! {
                        <select 
                            class="w-full p-2 border rounded"
                            on:change=move |ev| set_selected_addr.set(event_target_value(&ev))
                        >
                            <option value="">"Choose a contract"</option>
                            <For
                                each=move || contracts.clone()
                                key=|c| c.address.clone()
                                children=move |c| view! {
                                    <option value=c.address.clone()>{c.name} " (" {c.symbol} ")"</option>
                                }
                            />
                        </select>
                    }.into_view(),
                    Err(_) => view! { <p>"Failed to load contracts"</p> }.into_view()
                })}
            </Transition>

            <div class="mint-controls">
                <button 
                    class="w-full py-3 bg-purple-600 text-white font-bold rounded shadow-lg disabled:bg-gray-400"
                    disabled=move || selected_addr.get().is_empty()
                    on:click=|_| { /* Mint logic */ }
                >
                    "Mint NFT from selected collection"
                </button>
            </div>
        </div>
    }
}