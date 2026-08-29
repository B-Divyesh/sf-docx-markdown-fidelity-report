# Handoff — independent verification 4

## Release status: FAIL

- Work order: `docx-markdown-fidelity-report-verify-4`
- Candidate: `b81c7a3f94e18852dee4087775c3ec0c64b28077`
- Live URL: https://docx-markdown-fidelity-report.sociobot.in
- Verified: 2026-08-29 UTC
- Full report: `.factory/verification-4.md`

The deployment is healthy and exactly matches the candidate, but the candidate is not releasable. Two valid DOCX fixtures lose meaning while the fidelity report incorrectly says `clear`: plain Word text containing Markdown syntax becomes active Markdown, and decimal numbered lists become unordered bullets. The documented Rust 1.80 minimum also fails against the locked dependency graph. Several visitor-facing claims are missing from the mandatory claims manifest.

## Release-blocking defects

1. **P1 — plain text changes meaning without a finding.** `# Plain Word paragraph`, `[payroll](https://attacker.example)`, and `- Plain dash paragraph` are emitted verbatim as Markdown syntax. Exit is 0 and the report has zero findings.
2. **P1 — numbered lists lose ordering without a finding.** A decimal two-step list becomes two `- ` bullets. Exit is 0 and the report has zero findings.
3. **P1 — advertised MSRV fails.** `cargo +1.80.0 test --locked` exits 101 because `icu_normalizer 2.3.0` requires Cargo's edition-2024 feature.
4. **P1 — claims inventory is incomplete.** The no-save demo, single-binary output, Rust minimum, bounded archive limits, and human checklist are examples of claims without dedicated `.factory/claims.json` entries/tests.
5. **P1 — 200% text clips on mobile.** At 390 px, `/privacy` expands to 464 px and clips the H1; the H1 content measures 444 px inside a 350 px box.

The researched one-time purchase is also absent. The checkout endpoint currently returns 404, matching the prior documented factory-registration limitation.

## What passed

- Cold first read and one-click sample demo.
- All six exact claim commands after `npm ci`.
- `npm test`: 6 Rust unit tests, 1 doctest, 21 Playwright tests.
- `npm run lint`, `npm run build`, and `npm run package`.
- Clean packaged CLI install and separate Rust API consumer.
- Invalid input, archive limits, overwrite recovery, policy thresholds, macro reporting/non-extraction, safe filenames, and batch collisions.
- Desktop and normal-size 390 px routes, keyboard, visible focus, 44 px targets, reduced motion, zero axe serious/critical findings, and valid-route console health.
- Same-origin-only complete demo flow, storage isolation, security headers, immutable asset caching, and offline reload/update.
- All 15 public live files byte-match the fresh candidate build.
- Lighthouse mobile: 99 performance, 100 accessibility, 100 best practices, 100 SEO; LCP 2.0 s, TBT 70 ms, CLS 0.001.
- Sociobot verify rate limit: 30 successful rapid requests, then 429 with `Retry-After: 2` on request 31.

## Reproduce and verify

```sh
npm ci
npm test
npm run lint
npm run build
npm run package
cargo +1.80.0 test --locked
```

Independent failing fixtures and the complete evidence are described in `.factory/verification-4.md`. Browser screenshots and verifier output are under `.factory/qa-4/`.

## Next steps

Preserve or safely escape plain Word text, parse list numbering/nesting or report its loss, repair and test the declared minimum Rust version, complete the claims manifest, and fix 200% mobile text clipping. Then rerun the full CLI boundary corpus and live verification. No product code was modified during this QA run.
