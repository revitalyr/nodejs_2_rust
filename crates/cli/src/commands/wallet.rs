//! Wallet command implementation

use crate::utils::{print_banner, print_success, create_progress_bar};
use ethereum_boilerplate_utils::{Config, Result, validate_private_key, format_address_display, format_wei};
use alloy::providers::Provider;
use alloy::signers::local::PrivateKeySigner;
use dialoguer::Input;
use colored::Colorize;
use crate::WalletCommands;

async fn show_wallet_info(
    config: Config,
    private_key: Option<String>,
    balance_flag: bool, // Renamed to avoid conflict with variable balance
    nonce_flag: bool,
) -> Result<()> {
    print_banner();

    // 1. Get and validate private key
    let pk_str = match private_key {
        Some(key) => {
            validate_private_key(&key)?;
            key
        }
        None => Input::<String>::new()
            .with_prompt("Enter private key (0x...)")
            .interact()
            .map_err(|e| ethereum_boilerplate_utils::UtilsError::Internal(e.to_string()))?
    };

    let wallet = pk_str.parse::<PrivateKeySigner>()?;
    let address = wallet.address();

    println!("\n{}", "Wallet Information:".bold().underline());
    println!("  Address: {}", format!("{:#x}", address).bright_green());
    println!("  Short:   {}", format_address_display(&format!("{:#x}", address)));

    // 2. Asynchronously fetch data from network
    if balance_flag || nonce_flag {
        let progress = create_progress_bar(2);

        // Create provider once for all requests
        let provider = ethereum_boilerplate_utils::network::create_provider(&config.network)?;

        if balance_flag {
            progress.set_message("Fetching balance...");
            // ethers methods are called directly on provider
            let balance = provider.get_balance(address).await
                .map_err(|e| ethereum_boilerplate_utils::UtilsError::Ethereum(e.to_string()))?;

            progress.inc(1);
            println!("  Balance: {} ETH", format_wei(balance).bright_yellow());
        }

        if nonce_flag {
            progress.set_message("Fetching nonce...");
            let nonce = provider.get_transaction_count(address).await
                .map_err(|e| ethereum_boilerplate_utils::UtilsError::Ethereum(e.to_string()))?;

            progress.inc(1);
            println!("  Nonce:   {}", nonce.to_string().bright_blue());
        }

        progress.finish_and_clear();
    }

    print_success("Wallet information retrieved successfully!");
    Ok(())
}

pub async fn handle_wallet_command(config: Config, action: WalletCommands) -> Result<()> {
    match action {
        WalletCommands::Info { private_key, balance, nonce } => {
            show_wallet_info(config, private_key, balance, nonce).await
        }
    }
}