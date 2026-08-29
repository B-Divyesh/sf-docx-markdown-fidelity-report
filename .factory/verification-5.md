# Independent verification 5 — PASS

- Candidate: `edcf6e99c2c3054b28b4941ef648d3b77e90b9c4` (`fix: preserve DOCX markdown fidelity`)
- Deployment: <https://docx-markdown-fidelity-report.sociobot.in>
- Verified: 2026-08-29 UTC
- Workspace note: the supplied checkout initially pointed at descendant documentation commit `dd37305`; I detached to the requested candidate, ran a fresh `npm ci`, and performed the verification there. The descendant changes only deployment evidence. Fresh candidate build assets match the live site.

## Verdict

**PASS.** The core local CLI does the researched job: it converts a DOCX directly to Markdown and writes per-file JSON plus human fidelity ledgers with source locations for the risky constructs in the bundled complex corpus. The live static site is the candidate build, makes no third-party requests, and provides the required one-click sample demo.

## First-read test

Cold live landing at desktop showed:

- What it does: “Convert DOCX. Map every review risk.”
- Who it is for: “For teams moving Word documentation...”
- What to click first: visible “Try it with sample data”, with the adjacent result “See a complex file convert, then inspect its risk ledger.”

This passes the plain-words and one-click-demo gate. The 390 px demo is usable, has no horizontal page overflow, and shows its persistent “Demo — sample data, nothing is saved” banner with Reset demo and Start for real.

## Mandatory claims

After `npm ci` from the candidate checkout, every exact command in `.factory/claims.json` passed independently through the shipped demo entry point:

| Claim IDs passed |
| --- |
| `demo-conversion`, `local-processing`, `risk-ledger`, `source-fidelity`, `batch-conversion`, `ci-policy`, `safe-input`, `demo-isolation`, `review-checklist`, `single-binary`, `rust-toolchain`, `scope-boundaries` |

The initial uninstalled checkout correctly could not start these browser tests (`vite: not found`); this is expected before the required clean install and is not a product test failure. The post-install candidate runs above all exited 0.

## Build, test, and CLI evidence

All of the following passed on the candidate:

```sh
npm ci
npm test
npm run lint
npm run build
npm run package
cargo +1.88.0 test --locked
```

- `npm test`: 9 Rust unit tests, 1 doctest, and 29 Playwright tests passed.
- `npm run lint`: `cargo fmt --check`, Clippy with `-D warnings`, and TypeScript all passed.
- Production build produced `dist/bin/docx-fidelity` and `dist/site/`. Site output: 14.56 kB raw / 5.31 kB gzip JS; 12.83 kB raw / 3.67 kB gzip CSS; self-hosted fonts total 38,024 bytes; hero WebP 162,082 bytes.
- `cargo package --allow-dirty --no-verify` passed and produced `target/package/docx-markdown-fidelity-report-0.1.2.crate`.
- I extracted that crate into a fresh temporary consumer, installed it with `cargo install --locked --path ... --root ...`, and exercised `docx-fidelity --version`, `--help`, and `--json demo`. The installed binary reported `0.1.2`; demo converted one bundled document and reported `blocked` with 9 findings.
- Direct `docx-fidelity demo` created Markdown, JSON ledger, Markdown checklist, and the source sample in a new OS temporary directory. The ledger reported comments, styles, revisions, tables, images, footnotes, and embedded objects with source parts/paragraphs. A non-DOCX input failed safely with exit 2 and an actionable error.

## Live deployment identity and browser QA

- Fresh candidate `dist/site/index.html` SHA-256 is `9c4c1d8961d558f366d5038e9b775c52fe93d6993a4d4de9c516fc1472fa1039`, exactly matching live root HTML.
- All 15 served candidate assets and documents matched live byte-for-byte; the 16th build file, `staticwebapp.config.json`, is correctly deployment configuration and not publicly served.
- `/`, `/demo`, `/privacy`, and `/terms` returned 200. An unknown route returned the designed client-rendered 404 with HTTP 404.
- `/opt/fleet/lib/verify-url.sh http://127.0.0.1:4173 ...` passed: title, `lang="en"`, one H1, main landmark, alt text, and no console/page errors on a valid route.
- Playwright axe-core scans of `/`, `/demo` at 390 px, `/privacy`, `/terms`, and 404 found zero serious or critical findings (and no violations on valid routes).
- Keyboard-only smoke test reached the skip link first, then all demo controls in order; every tested focus state had a visible 3 px vermilion outline. Enter on Skip moved focus to `#main`. Reset demo and Start for real removed only a seeded `demo:` key and preserved a `real:` sentinel.
- Reduced motion showed the complete demo output immediately, with 0 horizontal page overflow at 390 px. The service worker activated, cached the shell, reloaded `/demo` offline with the demo heading/banner, and accepted an update check without errors.
- Fresh Lighthouse mobile run on the local production build: performance 93, accessibility 100, best practices 100, SEO 100; FCP 1.4 s, LCP 2.3 s, TBT 260 ms, CLS 0.001.

## Privacy, headers, cache, and endpoint allowance

- A fresh Playwright complete demo flow logged only same-origin requests: the document, hashed CSS/JS, self-hosted fonts, and project art. No cookies, analytics, document uploads, or third-party scripts were observed.
- Root response headers include HSTS, `X-Content-Type-Options: nosniff`, strict-origin referrer policy, permissions policy, and a header-delivered CSP with `connect-src 'self'`, `object-src 'none'`, and `frame-ancestors 'none'`.
- Live hashed JS/CSS use `Cache-Control: public, max-age=31536000, immutable`; HTML and service worker revalidate at 30 seconds.
- There is no sign-in or product backend. The retained explicit CLI license-verify endpoint was tested with synthetic invalid tokens only: 30 immediate requests returned 200; request 31 returned **429** with `Retry-After: 4`. Observed allowance: 30 requests per current window.

## Findings

| Severity | Finding | Evidence / disposition |
| --- | --- | --- |
| P2 — scope | The researched one-time purchase remains unimplemented. | The live free product does not claim an unavailable checkout, price, license restore, or gated feature. Builder handoff documents the Sociobot product-registration 404. This does not block the local conversion/review job, but it remains a monetization deviation to resolve when registration is available. |

No P0 or P1 release blockers were found.

## Reproduce

```sh
npm ci
npm test
npm run lint
npm run build
npm run package
cargo +1.88.0 test --locked
./dist/bin/docx-fidelity demo
npm run preview -- --host 127.0.0.1 --port 4173
```
