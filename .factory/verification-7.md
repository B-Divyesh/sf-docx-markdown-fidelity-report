# Independent verification 7 — PASS

- Candidate: `3b1dcf92d8a47c2c77606a92082d013cd99124da`
- Production URL: <https://docx-markdown-fidelity-report.sociobot.in>
- Verified: 2026-08-29 UTC

## Verdict

**PASS.** The production deployment is the candidate build and the packaged Rust CLI completes the core job from the researched brief: it converts DOCX directly to Markdown and writes per-file JSON and human-review reports with source locations for risky Word constructs. The earlier alleged deployment-only failure was not reproducible.

## Mandatory gates

### Claims and cold first read

After `npm ci`, every exact command named by the 13 entries in `.factory/claims.json` was run from this clean checkout, using the shipped CLI/browser demo entry points. All passed:

`demo-conversion`, `local-processing`, `fidelity-report`, `source-fidelity`, `batch-conversion`, `ci-policy`, `safe-input`, `demo-isolation`, `review-checklist`, `single-binary`, `rust-toolchain`, `scope-boundaries`, and `mit-license`.

The manifest/tag audit also found exactly one `@claim:<id>` test for each of those 13 claims, with no missing, duplicate, or extra tags.

Cold-reading production before interacting: the headline says “Convert DOCX and list review issues.” It says it is for “teams moving Word documentation” and explains that it shows where Markdown needs human checking. The first action is **Try it with sample data**, immediately explained as converting the bundled field guide and opening its fidelity report. One click opens `/?demo=1`, showing the recorded sample conversion and persistent “Demo — sample data, nothing is saved to your files” banner with Reset demo and Start for real. The plain-words and demo gates pass.

## Local build and CLI exercise

All of the following passed:

```sh
npm ci
npm test                  # 9 Rust unit tests, 1 doctest, 31 Playwright tests
npm run lint              # rustfmt, clippy -D warnings, TypeScript
npm run build             # dist/bin/docx-fidelity and dist/site
npm run package           # docx-markdown-fidelity-report-0.1.4.crate
```

- `./dist/bin/docx-fidelity --json demo` created a fresh OS-temp sandbox and reported one `blocked` conversion with 9 findings. It wrote Markdown (486 B), JSON report (3.0 KB), and human checklist (1.8 KB). The report included comments, styles, revisions, tables, images, footnotes, and embedded objects; sample locations include `word/document.xml`, paragraph 2–4.
- A regular conversion left the input DOCX byte-identical (SHA-256 `614c1a…8697d6`) and wrote Markdown, JSON, checklist, and extracted `image-001.svg`.
- `--fail-on error` exited 3 after writing the report. A `.pdf` input exited 2 with the actionable error “input must be a .docx file”.
- The produced crate was extracted and installed into a clean temporary consumer root with `cargo install --locked --path … --root …`. Its installed `docx-fidelity 0.1.4` passed `--help` and `demo`; a missing input exited 2 with recovery guidance.

## Production QA

- Candidate identity: live `/`, `assets/index-BVu1iQhx.js`, and `assets/index-bxX-Os2N.css` are byte-for-byte equal to `dist/site`; all 15 publicly served built files compare equal. `staticwebapp.config.json` correctly returns 404 because it is deployment configuration, not a public asset.
- Routes `/`, `/?demo=1`, `/demo`, `/privacy`, and `/terms` return 200. An unknown route returns HTTP 404 and renders the designed recovery page. The external Param Factory link returns 200.
- `verify-url.sh` passed on production: title, `lang=en`, one H1, `<main>`, image alt text, labelled buttons, and no console/page errors. The standalone `@axe-core/cli` could not launch in this container because it cannot discover Chrome; the equivalent installed `@axe-core/playwright` scan was used instead.
- Axe Playwright scans on `/`, `/demo`, `/privacy`, and `/terms` at 390 px had zero violations, including zero serious/critical findings. Each page had `scrollWidth === clientWidth === 390`.
- Keyboard checks found the skip link, header navigation, demo action, copy action, and footer links reachable with a visible `rgb(184, 58, 45) solid 3px` focus ring. Demo navigation, Reset demo, and Start for real operate without errors. Reduced-motion rendering reduced the contour animation to 0.01 ms.
- Privacy: fresh landing, demo, privacy, and terms request logs contain only this product origin (HTML, local JS/CSS, self-hosted fonts, and local art). No cookie, analytics, upload, third-party script, or document-data request was observed. Demo state is `demo:sample`; Reset retains only that reset demo key and Start for real removes it.
- Response headers include header-delivered CSP with `connect-src 'self'`, `object-src 'none'`, and `frame-ancestors 'none'`; `X-Content-Type-Options: nosniff`; `Referrer-Policy: strict-origin-when-cross-origin`; HSTS; and a restrictive permissions policy. HTML and service worker use 30-second revalidation; hashed JS/CSS use `public, max-age=31536000, immutable`.
- Service worker activation, `registration.update()`, and an offline `/demo` reload passed; the offline page retained its demo H1 and banner.
- Fresh mobile Lighthouse 13 on `/?demo=1`: Performance 99, Accessibility 100, Best Practices 100, SEO 100; LCP 1204 ms, CLS 0.00065, TBT 133.5 ms. Production build sizes: JS 14.73 KB raw / 5.33 KB gzip; CSS 12.97 KB raw / 3.71 KB gzip.

## Endpoint allowance

The CLI's documented Sociobot license-verification endpoint was probed with 35 distinct synthetic invalid tokens from one client. Requests 1–30 returned 200. Requests 31–35 returned **429** with `Retry-After` of 3, 3, 3, 2, and 2 seconds respectively. Observed allowance: **30 requests per current window**. The product has no sign-in flow.

## Findings

| Severity | Finding | Evidence and disposition |
| --- | --- | --- |
| P0 | None | — |
| P1 | None | — |
| P2 | The brief names one-time monetization, but this shipped product is free/MIT and has no checkout or unlock. | The free CLI does not falsely advertise an unavailable paid feature; the researched core conversion/review job works. This remains an honest scope deviation to resolve only after a real Sociobot product registration can support a checkout. |

## Reproduce

```sh
npm ci
npm test
npm run lint
npm run build
npm run package
./dist/bin/docx-fidelity --json demo
```
