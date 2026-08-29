# Independent verification 2 — FAIL

**Candidate:** `5dd6ca2fc371e71be22c38a1092cec3a42a633a5` (`5dd6ca2`)

**Live URL:** https://docx-markdown-fidelity-report.sociobot.in

**Verified:** 2026-08-29 UTC from a clean checkout.

## Release decision

**FAIL.** The previous checkout and cache defects are repaired, but the live footer contains a broken required external link. It sends visitors to `https://www.sociobot.in`; that hostname's TLS certificate does not cover `www.sociobot.in`, so browsers reject the connection. The site-structure contract requires every link to work.

## Defects

| Severity | Finding | Fresh evidence | Required resolution |
| --- | --- | --- | --- |
| P1 / release blocker | **Built by Param Factory** footer link is unusable. | The candidate and live site link to `https://www.sociobot.in`. Chromium/Node `fetch` fails, and `curl` returns `SSL: no alternative certificate subject name matches target host name` (HTTP 000). `https://sociobot.in/` succeeds with HTTP 200 and a valid certificate. | Change the footer to a hostname covered by its certificate (currently `https://sociobot.in/`), deploy, then crawl all live links again. |

No other release-blocking defect was found.

## Required claims gate

`.factory/claims.json` exists and has six claims. After clean `npm ci`, I ran every exact command it declares, each through the product's demo fixtures. All passed (one Playwright test per command):

| Claim | Command | Result |
| --- | --- | --- |
| demo-conversion | `npm test -- --grep @claim:demo-conversion` | Pass |
| local-processing | `npm test -- --grep @claim:local-processing` | Pass |
| risk-ledger | `npm test -- --grep @claim:risk-ledger` | Pass |
| batch-conversion | `npm test -- --grep @claim:batch-conversion` | Pass |
| ci-policy | `npm test -- --grep @claim:ci-policy` | Pass |
| safe-input | `npm test -- --grep @claim:safe-input` | Pass |

The rerun log completed with status 0 at `/tmp/docx-claims-5dd6ca2-rerun.log` during verification.

## Cold first read

On a fresh desktop context the first screen said: **“Convert DOCX. Map every review risk.”** It clearly says it is for “teams moving Word documentation,” says that it shows where Markdown needs a human check, and makes **Try it with sample data** the first primary action. Clicking it opens `/demo`, immediately shows the sample fidelity ledger, and displays the persistent **Demo — sample data, nothing is saved** banner with **Reset demo** and **Start for real**. This gate passes.

## Local quality gates and CLI consumer

- `npm ci`: passed; npm reported 0 vulnerabilities.
- `npm test`: passed, **16 tests** (Rust tests/doctest, claims, live-site-shaped browser checks, axe, mobile keyboard, and service worker offline reload).
- `npm run lint`: passed (`cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, TypeScript typecheck).
- `npm run build`: passed; generated `dist/bin/docx-fidelity` and `dist/site/`.
- `npm run package`: passed; `cargo package` produced `docx-markdown-fidelity-report-0.1.0.crate` (83.7 KiB, 25.6 KiB compressed).
- Clean consumer: extracted that crate into `/tmp/docx-consumer-s14lXE`, `cargo install --path` installed `docx-fidelity 0.1.0`, and `--help`/`--json demo` worked. The demo converted one document with nine findings. A two-file batch produced two reports, each with all required categories: comments, embedded objects, footnotes, images, revisions, styles, and tables.
- Recovery and invalid input: a duplicate output returned exit **2** with an overwrite instruction; `--overwrite` returned **0**; a `.txt` input returned exit **2** with an actionable message. The unsafe-archive claim independently passed with exit 2 and no escaped file.

## Live product QA

- **Deployment identity:** all served candidate artifacts matched byte-for-byte: `index.html`, hashed JS/CSS, three fonts, hero/social images, favicon, `robots.txt`, `sitemap.xml`, and `sw.js`. The live JS SHA-256 is `26ecfe970f60d64de9a7c6f5111af18b2138944c499ade7110684eafa85f5ee8`; CSS is `8f73fd99ed10dfbf13d4304d66fb7a704496567487ba3ca1e5e060f11644da63`.
- **Routes and accessibility:** `/`, `/demo`, `/privacy`, `/terms`, and an unknown route all returned 200, each with one `<main>`, one `<h1>`, a route-specific title, no page/console errors, and no axe serious or critical findings. The same mobile audit at 390 px found no serious/critical issue and no horizontal overflow.
- **Keyboard and motion:** Tab exposes the skip link; Enter moves focus to `#main`. The demo action is operable with Enter and route change focuses the new h1. Its visible focus style is a 3 px vermilion outline. Under reduced motion all animations are disabled and transitions reduce to `0.00001s`.
- **Privacy:** cold landing and `/demo` request logs contained only the product origin (document, local JS/CSS, self-hosted fonts, and local art), with no analytics, third-party scripts, or document upload. The conversion claim additionally exercised the CLI through unreachable HTTP proxies successfully.
- **Headers/caching:** HTML and `sw.js` use short revalidation (`public, must-revalidate, max-age=30`); hashed JS/CSS return `public, max-age=31536000, immutable`. Live headers include CSP with `frame-ancestors 'none'`, HSTS, `X-Content-Type-Options: nosniff`, `Referrer-Policy`, and `Permissions-Policy`.
- **PWA:** live `sw.js` became the active controller with cache `docx-fidelity-shell-index-CycaVJvZ.js`; after network was disabled, a reload still rendered the landing h1 with no console errors. Candidate tests also confirm the build-specific precache has no unresolved placeholders.
- **Performance:** initial JS is 5.28 KB gzip and CSS 3.62 KB gzip; self-hosted fonts total 38 KB; the hero is 162 KB. Lighthouse 13.4.1 mobile reported performance 93, accessibility 100, best practices 100, SEO 100, LCP 2.03 s, CLS 0.0011. (Lighthouse emitted a post-audit browser-tab crash warning after producing the complete report; browser QA itself had no errors.)
- **Server allowance:** there is no product backend or sign-in. The retained explicit CLI `license verify` endpoint at `api.sociobot.in` was tested only with invalid synthetic tokens. Thirty rapid requests returned 200; request 31 returned **429** with `Retry-After: 1`, so the observed allowance is 30 requests per current window. No document data was sent.

## Retest checklist

1. Repair the footer's external hostname or certificate and deploy the correction.
2. Re-crawl the live site, including the footer external link, using normal TLS validation.
3. Retain the above passing claims, CLI, privacy, PWA, and accessibility checks in the release retest.
