import './style.css';

const PRODUCT = 'Docx Fidelity';
const SLUG = 'docx-markdown-fidelity-report';
const API = `https://api.sociobot.in/api/v1/products/${SLUG}`;
const LICENSE_KEY = `sb_license:${SLUG}`;
const VERDICT_KEY = `sb_license_verdict:${SLUG}`;
const DAY = 86_400_000;

type Route = { title: string; description: string; render: () => string };

const header = () => `
  <header class="site-header">
    <a class="wordmark" href="/" data-link aria-label="Docx Fidelity home"><span class="mark" aria-hidden="true">⌖</span> Docx Fidelity</a>
    <nav aria-label="Main navigation">
      <a href="/demo" data-link>Demo</a>
      <a href="/#install">Install</a>
      <a href="/privacy" data-link>Privacy</a>
    </nav>
  </header>`;

const footer = () => `
  <footer>
    <p>Convert DOCX and map every review risk.</p>
    <nav aria-label="Footer navigation"><a href="/privacy" data-link>Privacy</a><a href="/terms" data-link>Terms</a><a href="https://www.sociobot.in" rel="external">Built by Param Factory <span class="sr-only">(external)</span></a></nav>
    <p class="build">v0.1.0 · build 2026.08.28</p>
  </footer>`;

const contour = `<svg class="contours" viewBox="0 0 900 300" aria-hidden="true"><path d="M-30 240C130 100 220 315 385 175S680 40 940 135"/><path d="M-25 208C125 78 225 280 380 148S680 16 935 105"/><path d="M-20 176C120 56 230 245 375 121S680-8 930 75"/></svg>`;

const home = () => `
  ${header()}
  <main id="main" tabindex="-1">
    <section class="hero survey-grid">
      ${contour}
      <div class="hero-copy">
        <p class="eyebrow">Local conversion field report · v0.1</p>
        <h1>Convert DOCX. Map every review risk.</h1>
        <p class="lede">For teams moving Word documentation, it shows where Markdown needs a human check.</p>
        <div class="hero-action"><a class="button primary" href="/demo" data-link>Try it with sample data</a><span>See a complex file convert, then inspect its risk ledger.</span></div>
        <ul class="plain-facts" aria-label="Product facts">
          <li><span aria-hidden="true">01</span> Runs on your computer.</li>
          <li><span aria-hidden="true">02</span> Document data stays local.</li>
          <li><span aria-hidden="true">03</span> Free conversion. $29 CI checks.</li>
        </ul>
      </div>
      <figure class="hero-map">
        <img src="/terrain-report.webp" width="1280" height="853" alt="A survey map marks review risks across a document with tables and images." fetchpriority="high" />
        <figcaption>Every pin becomes a source-located finding.</figcaption>
      </figure>
    </section>

    <section class="ledger-section" aria-labelledby="ledger-heading">
      <div class="section-heading"><p class="eyebrow">Sample report · North Ridge guide</p><h2 id="ledger-heading">See the handoff before you publish</h2><p>The converter keeps useful Markdown and names each place that lost meaning.</p></div>
      <div class="ledger-wrap">
        <div class="status-strip"><span class="status blocked">● Blocked</span><span>9 findings</span><span>word/document.xml</span></div>
        <table>
          <caption class="sr-only">Sample fidelity findings by source location</caption>
          <thead><tr><th scope="col">Map ref</th><th scope="col">Finding</th><th scope="col">Source</th><th scope="col">Action</th></tr></thead>
          <tbody>
            <tr><td><span class="pin">01</span></td><td>Tracked edit</td><td>Paragraph 4</td><td>Confirm accepted text</td></tr>
            <tr><td><span class="pin">02</span></td><td>3×2 table</td><td>Table 1</td><td>Check merged cells</td></tr>
            <tr><td><span class="pin safe">03</span></td><td>Image extracted</td><td>Paragraph 11</td><td>Add useful alt text</td></tr>
            <tr><td><span class="pin">04</span></td><td>Embedded file</td><td>Package part</td><td>Inspect separately</td></tr>
          </tbody>
        </table>
      </div>
    </section>

    <section class="steps" aria-labelledby="steps-heading">
      <div class="section-heading"><p class="eyebrow">Three field steps</p><h2 id="steps-heading">Convert a whole document set</h2></div>
      <ol>
        <li><span>01</span><div><h3>Point at files</h3><p>Choose one DOCX or a directory. One command converts every DOCX in that directory.</p><code>docx-fidelity convert word-files/ --output migration/</code></div></li>
        <li><span>02</span><div><h3>Keep the useful parts</h3><p>Get Markdown, extracted images, a JSON ledger, and a human checklist.</p></div></li>
        <li><span>03</span><div><h3>Review exact locations</h3><p>Open each paragraph, table, row, or package part named in the report.</p></div></li>
      </ol>
    </section>

    <section class="install-band" id="install" aria-labelledby="install-heading">
      <div><p class="eyebrow">Single Rust binary</p><h2 id="install-heading">Run the real demo</h2><p>Build locally, then convert the bundled field guide in a fresh temporary directory.</p></div>
      <div class="command-stack">
        <div class="command-line"><code id="install-command">cargo install --path .<br />docx-fidelity demo</code><button class="copy-button" data-copy="cargo install --path .\ndocx-fidelity demo">Copy commands</button></div>
        <pre class="mini-recording" aria-label="Recorded output from the bundled demo"><code>$ docx-fidelity demo
Converted: field-guide.docx → field-guide.md
Report: blocked · 9 findings
Sandbox: /tmp/docx-fidelity-demo-…</code></pre>
      </div>
    </section>

    <section class="boundaries" aria-labelledby="boundaries-heading">
      <div><p class="eyebrow">Clear boundaries</p><h2 id="boundaries-heading">Know what stays untouched</h2></div>
      <ul><li>No OCR</li><li>No document editing</li><li>No PDF round-trip</li><li>No macro execution</li><li>No document upload</li></ul>
      <p>The CLI reads bounded ZIP and XML parts. Embedded files are reported, never opened.</p>
    </section>

    <section class="paid" aria-labelledby="paid-heading">
      <div class="price-mark"><span>$29</span><small>one time</small></div>
      <div><p class="eyebrow">Team license</p><h2 id="paid-heading">Make CI stop on review risks</h2><p>Conversion and every report stay free. The license adds <code>--fail-on warning|error</code> for migration checks in CI.</p><p class="legal-note">Sociobot/Dodo is the merchant of record. Refunds revoke the license.</p></div>
      <div class="license-actions">
        <a class="button primary" href="${API}/checkout">Buy the team license</a>
        <button class="button secondary" id="restore-toggle" aria-expanded="false" aria-controls="restore-form">Have a license?</button>
        <form id="restore-form" class="restore-form" hidden><label for="license-token">License token</label><div><input id="license-token" name="license" autocomplete="off" required /><button type="submit">Verify license</button></div><p id="license-message" aria-live="polite"></p></form>
        <p id="license-status" class="license-status" aria-live="polite"></p>
      </div>
    </section>
  </main>
  ${footer()}`;

const demo = () => `
  <div class="demo-banner" role="status"><span><strong>Demo</strong> — sample data, nothing is saved</span><span><button id="reset-demo">Reset demo</button><a href="/" data-link id="leave-demo">Start for real</a></span></div>
  ${header()}
  <main id="main" tabindex="-1" class="demo-main">
    <section class="demo-intro"><p class="eyebrow">Sandbox · bundled field guide</p><h1>Watch one DOCX map its risks.</h1><p>This recording uses the same sample shipped with the CLI. No account or document upload is involved.</p></section>
    <section class="terminal-shell" aria-labelledby="terminal-heading">
      <div class="terminal-bar"><span id="terminal-heading">docx-fidelity demo</span><button id="play-demo">Replay recording</button></div>
      <pre id="terminal-output" tabindex="0" aria-live="polite"><code></code></pre>
    </section>
    <section class="demo-result" aria-labelledby="demo-result-heading">
      <div><p class="eyebrow">Generated fidelity ledger</p><h2 id="demo-result-heading">The sample stops at six risk areas</h2><p>The Markdown remains usable. The ledger points reviewers back to the source.</p></div>
      <dl><div><dt>Tables</dt><dd>1 warning</dd></div><div><dt>Comments</dt><dd>1 warning</dd></div><div><dt>Revisions</dt><dd>2 warnings</dd></div><div><dt>Embedded objects</dt><dd>2 errors</dd></div><div><dt>Footnotes</dt><dd>1 moved</dd></div><div><dt>Images</dt><dd>1 extracted</dd></div></dl>
    </section>
    <a class="button primary next-step" href="/#install">Install the CLI</a>
  </main>
  ${footer()}`;

const privacy = () => `
  ${header()}<main id="main" tabindex="-1" class="prose-page"><p class="eyebrow">Policy · effective 28 August 2026</p><h1>Your documents stay on your computer.</h1><p>The CLI reads and writes local files. It has no telemetry and does not send document names or contents anywhere.</p><h2>License checks</h2><p>If you use a team license, the CLI sends only the token to Sociobot. The site stores that token and a dated verdict in your browser. You can remove both by clearing site storage.</p><h2>Demo data</h2><p>The browser demo loads only bundled sample files from this site. Demo state uses keys starting with <code>demo:</code>. Resetting or leaving the demo clears those keys.</p><h2>Site requests</h2><p>The static site loads fonts and art from its own origin. It has no analytics, advertising, or third-party scripts.</p><h2>Contact</h2><p>Email <a href="mailto:privacy@sociobot.in">privacy@sociobot.in</a> with a privacy question.</p></main>${footer()}`;

const terms = () => `
  ${header()}<main id="main" tabindex="-1" class="prose-page"><p class="eyebrow">Terms · effective 28 August 2026</p><h1>Use the report as a review aid.</h1><p>The software is provided under the MIT License. You remain responsible for checking converted Markdown before publishing it.</p><h2>Team license</h2><p>The $29 team license is a one-time purchase. It adds CI policy gates to the free conversion and report features. Sociobot/Dodo is the merchant of record and handles payment and refunds.</p><h2>Safe use</h2><p>Do not treat a clear report as proof that two formats render identically. Keep backups of source files. Do not use the tool to inspect documents you lack permission to access.</p><h2>Limits</h2><p>The tool does not perform OCR, edit documents, or open embedded objects. The MIT License contains the full warranty limits.</p><h2>Contact</h2><p>Email <a href="mailto:support@sociobot.in">support@sociobot.in</a> with a terms question.</p></main>${footer()}`;

const notFound = () => `
  ${header()}<main id="main" tabindex="-1" class="not-found">${contour}<p class="eyebrow">Map reference 404</p><h1>This path is outside the survey.</h1><p>The page moved or never existed. Return to the main report map.</p><a class="button primary" href="/" data-link>Return home</a></main>${footer()}`;

const routes: Record<string, Route> = {
  '/': { title: 'Docx Fidelity — convert DOCX and map review risks', description: 'Convert DOCX to Markdown locally and get a source-located report of tables, comments, revisions, images, footnotes, and styles.', render: home },
  '/demo': { title: 'Demo — Docx Fidelity', description: 'Watch the bundled DOCX sample convert to Markdown and a source-located fidelity report.', render: demo },
  '/privacy': { title: 'Privacy — Docx Fidelity', description: 'How Docx Fidelity processes documents locally and handles license tokens.', render: privacy },
  '/terms': { title: 'Terms — Docx Fidelity', description: 'Terms for Docx Fidelity and its one-time team license.', render: terms },
};

function renderRoute(focus = false) {
  const path = location.pathname.replace(/\/$/, '') || '/';
  const route = routes[path] ?? { title: 'Page not found — Docx Fidelity', description: 'This path is outside the Docx Fidelity survey.', render: notFound };
  document.title = route.title;
  document.querySelector<HTMLMetaElement>('meta[name="description"]')!.content = route.description;
  document.querySelector<HTMLLinkElement>('link[rel="canonical"]')!.href = `https://docx-markdown-fidelity-report.sociobot.in${path}`;
  document.querySelector<HTMLMetaElement>('meta[property="og:title"]')!.content = route.title;
  document.querySelector<HTMLMetaElement>('meta[property="og:description"]')!.content = route.description;
  document.querySelector<HTMLMetaElement>('meta[name="twitter:title"]')!.content = route.title;
  document.querySelector<HTMLMetaElement>('meta[name="twitter:description"]')!.content = route.description;
  document.getElementById('app')!.innerHTML = route.render();
  wireInteractions(path);
  if (focus) {
    scrollTo({ top: 0, behavior: matchMedia('(prefers-reduced-motion: reduce)').matches ? 'instant' : 'smooth' });
    const h1 = document.querySelector<HTMLHeadingElement>('h1');
    h1?.setAttribute('tabindex', '-1'); h1?.focus();
    document.getElementById('route-status')!.textContent = route.title;
  }
}

function navigate(path: string) {
  history.pushState({}, '', path);
  renderRoute(true);
}

function wireInteractions(path: string) {
  document.querySelectorAll<HTMLAnchorElement>('a[data-link]').forEach((link) => link.addEventListener('click', (event) => {
    if (event.metaKey || event.ctrlKey || event.shiftKey || link.target) return;
    event.preventDefault(); navigate(new URL(link.href).pathname);
  }));
  document.querySelectorAll<HTMLButtonElement>('[data-copy]').forEach((button) => button.addEventListener('click', async () => {
    await navigator.clipboard.writeText(button.dataset.copy || '');
    button.textContent = 'Commands copied';
    setTimeout(() => button.textContent = 'Copy commands', 1800);
  }));
  const toggle = document.querySelector<HTMLButtonElement>('#restore-toggle');
  toggle?.addEventListener('click', () => {
    const form = document.querySelector<HTMLFormElement>('#restore-form')!;
    const opening = form.hidden; form.hidden = !opening; toggle.setAttribute('aria-expanded', String(opening));
    if (opening) document.querySelector<HTMLInputElement>('#license-token')?.focus();
  });
  document.querySelector<HTMLFormElement>('#restore-form')?.addEventListener('submit', async (event) => {
    event.preventDefault(); const input = document.querySelector<HTMLInputElement>('#license-token')!; await saveAndVerifyLicense(input.value);
  });
  if (path === '/demo') wireDemo();
}

const demoLines = [
  '$ docx-fidelity demo',
  'Demo — sample data, nothing was read from your files',
  'Converted: field-guide.docx → field-guide.md',
  'Report: field-guide.fidelity.md (blocked, 9 findings)',
  '',
  'TABLES              1 warning   table 1, rows 1–3',
  'COMMENTS            1 warning   paragraph 2',
  'REVISIONS           2 warnings  paragraph 4',
  'EMBEDDED OBJECTS    2 errors    paragraph 13 + package',
  'FOOTNOTES           1 moved     paragraph 12',
  'IMAGES              1 extracted paragraph 11',
  '',
  'Sandbox: /tmp/docx-fidelity-demo-…'
];

function wireDemo() {
  const output = document.querySelector<HTMLElement>('#terminal-output code')!;
  const reduced = matchMedia('(prefers-reduced-motion: reduce)').matches;
  let timer = 0;
  const play = () => {
    clearInterval(timer); output.textContent = '';
    if (reduced) { output.textContent = demoLines.join('\n'); return; }
    let i = 0; timer = window.setInterval(() => { output.textContent += `${i ? '\n' : ''}${demoLines[i++]}`; if (i >= demoLines.length) clearInterval(timer); }, 110);
  };
  document.querySelector('#play-demo')?.addEventListener('click', play);
  document.querySelector('#reset-demo')?.addEventListener('click', () => { Object.keys(localStorage).filter((k) => k.startsWith('demo:')).forEach((k) => localStorage.removeItem(k)); play(); });
  document.querySelector('#leave-demo')?.addEventListener('click', () => Object.keys(localStorage).filter((k) => k.startsWith('demo:')).forEach((k) => localStorage.removeItem(k)));
  play();
}

async function saveAndVerifyLicense(token: string) {
  const message = document.querySelector<HTMLElement>('#license-message');
  const status = document.querySelector<HTMLElement>('#license-status');
  if (!token.trim()) { if (message) message.textContent = 'Paste a license token, then try again.'; return; }
  localStorage.setItem(LICENSE_KEY, token.trim());
  if (message) message.textContent = 'Checking this license…';
  try {
    const response = await fetch(`${API}/verify?license=${encodeURIComponent(token.trim())}`);
    const verdict = await response.json() as { valid: boolean; reason: string };
    localStorage.setItem(VERDICT_KEY, JSON.stringify({ ...verdict, checkedAt: Date.now() }));
    const result = verdict.valid ? 'License active. CI policy gates are ready.' : `License no longer active: ${verdict.reason}.`;
    if (message) message.textContent = result;
    if (status) status.textContent = result;
  } catch {
    if (message) message.textContent = 'The license check could not connect. Your free conversion still works.';
    if (status) status.textContent = 'License check unavailable. Free conversion still works.';
  }
}

async function handleReturnedLicense() {
  const url = new URL(location.href); const token = url.searchParams.get('license');
  if (token) { localStorage.setItem(LICENSE_KEY, token); url.searchParams.delete('license'); history.replaceState({}, '', `${url.pathname}${url.search}${url.hash}`); await saveAndVerifyLicense(token); return; }
  const stored = localStorage.getItem(LICENSE_KEY); const cached = localStorage.getItem(VERDICT_KEY);
  if (!stored) return;
  try { const verdict = cached ? JSON.parse(cached) as { valid: boolean; checkedAt: number } : null; if (verdict?.valid && Date.now() - verdict.checkedAt < DAY) return; } catch { /* verify below */ }
  await saveAndVerifyLicense(stored);
}

document.addEventListener('click', (event) => {
  const link = (event.target as Element).closest<HTMLAnchorElement>('a[href^="/#"]');
  if (link && location.pathname !== '/') { event.preventDefault(); history.pushState({}, '', link.pathname + link.hash); renderRoute(true); requestAnimationFrame(() => document.querySelector(link.hash)?.scrollIntoView()); }
});
window.addEventListener('popstate', () => renderRoute(true));
renderRoute();
handleReturnedLicense();
if ('serviceWorker' in navigator && location.hostname !== 'localhost') navigator.serviceWorker.register('/sw.js').catch(() => undefined);
