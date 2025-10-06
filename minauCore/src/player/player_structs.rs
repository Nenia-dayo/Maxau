use std::path::Path;
use std::sync::{Arc, Mutex};
use std::{fs::File, process::exit};
use symphonia::core::codecs::{CODEC_TYPE_NULL, Decoder, DecoderOptions};
use symphonia::core::formats::{FormatOptions, FormatReader};
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;

use crate::err;

pub struct Player {
    pub format: Arc<Mutex<Box<dyn FormatReader>>>,
    pub decoder: Arc<Mutex<Box<dyn Decoder>>>,
    pub track_id: u32,
    pub sample_rate: u32,
    pub channels: u16,
    pub path: String,
}

impl Clone for Player {
    fn clone(&self) -> Self {
        Player::new(&self.path)
    }
}

impl Player {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        let path_str = path.as_ref()
            .to_str()
            .unwrap_or("<invalid UTF-8 path>")
            .to_string();

        let file = File::open(&path).unwrap_or_else(|e| {
            err!("Failed to open {}: {}", path_str, e);
            exit(1);
        });

        let mss = MediaSourceStream::new(Box::new(file), Default::default());

        let mut hint = Hint::new();
        if let Some(ext) = path.as_ref().extension() {
            if let Some(ext_str) = ext.to_str() {
                hint.with_extension(ext_str);
            }
        }

        let meta_opts: MetadataOptions = Default::default();
        let fmt_opts: FormatOptions = Default::default();

        let probed = symphonia::default::get_probe()
            .format(&hint, mss, &fmt_opts, &meta_opts)
            .unwrap_or_else(|e| {
                err!("Failed to probe format: {}", e);
                exit(1);
            });

        let format = probed.format;

        let track = format
            .tracks()
            .iter()
            .find(|t| t.codec_params.codec != CODEC_TYPE_NULL)
            .unwrap_or_else(|| {
                err!("No supported audio track found");
                exit(1);
            });

        let track_id = track.id;
        let codec_params = &track.codec_params;

        // codec_paramsから情報を先に取得
        let sample_rate = codec_params.sample_rate.unwrap_or_else(|| {
            err!("No sample rate information found");
            exit(1);
        });

        let channels = codec_params.channels.map(|c| c.count() as u16).unwrap_or(2);

        let dec_opts: DecoderOptions = Default::default();
        let decoder = symphonia::default::get_codecs()
            .make(codec_params, &dec_opts)
            .unwrap_or_else(|e| {
                err!("Failed to create decoder: {}", e);
                exit(1);
            });

        Self {
            format: Arc::new(Mutex::new(format)),
            decoder: Arc::new(Mutex::new(decoder)),
            track_id,
            sample_rate,
            channels,
            path: path_str,
        }
    }

    pub fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    pub fn channels(&self) -> u16 {
        self.channels
    }
}
