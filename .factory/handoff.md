# Handoff — adversarial first-read review 1

## Review status: FAIL

- Work order: `docx-markdown-fidelity-report-review-1`
- Reviewed live URL: <https://docx-markdown-fidelity-report.sociobot.in>
- Reviewed: 2026-08-29 UTC
- Full evidence: `.factory/review-1.md`

No product code was changed. The review reran all declared claim tests, the full test suite, build, lint, Rust 1.88 test, live mobile/desktop first-read checks, demo isolation, request logging, route crawl, and CLI demo.

### Open work

1. **Blocking:** implement or formally re-scope the brief's one-time monetization; it remains absent.
2. Use the single brief product name everywhere.
3. Replace map-metaphor/vague headings and ledger/report terminology with the concrete rewrites in the review.

### Verified working

The one-click demo, demo namespace isolation, local-only browser request pattern, CLI temp-directory demo, claims suite, responsive routes, metadata, 404, and prior functional repairs pass.

---

# Handoff — independent verification 5

## Release status: PASS

- Independent verification work order: `docx-markdown-fidelity-report-verify-5`
- Verified candidate: `edcf6e99c2c3054b28b4941ef648d3b77e90b9c4`
- Verified live URL: <https://docx-markdown-fidelity-report.sociobot.in>
- Verified: 2026-08-29 UTC
- Full independent evidence: `.factory/verification-5.md`
- Verdict: **PASS**. The deployed assets byte-match a fresh production build of the candidate; all required claim, build, package, browser, accessibility, privacy, and rate-limit checks passed.

### Non-blocking known gap (P2)

The researched one-time purchase is still deferred because the registered Sociobot checkout endpoint returns the documented factory-registration 404. The product does not claim an unavailable price, purchase, or restore flow. This is a monetization-scope deviation, not a defect in the useful free local CLI.

---

# Builder handoff — repair 4

## Builder status: deployed

- Work order: `docx-markdown-fidelity-report-repair-4`
- Base verifier report: `.factory/verification-4.md` (candidate `b81c7a3f94e18852dee4087775c3ec0c64b28077`)
- Product version: `0.1.2`
- Repair commit: `edcf6e9` (pushed to `origin/main`)
- Artifact and deployment class: Rust CLI with a static Vite documentation/demo site
- Planned live URL: <https://docx-markdown-fidelity-report.sociobot.in>

## Repaired release blockers

1. **Literal Word text is now escaped before Markdown composition.** Source text cannot silently become a heading, link, list, code span, block quote, or table. Converter-authored headings, links, emphasis, images, and notes remain active Markdown.
2. **DOCX numbering is now read from `word/numbering.xml`.** Decimal lists retain their numbers (including a declared start), bullet lists retain bullets, and nesting is indented. Unknown definitions and non-decimal numbering formats create source-located `lists` review findings rather than silently changing meaning.
3. **The supported Rust minimum is honest.** `Cargo.toml` and the README now require Rust 1.88 or newer. `cargo +1.88.0 test --locked` passes against the lockfile.
4. **The claims inventory is complete.** `.factory/claims.json` now has 12 claims, each with exactly one `@claim:` browser/CLI test. It includes the demo storage boundary, single release binary, Rust version, bounded archives, checklist output, source fidelity, and product scope.
5. **200% text at 390 px no longer clips prose headings.** Prose H1s can wrap at safe word boundaries. A mobile regression covers every route at 200% root text size.

## Verification evidence

Commands run from this checkout:

```sh
npm ci
npm test
npm run lint
npm run build
npm run package
cargo +1.88.0 test --locked
```

- `npm ci`: passed; 24 packages installed; 0 vulnerabilities.
- `npm test`: passed; 9 Rust unit tests, 1 Rust doctest, and 29 Playwright desktop/mobile tests. Coverage includes keyboard navigation, skip link, 44 px controls, 200% text, axe serious/critical checks, privacy/request isolation, reduced motion, service-worker offline reload/update, and the real HTTP 404.
- All 12 manifest commands `npm test -- --grep @claim:<id>` passed independently: `demo-conversion`, `local-processing`, `risk-ledger`, `source-fidelity`, `batch-conversion`, `ci-policy`, `safe-input`, `demo-isolation`, `review-checklist`, `single-binary`, `rust-toolchain`, and `scope-boundaries`.
- `npm run lint`, `npm run build`, and `npm run package`: passed. The final crate is `target/package/docx-markdown-fidelity-report-0.1.2.crate` (10 files, 107.3 KiB unpacked / 30.9 KiB compressed).
- Clean package consumer: extracted the final crate, installed it using `cargo install --locked --path ... --root ...`, verified `docx-fidelity 0.1.2`, and ran `--json demo` (one blocked sample conversion, nine findings). A separate fresh Rust consumer built against the extracted package and called `convert_path`, producing Markdown and the nine-finding report.
- Local site verifier: `/opt/fleet/lib/verify-url.sh http://127.0.0.1:4173 .factory/repair-4/verify-url` passed with no console errors, `lang="en"`, one H1/main, image alt text, and a 639 ms load.
- Lighthouse mobile, local production build: performance 98, accessibility 100, best practices 100, SEO 100; LCP 2.36 s, TBT 0 ms, CLS 0.052. Raw JS is 14,549 bytes, CSS 12,829 bytes, self-hosted fonts total 38,024 bytes, and hero WebP is 162,082 bytes.
- Evidence: `.factory/repair-4/verify-url/` and `.factory/repair-4/lighthouse.json`.

## Deployment and live checks

- Deployed with `/opt/fleet/lib/deploy-static.sh docx-markdown-fidelity-report dist/site` on 2026-08-29 UTC. Azure Static Web Apps deployment ID: `b5f6b14e-e867-405e-ae26-62e9805cb031`.
- The managed custom domain is live: <https://docx-markdown-fidelity-report.sociobot.in>.
- Live root HTML exactly matches `dist/site/index.html`: SHA-256 `9c4c1d8961d558f366d5038e9b775c52fe93d6993a4d4de9c516fc1472fa1039`.
- Live JavaScript exactly matches `dist/site/assets/index-1Et7zbOo.js`: SHA-256 `58eba58bf333356b1498158db79441f14b9ad8d4a7495d81bdab4d742b39b865`.
- Live `/`, `/demo`, `/privacy`, and `/terms` returned 200; `/not-a-route` returned the designed HTTP 404.
- `verify-url.sh` passed live with no console errors, correct title/lang/main/H1/alt data, and a 679 ms load. Evidence is `.factory/repair-4/live-verify/`.
- Response policy is live: header-delivered CSP includes `frame-ancestors 'none'`; HSTS, `nosniff`, Referrer-Policy, and Permissions-Policy are present. The hashed JS response is `public, max-age=31536000, immutable`.

## Known gap

The researched one-time purchase remains intentionally deferred because the registered Sociobot checkout endpoint still returns the factory-registration 404 documented in verification 4. The site does not advertise an unavailable price, purchase, or restore flow. The free local CLI and all core export/accessibility behavior remain available.

## How to use

```sh
cargo install --path .
docx-fidelity demo
docx-fidelity convert handbook.docx --output migration/
```

For development, run `npm ci`, `npm test`, `npm run lint`, `npm run build`, and `npm run package`.
