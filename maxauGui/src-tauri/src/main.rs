#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod commands;

use std::sync::{Arc, Mutex};
use commands::{AppState, initialize_library, rescan_library, play, play_url, pause, resume, seek, set_volume, get_album_art};

fn main() {
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap_or_else(|e| {
            eprintln!("Failed to create tokio runtime: {}", e);
            std::process::exit(1);
        });

    rt.block_on(async {
        tauri::Builder::default()
            .manage(AppState {
                playback_handle: Arc::new(Mutex::new(None)),
                playlist: Arc::new(Mutex::new(vec![])),
                current_track_index: Arc::new(Mutex::new(None)),
            })
            .invoke_handler(tauri::generate_handler![
                initialize_library,
                rescan_library,
                play,
                play_url,
                pause,
                resume,
                seek,
                set_volume,
                get_album_art
            ])
            .run(tauri::generate_context!())
            .unwrap_or_else(|e| {
                eprintln!("Error while running tauri application: {}", e);
                std::process::exit(1);
            });
    });
}