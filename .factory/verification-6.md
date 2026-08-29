# Independent verification 6 — PASS

- Candidate: `69841b4a6701b64b80ddc560c14bf51e616043d3` (`docs: record polish verification`)
- Live URL: <https://docx-markdown-fidelity-report.sociobot.in>
- Verified: 2026-08-29 UTC

## Verdict

**PASS.** Fresh evidence shows the live static deployment is the candidate build and the local CLI completes the researched core job: direct DOCX-to-Markdown conversion with per-file JSON and human-review fidelity reports that locate tables, comments, revisions, embedded objects, footnotes, styles, and image results. No deployment-only failure was reproduced.

## Required claim and first-read gates

After `npm ci`, I ran every exact command in `.factory/claims.json` independently from the clean candidate. All 13 passed with one matching claim test each:

`demo-conversion`, `local-processing`, `fidelity-report`, `source-fidelity`, `batch-conversion`, `ci-policy`, `safe-input`, `demo-isolation`, `review-checklist`, `single-binary`, `rust-toolchain`, `scope-boundaries`, and `mit-license`.

Claim logs: `/tmp/docx-fidelity-claims-8jt1wA/`.

Cold live first read, before interacting: it says “Convert DOCX and list review issues,” identifies “teams moving Word documentation,” and presents **Try it with sample data**, with the adjacent explanation that it converts the bundled field guide and opens its fidelity report. This passes the plain-words gate. The action opens `/?demo=1` in one click; the first demo view shows the conversion recording plus the persistent “Demo — sample data, nothing is saved to your files” banner, Reset demo, and Start for real.

## Local build, package, and CLI

All passed:

```sh
npm ci
npm test                  # 9 Rust unit tests, 1 doctest, 31 Playwright tests
npm run typecheck
npm run lint              # rustfmt, clippy -D warnings, TypeScript
npm run build             # dist/bin/docx-fidelity and dist/site
npm run package           # docx-markdown-fidelity-report-0.1.3.crate
```

- The release CLI `--json demo` created a fresh OS-temp sandbox and reported one converted file, `blocked`, with 9 findings.
- Invalid non-DOCX input exited 2 with an actionable “input must be a .docx file” error.
- I extracted the produced crate, installed it with `cargo install --locked --path ... --root ...` into `/tmp/docx-fidelity-consumer-tZCCff`, and ran `--version`, `--help`, and `--json demo`; it reported `0.1.3` and the expected 9 findings.
- A clean consumer binary depending on the extracted crate called `docx_fidelity::convert_path` on the packaged sample. It wrote `field-guide.md`, `.fidelity.json`, `.fidelity.md`, and `image-001.svg`, and returned `Blocked 9`.

## Live deployment, privacy, and browser QA

- Current production `index.html`, JavaScript, CSS, fonts, art, service worker, legal/metadata files, and the test-size stylesheet all match the candidate build byte-for-byte (16 served files).
- `/`, `/?demo=1`, `/demo`, `/privacy`, and `/terms` returned 200. `/not-a-route` returned the designed page with HTTP 404.
- `verify-url.sh` against live passed: title, `lang=en`, one H1, main landmark, image alt text, labeled buttons, and no page/console errors. Cold load was 847 ms in that check.
- Playwright axe scans on `/`, `/demo`, `/privacy`, `/terms`, `/not-a-route`, and 390 px `/?demo=1` found zero serious or critical violations. At 390 px the checked routes had `scrollWidth === clientWidth === 390`.
- Keyboard: first Tab reaches the skip link with a visible `rgb(184, 58, 45) solid 3px` focus outline; Enter moves focus to `#main`. Keyboard activation of the demo link moves focus to the demo H1. Reset clears injected `demo:` keys while preserving `real:` keys; Start for real clears only demo state and returns home.
- A complete landing → demo → replay → reset → leave request log contained only `https://docx-markdown-fidelity-report.sociobot.in` assets. No cookies, analytics, upload, or third-party request was observed. Header CSP has `connect-src 'self'`, `object-src 'none'`, and header-delivered `frame-ancestors 'none'`; HSTS, `nosniff`, strict-origin referrer policy, and permissions policy are present.
- Service worker scope/active script is `/sw.js`; `registration.update()` completed. After activation, `/demo` reloaded offline with its H1 and demo banner.
- Cache policy is 30-second revalidation for HTML/service worker and `public, max-age=31536000, immutable` for hashed JS/CSS.
- Live mobile Lighthouse: performance 98, accessibility 100, best practices 100, SEO 100; FCP 1.2 s, LCP 2.1 s, TBT 120 ms, CLS 0.001. Initial JS is 14,600 B raw / 5,311 B gzip; CSS is 12,973 B raw / 3,732 B gzip; self-hosted fonts total 38,024 B; hero art is 162,082 B.

## Endpoint allowance

The documented CLI license-verification endpoint was tested with 35 distinct synthetic invalid tokens from one client. Requests 1–30 returned 200; request 31 returned **429** with `Retry-After: 3`, and 32–35 returned 429 with `Retry-After: 2`. Observed allowance: 30 requests per current window. No sign-in flow exists.

## Findings

| Severity | Finding | Evidence / disposition |
| --- | --- | --- |
| P0 | None | — |
| P1 | None | — |
| P2 | The original researched brief names one-time purchase monetization, but the released product is free/MIT and has no checkout. | Fresh `GET https://api.sociobot.in/api/v1/products/docx-markdown-fidelity-report/checkout` returned 404 `{ "error": "enabled factory product" }`. The checkout cannot presently be integrated; the product does not falsely advertise a price or a broken purchase. This is an honest scope deviation, not a core conversion/review failure. |

## Reproduce

```sh
npm ci
npm test
npm run lint
npm run build
npm run package
./dist/bin/docx-fidelity demo
npm run preview -- --host 127.0.0.1 --port 4173
```
