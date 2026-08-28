import { test, expect } from '@playwright/test';
import AxeBuilder from '@axe-core/playwright';
import { execFile } from 'node:child_process';
import { promisify } from 'node:util';
import { mkdtemp, mkdir, copyFile, readFile, access } from 'node:fs/promises';
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

test('@claim:paid-policy valid license enables CI threshold and invalid license stays locked', async ({ page }) => {
  await page.route('https://api.sociobot.in/api/v1/products/docx-markdown-fidelity-report/verify?*', (route) => route.fulfill({ json: { valid: true, reason: 'ok', expires_at: null } }));
  await page.goto('/?license=browser-token');
  await expect(page).toHaveURL('http://127.0.0.1:4173/');
  await expect.poll(() => page.evaluate(() => localStorage.getItem('sb_license:docx-markdown-fidelity-report'))).toBe('browser-token');
  await expect(page.locator('#license-status')).toContainText('License active');
  await expect(page.getByRole('link', { name: 'Buy the team license' })).toHaveAttribute('href', 'https://api.sociobot.in/api/v1/products/docx-markdown-fidelity-report/checkout');
  const server = spawn(process.execPath, [join(repo, 'tests', 'license-server.mjs')], { cwd: repo, env: { ...process.env, PORT: '43999' }, stdio: ['ignore', 'pipe', 'inherit'] });
  await new Promise<void>((resolveReady, reject) => { server.stdout.once('data', () => resolveReady()); server.once('error', reject); });
  const run = async (token: string, output: string) => {
    try {
      await exec('cargo', ['run', '--quiet', '--', 'convert', sample, '--output', output, '--fail-on', 'error', '--license', token], { cwd: repo, env: { ...process.env, DOCX_FIDELITY_VERIFY_URL: 'http://127.0.0.1:43999/verify' } });
      return 0;
    } catch (error) { return Number((error as { code: number }).code); }
  };
  const root = await mkdtemp(join(tmpdir(), 'fidelity-license-'));
  try {
    expect(await run('valid-token', join(root, 'valid'))).toBe(3);
    expect(await run('invalid-token', join(root, 'invalid'))).toBe(4);
  } finally { server.kill(); }
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
  await page.getByRole('link', { name: 'Try it with sample data' }).focus();
  await page.keyboard.press('Enter');
  await expect(page).toHaveURL(/\/demo$/);
  await expect(page.locator('h1')).toBeFocused();
  await expect(page.getByText('Demo — sample data, nothing is saved')).toBeVisible();
  expect((await page.evaluate(() => document.documentElement.scrollWidth <= innerWidth))).toBe(true);
});
