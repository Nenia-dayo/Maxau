# 🎵 Maxau - 実行方法まとめ

## 🔍 現在の状況

**環境**: WSL2 (Windows Subsystem for Linux 2)  
**問題**: ALSAオーディオデバイスがない → 音が出ない

---

## ✅ 解決策: 3つの方法

### 方法1: Windows環境でビルド・実行（最推奨）

WSL2ではなく、**Windowsネイティブ環境**でビルドしてください。

#### ステップ

1. **Windowsに必要なツールをインストール**
   - [Rust for Windows](https://rustup.rs/) をダウンロード・インストール
   - [Node.js](https://nodejs.org/) (v20 LTS) をインストール
   - Visual Studio Build Tools または Visual Studio 2022をインストール

2. **PowerShellまたはコマンドプロンプトで実行**
   ```powershell
   # リポジトリに移動
   cd C:\Users\<username>\repos\Maxau\maxauGui
   
   # 依存関係インストール
   yarn install
   
   # 開発モードで実行
   yarn tauri dev
   
   # MSIXパッケージビルド
   yarn tauri build --bundles msix
   ```

3. **実行**
   - 開発モード: GUIウィンドウが自動起動
   - ビルド後: `src-tauri\target\release\bundle\msix\Maxau_0.1.0_x64.msix`をダブルクリック

---

### 方法2: Linux AppImageをビルド（WSL2で実行は不可）

WSL2でビルドして、**ネイティブLinux環境**（別のマシンやVM）で実行:

```bash
cd /home/nenia/repos/Maxau/maxauGui
yarn tauri build --bundles appimage

# 生成物:
# src-tauri/target/release/bundle/appimage/maxau_0.1.0_amd64.AppImage
```

このAppImageをUSBメモリなどでネイティブLinux環境に移動して実行。

---

### 方法3: WSL2でダミーオーディオ + 開発モード

音は出ませんが、UI開発には使えます。

#### セットアップ

```bash
# ALSAダミーモジュールをロード
sudo modprobe snd-dummy

# ALSA設定ファイル作成
cat > ~/.asoundrc << 'EOF'
pcm.!default {
    type plug
    slave.pcm "null"
}
EOF

# Tauri開発モード起動
cd /home/nenia/repos/Maxau/maxauGui
yarn tauri dev
```

**注意**: 音は出ませんが、GUIの動作確認は可能です。

---

## 🚀 推奨フロー

### 開発時

```
WSL2: コード編集・Rustビルドチェック
  ↓
Windows: 実行・テスト
```

**具体的な手順**:

1. WSL2でコード編集
   ```bash
   code /home/nenia/repos/Maxau
   ```

2. WSL2でRustチェック（高速）
   ```bash
   cd /home/nenia/repos/Maxau
   cargo check --workspace
   ```

3. Windowsで実行・テスト
   ```powershell
   cd \\wsl$\Ubuntu\home\nenia\repos\Maxau\maxauGui
   yarn tauri dev
   ```

### リリース時

```bash
# Windows PowerShellで実行
cd C:\Users\<username>\repos\Maxau\maxauGui
yarn tauri build --bundles msix

# 生成された.msixをMicrosoft Storeに提出
```

---

## 📊 各方法の比較

| 方法 | オーディオ | GUI表示 | ビルド速度 | 推奨度 |
|------|----------|--------|-----------|--------|
| **Windowsネイティブ** | ✅ 完全動作 | ✅ 完全 | 🚀 高速 | ⭐⭐⭐⭐⭐ |
| **Linux AppImage** | ✅ 別マシンで | ✅ 完全 | 🐢 普通 | ⭐⭐⭐ |
| **WSL2 + ダミー** | ❌ なし | ✅ 完全 | 🚀 高速 | ⭐⭐ (開発のみ) |

---

## 🔧 Windows開発環境セットアップ詳細

### 1. Rustインストール

1. https://rustup.rs/ にアクセス
2. `rustup-init.exe`をダウンロード・実行
3. デフォルト設定でインストール
4. PowerShellを再起動

確認:
```powershell
rustc --version
# rustc 1.90.0 (...)
```

### 2. Node.js & Yarnインストール

1. https://nodejs.org/ から "LTS" 版をダウンロード
2. インストール（デフォルト設定）
3. PowerShellで:
   ```powershell
   npm install -g yarn
   ```

確認:
```powershell
node --version  # v20.x.x
yarn --version  # 1.22.x
```

### 3. Visual Studio Build Tools

最も簡単な方法:
```powershell
# wingetを使用（Windows 10/11）
winget install Microsoft.VisualStudio.2022.BuildTools

# または手動: https://visualstudio.microsoft.com/downloads/
# "Build Tools for Visual Studio 2022"をダウンロード
# "Desktop development with C++"をチェックしてインストール
```

### 4. Tauriの前提条件

```powershell
# WebView2（通常は既にインストール済み）
winget install Microsoft.EdgeWebView2Runtime
```

### 5. プロジェクトセットアップ

```powershell
# GitでWSLのリポジトリにアクセス
cd \\wsl$\Ubuntu\home\nenia\repos\Maxau\maxauGui

# または、Windowsにクローン
git clone https://github.com/Nenia-dayo/Maxau.git C:\Users\<username>\repos\Maxau
cd C:\Users\<username>\repos\Maxau\maxauGui

# 依存関係インストール
yarn install

# 開発モード起動
yarn tauri dev
```

---

## 🐛 トラブルシューティング

### エラー: "link.exe not found"

**原因**: Visual Studio Build Toolsが未インストール  
**解決**: 上記"3. Visual Studio Build Tools"を実行

### エラー: "OpenSSL not found"

**解決** (Windows):
```powershell
# vcpkgを使用
git clone https://github.com/microsoft/vcpkg.git
cd vcpkg
.\bootstrap-vcpkg.bat
.\vcpkg install openssl:x64-windows-static
```

または:
```powershell
$env:OPENSSL_DIR = "C:\Program Files\OpenSSL-Win64"
```

### WSL2のファイルにWindowsからアクセスできない

**確認**:
```powershell
# WSL2が起動していることを確認
wsl --list --running

# アクセス
cd \\wsl$\Ubuntu\home\nenia\repos\Maxau
```

---

## 📝 まとめ

### 🎯 最適な開発フロー

```
1. コード編集: WSL2のVS Codeで快適に編集
2. Rustチェック: WSL2で高速チェック（cargo check）
3. 実行・テスト: Windowsで実行（音が出る！）
4. リリース: WindowsでMSIXビルド
```

### 今すぐ試す

**最も簡単**:
1. WindowsにRust + Node.jsをインストール
2. PowerShellで:
   ```powershell
   cd \\wsl$\Ubuntu\home\nenia\repos\Maxau\maxauGui
   yarn tauri dev
   ```

音楽が流れます！🎵

---

**作成日**: 2025年10月7日  
**対象**: WSL2開発者  
**ステータス**: ✅ 解決策完全版
