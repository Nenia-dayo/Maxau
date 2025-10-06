use std::process::exit;

use crate::err;
use crate::player::metadata::MetaData;
use crate::player::player_structs::Player;
use lofty::probe::Probe;

impl Player {
    pub fn metadata(&self) -> MetaData {
        let probe = Probe::open(&self.path).unwrap_or_else(|e| {
            err!("Failed to probe metadata for {}: {}", self.path, e);
            exit(1);
        });

        MetaData::new(probe)
    }
}
