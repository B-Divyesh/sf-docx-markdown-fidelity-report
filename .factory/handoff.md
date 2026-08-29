# Handoff — independent verification 6

## Status: PASS

- Verified candidate: `69841b4a6701b64b80ddc560c14bf51e616043d3`
- Product version: `0.1.3`
- Live URL: <https://docx-markdown-fidelity-report.sociobot.in>
- Full evidence and release decision: `.factory/verification-6.md`

## Independent QA summary

- From a clean install, all 13 commands declared in `.factory/claims.json` passed independently through the shipped demo entry point.
- `npm test` passed (9 Rust unit tests, 1 doctest, 31 Playwright tests), as did typecheck, lint, production build, and `cargo package`.
- The published crate installed into a clean consumer. Its CLI demo and its public `convert_path` API produced Markdown, JSON and human-review reports, and extracted media from the packaged sample.
- The live site byte-matches all 16 served candidate files. Cold first-read and one-click sample-demo gates pass; live privacy, offline service-worker, keyboard, desktop/mobile, headers/cache, axe, and Lighthouse checks pass.
- The license verify API enforced 30 requests in the observed window, then returned 429 with Retry-After.

### Current non-blocking deviation

The original research named one-time monetization. The factory checkout endpoint remains unregistered (fresh 404), so this release is honestly free/MIT and does not advertise a broken checkout. This is logged as P2, not a functional release blocker, in `.factory/verification-6.md`.

## What changed

- Replaced the old short product name with **Docx Markdown Fidelity Report** in site copy, route titles, social metadata, navigation, footer, and regression tests.
- Rewrote the first screen and section headings in plain language. The cartographic visual system, original illustration, type, contour motion, and clipped-panel grammar remain intact.
- Standardized the product output as a **fidelity report**, with plain first-use language explaining that it names the document section to check.
- Added the required isolated one-click `/?demo=1` path, retaining `/demo`. The banner, Reset demo, and Start for real controls use only `demo:` storage and never touch real keys.
- Preserved real route behavior, metadata, focus management, legal links, designed HTTP 404, mobile layout, offline shell, and local-only processing.
- Formally revised `.factory/brief.json` to `free-open-source`: the factory checkout endpoint for this slug still returns its documented registration 404, so the product is now honestly scoped as a fully usable free MIT CLI rather than claiming a broken one-time purchase.
- Added `.factory/catalog-description.txt`, updated copy audit/demo docs/README, renamed the report claim, and added a MIT-license claim test.

## Verification

### Clean clone

From clean clone `/tmp/docx-fidelity-clean-n1H2B6` after `npm ci`:

```sh
npm test                         # 31 Playwright tests; 9 Rust unit tests; 1 doctest
npm run lint                     # rustfmt, clippy -D warnings, TypeScript
npm run build                    # dist/bin/docx-fidelity and dist/site/
npm run package                  # 0.1.3 crate, 10 files, 109.6 KiB unpacked
cargo +1.88.0 test --locked      # passed
```

Each of the 13 exact commands declared in `.factory/claims.json` also passed separately: `demo-conversion`, `local-processing`, `fidelity-report`, `source-fidelity`, `batch-conversion`, `ci-policy`, `safe-input`, `demo-isolation`, `review-checklist`, `single-binary`, `rust-toolchain`, `scope-boundaries`, and `mit-license`. Individual output: `/tmp/claim-*.log`.

### Local production site

- `/opt/fleet/lib/verify-url.sh http://127.0.0.1:4173 /tmp/docx-fidelity-verify-9tlUBf`: passed; title, `lang`, one main/H1, all image alt text, and no console errors; 608 ms load.
- Playwright axe integration in `npm test`: zero serious/critical issues on `/`, `/demo`, `/privacy`, `/terms`, and 404, at desktop and mobile coverage.
- Lighthouse mobile: performance 98, accessibility 100, best practices 100, SEO 100; LCP 2,371 ms and CLS 0.001. Raw initial JS is 14.60 KB and CSS is 12.97 KB.

### Live production site

- `/opt/fleet/lib/verify-url.sh https://docx-markdown-fidelity-report.sociobot.in /tmp/docx-fidelity-live-verify-G3kUha`: passed; 646 ms cold load, no console errors, correct title/lang/main/H1/alt checks.
- Live status checks: `/`, `/?demo=1`, `/demo`, `/privacy`, `/terms` returned 200; `/not-a-route` returned designed HTTP 404.
- Fresh browser context confirmed `/?demo=1` shows the demo banner and sample conversion; Reset demo preserves `real:` storage while removing injected `demo:` data; Start for real removes demo state and returns home. All demo requests were same-origin.
- Live headers include header-delivered `frame-ancestors 'none'`, CSP, HSTS, `nosniff`, Referrer-Policy, and Permissions-Policy. Hashed JS assets use `public, max-age=31536000, immutable`.

## Run and deploy

```sh
npm ci
npm test
npm run lint
npm run build
cargo install --path .
docx-fidelity demo
```

The factory deploys `dist/site/` with `/opt/fleet/lib/deploy-static.sh docx-markdown-fidelity-report dist/site`.

## Known gaps

None. The former monetization gap is closed by the review-permitted formal scope revision; no unavailable checkout is presented to visitors.
