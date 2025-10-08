# 🔊 WSL2でのオーディオ再生問題と解決策

## 🔍 問題の原因

Maxauは**WSL2（Windows Subsystem for Linux 2）**環境で実行されています。
WSL2では、Linuxカーネルが仮想化されており、**ALSAオーディオデバイスへの直接アクセスができません**。

### エラーメッセージ
```
ALSA lib confmisc.c:855:(parse_card) cannot find card '0'
Error: The requested device is no longer available.
```

これは、CPALライブラリ（Rustのオーディオ出力ライブラリ）がALSAデバイスを見つけられないことを示しています。

---

## ✅ 解決策

### 🎯 推奨: Windows版をビルドして実行

WSL2で開発し、Windows側で実行するのが最も確実です。

#### オプション1: Windows用MSIXパッケージをビルド
```bash
# WSL2内で実行
cd /home/nenia/repos/Maxau/maxauGui
yarn tauri build --bundles msix

# ビルド完了後、生成された.msixファイルをWindowsで実行
# 場所: src-tauri/target/release/bundle/msix/Maxau_0.1.0_x64.msix
```

Windowsエクスプローラーで以下にアクセス:
```
\\wsl$\Ubuntu\home\nenia\repos\Maxau\maxauGui\src-tauri\target\release\bundle\msix\
```

#### オプション2: Windowsの.exeを直接ビルド
```bash
cd /home/nenia/repos/Maxau/maxauGui
yarn tauri build --bundles msi

# 生成されたインストーラー:
# src-tauri/target/release/bundle/msi/Maxau_0.1.0_x64_en-US.msi
```

---

### 🔧 代替案: WSL2でオーディオを有効化

#### 方法1: PulseAudio経由でWindowsオーディオにアクセス

**手順**:

1. **Windows側でPulseAudioサーバーをインストール**
   - [PulseAudio for Windows](https://www.freedesktop.org/wiki/Software/PulseAudio/Ports/Windows/Support/)
   - または [vcxsrv](https://sourceforge.net/projects/vcxsrv/) (PulseAudio付き)

2. **WSL2でPulseAudioクライアントをインストール**
   ```bash
   sudo apt update
   sudo apt install pulseaudio pulseaudio-utils
   ```

3. **環境変数を設定**
   ```bash
   # ~/.bashrcに追加
   export PULSE_SERVER=tcp:$(grep nameserver /etc/resolv.conf | awk '{print $2}'):4713
   
   # 適用
   source ~/.bashrc
   ```

4. **動作確認**
   ```bash
   pactl info
   # "Server Name: PulseAudio"と表示されればOK
   ```

#### 方法2: WSLg（Windows 11 22H2以降）

Windows 11の最新版では、WSLgによってオーディオサポートが改善されています。

**確認**:
```bash
# WSLgが有効か確認
echo $WAYLAND_DISPLAY
# "wayland-0"などが表示されればWSLg有効
```

**PipeWireを使用**:
```bash
sudo apt install pipewire pipewire-audio-client-libraries
systemctl --user enable pipewire pipewire-pulse
systemctl --user start pipewire pipewire-pulse
```

---

### 🐛 デバッグ用: ダミーオーディオデバイス

開発中でオーディオが必要ない場合、ダミーデバイスを作成:

```bash
# ALSAダミーモジュールをロード
sudo modprobe snd-dummy

# 確認
aplay -l
# "card 0: Dummy"と表示されればOK
```

**~/.asoundrcを作成**:
```bash
cat > ~/.asoundrc << 'EOF'
pcm.!default {
    type plug
    slave.pcm "null"
}

ctl.!default {
    type hw
    card 0
}
EOF
```

これで音は出ませんが、エラーは回避できます。

---

### 🚀 最も簡単な方法: ネイティブLinux環境

物理的なLinuxマシンやLinux VMでは、オーディオは通常通り動作します。

---

## 📊 各方法の比較

| 方法 | 難易度 | オーディオ出力 | 推奨度 |
|------|--------|---------------|--------|
| **Windows版ビルド** | ⭐ 簡単 | ✅ 完全動作 | ⭐⭐⭐⭐⭐ |
| **PulseAudio経由** | ⭐⭐⭐ 中級 | ✅ 動作（遅延あり） | ⭐⭐⭐ |
| **WSLg + PipeWire** | ⭐⭐ 中級 | ✅ 動作 | ⭐⭐⭐⭐ |
| **ダミーデバイス** | ⭐ 簡単 | ❌ 音なし | ⭐⭐ (開発のみ) |
| **ネイティブLinux** | ⭐ 簡単 | ✅ 完全動作 | ⭐⭐⭐⭐⭐ |

---

## 🎯 今すぐ試せる方法

### 1. Windows版をビルド（推奨）

```bash
cd /home/nenia/repos/Maxau/maxauGui

# MSIXパッケージ（Windows Store形式）
yarn tauri build --bundles msix

# または通常のインストーラー
yarn tauri build --bundles msi
```

ビルド後、Windowsエクスプローラーで開く:
```
\\wsl$\Ubuntu\home\nenia\repos\Maxau\maxauGui\src-tauri\target\release\bundle\
```

### 2. CLIでテスト（オーディオ出力なしで動作確認）

```bash
cd /home/nenia/repos/Maxau

# ダミーmp3ファイルを作成（サイレンス）
# または既存の音楽ファイルで試す
./target/release/minau-cli /path/to/music.mp3
```

CLIもALSAを使うため同じエラーが出ますが、コードロジックの確認には使えます。

---

## 🔍 トラブルシューティング

### エラー: `cannot find card '0'`
**原因**: ALSAデバイスが存在しない  
**解決**: Windows版をビルドするか、PulseAudioを設定

### エラー: `Device no longer available`
**原因**: CPALがデフォルトデバイスを取得できない  
**解決**: ダミーデバイスを作成するか、Windows版を使用

### WSLgが動作しない
**確認**:
```bash
# Windows 11のバージョン確認
wsl --version

# WSLgサポートはWindows 11 22H2以降
```

**更新**:
```powershell
# PowerShellで実行（Windowsホスト側）
wsl --update
```

---

## 📝 次のステップ

1. ✅ **Windows版をビルド** - 最も確実
2. ⏳ **PulseAudioを設定** - Linux環境で開発したい場合
3. 🐛 **ダミーデバイス** - UI開発のみの場合

### Windows版ビルドコマンド（再掲）

```bash
cd /home/nenia/repos/Maxau/maxauGui

# フルビルド（MSIX + MSI + exeすべて）
yarn tauri build

# 生成物:
# - src-tauri/target/release/bundle/msix/Maxau_0.1.0_x64.msix (推奨)
# - src-tauri/target/release/bundle/msi/Maxau_0.1.0_x64_en-US.msi
# - src-tauri/target/release/maxaugui.exe
```

---

**まとめ**: WSL2では直接オーディオ出力できないため、**Windows版をビルドして実行**することを強く推奨します。これが最もシンプルで確実な解決策です。

**作成日**: 2025年10月7日  
**環境**: WSL2 (Linux 6.6.87.2-microsoft-standard-WSL2)  
**ステータス**: ✅ 解決策提供済み
