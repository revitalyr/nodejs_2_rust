use crate::{Cli, Terminal, create_spinner};
use crate::utils::Messenger;
use ethereum_boilerplate_utils::{Result, UtilsError};
use tokio::process::Command;
use std::fs;
use std::path::{Path, PathBuf};
use dialoguer::{Input, Select};
use clap::Parser;
use colored::Colorize;

/// Initialize new project from template
#[allow(dead_code)]
pub async fn init_project(name: Option<String>, template_arg: String, git: bool) -> Result<()> {
    let cli_ui = Cli::parse(); // For access to Terminal methods (banner, success, etc.)
    cli_ui.banner();

    // 1. Interactive data input
    let project_name = match name {
        Some(ref n) => n.clone(),
        None => Input::<String>::new()
            .with_prompt("Project name")
            .default("my-eth-dapp".into())
            .interact_text()
            .map_err(|e| UtilsError::interactive_error(e.to_string()))?,
    };

    let selected_template = if template_arg == "basic" && name.is_none() {
        let options = vec!["basic", "defi", "nft", "marketplace"];
        let selection = Select::new()
            .with_prompt("Select project template")
            .items(&options)
            .default(0)
            .interact()
            .map_err(|e| UtilsError::interactive_error(e.to_string()))?;
        options[selection].to_string()
    } else {
        template_arg
    };

    let project_path = PathBuf::from(&project_name);
    if project_path.exists() {
        cli_ui.fail(&format!("Directory '{}' already exists", project_name));
        return Ok(());
    }

    // 2. Initialize structure
    let pb = create_spinner("Deploying workspace...");

    fs::create_dir_all(&project_path)?;
    std::env::set_current_dir(&project_path)?;

    setup_workspace(&project_name)?;
    apply_template(&selected_template, &project_name)?;

    // 3. Dependencies and Git
    pb.set_message("Setting up dependencies (cargo update)...");
    run_cmd("cargo", &["update"]).await?;

    if git {
        pb.set_message("Initializing Git...");
        setup_git().await?;
    }

    pb.finish_and_clear();
    cli_ui.success(&format!("Project '{}' ready!", project_name));

    // 4. Instructions
    println!("\n🚀 Quick start:");
    println!("  cd {}", project_name.cyan());
    println!("  eth-bp server --watch");

    Ok(())
}

fn setup_workspace(_project_name: &str) -> Result<()> {
    let dirs = [
        "crates/server/src",
        "crates/frontend/src",
        "crates/smart-contracts",
        "crates/shared/src",
        "crates/utils",
        "config",
        "scripts",
    ];

    for dir in dirs {
        fs::create_dir_all(dir)?;
    }

    // Генерация корневого Cargo.toml (используем зависимости из вашего примера)
    let workspace_toml = format!(
        r#"[workspace]
members = ["crates/*"]
resolver = "2"

[workspace.dependencies]
axum = "0.7"
tokio = {{ version = "1.0", features = ["full"] }}
leptos = {{ version = "0.6", features = ["csr"] }}
# ... остальные зависимости
"#
    );
    fs::write("Cargo.toml", workspace_toml)?;
    Ok(())
}

fn apply_template(template: &str, project_name: &str) -> Result<()> {
    // Server setup
    let server_cargo = format!(
        r#"[package]
name = "{}-server"
version = "0.1.0"
edition = "2021"

[dependencies]
tokio = {{ workspace = true }}
axum = {{ workspace = true }}
"# , project_name);

    write_file("crates/server/Cargo.toml", &server_cargo)?;
    write_file("crates/server/src/main.rs", "fn main() { println!(\"Hello Server\"); }")?;

    // Frontend setup (Leptos 0.6)
    let frontend_cargo = format!(
        r#"[package]
name = "{}-frontend"
version = "0.1.0"
edition = "2021"

[dependencies]
leptos = {{ workspace = true }}
"# , project_name);

    write_file("crates/frontend/Cargo.toml", &frontend_cargo)?;

    // Специфичные для шаблонов файлы
    match template {
        "defi" => { /* Добавить логику для DeFi шаблона */ },
        "nft" => { /* Добавить логику для NFT шаблона */ },
        _ => {}
    }

    Ok(())
}

async fn run_cmd(bin: &str, args: &[&str]) -> Result<()> {
    let status = Command::new(bin)
        .args(args)
        .stdout(std::process::Stdio::null())
        .spawn()?
        .wait()
        .await?;

    if !status.success() {
        return Err(UtilsError::internal(format!("Команда {} завершилась с ошибкой", bin)));
    }
    Ok(())
}

fn write_file<P: AsRef<Path>>(path: P, content: &str) -> Result<()> {
    if let Some(parent) = path.as_ref().parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, content)?;
    Ok(())
}

/// Инициализация Git репозитория
#[allow(dead_code)]
async fn setup_git() -> Result<()> {
    run_cmd("git", &["init"]).await?;
    let ignore = "/target\n.env\nCargo.lock";
    fs::write(".gitignore", ignore)?;
    Ok(())
}