//! Streaming PCM output with bounded queued buffers and a device sample clock.
use anyhow::{ensure, Result};
#[cfg(windows)]
use anyhow::Context;
#[cfg(windows)]
mod windows {
    use super::*;
    use std::{cell::UnsafeCell, collections::VecDeque};
    #[repr(C, packed)]
    struct Format {
        tag: u16,
        channels: u16,
        rate: u32,
        bytes: u32,
        align: u16,
        bits: u16,
        extra: u16,
    }
    #[repr(C)]
    struct Header {
        data: *mut i8,
        length: u32,
        recorded: u32,
        user: usize,
        flags: u32,
        loops: u32,
        next: *mut Header,
        reserved: usize,
    }
    #[repr(C)]
    struct Time {
        kind: u32,
        value: [u32; 2],
    }
    #[link(name = "winmm")]
    extern "system" {
        fn waveOutOpen(
            h: *mut usize,
            device: u32,
            f: *const Format,
            callback: usize,
            instance: usize,
            flags: u32,
        ) -> u32;
        fn waveOutPrepareHeader(h: usize, b: *mut Header, size: u32) -> u32;
        fn waveOutWrite(h: usize, b: *mut Header, size: u32) -> u32;
        fn waveOutUnprepareHeader(h: usize, b: *mut Header, size: u32) -> u32;
        fn waveOutPause(h: usize) -> u32;
        fn waveOutRestart(h: usize) -> u32;
        fn waveOutGetPosition(h: usize, t: *mut Time, size: u32) -> u32;
        fn waveOutReset(h: usize) -> u32;
        fn waveOutClose(h: usize) -> u32;
    }
    struct Buffer {
        header: Box<UnsafeCell<Header>>,
        samples: Vec<i16>,
    }
    pub struct Stream {
        handle: usize,
        buffers: VecDeque<Buffer>,
        pub rate: u32,
        pub channels: u16,
        pub submitted: u64,
        pub started: bool,
    }
    impl Stream {
        pub fn open(rate: u32, channels: u16) -> Result<Self> {
            ensure!(rate > 0 && channels > 0, "Invalid movie audio format");
            let align = channels
                .checked_mul(2)
                .context("Audio alignment overflow")?;
            let format = Format {
                tag: 1,
                channels,
                rate,
                bytes: rate
                    .checked_mul(u32::from(align))
                    .context("Audio rate overflow")?,
                align,
                bits: 16,
                extra: 0,
            };
            let mut handle = 0;
            ensure!(
                unsafe { waveOutOpen(&mut handle, u32::MAX, &format, 0, 0, 0) } == 0,
                "Movie audio device open failed"
            );
            let mut stream = Self {
                handle,
                buffers: VecDeque::new(),
                rate,
                channels,
                submitted: 0,
                started: false,
            };
            if unsafe { waveOutPause(handle) } != 0 {
                stream.close();
                anyhow::bail!("Movie audio device pause failed");
            }
            Ok(stream)
        }
        pub fn queue(&mut self, mut samples: Vec<i16>) -> Result<()> {
            ensure!(
                samples.len() % usize::from(self.channels) == 0,
                "Movie audio chunk is not frame-aligned"
            );
            let length = u32::try_from(
                samples
                    .len()
                    .checked_mul(2)
                    .context("Audio buffer overflow")?,
            )?;
            if length == 0 {
                return Ok(());
            }
            let b = Buffer {
                header: Box::new(UnsafeCell::new(Header {
                    data: samples.as_mut_ptr().cast(),
                    length,
                    recorded: 0,
                    user: 0,
                    flags: 0,
                    loops: 0,
                    next: std::ptr::null_mut(),
                    reserved: 0,
                })),
                samples,
            };
            ensure!(
                unsafe {
                    waveOutPrepareHeader(
                        self.handle,
                        b.header.get(),
                        std::mem::size_of::<Header>() as u32,
                    )
                } == 0,
                "Movie audio header preparation failed"
            );
            let result = unsafe {
                waveOutWrite(
                    self.handle,
                    b.header.get(),
                    std::mem::size_of::<Header>() as u32,
                )
            };
            if result != 0 {
                if unsafe {
                    waveOutUnprepareHeader(
                        self.handle,
                        b.header.get(),
                        std::mem::size_of::<Header>() as u32,
                    )
                } != 0
                {
                    std::mem::forget(b);
                }
                anyhow::bail!("Movie audio write failed ({result})");
            }
            self.submitted += b.samples.len() as u64 / u64::from(self.channels);
            self.buffers.push_back(b);
            Ok(())
        }
        pub fn play(&mut self) -> Result<()> {
            if !self.started {
                ensure!(
                    unsafe { waveOutRestart(self.handle) } == 0,
                    "Movie audio restart failed"
                );
                self.started = true;
            }
            Ok(())
        }
        pub fn poll(&mut self) -> Result<()> {
            while self.buffers.front().is_some_and(|b|unsafe{std::ptr::read_volatile(std::ptr::addr_of!((*b.header.get()).flags))}&1!=0){
                let b=self.buffers.pop_front().unwrap();let result=unsafe{waveOutUnprepareHeader(self.handle,b.header.get(),std::mem::size_of::<Header>() as u32)};
                if result!=0{std::mem::forget(b);anyhow::bail!("Movie audio header release failed ({result})");}
            }
            Ok(())
        }
        pub fn position(&self) -> Result<u64> {
            if !self.started {
                return Ok(0);
            }
            let mut time = Time {
                kind: 2,
                value: [0; 2],
            };
            ensure!(
                unsafe {
                    waveOutGetPosition(self.handle, &mut time, std::mem::size_of::<Time>() as u32)
                } == 0,
                "Movie audio clock query failed"
            );
            Ok(match time.kind {
                2 => u64::from(time.value[0]),
                1 => u64::from(time.value[0]) * u64::from(self.rate) / 1000,
                4 => u64::from(time.value[0]) / (u64::from(self.channels) * 2),
                _ => anyhow::bail!("Unsupported movie audio clock format"),
            })
        }
        pub fn empty(&self) -> bool {
            self.buffers.is_empty()
        }
        pub fn queued_frames(&self) -> Result<u64> {
            Ok(self.submitted.saturating_sub(self.position()?))
        }
        fn close(&mut self) {
            if self.handle == 0 {
                return;
            }
            unsafe {
                waveOutReset(self.handle);
            }
            while let Some(b) = self.buffers.pop_front() {
                if unsafe {
                    waveOutUnprepareHeader(
                        self.handle,
                        b.header.get(),
                        std::mem::size_of::<Header>() as u32,
                    )
                } != 0
                {
                    log::error!("Movie driver retained audio buffer");
                    std::mem::forget(b);
                }
            }
            if unsafe { waveOutClose(self.handle) } != 0 {
                log::error!("Movie audio device close failed");
            }
            self.handle = 0;
        }
    }
    impl Drop for Stream {
        fn drop(&mut self) {
            self.close();
        }
    }
}
#[cfg(windows)]
pub use windows::Stream;
#[cfg(target_os = "macos")]
mod macos {
    use super::super::macos_audio;
    use super::*;
    use std::time::Duration;

    pub struct Stream {
        output: macos_audio::Output,
        pub rate: u32,
        pub channels: u16,
        pub submitted: u64,
        pub started: bool,
    }
    impl Stream {
        pub fn open(rate: u32, channels: u16) -> Result<Self> {
            ensure!(rate > 0 && channels > 0, "Invalid movie audio format");
            let capacity = (rate as usize).saturating_mul(2);
            let output = macos_audio::Output::open(rate, channels, capacity, false)?;
            Ok(Self {
                output,
                rate,
                channels,
                submitted: 0,
                started: false,
            })
        }
        pub fn queue(&mut self, samples: Vec<i16>) -> Result<()> {
            ensure!(
                samples.len() % usize::from(self.channels) == 0,
                "Movie audio chunk is not frame-aligned"
            );
            if samples.is_empty() {
                return Ok(());
            }
            self.submitted = self.output.push(&samples, Duration::from_millis(100))?;
            Ok(())
        }
        pub fn play(&mut self) -> Result<()> {
            if !self.started {
                self.output.start()?;
                self.started = true;
            }
            Ok(())
        }
        pub fn poll(&mut self) -> Result<()> {
            Ok(())
        }
        pub fn position(&self) -> Result<u64> {
            Ok(if self.started {
                self.output.position_frames()
            } else {
                0
            })
        }
        pub fn empty(&self) -> bool {
            self.output.consumed_frames() >= self.submitted
        }
        pub fn queued_frames(&self) -> Result<u64> {
            Ok(self.submitted.saturating_sub(self.position()?))
        }
    }
}
#[cfg(target_os = "macos")]
pub use macos::Stream;
#[cfg(not(any(windows, target_os = "android", target_os = "macos")))]
pub struct Stream {
    pub rate: u32,
    pub channels: u16,
    pub submitted: u64,
    pub started: bool,
}
#[cfg(not(any(windows, target_os = "android", target_os = "macos")))]
impl Stream {
    pub fn open(_: u32, _: u16) -> Result<Self> {
        anyhow::bail!("Movie audio output is not implemented on this platform")
    }
    pub fn queue(&mut self, _: Vec<i16>) -> Result<()> {
        unreachable!()
    }
    pub fn play(&mut self) -> Result<()> {
        unreachable!()
    }
    pub fn poll(&mut self) -> Result<()> {
        unreachable!()
    }
    pub fn position(&self) -> Result<u64> {
        unreachable!()
    }
    pub fn empty(&self) -> bool {
        true
    }
    pub fn queued_frames(&self) -> Result<u64> {
        unreachable!()
    }
}

#[cfg(target_os = "android")]
#[path = "movie_audio_android.rs"]
mod android;
#[cfg(target_os = "android")]
pub use android::Stream;
