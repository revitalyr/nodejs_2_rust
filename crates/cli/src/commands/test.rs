//! Test command implementation
use crate::{Cli, Terminal};
use crate::utils::{create_spinner, Messenger}; // Assume spinner is available here
use ethereum_boilerplate_utils::{Result, UtilsError};
use colored::*;
use clap::Parser;
use tokio::process::Command;
use std::process::Stdio;

/// Run tests based on selected type
pub async fn run_tests(
    _config: ethereum_boilerplate_utils::Config,
    test_type: String,
    release: bool,
    verbose: bool,
) -> Result<()> {
    // In this context, Cli is usually passed or parsed internally
    let ui = Cli::parse(); 
    
    match test_type.as_str() {
        "unit" => run_unit_tests(&ui, release, verbose).await,
        "integration" => run_integration_tests(&ui, release, verbose).await,
        "all" => {
            run_unit_tests(&ui, release, verbose).await?;
            run_integration_tests(&ui, release, verbose).await
        }
        _ => {
            ui.fail(&format!("Unknown test type: {}", test_type));
            Err(UtilsError::validation_error("Invalid test type"))
        }
    }
}

/// Run unit tests for all packages
async fn run_unit_tests(ui: &Cli, release: bool, verbose: bool) -> Result<()> {
    ui.info("🚀 Running Unit tests...");
    
    let packages = [
        "ethereum-boilerplate-utils",
        "ethereum-boilerplate-smart-contracts",
    ];

    for pkg in packages {
        execute_cargo_test(ui, pkg, release, verbose, None).await?;
    }

    ui.success("All unit tests passed successfully!");
    Ok(())
}

/// Run integration tests (Rust + WASM)
async fn run_integration_tests(ui: &Cli, release: bool, verbose: bool) -> Result<()> {
    ui.info("📡 Running integration tests...");

    // 1. Standard integration tests
    execute_cargo_test(ui, "integration", release, verbose, Some("--test")).await?;

    // 2. Frontend tests in WASM environment
    if check_wasm_pack().await {
        let pb = create_spinner("Running WebAssembly tests (headless)...");
        
        let mut wasm_cmd = Command::new("wasm-pack");
        wasm_cmd.args(&["test", "--headless", "--firefox"])
            .current_dir("crates/frontend");

        if verbose { wasm_cmd.arg("--verbose"); }

        let status = wasm_cmd.status().await?;
        pb.finish_and_clear();

        if !status.success() {
            ui.fail("WebAssembly tests failed.");
            return Err(UtilsError::internal("WASM tests failed"));
        }
    } else {
        ui.warn("Tool 'wasm-pack' not found. Skipping WASM tests.");
    }

    ui.success("Integration tests passed successfully!");
    Ok(())
}

// --- Helper Functions ---

async fn execute_cargo_test(
    ui: &Cli,
    target: &str,
    release: bool,
    verbose: bool,
    mode: Option<&str>,
) -> Result<()> {
    let msg = format!("Testing {}...", target);
    
    // Manage spinner or direct output
    let pb = if !verbose {
        Some(create_spinner(&msg))
    } else {
        ui.info(&msg);
        None
    };

    let mut cmd = Command::new("cargo");
    cmd.arg("test");

    if let Some(m) = mode {
        cmd.args(&[m, target]);
    } else {
        cmd.args(&["--package", target]);
    }

    if release { cmd.arg("--release"); }

    if verbose {
        cmd.args(&["--", "--nocapture"]);
        cmd.stdout(Stdio::inherit());
        cmd.stderr(Stdio::inherit());
    } else {
        cmd.stdout(Stdio::null());
        cmd.stderr(Stdio::null());
    }

    let status = cmd.status().await?;

    if let Some(p) = pb { p.finish_and_clear(); }

    if !status.success() {
        ui.fail(&format!("Error in tests: {}", target));
        return Err(UtilsError::internal(format!("Tests failed for {}", target)));
    }

    Ok(())
}

async fn check_wasm_pack() -> bool {
    Command::new("wasm-pack")
        .arg("--version")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .await
        .is_ok()
}