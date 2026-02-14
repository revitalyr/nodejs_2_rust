//! Config command implementation

use crate::{print_banner, print_success, print_error, create_progress_bar};
use ethereum_boilerplate_utils::{Config, Result};
use crate::ConfigCommands;

pub async fn handle_config_command(config: Config, action: ConfigCommands) -> Result<()> {
    print_banner();
    
    match action {
        ConfigCommands::Show => show_config(config).await?,
        ConfigCommands::Set { key, value } => set_config_value(key, value).await?,
        ConfigCommands::Validate => validate_config(config).await?,
        ConfigCommands::Reset => reset_config().await?,
    }
    
    Ok(())
}

async fn show_config(config: Config) -> Result<()> {
    println!("Current Configuration:");
    println!("  Ethereum RPC URL: {}", config.ethereum_rpc_url);
    println!("  Moralis API Key: {}", config.moralis_api_key.as_deref().unwrap_or("Not set"));
    println!("  Database URL: {}", config.database_url.as_deref().unwrap_or("Not set"));
    println!("  Log Level: {}", config.log_level);
    println!("  Server Port: {}", config.server_port);
    println!("  Network:");
    println!("    Name: {}", config.network.name);
    println!("    Chain ID: {}", config.network.chain_id);
    println!("    RPC URL: {}", config.network.rpc_url);
    println!("    Explorer: {}", config.network.explorer_url);
    println!("    Native Currency: {}", config.network.native_currency);
    
    print_success("Configuration displayed successfully!");
    Ok(())
}

async fn set_config_value(key: String, value: String) -> Result<()> {
    let progress = create_progress_bar(2);
    
    progress.set_message("Loading current configuration...");
    progress.inc(1);
    
    let mut config = Config::from_env().unwrap_or_default();
    
    progress.set_message("Updating configuration...");
    progress.inc(1);
    
    // Update configuration based on key
    match key.as_str() {
        "ethereum_rpc_url" => config.ethereum_rpc_url = value.clone(),
        "moralis_api_key" => config.moralis_api_key = Some(value.clone()),
        "database_url" => config.database_url = Some(value.clone()),
        "log_level" => config.log_level = value.clone(),
        "server_port" => {
            config.server_port = value.parse()
                .map_err(|_| ethereum_boilerplate_utils::UtilsError::config_error("Invalid server port"))?;
        }
        "network" => {
            config.network = ethereum_boilerplate_utils::config::NetworkConfig::get_by_name(&value)
                .ok_or_else(|| ethereum_boilerplate_utils::UtilsError::config_error(format!("Unknown network: {}", value)))?;
        }
        _ => {
            progress.finish();
            print_error(&format!("Unknown configuration key: {}", key));
            return Ok(());
        }
    }
    
    // Save configuration
    config.save_to_file("config.json")?;
    
    progress.finish();
    
    println!("Configuration updated:");
    println!("  {} = {}", key, value);
    
    print_success("Configuration saved successfully!");
    Ok(())
}

async fn validate_config(config: Config) -> Result<()> {
    let progress = create_progress_bar(3);
    
    progress.set_message("Validating configuration...");
    progress.inc(1);
    
    // Validate configuration
    config.validate()?;
    
    progress.set_message("Testing network connection...");
    progress.inc(1);
    
    // Test network connection
    let provider = ethereum_boilerplate_utils::network::create_provider(&config.network)?;
    ethereum_boilerplate_utils::network::check_provider_health(&provider).await?;
    
    progress.set_message("Finalizing validation...");
    progress.inc(1);
    
    progress.finish();
    
    println!("Configuration validation:");
    println!("  ✅ Configuration file is valid");
    println!("  ✅ Network connection successful");
    println!("  ✅ All settings are correct");
    
    print_success("Configuration is valid!");
    Ok(())
}

async fn reset_config() -> Result<()> {
    let progress = create_progress_bar(2);
    
    progress.set_message("Creating default configuration...");
    progress.inc(1);
    
    let config = Config::default();
    
    progress.set_message("Saving configuration...");
    progress.inc(1);
    
    config.save_to_file("config.json")?;
    
    progress.finish();
    
    println!("Configuration has been reset to defaults:");
    println!("  Ethereum RPC URL: {}", config.ethereum_rpc_url);
    println!("  Log Level: {}", config.log_level);
    println!("  Server Port: {}", config.server_port);
    println!("  Network: {}", config.network.name);
    
    print_success("Configuration reset successfully!");
    Ok(())
}
