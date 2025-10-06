// /workspaces/Maxau/minauCli/src/main.rs

use clap::Parser;
use std::{path::Path, process::exit};
use url::Url;

// `minau_core` ライブラリから必要なモジュールとマクロをインポートします
use minau_core::{err, m3u, play_music, play_url};

#[derive(Parser)]
#[command(name = env!("CARGO_PKG_NAME"))]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(about = env!("CARGO_PKG_DESCRIPTION"))]
struct Cli {
    /// Files to play (multiple selections allowed)
    files: Vec<String>,
    /// Specify the default playback volume (minimum: 1, maximum: 100)
    #[arg(short, long)]
    volume: Option<u16>,
    /// Display album art in a GUI
    #[arg(short, long)]
    gui: bool,
}

const DEFAULT_VOLUME: u16 = 100;
const MIN_VOLUME: u16 = 1;
const MAX_VOLUME: u16 = 100;

#[tokio::main]
async fn main() {
    let args = Cli::parse();

    let volume = args
        .volume
        .map(|vol| {
            if (MIN_VOLUME..=MAX_VOLUME).contains(&vol) {
                Ok(vol as f32 / 100.0)
            } else {
                Err(vol)
            }
        })
        .unwrap_or(Ok(DEFAULT_VOLUME as f32 / 100.0))
        .unwrap_or_else(|vol| {
            err!("{} is not available volume", vol);
            exit(1);
        });

    if args.files.is_empty() {
        err!("Music file is not specified!");
        exit(1);
    }

    for path in args.files {
        let path_extens: &Path = path.as_ref();
        if let Some(ext) = path_extens.extension() {
            if ext == "m3u" || ext == "m3u8" {
                m3u::play_m3u(&path, volume, args.gui).await;
                continue;
            }
        }

        let bind = path.clone();
        if let Ok(url) = Url::parse(&bind) {
            match url.scheme() {
                "file" => {
                    if let Ok(file_path) = url.to_file_path() {
                        play_music::play_music(&file_path, volume, args.gui, None).await;
                        continue;
                    }
                }
                "http" | "https" => {
                    play_url::play_url(&bind, volume, None).await;
                    continue;
                }
                _ => {}
            }
        }

        play_music::play_music(&path, volume, args.gui, None).await;
    }
}