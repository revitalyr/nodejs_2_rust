//! Development utilities command implementation

use colored::Colorize;
use crate::{Cli, DevSubcommands, Terminal};
use crate::utils::{create_spinner, Messenger};
use ethereum_boilerplate_utils::{Config, Result, format_address_display};
use alloy::signers::{Signer, local::PrivateKeySigner};
use rand::Rng;
use std::fs;
use tokio::process::Command;
use ethereum_boilerplate_shared::{ContractTemplate};
use clap::Parser;

/// Обработка dev подкоманд
#[allow(dead_code)]
pub async fn handle_dev_command(config: Config, action: DevSubcommands) -> Result<()> {
    let ui = Cli::parse(); // Используем для Terminal методов
    ui.banner();

    match action {
        DevSubcommands::GenerateWallet => generate_wallet(&ui).await,
        DevSubcommands::RunNode { port } => run_local_node(&ui, port, 0).await,
        DevSubcommands::Migrate { rollback } => migrate_database(&ui, config, rollback).await,
    }
}

// --- Генерация кошелька ---

pub async fn generate_wallet(ui: &Cli) -> Result<()> {
    let pb = create_spinner("Генерация криптографических ключей...");

    let wallet = PrivateKeySigner::random();
    let address = format!("{:#x}", wallet.address());
    let private_key = format!("{:#x}", wallet.to_bytes());

    pb.finish_and_clear();

    println!("{}", "🔐 Новый кошелек успешно создан:".bright_green().bold());
    println!("  {:<15} {}", "Адрес:".cyan(), address);
    println!("  {:<15} {}", "Отображение:".cyan(), format_address_display(&address));
    println!("  {:<15} {}", "Приватный ключ:".red().bold(), private_key);

    println!("\n{}", "⚠️  МЕРЫ ПРЕДОСТОРОЖНОСТИ:".yellow().bold());
    println!("  - Никогда не передавайте Private Key третьим лицам.");
    println!("  - Этот ключ дает полный доступ к вашим средствам.");

    ui.success("Кошелек готов к использованию в тестовых сетях.");
    Ok(())
}

// --- Создание смарт-контрактов ---

/// Создание нового смарт-контракта
#[allow(dead_code)]
async fn create_contract(ui: &Cli, contract_type: String, name: String) -> Result<()> {
    let pb = create_spinner(format!("Подготовка шаблона {}...", contract_type));

    let content = match ContractTemplate::parse(&contract_type) {
        Some(ContractTemplate::Erc20) => templates::erc20(&name),
        Some(ContractTemplate::Erc721) => templates::erc721(&name),
        Some(ContractTemplate::Custom) => templates::custom(&name),
        None => {
            pb.finish_and_clear();
            let available_templates = ContractTemplate::all()
                .iter()
                .map(|t: &ContractTemplate| t.to_string())
                .collect::<Vec<_>>()
                .join(", ");
            Messenger::fail(ui, &format!("Неверный тип контракта. Доступны: {}", available_templates));
            return Ok(());
        }
    };

    fs::create_dir_all("contracts")?;
    let path = format!("contracts/{}.sol", name);
    fs::write(&path, content)?;

    pb.finish_with_message("Файл создан!");
    ui.success(&format!("Контракт сохранен в: {}", path.cyan()));
    Ok(())
}

// --- Запуск локальной сети ---

pub async fn run_local_node(ui: &Cli, port: u16, accounts: u32) -> Result<()> {
    // 1. Проверяем доступность инструментов
    let port_str = port.to_string();
    let accounts_str = accounts.to_string();
    let (cmd, args) = if check_cmd("anvil").await {
        ("anvil", vec![
            "--port", &port_str,
            "--accounts", &accounts_str,
            "--state-interval", "10"
        ])
    } else if check_cmd("npx").await {
        ("npx", vec!["hardhat", "node", "--port", &port_str])
    } else {
        Messenger::fail(ui, "Ни Foundry (anvil), ни Hardhat не найдены. Установите один из них.");
        return Ok(());
    };

    println!("🚀 Запуск локальной ноды ({}) на порту {}...", cmd.bright_green(), port);

    let mut child = Command::new(cmd)
        .args(args)
        .spawn()
        .map_err(|e| ethereum_boilerplate_utils::UtilsError::internal(e.to_string()))?;

    // Ожидаем завершения (например, через Ctrl+C)
    let status: std::process::ExitStatus = child.wait().await?;
    if !status.success() {
        Messenger::fail(ui, "Локальная нода завершила работу с ошибкой.");
    }
    Ok(())
}

// --- Миграции БД ---

pub async fn migrate_database(ui: &Cli, config: Config, rollback: bool) -> Result<()> {
    let db_url = config.database_url.ok_or_else(|| {
        Messenger::fail(ui, "DATABASE_URL не настроен в конфигурации.");
        ethereum_boilerplate_utils::UtilsError::config_error("Missing DB URL")
    })?;

    let pb = create_spinner("Выполнение миграций SQLx...");

    let action = if rollback { "rollback" } else { "run" };
    let status = Command::new("sqlx")
        .args(&["migrate", action])
        .env("DATABASE_URL", db_url)
        .status()
        .await?;

    pb.finish_and_clear();

    if status.success() {
        ui.success(&format!("Миграция ({}) успешно завершена.", action));
    } else {
        Messenger::fail(ui, "Ошибка при выполнении миграции. Проверьте статус базы данных.");
    }
    Ok(())
}

// --- Вспомогательные функции ---

async fn check_cmd(cmd: &str) -> bool {
    Command::new(cmd).arg("--version").output().await.is_ok()
}

mod templates {
    pub fn erc20(name: &str) -> String {
        format!(r#"// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "@openzeppelin/contracts/token/ERC20/ERC20.sol";

contract {name} is ERC20 {{
    constructor() ERC20("{name}", "SYM") {{
        _mint(msg.sender, 1000000 * 10**decimals());
    }}
}}"#)
    }

    pub fn erc721(name: &str) -> String {
        format!(r#"// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "@openzeppelin/contracts/token/ERC721/ERC721.sol";

contract {name} is ERC721 {{
    constructor() ERC721("{name}", "NFT") {{}}
}}"#)
    }

    pub fn custom(name: &str) -> String {
        format!("// SPDX-License-Identifier: MIT\npragma solidity ^0.8.20;\n\ncontract {name} {{\n    // Your logic here\n}}")
    }
}