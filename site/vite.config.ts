import { defineConfig, type Plugin } from 'vite';
import { copyFileSync, readdirSync, readFileSync, writeFileSync } from 'node:fs';
import { resolve } from 'node:path';

const output = resolve(import.meta.dirname, '../dist/site');

const precacheShell = (): Plugin => ({
  name: 'precache-docx-fidelity-shell',
  closeBundle() {
    const assets = readdirSync(resolve(output, 'assets')).filter((entry) => /\.(?:js|css)$/.test(entry));
    const script = assets.find((entry) => entry.endsWith('.js'));
    if (!script) throw new Error('The application bundle is missing.');
    const workerPath = resolve(output, 'sw.js');
    const worker = readFileSync(workerPath, 'utf8')
      .replace('__BUILD_ID__', script)
      .replace('__PRECACHE_ASSETS__', JSON.stringify(assets.map((entry) => `/assets/${entry}`)));
    if (worker.includes('__BUILD_ID__') || worker.includes('__PRECACHE_ASSETS__')) throw new Error('The service worker precache placeholders were not replaced.');
    writeFileSync(workerPath, worker);
    copyFileSync(resolve(output, 'index.html'), resolve(output, '404.html'));
  },
});

export default defineConfig({
  root: resolve(import.meta.dirname),
  publicDir: resolve(import.meta.dirname, 'public'),
  build: {
    outDir: output,
    emptyOutDir: true,
    target: 'es2022',
    sourcemap: true,
  },
  plugins: [precacheShell()],
});
