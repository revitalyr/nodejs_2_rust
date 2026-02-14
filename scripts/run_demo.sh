#!/bin/bash

# Ethereum Boilerplate Demo Runner
# This script sets up and runs the demo application with mock data

set -e

echo "🦀 Ethereum Boilerplate Demo Runner"
echo "=================================="

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

print_step() {
    echo -e "${BLUE}[STEP]${NC} $1"
}

# Check if we're in the right directory
if [ ! -f "Cargo.toml" ]; then
    print_error "Please run this script from the project root directory"
    exit 1
fi

# Check dependencies
check_dependencies() {
    print_step "Checking dependencies..."
    
    # Check if Rust is installed
    if ! command -v cargo &> /dev/null; then
        print_error "Rust/Cargo is not installed. Please install Rust first."
        exit 1
    fi
    
    # Check if Trunk is installed
    if ! command -v trunk &> /dev/null; then
        print_warning "Trunk is not installed. Installing..."
        cargo install trunk
    fi
    
    # Check if wasm-pack is installed
    if ! command -v wasm-pack &> /dev/null; then
        print_warning "wasm-pack is not installed. Installing..."
        cargo install wasm-pack
    fi
    
    print_status "All dependencies are available"
}

# Build the demo
build_demo() {
    print_step "Building demo application..."
    
    cd examples/demo
    
    # Build the demo
    if [ "$1" = "--release" ]; then
        print_status "Building in release mode..."
        trunk build --release
    else
        print_status "Building in debug mode..."
        trunk build
    fi
    
    cd ../..
    
    print_status "Demo built successfully"
}

# Run the demo server
run_demo() {
    print_step "Starting demo server..."
    
    cd examples/demo
    
    # Check if port is already in use
    if lsof -Pi :8080 -sTCP:LISTEN -t >/dev/null 2>&1; then
        print_warning "Port 8080 is already in use. Trying port 8081..."
        if lsof -Pi :8081 -sTCP:LISTEN -t >/dev/null 2>&1; then
            print_error "Ports 8080 and 8081 are both in use"
            exit 1
        fi
        trunk serve --port 8081 --open
    else
        trunk serve --port 8080 --open
    fi
}

# Run tests
run_tests() {
    print_step "Running demo tests..."
    
    cd examples/demo
    
    # Run WebAssembly tests
    wasm-pack test --headless --firefox
    
    cd ../..
    
    print_status "All tests passed"
}

# Clean build artifacts
clean_demo() {
    print_step "Cleaning demo artifacts..."
    
    cd examples/demo
    
    # Clean Trunk artifacts
    if [ -d "dist" ]; then
        rm -rf dist
        print_status "Removed dist directory"
    fi
    
    # Clean Cargo artifacts
    cargo clean
    
    cd ../..
    
    print_status "Demo cleaned successfully"
}

# Show demo information
show_info() {
    echo ""
    echo "🎯 Demo Information:"
    echo "===================="
    echo "📁 Location: examples/demo/"
    echo "🌐 URL: http://localhost:8080"
    echo "🔧 Technologies: Rust + Leptos + WebAssembly"
    echo "📊 Features: Wallet, NFT Gallery, DeFi Dashboard, and more"
    echo ""
    echo "🚀 Available Commands:"
    echo "  ./scripts/run_demo.sh build     - Build the demo"
    echo "  ./scripts/run_demo.sh build-rel - Build in release mode"
    echo "  ./scripts/run_demo.sh run       - Run the demo server"
    echo "  ./scripts/run_demo.sh test      - Run tests"
    echo "  ./scripts/run_demo.sh clean     - Clean artifacts"
    echo "  ./scripts/run_demo.sh info      - Show this information"
    echo ""
    echo "🎨 Demo Features:"
    echo "  🔐 Wallet Connection with mock data"
    echo "  🎨 Interactive NFT Gallery"
    echo "  💰 Token Balances and Portfolio"
    echo "  📊 DeFi Dashboard with yield farming"
    echo "  📜 Smart Contract Interaction"
    echo "  🌐 Multi-chain Bridge simulation"
    echo "  🗳️ DAO Voting system"
    echo "  🏪 NFT Marketplace"
    echo ""
    echo "📋 Mock Data:"
    echo "  • 3 Pre-configured wallets"
    echo "  • 3 NFTs with metadata"
    echo "  • 4 Token balances"
    echo "  • 3 Transaction history"
    echo "  • 3 Smart contracts"
    echo "  • Complete DeFi portfolio"
    echo ""
}

# Main script logic
case "${1:-info}" in
    "build")
        check_dependencies
        build_demo
        ;;
    "build-rel")
        check_dependencies
        build_demo --release
        ;;
    "run")
        check_dependencies
        build_demo
        run_demo
        ;;
    "test")
        check_dependencies
        run_tests
        ;;
    "clean")
        clean_demo
        ;;
    "info"|*)
        show_info
        ;;
esac

echo ""
print_status "Demo script completed successfully! 🦀"
