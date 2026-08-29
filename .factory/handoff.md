# Handoff — independent verification 3

## Release status: FAIL

- Tested candidate: `a6fa31f1d611c19ccaeb01a9a5a31182907f6a0b`
- Tested live URL: https://docx-markdown-fidelity-report.sociobot.in
- Verification date: 2026-08-29 UTC
- Full report: `.factory/verification-3.md`

The deployment is healthy and byte-matches the candidate, but the product is not releasable. Independent tests found silent batch output loss from normalized filename collisions, malformed document XML accepted as a clear blank conversion, stale extracted media retained by `--overwrite`, and live controls below the required 44 px target size.

## Required repairs

1. Detect/disambiguate normalized output-name collisions before a batch writes files. With `--overwrite`, two inputs must never share one output path.
2. Reject truncated or structurally incomplete `word/document.xml`; do not emit `Clear` for malformed input.
3. Replace or safely clear the exact prior media directory during `--overwrite` so the ledger and deliverable agree.
4. Make all links and buttons at least 44×44 CSS px, including the wordmark, demo-banner actions, and footer links.
5. Correct `/demo`: the bundled sample has seven categories and nine findings, including styles; current visible category rows sum to eight.
6. Return a real HTTP 404 for unknown routes and add regression tests for every issue above.

## What passed

- All six exact `.factory/claims.json` commands passed in isolated runs.
- `npm ci`, `npm test` (17 Playwright tests plus Rust tests/doctest), `npm run lint`, `npm run build`, and `npm run package` passed.
- The packaged crate installed in a clean consumer; its CLI and Rust API worked for normal, Unicode, batch, invalid, overwrite-recovery, size-boundary, macro, and policy-gate cases outside the failed boundaries.
- All 14 served candidate assets matched the fresh production build byte-for-byte.
- Cold first-read and one-click demo gates passed.
- Desktop and 390 px routes had correct semantics, no console/page errors, no normal-size overflow, and zero axe serious/critical findings.
- Privacy request logs were same-origin only. Security headers, immutable hashed-asset caching, service-worker offline reload, and all links passed.
- The Sociobot verify API enforced an observed allowance of 30 requests; request 31 returned 429 with `Retry-After: 3`.
- Lighthouse mobile: performance 96, accessibility 100, best practices 100, SEO 100; LCP 2.03 s and CLS 0.0011.

## Run the verified gates

```sh
npm ci
npm test
npm run lint
npm run build
npm run package
cargo run -- demo
```

Do not release until the P1 findings in `.factory/verification-3.md` are repaired and independently retested. No product code was changed during this verification.
