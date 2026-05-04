use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::io::Write;
use std::path::Path;
use std::time::Duration;

use crossterm::event::{self, Event, KeyCode};
use crossterm::terminal;

use crate::cli::OutputFormat;
use crate::error::AppError;

struct RawModeGuard;

impl RawModeGuard {
    fn enable() -> Result<Self, AppError> {
        terminal::enable_raw_mode().map_err(|e| AppError::Io(e.into()))?;
        Ok(RawModeGuard)
    }
}

impl Drop for RawModeGuard {
    fn drop(&mut self) {
        let _ = terminal::disable_raw_mode();
    }
}

fn hash_image(img: &crate::clipboard::RawImage) -> u64 {
    let mut h = DefaultHasher::new();
    img.width.hash(&mut h);
    img.height.hash(&mut h);
    img.data.hash(&mut h);
    h.finish()
}

macro_rules! eprint_ln {
    ($($arg:tt)*) => {{
        eprint!($($arg)*);
        eprint!("\r\n");
    }};
}

pub fn interactive_loop(
    dir: &Path,
    format: OutputFormat,
    quality: u8,
    quiet: bool,
    interval_ms: u64,
) -> Result<(), AppError> {
    std::fs::create_dir_all(dir)?;

    let _raw = RawModeGuard::enable()?;

    if !quiet {
        eprint_ln!("imgclip: interactive mode — watching clipboard for changes");
        eprint_ln!("         [s] save  [d] discard  [q] quit");
    }

    let mut last_hash = 0u64;
    let mut save_seq: u64 = 0;
    let mut pending: Option<(crate::clipboard::RawImage, u64)> = None;
    let interval = Duration::from_millis(interval_ms);

    loop {
        if pending.is_none() {
            match crate::clipboard::read_image() {
                Ok(img) => {
                    let hash = hash_image(&img);
                    if hash != last_hash {
                        if !quiet {
                            eprint_ln!("imgclip: new image ({}x{})  [s]ave [d]iscard [q]uit? ", img.width, img.height);
                        }
                        let _ = std::io::stderr().flush();
                        pending = Some((img, hash));
                    }
                }
                Err(_) => {}
            }
        }

        if event::poll(interval).map_err(|e| AppError::Io(e.into()))? {
            if let Event::Key(key) = event::read().map_err(|e| AppError::Io(e.into()))? {
                match key.code {
                    KeyCode::Char('s') if pending.is_some() => {
                        let (img, hash) = pending.take().unwrap();
                        last_hash = hash;
                        save_seq += 1;
                        match crate::watch::save_image(&img, dir, format, quality, save_seq) {
                            Ok(path) if !quiet => {
                                eprint_ln!("imgclip: saved {}", path.display());
                            }
                            Err(e) => eprint_ln!("imgclip: error: {e}"),
                            _ => {}
                        }
                    }
                    KeyCode::Char('d') if pending.is_some() => {
                        let (_, hash) = pending.take().unwrap();
                        last_hash = hash;
                        if !quiet {
                            eprint_ln!("imgclip: discarded");
                        }
                    }
                    KeyCode::Char('q') | KeyCode::Esc => {
                        if !quiet {
                            eprint_ln!("imgclip: stopped");
                        }
                        break;
                    }
                    _ => {}
                }
                let _ = std::io::stderr().flush();
            }
        }
    }

    Ok(())
}
