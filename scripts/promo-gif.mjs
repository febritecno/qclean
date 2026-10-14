#!/usr/bin/env node
/**
 * Render docs/promo/index.html frame by frame into docs/promo/qclean.gif.
 *
 * The page is deterministic (every frame is a function of the timeline time),
 * so this seeks, screenshots, and lets ffmpeg assemble the result.
 *
 * Needs: ffmpeg on PATH, and `puppeteer-core` (not a project dependency —
 * `npm i --no-save puppeteer-core`). Point BROWSER at any Chromium-based
 * browser binary.
 */
import { execFileSync } from 'node:child_process';
import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import { pathToFileURL } from 'node:url';

const FPS = Number(process.env.FPS ?? 15);
const WIDTH = Number(process.env.WIDTH ?? 720);
const OUT = 'docs/promo/qclean.gif';
const browserPath =
	process.env.BROWSER ?? '/Applications/Google Chrome.app/Contents/MacOS/Google Chrome';

const { default: puppeteer } = await import('puppeteer-core');
const frames = fs.mkdtempSync(path.join(os.tmpdir(), 'qclean-promo-'));
const browser = await puppeteer.launch({
	executablePath: browserPath,
	headless: true,
	args: ['--no-first-run', '--hide-scrollbars', '--allow-file-access-from-files']
});
try {
	const page = await browser.newPage();
	await page.setViewport({ width: WIDTH, height: (WIDTH * 9) / 16 });
	await page.goto(pathToFileURL(path.resolve('docs/promo/index.html')).href + '?clean=1');
	await page.waitForSelector('html[data-ready]');
	await page.waitForFunction(() => typeof window.__seek === 'function');
	const duration = await page.evaluate(() => window.__duration);
	const total = Math.round(duration * FPS);
	for (let i = 0; i < total; i++) {
		await page.evaluate((t) => window.__seek(t), i / FPS);
		await page.screenshot({ path: path.join(frames, `${String(i).padStart(4, '0')}.png`) });
	}
	console.log(`captured ${total} frames`);
} finally {
	await browser.close();
}

const input = ['-y', '-framerate', String(FPS), '-i', path.join(frames, '%04d.png')];
const palette = path.join(frames, 'palette.png');
execFileSync('ffmpeg', [...input, '-vf', 'palettegen=max_colors=128:stats_mode=diff', palette], {
	stdio: 'ignore'
});
execFileSync(
	'ffmpeg',
	[
		...input,
		'-i',
		palette,
		'-lavfi',
		'paletteuse=dither=bayer:bayer_scale=5:diff_mode=rectangle',
		'-loop',
		'0',
		OUT
	],
	{ stdio: 'ignore' }
);
fs.rmSync(frames, { recursive: true, force: true });
console.log(`wrote ${OUT} (${(fs.statSync(OUT).size / 1e6).toFixed(1)} MB)`);
