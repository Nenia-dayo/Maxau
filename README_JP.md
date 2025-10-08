# Maxau 🎵

**Focus Flow Music Experience** - モダンな音楽プレーヤー

Maxauは、CLIとGUIの両方を備えた、シンプルで強力な音楽プレーヤーです。ローカルファイル再生とURL経由のストリーミング再生の両方に対応しています。

![Version](https://img.shields.io/badge/version-0.1.0-blue)
![License](https://img.shields.io/badge/license-BSD--3--Clause-green)

## ✨ 特徴

### 🎼 対応フォーマット
- **オーディオ**: MP3, FLAC, OGG Vorbis, WAV, M4A/AAC, OPUS, WMA
- **プレイリスト**: M3U, M3U8
- **ストリーミング**: HTTP/HTTPS URL再生

### 🖥️ デュアルインターフェース

#### CLI版
- 軽量で高速
- ターミナルから直接操作
- アルバムアート表示（GUIモード）
- キーボードショートカット対応

#### GUI版
- モダンなReact + Tauriベースのインターフェース
- 自動ミュージックライブラリスキャン
- **URL再生機能**: HTTPSストリーミング対応
- リアルタイム再生コントロール
- アルバムアート表示
- 次に再生される曲のプレビュー

### 🔒 セキュリティ

- **MSIX対応**: Windows 10/11のサンドボックス環境で安全に実行
- **明示的な権限管理**: ミュージックライブラリへのアクセスはユーザーの許可が必要
- **セキュアなストリーミング**: HTTPS対応

## 🚀 クイックスタート

### インストール

```bash
# リポジトリをクローン
git clone https://github.com/Nenia-dayo/Maxau.git
cd Maxau

# CLI版をビルド
cd minauCli
cargo build --release

# GUI版をビルド
cd ../maxauGui
yarn install
yarn tauri build
```

詳細なビルド手順は [BUILD_GUIDE.md](BUILD_GUIDE.md) を参照してください。

### 使い方

#### CLI版

```bash
# 単一ファイルを再生
minau-cli song.mp3

# 音量を指定して再生（1-100）
minau-cli --volume 80 song.mp3

# アルバムアートをGUIで表示
minau-cli --gui song.flac

# URLから再生
minau-cli https://example.com/stream.mp3

# M3Uプレイリストを再生
minau-cli playlist.m3u

# 複数ファイルを順番に再生
minau-cli song1.mp3 song2.flac song3.ogg
```

#### GUI版

```bash
# 開発モードで起動
cd maxauGui
yarn tauri dev

# ビルド済みアプリを起動
./target/release/maxaugui
```

### キーボードショートカット（CLI版）

- `Space`: 一時停止/再開
- `←/→`: 10秒シーク
- `↑/↓`: 音量調整
- `q`: 終了

## 🏗️ アーキテクチャ

```
Maxau
├── minauCore       # 🎵 コア再生エンジン（Rust）
│   ├── player/     #    - デコード、再生、リサンプリング
│   ├── play_url    #    - HTTPストリーミング
│   ├── play_music  #    - ローカルファイル再生
│   └── m3u         #    - プレイリスト処理
│
├── minauCli        # 💻 CLIフロントエンド
│   └── main.rs     #    - コマンドライン引数処理
│
└── maxauGui        # 🖼️ GUIフロントエンド（Tauri）
    ├── src/        #    - React + TypeScript UI
    └── src-tauri/  #    - Rustバックエンド
```

### 主要な機能実装

#### URL再生機能
- **minauCore/play_url.rs**: HTTPストリーミングエンジン
- **maxauGui/src-tauri/commands.rs**: `play_url` コマンド
- **maxauGui/src/components/UrlPlayback.tsx**: URL入力UI
- **maxauGui/src/lib/tauriApi.ts**: フロントエンドAPIラッパー

#### ライブラリスキャン
- **commands.rs**: `scan_music_folder()` - 再帰的にミュージックフォルダをスキャン
- 対応拡張子を自動検出
- メタデータを抽出して表示

## 🔧 技術スタック

### Backend
- **Rust**: 高速で安全な実行環境
- **Symphonia**: オーディオデコーディング
- **CPAL**: クロスプラットフォームオーディオ出力
- **Rubato**: 高品質リサンプリング
- **Hyper**: HTTPクライアント

### Frontend (GUI)
- **Tauri 2.0**: 軽量なデスクトップアプリフレームワーク
- **React**: モダンなUIライブラリ
- **TypeScript**: 型安全な開発
- **Zustand**: 状態管理
- **Tailwind CSS**: ユーティリティファーストCSS

## 📦 ディストリビューション

### Windows
- **MSIX**: Windows 10/11のネイティブパッケージ形式
  - Microsoft Storeに対応
  - 自動更新サポート
  - サンドボックス化されたセキュリティ
  
- **NSIS**: 従来のインストーラー

### macOS
- **DMG**: ドラッグ&ドロップインストール
- **App Bundle**: スタンドアロン実行

### Linux
- **AppImage**: 単一実行可能ファイル
- **Deb**: Debian/Ubuntu用パッケージ

## 🐛 既知の問題と制限

### 修正済み
- ✅ GUI版でのURL再生機能を実装
- ✅ エラーハンドリングの改善
- ✅ MSIX対応の権限設定
- ✅ Rust edition 2024 → 2021への修正

### 計画中の機能
- [ ] プレイリスト保存機能
- [ ] イコライザー
- [ ] クロスフェード
- [ ] ラジオストリーム対応
- [ ] プラグインシステム
- [ ] 歌詞表示

## 🤝 貢献

プルリクエストを歓迎します！

1. このリポジトリをフォーク
2. 機能ブランチを作成 (`git checkout -b feature/AmazingFeature`)
3. 変更をコミット (`git commit -m 'Add some AmazingFeature'`)
4. ブランチにプッシュ (`git push origin feature/AmazingFeature`)
5. プルリクエストを開く

## 📄 ライセンス

BSD-3-Clause License - 詳細は [LICENSE](LICENSE) を参照

## 👥 作者

- **Nenia-dayo** - [GitHub](https://github.com/Nenia-dayo)
- **sirasaki-konoha** - [GitHub](https://github.com/sirasaki-konoha)

## 🙏 謝辞

- [Symphonia](https://github.com/pdeljanov/Symphonia) - 優れたRustオーディオライブラリ
- [Tauri](https://tauri.app/) - 軽量なデスクトップアプリフレームワーク
- すべてのコントリビューターの皆様

---

**注意**: このプロジェクトは開発中です。フィードバックやバグレポートをお待ちしています！
