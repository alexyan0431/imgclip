use std::path::PathBuf;

use crate::error::AppError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    OneShot,
    Watch,
    Interactive,
    Copy,
    Install,
    Uninstall,
}

impl Mode {
    fn flag_name(self) -> &'static str {
        match self {
            Mode::OneShot => unreachable!(),
            Mode::Watch => "--watch",
            Mode::Interactive => "--interactive",
            Mode::Copy => "--copy",
            Mode::Install => "--install",
            Mode::Uninstall => "--uninstall",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputFormat {
    Png,
    Jpeg,
}

impl OutputFormat {
    pub fn extension(self) -> &'static str {
        match self {
            OutputFormat::Png => "png",
            OutputFormat::Jpeg => "jpeg",
        }
    }

    pub fn mime(self) -> &'static str {
        match self {
            OutputFormat::Png => "image/png",
            OutputFormat::Jpeg => "image/jpeg",
        }
    }
}

#[derive(Debug)]
pub struct CliArgs {
    pub mode: Mode,
    pub output: Option<PathBuf>,
    pub format: OutputFormat,
    pub quality: u8,
    pub data_uri: bool,
    pub temp_file: bool,
    pub quiet: bool,
    pub copy: Option<PathBuf>,
    pub watch_dir: Option<PathBuf>,
    pub interval: u64,
}

fn set_mode(current: &mut Mode, new: Mode) -> Result<(), AppError> {
    if *current != Mode::OneShot {
        return Err(AppError::Args(format!(
            "cannot use {} and {} together",
            current.flag_name(),
            new.flag_name(),
        )));
    }
    *current = new;
    Ok(())
}

fn parse_impl(args: Vec<std::ffi::OsString>) -> Result<CliArgs, AppError> {
    use lexopt::prelude::*;

    let mut mode = Mode::OneShot;
    let mut output = None;
    let mut format = OutputFormat::Png;
    let mut quality: Option<u8> = None;
    let mut data_uri = false;
    let mut temp_file = false;
    let mut quiet = false;
    let mut copy = None;
    let mut watch_dir = None;
    let mut interval: Option<u64> = None;

    let mut parser = lexopt::Parser::from_args(args);
    while let Some(arg) = parser.next()? {
        match arg {
            Short('o') | Long("output") => {
                output = Some(parser.value()?.string()?.into());
            }
            Short('f') | Long("format") => {
                let s = parser.value()?.string()?.to_ascii_lowercase();
                format = match s.as_str() {
                    "png" => OutputFormat::Png,
                    "jpeg" | "jpg" => OutputFormat::Jpeg,
                    _ => {
                        return Err(AppError::Args(format!(
                            "unknown format '{s}'. Supported: png, jpeg"
                        )))
                    }
                };
            }
            Short('q') | Long("quality") => {
                let s = parser.value()?.string()?;
                let q: u8 = s.parse().map_err(|_| {
                    AppError::Args(format!("quality must be between 1 and 100, got '{s}'"))
                })?;
                if q == 0 || q > 100 {
                    return Err(AppError::Args(format!(
                        "quality must be between 1 and 100, got {q}"
                    )));
                }
                quality = Some(q);
            }
            Long("data-uri") => {
                data_uri = true;
            }
            Long("temp") => {
                temp_file = true;
            }
            Long("quiet") => {
                quiet = true;
            }
            Long("watch") => {
                set_mode(&mut mode, Mode::Watch)?;
            }
            Long("interactive") => {
                set_mode(&mut mode, Mode::Interactive)?;
            }
            Long("copy") => {
                copy = Some(parser.value()?.string()?.into());
                set_mode(&mut mode, Mode::Copy)?;
            }
            Long("dir") => {
                watch_dir = Some(parser.value()?.string()?.into());
            }
            Long("interval") => {
                let s = parser.value()?.string()?;
                let ms: u64 = s.parse().map_err(|_| {
                    AppError::Args(format!(
                        "interval must be a positive number (ms), got '{s}'"
                    ))
                })?;
                if ms == 0 {
                    return Err(AppError::Args("interval must be greater than 0".into()));
                }
                interval = Some(ms);
            }
            Long("install") => {
                set_mode(&mut mode, Mode::Install)?;
            }
            Long("uninstall") => {
                set_mode(&mut mode, Mode::Uninstall)?;
            }
            Short('h') | Long("help") => {
                print_help();
                std::process::exit(0);
            }
            Short('V') | Long("version") => {
                println!("imgclip {}", env!("CARGO_PKG_VERSION"));
                std::process::exit(0);
            }
            _ => return Err(AppError::Args(format!("{arg:?}: unexpected argument"))),
        }
    }

    if output.is_some() && temp_file {
        return Err(AppError::Args(
            "cannot use --output and --temp together".into(),
        ));
    }

    Ok(CliArgs {
        mode,
        output,
        format,
        quality: quality.unwrap_or(85),
        data_uri,
        temp_file,
        quiet,
        copy,
        watch_dir,
        interval: interval.unwrap_or(500),
    })
}

pub fn parse_args() -> Result<CliArgs, AppError> {
    parse_impl(std::env::args_os().skip(1).collect())
}

fn print_help() {
    println!(
        "imgclip {version} — Extract images from the clipboard

USAGE:
    imgclip [OPTIONS]               Save clipboard image (one-shot)
    imgclip --copy <FILE>           Copy image file to clipboard
    imgclip --watch [OPTIONS]       Watch clipboard, auto-save new images
    imgclip --interactive [OPTIONS] Watch clipboard, choose to save or discard
    imgclip --install               Install auto-start (runs --watch on login)
    imgclip --uninstall             Remove auto-start

OPTIONS:
    -o, --output <PATH>     Write image to specified file
    -f, --format <FORMAT>   Output format: png (default), jpeg
    -q, --quality <1-100>   JPEG quality (default: 85)
        --copy <FILE>       Copy image file to the clipboard
        --data-uri          Output as data URI string
        --temp              Write to temp file, print path to stdout
        --dir <PATH>        Save directory for --watch/--interactive (default: ~/Pictures/imgclip)
        --interval <MS>     Poll interval in ms for --watch/--interactive (default: 500)
        --quiet             Suppress informational output
    -h, --help              Print help
    -V, --version           Print version

Default behavior: write PNG to stdout (must be piped/redirected).
Watch mode saves to ~/Pictures/imgclip/ by default.
Interactive mode prompts [s]ave / [d]iscard / [q]uit for each new image.",
        version = env!("CARGO_PKG_VERSION")
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(args: &[&str]) -> Result<CliArgs, AppError> {
        let os_args: Vec<std::ffi::OsString> = args.iter().map(|s| s.into()).collect();
        parse_impl(os_args)
    }

    #[test]
    fn defaults() {
        let args = parse(&[]).unwrap();
        assert_eq!(args.mode, Mode::OneShot);
        assert!(args.output.is_none());
        assert_eq!(args.format, OutputFormat::Png);
        assert_eq!(args.quality, 85);
        assert!(!args.data_uri);
        assert!(!args.temp_file);
        assert!(!args.quiet);
        assert_eq!(args.interval, 500);
        assert!(args.watch_dir.is_none());
    }

    #[test]
    fn output_path() {
        let args = parse(&["-o", "test.png"]).unwrap();
        assert_eq!(args.output.unwrap(), PathBuf::from("test.png"));
    }

    #[test]
    fn format_jpeg() {
        let args = parse(&["-f", "jpeg"]).unwrap();
        assert_eq!(args.format, OutputFormat::Jpeg);
        let args = parse(&["-f", "jpg"]).unwrap();
        assert_eq!(args.format, OutputFormat::Jpeg);
    }

    #[test]
    fn quality() {
        let args = parse(&["-q", "50"]).unwrap();
        assert_eq!(args.quality, 50);
    }

    #[test]
    fn quality_default() {
        let args = parse(&[]).unwrap();
        assert_eq!(args.quality, 85);
    }

    #[test]
    fn quality_zero_rejected() {
        assert!(parse(&["-q", "0"]).is_err());
    }

    #[test]
    fn quality_above_100_rejected() {
        assert!(parse(&["-q", "101"]).is_err());
        assert!(parse(&["-q", "255"]).is_err());
    }

    #[test]
    fn quality_100_accepted() {
        let args = parse(&["-q", "100"]).unwrap();
        assert_eq!(args.quality, 100);
    }

    #[test]
    fn bad_format_rejected() {
        let err = parse(&["-f", "bmp"]).unwrap_err();
        assert!(matches!(err, AppError::Args(_)));
    }

    #[test]
    fn output_and_temp_mutually_exclusive() {
        let err = parse(&["--output", "x.png", "--temp"]).unwrap_err();
        assert!(matches!(err, AppError::Args(_)));
    }

    #[test]
    fn combined_flags() {
        let args = parse(&["-f", "jpeg", "-q", "90", "--data-uri", "--quiet"]).unwrap();
        assert_eq!(args.format, OutputFormat::Jpeg);
        assert_eq!(args.quality, 90);
        assert!(args.data_uri);
        assert!(args.quiet);
    }

    #[test]
    fn watch_mode() {
        let args = parse(&["--watch"]).unwrap();
        assert_eq!(args.mode, Mode::Watch);
    }

    #[test]
    fn watch_with_format() {
        let args = parse(&["--watch", "-f", "jpeg", "-q", "90"]).unwrap();
        assert_eq!(args.mode, Mode::Watch);
        assert_eq!(args.format, OutputFormat::Jpeg);
        assert_eq!(args.quality, 90);
    }

    #[test]
    fn watch_with_dir_and_interval() {
        let args = parse(&["--watch", "--dir", "/tmp/shots", "--interval", "200"]).unwrap();
        assert_eq!(args.mode, Mode::Watch);
        assert_eq!(args.watch_dir.unwrap(), PathBuf::from("/tmp/shots"));
        assert_eq!(args.interval, 200);
    }

    #[test]
    fn interval_zero_rejected() {
        assert!(parse(&["--watch", "--interval", "0"]).is_err());
    }

    #[test]
    fn copy_mode() {
        let args = parse(&["--copy", "photo.png"]).unwrap();
        assert_eq!(args.mode, Mode::Copy);
        assert_eq!(args.copy.unwrap(), PathBuf::from("photo.png"));
    }

    #[test]
    fn install_mode() {
        let args = parse(&["--install"]).unwrap();
        assert_eq!(args.mode, Mode::Install);
    }

    #[test]
    fn uninstall_mode() {
        let args = parse(&["--uninstall"]).unwrap();
        assert_eq!(args.mode, Mode::Uninstall);
    }

    #[test]
    fn mode_conflict_rejected() {
        let err = parse(&["--watch", "--install"]).unwrap_err();
        assert!(matches!(err, AppError::Args(_)));
        let msg = err.to_string();
        assert!(msg.contains("cannot use"), "got: {msg}");

        let err = parse(&["--copy", "a.png", "--watch"]).unwrap_err();
        assert!(matches!(err, AppError::Args(_)));

        let err = parse(&["--install", "--uninstall"]).unwrap_err();
        assert!(matches!(err, AppError::Args(_)));
    }
}
