# Independent verification — FAIL

**Candidate:** `4471c0df14102354e2d08b3bd43778eb4854f341` (`4471c0d`)

**Live URL:** https://docx-markdown-fidelity-report.sociobot.in

**Verified:** 2026-08-28 UTC, from a clean checkout after `npm ci`.

## Release decision

**FAIL.** The advertised paid team license cannot be bought: the live checkout endpoint returns HTTP 404. The page presents a $29 one-time license as an available feature, so this is a release-blocking production failure even though the free CLI and its local checks work.

## Release-blocking finding

| Severity | Finding | Fresh evidence | Required resolution |
| --- | --- | --- | --- |
| P1 / release blocker | The live **Buy the team license** link is unusable. | `GET https://api.sociobot.in/api/v1/products/docx-markdown-fidelity-report/checkout` returned **404** on 2026-08-28. The landing page links this exact URL and says the $29 license enables CI policy gates. | Register/configure the product in the Sociobot billing engine, set its return URL, and retest checkout plus a completed license-return/verification flow on the deployed site. |

## Additional finding

| Severity | Finding | Evidence | Required resolution |
| --- | --- | --- | --- |
| P2 | Hashed production assets are not long-cacheable. | `GET /assets/index-CZofUygf.js` returns `Cache-Control: public, must-revalidate, max-age=30`; the same short policy is served for the static site. The hashed JS is only 6.12 KB gzip, but the response does not meet the required immutable long-lived static-asset caching policy. | Configure the static host to serve hashed assets with a long `max-age` and `immutable`; retain short revalidation only for HTML/service-worker entry points. |

The paid-policy claim test passes only against a local fixture verifier and asserts the checkout link string; it does not prove that the deployed checkout exists. That gap allowed the P1 production failure through the claim suite.

## Required claim gate

`.factory/claims.json` exists and contains six claims. After clean-installing dependencies with `npm ci`, I ran every exact command listed in it. All passed:

| Claim | Exact command | Result |
| --- | --- | --- |
| demo-conversion | `npm test -- --grep @claim:demo-conversion` | Pass |
| local-processing | `npm test -- --grep @claim:local-processing` | Pass |
| risk-ledger | `npm test -- --grep @claim:risk-ledger` | Pass |
| batch-conversion | `npm test -- --grep @claim:batch-conversion` | Pass |
| paid-policy | `npm test -- --grep @claim:paid-policy` | Pass (fixture only; see blocker) |
| safe-input | `npm test -- --grep @claim:safe-input` | Pass |

The initial claim invocation before dependency installation stopped at `vite: not found`, as expected in an uninstalled clean checkout. The required rerun after `npm ci` passed.

## First read and live product QA

Cold, desktop live-page read: “This converts DOCX to Markdown and maps every place needing a human review. It is for teams moving Word-heavy documentation. Click **Try it with sample data** to watch a complex document convert and inspect its fidelity ledger.” The first screen states the job, audience, and action in plain words, and the one-click demo is present.

- Keyboard: the primary demo link is reachable, Enter opens `/demo`, the new `<h1>` receives focus, and the focus indicator is a visible 3 px vermilion outline.
- Demo: `/demo` shows “Demo — sample data, nothing is saved,” **Reset demo**, and **Start for real**. Both controls work in a fresh context.
- 390 px mobile: landing and demo each had 0 px horizontal overflow; the headline and demo banner were visible.
- Reduced motion: contour animation and transition computed durations are `0.00001s` under `prefers-reduced-motion: reduce`.
- Axe on `/`, `/demo`, `/privacy`, `/terms`, and `/not-a-route`: no serious or critical violations. Each route returned 200, had one `<main>` and one `<h1>`, distinct title, and no page/console errors.
- Live deployment identity: downloaded `index.html`, hashed JS/CSS, hero image, and all three fonts were byte-for-byte equal to this candidate's `dist/site/` output.

## CLI and package QA

- `npm test`: **13 passed** (Rust tests, all six claim tests, axe route checks, and 390 px keyboard flow).
- `npm run build`: passed; produced `dist/bin/docx-fidelity` and `dist/site/`.
- `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, and explicit `tsc --noEmit` check: passed.
- `npm run package`: passed; package is 84.9 KiB / 25.9 KiB compressed.
- Clean package consumer: extracted `target/package/docx-markdown-fidelity-report-0.1.0.crate`, installed it into a new Cargo root, then ran `docx-fidelity 0.1.0` and `docx-fidelity --json demo`; the demo converted one bundled document with nine findings.
- Release binary normal case: converted `examples/field-guide.docx` to Markdown plus JSON/Markdown ledgers; counts were tables 1, comments 1, revisions 2, embedded objects 2, footnotes 1, styles 1, images 1.
- Boundary/recovery cases: directory batch converted two files; existing output, non-DOCX input, and empty input directory each returned documented exit code 2 with actionable errors. The safe-input claim also passed for a parent-directory ZIP member.

## Privacy, headers, performance, and allowance

- Direct live `/demo` request log contained only the product origin (HTML, local JS/CSS, and self-hosted fonts); no document data, analytics, or third-party request was observed. The cold landing page was also same-origin only.
- Live HTML headers include CSP, `X-Content-Type-Options: nosniff`, HSTS, `Referrer-Policy: strict-origin-when-cross-origin`, `Permissions-Policy`, and HTML `Cache-Control: public, must-revalidate, max-age=30`.
- Production bundle sizes: JS 16.72 KB / **6.12 KB gzip**; CSS 12.74 KB / **3.68 KB gzip**; both are within budget. Fonts and hero are self-hosted; no CDN was requested.
- The Sociobot verification endpoint accepted 30 invalid-token requests from one client and returned **429** on request 31 with `Retry-After: 2`. Observed allowance: 30 requests per current window. This rate-limit check passed.
- `GET .../verify?license=qa-invalid-token` returned 200; no real license token was used.

## Retest checklist

1. Register the paid product, then verify the live checkout returns a hosted checkout response (not 404).
2. Complete a test purchase or registered test license return and prove the deployed page stores and verifies it.
3. Deploy immutable cache headers for hashed assets and recheck their response headers.
4. Rerun this report's claim, CLI, and live-browser checks.
