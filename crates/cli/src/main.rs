//! Ethereum Boilerplate CLI
//! Central entry point for all Ethereum Boilerplate functionality

use clap::{Parser, Subcommand};
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use std::path::Path;
use tracing::{info, error, warn};

// Предполагаем наличие этих модулей в структуре проекта
mod commands;
mod utils;

use ethereum_boilerplate_utils::{Config, init_logging, Result};

// Re-export utilities for command modules
pub use utils::{print_banner, print_success, print_error, create_progress_bar};

// Re-export command types
pub type NetworkCommands = NetworkSubcommands;
pub type ConfigCommands = ConfigSubcommands;
pub type WalletCommands = WalletSubcommands;

#[derive(Parser)]
#[command(
    name = "eth-bp",
    about = "🚀 Full-stack Web3 development toolkit",
    version,
    author
)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Enable verbose debug logging
    #[arg(short, long, global = true)]
    debug: bool,

    /// Path to config file
    #[arg(short, long, default_value = "config.json", global = true)]
    config: String,

    /// Target network (mainnet, sepolia, localhost, etc.)
    #[arg(short, long, default_value = "localhost", global = true)]
    network: String,
}

#[derive(Subcommand)]
enum Commands {
    /// 🌐 Backend server management
    Server {
        #[arg(short, long, default_value_t = 3000)]
        port: u16,
        #[arg(short, long)]
        watch: bool,
    },

    /// 💻 Frontend development server
    Frontend {
        #[arg(short, long, default_value_t = 3001)]
        port: u16,
        #[arg(short, long)]
        build: bool,
    },

    /// 🚀 Smart contract deployment
    Deploy {
        #[arg(short, long, default_value = "all")]
        contract_type: String,
        #[arg(short, long)]
        private_key: Option<String>,
        #[arg(short, long)]
        yes: bool,
    },

    /// 🛠️ Dev utilities (Node, Wallet, Migration)
    Dev {
        #[command(subcommand)]
        action: DevSubcommands,
    },

    /// ⚙️ Configuration management
    Config {
        #[command(subcommand)]
        action: ConfigSubcommands,
    },

    /// 🌐 Network operations
    Network {
        #[command(subcommand)]
        action: NetworkSubcommands,
    },

    /// 👛 Wallet operations
    Wallet {
        #[command(subcommand)]
        action: WalletSubcommands,
    },
}

#[derive(Subcommand)]
enum DevSubcommands {
    GenerateWallet,
    RunNode { #[arg(short, long, default_value_t = 8545)] port: u16 },
    Migrate { #[arg(short, long)] rollback: bool },
}

#[derive(Subcommand)]
pub enum ConfigSubcommands { 
    Show, 
    Validate, 
    Reset,
    Set { key: String, value: String },
}

#[derive(Subcommand)]
pub enum NetworkSubcommands {
    Status,
    GasPrice,
    BlockNumber,
    Switch { network: String },
    List,
}

#[derive(Subcommand)]
pub enum WalletSubcommands {
    Info {
        #[arg(short, long)]
        private_key: Option<String>,
        #[arg(short, long)]
        balance: bool,
        #[arg(short, long)]
        nonce: bool,
    },
}

// --- Трейт для улучшения взаимодействия с пользователем ---

trait Terminal {
    fn success(&self, msg: &str);
    fn banner(&self);
}

impl Terminal for Cli {
    fn success(&self, msg: &str) { println!("{} {}", "✅".green(), msg); }
    fn banner(&self) {
        println!("{}", "\n--- Ethereum Boilerplate CLI ---".bright_cyan().bold());
    }
}

impl crate::utils::Messenger for Cli {
    fn info(&self, msg: &str) { println!("{} {}", "ℹ️".blue(), msg); }
    fn warn(&self, msg: &str) { println!("{} {}", "⚠️".yellow(), msg); }
    fn fail(&self, msg: &str) { eprintln!("{} {}", "❌".red(), msg); }
}

// --- Main Entry Point ---

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // 1. Logging
    let log_level = if cli.debug { "debug" } else { "info" };
    init_logging(log_level).map_err(|e| {
        eprintln!("Critical: Logging init failed: {}", e);
        e
    })?;

    // 2. Configuration
    let config = load_config(&cli.config, &cli.network).await?;

    // 3. Greeting (только если не в тихом режиме)
    if !cli.debug { cli.banner(); }
    info!("Network: {}", config.network.name.cyan());

    // 4. Execution
    if let Err(e) = execute(cli, config).await {
        error!("Execution error: {}", e);
        std::process::exit(1);
    }

    Ok(())
}

async fn execute(cli: Cli, config: Config) -> Result<()> {
    match cli.command {
        Commands::Server { port, watch } => {
            commands::server::run_server(config, port, watch).await
        }
        Commands::Frontend { port, build } => {
            commands::frontend::run_frontend(config, port, build).await
        }
        Commands::Deploy { contract_type, private_key, yes } => {
            commands::deploy::deploy_contracts(config, contract_type, private_key, None, yes).await
        }
        Commands::Dev { ref action } => match action {
            DevSubcommands::GenerateWallet => commands::dev::generate_wallet(&cli).await,
            DevSubcommands::RunNode { port } => commands::dev::run_local_node(&cli, *port, 0).await,
            DevSubcommands::Migrate { rollback } => commands::dev::migrate_database(&cli, config, *rollback).await,
        },
        Commands::Config { action } => commands::config::handle_config_command(config, action).await,
        Commands::Network { action } => commands::network::handle_network_command(config, action).await,
        Commands::Wallet { action } => commands::wallet::handle_wallet_command(config, action).await,
    }
}

async fn load_config(path: &str, network_name: &str) -> Result<Config> {
    let mut config = if Path::new(path).exists() {
        Config::from_file(path)?
    } else {
        warn!("Config file not found at {}, using defaults", path);
        Config::from_env()?
    };

    // Принудительно меняем сеть, если передана через флаг --network
    if network_name != "localhost" {
        config.set_network(network_name)?;
    }

    Ok(config)
}

// --- Утилиты прогресса ---

pub fn create_spinner(msg: &str) -> ProgressBar {
    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"])
            .template("{spinner:.green} {msg}")
            .expect("Template error")
    );
    pb.set_message(msg.to_string());
    pb
}