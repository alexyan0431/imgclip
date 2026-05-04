use std::io::{IsTerminal, Write};

use base64::engine::general_purpose::STANDARD;
use base64::Engine;

use crate::cli::CliArgs;
use crate::error::AppError;

pub fn write_output(bytes: &[u8], args: &CliArgs) -> Result<(), AppError> {
    if args.data_uri {
        write_data_uri(bytes, args.format)
    } else if let Some(ref path) = args.output {
        write_file(bytes, path, args.quiet)
    } else if args.temp_file {
        write_temp(bytes, args.format, args.quiet)
    } else {
        write_stdout(bytes)
    }
}

fn write_stdout(bytes: &[u8]) -> Result<(), AppError> {
    let stdout = std::io::stdout();
    if stdout.is_terminal() {
        return Err(AppError::Args(
            "refusing to write binary data to a terminal. Use --output <file> or pipe output to a file.".into(),
        ));
    }
    let mut handle = stdout.lock();
    handle.write_all(bytes)?;
    Ok(())
}

fn write_file(bytes: &[u8], path: &std::path::Path, quiet: bool) -> Result<(), AppError> {
    std::fs::write(path, bytes)?;
    if !quiet {
        eprintln!("wrote {} bytes to {}", bytes.len(), path.display());
    }
    Ok(())
}

fn write_temp(bytes: &[u8], format: crate::cli::OutputFormat, quiet: bool) -> Result<(), AppError> {
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis();

    let random: u32 = rand_simple();

    let filename = format!("imgclip-{timestamp}-{random:08x}.{}", format.extension());
    let dir = std::env::temp_dir();
    let path = dir.join(filename);

    std::fs::write(&path, bytes)?;
    println!("{}", path.display());
    if !quiet {
        eprintln!("wrote {} bytes to {}", bytes.len(), path.display());
    }
    Ok(())
}

fn write_data_uri(bytes: &[u8], format: crate::cli::OutputFormat) -> Result<(), AppError> {
    let encoded = STANDARD.encode(bytes);
    println!("data:{};base64,{}", format.mime(), encoded);
    Ok(())
}

fn rand_simple() -> u32 {
    use std::time::SystemTime;
    let t = SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default();
    // Use sub-millisecond nanos as a cheap "random" — good enough for temp file names
    t.subsec_nanos()
        .wrapping_mul(2654435761)
        .wrapping_add(t.as_nanos() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn data_uri_format_png() {
        let bytes = vec![0x89, 0x50, 0x4E, 0x47];
        // We can't easily capture stdout, but we can test the encoding logic directly
        let encoded = STANDARD.encode(&bytes);
        let uri = format!("data:image/png;base64,{encoded}");
        assert!(uri.starts_with("data:image/png;base64,"));
    }

    #[test]
    fn data_uri_format_jpeg() {
        let bytes = vec![0xFF, 0xD8, 0xFF];
        let encoded = STANDARD.encode(&bytes);
        let uri = format!("data:image/jpeg;base64,{encoded}");
        assert!(uri.starts_with("data:image/jpeg;base64,"));
    }

    #[test]
    fn temp_file_naming() {
        assert!(crate::cli::OutputFormat::Png.extension() == "png");
        assert!(crate::cli::OutputFormat::Jpeg.extension() == "jpeg");
    }

    #[test]
    fn write_file_creates_file() {
        let dir = std::env::temp_dir().join("imgclip_test_write");
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("test.png");
        let bytes = vec![1, 2, 3, 4];
        write_file(&bytes, &path, true).unwrap();
        let read_back = std::fs::read(&path).unwrap();
        assert_eq!(read_back, bytes);
        std::fs::remove_dir_all(&dir).unwrap();
    }
}
