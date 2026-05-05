const puppeteer = require('puppeteer-core');
const path = require('path');
const fs = require('fs');
const { execSync } = require('child_process');

const CHROME_PATH = 'C:\\Program Files\\Google\\Chrome\\Application\\chrome.exe';
const DEMO_HTML = path.resolve(__dirname, 'demo.html');
const FRAMES_DIR = path.resolve(__dirname, 'frames');
const OUTPUT_GIF = path.resolve(__dirname, '..', '..', 'docs', 'demo.gif');

async function main() {
  // Clean/create frames dir
  if (fs.existsSync(FRAMES_DIR)) fs.rmSync(FRAMES_DIR, { recursive: true });
  fs.mkdirSync(FRAMES_DIR, { recursive: true });

  console.log('Launching Chrome...');
  const browser = await puppeteer.launch({
    executablePath: CHROME_PATH,
    headless: 'new',
    args: ['--no-sandbox', '--disable-gpu', '--disable-software-rasterizer'],
    defaultViewport: { width: 960, height: 540, deviceScaleFactor: 2 },
  });

  const page = await browser.newPage();
  await page.goto(`file:///${DEMO_HTML.replace(/\\/g, '/')}`, { waitUntil: 'networkidle0' });

  // Wait for demo to be ready
  await page.waitForFunction(() => window.__demoReady === true, { timeout: 5000 });

  console.log('Recording frames...');
  let frame = 0;
  const FPS = 15;
  const INTERVAL = 1000 / FPS;

  // Capture frames until demo is done
  const start = Date.now();
  while (true) {
    const done = await page.evaluate(() => window.__demoDone === true);
    if (done) break;

    const framePath = path.join(FRAMES_DIR, `frame_${String(frame).padStart(5, '0')}.png`);
    await page.screenshot({ path: framePath, type: 'png' });
    frame++;

    await new Promise(r => setTimeout(r, INTERVAL));

    // Safety: max 30 seconds
    if (Date.now() - start > 30000) {
      console.log('Timeout reached, stopping capture');
      break;
    }
  }

  // Capture a few more frames at the end for a pause
  for (let i = 0; i < FPS * 2; i++) {
    const framePath = path.join(FRAMES_DIR, `frame_${String(frame).padStart(5, '0')}.png`);
    await page.screenshot({ path: framePath, type: 'png' });
    frame++;
  }

  console.log(`Captured ${frame} frames`);
  await browser.close();

  // Find ffmpeg
  const ffmpegPaths = [
    'ffmpeg',
    path.join(process.env.LOCALAPPDATA || '', 'Microsoft', 'WinGet', 'Packages',
      ...fs.readdirSync(path.join(process.env.LOCALAPPDATA || '', 'Microsoft', 'WinGet', 'Packages'))
        .filter(d => d.startsWith('Gyan.FFmpeg'))[0] || '',
      ...['ffmpeg-8.1.1-full_build', 'bin', 'ffmpeg.exe'])
  ];

  let ffmpeg = 'ffmpeg';
  for (const p of ffmpegPaths) {
    try {
      if (fs.existsSync(p) || execSync(`where ${p} 2>nul`).toString().trim()) {
        ffmpeg = p;
        break;
      }
    } catch {}
  }

  // Try to find ffmpeg in PATH with a broader search
  try {
    const wingetPkgs = path.join(process.env.LOCALAPPDATA, 'Microsoft', 'WinGet', 'Packages');
    const dirs = fs.readdirSync(wingetPkgs).filter(d => d.startsWith('Gyan.FFmpeg'));
    if (dirs.length > 0) {
      const binDir = fs.readdirSync(path.join(wingetPkgs, dirs[0]))[0];
      ffmpeg = path.join(wingetPkgs, dirs[0], binDir, 'bin', 'ffmpeg.exe');
    }
  } catch {}

  console.log(`Using ffmpeg: ${ffmpeg}`);
  console.log('Generating GIF...');

  // Use ffmpeg to create GIF with optimized palette
  const palette = path.join(FRAMES_DIR, 'palette.png');

  execSync(`"${ffmpeg}" -y -framerate ${FPS} -i "${path.join(FRAMES_DIR, 'frame_%05d.png')}" -vf "palettegen=max_colors=128:stats_mode=diff" "${palette}"`, { stdio: 'inherit' });

  execSync(`"${ffmpeg}" -y -framerate ${FPS} -i "${path.join(FRAMES_DIR, 'frame_%05d.png')}" -i "${palette}" -lavfi "paletteuse=dither=sierra2_4a" -loop 0 "${OUTPUT_GIF}"`, { stdio: 'inherit' });

  // Cleanup frames
  fs.rmSync(FRAMES_DIR, { recursive: true });

  const sizeMB = (fs.statSync(OUTPUT_GIF).size / 1024 / 1024).toFixed(2);
  console.log(`\nDone! GIF saved to: ${OUTPUT_GIF} (${sizeMB} MB)`);
}

main().catch(err => {
  console.error('Error:', err.message || err);
  process.exit(1);
});
