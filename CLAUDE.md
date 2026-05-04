# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build & Test Commands

```bash
cargo build                    # debug build
cargo build --release          # optimized release build (LTO, stripped, size-optimized)
cargo test                     # run all unit tests
cargo test -- <test_name>      # run a single test (e.g. cargo test -- defaults)
cargo run -- [OPTIONS]         # run the CLI
```

## Architecture

`imgclip` is a single-binary Rust CLI with four modes of operation:
- **OneShot**: reads a clipboard image and outputs it once
- **Copy**: reads an image file and writes it to the clipboard
- **Watch**: polls the clipboard in a loop and auto-saves new images
- **Install/Uninstall**: write/remove platform auto-start entry

```
CLI args → dispatch by Mode
  ├─ OneShot: clipboard read → image encode → output write
  ├─ Copy:    file read → image decode → clipboard write
  ├─ Watch:   clipboard poll → hash compare → image encode → file save (loop)
  ├─ Install: write platform auto-start entry
  └─ Uninstall: remove auto-start entry
```

Each stage maps to one module:

- **`cli.rs`** — Argument parsing via `lexopt`. Defines `Mode` (OneShot/Copy/Watch/Install/Uninstall), `OutputFormat` (Png/Jpeg), and `CliArgs`. Watch mode supports configurable `--dir` and `--interval`.
- **`clipboard.rs`** — Reads/writes RGBA pixel data from/to clipboard via `arboard`. Returns a `RawImage` struct (width, height, RGBA bytes).
- **`convert.rs`** — Encodes `RawImage` to PNG or JPEG bytes using the `image` crate. JPEG encoding composites onto white background before converting to RGB. Also decodes image files to `RawImage` for the `--copy` mode.
- **`output.rs`** — Writes bytes to the chosen destination: file, stdout (binary, rejects terminals), data URI (base64), or temp file with timestamp-based naming.
- **`watch.rs`** — Watch mode: polls clipboard at configurable interval, hashes image data via `DefaultHasher` to detect changes, auto-saves to configurable directory (default `~/Pictures/imgclip/`). Persists last hash to `.imgclip_last_hash` file to avoid re-saving after restart.
- **`daemon.rs`** — Platform-specific auto-start: Windows (VBS in Startup folder), Linux (.desktop in autostart), macOS (LaunchAgent plist).
- **`error.rs`** — `AppError` enum with three variants (Clipboard, Io, Args). Exit codes: 1=clipboard, 2=I/O, 3=args.

## Key Dependencies

- `lexopt` — lightweight argument parser (no derive macros)
- `arboard` — cross-platform clipboard access (read and write)
- `image` — image encoding/decoding (PNG, JPEG, and many more for `--copy` input)
- `base64` — data URI encoding
- `dirs` — cross-platform directory discovery (Pictures, AppData, home, etc.)

## CI/CD

- `.github/workflows/ci.yml` — Tests on Windows, Linux, macOS on every push/PR.
- `.github/workflows/release.yml` — Builds release binaries for 6 targets (x86_64 + aarch64 for Windows/Linux/macOS) on version tags (`v*`). Creates a GitHub Release with all binaries attached.
