#!/bin/bash

# Maxau Development Quick Start Script
# このスクリプトは開発環境のセットアップを自動化します

set -e

echo "======================================"
echo "  Maxau Development Setup"
echo "======================================"
echo ""

# カラー定義
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# 依存関係チェック
check_command() {
    if command -v $1 &> /dev/null; then
        echo -e "${GREEN}✓${NC} $1 is installed"
        return 0
    else
        echo -e "${RED}✗${NC} $1 is not installed"
        return 1
    fi
}

# 必要なツールのチェック
echo "Checking dependencies..."
echo ""

MISSING_DEPS=0

if ! check_command cargo; then
    echo "  Please install Rust: https://rustup.rs/"
    MISSING_DEPS=1
fi

if ! check_command node; then
    echo "  Please install Node.js: https://nodejs.org/"
    MISSING_DEPS=1
fi

if ! check_command yarn; then
    echo "  Please install Yarn: npm install -g yarn"
    MISSING_DEPS=1
fi

echo ""

if [ $MISSING_DEPS -eq 1 ]; then
    echo -e "${RED}Missing required dependencies. Please install them first.${NC}"
    exit 1
fi

# プロジェクトのビルド
echo "======================================"
echo "  Building Projects"
echo "======================================"
echo ""

# CLI版のビルド
echo -e "${YELLOW}Building CLI version...${NC}"
cd minauCli
cargo build --release
if [ $? -eq 0 ]; then
    echo -e "${GREEN}✓ CLI build successful${NC}"
else
    echo -e "${RED}✗ CLI build failed${NC}"
    exit 1
fi
cd ..

echo ""

# GUI版の準備
echo -e "${YELLOW}Setting up GUI version...${NC}"
cd maxauGui

# 依存関係のインストール
echo "Installing frontend dependencies..."
yarn install
if [ $? -eq 0 ]; then
    echo -e "${GREEN}✓ Dependencies installed${NC}"
else
    echo -e "${RED}✗ Dependency installation failed${NC}"
    exit 1
fi

# フロントエンドのビルド
echo "Building frontend..."
yarn build
if [ $? -eq 0 ]; then
    echo -e "${GREEN}✓ Frontend build successful${NC}"
else
    echo -e "${RED}✗ Frontend build failed${NC}"
    exit 1
fi

cd ..

echo ""
echo "======================================"
echo "  Setup Complete!"
echo "======================================"
echo ""
echo "You can now run:"
echo ""
echo -e "  ${GREEN}CLI version:${NC}"
echo "    ./minauCli/target/release/minau-cli <audio-file>"
echo ""
echo -e "  ${GREEN}GUI version (dev mode):${NC}"
echo "    cd maxauGui && yarn tauri dev"
echo ""
echo -e "  ${GREEN}GUI version (build):${NC}"
echo "    cd maxauGui && yarn tauri build"
echo ""
echo "For more information, see BUILD_GUIDE.md"
