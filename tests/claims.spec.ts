import { test, expect } from '@playwright/test';
import AxeBuilder from '@axe-core/playwright';
import { execFile } from 'node:child_process';
import { promisify } from 'node:util';
import { mkdtemp, mkdir, copyFile, readFile, access, readdir } from 'node:fs/promises';
import { tmpdir } from 'node:os';
import { join, resolve } from 'node:path';
import { spawn } from 'node:child_process';

const exec = promisify(execFile);
const repo = resolve(import.meta.dirname, '..');
const sample = join(repo, 'examples', 'field-guide.docx');

test('@claim:demo-conversion bundled demo writes Markdown and reports', async () => {
  const { stdout } = await exec('cargo', ['run', '--quiet', '--', '--json', 'demo'], { cwd: repo });
  const result = JSON.parse(stdout.trim());
  expect(result.converted).toBe(1);
  expect(result.outputs[0].findings).toBeGreaterThan(6);
  await access(result.outputs[0].markdown);
  await access(result.outputs[0].report);
  await access(join(result.outputs[0].markdown.replace(/\.md$/, '.media'), 'image-001.svg'));
});

test('@claim:local-processing demo sends no document data away', async ({ page }) => {
  const seen: string[] = [];
  page.on('request', (request) => seen.push(request.url()));
  await page.goto('/demo');
  await expect(page.getByRole('heading', { level: 1 })).toContainText('Watch one DOCX');
  await expect(page.locator('#terminal-output')).toContainText('field-guide.fidelity.md');
  expect(seen.every((url) => new URL(url).origin === 'http://127.0.0.1:4173')).toBe(true);
  const root = await mkdtemp(join(tmpdir(), 'fidelity-offline-'));
  const { stdout } = await exec('cargo', ['run', '--quiet', '--', '--json', 'convert', sample, '--output', join(root, 'out')], { cwd: repo, env: { ...process.env, HTTP_PROXY: 'http://127.0.0.1:1', HTTPS_PROXY: 'http://127.0.0.1:1', NO_PROXY: '' } });
  expect(JSON.parse(stdout).converted).toBe(1);
});

test('@claim:risk-ledger complex DOCX reports every promised category and location', async () => {
  const root = await mkdtemp(join(tmpdir(), 'fidelity-ledger-'));
  const output = join(root, 'out');
  await exec('cargo', ['run', '--quiet', '--', 'convert', sample, '--output', output], { cwd: repo });
  const report = JSON.parse(await readFile(join(output, 'field-guide.fidelity.json'), 'utf8'));
  const markdown = await readFile(join(output, 'field-guide.md'), 'utf8');
  expect(markdown).toContain('**before the trail opens**');
  expect(markdown).toContain('[*trail notices*](https://example.com/trail-notices)');
  for (const category of ['tables', 'comments', 'revisions', 'embedded_objects', 'footnotes', 'styles', 'images']) expect(report.counts[category]).toBeGreaterThan(0);
  expect(report.findings.every((finding: { location: { part?: string } }) => Boolean(finding.location.part))).toBe(true);
  expect(report.findings.some((finding: { location: { paragraph?: number } }) => Boolean(finding.location.paragraph))).toBe(true);
});

test('@claim:batch-conversion a directory creates one result per DOCX', async () => {
  const root = await mkdtemp(join(tmpdir(), 'fidelity-batch-'));
  const input = join(root, 'in'); const output = join(root, 'out');
  await mkdir(input); await copyFile(sample, join(input, 'guide-one.docx')); await copyFile(sample, join(input, 'guide-two.docx'));
  const { stdout } = await exec('cargo', ['run', '--quiet', '--', '--json', 'convert', input, '--output', output], { cwd: repo });
  const result = JSON.parse(stdout.trim());
  expect(result.converted).toBe(2);
  await access(join(output, 'guide-one.md')); await access(join(output, 'guide-two.fidelity.json'));
});

test('@claim:ci-policy policy gates work locally with the network unavailable', async () => {
  const root = await mkdtemp(join(tmpdir(), 'fidelity-policy-'));
  let code = 0;
  try {
    await exec('cargo', ['run', '--quiet', '--', 'convert', sample, '--output', join(root, 'out'), '--fail-on', 'error'], {
      cwd: repo,
      env: { ...process.env, HTTP_PROXY: 'http://127.0.0.1:1', HTTPS_PROXY: 'http://127.0.0.1:1', NO_PROXY: '' },
    });
  } catch (error) { code = Number((error as { code: number }).code); }
  expect(code).toBe(3);
});

test('@regression:legacy-license-verify an existing token can still be checked directly', async () => {
  const server = spawn(process.execPath, [join(repo, 'tests', 'license-server.mjs')], { cwd: repo, env: { ...process.env, PORT: '43999' }, stdio: ['ignore', 'pipe', 'inherit'] });
  await new Promise<void>((resolveReady, reject) => { server.stdout.once('data', () => resolveReady()); server.once('error', reject); });
  try {
    const valid = await exec('cargo', ['run', '--quiet', '--', '--json', 'license', 'verify', 'valid-token'], { cwd: repo, env: { ...process.env, DOCX_FIDELITY_VERIFY_URL: 'http://127.0.0.1:43999/verify' } });
    expect(JSON.parse(valid.stdout)).toMatchObject({ valid: true, reason: 'ok' });
    await expect(exec('cargo', ['run', '--quiet', '--', 'license', 'verify', 'invalid-token'], { cwd: repo, env: { ...process.env, DOCX_FIDELITY_VERIFY_URL: 'http://127.0.0.1:43999/verify' } })).rejects.toMatchObject({ code: 4 });
  } finally {
    server.kill();
  }
});

test('@regression:unregistered-checkout the site ships no unavailable purchase flow and immutable asset policy', async ({ page }) => {
  const requests: string[] = [];
  page.on('request', (request) => requests.push(request.url()));
  await page.goto('/');
  await expect(page.getByRole('heading', { name: 'Stop CI on review risks' })).toBeVisible();
  expect(await page.locator('a[href*="checkout"]').count()).toBe(0);
  expect(await page.content()).not.toContain('api.sociobot.in');
  expect(requests.every((url) => new URL(url).origin === 'http://127.0.0.1:4173')).toBe(true);

  const config = JSON.parse(await readFile(join(repo, 'site', 'public', 'staticwebapp.config.json'), 'utf8')) as { routes: Array<{ route: string; headers?: Record<string, string> }> };
  const assetRoute = config.routes.find((route) => route.route === '/assets/*');
  expect(assetRoute?.headers?.['Cache-Control']).toBe('public, max-age=31536000, immutable');
  const assets = await readdir(join(repo, 'dist', 'site', 'assets'));
  expect(assets.some((asset) => /^index-[\w-]+\.js$/.test(asset))).toBe(true);
  expect(assets.some((asset) => /^index-[\w-]+\.css$/.test(asset))).toBe(true);
});

test('@offline-update the service worker reloads the shell without reachable network and uses the network for later navigations', async ({ page }) => {
  await page.goto('/');
  await page.waitForFunction(() => navigator.serviceWorker.ready.then(() => true));
  await page.reload();
  await page.waitForFunction(() => Boolean(navigator.serviceWorker.controller));
  await expect.poll(() => page.evaluate(async () => {
    const script = document.querySelector<HTMLScriptElement>('script[type="module"]')?.src;
    const cacheNames = await caches.keys();
    return Boolean(script) && (await Promise.all(cacheNames.map(async (name) => (await caches.open(name)).match(script!)))).some(Boolean);
  })).toBe(true);
  await page.route('**/*', (route) => route.abort());
  try {
    await page.reload();
    await expect(page.getByRole('heading', { level: 1 })).toContainText('Convert DOCX');
  } finally {
    await page.unrouteAll({ behavior: 'ignoreErrors' });
  }
  const worker = await readFile(join(repo, 'dist', 'site', 'sw.js'), 'utf8');
  expect(worker).toContain("event.request.mode === 'navigate'");
  expect(worker).toContain('fetch(event.request).then');
  expect(worker).not.toContain('__PRECACHE_ASSETS__');
  expect(worker).not.toContain('__BUILD_ID__');
  expect(worker).not.toContain('undefined');
});

test('@claim:safe-input unsafe archive paths are rejected without extraction', async () => {
  const root = await mkdtemp(join(tmpdir(), 'fidelity-unsafe-'));
  const input = join(root, 'unsafe.docx');
  const script = "import sys,zipfile; z=zipfile.ZipFile(sys.argv[1],'w'); z.writestr('../escaped.txt','must not escape'); z.writestr('word/document.xml','<w:document/>'); z.close()";
  await exec('python3', ['-c', script, input]);
  let code = 0; let stderr = '';
  try { await exec('cargo', ['run', '--quiet', '--', 'convert', input, '--output', join(root, 'out')], { cwd: repo }); }
  catch (error) { code = Number((error as { code: number }).code); stderr = String((error as { stderr: string }).stderr); }
  expect(code).toBe(2);
  expect(stderr).toContain('unsafe archive path');
  await expect(access(join(root, 'escaped.txt'))).rejects.toThrow();
});

for (const route of ['/', '/demo', '/privacy', '/terms', '/not-a-route']) {
  test(`accessible page ${route}`, async ({ page }) => {
    await page.goto(route);
    await expect(page.locator('html')).toHaveAttribute('lang', 'en');
    await expect(page.locator('main')).toHaveCount(1);
    await expect(page.locator('h1')).toHaveCount(1);
    const results = await new AxeBuilder({ page }).analyze();
    expect(results.violations.filter((item) => ['serious', 'critical'].includes(item.impact || ''))).toEqual([]);
  });
}

test('@mobile first screen and demo keyboard path work at 390px', async ({ page }) => {
  await page.goto('/');
  await expect(page.getByRole('heading', { level: 1 })).toBeVisible();
  const results = await new AxeBuilder({ page }).analyze();
  expect(results.violations.filter((item) => ['serious', 'critical'].includes(item.impact || ''))).toEqual([]);
  await page.getByRole('link', { name: 'Try it with sample data' }).focus();
  await page.keyboard.press('Enter');
  await expect(page).toHaveURL(/\/demo$/);
  await expect(page.locator('h1')).toBeFocused();
  await expect(page.getByText('Demo — sample data, nothing is saved')).toBeVisible();
  expect((await page.evaluate(() => document.documentElement.scrollWidth <= innerWidth))).toBe(true);
});
