//! Deploy command implementation

use crate::{Cli, Terminal, create_spinner};
use crate::utils::Messenger;
use ethereum_boilerplate_utils::{Config, Result, print_banner, print_error};
use crate::commands::contract::prompt_if_none;
use tokio::process::Command;
use ethereum_boilerplate_shared::ContractTemplate;
use clap::Parser;

pub async fn deploy_contracts(
    config: Config,
    contract_type: String,
    private_key: Option<String>,
    _network_opt: Option<String>, // Не используется пока
    yes: bool,
) -> Result<()> {
    let ui = Cli::parse();
    print_banner();

    let contract_template = ContractTemplate::parse(&contract_type);
    let _contract_name: String = match contract_template {
        Some(ContractTemplate::Erc20) => ContractTemplate::Erc20.default_name().to_string(),
        Some(ContractTemplate::Erc721) => ContractTemplate::Erc721.default_name().to_string(),
        Some(ContractTemplate::Custom) => {
            let proceed = prompt_if_none(_network_opt, "Enter contract name", None)?;
            format!("contracts/{}.sol", proceed.trim())
        }
        None => {
            let available_templates = ContractTemplate::all()
                .iter()
                .map(|t: &ContractTemplate| t.to_string())
                .collect::<Vec<_>>()
                .join(", ");
            print_error(&format!("Неверный тип контракта. Доступны: {}", available_templates));
            return Ok(());
        }
    };

    // Получаем и валидируем приватный ключ
    let pk = match private_key {
        Some(key) => key,
        None => {
            ui.fail("Приватный ключ обязателен для развертывания контракта");
            return Ok(());
        }
    };

    // 3. Выполнение развертывания
    match contract_template {
        Some(ContractTemplate::Erc20) | Some(ContractTemplate::Erc721) => {
            execute_deployment(&ui, &config, &pk, &contract_type).await?;
        },
        Some(ContractTemplate::Custom) => {
            execute_deployment(&ui, &config, &pk, &contract_type).await?;
        },
        None => if !yes {
            ui.fail("Развертывание отменено пользователем.");
            return Ok(());
        }
    }

    ui.success("Все контракты успешно развернуты!");
    Ok(())
}

async fn execute_deployment(ui: &Cli, config: &Config, pk: &str, contract_kind: &str) -> Result<()> {
    let pb = create_spinner(format!("Работа с {}", contract_kind.to_uppercase()).as_str());
    
    // Шаг 1: Компиляция и развертывание через внутренний скрипт
    // Мы объединяем компиляцию и деплой в один вызов для скорости
    pb.set_message(format!("Компиляция и деплой {}...", contract_kind));
    
    let status = Command::new("cargo")
        .args(&["run", "--bin", "deploy", "--", "--contract", contract_kind, "--deploy"])
        .env("PRIVATE_KEY", pk)
        .env("NETWORK", &config.network.name)
        .current_dir("crates/smart-contracts")
        .status()
        .await?;
    
    pb.finish_and_clear();
    
    if status.success() {
        ui.success(&format!("Contract {} deployed successfully!", contract_kind));
    } else {
        ui.fail("Contract deployment failed");
    }
    
    Ok(())
}