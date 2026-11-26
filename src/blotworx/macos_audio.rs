//! macOS audio output backed by CoreAudio through cpal.
//!
//! CoreAudio's default output unit performs sample-rate conversion
use anyhow::{bail, ensure, Context, Result};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::sync::{Arc, Condvar, Mutex, TryLockError};
use std::time::{Duration, Instant};

struct Ring {
    data: Vec<i16>,
    read: usize,
    write: usize,
    len: usize,
    channels: usize,
    submitted_frames: u64,
    consumed_frames: u64,
    first_pop: Option<Instant>,
}
impl Ring {
    fn new(capacity_frames: usize, channels: usize) -> Self {
        Self {
            data: vec![0; capacity_frames.max(1) * channels],
            read: 0,
            write: 0,
            len: 0,
            channels,
            submitted_frames: 0,
            consumed_frames: 0,
            first_pop: None,
        }
    }
    fn space(&self) -> usize {
        self.data.len().saturating_sub(self.len)
    }
    fn push(&mut self, samples: &[i16]) {
        let capacity = self.data.len();
        let first = (capacity - self.write).min(samples.len());
        self.data[self.write..self.write + first].copy_from_slice(&samples[..first]);
        let rest = samples.len() - first;
        if rest > 0 {
            self.data[..rest].copy_from_slice(&samples[first..]);
        }
        self.write = (self.write + samples.len()) % capacity;
        self.len += samples.len();
        self.submitted_frames += (samples.len() / self.channels) as u64;
    }
    fn pop<T: cpal::SizedSample + cpal::FromSample<i16>>(&mut self, out: &mut [T]) {
        let frames = out.len() / self.channels;
        let available = self.len / self.channels;
        let take = frames.min(available);
        let samples = take * self.channels;
        for (index, sample) in out[..samples].iter_mut().enumerate() {
            *sample = T::from_sample(self.data[(self.read + index) % self.data.len()]);
        }
        self.read = (self.read + samples) % self.data.len();
        self.len -= samples;
        self.consumed_frames += take as u64;
        if take > 0 && self.first_pop.is_none() {
            self.first_pop = Some(Instant::now());
            log::info!("Audio output started consuming");
        }
        for sample in &mut out[samples..] {
            *sample = T::EQUILIBRIUM;
        }
    }
}

#[derive(Clone)]
pub struct Handle {
    ring: Arc<Mutex<Ring>>,
    ready: Arc<Condvar>,
    rate: u32,
    channels: u16,
}
impl Handle {
    pub fn push(&self, samples: &[i16], timeout: Duration) -> Result<u64> {
        ensure!(
            samples.len() % usize::from(self.channels) == 0,
            "Audio chunk is not frame-aligned"
        );
        if samples.is_empty() {
            return Ok(self.submitted_frames());
        }
        let mut ring = self
            .ring
            .lock()
            .map_err(|_| anyhow::anyhow!("Audio ring lock is broken"))?;
        let deadline = Instant::now() + timeout;
        while ring.space() < samples.len() {
            let now = Instant::now();
            if now >= deadline {
                bail!("Audio output queue is full");
            }
            let (guard, result) = self
                .ready
                .wait_timeout(ring, deadline - now)
                .map_err(|_| anyhow::anyhow!("Audio ring wait is broken"))?;
            ring = guard;
            if result.timed_out() && ring.space() < samples.len() {
                bail!("Audio output queue is full");
            }
        }
        ring.push(samples);
        Ok(ring.submitted_frames)
    }
    pub fn consumed_frames(&self) -> u64 {
        self.ring.lock().map_or(0, |ring| ring.consumed_frames)
    }
    pub fn submitted_frames(&self) -> u64 {
        self.ring.lock().map_or(0, |ring| ring.submitted_frames)
    }
    pub fn position_frames(&self) -> u64 {
        self.ring.lock().map_or(0, |ring| {
            ring.first_pop
                .map_or(0, |started| {
                    (started.elapsed().as_secs_f64() * f64::from(self.rate)) as u64
                })
                .min(ring.submitted_frames)
        })
    }
    pub fn done(&self, end_frame: u64) -> bool {
        self.consumed_frames() >= end_frame
    }
}

pub struct Output {
    handle: Handle,
    stream: cpal::Stream,
}
impl Output {
    /// `start` controls whether audio begins immediately
    pub fn open(rate: u32, channels: u16, capacity_frames: usize, start: bool) -> Result<Self> {
        ensure!(rate > 0 && channels > 0, "Invalid audio format");
        let host = cpal::default_host();
        let device = host
            .default_output_device()
            .context("No default audio output device")?;
        let default = device
            .default_output_config()
            .context("Audio output device has no default format")?;
        log::info!(
            "Audio output ready on {:?} at {} Hz, {} channels ({:?})",
            device.name().unwrap_or_else(|_| "default device".into()),
            rate,
            channels,
            default.sample_format()
        );
        let config = cpal::StreamConfig {
            channels,
            sample_rate: cpal::SampleRate(rate),
            buffer_size: cpal::BufferSize::Default,
        };
        let ring = Arc::new(Mutex::new(Ring::new(
            capacity_frames,
            usize::from(channels),
        )));
        let ready = Arc::new(Condvar::new());
        let stream = build_stream(
            &device,
            &config,
            default.sample_format(),
            ring.clone(),
            ready.clone(),
        )?;
        if start {
            stream.play()?;
        } else {
            stream.pause()?;
        }
        Ok(Self {
            handle: Handle {
                ring,
                ready,
                rate,
                channels,
            },
            stream,
        })
    }
    pub fn handle(&self) -> Handle {
        self.handle.clone()
    }
    pub fn start(&self) -> Result<()> {
        self.stream.play()?;
        Ok(())
    }
    pub fn push(&self, samples: &[i16], timeout: Duration) -> Result<u64> {
        self.handle.push(samples, timeout)
    }
    pub fn consumed_frames(&self) -> u64 {
        self.handle.consumed_frames()
    }
    pub fn position_frames(&self) -> u64 {
        self.handle.position_frames()
    }
}

fn build_stream(
    device: &cpal::Device,
    config: &cpal::StreamConfig,
    sample_format: cpal::SampleFormat,
    ring: Arc<Mutex<Ring>>,
    ready: Arc<Condvar>,
) -> Result<cpal::Stream> {
    macro_rules! typed {
        ($sample:ty) => {
            build_typed::<$sample>(device, config, ring.clone(), ready.clone())?
        };
    }
    Ok(match sample_format {
        cpal::SampleFormat::I8 => typed!(i8),
        cpal::SampleFormat::I16 => typed!(i16),
        cpal::SampleFormat::I32 => typed!(i32),
        cpal::SampleFormat::I64 => typed!(i64),
        cpal::SampleFormat::U8 => typed!(u8),
        cpal::SampleFormat::U16 => typed!(u16),
        cpal::SampleFormat::U32 => typed!(u32),
        cpal::SampleFormat::U64 => typed!(u64),
        cpal::SampleFormat::F32 => typed!(f32),
        cpal::SampleFormat::F64 => typed!(f64),
        other => bail!("Unsupported audio sample format {other:?}"),
    })
}
fn build_typed<T>(
    device: &cpal::Device,
    config: &cpal::StreamConfig,
    ring: Arc<Mutex<Ring>>,
    ready: Arc<Condvar>,
) -> Result<cpal::Stream>
where
    T: cpal::SizedSample + cpal::FromSample<i16> + Send + 'static,
{
    let stream = device.build_output_stream(
        config,
        move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
            let mut guard = match ring.try_lock() {
                Ok(guard) => guard,
                Err(TryLockError::Poisoned(error)) => error.into_inner(),
                Err(TryLockError::WouldBlock) => {
                    for sample in data {
                        *sample = T::EQUILIBRIUM;
                    }
                    return;
                }
            };
            guard.pop(data);
            drop(guard);
            ready.notify_one();
        },
        move |error| log::error!("Audio output stream error: {error}"),
        None,
    )?;
    Ok(stream)
}
