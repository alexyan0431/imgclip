mod cli;
mod clipboard;
mod convert;
mod daemon;
mod error;
mod interactive;
mod output;
mod watch;

use error::AppError;

fn run() -> Result<(), AppError> {
    let args = cli::parse_args()?;

    match args.mode {
        cli::Mode::OneShot => {
            let image = clipboard::read_image()?;
            let bytes = convert::encode(&image, args.format, args.quality)?;
            output::write_output(&bytes, &args)?;
        }
        cli::Mode::Copy => {
            let path = args
                .copy
                .as_ref()
                .ok_or_else(|| AppError::Args("--copy requires a file path".into()))?;
            let data = std::fs::read(path)?;
            let image = convert::decode_file(&data)?;
            clipboard::write_image(&image)?;
            if !args.quiet {
                eprintln!(
                    "imgclip: copied {} to clipboard ({}x{})",
                    path.display(),
                    image.width,
                    image.height
                );
            }
        }
        cli::Mode::Watch => {
            let dir = watch::resolve_watch_dir(args.watch_dir.as_deref())?;
            watch::watch_loop(&dir, args.format, args.quality, args.quiet, args.interval)?;
        }
        cli::Mode::Interactive => {
            let dir = watch::resolve_watch_dir(args.watch_dir.as_deref())?;
            interactive::interactive_loop(
                &dir,
                args.format,
                args.quality,
                args.quiet,
                args.interval,
            )?;
        }
        cli::Mode::Install => daemon::install()?,
        cli::Mode::Uninstall => daemon::uninstall()?,
    }
    Ok(())
}

fn main() {
    if let Err(e) = run() {
        let (prefix, code) = match &e {
            AppError::Clipboard(_) => ("clipboard error", 1),
            AppError::Io(_) => ("I/O error", 2),
            AppError::Args(_) => ("error", 3),
        };
        eprintln!("imgclip: {prefix}: {e}");
        std::process::exit(code);
    }
}
