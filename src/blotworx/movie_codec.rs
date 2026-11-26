//! Pinned, isolated FFmpeg codec worker. No Python runtime or system installation.
use anyhow::{ensure, Context, Result};
use std::{
    io::Read,
    path::{Path, PathBuf},
    process::{Command, Stdio},
};
#[cfg(all(not(target_os = "android"), not(target_os = "horizon"), not(target_os = "macos")))]
pub const ARCHIVE_URL:&str="https://files.pythonhosted.org/packages/2c/c6/fa760e12a2483469e2bf5058c5faff664acf66cadb4df2ad6205b016a73d/imageio_ffmpeg-0.6.0-py3-none-win_amd64.whl";
#[cfg(all(not(target_os = "android"), not(target_os = "horizon"), not(target_os = "macos")))]
pub const ARCHIVE_SHA: &str = "02fa47c83703c37df6bfe4896aab339013f62bf02c5ebf2dce6da56af04ffc0a";
#[cfg(all(not(target_os = "android"), not(target_os = "horizon"), not(target_os = "macos")))]
pub const EXE_SHA: &str = "2ce797a0f88d7f067180338fb227f7b1928ea727bd9a4d7a1d022f7c52af71a3";
#[cfg(not(target_os = "horizon"))]
pub fn digest(bytes: &[u8]) -> String {
    ring::digest::digest(&ring::digest::SHA256, bytes)
        .as_ref()
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect()
}
#[cfg(target_os = "horizon")]
pub fn digest(_bytes: &[u8]) -> String {
    String::new()
}
pub fn command(exe: &Path) -> Command {
    let mut c = Command::new(exe);
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        c.creation_flags(0x08000000);
    }
    c.stdin(Stdio::null());
    c
}
#[cfg(all(not(target_os = "android"), not(target_os = "horizon"), not(target_os = "macos")))]
fn valid(path: &Path) -> bool {
    std::fs::read(path).is_ok_and(|b| digest(&b) == EXE_SHA)
}
#[cfg(target_os = "android")]
pub fn provision() -> Result<PathBuf> {
    let directory = std::env::var_os("BLOTWORX_ANDROID_CODECS").context("Android codec directory is unavailable")?;
    let path = PathBuf::from(directory).join("libbwx_ffmpeg.so");
    ensure!(path.is_file(), "This Android build does not include the Bink codec");
    Ok(path)
}

#[cfg(target_os = "horizon")]
pub fn provision() -> Result<PathBuf> {
    anyhow::bail!("External movie codec binary is unsupported on Nintendo 3DS")
}

/// A codec distribution for different operating systems
#[cfg(target_os = "macos")]
pub(super) struct Pinned {
    pub url: &'static str,
    pub archive_sha256: &'static str,
    pub member: &'static str,
    pub executable: &'static str,
    pub executable_sha256: &'static str,
}

// imageio-ffmpeg 0.6.0 ships the static FFmpeg 7.1 distribution
#[cfg(all(target_os = "macos", target_arch = "aarch64"))]
pub(super) const PINNED: Option<Pinned> = Some(Pinned {
    url: "https://files.pythonhosted.org/packages/40/5c/f3d8a657d362cc93b81aab8feda487317da5b5d31c0e1fdfd5e986e55d17/imageio_ffmpeg-0.6.0-py3-none-macosx_11_0_arm64.whl",
    archive_sha256: "b1ae3173414b5fc5f538a726c4e48ea97edc0d2cdc11f103afee655c463fa742",
    member: "imageio_ffmpeg/binaries/ffmpeg-macos-aarch64-v7.1",
    executable: "ffmpeg",
    executable_sha256: "6d175a4743ca50256e89a8cdd731100f9cee33bd79aeea46894d209410dc6617",
});

#[cfg(all(target_os = "macos", target_arch = "x86_64"))]
pub(super) const PINNED: Option<Pinned> = Some(Pinned {
    url: "https://files.pythonhosted.org/packages/da/58/87ef68ac83f4c7690961bce288fd8e382bc5f1513860fc7f90a9c1c1c6bf/imageio_ffmpeg-0.6.0-py3-none-macosx_10_9_intel.macosx_10_9_x86_64.whl",
    archive_sha256: "9d2baaf867088508d4a3458e61eeb30e945c4ad8016025545f66c4b5aaef0a61",
    member: "imageio_ffmpeg/binaries/ffmpeg-macos-x86_64-v7.1",
    executable: "ffmpeg",
    executable_sha256: "4a4a968b98859588e98500ae25973d80a5ca5eed0724222b9f76360dcb72a001",
});

// OS' without a pinned desktop build keep using the system executable
#[cfg(all(
    target_os = "macos",
    not(any(target_arch = "aarch64", target_arch = "x86_64"))
))]
pub(super) const PINNED: Option<Pinned> = None;

/// Install an archive member into the runtime codec cache
#[cfg(target_os = "macos")]
pub(super) fn install_pinned(pinned: &Pinned, folder: &str, notices: &str) -> Result<PathBuf> {
    let cache = dirs::cache_dir()
        .context("No codec cache directory")?
        .join("Blotworx Engine/runtime")
        .join(folder);
    let path = cache.join(pinned.executable);
    let valid = |bytes: &[u8]| digest(bytes) == pinned.executable_sha256;
    if std::fs::read(&path).is_ok_and(|bytes| valid(&bytes)) {
        return Ok(path);
    }
    std::fs::create_dir_all(&cache)?;
    // The largest desktop payload is the 76 MiB Intel FFmpeg build
    let bytes = ureq::get(pinned.url)
        .call()?
        .body_mut()
        .with_config()
        .limit(96 * 1024 * 1024 + 1)
        .read_to_vec()?;
    ensure!(
        digest(&bytes) == pinned.archive_sha256,
        "Codec archive checksum mismatch"
    );
    let mut archive = zip::ZipArchive::new(std::io::Cursor::new(bytes))?;
    let mut binary = Vec::new();
    archive.by_name(pinned.member)?.read_to_end(&mut binary)?;
    ensure!(
        valid(&binary),
        "Codec executable checksum mismatch: {}",
        pinned.executable
    );
    let temporary = cache.join(format!("{}.{}.tmp", pinned.executable, std::process::id()));
    std::fs::write(&temporary, &binary)?;
    // Publish the worker executable only once it is runnable
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(&temporary, std::fs::Permissions::from_mode(0o755))?;
    }
    if std::fs::read(&path).is_ok_and(|existing| valid(&existing)) {
        std::fs::remove_file(temporary)?;
        return Ok(path);
    }
    if path.exists() {
        std::fs::remove_file(&path)?;
    }
    std::fs::rename(&temporary, &path)?;
    std::fs::write(cache.join("SOURCE.txt"), notices)?;
    Ok(path)
}

#[cfg(target_os = "macos")]
pub fn provision() -> Result<PathBuf> {
    // The release ships a Windows executable. Other platforms use a system FFmpeg
    static READY: std::sync::OnceLock<PathBuf> = std::sync::OnceLock::new();
    if let Some(path) = READY.get() {
        return Ok(path.clone());
    }
    let mut candidates = Vec::new();
    if let Some(path) = std::env::var_os("BLOTWORX_FFMPEG") {
        candidates.push(PathBuf::from(path));
    }
    if let Ok(exe) = std::env::current_exe() {
        if let Some(parent) = exe.parent() {
            candidates.push(parent.join("Codecs/ffmpeg"));
        }
    }
    if let Some(path) = std::env::var_os("PATH") {
        for directory in std::env::split_paths(&path) {
            candidates.push(directory.join("ffmpeg"));
        }
    }
    candidates.extend(
        ["/opt/homebrew/bin/ffmpeg", "/usr/local/bin/ffmpeg", "/usr/bin/ffmpeg"]
            .map(PathBuf::from),
    );
    for candidate in candidates {
        if candidate.is_file()
            && command(&candidate)
                .arg("-version")
                .output()
                .is_ok_and(|output| output.status.success())
        {
            let _ = READY.set(candidate.clone());
            return Ok(candidate);
        }
    }
    // No package manager install is required!
    if let Some(pinned) = &PINNED {
        let path = install_pinned(
            pinned,
            "ffmpeg-7.1",
            &format!(
                "Pinned distribution: {}\nArchive SHA256: {}\nExecutable: {}\nExecutable SHA256: {}\nFFmpeg source: https://github.com/FFmpeg/FFmpeg/tree/n7.1\n",
                pinned.url, pinned.archive_sha256, pinned.executable, pinned.executable_sha256
            ),
        )?;
        if let Ok(output) = command(&path).arg("-L").output() {
            let mut notice = output.stdout;
            notice.extend(output.stderr);
            let _ = std::fs::write(path.with_file_name("FFmpeg-LICENSE.txt"), notice);
        }
        if command(&path)
            .arg("-version")
            .output()
            .is_ok_and(|output| output.status.success())
        {
            let _ = READY.set(path.clone());
            return Ok(path);
        }
    }
    anyhow::bail!(
        "FFmpeg could not be provisioned automatically. Set BLOTWORX_FFMPEG to an ffmpeg executable."
    )
}

#[cfg(all(not(target_os = "android"), not(target_os = "horizon"), not(target_os = "macos")))]
pub fn provision() -> Result<PathBuf> {
    static PROVISION: std::sync::Mutex<()> = std::sync::Mutex::new(());
    let _guard = PROVISION
        .lock()
        .map_err(|_| anyhow::anyhow!("Movie codec provisioning lock poisoned"))?;
    ensure!(
        cfg!(all(windows, target_arch = "x86_64")),
        "Movie codec provisioning is currently available for Windows x64"
    );
    if let Ok(exe) = std::env::current_exe() {
        let bundled = exe
            .parent()
            .context("Executable has no folder")?
            .join("Codecs/ffmpeg.exe");
        if valid(&bundled) {
            return Ok(bundled);
        }
    }
    let cache = dirs::cache_dir()
        .context("No codec cache directory")?
        .join("Blotworx Engine/runtime/ffmpeg-7.1");
    let exe = cache.join("ffmpeg.exe");
    if valid(&exe) {
        return Ok(exe);
    }
    // Other movie requests may provision concurrently in separate local games.
    // Install by a unique temporary filename, after validating both layers.
    std::fs::create_dir_all(&cache)?;
    let bytes = ureq::get(ARCHIVE_URL)
        .call()?
        .body_mut()
        .with_config()
        .limit(31_246_824 + 1)
        .read_to_vec()?;
    ensure!(
        digest(&bytes) == ARCHIVE_SHA,
        "Movie codec archive checksum mismatch"
    );
    let mut zip = zip::ZipArchive::new(std::io::Cursor::new(bytes))?;
    let mut binary = Vec::new();
    zip.by_name("imageio_ffmpeg/binaries/ffmpeg-win-x86_64-v7.1.exe")?
        .read_to_end(&mut binary)?;
    ensure!(
        digest(&binary) == EXE_SHA,
        "Movie codec executable checksum mismatch"
    );
    if valid(&exe) {
        return Ok(exe);
    }
    let temp = cache.join(format!("ffmpeg-{}.tmp", std::process::id()));
    std::fs::write(&temp, &binary)?;
    if exe.exists() {
        std::fs::remove_file(&exe)?;
    }
    std::fs::rename(&temp, &exe)?;
    let mut license = String::new();
    zip.by_name("imageio_ffmpeg-0.6.0.dist-info/LICENSE")?
        .read_to_string(&mut license)?;
    std::fs::write(cache.join("imageio-LICENSE.txt"), license)?;
    let output = command(&exe).arg("-L").output()?;
    ensure!(output.status.success(), "Movie codec license query failed");
    let mut notice = output.stdout;
    notice.extend(output.stderr);
    notice.extend_from_slice(format!("\nPinned distribution: {ARCHIVE_URL}\nArchive SHA256: {ARCHIVE_SHA}\nExecutable SHA256: {EXE_SHA}\nFFmpeg source: https://github.com/FFmpeg/FFmpeg/tree/n7.1\nBuild provider: https://www.gyan.dev/ffmpeg/builds/\n").as_bytes());
    std::fs::write(cache.join("FFmpeg-LICENSE.txt"), notice)?;
    Ok(exe)
}
