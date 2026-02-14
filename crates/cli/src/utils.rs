//! CLI utility functions and UI toolkit

use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use std::process::{ExitStatus, Stdio};
use tokio::process::Command;
use crate::Result; // Используем внутренний псевдоним типа из lib.rs или main.rs

/// Набор инструментов для визуального взаимодействия с пользователем
#[allow(dead_code)]
pub trait Messenger {
    fn info(&self, msg: &str);
    fn warn(&self, msg: &str);
    fn fail(&self, msg: &str);
}

pub struct CliUi;

impl Messenger for CliUi {
    fn info(&self, msg: &str) { println!("{} {}", "ℹ️".blue().bold(), msg); }
    fn warn(&self, msg: &str) { println!("{} {}", "⚠️".yellow().bold(), msg); }
    fn fail(&self, msg: &str) { eprintln!("{} {}", "❌".red().bold(), msg); }
}

/// Выводит стилизованный баннер приложения
pub fn print_banner() {
    let banner = format!(
        "╔{:═^60}╗\n║{:^60}║\n║{:^60}║\n╚{:═^60}╝",
        "",
        "Ethereum Boilerplate CLI v0.1.0".bright_cyan().bold(),
        "Full-stack Web3 development toolkit".bright_black(),
        ""
    );
    println!("{}", banner);
}

/// Выводит сообщение об успехе
pub fn print_success(msg: &str) {
    println!("{} {}", "✅".green().bold(), msg);
}

/// Выводит сообщение об ошибке
pub fn print_error(msg: &str) {
    eprintln!("{} {}", "❌".red().bold(), msg);
}

// --- Индикаторы прогресса ---

/// Создает спиннер для асинхронных задач (например, деплой)
pub fn create_spinner(msg: impl Into<String>) -> ProgressBar {
    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"])
            .template("{spinner:.green} {msg}")
            .unwrap()
    );
    pb.set_message(msg.into());
    pb.enable_steady_tick(std::time::Duration::from_millis(80));
    pb
}

/// Создает классический progress bar для шаговых задач (например, установка зависимостей)
pub fn create_progress_bar(total: u64) -> ProgressBar {
    let pb = ProgressBar::new(total);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})")
            .unwrap()
            .progress_chars("█▉▊▋▌▍▎▏  ")
    );
    pb
}

// --- Форматирование данных ---

/// Сокращает Ethereum адрес: 0x1234...abcd
#[allow(dead_code)]
pub fn format_address(address: &str) -> String {
    if address.starts_with("0x") && address.len() == 42 {
        format!("{}...{}", &address[..6], &address[38..])
    } else {
        address.chars().take(10).collect::<String>()
    }
}

// --- Системные команды ---

/// Проверяет наличие утилиты в системе
/// Проверка наличия бинарного файла
#[allow(dead_code)]
pub async fn is_installed(binary: &str) -> bool {
    Command::new(binary)
        .arg("--version")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .await
        .map(|s| s.success())
        .unwrap_or(false)
}

/// Обертка над выполнением команд с логированием ошибок
/// Выполняет команду в тихом режиме
#[allow(dead_code)]
pub async fn run_quiet_command(
    cmd_name: &str,
    args: &[&str],
    dir: Option<&str>,
) -> Result<ExitStatus> {
    let mut cmd = Command::new(cmd_name);
    cmd.args(args);

    if let Some(d) = dir {
        cmd.current_dir(d);
    }

    // Подавляем stdout, если не нужно засорять терминал деталями сборки
    let status = cmd
        .stdout(Stdio::null())
        .status()
        .await?;

    Ok(status)
}

/// Гарантирует наличие cargo-пакета (устанавливает, если нет)
/// Установка инструмента если отсутствует
#[allow(dead_code)]
pub async fn ensure_tool(tool_name: &str, install_package: &str) -> Result<()> {
    let ui = CliUi;
    if !is_installed(tool_name).await {
        ui.info(&format!("Инструмент '{}' не найден. Установка...", tool_name));
        let status = Command::new("cargo")
            .args(&["install", install_package])
            .status()
            .await?;

        if !status.success() {
            return Err(anyhow::anyhow!("Не удалось установить {}", tool_name).into());
        }
    }
    Ok(())
}