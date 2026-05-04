[English](#english) | [中文](#中文)

---

<a id="english"></a>

# imgclip

A minimal CLI tool to extract images from the clipboard and save them as files, pipe them to stdout, or convert them to data URIs. Also copies image files **to** the clipboard. Supports a **watch mode** that automatically saves new clipboard images, and an **interactive mode** that lets you selectively save or discard each one.

## Install

**Option 1: Download a prebuilt binary** (recommended)

Grab the latest release for your platform from the [Releases page](https://github.com/alexyan0431/imgclip/releases).

**Option 2: Build from source**

**Prerequisites:** [Rust](https://rustup.rs/) (1.70+)

```bash
cargo install --git https://github.com/alexyan0431/imgclip.git
```

Or build manually:

```bash
git clone https://github.com/alexyan0431/imgclip.git
cd imgclip
cargo build --release
```

The binary will be at `target/release/imgclip`. Copy it somewhere in your `$PATH`.

**Windows users:** If you encounter clipboard errors, make sure the terminal session has clipboard access permissions.

## Uninstall

- **`cargo install`:** run `cargo uninstall imgclip`.
- **Prebuilt zip/binary or manual copy:** delete the executable (find it with `which imgclip` on Unix or `where imgclip` on Windows).
- **`--install` auto-start:** run `imgclip --uninstall` first to remove the login startup entry, then remove the binary using either bullet above.

## Quick Start

```bash
# One-time setup: install auto-start (watches clipboard on every login)
imgclip --install
```

That's it. After restarting (or logging in), any image you copy or screenshot will be automatically saved to `~/Pictures/imgclip/` (or `%USERPROFILE%\Pictures\imgclip\` on Windows).

To remove auto-start later: `imgclip --uninstall`. To remove imgclip entirely, see [Uninstall](#uninstall).

## Usage

### Watch Mode (Recommended)

```bash
# Start watching (saves to ~/Pictures/imgclip/ by default)
imgclip --watch

# Watch with JPEG output
imgclip --watch -f jpeg -q 90

# Watch with custom directory and poll interval
imgclip --watch --dir ~/screenshots --interval 200

# Run silently in the background
imgclip --watch --quiet
```

### One-Shot Mode

```bash
# Copy an image (screenshot, browser image, etc.), then:

# Save to a file
imgclip -o screenshot.png

# Save as JPEG with quality 90
imgclip -o photo.jpg -f jpeg -q 90

# Pipe to another command (default outputs PNG to stdout)
imgclip | some-command

# Get a data URI (useful for HTML/CSS embedding)
imgclip --data-uri

# Write to a temp file and print the path
imgclip --temp

# Suppress info messages
imgclip -o out.png --quiet
```

### Interactive Mode

```bash
# Watch clipboard and prompt for each new image: [s]ave / [d]iscard / [q]uit
imgclip --interactive

# Interactive with JPEG output
imgclip --interactive -f jpeg -q 90

# Custom save directory
imgclip --interactive --dir ~/screenshots
```

### Copy File to Clipboard

```bash
# Copy an image file to the clipboard (supports PNG, JPEG, BMP, WebP, etc.)
imgclip --copy photo.png
imgclip --copy screenshot.jpg
```

## Options

| Option | Description |
|--------|-------------|
| `--watch` | Watch clipboard, auto-save new images |
| `--interactive` | Watch clipboard, prompt to save or discard each image |
| `--copy <FILE>` | Copy image file to the clipboard |
| `--install` | Install auto-start (runs `--watch` on login) |
| `--uninstall` | Remove auto-start |
| `-o, --output <PATH>` | Write image to the specified file |
| `-f, --format <FORMAT>` | Output format: `png` (default), `jpeg`/`jpg` |
| `-q, --quality <1-100>` | JPEG quality (default: 85) |
| `--data-uri` | Output as `data:image/...;base64,...` string |
| `--temp` | Write to a temp file, print the path to stdout |
| `--dir <PATH>` | Save directory for `--watch`/`--interactive` (default: ~/Pictures/imgclip) |
| `--interval <MS>` | Poll interval in ms for `--watch`/`--interactive` (default: 500) |
| `--quiet` | Suppress informational messages |
| `-h, --help` | Print help |
| `-V, --version` | Print version |

## Examples

```bash
# Quick screenshot → clipboard → file
imgclip -o shot.png

# Embed image in HTML
echo "<img src=\"$(imgclip --data-uri)\" />"

# Convert clipboard image to JPEG and pipe to curl
imgclip -f jpeg -q 80 | curl -T - https://upload.example.com

# Use in a script to auto-save screenshots
imgclip --temp --quiet | xargs -I{} mv {} ~/screenshots/

# Copy a file to clipboard, then paste into Slack/Docs
imgclip --copy diagram.png

# Selective capture: only save the screenshots you want
imgclip --interactive --dir ~/screenshots
```

## License

[MIT](LICENSE)

---

<a id="中文"></a>

# imgclip

一个轻量级命令行工具，从剪贴板提取图片并保存为文件、输出到 stdout 或转换为 data URI。也支持将图片文件复制**到**剪贴板。支持**监听模式**自动保存新图片，以及**交互模式**让你逐张选择保存或丢弃。

## 安装

**方式一：下载预编译二进制**（推荐）

从 [Releases 页面](https://github.com/alexyan0431/imgclip/releases) 下载对应平台的最新版本。

**方式二：从源码构建**

**前置条件：** [Rust](https://rustup.rs/) (1.70+)

```bash
cargo install --git https://github.com/alexyan0431/imgclip.git
```

或手动构建：

```bash
git clone https://github.com/alexyan0431/imgclip.git
cd imgclip
cargo build --release
```

编译产物在 `target/release/imgclip`，将其复制到 `$PATH` 中的目录。

**Windows 用户：** 如遇剪贴板错误，请确认终端会话拥有剪贴板访问权限。

## 卸载

- **通过 `cargo install` 安装：** 执行 `cargo uninstall imgclip`。
- **下载的压缩包或手动拷贝的二进制：** 删除该可执行文件（Unix 可用 `which imgclip`，Windows 可用 `where imgclip` 查看路径）。
- **曾执行过 `--install`：** 先运行 `imgclip --uninstall` 移除开机自启项，再按上两种方式之一删除程序本体。

## 快速开始

```bash
# 一次性设置：安装开机自启（登录后自动监听剪贴板）
imgclip --install
```

就这样。重启（或重新登录）后，你复制的任何图片或截图都会自动保存到 `~/Pictures/imgclip/`（Windows 为 `%USERPROFILE%\Pictures\imgclip\`）。

如需移除自启：`imgclip --uninstall`。若要彻底卸载本工具，见上文 [卸载](#卸载)。

## 使用

### 监听模式（推荐）

```bash
# 开始监听（默认保存到 ~/Pictures/imgclip/）
imgclip --watch

# 以 JPEG 格式监听
imgclip --watch -f jpeg -q 90

# 指定保存目录和轮询间隔
imgclip --watch --dir ~/screenshots --interval 200

# 静默后台运行
imgclip --watch --quiet
```

### 单次模式

```bash
# 先复制一张图片（截图、浏览器图片等），然后：

# 保存到文件
imgclip -o screenshot.png

# 以质量 90 保存为 JPEG
imgclip -o photo.jpg -f jpeg -q 90

# 通过管道传递给其他命令（默认向 stdout 输出 PNG）
imgclip | some-command

# 获取 data URI（用于 HTML/CSS 内嵌）
imgclip --data-uri

# 写入临时文件并打印路径
imgclip --temp

# 静默模式
imgclip -o out.png --quiet
```

### 交互模式

```bash
# 监听剪贴板，检测到新图片时提示 [s]保存 / [d]丢弃 / [q]退出
imgclip --interactive

# 以 JPEG 格式交互保存
imgclip --interactive -f jpeg -q 90

# 自定义保存目录
imgclip --interactive --dir ~/screenshots
```

### 复制文件到剪贴板

```bash
# 将图片文件复制到剪贴板（支持 PNG、JPEG、BMP、WebP 等格式）
imgclip --copy photo.png
imgclip --copy screenshot.jpg
```

## 选项

| 选项 | 说明 |
|------|------|
| `--watch` | 监听剪贴板，自动保存新图片 |
| `--interactive` | 监听剪贴板，逐张选择保存或丢弃 |
| `--copy <FILE>` | 将图片文件复制到剪贴板 |
| `--install` | 安装开机自启（登录时运行 `--watch`） |
| `--uninstall` | 移除开机自启 |
| `-o, --output <PATH>` | 将图片保存到指定文件 |
| `-f, --format <FORMAT>` | 输出格式：`png`（默认）、`jpeg`/`jpg` |
| `-q, --quality <1-100>` | JPEG 质量（默认 85） |
| `--data-uri` | 输出为 `data:image/...;base64,...` 字符串 |
| `--temp` | 写入临时文件，将路径打印到 stdout |
| `--dir <PATH>` | `--watch`/`--interactive` 保存目录（默认 ~/Pictures/imgclip） |
| `--interval <MS>` | `--watch`/`--interactive` 轮询间隔（毫秒，默认 500） |
| `--quiet` | 静默信息输出 |
| `-h, --help` | 显示帮助 |
| `-V, --version` | 显示版本 |

## 示例

```bash
# 快速截图 → 剪贴板 → 文件
imgclip -o shot.png

# 在 HTML 中内嵌图片
echo "<img src=\"$(imgclip --data-uri)\" />"

# 将剪贴板图片转为 JPEG 并通过 curl 上传
imgclip -f jpeg -q 80 | curl -T - https://upload.example.com

# 在脚本中自动保存截图
imgclip --temp --quiet | xargs -I{} mv {} ~/screenshots/

# 复制文件到剪贴板，然后粘贴到 Slack/文档
imgclip --copy diagram.png

# 选择性截图：只保存想要的截图
imgclip --interactive --dir ~/screenshots
```

## 许可证

[MIT](LICENSE)
