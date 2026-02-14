//! Build command implementation

use crate::{Cli, Terminal, create_spinner};
use ethereum_boilerplate_utils::{Config, Result, UtilsError};
use crate::utils::{CliUi, Messenger};
use tokio::process::Command;
use std::path::Path;
use colored::Colorize;
use clap::Parser;

/// Сборка всего проекта
#[allow(dead_code)]
pub async fn build_project(
    _config: Config,
    release: bool,
    skip_frontend: bool,
    skip_backend: bool,
) -> Result<()> {
    let ui = Cli::parse();
    ui.banner();

    // 1. Сборка Бэкенда
    if !skip_backend {
        build_backend(&ui, release).await?;
    }

    // 2. Сборка Фронтенда
    if !skip_frontend {
        build_frontend(&ui, release).await?;
    }

    // 3. Итоговый отчет
    show_artifacts(&ui, release, skip_backend, skip_frontend);

    Ok(())
}

/// Сборка Rust бэкенда
#[allow(dead_code)]
async fn build_backend(ui: &Cli, release: bool) -> Result<()> {
    let pb = create_spinner("Сборка бэкенда (Rust server)...");

    if !Path::new("crates/server").exists() {
        pb.finish_and_clear();
        return Err(UtilsError::config_error("Крейт 'server' не найден в crates/server"));
    }

    let mut args = vec!["build", "-p", "server"];
    if release {
        args.push("--release");
    }

    let status = Command::new("cargo")
        .args(&args)
        .status()
        .await?;

    pb.finish_and_clear();

    if !status.success() {
        ui.fail("Сборка бэкенда провалилась.");
        return Err(UtilsError::internal("Backend build failed"));
    }

    ui.success("Бэкенд успешно собран.");
    Ok(())
}

/// Сборка Frontend (Leptos + WASM)
#[allow(dead_code)]
async fn build_frontend(ui: &Cli, release: bool) -> Result<()> {
    let pb = create_spinner("Подготовка Trunk и сборка WASM...");

    if !Path::new("crates/frontend").exists() {
        pb.finish_and_clear();
        return Err(UtilsError::config_error("Крейт 'frontend' не найден"));
    }

    // Проверка Trunk
    if !check_or_install_trunk(ui).await? {
        pb.finish_and_clear();
        return Err(UtilsError::internal("Trunk not available"));
    }

    let mut args = vec!["build"];
    if release {
        args.push("--release");
    }

    let status = Command::new("trunk")
        .args(&args)
        .current_dir("crates/frontend")
        .status()
        .await?;

    pb.finish_and_clear();

    if !status.success() {
        ui.fail("Сборка фронтенда (Trunk) провалилась.");
        return Err(UtilsError::internal("Frontend build failed"));
    }

    ui.success("Фронтенд успешно скомпилирован в WASM.");
    Ok(())
}

/// Проверка и установка Trunk если нужно
#[allow(dead_code)]
async fn check_or_install_trunk(_ui: &Cli) -> Result<bool> {
    let ui = CliUi;
    let has_trunk = Command::new("trunk").arg("--version").output().await.is_ok();

    if !has_trunk {
        ui.info("Trunk не найден. Попытка автоматической установки...");
        let install_status = Command::new("cargo")
            .args(&["install", "trunk"])
            .status()
            .await?;

        return Ok(install_status.success());
    }

    Ok(true)
}

/// Проверка наличия инструмента
#[allow(dead_code)]
async fn is_installed(tool: &str) -> bool {
    Command::new(tool)
        .arg("--version")
        .output()
        .await
        .map(|output| output.status.success())
        .unwrap_or(false)
}

fn show_artifacts(ui: &Cli, release: bool, skip_backend: bool, skip_frontend: bool) {
    println!("\n{}", "📦 Сборка завершена. Артефакты:".bold().underline());

    let mode = if release { "release" } else { "debug" };

    if !skip_backend {
        let backend_binary = format!("target/{}/server", mode);
        println!("  {:<12} {}", "Бэкенд:".cyan(), backend_binary.green());
    }

    if !skip_frontend {
        println!("  {:<12} {}", "Фронтенд:".cyan(), "crates/frontend/dist/".green());
    }

    ui.success("Проект готов к деплою!");
}