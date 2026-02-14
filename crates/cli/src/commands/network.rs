//! Network command implementation

use crate::{print_banner, print_success, print_error, create_progress_bar};
use ethereum_boilerplate_utils::{Config, Result};
use ethereum_boilerplate_utils::formatting::format_gas_price;
use colored::Colorize;
use crate::NetworkCommands;

pub async fn handle_network_command(config: Config, action: NetworkCommands) -> Result<()> {
    print_banner();
    
    match action {
        NetworkCommands::Status => show_network_status(config).await?,
        NetworkCommands::GasPrice => show_gas_price(config).await?,
        NetworkCommands::BlockNumber => show_block_number(config).await?,
        NetworkCommands::Switch { network } => switch_network(network).await?,
        NetworkCommands::List => list_networks().await?,
    }
    
    Ok(())
}

async fn show_network_status(config: Config) -> Result<()> {
    let progress = create_progress_bar(3);
    
    progress.set_message("Connecting to network...");
    progress.inc(1);
    
    // Check network health
    let provider = ethereum_boilerplate_utils::network::create_provider(&config.network)?;
    ethereum_boilerplate_utils::network::check_provider_health(&provider).await?;
    
    progress.set_message("Getting network info...");
    progress.inc(1);
    
    // Get network information
    let chain_id = ethereum_boilerplate_utils::network::get_chain_id(&provider).await?;
    let block_number = ethereum_boilerplate_utils::network::get_latest_block_number(&provider).await?;
    
    progress.set_message("Finalizing...");
    progress.inc(1);
    
    progress.finish();
    
    println!("Network Status:");
    println!("  Name:       {}", config.network.name);
    println!("  Chain ID:   {}", chain_id);
    println!("  Block:       {}", block_number);
    println!("  RPC URL:     {}", config.network.rpc_url);
    println!("  Explorer:    {}", config.network.explorer_url);
    
    print_success("Network status retrieved successfully!");
    Ok(())
}

async fn show_gas_price(config: Config) -> Result<()> {
    let progress = create_progress_bar(2);
    
    progress.set_message("Getting gas price...");
    progress.inc(1);
    
    // Get gas price
    let provider = ethereum_boilerplate_utils::network::create_provider(&config.network)?;
    let gas_price = ethereum_boilerplate_utils::network::get_gas_price(&provider).await?;
    
    progress.set_message("Formatting...");
    progress.inc(1);
    
    progress.finish();
    
    println!("Gas Price Information:");
    println!("  Current:   {}", format_gas_price(gas_price));
    println!("  Wei:        {}", gas_price);
    
    print_success("Gas price retrieved successfully!");
    Ok(())
}

async fn show_block_number(config: Config) -> Result<()> {
    let progress = create_progress_bar(2);
    
    progress.set_message("Getting latest block...");
    progress.inc(1);
    
    // Get block number
    let provider = ethereum_boilerplate_utils::network::create_provider(&config.network)?;
    let block_number = ethereum_boilerplate_utils::network::get_latest_block_number(&provider).await?;
    
    progress.set_message("Formatting...");
    progress.inc(1);
    
    progress.finish();
    
    println!("Latest Block:");
    println!("  Number: {}", block_number);
    println!("  Hex:    0x{:x}", block_number);
    
    print_success("Block number retrieved successfully!");
    Ok(())
}

async fn switch_network(network: String) -> Result<()> {
    let progress = create_progress_bar(2);
    
    progress.set_message("Validating network...");
    progress.inc(1);
    
    // Validate network
    let network_config = match ethereum_boilerplate_utils::config::NetworkConfig::get_by_name(&network) {
        Some(config) => config,
        None => {
            progress.finish();
            print_error(&format!("Unknown network: {}", network));
            return Ok(());
        }
    };
    
    progress.set_message("Updating configuration...");
    progress.inc(1);
    
    // Update configuration
    let mut config = Config::from_env()?;
    config.network = network_config.clone();
    config.save_to_file("config.json")?;
    
    progress.finish();
    
    println!("Switched to network: {}", network);
    println!("Chain ID: {}", network_config.chain_id);
    
    print_success("Network switched successfully!");
    Ok(())
}

async fn list_networks() -> Result<()> {
    println!("Supported Networks:");
    
    let networks = vec![
        ("mainnet", "Ethereum Mainnet", 1),
        ("sepolia", "Ethereum Sepolia", 11155111),
        ("polygon", "Polygon", 137),
        ("arbitrum", "Arbitrum One", 42161),
        ("optimism", "Optimism", 10),
        ("localhost", "Localhost", 31337),
    ];
    
    for (name, description, chain_id) in networks {
        println!("  {} ({})", name.bright_green(), description);
        println!("    Chain ID: {}", chain_id);
        println!();
    }
    
    print_success("Networks listed successfully!");
    Ok(())
}
