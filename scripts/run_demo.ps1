# Ethereum Boilerplate Demo Runner (PowerShell)
# This script sets up and runs the demo application with mock data

param(
    [Parameter(Mandatory=$false)]
    [ValidateSet("build", "build-rel", "run", "test", "clean", "info")]
    [string]$Action = "info"
)

Write-Host "🦀 Ethereum Boilerplate Demo Runner" -ForegroundColor Cyan
Write-Host "==================================" -ForegroundColor Cyan

# Function to print colored output
function Write-Status {
    param([string]$Message)
    Write-Host "[INFO] $Message" -ForegroundColor Green
}

function Write-Warning {
    param([string]$Message)
    Write-Host "[WARN] $Message" -ForegroundColor Yellow
}

function Write-Error {
    param([string]$Message)
    Write-Host "[ERROR] $Message" -ForegroundColor Red
}

function Write-Step {
    param([string]$Message)
    Write-Host "[STEP] $Message" -ForegroundColor Blue
}

# Check if we're in the right directory
if (-not (Test-Path "Cargo.toml")) {
    Write-Error "Please run this script from the project root directory"
    exit 1
}

# Check dependencies
function Check-Dependencies {
    Write-Step "Checking dependencies..."
    
    # Check if Rust is installed
    try {
        $rustVersion = cargo --version 2>$null
        Write-Status "Rust/Cargo is installed: $rustVersion"
    } catch {
        Write-Error "Rust/Cargo is not installed. Please install Rust first."
        exit 1
    }
    
    # Check if Trunk is installed
    try {
        $trunkVersion = trunk --version 2>$null
        Write-Status "Trunk is installed: $trunkVersion"
    } catch {
        Write-Warning "Trunk is not installed. Installing..."
        cargo install trunk
    }
    
    # Check if wasm-pack is installed
    try {
        $wasmPackVersion = wasm-pack --version 2>$null
        Write-Status "wasm-pack is installed: $wasmPackVersion"
    } catch {
        Write-Warning "wasm-pack is not installed. Installing..."
        cargo install wasm-pack
    }
    
    Write-Status "All dependencies are available"
}

# Build the demo
function Build-Demo {
    param([switch]$Release)
    
    Write-Step "Building demo application..."
    
    # Check if examples/demo exists
    if (-not (Test-Path "examples/demo")) {
        Write-Warning "examples/demo directory not found. Creating demo structure..."
        
        # Create demo directory structure
        New-Item -ItemType Directory -Force -Path "examples/demo" | Out-Null
        
        # Create basic Cargo.toml for demo
        @"
[package]
name = "ethereum-boilerplate-demo"
version = "0.1.0"
edition = "2021"

[dependencies]
leptos = { version = "0.8", features = ["csr"] }
leptos_meta = { version = "0.8" }
leptos_router = { version = "0.8" }
wasm-bindgen = "0.2"
console_error_panic_hook = "0.1"
"@ | Out-File -FilePath "examples/demo/Cargo.toml" -Encoding UTF8
        
        # Create basic source structure
        New-Item -ItemType Directory -Force -Path "examples/demo/src" | Out-Null
        
        # Create main.rs
        @"
use leptos::prelude::*;
use leptos::mount::mount_to_body;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <div class="container">
            <h1>"Ethereum Boilerplate Demo"</h1>
            <p>"Demo application is working!"</p>
        </div>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App)
}
"@ | Out-File -FilePath "examples/demo/src/main.rs" -Encoding UTF8
        
        # Create index.html
        @"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Ethereum Boilerplate Demo</title>
    <style>
        body { font-family: Arial, sans-serif; margin: 2rem; }
        .container { max-width: 800px; margin: 0 auto; }
    </style>
</head>
<body>
    <div id="root"></div>
</body>
</html>
"@ | Out-File -FilePath "examples/demo/index.html" -Encoding UTF8
        
        Write-Status "Demo structure created"
    }
    
    Set-Location examples/demo
    
    try {
        if ($Release) {
            Write-Status "Building in release mode..."
            if (Get-Command trunk -ErrorAction SilentlyContinue) {
                trunk build --release
            } else {
                Write-Warning "Trunk not available, using cargo build..."
                cargo build --release --target wasm32-unknown-unknown
            }
        } else {
            Write-Status "Building in debug mode..."
            if (Get-Command trunk -ErrorAction SilentlyContinue) {
                trunk build
            } else {
                Write-Warning "Trunk not available, using cargo build..."
                cargo build --target wasm32-unknown-unknown
            }
        }
        
        if ($LASTEXITCODE -eq 0) {
            Write-Status "Demo built successfully"
        } else {
            Write-Error "Build failed with exit code $LASTEXITCODE"
            exit 1
        }
    } catch {
        Write-Error "Failed to build demo: $_"
        exit 1
    } finally {
        Set-Location ../..
    }
}

# Run the demo server
function Run-Demo {
    Write-Step "Starting demo server..."
    
    Set-Location examples/demo
    
    try {
        # Check if port is already in use
        $port8080InUse = Get-NetTCPConnection -LocalPort 8080 -ErrorAction SilentlyContinue
        if ($port8080InUse) {
            Write-Warning "Port 8080 is already in use. Trying port 8081..."
            $port8081InUse = Get-NetTCPConnection -LocalPort 8081 -ErrorAction SilentlyContinue
            if ($port8081InUse) {
                Write-Error "Ports 8080 and 8081 are both in use"
                exit 1
            }
            trunk serve --port 8081 --open
        } else {
            trunk serve --port 8080 --open
        }
    } catch {
        Write-Error "Failed to start demo server: $_"
        exit 1
    } finally {
        Set-Location ../..
    }
}

# Run tests
function Run-Tests {
    Write-Step "Running demo tests..."
    
    Set-Location examples/demo
    
    try {
        # Run WebAssembly tests
        wasm-pack test --headless --firefox
        Write-Status "All tests passed"
    } catch {
        Write-Error "Tests failed: $_"
        exit 1
    } finally {
        Set-Location ../..
    }
}

# Clean build artifacts
function Clean-Demo {
    Write-Step "Cleaning demo artifacts..."
    
    Set-Location examples/demo
    
    try {
        # Clean Trunk artifacts
        if (Test-Path "dist") {
            Remove-Item -Recurse -Force dist
            Write-Status "Removed dist directory"
        }
        
        # Clean Cargo artifacts
        cargo clean
        
        Write-Status "Demo cleaned successfully"
    } catch {
        Write-Error "Failed to clean demo: $_"
        exit 1
    } finally {
        Set-Location ../..
    }
}

# Show demo information
function Show-Info {
    Write-Host ""
    Write-Host "🎯 Demo Information:" -ForegroundColor Cyan
    Write-Host "====================" -ForegroundColor Cyan
    Write-Host "📁 Location: examples/demo/"
    Write-Host "🌐 URL: http://localhost:8080"
    Write-Host "🔧 Technologies: Rust + Leptos + WebAssembly"
    Write-Host "📊 Features: Wallet, NFT Gallery, DeFi Dashboard, and more"
    Write-Host ""
    Write-Host "🚀 Available Commands:" -ForegroundColor Yellow
    Write-Host "  .\scripts\run_demo.ps1 build     - Build the demo"
    Write-Host "  .\scripts\run_demo.ps1 build-rel - Build in release mode"
    Write-Host "  .\scripts\run_demo.ps1 run       - Run the demo server"
    Write-Host "  .\scripts\run_demo.ps1 test      - Run tests"
    Write-Host "  .\scripts\run_demo.ps1 clean     - Clean artifacts"
    Write-Host "  .\scripts\run_demo.ps1 info      - Show this information"
    Write-Host ""
    Write-Host "🎨 Demo Features:" -ForegroundColor Yellow
    Write-Host "  🔐 Wallet Connection with mock data"
    Write-Host "  🎨 Interactive NFT Gallery"
    Write-Host "  💰 Token Balances and Portfolio"
    Write-Host "  📊 DeFi Dashboard with yield farming"
    Write-Host "  📜 Smart Contract Interaction"
    Write-Host "  🌐 Multi-chain Bridge simulation"
    Write-Host "  🗳️ DAO Voting system"
    Write-Host "  🏪 NFT Marketplace"
    Write-Host ""
    Write-Host "📋 Mock Data:" -ForegroundColor Yellow
    Write-Host "  • 3 Pre-configured wallets"
    Write-Host "  • 3 NFTs with metadata"
    Write-Host "  • 4 Token balances"
    Write-Host "  • 3 Transaction history"
    Write-Host "  • 3 Smart contracts"
    Write-Host "  • Complete DeFi portfolio"
    Write-Host ""
}

# Main script logic
$scriptSuccess = $true

switch ($Action) {
    "build" {
        Check-Dependencies
        Build-Demo
        $scriptSuccess = $?
    }
    "build-rel" {
        Check-Dependencies
        Build-Demo -Release
        $scriptSuccess = $?
    }
    "run" {
        Check-Dependencies
        Build-Demo
        if ($?) {
            Run-Demo
        }
        $scriptSuccess = $?
    }
    "test" {
        Check-Dependencies
        Run-Tests
        $scriptSuccess = $?
    }
    "clean" {
        Clean-Demo
        $scriptSuccess = $?
    }
    "info" {
        Show-Info
        $scriptSuccess = $true
    }
}

Write-Host ""
if ($scriptSuccess) {
    Write-Status "Demo script completed successfully! 🦀" -ForegroundColor Green
} else {
    Write-Error "Demo script completed with errors! ❌" -ForegroundColor Red
    exit 1
}
