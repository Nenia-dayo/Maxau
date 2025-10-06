#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::sync::{Arc, Mutex};
use std::time::Duration;
use tauri::{AppHandle, Manager, State, Wry};
use minau_core::player::{metadata::MetaData, play::MusicPlay, player_structs::Player};
use walkdir::WalkDir;

#[derive(serde::Serialize, Clone, Debug)]
struct Track {
    id: u32,
    path: String,
    filename: String,
    title: Option<String>,
    artist: Option<String>,
    album: Option<String>,
    duration_secs: u64,
}

#[derive(serde::Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
struct PlayerStateSnapshot {
    status: PlaybackStatus,
    playlist: Vec<Track>,
    current_track_index: Option<usize>,
    current_position_secs: u64,
    volume: f32,
    is_initialized: bool,
}

#[derive(serde::Serialize, Clone, Debug)]
enum PlaybackStatus {
    Playing,
    Paused,
    Stopped,
}

struct AppState {
    playback_handle: Arc<Mutex<Option<MusicPlay>>>,
    playlist: Arc<Mutex<Vec<Track>>>,
    current_track_index: Arc<Mutex<Option<usize>>>,
}

lazy_static::lazy_static! {
    static ref TRACK_ID_COUNTER: Mutex<u32> = Mutex::new(0);
}

fn scan_music_folder() -> Vec<Track> {
    let mut tracks = Vec::new();
    if let Some(music_dir) = dirs::audio_dir() {
        for entry in WalkDir::new(music_dir)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.path().is_file())
        {
            let path = entry.path();
            if let Some(extension) = path.extension().and_then(|s| s.to_str()) {
                match extension.to_lowercase().as_str() {
                    "mp3" | "wav" | "flac" | "ogg" => {
                        if let Ok(player) = std::panic::catch_unwind(|| Player::new(path)) {
                            let metadata = player.metadata();
                            let mut id_counter = TRACK_ID_COUNTER.lock().unwrap();
                            *id_counter += 1;
                            
                            let track = Track {
                                id: *id_counter,
                                path: path.to_string_lossy().to_string(),
                                filename: path.file_name().unwrap_or_default().to_string_lossy().to_string(),
                                title: metadata.title(),
                                artist: metadata.artist(),
                                album: metadata.album(),
                                duration_secs: metadata.duration().as_secs(),
                            };
                            tracks.push(track);
                        }
                    }
                    _ => {}
                }
            }
        }
    }
    tracks
}

#[tauri::command]
fn initialize_library(app_state: State<AppState>) -> PlayerStateSnapshot {
    let new_playlist = scan_music_folder();
    *app_state.playlist.lock().unwrap() = new_playlist.clone();

    PlayerStateSnapshot {
        status: PlaybackStatus::Stopped,
        playlist: new_playlist,
        current_track_index: None,
        current_position_secs: 0,
        volume: 1.0,
        is_initialized: true,
    }
}

#[tauri::command]
fn play(track_id: u32, app_state: State<AppState>, app_handle: AppHandle) {
    let playlist = app_state.playlist.lock().unwrap();
    if let Some(index) = playlist.iter().position(|t| t.id == track_id) {
        let track = &playlist[index];
        
        let player = Player::new(&track.path);
        let music_play = player.play().set_volume(0.8);

        *app_state.playback_handle.lock().unwrap() = Some(music_play);
        *app_state.current_track_index.lock().unwrap() = Some(index);

        let track_info_payload = serde_json::json!({ "index": index, "track": track });
        app_handle.emit_all("track-changed", track_info_payload).unwrap();
        app_handle.emit_all("playback-state-changed", PlaybackStatus::Playing).unwrap();
        
        start_progress_emitter(app_handle.clone(), app_state.playback_handle.clone());
    }
}

#[tauri::command]
fn pause(app_state: State<AppState>, app_handle: AppHandle) {
    if let Some(handle) = app_state.playback_handle.lock().unwrap().as_mut() {
        handle.pause();
        app_handle.emit_all("playback-state-changed", PlaybackStatus::Paused).unwrap();
    }
}

#[tauri::command]
fn resume(app_state: State<AppState>, app_handle: AppHandle) {
    if let Some(handle) = app_state.playback_handle.lock().unwrap().as_mut() {
        handle.resume();
        app_handle.emit_all("playback-state-changed", PlaybackStatus::Playing).unwrap();
    }
}

#[tauri::command]
fn seek(position_secs: u64, app_state: State<AppState>) {
    if let Some(handle) = app_state.playback_handle.lock().unwrap().as_ref() {
        let _ = handle.seek(Duration::from_secs(position_secs));
    }
}

#[tauri::command]
fn set_volume(volume: f32, app_state: State<AppState>) {
    if let Some(handle) = app_state.playback_handle.lock().unwrap().as_ref() {
        handle.set_volume(volume);
    }
}

#[tauri::command]
fn get_album_art(track_id: u32, app_state: State<AppState>) -> Option<Vec<u8>> {
    let playlist = app_state.playlist.lock().unwrap();
    if let Some(track) = playlist.iter().find(|t| t.id == track_id) {
        let player = Player::new(&track.path);
        let metadata = player.metadata();
        return metadata.picture();
    }
    None
}

fn start_progress_emitter(app_handle: AppHandle<Wry>, playback_handle: Arc<Mutex<Option<MusicPlay>>>) {
    let handle_clone = Arc::clone(&playback_handle);
    tokio::spawn(async move {
        loop {
            tokio::time::sleep(Duration::from_millis(500)).await;
            
            let handle_guard = handle_clone.lock().unwrap();
            if let Some(handle) = handle_guard.as_ref() {
                if handle.is_paused() { continue; }
                if handle.is_empty() { break; }

                let position = handle.get_pos();
                app_handle.emit_all("playback-progress", serde_json::json!({ "position_secs": position.as_secs() })).unwrap();
            } else {
                break;
            }
        }
    });
}


fn main() {
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();

    rt.block_on(async {
        tauri::Builder::default()
            .manage(AppState {
                playback_handle: Arc::new(Mutex::new(None)),
                playlist: Arc::new(Mutex::new(vec![])),
                current_track_index: Arc::new(Mutex::new(None)),
            })
            .invoke_handler(tauri::generate_handler![
                initialize_library,
                play,
                pause,
                resume,
                seek,
                set_volume,
                get_album_art
            ])
            .run(tauri::generate_context!())
            .expect("error while running tauri application");
    });
}