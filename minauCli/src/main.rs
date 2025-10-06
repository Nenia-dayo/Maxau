#![doc(
    html_favicon_url = "https://raw.githubusercontent.com/sirasaki-konoha/minau/refs/heads/master/icon/minau-icon.png"
)]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/sirasaki-konoha/minau/refs/heads/master/icon/minau-icon.png"
)]
use std::{path::Path, process::exit};
use clap::Parser;
use url::Url;
use async_compat::CompatExt;
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

fn main() {
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
        if let Some(ext) = path_extens.extension()
            && (ext == "m3u" || ext == "m3u8")
        {
            smol::block_on(async {
                m3u::play_m3u(&path, volume, args.gui).compat().await;
            });
            continue;
        }

        let bind = path.clone();
        if (bind.starts_with("file://")
            || bind.starts_with("http://")
            || bind.starts_with("https://"))
            && let Ok(url) = Url::parse(&bind)
        {
            if let Ok(file_url) = url.to_file_path() {
                smol::block_on(async {
                    play_music::play_music(
                        file_url.to_string_lossy().to_string(),
                        volume,
                        args.gui,
                        None,
                    )
                    .await;
                });
                continue;
            }
            smol::block_on(async {
                play_url::play_url(&bind, volume, None).compat().await;
            });
            continue;
        }

        smol::block_on(async {
            play_music::play_music(&path, volume, args.gui, None).await;
        });
    }
}
