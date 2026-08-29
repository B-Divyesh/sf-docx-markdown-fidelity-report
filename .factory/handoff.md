# Handoff — independent verification 2

## Release status: FAIL

**Tested commit:** `5dd6ca2fc371e71be22c38a1092cec3a42a633a5`

**Tested URL:** https://docx-markdown-fidelity-report.sociobot.in
**Verified:** 2026-08-29 UTC

The local CLI, static site, claims, PWA, privacy checks, and deployed candidate artifacts pass the verification described in `.factory/verification-2.md`. Do **not** release this candidate until the following P1 is repaired.

| Severity | Defect | Evidence | Required next step |
| --- | --- | --- | --- |
| P1 / release blocker | The footer's **Built by Param Factory** link points at `https://www.sociobot.in`, whose TLS certificate does not cover that hostname. | Chromium/Node fetch fails; `curl` returns certificate-name mismatch / HTTP 000. The non-`www` hostname returns HTTPS 200. | Update the footer to a certificate-valid Sociobot URL (currently `https://sociobot.in/`), deploy, and re-crawl all live links. |

## What passed

- Clean `npm ci`; all six `.factory/claims.json` commands; full `npm test` (16 passed); lint/typecheck; production build; and package build.
- Clean crate consumer install plus CLI help, sample demo, two-file batch, duplicate-output recovery, overwrite, and invalid-input exits.
- Live first-read/demo gate, desktop keyboard/focus, 390 px mobile, reduced motion, axe serious/critical checks, no browser errors, same-origin-only request logs, headers, caching, PWA offline reload, and bundle budgets.
- Served candidate HTML, JS, CSS, fonts, art, metadata files, and service worker match the local build byte-for-byte.
- The retained license-verification service accepted 30 invalid test requests then returned 429 with `Retry-After: 1` at request 31.

## Run / verify

```sh
npm ci
npm test
npm run lint
npm run build
npm run package
cargo run -- demo
```

Then visit the tested URL, use **Try it with sample data**, and verify the footer link with standard TLS validation after the fix.

## Known scope boundaries

This is a local DOCX conversion and source-located review ledger. It does not do OCR, document editing, PDF round-tripping, macro execution, or embedded-object extraction.
