use std::time::Duration;

use crate::err;
use crate::player::player_structs::Player;
use rodio::{OutputStream, Sink};

pub struct MusicPlay {
    sink: Sink,
    _stream_handle: OutputStream,
}

impl Player {
    pub fn play(self) -> MusicPlay {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap_or_else(|e| {
            err!("Failed to open stream: {}", e);
            std::process::exit(1);
        });

        let sink = Sink::try_new(&stream_handle).unwrap_or_else(|e| {
            err!("Failed to create sink: {}", e);
            std::process::exit(1);
        });

        sink.append(self.decoder);

        MusicPlay {
            sink,
            _stream_handle: _stream,
        }
    }
}

impl MusicPlay {
    pub fn is_empty(&self) -> bool {
        self.sink.empty()
    }

    pub fn pause(&mut self) {
        self.sink.pause();
    }

    pub fn resume(&mut self) {
        self.sink.play();
    }

    pub fn is_paused(&self) -> bool {
        self.sink.is_paused()
    }

    pub fn get_volume(&self) -> f32 {
        self.sink.volume()
    }

    pub fn set_volume(self, vol: f32) -> Self {
        self.sink.set_volume(vol);
        self
    }

    pub fn set_volume_mut(&mut self, vol: f32) {
        self.sink.set_volume(vol);
    }

    pub fn seek(&self, _dur: Duration) {
        // Note: rodio 0.17 removed try_seek functionality
        // Seeking is not currently supported
    }

    pub fn get_pos(&self) -> std::time::Duration {
        // Note: rodio 0.17 removed get_pos functionality
        // Returning zero as a placeholder
        Duration::from_secs(0)
    }
}
