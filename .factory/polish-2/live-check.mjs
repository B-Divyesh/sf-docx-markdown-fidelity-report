import { chromium, request } from '@playwright/test';
import AxeBuilder from '@axe-core/playwright';
import { mkdir, writeFile } from 'node:fs/promises';
import { resolve } from 'node:path';

const origin = 'https://docx-markdown-fidelity-report.sociobot.in';
const evidence = resolve('.factory/polish-2/live');
await mkdir(evidence, { recursive: true });

const browser = await chromium.launch();
const context = await browser.newContext({ viewport: { width: 390, height: 844 } });
const errors = [];
const requests = [];
const page = await context.newPage();
page.on('pageerror', (error) => errors.push(String(error)));
page.on('console', (message) => {
  const expected404 = page.url().endsWith('/not-a-route') && /status of 404/i.test(message.text());
  if (message.type() === 'error' && !expected404) errors.push(message.text());
});
page.on('request', (outgoing) => requests.push(outgoing.url()));

const routes = [
  ['/', 200, 'Docx Markdown Fidelity Report — review DOCX', 'Convert DOCX and list review issues.'],
  ['/?demo=1', 200, 'Demo — Docx Markdown Fidelity Report', 'See the sample DOCX conversion report.'],
  ['/demo', 200, 'Demo — Docx Markdown Fidelity Report', 'See the sample DOCX conversion report.'],
  ['/privacy', 200, 'Privacy — Docx Markdown Fidelity Report', 'Your documents stay on your computer.'],
  ['/terms', 200, 'Terms — Docx Markdown Fidelity Report', 'Terms for using the CLI.'],
  ['/not-a-route', 404, 'Page not found — Docx Markdown Fidelity Report', 'Page not found.'],
];

const routeResults = [];
const links = new Set();
for (const [path, status, title, h1] of routes) {
  const response = await page.goto(`${origin}${path}`, { waitUntil: 'networkidle' });
  if (response?.status() !== status) throw new Error(`${path}: expected ${status}, got ${response?.status()}`);
  if ((await page.title()) !== title) throw new Error(`${path}: wrong title`);
  if ((await page.locator('h1').allTextContents()).join('') !== h1) throw new Error(`${path}: wrong h1`);
  if ((await page.locator('h1').count()) !== 1 || (await page.locator('main').count()) !== 1) throw new Error(`${path}: landmark or h1 count`);
  if ((await page.locator('html').getAttribute('lang')) !== 'en') throw new Error(`${path}: missing language`);
  const axe = await new AxeBuilder({ page }).withTags(['wcag2a', 'wcag2aa', 'wcag21a', 'wcag21aa']).analyze();
  if (axe.violations.length) throw new Error(`${path}: axe violations ${axe.violations.map((item) => item.id).join(', ')}`);
  for (const href of await page.locator('a').evaluateAll((anchors) => anchors.map((anchor) => anchor.href))) links.add(href);
  routeResults.push({ path, status, title, h1, axeViolations: 0 });
}

await page.goto(`${origin}/`, { waitUntil: 'networkidle' });
const landing = await page.locator('main').innerText();
if (!landing.includes('Stop automated checks when reports find selected risks.')) throw new Error('missing plain first-screen fact');
if (!landing.includes('Stop automated checks at selected risk levels')) throw new Error('missing plain automated-check heading');
if (!landing.includes('Use --fail-on warning|error to stop an automated check at the selected risk level.')) throw new Error('missing plain automated-check explanation');
if (/CI policy|Stop CI|risk ledger|fidelity ledger|Map every review risk/i.test(landing)) throw new Error('retired wording remains');
const firstScreenFits = await page.locator('.hero-action, .plain-facts').evaluateAll((elements) => elements.every((element) => element.getBoundingClientRect().bottom <= innerHeight));
if (!firstScreenFits) throw new Error('first-screen action or facts fall below 390x844 viewport');
await page.screenshot({ path: `${evidence}/home-mobile.png`, fullPage: true });

await page.getByRole('link', { name: 'Privacy', exact: true }).first().click();
if (!(await page.locator('h1').evaluate((node) => node === document.activeElement))) throw new Error('route h1 did not receive focus');
await page.goBack({ waitUntil: 'networkidle' });
if (!(await page.locator('h1').evaluate((node) => node === document.activeElement))) throw new Error('back navigation did not restore route focus');

requests.length = 0;
await page.goto(`${origin}/?demo=1`, { waitUntil: 'networkidle' });
if (!requests.every((url) => new URL(url).origin === origin)) throw new Error('demo made a cross-origin request');
if (!(await page.getByText('Demo — sample data, nothing is saved to your files').isVisible())) throw new Error('demo banner missing');
await page.evaluate(() => {
  localStorage.setItem('demo:old', 'discard');
  localStorage.setItem('real:review', 'keep');
});
await page.getByRole('button', { name: 'Reset demo' }).click();
let storage = await page.evaluate(() => ({ old: localStorage.getItem('demo:old'), sample: localStorage.getItem('demo:sample'), real: localStorage.getItem('real:review') }));
if (JSON.stringify(storage) !== JSON.stringify({ old: null, sample: 'field-guide', real: 'keep' })) throw new Error('reset crossed demo storage boundary');
await page.screenshot({ path: `${evidence}/demo-mobile.png`, fullPage: true });
await page.getByRole('link', { name: 'Start for real' }).click();
storage = await page.evaluate(() => ({ sample: localStorage.getItem('demo:sample'), real: localStorage.getItem('real:review') }));
if (JSON.stringify(storage) !== JSON.stringify({ sample: null, real: 'keep' })) throw new Error('leaving demo crossed storage boundary');

await page.goto(`${origin}/?demo=1`, { waitUntil: 'networkidle' });
await page.evaluate(() => { document.documentElement.style.fontSize = '200%'; });
if (!(await page.evaluate(() => document.documentElement.scrollWidth <= innerWidth))) throw new Error('demo overflows at 200% text');
await page.screenshot({ path: `${evidence}/demo-text-200.png`, fullPage: true });

await page.goto(`${origin}/not-a-route`, { waitUntil: 'networkidle' });
await page.screenshot({ path: `${evidence}/not-found-mobile.png`, fullPage: true });

await page.goto(`${origin}/`, { waitUntil: 'networkidle' });
await page.waitForFunction(() => navigator.serviceWorker.ready.then(() => true));
await page.reload({ waitUntil: 'networkidle' });
await page.waitForFunction(() => Boolean(navigator.serviceWorker.controller));
await context.setOffline(true);
await page.reload({ waitUntil: 'domcontentloaded' });
if (!(await page.getByRole('heading', { level: 1 }).isVisible())) throw new Error('offline shell did not reload');
await context.setOffline(false);

const api = await request.newContext();
const linkResults = [];
for (const href of [...links].sort()) {
  if (href.startsWith('mailto:') || href.includes('#')) continue;
  const response = await api.get(href);
  if (response.status() >= 400) throw new Error(`dead link ${href}: ${response.status()}`);
  linkResults.push({ href, status: response.status() });
}
await api.dispose();

if (errors.length) throw new Error(`browser errors: ${errors.join(' | ')}`);
const result = {
  checkedAt: new Date().toISOString(),
  origin,
  routes: routeResults,
  links: linkResults,
  firstScreenFits,
  sameOriginDemoRequests: true,
  demoIsolation: true,
  routeFocusAndBack: true,
  offlineReload: true,
  consoleErrors: errors,
};
await writeFile(`${evidence}/live-check.json`, `${JSON.stringify(result, null, 2)}\n`);
console.log(JSON.stringify(result, null, 2));
await browser.close();
