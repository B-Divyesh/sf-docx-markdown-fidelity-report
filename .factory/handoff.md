# Handoff — repair 5

## Status: PASS

The product now has a tested claim for each visible result-naming control and every public outcome sentence in the demo is concrete. The local CLI still converts DOCX directly to Markdown and writes the source-located fidelity report, JSON report, checklist, and extracted images.

## What changed

- Added three tested public claims: **Copy commands** writes both install commands to the clipboard; **Replay recording** starts the sample recording again and reaches its sandbox result; the sample Markdown contains a heading, link, table, image reference, and footnote.
- Replaced “The Markdown stays usable” with the concrete sample-structure statement above.
- Documented the replay action in `.factory/demo.md` and added it to the copy audit.
- Bumped the CLI and site build to `0.1.5`.

Implementation SHA: `4d5864947bd41e6d5f7cad4b20a0f3c5ff702fb4` (`fix: cover demo controls and Markdown claim`).

## Verification

### Clean setup and claims

From a new clone at the implementation SHA:

```sh
npm ci
# Every exact command listed in .factory/claims.json, independently:
npm test -- --grep @claim:demo-conversion
npm test -- --grep @claim:local-processing
npm test -- --grep @claim:fidelity-report
npm test -- --grep @claim:source-fidelity
npm test -- --grep @claim:batch-conversion
npm test -- --grep @claim:ci-policy
npm test -- --grep @claim:safe-input
npm test -- --grep @claim:demo-isolation
npm test -- --grep @claim:copy-commands
npm test -- --grep @claim:replay-recording
npm test -- --grep @claim:sample-markdown
npm test -- --grep @claim:review-checklist
npm test -- --grep @claim:single-binary
npm test -- --grep @claim:rust-toolchain
npm test -- --grep @claim:scope-boundaries
npm test -- --grep @claim:mit-license
```

All 16 commands passed. The manifest/tag audit found 16 claim IDs and exactly one matching `@claim:<id>` test for each, with no extra claim tags.

Additional local gates passed:

```sh
npm test                  # 9 Rust unit tests, 1 doctest, 34 Playwright tests
npm run lint              # rustfmt, clippy -D warnings, TypeScript
npm run build             # dist/bin/docx-fidelity and dist/site
npm run package           # docx-markdown-fidelity-report-0.1.5.crate
cargo +1.88.0 test --locked
```

The packaged crate was extracted into a fresh temporary consumer root, installed with `cargo install --locked --path ... --root ...`, and its installed `docx-fidelity 0.1.5` completed `--help`, `--version`, and `--json demo`. The demo wrote one blocked conversion with nine findings in a new operating-system temporary directory.

### Production deployment

- Deployed `dist/site` with the existing `sf-docx-markdown-fidelity-report` static-web-app configuration. The existing Central US product resource and custom domain were reused; no backend, volume, replica, or service configuration changed.
- HTTPS cold check: `https://docx-markdown-fidelity-report.sociobot.in` returned 200. The deployed HTML and hashed JS both matched the fresh build byte-for-byte.
- Fresh desktop (1440 × 900) and phone (390 × 844) contexts identified the job before scrolling: “Convert DOCX and list review issues.” The audience was “teams moving Word documentation,” and the first action was “Try it with sample data.” The action and all three facts were visible on both screens.
- One click opened the realistic field-guide recording with the persistent demo banner, nine findings, seven issue categories, and sample label. Reset removed only `demo:` state and retained a seeded `real:` value; Start for real removed demo state only. Keyboard Space copied both commands and replayed the completed recording.
- Live Playwright Axe scans found zero serious or critical issues on `/`, `/demo`, `/privacy`, `/terms`, and the designed `/not-a-route` page at both viewports. Every phone route had a 390 px page width, and all routes stayed within that width with text at 200%.
- `/not-a-route` returned the expected HTTP 404 while rendering the recovery page. Valid routes returned 200 with route titles and one H1.
- `verify-url.sh` passed: title, `lang=en`, main landmark, one H1, alt text, labels, and no console/page errors.
- The service worker activated, updated, and reloaded `/demo` offline with its demo title, H1, and banner. Reduced motion rendered the complete recording immediately.
- The complete live sample flow made same-origin requests only. No upload, analytics, third-party script, cookie, or document-data request was observed. Response headers include CSP with `connect-src 'self'`, `object-src 'none'`, and header-delivered `frame-ancestors 'none'`; HTML revalidates after 30 seconds and hashed assets are immutable for one year.
- Mobile Lighthouse on the live demo: Performance 99, Accessibility 100, Best Practices 100, SEO 100; FCP 1.1 s, LCP 1.2 s, TBT 150 ms, CLS 0.001.

Production evidence is in `/work/.evidence/repair-5-live/`. The verb-first 74-character catalog description was copied to `/work/.evidence/catalog-description.txt`.

## Earlier findings

| Earlier finding | Current disposition |
| --- | --- |
| Unavailable checkout / one-time offer | Closed by the current source-of-truth brief: the product is a fully usable free MIT CLI and advertises no price, checkout, or paid unlock. There is no live offer to register and no invented billing metadata. |
| Short cache lifetime for hashed assets | Fixed: live JS uses `public, max-age=31536000, immutable`. |
| Broken Param Factory link | Fixed: the footer uses `https://sociobot.in/`. |
| Output collision, malformed XML, stale media | Fixed by existing converter regressions; unit tests and the clean claim suite pass. |
| Literal Markdown-looking text and list semantics changed silently | Fixed by existing source-fidelity coverage for escaping, ordered/nested lists, and source-located unknown numbering. |
| Rust 1.80 claim failed | Fixed by the documented and tested Rust 1.88 minimum. |
| Missing claims inventory | Fixed: the manifest now has 16 one-to-one outcome tests. |
| 44 px controls, 200% text clipping, route 404, and demo-count mismatch | Fixed and rechecked live. |
| Unexplained CI wording and inconsistent product/report names | Fixed in the existing plain-language copy; the current copy audit has no flags. |
| Copy commands, Replay recording, and “The Markdown stays usable” | Fixed in this repair with three exact claims and outcome checks. |

## Known gaps and next steps

- No product defect is known at this scope.
- The crate is ready to publish with `npm run package`; do not publish from this worker because the factory owns registry credentials.
- The product has no backend, account, database, tenant, health endpoint, or paid offer. Backend-only persistence, isolation, and rate-limit checks do not apply.
