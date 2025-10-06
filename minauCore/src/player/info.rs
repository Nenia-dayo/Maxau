use std::process::exit;

use crate::err;
use crate::player::metadata::MetaData;
use crate::player::player_structs::Player;
use lofty::Probe;
use rodio::Source;

impl Player {
    pub fn sample_rate(&self) -> u32 {
        self.decoder.sample_rate()
    }

    pub fn channels(&self) -> u16 {
        self.decoder.channels()
    }

    pub fn metadata(&self) -> MetaData {
        let probe = Probe::open(&self.path).unwrap_or_else(|e| {
            err!("Failed to probe metadata for {}: {}", self.path, e);
            exit(1);
        });

        MetaData::new(probe)
    }
}
