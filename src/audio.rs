use anyhow::Result;
use cpal::{
    traits::{DeviceTrait, HostTrait},
    Device, Host, SampleRate, StreamConfig,
};
use rubato::{Resampler, SincFixedIn, SincInterpolationType, SincInterpolationParameters, WindowFunction};
use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};
use symphonia::{
    core::{
        audio::{AudioBufferRef, Signal, SignalSpec},
        codecs::DecoderOptions,
        formats::{FormatOptions},
        io::MediaSourceStream,
        meta::MetadataOptions,
        probe::Hint,
    },
    default::get_probe,
};

pub struct AudioPlayer {
    host: Host,
    device: Device,
    stream_config: StreamConfig,
    sample_buffer: Arc<Mutex<VecDeque<f32>>>,
    is_playing: Arc<Mutex<bool>>,
    current_position: Arc<Mutex<Duration>>,
    duration: Arc<Mutex<Duration>>,
    volume: f32,
}

impl AudioPlayer {
    pub fn new() -> Result<Self> {
        let host = cpal::default_host();
        let device = host
            .default_output_device()
            .ok_or_else(|| anyhow::anyhow!("No output device available"))?;

        let mut supported_configs = device.supported_output_configs()?;
        let config = supported_configs
            .next()
            .ok_or_else(|| anyhow::anyhow!("No supported configs"))?
            .with_sample_rate(SampleRate(48000));

        let stream_config = config.into();

        Ok(Self {
            host,
            device,
            stream_config,
            sample_buffer: Arc::new(Mutex::new(VecDeque::new())),
            is_playing: Arc::new(Mutex::new(false)),
            current_position: Arc::new(Mutex::new(Duration::ZERO)),
            duration: Arc::new(Mutex::new(Duration::ZERO)),
            volume: 0.7,
        })
    }

    pub fn load_file(&mut self, path: &str) -> Result<()> {
        let file = std::fs::File::open(path)?;
        let mss = MediaSourceStream::new(Box::new(file), Default::default());
        let hint = Hint::new();
        let meta_opts: MetadataOptions = Default::default();
        let fmt_opts: FormatOptions = Default::default();

        let probed = get_probe().format(&hint, mss, &fmt_opts, &meta_opts)?;
        let mut format = probed.format;

        let track = format
            .tracks()
            .iter()
            .find(|t| t.codec_params.codec != symphonia::core::codecs::CODEC_TYPE_NULL)
            .ok_or_else(|| anyhow::anyhow!("No supported audio tracks"))?;

        let track_id = track.id;
        let mut decoder = symphonia::default::get_codecs()
            .make(&track.codec_params, &DecoderOptions::default())?;

        let spec = track.codec_params.sample_rate.map(|rate| {
            SignalSpec::new(rate, symphonia::core::audio::Channels::FRONT_LEFT | symphonia::core::audio::Channels::FRONT_RIGHT)
        }).unwrap_or_else(|| SignalSpec::new(48000, symphonia::core::audio::Channels::FRONT_LEFT | symphonia::core::audio::Channels::FRONT_RIGHT));
        let sample_rate = spec.rate as usize;
        let _channels = spec.channels.count();

        // Create resampler if needed
        let mut resampler = if sample_rate != 48000 {
            Some(SincFixedIn::<f32>::new(
                (48000.0 / sample_rate as f32) as f64,
                2.0,
                SincInterpolationParameters {
                    sinc_len: 256,
                    f_cutoff: 0.95,
                    interpolation: SincInterpolationType::Linear,
                    oversampling_factor: 256,
                    window: WindowFunction::BlackmanHarris2,
                },
                256,
                256,
            )?)
        } else {
            None
        };

        let sample_buffer = Arc::clone(&self.sample_buffer);
        let is_playing = Arc::clone(&self.is_playing);
        let current_position = Arc::clone(&self.current_position);
        let duration = Arc::clone(&self.duration);
        let volume = self.volume;

        // Calculate duration
        if let Some(dur) = track.codec_params.n_frames {
            let duration_secs = dur as f64 / sample_rate as f64;
            *duration.lock().unwrap() = Duration::from_secs_f64(duration_secs);
        }

        thread::spawn(move || {
            let _samples: Vec<f32> = Vec::new();
            let mut position = Duration::ZERO;

            loop {
                if !*is_playing.lock().unwrap() {
                    thread::sleep(Duration::from_millis(10));
                    continue;
                }

                match format.next_packet() {
                    Ok(packet) => {
                        if packet.track_id() != track_id {
                            continue;
                        }

                        match decoder.decode(&packet) {
                            Ok(audio_buf) => {
                                let spec = audio_buf.spec();
                                let _sample_rate = spec.rate as usize;
                                let channels = spec.channels.count();

                                // Convert to f32 samples
                                let mut f32_samples = match audio_buf {
                                    AudioBufferRef::F32(buf) => buf.chan(0).to_vec(),
                                    AudioBufferRef::U8(buf) => {
                                        buf.chan(0).iter().map(|&s| s as f32 / 128.0 - 1.0).collect()
                                    }
                                    AudioBufferRef::U16(buf) => {
                                        buf.chan(0).iter().map(|&s| s as f32 / 32768.0).collect()
                                    }
                                    AudioBufferRef::U24(buf) => {
                                        buf.chan(0).iter().map(|&s| s.inner() as f32 / 8388608.0).collect()
                                    }
                                    AudioBufferRef::U32(buf) => {
                                        buf.chan(0).iter().map(|&s| s as f32 / 2147483648.0).collect()
                                    }
                                    AudioBufferRef::S8(buf) => {
                                        buf.chan(0).iter().map(|&s| s as f32 / 128.0).collect()
                                    }
                                    AudioBufferRef::S16(buf) => {
                                        buf.chan(0).iter().map(|&s| s as f32 / 32768.0).collect()
                                    }
                                    AudioBufferRef::S24(buf) => {
                                        buf.chan(0).iter().map(|&s| s.inner() as f32 / 8388608.0).collect()
                                    }
                                    AudioBufferRef::S32(buf) => {
                                        buf.chan(0).iter().map(|&s| s as f32 / 2147483648.0).collect()
                                    }
                                    AudioBufferRef::F64(buf) => {
                                        buf.chan(0).iter().map(|&s| s as f32).collect()
                                    }
                                };

                                // Resample if needed
                                if let Some(resampler) = &mut resampler {
                                    let input = vec![f32_samples.clone()];
                                    if let Ok(resampled) = resampler.process(&input, None) {
                                        f32_samples = resampled[0].clone();
                                    }
                                }

                                // Apply volume
                                for sample in &mut f32_samples {
                                    *sample *= volume;
                                }

                                // Add to buffer
                                let sample_count = f32_samples.len();
                                {
                                    let mut buffer = sample_buffer.lock().unwrap();
                                    for sample in f32_samples {
                                        buffer.push_back(sample);
                                    }
                                }

                                // Update position
                                position += Duration::from_secs_f64(
                                    sample_count as f64 / (48000.0 * channels as f64),
                                );
                                *current_position.lock().unwrap() = position;
                            }
                            Err(symphonia::core::errors::Error::ResetRequired) => {
                                decoder.reset();
                            }
                            Err(_) => break,
                        }
                    }
                    Err(symphonia::core::errors::Error::ResetRequired) => {
                        decoder.reset();
                    }
                    Err(_) => break,
                }
            }
        });

        Ok(())
    }

    pub fn play(&self) {
        *self.is_playing.lock().unwrap() = true;
    }

    pub fn pause(&self) {
        *self.is_playing.lock().unwrap() = false;
    }

    pub fn is_playing(&self) -> bool {
        *self.is_playing.lock().unwrap()
    }

    pub fn get_position(&self) -> Duration {
        *self.current_position.lock().unwrap()
    }

    pub fn get_duration(&self) -> Duration {
        *self.duration.lock().unwrap()
    }

    pub fn get_samples(&self) -> Vec<f32> {
        let mut buffer = self.sample_buffer.lock().unwrap();
        let samples: Vec<f32> = buffer.drain(..).collect();
        samples
    }

    pub fn set_volume(&mut self, volume: f32) {
        self.volume = volume.clamp(0.0, 1.0);
    }
}