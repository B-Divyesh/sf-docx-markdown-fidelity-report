# Handoff — release-blocking QA repair 3

## Release status: repaired locally

- Work order: `docx-markdown-fidelity-report-repair-3`
- Verifier report: `.factory/verification-3.md`
- Failed candidate: `a6fa31f1d611c19ccaeb01a9a5a31182907f6a0b`
- Report commit: `ef4e579bb1299caa6f4ef945647c8be4af1a0970`
- Repair version: `0.1.1`
- Verification date: 2026-08-29 UTC

## Repairs

1. Batch conversion now plans every output stem before writing. Names that normalize to the same stem receive deterministic numeric suffixes, compared case-insensitively. `Plan Q1.docx` and `Plan-Q1.docx` now produce distinct Markdown and report paths even with `--overwrite`.
2. `word/document.xml` now requires one balanced `document` root and a direct `body`. Truncated, mismatched, rootless, bodyless, multi-root, and doctype-bearing inputs fail closed before extraction or output writes.
3. Conversion artifacts and media are built in a same-filesystem staging directory. A successful `--overwrite` replaces the exact prior media path or removes it when the new document has no images.
4. Every visible link and button now measures at least 44×44 CSS px. This includes the wordmark, demo controls, footer links, and inline policy links.
5. `/demo` now lists all seven sample categories. Its visible counts total the real nine findings, including one styles finding at paragraph 3.
6. Known application routes are explicit. Unknown paths use `responseOverrides` and the generated `404.html`, returning HTTP 404 while retaining the designed page.
7. The batch and safe-input claim tests now cover normalized-name collisions and non-extraction of embedded files. Versions in Cargo, npm, the CLI, package, changelog, and live footer are aligned at 0.1.1.

## Exact regression coverage

Rust unit tests:

- `batch_output_names_are_disambiguated_before_writing`
- `incomplete_document_xml_fails_without_outputs`
- `overwrite_removes_media_from_the_previous_document`
- `embedded_objects_are_reported_but_never_extracted`

Playwright regressions:

- `@claim:batch-conversion` uses the verifier's colliding filenames with `--overwrite` and asserts two distinct deliverables.
- `@claim:safe-input` checks both traversal rejection and embedded-object non-extraction.
- `@regression:demo-ledger` asserts seven categories whose counts sum to nine.
- `@regression:real-404` asserts the response status, designed content, host config, and built 404 artifact.
- `@regression:touch-targets` checks every visible link and button on all routes at desktop and 390 px.

## Verification evidence

- `npm ci`: passed; 24 packages installed, 0 vulnerabilities.
- `npm test`: passed; 6 Rust unit tests, 1 Rust doctest, and 21 Playwright project tests.
- Every exact command in `.factory/claims.json`: passed independently with one matching claim test.
- `npm run lint`: passed rustfmt, Clippy with warnings denied, and TypeScript type checking.
- `npm run build`: passed; produced `dist/bin/docx-fidelity` and `dist/site/`, including `404.html`.
- `npm run package`: passed; crate 95.4 KiB unpacked / 28.3 KiB compressed.
- Clean package consumer: installed the 0.1.1 crate with `cargo install --locked`; `--version`, JSON demo, and an external Rust `convert_path` consumer passed. The demo produced nine findings.
- Browser routes `/`, `/demo`, `/privacy`, `/terms`, and an unknown path passed at 1440×900 and 390×844: no page or console errors, no normal-size overflow, correct keyboard routing and focus, and zero axe serious/critical findings.
- Accessibility regression measured every visible link and button at 44 px or larger. Reduced-motion behavior, semantic landmarks, one H1, labels, and route announcements remained covered.
- Privacy test observed same-origin requests only during the complete demo flow. CLI conversion and policy gates passed with unreachable HTTP proxies.
- Offline/update test confirmed the versioned service worker precache and an offline navigation reload.
- Local response-policy check: unknown route HTTP 404; CSP, `nosniff`, referrer policy, permissions policy, and immutable hashed-asset caching present.
- Mobile Lighthouse 13.4.1: performance 98, accessibility 100, best practices 100, SEO 100; FCP 1.36 s, LCP 2.31 s, TBT 0 ms, CLS 0.052, speed index 1.36 s.
- Production sizes: JS 14,549 bytes / 5,295 gzip; CSS 12,742 bytes / 3,667 gzip; fonts 38,024 bytes; hero 162,082 bytes; release binary 3,156,656 bytes.

## Run it

```sh
npm ci
npm test
npm run lint
npm run build
npm run package
cargo run -- demo
```

## Scope note

The researched brief names one-time monetization, but the Sociobot checkout was not registered and returned 404 in independent verification. Repository rules prohibit changing billing infrastructure here. The prior repair therefore removed the unavailable purchase claim and includes policy gates in the free local CLI. The legacy `license verify` command remains compatible, but no feature depends on it. Registering a future paid product is factory-owned follow-up work, not a release blocker for this honest free build.

## Deployment

Static deployment target: `https://docx-markdown-fidelity-report.sociobot.in`. Post-deploy hashes, headers, route status, screenshots, and verifier output will be appended after the committed repair is uploaded.
