//! Server command implementation

use crate::{Cli, Terminal, create_spinner};
use crate::utils::Messenger;
use ethereum_boilerplate_utils::{Config, Result, UtilsError};
use clap::Parser;
use tokio::process::Command;
use std::path::Path;

pub async fn run_server(config: Config, port: u16, watch: bool) -> Result<()> {
    let ui = Cli::parse();
    ui.banner();

    // 1. Validate project structure
    let server_dir = Path::new("crates/server");
    if !server_dir.exists() {
        let msg = "'server' crate not found. Make sure you are in the project root.";
        ui.fail(msg);
        return Err(UtilsError::config_error(msg));
    }

    // 2. Preparation and build
    let pb = create_spinner("Preparing environment and building server...");

    // Pass environment variables from config (DB, keys, etc.)
    let db_url = config.database_url.unwrap_or_else(|| "postgres://localhost/db".into());

    if !watch {
        let build_status = Command::new("cargo")
            .args(&["build", "-p", "server"]) // Build specific package from workspace
            .status()
            .await?;

        if !build_status.success() {
            pb.finish_and_clear();
            ui.fail("Server compilation failed.");
            return Err(UtilsError::internal("Build failed"));
        }
    }

    pb.finish_with_message("Server ready to launch!");

    // 3. Choose launch strategy
    //
    let mut child = if watch {
        ui.success(&format!("Starting in WATCH mode on http://localhost:{}", port));

        // Check for cargo-watch
        if !check_cargo_watch().await {
            ui.fail("Tool 'cargo-watch' not found. Install it: cargo install cargo-watch");
            return Err(UtilsError::internal("cargo-watch missing"));
        }

        Command::new("cargo")
            .args(&["watch", "-x", &format!("run -p server -- --port {}", port)])
            .env("DATABASE_URL", db_url)
            .env("RUST_LOG", "debug")
            .spawn()?
    } else {
        ui.success(&format!("Starting server on port {}...", port));

        // Binary path in workspace: target/debug/server
        let binary_path = Path::new("target/debug/server");

        Command::new(binary_path)
            .arg("--port")
            .arg(port.to_string())
            .env("DATABASE_URL", db_url)
            .env("RUST_LOG", "info")
            .spawn()
            .map_err(|e| UtilsError::internal(format!("Failed to launch binary file: {}. Run cargo build.", e)))?
    };

    // 4. Wait for completion
    let status = child.wait().await?;

    if !status.success() {
        ui.fail("Server terminated with error.");
    }

    Ok(())
}

async fn check_cargo_watch() -> bool {
    Command::new("cargo-watch")
        .arg("--version")
        .output()
        .await
        .is_ok()
}