use crate::{
    err,
    info::{info, info_with_restore, info_with_restore_url},
    play_url::UrlPlayer,
    player::{metadata::MetaData, play::MusicPlay},
};
use crossterm::{
    cursor::{Hide, Show},
    event::{Event, KeyCode, KeyEventKind, poll, read},
    execute,
    style::Stylize,
    terminal::{disable_raw_mode, enable_raw_mode},
};
use parking_lot::Mutex;
use std::{io::stdout, process::exit, sync::Arc, time::Duration};

pub fn init_terminal() {
    enable_raw_mode().unwrap_or_else(|e| {
        err!("Failed to initialize terminal: {}", e);
        exit(1);
    });
    execute!(stdout(), Hide).unwrap_or_else(|e| {
        err!("Failed to initialize terminal: {}", e);
        exit(1);
    });
}

pub fn deinit() {
    disable_raw_mode().unwrap_or_else(|e| {
        err!("Failed to disable raw mode: {}", e);
        err!("Please execute 'reset' command");
        exit(1);
    });

    execute!(stdout(), Show).unwrap_or_else(|e| {
        err!("Failed to initialize terminal: {}", e);
        exit(1);
    });
}

const VOLUME_STEP: f32 = 0.05;
const POLL_INTERVAL_MS: u64 = 100;
const SEEK_STEP_SECS: u64 = 5;

pub async fn get_input_url_mode(
    url_player: Arc<Mutex<UrlPlayer>>,
    url: String,
    key_state: Arc<Mutex<bool>>,
) {
    let url = url.as_str();
    init_terminal();
    loop {
        if *key_state.lock() {
            break;
        }

        if !poll(Duration::from_millis(POLL_INTERVAL_MS)).unwrap_or(false) {
            continue;
        }

        let event = match read() {
            Ok(e) => e,
            Err(e) => {
                err!("Failed to read key: {}", e);
                exit(1);
            }
        };

        if let Event::Key(key) = event {
            if key.kind != KeyEventKind::Press {
                continue;
            }

            match key.code {
                KeyCode::Char('q') => {
                    info("Exitting...");
                    deinit();
                    println!();
                    exit(0);
                }
                KeyCode::Char('>') | KeyCode::Right => {
                    info("Next track");
                    return;
                }
                KeyCode::Char(' ') => {
                    let play = url_player.lock();
                    let msg = if play.is_paused() {
                        play.resume();
                        "|> Resumed"
                    } else {
                        play.pause();
                        "|| Paused"
                    };
                    info_with_restore_url(msg, url);
                }
                KeyCode::Char('+') | KeyCode::Char('=') | KeyCode::Char('k') => {
                    adjust_volume_url(Arc::clone(&url_player), VOLUME_STEP, url);
                }
                KeyCode::Char('-') | KeyCode::Char('_') | KeyCode::Char('j') => {
                    adjust_volume_url(Arc::clone(&url_player), -VOLUME_STEP, url);
                }
                KeyCode::Char('l') => {
                    info_with_restore_url(
                        "Seek is not supported in stream mode".red().to_string(),
                        url,
                    );
                }
                KeyCode::Char('h') => {
                    info_with_restore_url(
                        "Seek is not supported in stream mode".red().to_string(),
                        url,
                    );
                }
                KeyCode::Char(c) => {
                    info_with_restore_url(format!("Unknown key: {}", c.red()), url);
                }
                _ => {}
            }
        }
    }
}

pub async fn get_input(
    music_play: Arc<Mutex<MusicPlay>>,
    quit: Arc<Mutex<bool>>,
    filename: String,
    path: String,
    metadata: MetaData,
) {
    let path = path.as_str();
    init_terminal();
    loop {
        if *quit.lock() {
            return;
        }

        if !poll(Duration::from_millis(POLL_INTERVAL_MS)).unwrap() {
            continue;
        }

        let event = read().unwrap_or_else(|e| {
            err!("Failed to read key: {}", e);
            exit(1);
        });

        if let Event::Key(key) = event {
            if key.kind != KeyEventKind::Press {
                continue;
            }

            match key.code {
                KeyCode::Char('q') => {
                    info("Exitting...");
                    deinit();
                    println!();
                    exit(0);
                }
                KeyCode::Char('>') | KeyCode::Right => {
                    info("Next track");
                    return;
                }
                KeyCode::Char(' ') => {
                    let mut play = music_play.lock();
                    let msg = if play.is_paused() {
                        play.resume();
                        "|> Resumed"
                    } else {
                        play.pause();
                        "|| Paused"
                    };
                    info_with_restore(msg, filename.clone(), path.to_string(), metadata.clone());
                }
                KeyCode::Char('+') | KeyCode::Char('=') | KeyCode::Char('k') => {
                    adjust_volume(&music_play, VOLUME_STEP, &filename, path, &metadata);
                }
                KeyCode::Char('-') | KeyCode::Char('_') | KeyCode::Char('j') => {
                    adjust_volume(&music_play, -VOLUME_STEP, &filename, path, &metadata);
                }
                KeyCode::Char('l') => {
                    let play = music_play.lock();
                    let cur_pos = play.get_pos();
                    let new_pos = cur_pos + Duration::from_secs(SEEK_STEP_SECS);
                    if play.seek(new_pos).is_err() {
                        info_with_restore(
                            "Seek not supported for this audio format".red().to_string(),
                            filename.clone(),
                            path.to_string(),
                            metadata.clone(),
                        );
                    }
                    info(format!(
                        "Seeked forward ({} -> {})",
                        humantime::format_duration(cur_pos),
                        humantime::format_duration(new_pos)
                    ));
                    info_with_restore(
                        format!(
                            "Seeked forward ({} -> {})",
                            humantime::format_duration(cur_pos),
                            humantime::format_duration(new_pos)
                        ),
                        filename.clone(),
                        path.to_string(),
                        metadata.clone(),
                    );
                }
                KeyCode::Char('h') => {
                    let play = music_play.lock();
                    let cur_pos = play.get_pos();
                    let new_pos = cur_pos.saturating_sub(Duration::from_secs(SEEK_STEP_SECS));
                    match play.seek(new_pos) {
                        Ok(_) => {
                            info_with_restore(
                                format!(
                                    "Seeked backward ({} -> {})",
                                    humantime::format_duration(cur_pos),
                                    humantime::format_duration(new_pos)
                                ),
                                filename.clone(),
                                path.to_string(),
                                metadata.clone(),
                            );
                        }
                        Err(e) => {
                            info_with_restore(
                                format!(
                                    "Seek failed: {:?} (pos: {}s -> {}s)",
                                    e,
                                    cur_pos.as_secs(),
                                    new_pos.as_secs()
                                )
                                .red()
                                .to_string(),
                                filename.clone(),
                                path.to_string(),
                                metadata.clone(),
                            );
                        }
                    }
                }
                KeyCode::Char(c) => {
                    info_with_restore(
                        format!("Unknown key: {}", c.red()),
                        filename.clone(),
                        path.to_string(),
                        metadata.clone(),
                    );
                }
                _ => {}
            }
        }
    }
}

fn adjust_volume_url(url_player: Arc<Mutex<UrlPlayer>>, delta: f32, url: &str) {
    let play = url_player.lock();
    let vol = play.get_volume();
    let new_vol = (vol + delta).clamp(0.0, 1.0);

    if new_vol == vol {
        let msg = if delta > 0.0 {
            "Already at maximum volume!".red().to_string()
        } else {
            "Already at minimum volume!".red().to_string()
        };
        info_with_restore_url(msg, url);
    } else {
        play.set_volume(new_vol);
        let percent = (new_vol * 100.0).round() as u16;
        info_with_restore_url(format!("Volume set to {}", percent.to_string().cyan()), url);
    }
}

fn adjust_volume(
    music_play: &Arc<Mutex<MusicPlay>>,
    delta: f32,
    filename: &str,
    path: &str,
    metadata: &MetaData,
) {
    let mut play = music_play.lock();
    let vol = play.get_volume();
    let new_vol = (vol + delta).clamp(0.0, 1.0);

    if new_vol == vol {
        let msg = if delta > 0.0 {
            "Already at maximum volume!".red().to_string()
        } else {
            "Already at minimum volume!".red().to_string()
        };
        info_with_restore(
            msg,
            filename.to_string(),
            path.to_string(),
            metadata.clone(),
        );
    } else {
        play.set_volume_mut(new_vol);
        let percent = (new_vol * 100.0).round() as u16;
        info_with_restore(
            format!("Volume set to {}", percent.to_string().cyan()),
            filename.to_string(),
            path.to_string(),
            metadata.clone(),
        );
    }
}
