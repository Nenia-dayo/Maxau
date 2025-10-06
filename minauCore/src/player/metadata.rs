use lofty::file::{AudioFile, TaggedFileExt};
use lofty::probe::Probe;
use lofty::properties::FileProperties;
use lofty::tag::{Accessor, Tag};
use std::fs::File;
use std::io::BufReader;
use std::process::exit;
use std::time::Duration;

use crate::err;

#[derive(Clone)]
pub struct MetaData {
    pub tag: Option<Tag>,
    pub prop: FileProperties,
    pub title: Option<String>,
}

impl MetaData {
    pub fn new(probe: Probe<BufReader<File>>) -> Self {
        let bind = match probe.read() {
            Ok(bind) => bind,
            Err(e) => {
                err!("Failed to read metadata: {}", e);
                exit(1);
            }
        };

        let Some(s) = bind.primary_tag() else {
            return Self {
                tag: None,
                prop: bind.properties().clone(),
                title: None,
            };
        };

        Self {
            tag: Some(s.clone()),
            prop: bind.properties().clone(),
            title: None,
        }
    }

    pub fn set_title(&mut self, title: Option<String>) {
        self.title = title
    }

    pub fn title(&self) -> Option<String> {
        if self.title.is_some() {
            self.title.clone()
        } else if let Some(tag) = &self.tag.clone() {
            tag.title().as_ref().map(|title| title.to_string())
        } else {
            None
        }
    }

    pub fn artist(&self) -> Option<String> {
        if let Some(tag) = &self.tag.clone() {
            tag.artist().as_ref().map(|artist| artist.to_string())
        } else {
            None
        }
    }

    pub fn album(&self) -> Option<String> {
        if let Some(tag) = &self.tag.clone() {
            tag.album().as_ref().map(|album| album.to_string())
        } else {
            None
        }
    }

    pub fn duration(&self) -> Duration {
        self.prop.duration()
    }

    /// returns first of picture data
    pub fn picture(&self) -> Option<Vec<u8>> {
        if let Some(s) = self.tag.clone()
            && let Some(s) = s.pictures().first()
        {
            return Some(s.data().to_vec());
        }
        None
    }
}
