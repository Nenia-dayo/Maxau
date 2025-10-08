# セキュリティと安定性ガイド

## Rust 2024 Edition の使用

このプロジェクトは最新のRust 2024 Editionを使用しています。これにより以下のセキュリティと安定性機能が有効になります:

### 主要な改善点

1. **RPIT lifetime capture rules** - より安全なライフタイム推論
2. **unsafe_op_in_unsafe_fn warning** - unsafe操作の明示的な要求
3. **Disallow references to static mut** - 静的ミュータブル参照の禁止
4. **Never type fallback change** - より予測可能な型推論

### コンパイラフラグ

プロダクションビルドでは以下の最適化とセキュリティフラグが有効です:

```toml
[profile.release]
opt-level = 3          # 最大最適化
lto = true             # Link Time Optimization
codegen-units = 1      # 単一コード生成ユニット（最適化）
strip = true           # デバッグシンボル削除
panic = "abort"        # パニック時即座に中断（スタック展開なし）
```

## Win32 App Isolation 対応

### 必要な Windows バージョン

- **最小**: Windows 11, version 24H2 (build 26100)
- **推奨**: 最新の Windows 11

### 有効化されたCapabilities

#### UWP Capabilities
- `musicLibrary` - ミュージックライブラリへのプログラマティックアクセス
- `internetClient` - インターネット接続（ストリーミング用）
- `internetClientServer` - ネットワークサーバー機能
- `runFullTrust` - 完全信頼での実行

#### Win32 Isolation Capabilities
- `isolatedWin32-promptForAccess` - ファイルアクセスのプロンプト表示
- `isolatedWin32-userProfileMinimal` - 最小限のユーザープロファイルアクセス

### セキュリティ境界

Win32 App Isolation により、アプリは以下のサンドボックス環境で実行されます:

1. **ファイルシステムアクセス制限**
   - ミュージックライブラリのみアクセス可能
   - その他のフォルダはユーザーの明示的な許可が必要

2. **ネットワークアクセス制限**
   - 宣言されたCapabilitiesのみ許可
   - 送信接続のみ（HTTP/HTTPS）

3. **レジストリアクセス制限**
   - 読み取り専用アクセス
   - アプリ専用キーのみ書き込み可能

## セキュリティベストプラクティス

### 1. 入力の検証

```rust
// URL検証の例
pub fn play_url(url: String, ...) -> Result<(), String> {
    let parsed_url = Url::parse(&url)
        .map_err(|e| format!("Invalid URL: {}", e))?;
    
    if parsed_url.scheme() != "http" && parsed_url.scheme() != "https" {
        return Err("Only HTTP and HTTPS URLs are supported".to_string());
    }
    
    // ...
}
```

### 2. エラーハンドリング

すべての公開APIは`Result`型を返し、適切なエラーメッセージを提供します。

```rust
#[tauri::command]
pub fn some_command() -> Result<Data, String> {
    // エラーを適切に伝搬
    let result = risky_operation()
        .map_err(|e| format!("Operation failed: {}", e))?;
    Ok(result)
}
```

### 3. メモリ安全性

- `unsafe`ブロックの最小化
- すべての`unsafe`操作にコメントを付与
- Rust 2024の警告に従う

## 依存関係の管理

### セキュリティ監査

定期的に依存関係の脆弱性をチェック:

```bash
# cargo-auditのインストール
cargo install cargo-audit

# 監査実行
cargo audit

# 依存関係の更新
cargo update
```

### 依存関係の最小化

必要な機能のみをインポート:

```toml
[dependencies]
symphonia = { version = "0.5", features = ["all"] }  # 必要に応じて機能を限定
```

## MSIX パッケージのセキュリティ

### コード署名

プロダクション環境では必ずコード署名を行ってください:

```powershell
# 証明書の作成（開発用）
New-SelfSignedCertificate -Type Custom -Subject "CN=Nenia" `
    -KeyUsage DigitalSignature -FriendlyName "Maxau Dev Certificate" `
    -CertStoreLocation "Cert:\CurrentUser\My" `
    -TextExtension @("2.5.29.37={text}1.3.6.1.5.5.7.3.3")

# MSIXパッケージの署名
SignTool sign /fd SHA256 /a /f MyCert.pfx /p MyPassword Maxau.msix
```

### AppxManifest セキュリティ設定

現在の設定:

```xml
<Dependencies>
  <TargetDeviceFamily Name="Windows.Desktop" 
                      MinVersion="10.0.26100.0" 
                      MaxVersionTested="10.0.26100.0" />
</Dependencies>

<Capabilities>
  <rescap:Capability Name="runFullTrust" />
  <uap:Capability Name="musicLibrary" />
  <Capability Name="internetClient" />
  <Capability Name="internetClientServer" />
  <rescap:Capability Name="isolatedWin32-promptForAccess" />
  <rescap:Capability Name="isolatedWin32-userProfileMinimal" />
</Capabilities>
```

## セキュリティチェックリスト

リリース前に確認:

- [ ] Rust 2024 Edition使用確認
- [ ] すべてのコンパイラ警告を解決
- [ ] `cargo audit`でセキュリティ監査
- [ ] 依存関係を最新版に更新
- [ ] コード署名証明書の有効性確認
- [ ] AppxManifestの最小権限原則確認
- [ ] すべての入力検証が実装されている
- [ ] エラーハンドリングが適切
- [ ] `unsafe`コードのレビュー完了

## 脆弱性報告

セキュリティ問題を発見した場合:

1. **公開しない** - GitHubのIssuesには投稿しないでください
2. メール送信: security@example.com（実際のメールアドレスに置き換え）
3. 詳細な再現手順を含める
4. 24-48時間以内に返信します

## 参考資料

- [Rust Security Guidelines](https://anssi-fr.github.io/rust-guide/)
- [Win32 App Isolation Documentation](https://learn.microsoft.com/en-us/windows/security/application-security/application-isolation/win32-app-isolation/overview)
- [OWASP Secure Coding Practices](https://owasp.org/www-project-secure-coding-practices-quick-reference-guide/)
- [Tauri Security Best Practices](https://tauri.app/v1/references/architecture/security/)

---

**最終更新**: 2025年10月7日  
**対象バージョン**: Maxau v0.1.0  
**セキュリティレベル**: Production Ready
