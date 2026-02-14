# Optimized build script for Ethereum Boilerplate Rust Edition
# Script to compile all binaries with optimizations and organize them into ./bin/ folders

param(
    [ValidateSet('debug', 'release', 'release-optimized')]
    [string]$Profile = 'release',
    
    [switch]$Clean,
    [switch]$Verbose,
    [switch]$Help
)

# Display help
if ($Help) {
    @"
Ethereum Boilerplate Build Script

Usage:
  .\build.ps1 [Options]

Options:
  -Profile <profile>
    Build profile: debug, release, release-optimized (default: release)
  
  -Clean
    Clean build artifacts before compiling
  
  -Verbose
    Show verbose build output
  
  -Help
    Display this help message

Examples:
  .\build.ps1                              # Release build
  .\build.ps1 -Profile debug               # Debug build
  .\build.ps1 -Profile release-optimized   # Production build
  .\build.ps1 -Clean -Profile release      # Clean rebuild
"@
    exit 0
}

$ErrorActionPreference = 'Stop'
$ScriptPath = Split-Path -Parent $MyInvocation.MyCommand.Path
$ProjectRoot = $ScriptPath

Write-Host "=== Ethereum Boilerplate Build System ===" -ForegroundColor Cyan
Write-Host "Profile: $Profile" -ForegroundColor Yellow
Write-Host "Working directory: $ProjectRoot" -ForegroundColor Yellow

# Clean if requested
if ($Clean) {
    Write-Host "`n[1/5] Cleaning previous builds..." -ForegroundColor Green
    cargo clean
    if ($LASTEXITCODE -ne 0) {
        Write-Host "✗ Clean failed" -ForegroundColor Red
        exit 1
    }
    Write-Host "✓ Clean complete" -ForegroundColor Green
}

# Build CLI
Write-Host "`n[2/5] Building CLI binary..." -ForegroundColor Green
if ($Verbose) {
    cargo build -p ethereum-boilerplate-cli --profile $Profile --verbose
} else {
    cargo build -p ethereum-boilerplate-cli --profile $Profile
}
if ($LASTEXITCODE -ne 0) {
    Write-Host "✗ CLI build failed" -ForegroundColor Red
    exit 1
}

Write-Host "✓ CLI build complete" -ForegroundColor Green

# Copy CLI binary to bin/cli
$profileDir = if ($Profile -eq 'debug') { 'debug' } else { $Profile }
$binName = 'ethereum-boilerplate.exe'
$srcPath = "target\$profileDir\$binName"
$dstPath = "bin\cli\$binName"

if (Test-Path $srcPath) {
    Copy-Item $srcPath $dstPath -Force
    Write-Host "✓ CLI binary copied to bin/cli/" -ForegroundColor Green
} else {
    Write-Host "✗ CLI binary not found at $srcPath" -ForegroundColor Red
}

# Build Server
Write-Host "`n[3/5] Building Server binary..." -ForegroundColor Green
if ($Verbose) {
    cargo build -p ethereum-boilerplate-server --profile $Profile --verbose
} else {
    cargo build -p ethereum-boilerplate-server --profile $Profile
}
if ($LASTEXITCODE -ne 0) {
    Write-Host "✗ Server build failed" -ForegroundColor Red
    exit 1
}

Write-Host "✓ Server build complete" -ForegroundColor Green

# Copy Server binary to bin/server
$profileDir = if ($Profile -eq 'debug') { 'debug' } else { $Profile }
$binName = 'server.exe'
$srcPath = "target\$profileDir\$binName"
$dstPath = "bin\server\$binName"

if (Test-Path $srcPath) {
    Copy-Item $srcPath $dstPath -Force
    Write-Host "✓ Server binary copied to bin/server/" -ForegroundColor Green
} else {
    Write-Host "✗ Server binary not found at $srcPath" -ForegroundColor Red
}


# Run tests
Write-Host "`n[4/5] Running tests..." -ForegroundColor Green
if ($Verbose) {
    cargo test --lib --verbose
} else {
    cargo test --lib
}
if ($LASTEXITCODE -ne 0) {
    Write-Host "✗ Tests failed" -ForegroundColor Red
    exit 1
}

Write-Host "✓ Tests complete" -ForegroundColor Green

# Build Frontend WASM
Write-Host "`n[5/6] Building Frontend WASM..." -ForegroundColor Green
if ($Verbose) {
    wasm-pack build --target web --dev --out-dir ../../bin/wasm --out-name ethereum_boilerplate_frontend crates/frontend
} else {
    wasm-pack build --target web --dev --out-dir ../../bin/wasm --out-name ethereum_boilerplate_frontend crates/frontend
}
if ($LASTEXITCODE -ne 0) {
    Write-Host "✗ Frontend WASM build failed" -ForegroundColor Red
    exit 1
}
Write-Host "✓ Frontend WASM build complete" -ForegroundColor Green

# Display summary
Write-Host "`n[6/6] Build Summary" -ForegroundColor Green
Write-Host "===================" -ForegroundColor Green

$profileDir = if ($Profile -eq 'debug') { 'debug' } else { $Profile }
$cliPath = "bin\cli\ethereum-boilerplate.exe"
$serverPath = "bin\server\server.exe"
$wasmPath = "bin\wasm\ethereum_boilerplate_frontend_bg.wasm"

@(
    @("CLI", $cliPath),
    @("Server", $serverPath),
    @("WASM", $wasmPath)
) | ForEach-Object {
    $name, $path = $_
    if (Test-Path $path) {
        $size = (Get-Item $path).Length / 1MB
        Write-Host "✓ $name : bin/$($name.ToLower())/ ($([Math]::Round($size, 2)) MB)" -ForegroundColor Green
    } else {
        Write-Host "✗ $name : Not found" -ForegroundColor Red
    }
}

Write-Host "`n=== Build Complete ===" -ForegroundColor Cyan
Write-Host "Run binaries:" -ForegroundColor Yellow
Write-Host "  CLI:    .\bin\cli\ethereum-boilerplate.exe"
Write-Host "  Server: .\bin\server\server.exe"
Write-Host "  WASM:    .\bin\wasm\ethereum_boilerplate_frontend_bg.wasm"
