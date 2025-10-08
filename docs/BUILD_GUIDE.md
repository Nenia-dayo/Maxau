# Maxau Build and Development Guide

## ビルド手順

### 前提条件

- Rust (1.70以降)
- Node.js (18以降)
- Yarn または npm

### 開発モード

#### CLI版
```bash
cd minauCli
cargo run -- path/to/music.mp3
```

#### GUI版
```bash
cd maxauGui
yarn install
yarn tauri dev
```

### プロダクションビルド

#### CLI版
```bash
cd minauCli
cargo build --release
```

実行可能ファイルは `target/release/minau-cli` に生成されます。

#### GUI版
```bash
cd maxauGui
yarn install
yarn build
yarn tauri build
```

#### Windows MSIX パッケージのビルド

Windows 10/11用のMSIXパッケージを作成するには:

```bash
cd maxauGui
yarn tauri build --bundles msix
```

MSIXパッケージは `src-tauri/target/release/bundle/msix/` に生成されます。

**重要な注意事項:**
- MSIXビルドには、Windows SDKのインストールが必要です
- コード署名証明書が必要です（開発用には自己署名証明書を使用可能）
- ミュージックライブラリへのアクセス権限は、初回起動時にユーザーに明示的に要求されます

### MSIXパッケージの特徴

- **ミュージックライブラリアクセス**: `musicLibrary` capability により、ユーザーのミュージックフォルダへの安全なアクセスを提供
- **ファイル関連付け**: MP3, FLAC, OGG, WAV, M4A, AAC, OPUS, WMA, M3U, M3U8 に対応
- **ネットワークアクセス**: URL経由でのストリーミング再生に対応
- **サンドボックス化**: UWPと同様のセキュリティモデル

## 機能

### CLI版の機能
- ローカル音楽ファイルの再生
- URL経由のストリーミング再生（HTTP/HTTPS）
- M3U/M3U8プレイリストのサポート
- アルバムアートの表示（--guiオプション使用時）
- 音量調整
- シーク操作
- 一時停止/再開

### GUI版の機能
- **ミュージックライブラリ管理**: 自動的にミュージックフォルダをスキャン
- **URL再生**: HTTP/HTTPSストリーミング対応
- **再生コントロール**: 再生/一時停止、シーク、音量調整
- **アルバムアート表示**: 埋め込まれたアルバムアートの表示
- **メタデータ表示**: タイトル、アーティスト、アルバム情報
- **プレイリスト表示**: 次に再生される曲の一覧

### 対応フォーマット

- MP3
- FLAC
- OGG Vorbis
- WAV
- M4A/AAC
- OPUS
- WMA
- M3U/M3U8 プレイリスト

## アーキテクチャ

### モジュール構成

```
Maxau/
├── minauCore/       # コア再生エンジン
│   ├── player/      # プレーヤー実装
│   ├── play_url.rs  # URL再生
│   ├── play_music.rs # ローカル再生
│   └── m3u.rs       # プレイリスト処理
├── minauCli/        # CLIフロントエンド
└── maxauGui/        # Tauri GUIフロントエンド
    ├── src/         # React/TypeScript UI
    └── src-tauri/   # Rust バックエンド
```

## トラブルシューティング

### MSIXビルドエラー

**エラー**: "Windows SDK not found"
- **解決策**: Visual Studio Installerから Windows 10 SDK をインストール

**エラー**: "Certificate not found"
- **解決策**: 開発用自己署名証明書を作成:
  ```powershell
  New-SelfSignedCertificate -Type Custom -Subject "CN=Nenia" -KeyUsage DigitalSignature -FriendlyName "Maxau Dev Certificate" -CertStoreLocation "Cert:\CurrentUser\My" -TextExtension @("2.5.29.37={text}1.3.6.1.5.5.7.3.3")
  ```

### ミュージックライブラリが表示されない

- Windows: `C:\Users\<ユーザー名>\Music` を確認
- macOS: `~/Music` を確認
- Linux: `~/Music` を確認
- MSIX版: 初回起動時に権限を許可したか確認

### URL再生が失敗する

- インターネット接続を確認
- URLが有効なHTTP/HTTPSアドレスであることを確認
- ファイアウォール設定を確認

## ライセンス

BSD-3-Clause License

## 貢献者

- Nenia-dayo
- sirasaki-konoha
