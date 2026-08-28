# Handoff

## Shipped

- Rust 0.1.0 single-binary CLI with `convert`, `demo`, `license verify`, `--json`, batch directory input, safe output names, documented exit codes, and no interactive CI prompts.
- Direct DOCX ZIP/XML conversion for headings, paragraphs, lists, bold, italic, safe links, tables, footnotes, and extracted images.
- JSON and Markdown fidelity ledgers. Findings cover tables, comments, tracked revisions, embedded objects, footnotes/endnotes, non-default styles, every image outcome, unsafe links, equations, fields, text boxes, legacy drawings, and omitted package parts. Locations include the package part and available paragraph, table, row, and cell coordinates.
- Input defenses: archive entry, expanded-size, and entry-count bounds; unsafe archive paths rejected; macros and embedded files reported but never executed or extracted.
- Bundled `examples/field-guide.docx` and `docx-fidelity demo`, which work in a fresh temporary directory and produce nine realistic findings.
- Free single-file and batch conversion. The $29 one-time team license adds `--fail-on warning|error`; the CLI verifies tokens through Sociobot. The site handles checkout returns, daily browser-side verification caching, license restore, invalid/revoked states, and free-mode fallback.
- Static Vite site in the topographic-cartography system with `/`, `/demo`, `/privacy`, `/terms`, and a designed 404. It includes keyboard routing, route announcements, a 390 px layout, reduced-motion behavior, local fonts, a service worker, metadata, social art, sitemap, robots, CSP, and security headers.
- Original hero art generated with the factory image deployment and optimized to a 160 KB WebP. Its full prompt and provenance are in `.factory/design.md` and `.factory/terrain-report-generation.json`.
- Product contracts: brief, design, demo, claims, copy audit, changelog, MIT license, third-party font notices, and README.

## Run and verify

```sh
npm install
npm test
npm run build
npm run package
cargo run -- demo
```

Final local results on 2026-08-28:

- `npm test`: 13 passed. This includes Rust tests, all six claim tests, axe checks on five routes, and the 390 px keyboard path.
- `cargo clippy --all-targets -- -D warnings`: passed.
- `npm audit`: 0 vulnerabilities.
- `npm run build`: passed. Static deploy output is `dist/site/index.html`; the Linux CLI is `dist/bin/docx-fidelity` (3.0 MB).
- `npm run package`: passed. Crate package is 26 KB and ready for registry publishing; it was not published.
- Factory `verify-url.sh`: HTTP 200, no console errors, one `<h1>`, one `<main>`, `lang=en`, all images have alt text, all buttons have names.
- Lighthouse 12.8.2 mobile: performance 98, accessibility 100, best practices 100, SEO 100; LCP 2.2 s, FCP 1.4 s, TBT 0 ms, CLS 0.001.
- Initial assets: JavaScript 6.12 KB gzip, CSS 3.68 KB gzip, fonts 38 KB total, hero 160 KB.

Every command in `.factory/claims.json` supports `npm test -- --grep @claim:<id>` from a fresh checkout.

## Known boundaries

- This is a conversion and review tool, not a pixel-identical DOCX renderer. It does not run OCR, edit documents, render PDF, execute macros, or open embedded files.
- Complex page geometry, SmartArt, charts, equations, headers, footers, endnotes, text boxes, and fields become explicit review findings. Reviewers must replace or confirm them.
- The release binary in `dist/bin` targets this Linux worker. Build another Rust target before distributing to macOS or Windows.
- The factory still needs to register the slug with the Sociobot billing engine before the checkout URL can sell licenses. No product ID or secret is hardcoded.

## Next factory steps

1. Register the paid product and set its return URL to the deployed root.
2. Build signed binaries for supported operating systems and attach them to the release.
3. Deploy `dist/site/` through the factory static pipeline.
