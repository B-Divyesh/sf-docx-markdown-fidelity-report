# Independent verification 3 — FAIL

- Candidate: `a6fa31f1d611c19ccaeb01a9a5a31182907f6a0b`
- Live URL: https://docx-markdown-fidelity-report.sociobot.in
- Verified: 2026-08-29 UTC from the clean candidate checkout
- Acceptance source: `.factory/brief.json`, the supplied factory work order, and attached product skills

## Release decision

**FAIL.** The deployed static files exactly match the candidate, and the declared test suite passes. Independent boundary testing nevertheless found three CLI correctness defects, including a batch path that silently reports two conversions while retaining only one result. The live UI also violates the attached 44 px touch-target baseline. These are product defects, not deployment-only failures.

## Release-blocking defects

| Severity | Finding | Fresh evidence | Contract impact |
| --- | --- | --- | --- |
| P1 | Safe output-name collisions cause partial or silent batch data loss. | Two valid inputs, `Plan Q1.docx` and `Plan-Q1.docx`, both map to `Plan-Q1.*`. Without `--overwrite`, the installed package exited 2 after writing only the first result. With `--overwrite`, it exited 0 and printed `converted: 2`, but both JSON entries named the same output paths and the sole surviving Markdown contained only `SECOND DOCUMENT`. | Falsifies the listed “One command converts every DOCX in a directory” claim for valid filenames and violates the brief's safe-filename constraint. Output names must be collision-proof before any file is written. |
| P1 | Malformed document XML is accepted as a clear conversion. | A valid ZIP containing unclosed `word/document.xml` (`<w:document><broken>`) exited 0. It wrote one-byte Markdown containing only a newline and a fidelity report with `status: clear`, zero findings, and every category count at zero. Non-ZIP input and a ZIP missing `word/document.xml` correctly exited 2. | An untrusted, lossy input is presented as safe to accept. This contradicts the success measure that lossy constructs be reported before output is accepted. Validate balanced XML/document completion and fail closed. |
| P1 | `--overwrite` retains stale extracted media from the prior document. | The sample first produced `guide.media/image-001.svg`. Replacing the same input with a valid image-free DOCX and running `--overwrite` returned 0 with `status: clear` and `images: 0`, while the old SVG remained in the output directory. | The deliverable and ledger disagree. Publishing the output directory can carry old document data. Replace the media directory atomically or clear only the exact prior media directory before extraction. |
| P1 / accessibility | Required touch targets are smaller than 44 px. | At 390 px, computed live rectangles measured the wordmark at 34 px high, footer links at 25 px high, and the persistent demo controls **Reset demo** and **Start for real** at 37 px high. Desktop measurements have the same height failures. | Violates the attached non-negotiable accessibility and design baseline even though axe does not flag these targets. |

## Other defects and gaps

| Severity | Finding | Evidence |
| --- | --- | --- |
| P2 | The one-click browser demo does not faithfully represent its bundled sample. | The real sample report has seven categories and nine findings, including one `styles` finding. `/demo` says “six risk areas,” omits styles, and its displayed category counts sum to eight while it says “9 findings.” |
| P2 | The designed 404 is not a real HTTP 404. | `GET /not-a-route` returns HTTP 200 and the SPA renders the not-found content. The supplied site-structure contract requires a real 404 response path. |
| P2 | Claim automation is narrower than claim text. | `@claim:safe-input` tests path traversal but does not assert the claim's second promise that embedded objects are never extracted. Independent QA verified the behavior with macro and embedded-object fixtures, but the required automated proof is missing. `@claim:batch-conversion` also lacks a normalization-collision case and therefore passes while the broad claim is false. |
| P2 / scope | The researched one-time monetization is absent. | The site has no price, checkout, restore-license flow, or paid feature. The binary retains `license verify`, but a valid license does not unlock anything. The handoff must either document an honest scope deviation or implement the Sociobot unlock after product registration. |
| P3 | Release identity is inconsistent. | `CHANGELOG.md` announces 0.1.1, while `Cargo.toml`, the packaged crate, CLI `--version`, and live footer remain 0.1.0. |

## Required claims gate

`.factory/claims.json` exists. After `npm ci`, each exact command was run alone from the candidate checkout so Cargo, Vite, and Playwright did not share ports or build directories. Every declared command exited 0:

| Claim | Exact command | Result | Observable evidence |
| --- | --- | --- | --- |
| `demo-conversion` | `npm test -- --grep @claim:demo-conversion` | Pass; 1 Playwright test | Fresh CLI demo wrote Markdown, JSON report, Markdown checklist, and `image-001.svg`; nine findings. |
| `local-processing` | `npm test -- --grep @claim:local-processing` | Pass; 1 Playwright test | `/demo` requests were same-origin and CLI conversion succeeded with unreachable HTTP proxies. |
| `risk-ledger` | `npm test -- --grep @claim:risk-ledger` | Pass; 1 Playwright test | Sample report contained all seven promised categories and source parts/paragraph locations. |
| `batch-conversion` | `npm test -- --grep @claim:batch-conversion` | Pass; 1 Playwright test | Two non-colliding fixtures wrote two results. Independent colliding-filename evidence above disproves the unrestricted claim. |
| `ci-policy` | `npm test -- --grep @claim:ci-policy` | Pass; 1 Playwright test | `--fail-on error` returned exit 3 with network proxies unreachable. |
| `safe-input` | `npm test -- --grep @claim:safe-input` | Pass; 1 Playwright test | Parent-directory ZIP entry returned exit 2 and did not escape. The embedded-object half is not asserted by this test. |

Because the exact tests pass but a valid boundary case falsifies `batch-conversion`, the scripts do not rescue the release verdict.

## Cold first-read and demo gate

This gate passes. A fresh 1440×900 browser context showed:

- Job: **“Convert DOCX. Map every review risk.”**
- Audience/outcome: **“For teams moving Word documentation, it shows where Markdown needs a human check.”**
- First action: **“Try it with sample data”**, beside text explaining that it converts a complex file and shows its ledger.

The action was above the fold and opened `/demo` in one click. The next screen immediately showed the recorded conversion and the persistent **Demo — sample data, nothing is saved** banner with **Reset demo** and **Start for real**. A fresh demo had no localStorage, sessionStorage, or IndexedDB entries. Reset and leaving removed an injected `demo:` key.

## Clean checkout gates

- Clean identity before edits: `git rev-parse HEAD` returned the candidate SHA.
- `npm ci`: passed; 24 packages installed; 0 vulnerabilities.
- `npm test`: passed; Rust unit tests, README doctest, site build, and all 17 Playwright tests.
- `npm run lint`: passed; rustfmt, Clippy with warnings denied, and TypeScript typecheck.
- `npm run build`: passed; produced `dist/bin/docx-fidelity` and `dist/site`.
- `npm run package`: passed; 10-file crate, 83.7 KiB unpacked / 25.6 KiB compressed.

## Packaged consumer and CLI QA

The produced `.crate` was extracted and installed with `cargo install --path ... --root ... --locked` into `/tmp/docx-fidelity-consumer.eqJkTV`. The installed executable reported `docx-fidelity 0.1.0`; `--help` described commands, JSON output, and exit behavior. Its bundled `--json demo` converted one document with nine findings.

Passing independent cases:

- A filename containing spaces, parentheses, and an emoji converted with a safe output stem.
- A mixed-case two-file batch converted both `.docx` and `.DOCX` inputs.
- Invalid extension and empty directory returned exit 2 with actionable messages.
- Existing output returned exit 2; `--overwrite` recovered with exit 0.
- `--fail-on error` produced reports and returned exit 3.
- A 32 MiB archive part was accepted; a 32 MiB + 1 byte part returned exit 2 with the documented limit.
- A synthetic macro plus embedded file produced two blocked findings and neither binary was extracted.
- A separate Rust consumer compiled against the packaged public library and called `convert_path`, producing nine findings.

Failing independent cases are listed in the release-blocker table.

## Live deployment identity

The deployment matches this candidate. Fourteen served files matched the fresh `dist/site` build byte-for-byte: HTML, JS, source map, CSS, three fonts, two images, favicon, Apple icon, `robots.txt`, `sitemap.xml`, and `sw.js`.

- HTML SHA-256: `e1595ec528f129b6b5075b49d1a55acfdd7678f6834dd921fa5d60ea395ff24a`
- JS SHA-256: `c87315d770e22235e48fe7a7f2ad2ac129fb917c6fa71f70744aa9dcf5997b32`
- CSS SHA-256: `8f73fd99ed10dfbf13d4304d66fb7a704496567487ba3ca1e5e060f11644da63`
- Service-worker SHA-256: `93e58c8f3cf5f4ef7ebc722ef6f3ceef386cadbe22cdcd3a1891646caf56cbd4`

## Browser, accessibility, and privacy

Fresh Chromium contexts covered `/`, `/demo`, `/privacy`, `/terms`, and an unknown path at 1440×900 and 390×844.

- Every rendered page had `lang="en"`, one `<main>`, one `<h1>`, ordered headings, route-specific title, image alt text, and no horizontal overflow at normal text size.
- Axe 4.10.2 found zero serious or critical violations on every route and viewport; in fact it reported no violations in this matrix.
- No console errors or page errors occurred.
- Keyboard: first Tab exposed the skip link; its 3 px vermilion ring has 5.01:1 contrast against paper. Enter focused `main`. Enter on the demo action navigated and focused the new `<h1>`.
- Reduced motion matched and forced computed animation/transition durations to `0.00001s`; demo output appeared without timed playback.
- Same-origin-only request logging covered cold landing and the complete demo flow. There were no analytics, third-party scripts, document uploads, or browser storage writes.
- The factory `/opt/fleet/lib/verify-url.sh` passed: HTTP 200, title/lang/main/h1/alt checks, 951 ms network-idle load, and zero console errors.
- Every distinct HTTP link resolved with normal TLS validation, including `https://sociobot.in/`.

The touch-target failures are not visible in axe's result and remain release-blocking under the supplied baseline.

## Headers, caching, offline, and rate limiting

- HTML and `sw.js`: `Cache-Control: public, must-revalidate, max-age=30`.
- Hashed JS/CSS: `Cache-Control: public, max-age=31536000, immutable`.
- Live responses include CSP with header-delivered `frame-ancestors 'none'`, HSTS, `nosniff`, `Referrer-Policy`, and `Permissions-Policy`.
- The service worker activated as `/sw.js` with cache `docx-fidelity-shell-index-C1ZRaeIZ.js`. After the network was disabled, `/demo` reloaded with its heading and banner and no errors.
- There is no product backend or sign-in. The retained Sociobot product verify endpoint allowed 30 rapid invalid-token requests; request 31 returned HTTP 429 with `Retry-After: 3`.

## Performance and budgets

Fresh production assets are comfortably within the supplied budgets:

- JS: 14,459 bytes raw / 5,281 bytes gzip.
- CSS: 12,404 bytes raw / 3,641 bytes gzip.
- Self-hosted fonts: 38,024 bytes total.
- Hero WebP: 162,082 bytes.
- Release binary: 3,152,480 bytes.

Lighthouse 13.4.1 mobile completed with performance 96, accessibility 100, best practices 100, SEO 100; FCP 1.05 s, LCP 2.03 s, CLS 0.0011, total blocking time 207 ms, and speed index 1.10 s.

## Product judgment

The deterministic conversion and risk-ledger job does not need an AI feature. The obvious valuable extension is already implied by the core product: collision-safe, atomic batch output with strict malformed-package validation. Those correctness repairs take precedence over adding intelligence or hosted processing.

## Required retest

1. Preflight all normalized output names and reject or disambiguate collisions; never report two conversions with one surviving output.
2. Reject truncated/unbalanced XML and incomplete Word document structure before writing any output.
3. Make `--overwrite` replace the prior media directory without leaving stale files, preferably through an atomic staging directory.
4. Raise every interactive target to at least 44×44 CSS px and retest desktop/mobile keyboard and touch.
5. Make the browser demo's categories and counts match the bundled CLI output; add regression coverage.
6. Return an actual 404 response for unknown routes and broaden claim tests to cover the failed boundaries.

