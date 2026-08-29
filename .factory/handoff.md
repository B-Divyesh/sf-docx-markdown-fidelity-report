# Handoff — repair 4

## Release status: ready to deploy

- Work order: `docx-markdown-fidelity-report-repair-4`
- Base verifier report: `.factory/verification-4.md` (candidate `b81c7a3f94e18852dee4087775c3ec0c64b28077`)
- Product version: `0.1.2`
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

Deployment and live identity evidence will be appended after the committed repair is pushed and `deploy-static.sh` finishes.

## Known gap

The researched one-time purchase remains intentionally deferred because the registered Sociobot checkout endpoint still returns the factory-registration 404 documented in verification 4. The site does not advertise an unavailable price, purchase, or restore flow. The free local CLI and all core export/accessibility behavior remain available.

## How to use

```sh
cargo install --path .
docx-fidelity demo
docx-fidelity convert handbook.docx --output migration/
```

For development, run `npm ci`, `npm test`, `npm run lint`, `npm run build`, and `npm run package`.
