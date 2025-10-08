use std::sync::{Arc, Mutex};
use std::time::Duration;
use tauri::{AppHandle, Emitter, State, Wry};
use minau_core::player::{play::MusicPlay, player_structs::Player};
use walkdir::WalkDir;
use lazy_static::lazy_static;

#[derive(serde::Serialize, Clone, Debug)]
pub struct Track {
    pub id: u32,
    pub path: String,
    pub filename: String,
    pub title: Option<String>,
    pub artist: Option<String>,
    pub album: Option<String>,
    pub duration_secs: u64,
}

#[derive(serde::Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PlayerStateSnapshot {
    pub status: PlaybackStatus,
    pub playlist: Vec<Track>,
    pub current_track_index: Option<usize>,
    pub current_position_secs: u64,
    pub volume: f32,
    pub is_initialized: bool,
}

#[derive(serde::Serialize, Clone, Debug)]
pub enum PlaybackStatus {
    Playing,
    Paused,
    Stopped,
}

pub struct AppState {
    pub playback_handle: Arc<Mutex<Option<MusicPlay>>>,
    pub playlist: Arc<Mutex<Vec<Track>>>,
    pub current_track_index: Arc<Mutex<Option<usize>>>,
}

lazy_static! {
    static ref TRACK_ID_COUNTER: Mutex<u32> = Mutex::new(0);
}

pub fn scan_music_folder() -> Vec<Track> {
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
                    "mp3" | "wav" | "flac" | "ogg" | "m4a" | "aac" | "opus" | "wma" => {
                        if let Ok(player) = std::panic::catch_unwind(|| Player::new(path)) {
                            let metadata = player.metadata();
                            let mut id_counter = match TRACK_ID_COUNTER.lock() {
                                Ok(counter) => counter,
                                Err(_) => continue,
                            };
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
pub fn initialize_library(app_state: State<AppState>) -> PlayerStateSnapshot {
    let new_playlist = scan_music_folder();
    if let Ok(mut playlist) = app_state.playlist.lock() {
        *playlist = new_playlist.clone();
    }

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
pub fn rescan_library(app_state: State<AppState>, app_handle: AppHandle) -> Result<PlayerStateSnapshot, String> {
    let new_playlist = scan_music_folder();
    
    if let Ok(mut playlist) = app_state.playlist.lock() {
        *playlist = new_playlist.clone();
    }

    let _ = app_handle.emit("playlist-updated", new_playlist.clone());

    Ok(PlayerStateSnapshot {
        status: PlaybackStatus::Stopped,
        playlist: new_playlist,
        current_track_index: None,
        current_position_secs: 0,
        volume: 1.0,
        is_initialized: true,
    })
}

#[tauri::command]
pub fn play(track_id: u32, app_state: State<AppState>, app_handle: AppHandle) -> Result<(), String> {
    let playlist = app_state.playlist.lock().map_err(|e| format!("Failed to lock playlist: {}", e))?;
    
    if let Some(index) = playlist.iter().position(|t| t.id == track_id) {
        let track = &playlist[index];
        
        let player = Player::new(&track.path);
        let music_play = player.play().set_volume(0.8);

        if let Ok(mut handle) = app_state.playback_handle.lock() {
            *handle = Some(music_play);
        }
        if let Ok(mut idx) = app_state.current_track_index.lock() {
            *idx = Some(index);
        }

        let track_info_payload = serde_json::json!({ "index": index, "track": track });
        let _ = app_handle.emit("track-changed", track_info_payload);
        let _ = app_handle.emit("playback-state-changed", PlaybackStatus::Playing);
        
        start_progress_emitter(app_handle.clone(), app_state.playback_handle.clone());
        
        Ok(())
    } else {
        Err(format!("Track with ID {} not found", track_id))
    }
}

#[tauri::command]
pub fn play_url(url: String, app_state: State<AppState>, app_handle: AppHandle) -> Result<(), String> {
    use url::Url;
    
    // URLの検証
    let parsed_url = Url::parse(&url).map_err(|e| format!("Invalid URL: {}", e))?;
    
    // HTTPまたはHTTPSのみ許可
    if parsed_url.scheme() != "http" && parsed_url.scheme() != "https" {
        return Err("Only HTTP and HTTPS URLs are supported".to_string());
    }

    // 既存の再生を停止
    if let Ok(mut handle) = app_state.playback_handle.lock() {
        *handle = None;
    }

    // 新しいスレッドでURL再生を開始
    let app_handle_clone = app_handle.clone();
    let url_clone = url.clone();
    
    tokio::spawn(async move {
        use async_compat::CompatExt;
        
        // GUIモードフラグとして空文字列を渡す
        match minau_core::play_url::play_url(&url_clone, 0.8, Some(String::new())).compat().await {
            Ok(_) => {
                let _ = app_handle_clone.emit("playback-state-changed", PlaybackStatus::Stopped);
            }
            Err(e) => {
                eprintln!("URL playback error: {}", e);
                let _ = app_handle_clone.emit("playback-error", format!("Failed to play URL: {}", e));
            }
        }
    });

    let _ = app_handle.emit("playback-state-changed", PlaybackStatus::Playing);
    
    Ok(())
}

#[tauri::command]
pub fn pause(app_state: State<AppState>, app_handle: AppHandle) -> Result<(), String> {
    if let Ok(mut guard) = app_state.playback_handle.lock() {
        if let Some(handle) = guard.as_mut() {
            handle.pause();
            let _ = app_handle.emit("playback-state-changed", PlaybackStatus::Paused);
            Ok(())
        } else {
            Err("No active playback".to_string())
        }
    } else {
        Err("Failed to lock playback handle".to_string())
    }
}

#[tauri::command]
pub fn resume(app_state: State<AppState>, app_handle: AppHandle) -> Result<(), String> {
    if let Ok(mut guard) = app_state.playback_handle.lock() {
        if let Some(handle) = guard.as_mut() {
            handle.resume();
            let _ = app_handle.emit("playback-state-changed", PlaybackStatus::Playing);
            Ok(())
        } else {
            Err("No active playback".to_string())
        }
    } else {
        Err("Failed to lock playback handle".to_string())
    }
}

#[tauri::command]
pub fn seek(position_secs: u64, app_state: State<AppState>) -> Result<(), String> {
    if let Ok(guard) = app_state.playback_handle.lock() {
        if let Some(handle) = guard.as_ref() {
            handle.seek(Duration::from_secs(position_secs))
        } else {
            Err("No active playback".to_string())
        }
    } else {
        Err("Failed to lock playback handle".to_string())
    }
}

#[tauri::command]
pub fn set_volume(volume: f32, app_state: State<AppState>) -> Result<(), String> {
    if let Ok(mut guard) = app_state.playback_handle.lock() {
        if let Some(handle) = guard.as_mut() {
            handle.set_volume_mut(volume);
            Ok(())
        } else {
            Err("No active playback".to_string())
        }
    } else {
        Err("Failed to lock playback handle".to_string())
    }
}

#[tauri::command]
pub fn get_album_art(track_id: u32, app_state: State<AppState>) -> Option<Vec<u8>> {
    let playlist = app_state.playlist.lock().ok()?;
    let track = playlist.iter().find(|t| t.id == track_id)?;
    let player = Player::new(&track.path);
    let metadata = player.metadata();
    metadata.picture()
}

pub fn start_progress_emitter(app_handle: AppHandle<Wry>, playback_handle: Arc<Mutex<Option<MusicPlay>>>) {
    tokio::spawn(async move {
        loop {
            tokio::time::sleep(Duration::from_millis(500)).await;
            
            let handle_guard = match playback_handle.lock() {
                Ok(guard) => guard,
                Err(_) => break,
            };
            
            if let Some(handle) = handle_guard.as_ref() {
                if handle.is_paused() { continue; }
                if handle.is_empty() { break; }

                let position = handle.get_pos();
                let _ = app_handle.emit("playback-progress", serde_json::json!({ "position_secs": position.as_secs() }));
            } else {
                break;
            }
        }
    });
}
