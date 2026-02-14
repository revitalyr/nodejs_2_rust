# Ethereum Boilerplate (Rust Edition) - User Guide

## 📋 Overview

Ethereum Boilerplate (Rust Edition) is a comprehensive Web3 development project built with Rust, utilizing modern libraries like Alloy and Leptos. The project includes four main components:

- **CLI** - Command-line interface for project management
- **Server** - Backend server for API and business logic  
- **Frontend (WASM)** - Web interface compiled to WebAssembly
- **Demo** - Demonstration application for quick start

## 🚀 Quick Start

### 1. Build Project

```powershell
# Run build script
.\build.ps1
```

The script will automatically build all components and place binary files in:
- `bin/cli/` - CLI application
- `bin/server/` - Server application  
- `bin/wasm/` - WASM module for frontend

### 2. Run Demo Application

```powershell
# Build and run demo
.\scripts\run_demo.ps1 run

# Build only
.\scripts\run_demo.ps1 build
```

Demo will be available at `http://localhost:8080` (or `http://localhost:8081` if port is occupied).

### 3. Start Local Ethereum Network

For full functionality, you need a local Ethereum node:

```powershell
# Install Hardhat (recommended)
npm install -g hardhat

# Start Hardhat node
npx hardhat node --port 8545
```

### 4. Start Server

```powershell
# Set environment variable
$env:ETHEREUM_RPC_URL="http://localhost:8545"

# Start server
.\bin\server\server.exe
```

Server will be available at `http://localhost:3000` (or another port depending on configuration).

## 📖 Components

### CLI Application

**Purpose**: Project management, smart contract deployment, administrative tasks.

**Launch**:
```powershell
# Show help
.\bin\cli\ethereum-boilerplate.exe --help

# Example commands
.\bin\cli\ethereum-boilerplate.exe deploy
.\bin\cli\ethereum-boilerplate.exe status
```

**Main Functions**:
- Smart contract deployment
- Network status checking
- Configuration management
- Account generation

### Server Application

**Purpose**: Backend server providing API for frontend and external applications.

**Launch**:
```powershell
# With environment variable
$env:ETHEREUM_RPC_URL="http://localhost:8545"
.\bin\server\server.exe

# With command line parameters
.\bin\server\server.exe --rpc-url http://localhost:8545 --port 3000
```

Server will be available at `http://localhost:3000` (or another port depending on configuration).

**Main Functions**:
- REST API for Ethereum interaction
- Blockchain data caching
- Database integration (PostgreSQL)
- Request processing from frontend

### Demo Application

**Purpose**: Demonstration Web3 application using Leptos + WebAssembly for quick start and testing.

**Launch**:
```powershell
# Build and run demo
.\scripts\run_demo.ps1 run

# Build only
.\scripts\run_demo.ps1 build

# Build in release mode
.\scripts\run_demo.ps1 build-rel

# Run tests
.\scripts\run_demo.ps1 test

# Clean artifacts
.\scripts\run_demo.ps1 clean

# Show information
.\scripts\run_demo.ps1 info
```

**Access**: Open `http://localhost:8080` in your browser (auto-opens).

**Main Functions**:
- Interactive wallet management
- Balance and transaction visualization
- Smart contract deployment and interaction
- Network monitoring
- Hot reload during development

### Frontend (WASM)

**Purpose**: Web user interface running in browser via WebAssembly.

**Launch**:
```powershell
# Navigate to WASM directory
cd bin\wasm

# Start local web server
npx http-server . -p 8080
```

**Access**: Open `http://localhost:8080` in your browser.

**Main Functions**:
- Interactive wallet interface
- Balance and transaction display
- Smart contract interaction
- Network status monitoring
- WebAssembly performance

## 🔧 Health Checks

### 1. Build Verification

```powershell
# Run build script
.\build.ps1

# Check file existence
Test-Path .\bin\cli\ethereum-boilerplate.exe
Test-Path .\bin\server\server.exe  
Test-Path .\bin\wasm\ethereum_boilerplate_frontend_bg.wasm
```

### 2. Demo Application Check

```powershell
# Build demo
.\scripts\run_demo.ps1 build

# Check demo files
Test-Path examples\demo\dist
Test-Path examples\demo\index.html

# Run demo and verify
.\scripts\run_demo.ps1 run
# Browser opens with http://localhost:8080
```

### 3. Local Ethereum Network Check

```powershell
# Check if Hardhat node is running
Test-NetConnection -ComputerName localhost -Port 8545

# Alternative via curl
curl http://localhost:8545 -X POST -H "Content-Type: application/json" -d '{"jsonrpc":"2.0","method":"eth_blockNumber","params":[],"id":1}'
```

**Expected Result**: Response with current block number.

### 4. CLI Application Check

```powershell
# Show help
.\bin\cli\ethereum-boilerplate.exe --help

# Check network status
.\bin\cli\ethereum-boilerplate.exe network-status
```

### 5. Server Check

```powershell
# Start server in background
Start-Process -FilePath ".\bin\server\server.exe" -ArgumentList "--port 3000" -WindowStyle Hidden

# Wait for startup
Start-Sleep -Seconds 3

# Check API
curl http://localhost:3000/api/health

# Check specific endpoint
curl http://localhost:3000/api/wallet/0x742d35Cc6634C0532925a3b8D4C9db96C4b4d8b
```

### 6. WASM Frontend Check

```powershell
# Start web server for WASM
cd bin\wasm
npx http-server . -p 8080

# Open in browser
Start-Process http://localhost:8080
```

**In Browser**:
1. Open developer tools (F12)
2. Check console for errors
3. Verify WASM module loads successfully
4. Test interface functionality

## 🛠 Troubleshooting

### Problem: "Firefox can't establish a connection to server at localhost:8545"

**Cause**: Local Ethereum node is not running or running on different port.

**Solution**:
```powershell
# Install and start Hardhat (recommended)
npm install -g hardhat
npx hardhat node --port 8545

# Check connectivity
Test-NetConnection -ComputerName localhost -Port 8545
```

### Problem: "ETHEREUM_RPC_URL must be set"

**Cause**: Server cannot find Ethereum RPC URL environment variable.

**Solution**:
```powershell
# Windows PowerShell
$env:ETHEREUM_RPC_URL="http://localhost:8545"

# Start server
.\bin\server\server.exe

# Or pass parameter
.\bin\server\server.exe --rpc-url http://localhost:8545
```

### Problem: "Demo script completed with errors"

**Cause**: Demo compilation errors or missing dependencies.

**Solution**:
```powershell
# Check dependencies
cargo install trunk wasm-pack

# Rebuild demo
Remove-Item -Recurse -Force examples\demo
.\scripts\run_demo.ps1 build

# Check build logs
.\scripts\run_demo.ps1 build 2>&1 | Select-String "ERROR"
```

### Problem: "could not find root package of target crate"

**Cause**: Demo not included in workspace or incorrect project structure.

**Solution**:
```powershell
# Check workspace members
Get-Content Cargo.toml | Select-String "members"

# Add demo to workspace if missing
# "examples/demo" should be in members list

# Recreate demo structure
Remove-Item -Recurse -Force examples\demo
.\scripts\run_demo.ps1 build
```

### Problem: "WASM module not loading"

**Cause**: CORS issues or incorrect MIME types.

**Solution**:
```powershell
# Use server with CORS support
npx http-server . -p 8080 --cors

# Check MIME types in browser console
# Should be application/wasm for .wasm files
```

### Problem: "no method named `mount_to_body` found"

**Cause**: Incompatible Leptos version or incorrect imports.

**Solution**:
```powershell
# Check Leptos version in workspace
Get-Content Cargo.toml | Select-String "leptos"

# Update imports in main.rs
use leptos::prelude::*;
use leptos::mount::mount_to_body;

# Rebuild
.\scripts\run_demo.ps1 build
```

### Problem: "Compilation errors in main components"

**Cause**: Missing dependencies or incorrect Rust version.

**Solution**:
```powershell
# Update Rust
rustup update

# Check dependencies
cargo check

# Rebuild project
cargo clean
.\build.ps1

# Check dependencies
cargo check
```

## 📊 Monitoring

### Server Logs

```powershell
# Start server with logging
.\bin\server\server.exe --log-level debug

# Or redirect to file
.\bin\server\server.exe > server.log 2>&1
```

### Demo Application Logs

```powershell
# Run demo with verbose logs
cd examples\demo
trunk serve --port 8080 --open

# Check build logs
.\scripts\run_demo.ps1 build --verbose
```

### Performance Metrics

- **CLI**: Command execution time
- **Server**: API response time, request count
- **WASM**: Load time, module size
- **Demo**: Build time, bundle size, page load time

### System Health Check

```powershell
function Test-SystemHealth {
    Write-Host "System Health Check..." -ForegroundColor Green
    
    # Check binary files
    $cliExists = Test-Path ".\bin\cli\ethereum-boilerplate.exe"
    $serverExists = Test-Path ".\bin\server\server.exe"  
    $wasmExists = Test-Path ".\bin\wasm\ethereum_boilerplate_frontend_bg.wasm"
    
    Write-Host "CLI: $(if ($cliExists) { 'OK' } else { 'FAIL' })"
    Write-Host "Server: $(if ($serverExists) { 'OK' } else { 'FAIL' })"
    Write-Host "WASM: $(if ($wasmExists) { 'OK' } else { 'FAIL' })"
    
    # Check demo
    $demoExists = Test-Path "examples\demo\dist"
    Write-Host "Demo: $(if ($demoExists) { 'OK' } else { 'FAIL' })"
    
    # Check network
    try {
        $response = Invoke-WebRequest -Uri "http://localhost:8545" -Method POST -Body '{"jsonrpc":"2.0","method":"eth_blockNumber","params":[],"id":1}' -ContentType "application/json" -TimeoutSec 5
        Write-Host "Ethereum network: OK"
    } catch {
        Write-Host "Ethereum network: FAIL"
    }
    
    # Check server API
    try {
        $response = Invoke-WebRequest -Uri "http://localhost:3000/api/health" -TimeoutSec 5
        Write-Host "Server API: OK"
    } catch {
        Write-Host "Server API: FAIL"
    }
    
    # Check demo server
    try {
        $response = Invoke-WebRequest -Uri "http://localhost:8080" -TimeoutSec 5
        Write-Host "Demo server: OK"
    } catch {
        Write-Host "Demo server: FAIL"
    }
}

# Run health check
Test-SystemHealth
```

## 📚 Additional Resources

- [Alloy Documentation](https://github.com/alloy-rs)
- [Leptos Framework](https://leptos.dev/)
- [WebAssembly in Rust](https://rustwasm.github.io/docs/book/)
- [Ethereum JSON-RPC API](https://ethereum.org/en/developers/docs/apis/json-rpc/)

## 🆘 Support

If you encounter issues:

1. Check error logs
2. Ensure all components are running
3. Verify network connectivity
4. Check documentation for specific libraries
5. Review troubleshooting section above

---

**Version**: 0.1.0  
**Last Updated**: 2026-02-14
