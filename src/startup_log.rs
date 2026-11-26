use std::{
    fmt::Write as _,
    fs,
    io::{Read, Seek, SeekFrom, Write as _},
    path::{Path, PathBuf},
};

const LOG_BYTES: u64 = 24 * 1024;

fn directory() -> Option<PathBuf> {
    #[cfg(target_os = "android")]
    return crate::android_app::storage();
    #[cfg(target_os = "horizon")]
    return Some(PathBuf::from("sdmc:/blotworx"));
    #[cfg(not(any(target_os = "android", target_os = "horizon")))]
    Some(dirs::data_local_dir()?.join("Blotworx Engine"))
}

#[cfg(target_os = "horizon")]
pub fn install() -> Option<PathBuf> {
    None
}

#[cfg(not(target_os = "horizon"))]
pub fn install() -> Option<PathBuf> {
    let directory = directory()?;
    fs::create_dir_all(&directory).ok()?;
    let path = directory.join("blotworx.log");
    if path.exists() {
        fs::rename(&path, directory.join("blotworx-previous.log")).ok()?;
    }
    let file = fs::OpenOptions::new().create(true).append(true).open(&path).ok()?;
    let mut logger = env_logger::Builder::from_env(
        env_logger::Env::default().default_filter_or("warn,blotworx=info,blotworx_phase=info"),
    );
    // Every log line goes to the log file and to the console window the game was started from.
    struct Tee(fs::File);
    impl std::io::Write for Tee {
        fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
            let _ = std::io::stderr().write_all(bytes);
            self.0.write_all(bytes)?;
            Ok(bytes.len())
        }
        fn flush(&mut self) -> std::io::Result<()> {
            let _ = std::io::stderr().flush();
            self.0.flush()
        }
    }
    logger.target(env_logger::Target::Pipe(Box::new(Tee(file))));
    logger.try_init().ok()?;
    let previous = std::panic::take_hook();
    // Keep writing to the same session even if a second process rotates its name.
    let crash_log = std::sync::Mutex::new(fs::OpenOptions::new().append(true).open(&path).ok());
    std::panic::set_hook(Box::new(move |info| {
        if let Ok(mut file) = crash_log.lock() {
            if let Some(file) = file.as_mut() {
                let _ = writeln!(
                    file,
                    "{info}\n{}",
                    std::backtrace::Backtrace::force_capture()
                );
            }
        }
        previous(info);
    }));
    log::info!(
        "Blotworx {} on {} {}",
        env!("CARGO_PKG_VERSION"),
        std::env::consts::OS,
        std::env::consts::ARCH
    );
    Some(path)
}

fn tail(path: &Path) -> std::io::Result<String> {
    let mut file = fs::File::open(path)?;
    let start = file.metadata()?.len().saturating_sub(LOG_BYTES);
    file.seek(SeekFrom::Start(start))?;
    let mut bytes = Vec::new();
    file.take(LOG_BYTES).read_to_end(&mut bytes)?;
    let contents = String::from_utf8_lossy(&bytes);
    Ok(if start > 0 {
        contents.split_once('\n').map_or("", |(_, rest)| rest).to_owned()
    } else {
        contents.into_owned()
    })
}

pub fn report(level: Option<&str>, error: Option<&str>, elapsed: Option<std::time::Duration>) -> String {
    let mut report = format!(
        "Blotworx diagnostic report\nEngine {} | {} {}\n",
        env!("CARGO_PKG_VERSION"), std::env::consts::OS, std::env::consts::ARCH,
    );
    if let Some(level) = level {
        let _ = writeln!(report, "Level: {level}");
    }
    if let Some(error) = error {
        let _ = writeln!(report, "Application error: {error}");
    }
    if let Some(elapsed) = elapsed {
        let _ = writeln!(report, "Level loading for {} seconds", elapsed.as_secs());
    }
    if let Some(directory) = directory() {
        for (name, label) in [
            ("blotworx.log", "Latest session"),
            ("blotworx-previous.log", "Previous session"),
        ] {
            if let Ok(log) = tail(&directory.join(name)) {
                let _ = writeln!(report, "\n--- {label} ---\n{log}");
            }
        }
    }
    #[cfg(not(target_os = "horizon"))]
    if let Some(home) = dirs::home_dir() {
        for separator in ['\\', '/'] {
            let path = home.to_string_lossy().replace(['\\', '/'], &separator.to_string());
            // Debug messages can escape Windows paths or change drive-letter case.
            for path in [path.clone(), path.replace('\\', "\\\\")] {
                if path.chars().count() > 3 {
                    if cfg!(windows) {
                        let folded = report.to_ascii_lowercase();
                        let path = path.to_ascii_lowercase();
                        let mut redacted = String::with_capacity(report.len());
                        let mut start = 0;
                        for (at, _) in folded.match_indices(&path) {
                            redacted.push_str(&report[start..at]);
                            redacted.push_str("<home>");
                            start = at + path.len();
                        }
                        redacted.push_str(&report[start..]);
                        report = redacted;
                    } else {
                        report = report.replace(&path, "<home>");
                    }
                }
            }
        }
    }
    report
}

#[cfg(not(any(target_os = "android", target_os = "horizon")))]
pub fn save_report(report: &str) -> std::io::Result<bool> {
    let Some(path) = rfd::FileDialog::new()
        .set_file_name("blotworx-report.txt")
        .add_filter("Text report", &["txt"])
        .save_file()
    else {
        return Ok(false);
    };
    fs::write(path, report)?;
    Ok(true)
}

#[cfg(any(windows, target_os = "macos"))]
pub const GAME_PROCESS_ENV: &str = "BLOTWORX_CRASH_MONITOR_CHILD";

#[cfg(windows)]
pub fn supervise() -> anyhow::Result<()> {
    let result = std::process::Command::new(std::env::current_exe()?)
        .args(std::env::args_os().skip(1))
        .env(GAME_PROCESS_ENV, "1")
        .status();
    let error = match result {
        Ok(status) if status.success() => return Ok(()),
        Ok(status) => format!("Game process exited with {status}"),
        Err(error) => format!("Could not start the game process: {error}"),
    };
    let diagnostics = report(None, Some(&error), None);
    let answer = rfd::MessageDialog::new()
        .set_title("Blotworx closed unexpectedly")
        .set_description("Would you like to save a diagnostic report? You can attach it when reporting the issue.")
        .set_level(rfd::MessageLevel::Error)
        .set_buttons(rfd::MessageButtons::YesNo)
        .show();
    if answer == rfd::MessageDialogResult::Yes {
        if let Err(error) = save_report(&diagnostics) {
            rfd::MessageDialog::new()
                .set_title("Could not save the report")
                .set_description(error.to_string())
                .set_level(rfd::MessageLevel::Error)
                .show();
        }
    }
    anyhow::bail!(error)
}

/// Store the report dialog
#[cfg(target_os = "macos")]
pub fn supervise() -> anyhow::Result<()> {
    let result = std::process::Command::new(std::env::current_exe()?)
        .args(std::env::args_os().skip(1))
        .env(GAME_PROCESS_ENV, "1")
        .status();
    let error = match result {
        Ok(status) if status.success() => return Ok(()),
        Ok(status) => format!("Game process exited with {status}"),
        Err(error) => format!("Could not start the game process: {error}"),
    };
    let diagnostics = report(None, Some(&error), None);
    match directory().map(|directory| directory.join("blotworx-crash.txt")) {
        Some(path) if fs::write(&path, &diagnostics).is_ok() => {
            eprintln!("{error}\nDiagnostic report has been written to {}", path.display());
        }
        _ => eprintln!("{diagnostics}"),
    }
    anyhow::bail!(error)
}
