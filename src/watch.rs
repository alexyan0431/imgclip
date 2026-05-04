use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use crate::cli::OutputFormat;
use crate::error::AppError;

const HASH_FILE: &str = ".imgclip_last_hash";

pub fn resolve_watch_dir(cli_dir: Option<&Path>) -> Result<PathBuf, AppError> {
    if let Some(d) = cli_dir {
        return Ok(d.to_path_buf());
    }
    let base = dirs::picture_dir()
        .or_else(dirs::home_dir)
        .ok_or_else(|| AppError::Args("cannot determine save directory".into()))?;
    Ok(base.join("imgclip"))
}

pub fn watch_loop(
    dir: &Path,
    format: OutputFormat,
    quality: u8,
    quiet: bool,
    interval_ms: u64,
) -> Result<(), AppError> {
    std::fs::create_dir_all(dir)?;

    if !quiet {
        eprintln!("imgclip: watching clipboard, saving to {}", dir.display());
        eprintln!("         press Ctrl+C to stop");
    }

    let mut last_hash = load_last_hash(dir);
    let mut had_image = false;
    let mut save_seq: u64 = 0;

    loop {
        match crate::clipboard::read_image() {
            Ok(img) => {
                had_image = true;
                let hash = {
                    let mut h = DefaultHasher::new();
                    img.width.hash(&mut h);
                    img.height.hash(&mut h);
                    img.data.hash(&mut h);
                    h.finish()
                };
                if hash != last_hash {
                    last_hash = hash;
                    save_last_hash(dir, hash);
                    save_seq += 1;
                    match save_image(&img, dir, format, quality, save_seq) {
                        Ok(path) if !quiet => eprintln!("imgclip: saved {}", path.display()),
                        Err(e) => eprintln!("imgclip: error: {e}"),
                        _ => {}
                    }
                }
            }
            Err(e) => {
                if had_image {
                    eprintln!("imgclip: clipboard read error: {e}");
                    had_image = false;
                }
            }
        }
        std::thread::sleep(Duration::from_millis(interval_ms));
    }
}

fn load_last_hash(dir: &Path) -> u64 {
    let path = dir.join(HASH_FILE);
    std::fs::read_to_string(&path)
        .ok()
        .and_then(|s| s.trim().parse::<u64>().ok())
        .unwrap_or(0)
}

fn save_last_hash(dir: &Path, hash: u64) {
    let path = dir.join(HASH_FILE);
    let _ = std::fs::write(&path, hash.to_string());
}

pub(crate) fn save_image(
    img: &crate::clipboard::RawImage,
    dir: &Path,
    format: OutputFormat,
    quality: u8,
    seq: u64,
) -> Result<PathBuf, AppError> {
    let bytes = crate::convert::encode(img, format, quality)?;
    let millis = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis();
    let filename = format!("imgclip-{millis}-{seq}.{}", format.extension());
    let path = dir.join(&filename);
    std::fs::write(&path, &bytes)?;
    Ok(path)
}
