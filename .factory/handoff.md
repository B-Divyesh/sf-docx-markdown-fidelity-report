# Handoff

## Repair status — ready to deploy (2026-08-29)

This repair resolves both findings in the independent report for candidate `4471c0df14102354e2d08b3bd43778eb4854f341`:

- The unavailable $29 checkout and browser license flow were removed. CI policy gates are now included in the local CLI, require no account or network connection, and retain exit code `3` when a selected level is met. This is the closest honest scope while the external billing product is not registered; no billing or infrastructure configuration was changed from this repository.
- `staticwebapp.config.json` now gives `/assets/*` `Cache-Control: public, max-age=31536000, immutable`. Hashed JS and CSS are the only files covered; HTML and `sw.js` remain short-revalidated by the platform so updates can arrive.
- `site/vite.config.ts` writes the current hashed JS/CSS names and a build-specific cache name into the service worker. Navigation requests are network-first with a cached fallback; the shell reloads after all network routes are blocked.

The original failure evidence remains in `.factory/verification.md` for traceability.

## Shipped

- Rust 0.1.0 single-binary CLI with `convert`, `demo`, `--json`, local `--fail-on warning|error` policy gates, batch directory input, safe output names, documented exit codes, and no interactive CI prompts.
- Direct DOCX ZIP/XML conversion for headings, paragraphs, lists, bold, italic, safe links, tables, footnotes, and extracted images.
- JSON and Markdown fidelity ledgers. Findings cover tables, comments, tracked revisions, embedded objects, footnotes/endnotes, non-default styles, every image outcome, unsafe links, equations, fields, text boxes, legacy drawings, and omitted package parts. Locations include the package part and available paragraph, table, row, and cell coordinates.
- Input defenses: archive entry, expanded-size, and entry-count bounds; unsafe archive paths rejected; macros and embedded files reported but never executed or extracted.
- Bundled `examples/field-guide.docx` and `docx-fidelity demo`, which work in a fresh temporary directory and produce nine realistic findings.
- Free single-file and batch conversion with local `--fail-on warning|error` policy gates. The site has no checkout, account, analytics, or third-party runtime dependency.
- Static Vite site in the topographic-cartography system with `/`, `/demo`, `/privacy`, `/terms`, and a designed 404. It includes keyboard routing, route announcements, a 390 px layout, reduced-motion behavior, local fonts, a service worker, metadata, social art, sitemap, robots, CSP, and security headers.
- Original hero art generated with the factory image deployment and optimized to a 160 KB WebP. Its full prompt and provenance are in `.factory/design.md` and `.factory/terrain-report-generation.json`.
- Product contracts: brief, design, demo, claims, copy audit, changelog, MIT license, third-party font notices, and README.

## Run and verify

```sh
npm ci
npm test
npm run lint
npm run build
npm run package
cargo run -- demo
```

Final local results on 2026-08-29:

- Clean `npm ci`: passed; `npm audit` reported 0 vulnerabilities.
- `npm test`: 16 passed. This includes Rust unit/doctests, six claim tests, legacy license-verify compatibility, the checkout/cache regression, axe checks on five routes, 390 px keyboard flow, and the service-worker no-network reload test.
- All six exact commands in `.factory/claims.json` passed from the clean install.
- `npm run lint`: passed (`cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, and TypeScript `tsc --noEmit`).
- `npm run build`: passed. Static deploy output is `dist/site/index.html`; the Linux CLI is `dist/bin/docx-fidelity` (3.1 MB).
- `npm run package`: passed. The crate is 69.3 KiB / 21.6 KiB compressed and was not published. A fresh extracted consumer installed it with `cargo install --path`, then passed `docx-fidelity 0.1.0` and `docx-fidelity --json demo` (one conversion, nine findings).
- Factory `verify-url.sh` against the built site: HTTP 200, 606 ms load, no console errors, one `<h1>`, one `<main>`, `lang=en`, all images have alt text, and all buttons have names.
- Deployed static site (`22510aed-3bb4-438e-a34c-9d96ad20fb3c`): live JS is byte-identical to the built `index-CycaVJvZ.js` and returns `Cache-Control: public, max-age=31536000, immutable`. Live mobile Axe had no serious or critical issues on `/`, `/demo`, `/privacy`, `/terms`, or `/not-a-route`; keyboard demo navigation and the 390 px overflow check passed with no console errors.
- The static-host configuration and built service worker were checked: immutable asset rule present, hashed JS/CSS precached, and no unresolved cache placeholders.
- Lighthouse 13.4.1 mobile on the live landing page: performance 98, accessibility 100, best practices 100, SEO 100; LCP 2.0 s, CLS 0.001.
- Initial assets: JavaScript 5.28 KB gzip, CSS 3.62 KB gzip, fonts 38 KB total, hero 160 KB.

Every command in `.factory/claims.json` supports `npm test -- --grep @claim:<id>` from a fresh checkout.

## Known boundaries

- This is a conversion and review tool, not a pixel-identical DOCX renderer. It does not run OCR, edit documents, render PDF, execute macros, or open embedded files.
- Complex page geometry, SmartArt, charts, equations, headers, footers, endnotes, text boxes, and fields become explicit review findings. Reviewers must replace or confirm them.
- The release binary in `dist/bin` targets this Linux worker. Build another Rust target before distributing to macOS or Windows.
- The researched brief still records one-time monetization. This repair deliberately does not advertise a paid tier until a future authorized billing registration and a complete purchase-flow retest exist.

## Next factory steps

1. Deploy `dist/site/` through the factory static pipeline.
2. Build signed binaries for supported operating systems and attach them to the release.
3. If monetization resumes, register the paid product outside this repository, restore only a fully tested purchase return/verification flow, and add a deployed checkout availability regression.
