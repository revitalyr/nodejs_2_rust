//! Frontend command implementation

// Предполагаем, что утилиты печати на самом деле находятся в библиотеке utils,
// так как CLI обычно импортирует их оттуда для единообразия.
use ethereum_boilerplate_utils::{
    print_banner, print_success, print_error, create_progress_bar, // Проверьте, что они pub там
    Config, Result, UtilsError
};
use ethereum_boilerplate_shared::paths::FRONTEND_PATH;
use tokio::process::Command;
use std::path::Path;
use std::process::ExitStatus;

pub async fn run_frontend(_config: Config, port: u16, build: bool) -> Result<()> {
    print_banner();

    // 1. Валидация окружения
    ensure_frontend_exists()?;
    ensure_wasm_target().await?;
    ensure_trunk_installed().await?;

    if build {
        print_success("Запуск сборки фронтенда (release)...");
        execute_trunk(&["build", "--release"], Some("Сборка артефактов")).await?;
        print_success("Фронтенд собран в crates/frontend/dist/");
    } else {
        start_dev_server(port).await?;
    }

    Ok(())
}

/// Проверка существования директории фронтенда
fn ensure_frontend_exists() -> Result<()> {
    if !Path::new(FRONTEND_PATH).is_dir() {
        return Err(UtilsError::config_error(format!(
            "Исходники фронтенда не найдены по пути: {}. Проверьте структуру проекта.",
            FRONTEND_PATH
        )));
    }
    Ok(())
}

/// Проверка и установка wasm-target
async fn ensure_wasm_target() -> Result<()> {
    let output = Command::new("rustup")
        .args(["target", "list", "--installed"])
        .output()
        .await
        .map_err(|e| UtilsError::internal(format!("Rustup не найден: {}", e)))?;

    if !String::from_utf8_lossy(&output.stdout).contains("wasm32-unknown-unknown") {
        print_error("WASM target отсутствует. Установка...");
        run_silent_command("rustup", &["target", "add", "wasm32-unknown-unknown"]).await?;
    }
    Ok(())
}

/// Проверка наличия Trunk
async fn ensure_trunk_installed() -> Result<()> {
    if Command::new("trunk").arg("--version").output().await.is_err() {
        print_success("Trunk не найден. Установка через cargo...");
        run_silent_command("cargo", &["install", "trunk"]).await?;
    }
    Ok(())
}

/// Запуск сервера разработки
async fn start_dev_server(port: u16) -> Result<()> {
    print_success(&format!("Сервер запускается на http://localhost:{}", port));

    // Используем .status(), чтобы Trunk полностью завладел терминалом (цвета, логи, интерактивность)
    let status = Command::new("trunk")
        .args(["serve", "--port", &port.to_string(), "--open"])
        .current_dir(FRONTEND_PATH)
        .status()
        .await
        .map_err(|e| UtilsError::internal(format!("Не удалось запустить Trunk: {}", e)))?;

    handle_exit_status(status, "Trunk serve")
}

/// Хелпер для запуска Trunk с индикацией прогресса
async fn execute_trunk(args: &[&str], message: Option<&str>) -> Result<()> {
    let pb = if let Some(msg) = message {
        let p = create_progress_bar(1);
        p.set_message(msg.to_string());
        Some(p)
    } else {
        None
    };

    let status = Command::new("trunk")
        .args(args)
        .current_dir(FRONTEND_PATH)
        .status()
        .await
        .map_err(|e| UtilsError::internal(format!("Ошибка выполнения Trunk: {}", e)))?;

    if let Some(p) = pb { p.finish_and_clear(); }
    handle_exit_status(status, "Trunk")
}

/// Тихий запуск команд (для установки зависимостей)
async fn run_silent_command(cmd: &str, args: &[&str]) -> Result<()> {
    let status = Command::new(cmd)
        .args(args)
        .status()
        .await
        .map_err(|e| UtilsError::internal(format!("Ошибка при запуске {}: {}", cmd, e)))?;

    handle_exit_status(status, cmd)
}

fn handle_exit_status(status: ExitStatus, cmd_name: &str) -> Result<()> {
    if status.success() {
        Ok(())
    } else {
        Err(UtilsError::internal(format!(
            "Команда '{}' завершилась с кодом: {}",
            cmd_name,
            status.code().unwrap_or(-1)
        )))
    }
}