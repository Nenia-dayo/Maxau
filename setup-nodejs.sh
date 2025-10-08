#!/bin/bash

# Node.js と Yarn セットアップスクリプト
# Ubuntu/Debian用

set -e

echo "======================================"
echo "  Node.js & Yarn Installation"
echo "======================================"
echo ""

# カラー定義
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Node.jsがインストール済みかチェック
if command -v node &> /dev/null; then
    NODE_VERSION=$(node --version)
    echo -e "${GREEN}✓${NC} Node.js is already installed: $NODE_VERSION"
    
    # バージョンチェック（v18以降を推奨）
    MAJOR_VERSION=$(echo $NODE_VERSION | sed 's/v\([0-9]*\).*/\1/')
    if [ "$MAJOR_VERSION" -lt 18 ]; then
        echo -e "${YELLOW}⚠${NC} Node.js $NODE_VERSION is installed but v18+ is recommended"
        echo "Upgrading Node.js..."
    else
        echo -e "${GREEN}✓${NC} Node.js version is compatible"
    fi
else
    echo -e "${YELLOW}→${NC} Installing Node.js v20 (LTS)..."
    
    # NodeSourceリポジトリの追加
    curl -fsSL https://deb.nodesource.com/setup_20.x | sudo -E bash -
    
    # Node.jsのインストール
    sudo apt-get install -y nodejs
    
    if [ $? -eq 0 ]; then
        echo -e "${GREEN}✓${NC} Node.js installed successfully: $(node --version)"
    else
        echo -e "${RED}✗${NC} Failed to install Node.js"
        exit 1
    fi
fi

echo ""

# Yarnがインストール済みかチェック
if command -v yarn &> /dev/null; then
    YARN_VERSION=$(yarn --version)
    echo -e "${GREEN}✓${NC} Yarn is already installed: v$YARN_VERSION"
else
    echo -e "${YELLOW}→${NC} Installing Yarn..."
    
    # Yarnのグローバルインストール
    sudo npm install -g yarn
    
    if [ $? -eq 0 ]; then
        echo -e "${GREEN}✓${NC} Yarn installed successfully: v$(yarn --version)"
    else
        echo -e "${RED}✗${NC} Failed to install Yarn"
        exit 1
    fi
fi

echo ""
echo "======================================"
echo "  Installing Project Dependencies"
echo "======================================"
echo ""

# プロジェクトディレクトリに移動
cd "$(dirname "$0")/maxauGui"

echo -e "${BLUE}→${NC} Running: yarn install"
echo ""

yarn install

if [ $? -eq 0 ]; then
    echo ""
    echo -e "${GREEN}✓${NC} Dependencies installed successfully!"
    echo ""
    echo "======================================"
    echo "  Setup Complete!"
    echo "======================================"
    echo ""
    echo "You can now run:"
    echo ""
    echo -e "  ${GREEN}Development mode:${NC}"
    echo "    cd maxauGui && yarn tauri dev"
    echo ""
    echo -e "  ${GREEN}Production build:${NC}"
    echo "    cd maxauGui && yarn tauri build"
    echo ""
    echo -e "  ${GREEN}MSIX package (Windows):${NC}"
    echo "    cd maxauGui && yarn tauri build --bundles msix"
    echo ""
else
    echo ""
    echo -e "${RED}✗${NC} Failed to install dependencies"
    echo ""
    echo "Troubleshooting:"
    echo "  1. Check your internet connection"
    echo "  2. Clear yarn cache: yarn cache clean"
    echo "  3. Try manual install: cd maxauGui && yarn install --verbose"
    exit 1
fi
