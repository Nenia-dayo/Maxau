#![allow(clippy::needless_range_loop)]
use crate::err;
use crate::player::player_structs::Player;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{Stream, StreamConfig};
use ringbuf::HeapRb;
use rubato::{
    Resampler, SincFixedIn, SincInterpolationParameters, SincInterpolationType, WindowFunction,
};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use symphonia::core::audio::{AudioBufferRef, SampleBuffer};
use symphonia::core::codecs::Decoder;
use symphonia::core::errors::Error;
use symphonia::core::formats::SeekMode;
use symphonia::core::formats::{FormatReader, SeekTo};
use symphonia::core::units::Time;

pub struct MusicPlay {
    _stream: Stream,
    seeking: Arc<AtomicBool>,
    paused: Arc<AtomicBool>,
    volume: Arc<parking_lot::Mutex<f32>>,
    position: Arc<AtomicU64>,
    finished: Arc<AtomicBool>,
    format: Arc<Mutex<Box<dyn FormatReader>>>,
    decoder: Arc<Mutex<Box<dyn Decoder>>>,
    track_id: u32,
}
unsafe impl Send for MusicPlay {}

impl Player {
    pub fn play(self) -> MusicPlay {
        let host = cpal::default_host();
        let device = host.default_output_device().unwrap_or_else(|| {
            err!("No output device available");
            std::process::exit(1);
        });
        let device_config = device.default_output_config().unwrap_or_else(|e| {
            err!("Failed to get default output config: {}", e);
            std::process::exit(1);
        });

        let config = StreamConfig {
            channels: device_config.channels(),
            sample_rate: device_config.sample_rate(),
            buffer_size: cpal::BufferSize::Default,
        };

        let output_sample_rate = device_config.sample_rate().0;
        let input_sample_rate = self.sample_rate;

        let paused = Arc::new(AtomicBool::new(false));
        let volume = Arc::new(parking_lot::Mutex::new(1.0f32));
        let position = Arc::new(AtomicU64::new(0));
        let finished = Arc::new(AtomicBool::new(false));
        let seeking = Arc::new(AtomicBool::new(false));

        let format = Arc::clone(&self.format);
        let decoder = Arc::clone(&self.decoder);
        let track_id = self.track_id;

        let (mut producer, mut consumer) = HeapRb::<f32>::new(output_sample_rate as usize).split();

        let position_clone = Arc::clone(&position);
        let finished_clone = Arc::clone(&finished);
        let seeking_clone = Arc::clone(&seeking);
        let channels = self.channels as usize;

        let paused_stream = Arc::clone(&paused);
        let volume_stream = Arc::clone(&volume);

        std::thread::spawn(move || {
            let mut current_samples = Vec::new();
            let mut current_index = 0;
            let mut frames_played = 0u64;

            let needs_resampling = input_sample_rate != output_sample_rate;
            let mut resampler: Option<SincFixedIn<f32>> = if needs_resampling {
                let params = SincInterpolationParameters {
                    sinc_len: 256,
                    f_cutoff: 0.95,
                    interpolation: SincInterpolationType::Linear,
                    oversampling_factor: 256,
                    window: WindowFunction::BlackmanHarris2,
                };

                let chunk_size = 1024; // 固定チャンクサイズ

                match SincFixedIn::new(
                    output_sample_rate as f64 / input_sample_rate as f64,
                    2.0,
                    params,
                    chunk_size,
                    channels,
                ) {
                    Ok(r) => Some(r),
                    Err(e) => {
                        err!("Failed to create resampler: {}", e);
                        None
                    }
                }
            } else {
                None
            };

            let mut resample_buffer = Vec::new();

            // DECODER THREAD
            loop {
                if finished_clone.load(Ordering::Relaxed) {
                    break;
                }

                if seeking_clone.load(Ordering::Relaxed) {
                    current_samples.clear();
                    current_index = 0;
                    if let Some(ref mut r) = resampler {
                        r.reset();
                    }
                    frames_played =
                        position_clone.load(Ordering::Relaxed) * output_sample_rate as u64;
                    std::thread::sleep(Duration::from_millis(50));
                    continue;
                }

                let free_space = producer.free_len();

                if free_space > 2048 {
                    let mut samples_added = 0;
                    const MAX_DECODE_PER_CYCLE: usize = 4;
                    let mut decode_count = 0;

                    while samples_added < free_space && decode_count < MAX_DECODE_PER_CYCLE {
                        if current_index >= current_samples.len() {
                            let mut format = match format.lock() {
                                Ok(f) => f,
                                Err(_) => break,
                            };
                            let mut decoder = match decoder.lock() {
                                Ok(d) => d,
                                Err(_) => break,
                            };

                            let packet = match format.next_packet() {
                                Ok(packet) => packet,
                                Err(Error::IoError(e))
                                    if e.kind() == std::io::ErrorKind::UnexpectedEof =>
                                {
                                    finished_clone.store(true, Ordering::Relaxed);
                                    break;
                                }
                                Err(_) => {
                                    finished_clone.store(true, Ordering::Relaxed);
                                    break;
                                }
                            };

                            if packet.track_id() != track_id {
                                continue;
                            }

                            match decoder.decode(&packet) {
                                Ok(decoded) => {
                                    let samples = convert_samples(decoded);

                                    if let Some(ref mut resampler) = resampler {
                                        let chunk_size = resampler.input_frames_next();

                                        resample_buffer.extend_from_slice(&samples);

                                        let mut resampled_output = Vec::new();

                                        while resample_buffer.len() >= chunk_size * channels {
                                            let mut channel_data =
                                                vec![vec![0.0f32; chunk_size]; channels];
                                            for frame in 0..chunk_size {
                                                for ch in 0..channels {
                                                    let idx = frame * channels + ch;
                                                    channel_data[ch][frame] = resample_buffer[idx];
                                                }
                                            }

                                            match resampler.process(&channel_data, None) {
                                                Ok(resampled) => {
                                                    let out_frames = resampled[0].len();
                                                    for frame in 0..out_frames {
                                                        for ch in 0..channels {
                                                            resampled_output
                                                                .push(resampled[ch][frame]);
                                                        }
                                                    }
                                                }
                                                Err(e) => {
                                                    err!("Resampling error: {}", e);
                                                    break;
                                                }
                                            }

                                            resample_buffer.drain(0..chunk_size * channels);
                                        }
                                        current_samples = resampled_output;
                                    } else {
                                        current_samples = samples;
                                    }

                                    current_index = 0;
                                    decode_count += 1;
                                }
                                Err(_) => continue,
                            }
                        }

                        while current_index < current_samples.len() && samples_added < free_space {
                            let sample = current_samples[current_index];
                            if producer.push(sample).is_err() {
                                break;
                            }
                            current_index += 1;
                            samples_added += 1;

                            if current_index % channels == 0 {
                                frames_played += 1;
                                if frames_played.is_multiple_of(output_sample_rate as u64) {
                                    position_clone.store(
                                        frames_played / output_sample_rate as u64,
                                        Ordering::Relaxed,
                                    );
                                }
                            }
                        }
                    }

                    std::thread::sleep(Duration::from_millis(5));
                } else {
                    std::thread::sleep(Duration::from_millis(10));
                }
            }
        });

        let stream = device
            .build_output_stream(
                &config,
                move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                    if paused_stream.load(Ordering::Relaxed) {
                        for sample in data.iter_mut() {
                            *sample = 0.0;
                        }
                        return;
                    }

                    let vol = *volume_stream.lock();

                    for sample in data.iter_mut() {
                        *sample = consumer.pop().unwrap_or(0.0) * vol;
                    }
                },
                move |err| {
                    err!("Stream error: {}", err);
                },
                None,
            )
            .unwrap_or_else(|e| {
                err!("Failed to build output stream: {}", e);
                std::process::exit(1);
            });

        stream.play().unwrap_or_else(|e| {
            err!("Failed to play stream: {}", e);
            std::process::exit(1);
        });

        MusicPlay {
            _stream: stream,
            seeking,
            paused,
            volume,
            position,
            finished,
            format: self.format,
            decoder: self.decoder,
            track_id: self.track_id,
        }
    }
}

fn convert_samples(buffer: AudioBufferRef) -> Vec<f32> {
    let spec = *buffer.spec();
    let duration = buffer.frames();

    let mut sample_buf = SampleBuffer::<f32>::new(duration as u64, spec);
    sample_buf.copy_interleaved_ref(buffer);
    sample_buf.samples().to_vec()
}

impl MusicPlay {
    pub fn is_empty(&self) -> bool {
        self.finished.load(Ordering::Relaxed)
    }

    pub fn pause(&mut self) {
        self.paused.store(true, Ordering::Relaxed);
    }

    pub fn resume(&mut self) {
        self.paused.store(false, Ordering::Relaxed);
    }

    pub fn is_paused(&self) -> bool {
        self.paused.load(Ordering::Relaxed)
    }

    pub fn get_volume(&self) -> f32 {
        *self.volume.lock()
    }

    pub fn set_volume(self, vol: f32) -> Self {
        *self.volume.lock() = vol.clamp(0.0, 1.0);
        self
    }

    pub fn set_volume_mut(&mut self, vol: f32) {
        *self.volume.lock() = vol.clamp(0.0, 1.0);
    }

    pub fn seek(&self, dur: Duration) -> Result<(), String> {
        let time_secs = dur.as_secs();

        // シーク開始を通知
        self.seeking.store(true, Ordering::Relaxed);

        // デコーダースレッドが確実に停止するまで待機
        std::thread::sleep(Duration::from_millis(100));

        let mut format = self.format.lock().map_err(|_| "Failed to lock format".to_string())?;
        let mut decoder = self.decoder.lock().map_err(|_| "Failed to lock decoder".to_string())?;

        let seek_to = SeekTo::Time {
            time: Time::from(time_secs),
            track_id: Some(self.track_id),
        };

        format
            .seek(SeekMode::Accurate, seek_to)
            .map_err(|e| format!("Seek failed: {}", e))?;

        decoder.reset();

        // 位置を先に更新してからシーク完了フラグを立てる
        self.position.store(time_secs, Ordering::Relaxed);

        // 少し待ってからシーク完了を通知
        std::thread::sleep(Duration::from_millis(50));
        self.seeking.store(false, Ordering::Relaxed);

        Ok(())
    }

    pub fn get_pos(&self) -> Duration {
        Duration::from_secs(self.position.load(Ordering::Relaxed))
    }
}
