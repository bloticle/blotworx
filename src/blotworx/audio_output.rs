//! Shared host PCM endpoint with native-quantum global SIS mute control.
//! Admission, spatialization and buses remain separate. Resampling is linear, a
//! documented discrepancy from the original Wii mixer.
use super::audio_playback::Clip;
use anyhow::{ensure, Context, Result};
#[cfg(not(target_os = "horizon"))]
use std::sync::atomic::AtomicU64;
#[cfg(target_os = "horizon")]
#[derive(Default)]
struct AtomicU64(Mutex<u64>);
#[cfg(target_os = "horizon")]
impl AtomicU64 {
    fn new(v: u64) -> Self { Self(Mutex::new(v)) }
    fn load(&self, _: Ordering) -> u64 { *self.0.lock().unwrap_or_else(|e| e.into_inner()) }
    fn store(&self, v: u64, _: Ordering) { *self.0.lock().unwrap_or_else(|e| e.into_inner()) = v; }
    fn fetch_add(&self, v: u64, _: Ordering) -> u64 {
        let mut g = self.0.lock().unwrap_or_else(|e| e.into_inner());
        let old = *g;
        *g = old.wrapping_add(v);
        old
    }
}
use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        mpsc, Arc, Condvar, Mutex,
    },
    thread,
    time::Duration,
};
const RATE: u32 = 48_000;
const FRAMES: usize = 144; //96 native samples/32kHz =144 host samples/48kHz.
// Windows waits are not guaranteed to wake at the requested 2 ms interval.
// Twenty3ms blocks retain the previous60ms scheduler-jitter margin.
const QUEUED_BLOCKS: usize = 20;

struct Source {
    samples: Arc<[i16]>,
    reader:Option<super::audio_file::Reader>,silent:bool,
    rate: f64,
    gain: [f32; 2],
    base_gain: [f32;2],
    base_db: f32,
    pan: [f32;4],
    lineage: Arc<[u32]>,
    owner: Option<Arc<super::audio_events::RegisteredObject>>,
    mutes: Vec<(u32,f32)>,
    phase: f64,
    filter: Option<super::audio_filter::Stream>,
    layout:Option<super::audio_loop::Layout>,
    stop:Option<super::audio_transition::Mute>,
    attack: Option<super::audio_transition::Transition>,
    attack_gain: f32,
    base_filter:f32,sis_filter:f32,spatial_filter:f32,
    spatial:Option<[f32;4]>,
    state: Arc<State>,
}
struct State {
    cancelled: AtomicBool,
    paused: AtomicBool,
    level: std::sync::atomic::AtomicU32,
    finished: AtomicBool,
    rendered: AtomicU64,
}
pub struct Voice {
    state: Arc<State>,
}
impl Voice {
    pub(super) fn set_paused(&self, paused: bool) {
        self.state.paused.store(paused, Ordering::Release);
    }
    pub fn done(&self) -> bool {
        self.state.finished.load(Ordering::Acquire)
    }
    /// Host frames mixed, not a native source/device playback-position query.
    pub fn rendered_frames(&self) -> u64 {
        self.state.rendered.load(Ordering::Acquire)
    }
}
impl Drop for Voice {
    fn drop(&mut self) {
        self.state.cancelled.store(true, Ordering::Release);
    }
}
enum Command {
    Play(Source, Option<(u8, i32)>),
    Define(Vec<u32>),
    Mute(super::audio_mute_mix::Command),
    Spatial(Arc<State>,[f32;4]),
    Filter(Arc<State>,f32),
    Stop(Arc<State>,u8,i32),
    SisDefine(Vec<super::audio_sis::Definition>),
    Sis(super::audio_sis::Command),
    Shutdown,
}
#[derive(Default)]
struct Gate {
    blocked: Mutex<bool>,
    ready: Condvar,
    stopping: AtomicBool,
}
impl Gate {
    fn render(&self) -> Result<Option<std::sync::MutexGuard<'_, bool>>> {
        let mut blocked = self
            .blocked
            .lock()
            .map_err(|_| anyhow::anyhow!("Audio command-phase lock poisoned"))?;
        while *blocked && !self.stopping.load(Ordering::Acquire) {
            blocked = self
                .ready
                .wait(blocked)
                .map_err(|_| anyhow::anyhow!("Audio command-phase wait poisoned"))?;
        }
        Ok(if self.stopping.load(Ordering::Acquire) {
            None
        } else {
            Some(blocked)
        })
    }
    fn stop(&self) {
        let _blocked = self
            .blocked
            .lock()
            .unwrap_or_else(|error| error.into_inner());
        self.stopping.store(true, Ordering::Release);
        self.ready.notify_all();
    }
}
/// Owns the gate rather than borrowing Output, so the concrete node host can
/// use Output while command execution holds rendering. Drop also releases it
/// on an early error. This is a render barrier, not rollback of prior actions.
pub struct CommandPhase {
    gate: Arc<Gate>,
}
impl Drop for CommandPhase {
    fn drop(&mut self) {
        let mut blocked = self
            .gate
            .blocked
            .lock()
            .unwrap_or_else(|error| error.into_inner());
        *blocked = false;
        self.gate.ready.notify_all();
    }
}
struct Worker {
    sender: mpsc::Sender<Command>,
    thread: Option<thread::JoinHandle<()>>,
    error: Arc<Mutex<Option<String>>>,
    gate: Arc<Gate>,
}
impl Drop for Worker {
    fn drop(&mut self) {
        let _ = self.sender.send(Command::Shutdown);
        self.gate.stop();
        if let Some(thread) = self.thread.take() {
            let _ = thread.join();
        }
        // A failed device-open attempt may be retried on the same Output.
        self.gate.stopping.store(false, Ordering::Release);
    }
}
#[derive(Default)]
pub struct Output {
    worker: Option<Worker>,
    gate: Arc<Gate>,
    clock: Arc<AtomicU64>,
    mute_nodes: std::collections::BTreeSet<u32>,
    sis_nodes:std::collections::BTreeMap<u32,Result<super::audio_sis::Definition,String>>,
}
impl Output {
    pub(super) fn set_level(&mut self, voice: &Voice, level: f32) -> Result<()> {
        ensure!(level.is_finite() && (0. ..=1.).contains(&level), "Invalid voice level");
        voice.state.level.store(level.to_bits(), Ordering::Release);
        Ok(())
    }
    pub fn tick(&self)->u32 {self.clock.load(Ordering::Acquire) as u32}
    pub fn define_mute_nodes(&mut self,ids:impl IntoIterator<Item=u32>)->Result<()> {
        let ids=ids.into_iter().filter(|id|self.mute_nodes.insert(*id)).collect::<Vec<_>>();
        if let Some(worker)=&self.worker {worker.sender.send(Command::Define(ids)).map_err(|_|anyhow::anyhow!("Audio worker disconnected"))?;}
        Ok(())
    }
    pub fn has_mute_node(&self,id:u32)->bool {self.mute_nodes.contains(&id)}
    pub fn define_sis_nodes(&mut self,nodes:impl IntoIterator<Item=(u32,Result<super::audio_sis::Definition,String>)>)->Result<()> {
        let mut definitions=Vec::new();for (id,node) in nodes {if let std::collections::btree_map::Entry::Vacant(entry)=self.sis_nodes.entry(id) {if let Ok(d)=&node {definitions.push(*d);}entry.insert(node);}}
        if let Some(w)=&self.worker {w.sender.send(Command::SisDefine(definitions)).map_err(|_|anyhow::anyhow!("Audio worker disconnected"))?;}Ok(())
    }
    pub fn has_sis_node(&self,id:u32)->Result<bool> {match self.sis_nodes.get(&id){Some(Ok(_))=>Ok(true),Some(Err(e))=>anyhow::bail!("Unsupported SIS node {id:08x}: {e}"),None=>Ok(false)}}
    pub fn sis(&mut self,c:super::audio_sis::Command)->Result<()> {self.ensure_worker()?;self.worker.as_ref().unwrap().sender.send(Command::Sis(c)).map_err(|_|anyhow::anyhow!("Audio worker disconnected"))}
    pub fn mute(&mut self,command:super::audio_mute_mix::Command)->Result<()> {
        self.ensure_worker()?;
        self.worker.as_ref().unwrap().sender.send(Command::Mute(command)).map_err(|_|anyhow::anyhow!("Audio worker disconnected"))
    }
    pub fn begin_commands(&self) -> Result<CommandPhase> {
        self.check()?;
        let mut blocked = self
            .gate
            .blocked
            .lock()
            .map_err(|_| anyhow::anyhow!("Audio command-phase lock poisoned"))?;
        ensure!(!*blocked, "Nested audio command phases are not supported");
        *blocked = true;
        Ok(CommandPhase {
            gate: self.gate.clone(),
        })
    }
    pub fn check(&self) -> Result<()> {
        if let Some(worker) = &self.worker {
            let error = worker
                .error
                .lock()
                .map_err(|_| anyhow::anyhow!("Audio output error lock poisoned"))?;
            if let Some(error) = error.as_ref() {
                anyhow::bail!("{error}");
            }
            ensure!(
                !worker
                    .thread
                    .as_ref()
                    .is_some_and(|thread| thread.is_finished()),
                "Audio output worker terminated unexpectedly"
            );
        }
        Ok(())
    }
    fn ensure_worker(&mut self)->Result<()> {
        self.check()?;
        if self.worker.is_none() {
            let (sender, receiver) = mpsc::channel();
            let (ready_tx, ready_rx) = mpsc::sync_channel(1);
            let error = Arc::new(Mutex::new(None));
            let errors = error.clone();
            let gate = self.gate.clone();
            let clock=self.clock.clone();
            let mute_nodes=self.mute_nodes.iter().copied().collect::<Vec<_>>();
            let sis_nodes=self.sis_nodes.values().filter_map(|v|v.as_ref().ok().copied()).collect::<Vec<_>>();
            let thread = thread::Builder::new()
                .name("Blotworx audio".into())
                .spawn(move || {
                    let endpoint = endpoint::Endpoint::open();
                    match endpoint {
                        Err(error) => {
                            let _ = ready_tx.send(Err(format!("{error:#}")));
                        }
                        Ok(endpoint) => {
                            let _ = ready_tx.send(Ok(()));
                            if let Err(error) = run(receiver, endpoint, &gate,mute_nodes,sis_nodes,&clock) {
                                if let Ok(mut out) = errors.lock() {
                                    *out = Some(format!("{error:#}"));
                                }
                            }
                        }
                    }
                })?;
            let mut worker = Worker {
                sender,
                thread: Some(thread),
                error,
                gate: self.gate.clone(),
            };
            let ready = ready_rx
                .recv()
                .context("Audio worker stopped during startup")?;
            if let Err(error) = ready {
                if let Some(thread) = worker.thread.take() {
                    let _ = thread.join();
                }
                anyhow::bail!("{error}");
            }
            self.worker = Some(worker);
        }
        Ok(())
    }
    pub fn start(&mut self, clip: &Clip) -> Result<Voice> {
        ensure!(clip.spatial.is_none(),"Spatial voice requires resolved emitter/listener channels");
        self.start_spatial(clip,None)
    }
    pub fn low_pass(&mut self,voice:&Voice,value:f32)->Result<()> {
        super::audio_filter::coefficients(value)?;
        if let Some(worker)=&self.worker {worker.sender.send(Command::Filter(voice.state.clone(),value)).map_err(|_|anyhow::anyhow!("Audio output worker disconnected"))?;}Ok(())
    }
    pub fn stop(&mut self,voice:&Voice,curve:u8,duration:i32)->Result<()> {
        ensure!(curve<=8,"Unsupported stop transition curve");
        if let Some(worker)=&self.worker {worker.sender.send(Command::Stop(voice.state.clone(),curve,duration)).map_err(|_|anyhow::anyhow!("Audio output worker disconnected"))?;}Ok(())
    }
    pub fn spatial(&mut self,voice:&Voice,channels:[f32;4])->Result<()> {
        if let Some(worker)=&self.worker {worker.sender.send(Command::Spatial(voice.state.clone(),channels)).map_err(|_|anyhow::anyhow!("Audio output worker disconnected"))?;}Ok(())
    }
    pub fn start_spatial(&mut self, clip: &Clip,spatial:Option<[f32;4]>) -> Result<Voice> {
        self.start_spatial_filtered(clip,spatial,0.)
    }
    pub fn start_spatial_filtered(&mut self,clip:&Clip,spatial:Option<[f32;4]>,spatial_filter:f32)->Result<Voice> {
        self.start_with_fade(clip, spatial, spatial_filter, None)
    }
    pub fn start_with_fade(
        &mut self,
        clip: &Clip,
        spatial: Option<[f32; 4]>,
        spatial_filter: f32,
        fade: Option<(u8, i32)>,
    ) -> Result<Voice> {
        self.start_owned(clip, spatial, spatial_filter, fade, None)
    }
    pub(super) fn start_owned(
        &mut self,
        clip: &Clip,
        spatial: Option<[f32; 4]>,
        spatial_filter: f32,
        fade: Option<(u8, i32)>,
        owner: Option<Arc<super::audio_events::RegisteredObject>>,
    ) -> Result<Voice> {
        ensure!(fade.is_none_or(|(curve, duration)| curve <= 8 && duration > 0), "Invalid voice fade-in");
        if matches!(&clip.signal,super::audio_signal::Signal::Pcm) {if let Some(layout)=clip.layout {ensure!(layout.samples<=clip.samples.len(),"Authored sample count exceeds PCM");}}
        ensure!(
            clip.rate > 0 && (!clip.samples.is_empty() || (!matches!(&clip.signal,super::audio_signal::Signal::Pcm) && clip.layout.is_some())) && clip.pitch_ratio.is_finite() && clip.pitch_ratio>0.,
            "Invalid PCM source"
        );
        let gain = super::audio_panner::output_gain(clip.gain_db.min(0.))?;
        ensure!(
            clip.channel_gains.iter().all(|value| value.is_finite()),
            "Nonfinite PCM channel gain"
        );
        let gain =if let Some(channels)=spatial {super::audio_spatial::stereo(clip.gain_db.min(0.),channels)?}else {
            super::audio_panner::stereo_fold(clip.channel_gains.map(|channel| channel * gain))?};
        let mut filter=Some(super::audio_filter::Stream::dynamic(clip.low_pass.max(spatial_filter),clip.rate as f64*clip.pitch_ratio as f64)?);
        if let Some(filter)=&mut filter {filter.set_layout(clip.layout);}
        let reader=match &clip.signal {
            super::audio_signal::Signal::File(spec)=>Some(spec.open(clip.layout.context("Stream layout missing")?)?),
            super::audio_signal::Signal::MusicFile { spec, offset, lead, length } => Some(spec.open_window(*offset, *lead, *length)?),
            super::audio_signal::Signal::FilePending(_)=>anyhow::bail!("External audio was not prepared"),
            _=>None
        };
        self.ensure_worker()?;
        let state = Arc::new(State {
            cancelled: AtomicBool::new(false),
            paused: AtomicBool::new(false),
            level: std::sync::atomic::AtomicU32::new(1f32.to_bits()),
            finished: AtomicBool::new(false),
            rendered: AtomicU64::new(0),
        });
        self.worker
            .as_ref()
            .unwrap()
            .sender
            .send(Command::Play(Source {
                owner,
                samples: clip.samples.clone(),reader,silent:matches!(&clip.signal,super::audio_signal::Signal::Silence(_)),
                rate: clip.rate as f64 * clip.pitch_ratio as f64,
                gain,
                base_gain:gain,base_db:clip.gain_db,pan:clip.channel_gains,lineage:clip.lineage.clone(),mutes:Vec::new(),
                phase: 0.,
                filter,layout:clip.layout,stop:None,
                attack: None, attack_gain: 1.,
                base_filter:clip.low_pass,sis_filter:0.,spatial_filter,
                spatial,
                state: state.clone(),
            }, fade))
            .map_err(|_| anyhow::anyhow!("Audio output worker disconnected"))?;
        Ok(Voice { state })
    }
}
/// Retain fractional pitched source time across blocks. Unity pitch keeps the
/// previous integer numerator exactly. Host interpolation is not native SRC.
#[cfg(test)]
fn mix(sources:&mut Vec<Source>,frames:usize)->(Vec<i16>,Vec<Arc<State>>){mix_checked(sources,frames).expect("PCM test mixer")}
fn read_sample(samples:&[i16],layout:Option<super::audio_loop::Layout>,reader:&mut Option<super::audio_file::Reader>,silent:bool,time:u64)->Result<Option<i16>> {
    if let Some(reader)=reader {reader.sample(time)}else if silent {Ok(layout.and_then(|l|l.index(time)).map(|_|0))}else {Ok(super::audio_loop::sample(samples,layout,time))}
}
fn mix_checked(sources:&mut Vec<Source>,frames:usize)->Result<(Vec<i16>,Vec<Arc<State>>)> {
    let mut mixed = vec![[0f32; 2]; frames];
    let mut ended = Vec::new();
    let mut failure=None;
    sources.retain_mut(|source| {
      let result=(||->Result<bool>{
        if source.state.cancelled.load(Ordering::Acquire) || source.stop.as_ref().is_some_and(|fade|fade.transition.is_none()) {
            ended.push(source.state.clone());
            return Ok(false);
        }
        if source.state.paused.load(Ordering::Acquire) { return Ok(true); }
        let level = f32::from_bits(source.state.level.load(Ordering::Acquire));
        let mut rendered = 0;
        for output in &mut mixed {
            let index = (source.phase / RATE as f64) as u64;
            let duration=source.layout.map_or(Some(source.samples.len() as u64),|l|l.duration());if duration.is_some_and(|n|index>=n){break;}
            let sample=if let Some(filter)=&mut source.filter {
                let samples=&source.samples;let layout=source.layout;let reader=&mut source.reader;let silent=source.silent;
                filter.sample_with(&mut |time|read_sample(samples,layout,reader,silent,time))?
            }else {
                let Some(a)=read_sample(&source.samples,source.layout,&mut source.reader,source.silent,index)? else {break};
                let b=read_sample(&source.samples,source.layout,&mut source.reader,source.silent,index+1)?.unwrap_or(a);
                let fraction=(source.phase%RATE as f64) as f32/RATE as f32;a as f32+(b as f32-a as f32)*fraction
            };
            for channel in 0..2 {
                output[channel] += sample * source.gain[channel] * source.attack_gain * level;
            }
            source.phase += source.rate;
            rendered += 1;
        }
        source.state.rendered.fetch_add(rendered, Ordering::Release);
        let duration=source.layout.map_or(Some(source.samples.len() as u64),|layout|layout.duration());
        if duration.is_some_and(|end|source.phase / RATE as f64 >= end as f64) {
            ended.push(source.state.clone());
            Ok(false)
        } else {Ok(true)}
      })();match result {Ok(keep)=>keep,Err(error)=>{failure=Some(error);false}}
    });
    if let Some(error)=failure {return Err(error);}
    let pcm = mixed
        .into_iter()
        .flat_map(|frame| {
            frame.map(|value| value.round().clamp(i16::MIN as f32, i16::MAX as f32) as i16)
        })
        .collect();
    Ok((pcm, ended))
}
fn apply_mutes(sources:&mut [Source],changes:&[(u32,f32)])->Result<()> {
    for source in sources {
        for &(id,value) in changes {
            if source.lineage.contains(&id) {super::audio_mute_mix::notify(&mut source.mutes,id,value);}
        }
        source.gain=if let Some(channels)=source.spatial {
            let db=if source.mutes.is_empty() && source.stop.is_none(){source.base_db.min(0.)}else {super::audio_mute_mix::gain_db_local(source.base_db,&source.mutes,source.stop.as_ref().map_or(255,|s|s.byte))?};
            super::audio_spatial::stereo(db,channels)?
        }else if source.mutes.is_empty() && source.stop.is_none() {source.base_gain}else {
            let gain=super::audio_panner::output_gain(super::audio_mute_mix::gain_db_local(source.base_db,&source.mutes,source.stop.as_ref().map_or(255,|s|s.byte))?)?;
            super::audio_panner::stereo_fold(source.pan.map(|channel|channel*gain))?
        };
    }
    Ok(())
}
fn update_filter(source:&mut Source)->Result<()> {
    let coefficients=super::audio_filter::coefficients((source.base_filter+source.sis_filter).max(source.spatial_filter))?.unwrap_or([32768,0]);
    if let Some(filter)=&mut source.filter {filter.set_coefficients(coefficients);}Ok(())
}
fn apply_parameters(source:&mut Source,changes:impl IntoIterator<Item=super::audio_sis::Change>)->Result<()> {
    let owner=source.owner.as_ref().map(|owner|owner.handle);
    for c in changes {if (c.owner.is_none() || c.owner==owner) && source.lineage.contains(&c.node) {match c.property {super::audio_sis::Property::Volume=>source.base_db+=c.delta,super::audio_sis::Property::LowPass=>source.sis_filter+=c.delta}}}
    let gain=super::audio_panner::output_gain(source.base_db.min(0.))?;source.base_gain=super::audio_panner::stereo_fold(source.pan.map(|v|v*gain))?;update_filter(source)
}
fn run(receiver: mpsc::Receiver<Command>, endpoint: endpoint::Endpoint, gate: &Gate,ids:Vec<u32>,sis_ids:Vec<super::audio_sis::Definition>,clock:&AtomicU64) -> Result<()> {
    use std::collections::VecDeque;
    let mut sources: Vec<Source> = Vec::new();
    let mut controls=super::audio_mute_mix::Runtime::new(ids)?;
    let tables=super::audio_transition::Tables::retail()?;
    let mut parameters=super::audio_sis::Runtime::default();parameters.define(sis_ids);
    fn receive(command:Command,sources:&mut Vec<Source>,controls:&mut super::audio_mute_mix::Runtime,parameters:&mut super::audio_sis::Runtime,tables:&super::audio_transition::Tables)->Result<bool> {
        match command {
            Command::Play(mut source, fade)=>{
                if let Some((curve, duration)) = fade {
                    source.attack = Some(super::audio_transition::Transition::new_linear(0., 1., curve, duration, controls.tick));
                    source.attack_gain = 0.;
                }
                source.mutes=controls.initial(&source.lineage);let initial=parameters.initial(&source.lineage,source.owner.as_ref().map(|owner|owner.handle));source.base_db+=initial[0];source.sis_filter=initial[1];apply_parameters(&mut source,[])?;sources.push(source);},
            Command::Define(ids)=>controls.define(ids),
            Command::Mute(command)=>controls.enqueue(command),
            Command::Spatial(state,channels)=>{if let Some(source)=sources.iter_mut().find(|source|Arc::ptr_eq(&source.state,&state)){source.spatial=Some(channels);}},
            Command::Filter(state,value)=>{if let Some(source)=sources.iter_mut().find(|source|Arc::ptr_eq(&source.state,&state)){source.spatial_filter=value;update_filter(source)?;}},
            Command::Stop(state,curve,duration)=>{if let Some(source)=sources.iter_mut().find(|source|Arc::ptr_eq(&source.state,&state)){source.stop.get_or_insert_with(Default::default).request(tables,true,curve,duration,controls.tick)?;}},
            Command::SisDefine(nodes)=>parameters.define(nodes),
            Command::Sis(c)=>{parameters.enqueue(c);let changes=parameters.dispatch(tables,controls.tick)?;for source in sources {apply_parameters(source,changes.iter().copied())?;}},
            Command::Shutdown=>return Ok(false),
        }
        Ok(true)
    }
    let mut queued: VecDeque<(endpoint::Buffer, Vec<Arc<State>>)> = VecDeque::new();
    loop {
        let mut retired = false;
        while queued.front().is_some_and(|(buffer, _)| buffer.done()) {
            retired = true;
            let (_, ended) = queued.pop_front().unwrap();
            for state in ended {
                state.finished.store(true, Ordering::Release);
            }
        }
        if retired
            && queued.is_empty()
            && sources
                .iter()
                .any(|source| !source.state.cancelled.load(Ordering::Acquire))
        {
            log::warn!("Audio output underrun: host PCM queue exhausted before refill");
        }
        if sources.is_empty() && queued.is_empty() && !controls.active() && !parameters.active() {
            if !receive(receiver.recv().unwrap_or(Command::Shutdown),&mut sources,&mut controls,&mut parameters,&tables)? {break;}
        }
        {
            let Some(_render) = gate.render()? else {
                return Ok(());
            };
            // Drain after the command phase releases its gate. In particular,
            // do not render an old source set before adding the committed Plays.
            while let Ok(command) = receiver.try_recv() {
                if !receive(command,&mut sources,&mut controls,&mut parameters,&tables)? {return Ok(());}
            }
            // Refill before waiting, retaining the scheduling margin which
            // fixed starvation with late Windows timed-wait wakeups.
            while queued.len() < QUEUED_BLOCKS && (!sources.is_empty() || controls.active() || parameters.active()) {
                let parameter_changes=parameters.advance(&tables,controls.tick)?;
                let changes=controls.advance()?;
                if !parameter_changes.is_empty() {for source in &mut sources {apply_parameters(source,parameter_changes.iter().copied())?;}}
                for source in &mut sources {
                    if let Some(fade) = &mut source.stop {
                        fade.update(&tables, controls.tick)?;
                    }
                    if let Some(fade) = &mut source.attack {
                        let (gain, complete) = fade.sample(&tables, controls.tick)?;
                        source.attack_gain = gain;
                        if complete {
                            source.attack = None;
                        }
                    }
                }
                apply_mutes(&mut sources,&changes)?;
                let (samples, ended) = mix_checked(&mut sources, FRAMES)?;
                queued.push_back((endpoint.submit(samples)?, ended));
                clock.store(controls.tick as u64,Ordering::Release);
            }
        }
        match receiver.recv_timeout(Duration::from_millis(2)) {
            Ok(command)=>if !receive(command,&mut sources,&mut controls,&mut parameters,&tables)? {break;},
            Err(mpsc::RecvTimeoutError::Disconnected) => break,
            Err(mpsc::RecvTimeoutError::Timeout) => {}
        }
    }
    Ok(())
}

#[cfg(windows)]
pub(super) mod endpoint {
    use super::*;
    use std::cell::UnsafeCell;
    #[repr(C, packed)]
    struct Format {
        tag: u16,
        channels: u16,
        rate: u32,
        bytes_per_second: u32,
        alignment: u16,
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
    #[link(name = "winmm")]
    extern "system" {
        fn waveOutOpen(
            handle: *mut usize,
            device: u32,
            format: *const Format,
            callback: usize,
            instance: usize,
            flags: u32,
        ) -> u32;
        fn waveOutPrepareHeader(handle: usize, header: *mut Header, size: u32) -> u32;
        fn waveOutWrite(handle: usize, header: *mut Header, size: u32) -> u32;
        fn waveOutReset(handle: usize) -> u32;
        fn waveOutUnprepareHeader(handle: usize, header: *mut Header, size: u32) -> u32;
        fn waveOutClose(handle: usize) -> u32;
    }
    struct Handle(usize);
    impl Drop for Handle {
        fn drop(&mut self) {
            let status = unsafe { waveOutClose(self.0) };
            if status != 0 {
                log::error!("Audio endpoint close failed ({status})");
            }
        }
    }
    pub struct Endpoint {
        handle: Arc<Handle>,
    }
    impl Endpoint {
        pub fn open() -> Result<Self> {
            let format = Format {
                tag: 1,
                channels: 2,
                rate: RATE,
                bytes_per_second: RATE * 4,
                alignment: 4,
                bits: 16,
                extra: 0,
            };
            let mut handle = 0;
            let status = unsafe { waveOutOpen(&mut handle, u32::MAX, &format, 0, 0, 0) };
            ensure!(status == 0, "Audio output device open failed ({status})");
            Ok(Self {
                handle: Arc::new(Handle(handle)),
            })
        }
        pub fn submit(&self, mut samples: Vec<i16>) -> Result<Buffer> {
            let length = u32::try_from(samples.len().checked_mul(2).context("PCM size overflow")?)?;
            let header = Box::new(UnsafeCell::new(Header {
                data: samples.as_mut_ptr().cast(),
                length,
                recorded: 0,
                user: 0,
                flags: 0,
                loops: 0,
                next: std::ptr::null_mut(),
                reserved: 0,
            }));
            let mut buffer = Buffer {
                handle: self.handle.clone(),
                header: Some(header),
                samples: Some(samples),
                prepared: false,
            };
            let size = std::mem::size_of::<Header>() as u32;
            let status = unsafe {
                waveOutPrepareHeader(self.handle.0, buffer.header.as_ref().unwrap().get(), size)
            };
            ensure!(status == 0, "Audio buffer preparation failed ({status})");
            buffer.prepared = true;
            let status =
                unsafe { waveOutWrite(self.handle.0, buffer.header.as_ref().unwrap().get(), size) };
            ensure!(status == 0, "Audio buffer submission failed ({status})");
            Ok(buffer)
        }
    }
    pub struct Buffer {
        handle: Arc<Handle>,
        header: Option<Box<UnsafeCell<Header>>>,
        samples: Option<Vec<i16>>,
        prepared: bool,
    }
    impl Buffer {
        pub fn done(&self) -> bool {
            unsafe {
                std::ptr::read_volatile(std::ptr::addr_of!(
                    (*self.header.as_ref().unwrap().get()).flags
                )) & 1
                    != 0
            }
        }
    }
    impl Drop for Buffer {
        fn drop(&mut self) {
            if self.prepared && !self.done() {
                unsafe {
                    waveOutReset(self.handle.0);
                }
            }
            let status = if self.prepared {
                unsafe {
                    waveOutUnprepareHeader(
                        self.handle.0,
                        self.header.as_ref().unwrap().get(),
                        std::mem::size_of::<Header>() as u32,
                    )
                }
            } else {
                0
            };
            if status != 0 {
                log::error!(
                    "Audio buffer release failed ({status}); retaining driver-owned storage"
                );
                std::mem::forget(self.header.take());
                std::mem::forget(self.samples.take());
            }
        }
    }
}
#[cfg(target_os = "android")]
#[path = "audio_output_android.rs"]
pub(super) mod endpoint;

#[cfg(target_os = "macos")]
pub(super) mod endpoint {
    use super::*;
    use super::super::macos_audio;

    pub struct Endpoint {
        output: macos_audio::Output,
    }
    impl Endpoint {
        pub fn open() -> Result<Self> {
            let capacity = FRAMES * (QUEUED_BLOCKS + 4);
            let output = macos_audio::Output::open(RATE, 2, capacity, true)?;
            Ok(Self { output })
        }
        pub fn submit(&self, samples: Vec<i16>) -> Result<Buffer> {
            let handle = self.output.handle();
            let end = handle.push(&samples, Duration::from_millis(100))?;
            Ok(Buffer { handle, end })
        }
    }
    pub struct Buffer {
        handle: macos_audio::Handle,
        end: u64,
    }
    impl Buffer {
        pub fn done(&self) -> bool {
            self.handle.done(self.end)
        }
    }
}

#[cfg(not(any(windows, target_os = "android", target_os = "macos")))]
pub(super) mod endpoint {
    use super::*;
    pub struct Endpoint;
    pub struct Buffer;
    impl Endpoint {
        pub fn open() -> Result<Self> {
            anyhow::bail!("Native audio output unavailable on this platform")
        }
        pub fn submit(&self, _: Vec<i16>) -> Result<Buffer> {
            anyhow::bail!("Native audio output unavailable")
        }
    }
    impl Buffer {
        pub fn done(&self) -> bool {
            true
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn source(samples: &[i16], rate: u32, gain: f32) -> Source {
        Source {
            owner:None,
            samples: samples.to_vec().into(),reader:None,silent:false,
            rate: rate as f64,
            gain: [gain; 2],
            base_gain:[gain;2],base_db:0.,pan:[gain,gain,0.,0.],lineage:vec![].into(),mutes:vec![],
            phase: 0.,
            attack: None, attack_gain: 1.,
            filter: None,layout:None,stop:None,base_filter:0.,sis_filter:0.,spatial_filter:0.,
            spatial:None,
            state: Arc::new(State {
                cancelled: AtomicBool::new(false),
                paused: AtomicBool::new(false),
                level: std::sync::atomic::AtomicU32::new(1f32.to_bits()),
                finished: AtomicBool::new(false),
                rendered: AtomicU64::new(0),
            }),
        }
    }
    #[test]
    fn spatial_channel_updates_change_actual_pcm_without_resetting_voice_time()->Result<()> {
        let mut voice=source(&[1000;8],RATE,1.);voice.spatial=Some([0.,-97.,-97.,-97.]);
        let mut voices=vec![voice];apply_mutes(&mut voices,&[])?;assert_eq!(mix(&mut voices,2).0,[1000,0,1000,0]);
        voices[0].spatial=Some([-97.,0.,-97.,-97.]);apply_mutes(&mut voices,&[])?;assert_eq!(mix(&mut voices,2).0,[0,1000,0,1000]);
        assert_eq!(voices[0].phase,4.*RATE as f64);
        voices[0].mutes=vec![(7,0.)];apply_mutes(&mut voices,&[])?;assert_eq!(mix(&mut voices,2).0,[0;4]);Ok(())
    }
    #[test]
    fn global_mute_changes_actual_pcm_and_preserves_new_voice_quantization()->Result<()> {
        let mut controls=super::super::audio_mute_mix::Runtime::new([7])?;
        let mut existing=source(&[1000;16],RATE,1.);existing.lineage=vec![7].into();
        let mut voices=vec![existing];
        controls.enqueue(super::super::audio_mute_mix::Command{node:7,muting:true,curve:4,duration:0,due:0});
        apply_mutes(&mut voices,&controls.advance()?)?;
        assert_eq!(mix(&mut voices,4).0,vec![0;8]);
        controls.enqueue(super::super::audio_mute_mix::Command{node:7,muting:false,curve:4,duration:0,due:1});
        apply_mutes(&mut voices,&controls.advance()?)?;
        let fresh=controls.initial(&[7]);
        assert_eq!(fresh,[(7,254.)]);
        assert_eq!(voices[0].mutes[0].1.to_bits(),254.99998474121094f32.to_bits());
        assert!(mix(&mut voices,4).0.iter().all(|&sample|sample>990));Ok(())
    }
    #[test]
    fn pitched_source_preserves_fractional_clock_across_blocks() {
        let samples=[0,1000,2000,3000,4000,5000];
        let mut a=source(&samples,RATE,1.);a.rate*=0.5;
        let mut b=source(&samples,RATE,1.);b.rate*=0.5;
        let whole=mix(&mut vec![a],12).0;
        let mut split=vec![b];let mut pieces=mix(&mut split,5).0;pieces.extend(mix(&mut split,7).0);
        assert_eq!(whole,pieces);
        assert_eq!(&whole[..8],&[0,0,500,500,1000,1000,1500,1500]);
        assert!(split.is_empty());
    }
    #[test]
    fn procedural_silence_has_authored_duration_without_a_pcm_allocation()->Result<()> {
        let mut s=source(&[],48000,1.);s.silent=true;s.layout=Some(super::super::audio_loop::Layout::new(48,Some([0,48]),1)?);
        let mut voices=vec![s];assert_eq!(mix_checked(&mut voices,48)?.0,vec![0;96]);assert!(voices.is_empty());Ok(())
    }
    #[test]
    fn live_sis_changes_affect_only_descendants_and_keep_pcm_phase()->Result<()> {
        let samples=[1000,-1000,2000,-2000,3000,-3000,4000,-4000];
        let mut s=source(&samples,32000,1.);s.lineage=Arc::from([3,7]);s.base_filter=10.;s.spatial_filter=25.;s.filter=Some(super::super::audio_filter::Stream::dynamic(25.,32000.)?);
        let mut voices=vec![s];mix(&mut voices,3);let phase=voices[0].phase;
        apply_parameters(&mut voices[0],[super::super::audio_sis::Change{node:9,owner:None,property:super::super::audio_sis::Property::Volume,delta:-12.}])?;assert_eq!(voices[0].base_db,0.);
        apply_parameters(&mut voices[0],[super::super::audio_sis::Change{node:7,owner:None,property:super::super::audio_sis::Property::Volume,delta:-12.},super::super::audio_sis::Change{node:7,owner:None,property:super::super::audio_sis::Property::LowPass,delta:40.}])?;
        apply_mutes(&mut voices,&[])?;assert_eq!(voices[0].phase,phase);assert_eq!(voices[0].base_db,-12.);assert_eq!(voices[0].sis_filter,40.);assert!(voices[0].gain.iter().all(|g|*g<0.3));
        assert!(mix(&mut voices,3).0.iter().any(|v|*v!=0));assert!(voices[0].phase>phase);Ok(())
    }
    #[test]
    fn looped_pcm_preserves_intro_tail_pitch_and_filtered_packet_phase()->Result<()> {
        let samples=[1000,-1000,2000,-2000,3000,-3000,4000,-4000];
        let make=|filter:bool|->Result<Source>{let mut s=source(&samples,32000,1.);s.layout=Some(super::super::audio_loop::Layout::new(8,Some([2,5]),3)?);if filter {let mut stream=super::super::audio_filter::Stream::dynamic(35.,s.rate)?;stream.set_layout(s.layout);s.filter=Some(stream);}Ok(s)};
        for filtered in [false,true] {
            let mut whole=vec![make(filtered)?];let mut split=vec![make(filtered)?];
            let expected=mix(&mut whole,21).0;let mut actual=mix(&mut split,4).0;actual.extend(mix(&mut split,3).0);actual.extend(mix(&mut split,14).0);
            assert_eq!(actual,expected);assert!(whole.is_empty() && split.is_empty());
        }
        let mut source=make(false)?;source.layout=Some(super::super::audio_loop::Layout::new(8,Some([2,5]),0)?);
        let mut sources=vec![source];mix(&mut sources,10_000);assert_eq!(sources.len(),1);assert_eq!(sources[0].samples.len(),samples.len());Ok(())
    }
    #[test]
    fn filtered_pcm_uses_native_clock_and_survives_packet_boundaries()->Result<()> {
        let samples=[32767,-32768,0,0,1000,-1000,0,0];
        let make=||->Result<Source> {let mut s=source(&samples,32000,1.);s.filter=super::super::audio_filter::Stream::new(35.,32000.)?;Ok(s)};
        let mut whole=vec![make()?];let mut split=vec![make()?];
        let expected=mix(&mut whole,12).0;
        let mut actual=mix(&mut split,1).0;actual.extend(mix(&mut split,4).0);actual.extend(mix(&mut split,7).0);
        assert_eq!(actual,expected);assert_eq!(expected[0],15984);
        // At host sample3, two native samples elapsed. This is the third
        // filtered sample, independent of the host packet partition.
        let mut filter=super::super::audio_filter::Filter{coefficients:[15985,16783],history:0};
        filter.sample(samples[0]);filter.sample(samples[1]);assert_eq!(expected[6],filter.sample(samples[2]));
        assert!(split.is_empty());Ok(())
    }
    #[test]
    fn output_mixes_before_clipping_and_preserves_block_phase() {
        let mut sources = vec![
            source(&[30000; 8], RATE, 1.),
            source(&[30000; 8], RATE, 1.),
            source(&[-30000; 8], RATE, 1.),
        ];
        let (pcm, ended) = mix(&mut sources, 4);
        assert!(ended.is_empty());
        assert_eq!(pcm, vec![30000; 8]);
        let (pcm, ended) = mix(&mut sources, 4);
        assert_eq!(pcm, vec![30000; 8]);
        assert_eq!(ended.len(), 3);
        assert!(sources.is_empty());
        let samples = [0, 1000, 2000, 3000];
        let mut whole = vec![source(&samples, 16000, 1.)];
        let mut split = vec![source(&samples, 16000, 1.)];
        let expected = mix(&mut whole, 12).0;
        let mut actual = mix(&mut split, 5).0;
        actual.extend(mix(&mut split, 7).0);
        assert_eq!(actual, expected);
    }
    #[test]
    fn output_cancelled_source_does_not_silence_another_voice() {
        let cancelled = source(&[1000; 4], RATE, 1.);
        cancelled.state.cancelled.store(true, Ordering::Release);
        let (pcm, ended) = mix(&mut vec![cancelled, source(&[2000; 4], RATE, 1.)], 4);
        assert_eq!(pcm, vec![2000; 8]);
        assert_eq!(ended.len(), 2);
    }
    #[test]
    fn output_preserves_independent_left_and_right_gains() {
        let mut voice = source(&[1000; 4], RATE, 1.);
        voice.gain = [0.25, 0.75];
        let (pcm, ended) = mix(&mut vec![voice], 4);
        assert_eq!(pcm, [250, 750, 250, 750, 250, 750, 250, 750]);
        assert_eq!(ended.len(), 1);
    }
    #[test]
    fn stopped_worker_is_reported_even_without_a_returned_device_error() {
        let (sender, _receiver) = mpsc::channel();
        let thread = thread::spawn(|| {});
        while !thread.is_finished() {
            thread::yield_now();
        }
        let gate = Arc::<Gate>::default();
        let output = Output {
            worker: Some(Worker {
                sender,
                thread: Some(thread),
                error: Arc::new(Mutex::new(None)),
                gate: gate.clone(),
            }),
            gate,
            clock:Default::default(),mute_nodes:Default::default(),sis_nodes:Default::default(),
        };
        assert!(output
            .check()
            .unwrap_err()
            .to_string()
            .contains("terminated unexpectedly"));
    }
    #[test]
    fn command_phase_releases_on_error_and_rejects_nesting() -> Result<()> {
        let output = Output::default();
        let failed = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let _phase = output.begin_commands().unwrap();
            panic!("Injected command consumer failure");
        }));
        assert!(failed.is_err());
        let phase = output.begin_commands()?;
        assert!(output.begin_commands().is_err());
        drop(phase);
        let _next = output.begin_commands()?;
        Ok(())
    }
    #[cfg(windows)]
    #[test]
    #[ignore = "requires a live Windows audio output device"]
    fn original_device_command_phase_prevents_even_short_lived_source_rendering() -> Result<()> {
        let mut output = Output::default();
        let phase = output.begin_commands()?;
        let clip = Clip {
            samples: vec![1000; 4800].into(),
            rate: RATE,
            sound: 0,
            gain_db: 0.,
            channel_gains: [1., 1., 0., 0.],
            maximum_instances: 0,
            pitch_ratio: 1.,
            low_pass: 0.,
            pitch_cents:0.,random_ranges:Arc::from([]),pitch_controls:Arc::from([]),
            spatial:None,signal:super::super::audio_signal::Signal::Pcm,source_passes:1,layout:None,admission:None,
            lineage: vec![0].into(),
        };
        let cancelled = output.start(&clip)?;
        let state = cancelled.state.clone();
        thread::sleep(Duration::from_millis(150));
        assert_eq!(
            cancelled.rendered_frames(),
            0,
            "Source rendered while command phase was held"
        );
        assert!(!cancelled.done());
        drop(cancelled);
        let silent = Clip {
            samples: vec![0; 4800].into(),
            ..clip
        };
        let retained = output.start(&silent)?;
        drop(phase);
        let started = std::time::Instant::now();
        while !retained.done() {
            output.check()?;
            ensure!(
                started.elapsed() < Duration::from_secs(3),
                "Committed source did not finish"
            );
            thread::sleep(Duration::from_millis(5));
        }
        assert_eq!(
            state.rendered.load(Ordering::Acquire),
            0,
            "Cancelled source leaked samples after commit"
        );
        assert_eq!(retained.rendered_frames(), 4800);
        Ok(())
    }
    #[cfg(windows)]
    #[test]
    #[ignore = "requires a live Windows audio output device"]
    fn original_device_shutdown_does_not_wait_for_a_live_command_phase() -> Result<()> {
        let mut output = Output::default();
        let phase = output.begin_commands()?;
        let clip = Clip {
            samples: vec![0; 4800].into(),
            rate: RATE,
            sound: 0,
            gain_db: 0.,
            channel_gains: [1., 1., 0., 0.],
            maximum_instances: 0,
            pitch_ratio: 1.,
            low_pass: 0.,
            pitch_cents:0.,random_ranges:Arc::from([]),pitch_controls:Arc::from([]),
            spatial:None,signal:super::super::audio_signal::Signal::Pcm,source_passes:1,layout:None,admission:None,
            lineage: vec![0].into(),
        };
        let _voice = output.start(&clip)?;
        let (sender, receiver) = mpsc::channel();
        let shutdown = thread::spawn(move || {
            drop(output);
            let _ = sender.send(());
        });
        let result = receiver.recv_timeout(Duration::from_secs(2));
        drop(phase);
        shutdown.join().unwrap();
        ensure!(
            result.is_ok(),
            "Output shutdown waited for the command-phase guard"
        );
        Ok(())
    }
    #[cfg(windows)]
    #[test]
    #[ignore = "requires owned files and a live Windows audio output device"]
    fn original_device_shared_endpoint_finishes_only_after_pcm_playback() -> Result<()> {
        let root = std::env::var_os("BLOTWORX_TEST_DATA").context("Set BLOTWORX_TEST_DATA")?;
        let bytes = std::fs::read(std::path::Path::new(&root).join("Audio/Wii/Global.bnk"))?;
        let bank = super::super::audio_bank::Bank::parse(&bytes)?;
        let media = super::super::audio_dsp::Media::parse(bank.media[&0x17d3ec7f])?;
        let clip = Clip {
            samples: media.decode()?.into(),
            rate: media.sample_rate,
            sound: 0,
            gain_db: -10.,
            channel_gains: [1., 1., 0., 0.],
            maximum_instances: 0,
            pitch_ratio: 1.,
            low_pass: 0.,
            pitch_cents:0.,random_ranges:Arc::from([]),pitch_controls:Arc::from([]),
            spatial:None,signal:super::super::audio_signal::Signal::Pcm,source_passes:1,layout:None,admission:None,
            lineage: vec![0].into(),
        };
        let mut output = Output::default();
        let cancelled = output.start(&clip)?;
        let retained = output.start(&clip)?;
        assert!(!retained.done());
        drop(cancelled);
        let started = std::time::Instant::now();
        while !retained.done() {
            output.check()?;
            ensure!(
                started.elapsed() < Duration::from_secs(5),
                "PCM device never completed"
            );
            thread::sleep(Duration::from_millis(5));
        }
        ensure!(
            started.elapsed() >= Duration::from_millis(500),
            "Voice completed before its PCM duration"
        );
        output.check()?;
        Ok(())
    }
}
