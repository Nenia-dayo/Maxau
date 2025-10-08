# Yarn依存関係インストールスクリプト
# Node.jsがインストールされている環境で実行してください

cd maxauGui

echo "📦 Installing dependencies with Yarn..."
yarn install

if [ $? -eq 0 ]; then
    echo "✅ Dependencies installed successfully!"
    echo ""
    echo "次のコマンドでアプリを起動できます:"
    echo "  yarn tauri dev    # 開発モード"
    echo "  yarn tauri build  # プロダクションビルド"
else
    echo "❌ Failed to install dependencies"
    echo ""
    echo "Node.jsとYarnがインストールされているか確認してください:"
    echo "  node --version"
    echo "  yarn --version"
    exit 1
fi
