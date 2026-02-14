#!/bin/bash
# Optimized build script for Ethereum Boilerplate Rust Edition
# Script to compile all binaries with optimizations and organize them into ./bin/ folders

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Default values
PROFILE="release"
CLEAN=false
VERBOSE=false

# Display help
show_help() {
    cat << EOF
Ethereum Boilerplate Build Script

Usage:
  ./build.sh [OPTIONS]

Options:
  -p, --profile <profile>  Build profile: debug, release, release-optimized (default: release)
  -c, --clean             Clean build artifacts before compiling
  -v, --verbose           Show verbose build output
  -h, --help              Display this help message

Examples:
  ./build.sh                              # Release build
  ./build.sh -p debug                     # Debug build
  ./build.sh -p release-optimized         # Production build
  ./build.sh --clean -p release           # Clean rebuild
EOF
}

# Parse arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        -p|--profile)
            PROFILE="$2"
            shift 2
            ;;
        -c|--clean)
            CLEAN=true
            shift
            ;;
        -v|--verbose)
            VERBOSE=true
            shift
            ;;
        -h|--help)
            show_help
            exit 0
            ;;
        *)
            echo "Unknown option: $1"
            show_help
            exit 1
            ;;
    esac
done

# Validate profile
case $PROFILE in
    debug|release|release-optimized)
        ;;
    *)
        echo -e "${RED}Invalid profile: $PROFILE${NC}"
        show_help
        exit 1
        ;;
esac

SCRIPT_PATH="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
PROJECT_ROOT="$SCRIPT_PATH"

echo -e "${CYAN}=== Ethereum Boilerplate Build System ===${NC}"
echo -e "${YELLOW}Profile: $PROFILE${NC}"
echo -e "${YELLOW}Working directory: $PROJECT_ROOT${NC}"

# Clean if requested
if [ "$CLEAN" = true ]; then
    echo -e "\n${GREEN}[1/5] Cleaning previous builds...${NC}"
    cargo clean
    echo -e "${GREEN}✓ Clean complete${NC}"
fi

# Build CLI
echo -e "\n${GREEN}[2/5] Building CLI binary...${NC}"
if [ "$VERBOSE" = true ]; then
    cargo build -p ethereum-boilerplate-cli --profile "$PROFILE" --verbose
else
    cargo build -p ethereum-boilerplate-cli --profile "$PROFILE"
fi

echo -e "${GREEN}✓ CLI build complete${NC}"

# Copy CLI binary to bin/cli
PROFILE_DIR=$(if [ "$PROFILE" = "debug" ]; then echo "debug"; else echo "$PROFILE"; fi)
BIN_NAME="ethereum-boilerplate"
SRC_PATH="target/$PROFILE_DIR/$BIN_NAME"
DST_PATH="bin/cli/$BIN_NAME"

if [ -f "$SRC_PATH" ]; then
    mkdir -p "$(dirname "$DST_PATH")"
    cp "$SRC_PATH" "$DST_PATH"
    chmod +x "$DST_PATH"
    echo -e "${GREEN}✓ CLI binary copied to bin/cli/${NC}"
else
    echo -e "${RED}✗ CLI binary not found at $SRC_PATH${NC}"
fi

# Build Server
echo -e "\n${GREEN}[3/5] Building Server binary...${NC}"
if [ "$VERBOSE" = true ]; then
    cargo build -p ethereum-boilerplate-server --profile "$PROFILE" --verbose
else
    cargo build -p ethereum-boilerplate-server --profile "$PROFILE"
fi

echo -e "${GREEN}✓ Server build complete${NC}"

# Copy Server binary to bin/server
BIN_NAME="server"
SRC_PATH="target/$PROFILE_DIR/$BIN_NAME"
DST_PATH="bin/server/$BIN_NAME"

if [ -f "$SRC_PATH" ]; then
    mkdir -p "$(dirname "$DST_PATH")"
    cp "$SRC_PATH" "$DST_PATH"
    chmod +x "$DST_PATH"
    echo -e "${GREEN}✓ Server binary copied to bin/server/${NC}"
else
    echo -e "${RED}✗ Server binary not found at $SRC_PATH${NC}"
fi

# Run tests
echo -e "\n${GREEN}[4/5] Running tests...${NC}"
if [ "$VERBOSE" = true ]; then
    cargo test --lib --verbose
else
    cargo test --lib
fi

echo -e "${GREEN}✓ Tests complete${NC}"

# Display summary
echo -e "\n${GREEN}[5/5] Build Summary${NC}"
echo -e "${GREEN}===================${NC}"

for bin_info in "CLI:bin/cli/ethereum-boilerplate" "Server:bin/server/server"; do
    IFS=':' read -r name path <<< "$bin_info"
    if [ -f "$path" ]; then
        size=$(du -h "$path" | cut -f1)
        echo -e "${GREEN}✓ $name : $path ($size)${NC}"
    else
        echo -e "${RED}✗ $name : Not found${NC}"
    fi
done

echo -e "\n${CYAN}=== Build Complete ===${NC}"
echo -e "${YELLOW}Run binaries:${NC}"
echo "  CLI:    ./bin/cli/ethereum-boilerplate"
echo "  Server: ./bin/server/server"
