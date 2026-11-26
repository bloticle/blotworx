//! Isolated, pinned game-audio codec worker. Scheduling/mixing stays in Rust.
//! Reads original bank files directly; no extracted WEMs or full PCM cache.
use anyhow::{ensure, Context, Result};
use std::{io::{Read, BufReader}, path::{Path, PathBuf}, process::{Child, ChildStdout, Stdio}};

#[cfg(target_os = "horizon")]
pub fn provision() -> Result<PathBuf> {
    anyhow::bail!("External audio codec binary is unsupported on Nintendo 3DS")
}

// vgmstream publishes a macOS build, while the Windows version is in the runtime cache
#[cfg(all(target_os = "macos", target_arch = "aarch64"))]
const PINNED: Option<super::movie_codec::Pinned> = Some(super::movie_codec::Pinned {
    url: "https://github.com/vgmstream/vgmstream/releases/download/r2168/vgmstream-mac.zip",
    archive_sha256: "91fb9da354211acf41c3ea5626c003df1562900be90cb84ad3db278165b43dcb",
    member: "vgmstream-cli",
    executable: "vgmstream-cli",
    executable_sha256: "db0a5b4d72d939c72a41b9f21db1ddb7262b21843becfd069c2f2f13411dd0e1",
});

#[cfg(all(target_os = "macos", not(target_arch = "aarch64")))]
const PINNED: Option<super::movie_codec::Pinned> = None;

#[cfg(target_os = "macos")]
pub fn provision() -> Result<PathBuf> {
    // The release pins a Windows vgmstream build
    static READY: std::sync::OnceLock<PathBuf> = std::sync::OnceLock::new();
    if let Some(path) = READY.get() {
        return Ok(path.clone());
    }
    let mut candidates = Vec::new();
    if let Some(path) = std::env::var_os("BLOTWORX_VGMSTREAM") {
        candidates.push(PathBuf::from(path));
    }
    if let Ok(exe) = std::env::current_exe() {
        if let Some(parent) = exe.parent() {
            candidates.push(parent.join("Codecs/vgmstream-cli"));
        }
    }
    if let Some(path) = std::env::var_os("PATH") {
        for directory in std::env::split_paths(&path) {
            candidates.push(directory.join("vgmstream-cli"));
        }
    }
    candidates.extend(
        [
            "/opt/homebrew/bin/vgmstream-cli",
            "/usr/local/bin/vgmstream-cli",
            "/usr/bin/vgmstream-cli",
        ]
        .map(PathBuf::from),
    );
    for candidate in candidates {
        if candidate.is_file() {
            let _ = READY.set(candidate.clone());
            return Ok(candidate);
        }
    }
    if let Some(pinned) = &PINNED {
        let path = super::movie_codec::install_pinned(pinned, "vgmstream-r2168", &format!("Pinned codec distribution: {}\nArchive SHA256: {}\nExecutable SHA256: {}\nSource and notices: https://github.com/vgmstream/vgmstream/tree/r2168\nThis worker decodes audio only.\n", pinned.url, pinned.archive_sha256, pinned.executable_sha256))?;
        let _ = READY.set(path.clone());
        return Ok(path);
    }
    anyhow::bail!(
        "vgmstream-cli could not be provisioned on this platform. Install vgmstream-cli or set BLOTWORX_VGMSTREAM to the executable."
    )
}

#[cfg(all(not(target_os = "horizon"), not(target_os = "macos")))]
pub fn provision() -> Result<PathBuf> {
    static READY: std::sync::OnceLock<PathBuf> = std::sync::OnceLock::new();
    static INSTALL: std::sync::Mutex<()> = std::sync::Mutex::new(());
    if let Some(path)=READY.get() {return Ok(path.clone());}
    let _guard=INSTALL.lock().map_err(|_|anyhow::anyhow!("Audio codec installation lock poisoned"))?;
    if let Some(path)=READY.get() {return Ok(path.clone());}
    ensure!(cfg!(all(windows,target_arch="x86_64")),"Audio codec provisioning currently requires Windows x64");
    let manifest:serde_json::Value=serde_json::from_str(include_str!("audio_codec_manifest.json"))?;
    let files=manifest["files"].as_object().context("Invalid audio codec manifest")?;
    let valid=|root:&Path|files.iter().all(|(name,sha)|std::fs::read(root.join(name)).is_ok_and(|bytes|super::movie_codec::digest(&bytes)==sha.as_str().unwrap_or_default()));
    let bundled=std::env::current_exe()?.parent().context("Executable folder missing")?.join("Codecs/vgmstream-r2168");
    let cache=dirs::cache_dir().context("No runtime codec cache directory")?.join("Blotworx Engine/runtime/vgmstream-r2168");
    let root=if valid(&bundled) {bundled} else {cache};
    if !valid(&root) {
        let bytes=ureq::get(manifest["url"].as_str().context("Missing audio codec URL")?).call()?.body_mut().with_config().limit(4_574_355+1).read_to_vec()?;
        ensure!(super::movie_codec::digest(&bytes)==manifest["sha256"].as_str().unwrap_or_default(),"Audio codec archive checksum mismatch");
        let mut zip=zip::ZipArchive::new(std::io::Cursor::new(bytes))?;
        std::fs::create_dir_all(&root)?;
        for (name,sha) in files {
            ensure!(!name.contains(['/', '\\', ':']) && name!="..","Invalid codec manifest filename");
            let destination=root.join(name);
            if std::fs::read(&destination).is_ok_and(|bytes|super::movie_codec::digest(&bytes)==sha.as_str().unwrap_or_default()) {continue;}
            let mut binary=Vec::new();zip.by_name(name)?.read_to_end(&mut binary)?;
            ensure!(super::movie_codec::digest(&binary)==sha.as_str().unwrap_or_default(),"Audio codec file checksum mismatch: {name}");
            let temporary=root.join(format!("{}.{}.tmp",name,std::process::id()));
            std::fs::write(&temporary,binary)?;
            // Another game process can finish the same pinned install first.
            if std::fs::read(&destination).is_ok_and(|bytes|super::movie_codec::digest(&bytes)==sha.as_str().unwrap_or_default()) {
                std::fs::remove_file(temporary)?;continue;
            }
            if destination.exists(){std::fs::remove_file(&destination)?;}
            std::fs::rename(temporary,destination)?;
        }
        ensure!(valid(&root),"Audio codec installation verification failed");
        std::fs::write(root.join("SOURCE.txt"),format!("Pinned codec distribution: {}\nArchive SHA256: {}\nSource and notices: https://github.com/vgmstream/vgmstream/tree/r2168\nBundled library information: README.md, USAGE.md and COPYING\nThis worker decodes audio only; it does not execute game code or implement Wwise events.\n",manifest["url"].as_str().unwrap(),manifest["sha256"].as_str().unwrap()))?;
    }
    let executable=root.join("vgmstream-cli.exe");let _=READY.set(executable.clone());Ok(executable)
}

#[derive(Clone, Debug, serde::Serialize)]
pub struct Source { pub bank:PathBuf, pub subsong:u32, pub media_id:u32 }
#[derive(Clone, Debug, serde::Serialize)]
pub struct Info { pub rate:u32, pub channels:u16, pub frames:u64, pub media_id:u32 }
pub struct Decoder {
    pub info:Info,
    child:Child,
    output:BufReader<ChildStdout>,
    remaining:u64,
}
impl Decoder {
    pub fn open(source:&Source)->Result<Self> {
        ensure!(source.subsong>0,"Audio bank subsong must be positive");
        let executable=provision()?;
        let result=super::movie_codec::command(&executable).args(["-I","-m","-s"]).arg(source.subsong.to_string()).arg(&source.bank).output()?;
        ensure!(result.status.success(),"Audio codec metadata failed: {}",String::from_utf8_lossy(&result.stderr));
        let metadata:serde_json::Value=serde_json::from_slice(&result.stdout).context("Invalid audio codec metadata")?;
        let media_id=metadata["streamInfo"]["name"].as_str().context("Audio codec omitted media identity")?.parse::<u32>()?;
        ensure!(media_id==source.media_id,"Bank subsong resolved to media {media_id}, expected {}",source.media_id);
        let mut command = super::movie_codec::command(&executable);
        command.args(["-i", "-p", "-B", "2048", "-s"]).arg(source.subsong.to_string()).arg(&source.bank);
        Self::start(command, &metadata, media_id)
    }

    pub(super) fn open_file(path: &Path, media_id: u32) -> Result<Self> {
        let executable = provision()?;
        let result = super::movie_codec::command(&executable).args(["-I", "-m"]).arg(path).output()?;
        ensure!(result.status.success(), "Audio file codec metadata failed: {}", String::from_utf8_lossy(&result.stderr));
        let metadata: serde_json::Value = serde_json::from_slice(&result.stdout).context("Invalid audio file codec metadata")?;
        let mut command = super::movie_codec::command(&executable);
        command.args(["-i", "-p", "-B", "2048"]).arg(path);
        Self::start(command, &metadata, media_id)
    }

    fn start(mut command: std::process::Command, metadata: &serde_json::Value, media_id: u32) -> Result<Self> {
        let info=Info {rate:u32::try_from(metadata["sampleRate"].as_u64().context("Missing sample rate")?)?,channels:u16::try_from(metadata["channels"].as_u64().context("Missing channel count")?)?,frames:metadata["numberOfSamples"].as_u64().context("Missing sample count")?,media_id};
        ensure!(info.rate>0 && info.channels>0,"Invalid decoded audio format");
        let mut child=command.stdout(Stdio::piped()).stderr(Stdio::null()).spawn()?;
        let output=BufReader::new(child.stdout.take().context("Missing audio codec output")?);
        let mut decoder=Self {info,child,output,remaining:0};
        decoder.read_header()?;
        Ok(decoder)
    }
    fn read_header(&mut self)->Result<()> {
        let mut riff=[0;12];self.output.read_exact(&mut riff)?;
        ensure!(&riff[..4]==b"RIFF" && &riff[8..]==b"WAVE","Audio codec did not produce RIFF/WAVE");
        let mut format=false;
        loop {
            let mut chunk=[0;8];self.output.read_exact(&mut chunk)?;
            let size=u32::from_le_bytes(chunk[4..].try_into()?) as u64;
            match &chunk[..4] {
                b"fmt "=>{
                    ensure!(size>=16,"Truncated decoded audio format");
                    let mut fmt=[0;16];self.output.read_exact(&mut fmt)?;
                    let word=|i|u16::from_le_bytes([fmt[i],fmt[i+1]]);
                    ensure!(word(0)==1 && word(2)==self.info.channels && word(14)==16,"Unexpected decoded PCM format");
                    ensure!(u32::from_le_bytes(fmt[4..8].try_into()?)==self.info.rate,"Decoded audio rate differs from metadata");
                    let extra=size-16+(size&1);
                    let consumed=std::io::copy(&mut self.output.by_ref().take(extra),&mut std::io::sink())?;
                    ensure!(consumed==extra,"Truncated decoded format extension");
                    format=true;
                }
                b"data"=>{
                    ensure!(format,"Decoded PCM data precedes format");
                    ensure!(size==self.info.frames.checked_mul(u64::from(self.info.channels)*2).context("PCM size overflow")?,"Decoded sample count differs from source metadata");
                    self.remaining=size;return Ok(());
                }
                _=>{let consumed=std::io::copy(&mut self.output.by_ref().take(size+(size&1)),&mut std::io::sink())?;ensure!(consumed==size+(size&1),"Truncated decoded audio chunk");}
            }
        }
    }
    /// Two thousand and forty-eight frames per read. Pipe backpressure keeps
    /// long sources bounded; no complete recording is cached in this decoder.
    pub fn read_chunk(&mut self)->Result<Option<Vec<i16>>> {
        if self.remaining==0 {ensure!(self.child.wait()?.success(),"Audio codec worker failed");return Ok(None);}
        let bytes=self.remaining.min(2048*u64::from(self.info.channels)*2) as usize;
        let mut encoded=vec![0;bytes];self.output.read_exact(&mut encoded).context("Truncated decoded PCM")?;
        self.remaining-=bytes as u64;
        if self.remaining==0 {ensure!(self.child.wait()?.success(),"Audio codec worker failed");}
        Ok(Some(encoded.chunks_exact(2).map(|b|i16::from_le_bytes([b[0],b[1]])).collect()))
    }
}
impl Drop for Decoder {
    fn drop(&mut self) {let _=self.child.kill();let _=self.child.wait();}
}
