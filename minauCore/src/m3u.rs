use url::Url;

use crate::{
    err,
    play_music::{self, play_music},
    play_url,
};
use std::{fs, path::Path, process::exit};

struct M3uEntry {
    path: String,
    title: Option<String>,
    #[allow(dead_code)]
    duration: Option<i32>,
}

fn parse(m3u: &str) -> Vec<M3uEntry> {
    let mut entries = Vec::new();
    let mut current_title = None;
    let mut current_duration = None;

    // #EXTM3Uヘッダがない場合は、シンプルなM3Uとして扱う
    if !m3u.lines().next().unwrap_or("").starts_with("#EXTM3U") {
        return m3u
            .lines()
            .map(|line| line.trim())
            .filter(|line| !line.is_empty() && !line.starts_with('#'))
            .map(|line| M3uEntry {
                path: line.to_string(),
                title: None,
                duration: None,
            })
            .collect();
    }

    for line in m3u.lines().map(|l| l.trim()).filter(|l| !l.is_empty()) {
        if line.starts_with("#EXTINF:") {
            let info = line.strip_prefix("#EXTINF:").unwrap_or("").trim();
            let mut parts = info.splitn(2, ',');
            current_duration = parts.next().and_then(|s| s.parse::<i32>().ok());
            current_title = parts.next().map(String::from);
        } else if !line.starts_with('#') {
            entries.push(M3uEntry {
                path: line.to_string(),
                title: current_title.take(),
                duration: current_duration.take(),
            });
        }
    }
    entries
}

pub async fn play_m3u<P: AsRef<Path>>(path: P, volume: f32, gui: bool) {
    let path = path.as_ref();
    let content = fs::read_to_string(path).unwrap_or_else(|e| {
        err!("Failed to read m3u file: {}", e);
        exit(1);
    });

    for entry in parse(&content) {
        if let Ok(url) = Url::parse(&entry.path) {
            if let Ok(url_file) = url.to_file_path() {
                play_music(
                    url_file.to_string_lossy().to_string(),
                    volume,
                    gui,
                    entry.title.clone(),
                )
                .await;
                continue;
            }
            play_url::play_url(&entry.path, volume, entry.title).await;
            continue;
        }

        let file_path = if Path::new(&entry.path).is_absolute() {
            entry.path
        } else {
            path.parent()
                .unwrap_or_else(|| Path::new("."))
                .join(&entry.path)
                .to_string_lossy()
                .to_string()
        };

        play_music::play_music(file_path, volume, gui, entry.title).await;
    }
}
