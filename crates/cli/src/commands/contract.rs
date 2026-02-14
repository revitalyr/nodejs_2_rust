//! Contract interaction command implementation

use ethereum_boilerplate_utils::{
    print_banner, print_success, print_error, create_progress_bar,
    Config, Result, validate_address, validate_amount, UtilsError
};
use alloy::primitives::{Address, U256};
use alloy::providers::{Provider, ProviderBuilder, RootProvider};
use alloy::signers::Signer;
use alloy::signers::local::PrivateKeySigner;
use alloy::transports::http::{Client, Http};
use std::sync::Arc;
use tokio::process::Command;
use dialoguer::Input;
use ethereum_boilerplate_shared::{ContractAction};



/// Взаимодействие с развернутым контрактом
#[allow(dead_code)]
pub async fn interact_with_contract(
    config: Config,
    address: String,
    action: String,
    amount: Option<String>,
    to: Option<String>,
    token_id: Option<String>,
) -> Result<()> {
    print_banner();

    let contract_address = validate_address(&address)?;
    println!("Contract address: {:#x}", contract_address);
    println!("Network: {}", config.network.name);

    match ContractAction::parse(&action) {
        Some(ContractAction::Balance) => get_balance(config, contract_address).await,
        Some(ContractAction::Mint) => mint_tokens(config, contract_address, amount).await,
        Some(ContractAction::Transfer) => transfer_tokens(config, contract_address, amount, to).await,
        Some(ContractAction::Approve) => approve_tokens(config, contract_address, amount, to).await,
        Some(ContractAction::MintNft) => mint_nft(config, contract_address, token_id).await,
        Some(ContractAction::TransferNft) => transfer_nft_via_cli(config, contract_address, token_id, to).await,
        None => {
            let available_actions = ContractAction::all()
                .iter()
                .map(|a: &ContractAction| a.to_string())
                .collect::<Vec<_>>()
                .join(", ");
            print_error(&format!("Invalid action. Use: {}", available_actions));
            Ok(())
        }
    }
}

// --- Обработчики действий (Вызов CLI бинарника) ---

async fn get_balance(config: Config, contract_address: Address) -> Result<()> {
    run_interact_command(
        &config,
        &["--address", &format!("{:#x}", contract_address), "--action", &ContractAction::Balance.to_string()],
        "Retrieving balance...",
        "Balance retrieved successfully!",
    ).await
}

async fn mint_tokens(config: Config, contract_address: Address, amount: Option<String>) -> Result<()> {
    let amount_str = prompt_if_none(amount, "Enter amount to mint", Some(validate_amount))?;
    run_interact_command(
        &config,
        &[
            "--address", &format!("{:#x}", contract_address),
            "--action", &ContractAction::Mint.to_string(),
            "--amount", &amount_str,
        ],
        "Minting tokens...",
        &format!("Successfully minted {} tokens!", amount_str),
    ).await
}

async fn transfer_tokens(config: Config, contract_address: Address, amount: Option<String>, to: Option<String>) -> Result<()> {
    let amount_str = prompt_if_none(amount, "Enter amount to transfer", Some(validate_amount))?;
    let to_str = prompt_if_none(to, "Enter recipient address", None)?;
    validate_address(&to_str)?;

    run_interact_command(
        &config,
        &[
            "--address", &format!("{:#x}", contract_address),
            "--action", &ContractAction::Transfer.to_string(),
            "--amount", &amount_str,
            "--to", &to_str,
        ],
        "Executing transfer...",
        &format!("Successfully transferred {} to {}", amount_str, to_str),
    ).await
}

async fn approve_tokens(config: Config, contract_address: Address, amount: Option<String>, to: Option<String>) -> Result<()> {
    let amount_str = prompt_if_none(amount, "Enter amount to approve", Some(validate_amount))?;
    let to_str = prompt_if_none(to, "Enter spender address", None)?;
    validate_address(&to_str)?;

    run_interact_command(
        &config,
        &[
            "--address", &format!("{:#x}", contract_address),
            "--action", &ContractAction::Approve.to_string(),
            "--amount", &amount_str,
            "--to", &to_str,
        ],
        "Approving tokens...",
        &format!("Successfully approved {} for {}", amount_str, to_str),
    ).await
}

async fn mint_nft(config: Config, contract_address: Address, token_id: Option<String>) -> Result<()> {
    let id_str = prompt_if_none(token_id, "Enter Token ID", None)?;
    run_interact_command(
        &config,
        &[
            "--address", &format!("{:#x}", contract_address),
            "--action", &ContractAction::MintNft.to_string(),
            "--token-id", &id_str,
        ],
        "Minting NFT...",
        &format!("Successfully minted NFT #{}", id_str),
    ).await
}

/// Вызов через CLI для консистентности (refactored)
async fn transfer_nft_via_cli(config: Config, contract_address: Address, token_id: Option<String>, to: Option<String>) -> Result<()> {
    let id_str = prompt_if_none(token_id, "Enter Token ID", None)?;
    let to_str = prompt_if_none(to, "Enter recipient address", None)?;
    
    run_interact_command(
        &config,
        &[
            "--address", &format!("{:#x}", contract_address),
            "--action", &ContractAction::TransferNft.to_string(),
            "--token-id", &id_str,
            "--to", &to_str,
        ],
        "Transferring NFT...",
        &format!("Successfully transferred NFT #{} to {}", id_str, to_str),
    ).await
}

// --- Вспомогательные функции ---



/// Универсальный запуск подпроцесса взаимодействия с контрактом
#[allow(dead_code)]
async fn run_interact_command(
    config: &Config,
    extra_args: &[&str],
    loading_msg: &str,
    success_msg: &str,
) -> Result<()> {
    let pb = create_progress_bar(2);
    pb.set_message(loading_msg.to_string());
    pb.inc(1);

    let mut args = vec!["run", "--bin", "interact", "--"];
    args.extend_from_slice(extra_args);
    args.push("--rpc-url");
    args.push(&config.ethereum_rpc_url);

    let status = Command::new("cargo")
        .args(&args)
        .current_dir("crates/smart-contracts")
        .status()
        .await
        .map_err(|e| UtilsError::internal(format!("Failed to execute interact bin: {}", e)))?;

    pb.finish_and_clear();

    if status.success() {
        print_success(success_msg);
    } else {
        print_error("Failed to execute contract interaction");
    }

    Ok(())
}

/// Хелпер для получения ввода, если аргумент не был передан через CLI
pub fn prompt_if_none(
    value: Option<String>,
    prompt: &str,
    validator: Option<fn(&str) -> Result<()>>
) -> Result<String> {
    match value {
        Some(val) => {
            if let Some(v) = validator { v(&val)?; }
            Ok(val)
        }
        None => {
            let input: String = Input::new()
                .with_prompt(prompt)
                .interact()
                .map_err(|e| UtilsError::interactive_error(e.to_string()))?;
            
            if let Some(v) = validator { v(&input)?; }
            Ok(input)
        }
    }
}