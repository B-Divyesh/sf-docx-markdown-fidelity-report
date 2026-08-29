# Independent verification 4 — FAIL

- Candidate: `b81c7a3f94e18852dee4087775c3ec0c64b28077`
- Live URL: https://docx-markdown-fidelity-report.sociobot.in
- Verified: 2026-08-29 UTC from the candidate checkout
- Acceptance source: `.factory/brief.json`, the supplied work order, and attached product skills

## Release decision

**FAIL.** The live deployment byte-matches the candidate, the first-read/demo gate passes, and the declared post-install test suite passes. Independent DOCX fixtures nevertheless show ordinary Word content changing meaning while the fidelity ledger reports `clear`. The documented Rust 1.80 minimum also fails. These are product defects, not a deployment-only failure.

## Release-blocking defects

| Severity | Finding | Fresh evidence | Contract impact |
| --- | --- | --- | --- |
| P1 | Plain Word text is emitted as active Markdown syntax with no finding. | A DOCX with three ordinary paragraphs—`# Plain Word paragraph`, `[payroll](https://attacker.example)`, and `- Plain dash paragraph`—exited 0. The output became a Markdown heading, link, and list, while `plain-markdown-syntax.fidelity.json` said `status: clear` and contained zero findings. `handle_end` passes body text through `escape_markdown_minimal`, which only removes NUL (`src/lib.rs:783-793`, `1439-1446`). | Text meaning changes silently. This directly violates the smallest useful product and success measure that every lossy construct be source-located before acceptance. |
| P1 | Decimal numbered lists are silently changed to unordered bullets. | A valid DOCX with `numbering.xml`, decimal `numId=42`, and two ordered items produced `- First required step` and `- Second required step`. The report again said `clear` with zero findings. All list paragraphs receive the literal `- ` prefix (`src/lib.rs:826-829`); numbering definitions and levels are not read. | Order can carry procedure meaning. The converter neither preserves it nor warns a reviewer, contradicting the fidelity-ledger job. |
| P1 | The documented and declared Rust 1.80 minimum does not build the locked package. | `rustup toolchain install 1.80.0 --profile minimal` succeeded, then `cargo +1.80.0 test --locked` exited 101: `icu_normalizer 2.3.0` requires Cargo feature `edition2024`, unavailable in Cargo 1.80. `Cargo.toml` declares `rust-version = "1.80"`; README says “stable Rust 1.80 or newer.” | A supported installation path fails before compilation. Raise the minimum to a tested version or pin compatible dependencies, and add a toolchain claim test. |
| P1 / claims contract | Several visitor-facing claims are absent from `.factory/claims.json`. | Unlisted examples include “Demo — sample data, nothing is saved,” “single Rust binary,” the Rust 1.80 minimum, bounded ZIP/XML parts, and the separate human checklist output. The manifest contains only six claims. Some behaviors passed manual QA, but the required one-claim/one-test inventory is absent; the 1.80 claim is false. | The supplied claims contract says any unlisted claim fails review. Add exact manifest entries and observable tests, or remove/narrow the copy. |
| P1 / accessibility | The privacy page loses text at 200% text size on a 390 px viewport. | With the root font size set to 200%, `/privacy` measured 464 px of document width for a 390 px viewport. Its H1 had 444 px of content in a 350 px box, and “documents” was visibly clipped at the right edge. Programmatic horizontal scrolling remained at `scrollX = 0`. Evidence: `.factory/qa-4/live-privacy-text-200.png`. | Violates the attached non-negotiable requirement that text resize to 200% without loss. The large balanced heading needs to wrap at long-word boundaries. |

## Other gap

| Severity | Finding | Evidence |
| --- | --- | --- |
| P2 / scope | The researched one-time purchase is not shipped. | The page has no price, checkout, purchase restore, or paid feature. `GET https://api.sociobot.in/api/v1/products/docx-markdown-fidelity-report/checkout` returned 404 with `{"error":"enabled factory product","status":404}`. The prior handoff explains this as an honest factory-registration limitation, but it remains a deviation from the researched monetization. |

## Required claims gate

`.factory/claims.json` exists, and each of its six IDs appears in exactly one tagged test. The first literal invocations from the untouched clone reached the site build and exited 127 because dependencies were not installed (`vite: not found`). After the required clean-checkout install step (`npm ci`), every exact manifest command was rerun independently and passed:

| Claim | Exact command | Result |
| --- | --- | --- |
| `demo-conversion` | `npm test -- --grep @claim:demo-conversion` | PASS; one matching browser test, demo wrote Markdown, reports, media, and nine findings. |
| `local-processing` | `npm test -- --grep @claim:local-processing` | PASS; one matching browser test, same-origin browser requests and CLI success with unreachable proxies. |
| `risk-ledger` | `npm test -- --grep @claim:risk-ledger` | PASS; all seven listed categories and source locations in the bundled fixture. |
| `batch-conversion` | `npm test -- --grep @claim:batch-conversion` | PASS; colliding normalized filenames produced distinct outputs. |
| `ci-policy` | `npm test -- --grep @claim:ci-policy` | PASS; blocked fixture returned exit 3 with unreachable proxies. |
| `safe-input` | `npm test -- --grep @claim:safe-input` | PASS; traversal rejected and embedded binary not extracted. |

The post-install claims suite does not cover the independent failing fixtures or the unlisted claims above.

## Cold first-read and demo gate

**PASS.** In a fresh 1440×900 context, before inspecting implementation details, the first screen answered all three questions:

- What: **“Convert DOCX. Map every review risk.”**
- For whom/outcome: **“For teams moving Word documentation, it shows where Markdown needs a human check.”**
- First click: **“Try it with sample data”**, beside a plain explanation of what follows.

The action is above the fold and opens `/demo` in one click. The resulting screen immediately shows the recorded conversion and persistent **Demo — sample data, nothing is saved** banner with **Reset demo** and **Start for real**. Evidence: `.factory/qa-4/live-first-read-desktop.png`, `.factory/qa-4/live-home-mobile.png`, and `.factory/qa-4/live-demo-mobile.png`.

## Clean checkout, build, and package

- HEAD before QA: exact candidate SHA; branch `main` matched `origin/main`.
- `npm ci`: PASS; 24 packages, 0 npm audit vulnerabilities.
- `npm test`: PASS; 6 Rust unit tests, 1 Rust doctest, and 21 Playwright project tests.
- `npm run lint`: PASS; rustfmt, Clippy with warnings denied, and TypeScript type checking.
- `npm run build`: PASS; release CLI and `dist/site/` produced.
- `npm run package`: PASS; 10-file crate, 95.4 KiB unpacked / 28.3 KiB compressed.
- Release binary: 3,156,656 bytes.
- Clean package consumer: the `.crate` was extracted and installed with `cargo install --locked --path ... --root ...`; CLI `--version`, `--help`, JSON demo, direct conversion, and a separate Rust consumer calling `convert_path` all passed. Both conversion paths produced nine findings and all four artifact types.

## Independent CLI cases

Passing behavior:

- Normal complex sample, uppercase `.DOCX`, safe normalization of an all-punctuation filename, and batch collision handling.
- Clear input returned 0 under `--fail-on warning`; a warning-only table returned 3 for `warning` and 0 for `error`.
- Missing input, empty directory, wrong extension, corrupt ZIP, 10,001-entry archive, and a 32 MiB + 1 byte part returned exit 2 with actionable errors.
- Existing output returned exit 2; repeating with `--overwrite` recovered with exit 0.
- A synthetic macro part was not executed or extracted and produced a source-located blocking finding.

Failing meaning-preservation cases are the two P1 defects above.

## Live deployment identity

All 15 publicly served build files matched the fresh `dist/site` output byte-for-byte, including HTML, hashed JS/CSS and source map, three fonts, images, icons, robots, sitemap, and service worker.

- HTML/404 SHA-256: `b297cd08fa199558e9f6f98c8a7e6de51aac006c5f05f9e9dc5d334ab5479dd9`
- JS SHA-256: `659aba87a4f8c924bf407b8191fc6601f39f226bcf1304fdf697ffc3fc364b18`
- CSS SHA-256: `f4ab758dd13ec12944175e5876abe71dd393fa31c4ed3f962c000a52dbd43ec0`
- Service worker SHA-256: `17e664e753829fb3f824759d77feec04ad8081d1cf264e9ca9a23148c4f7cf58`

The footer reports v0.1.1/build 2026.08.29. No separate commit hash is exposed, so byte equality is the build-identity evidence.

## Browser, accessibility, privacy, and routing

Fresh Chromium contexts covered `/`, `/demo`, `/privacy`, `/terms`, and `/not-a-route` at 1440×900 and 390×844.

- Valid routes returned 200; the designed unknown route returned a real 404.
- Each page had route-specific title, `lang="en"`, one `<main>`, one `<h1>`, ordered headings, image alt text, and no normal-size horizontal page overflow.
- Axe 4.10.2 found zero serious/critical findings on every route and viewport. Every visible link/button measured at least 44×44 CSS px.
- First Tab exposed the skip link with a 3 px visible focus ring; Enter focused `main`. Keyboard activation of the demo link focused the new H1. Replay, Reset, Start for real, Back, and Forward were operable.
- At 200% root text size, `/`, `/demo`, `/terms`, and the 404 stayed at 390 px document width, and the terminal remained keyboard-scrollable. `/privacy` failed as described above. Evidence: `.factory/qa-4/live-mobile-text-200.png` and `.factory/qa-4/live-privacy-text-200.png`.
- Reduced motion forced animation and transition durations to `0.00001s`, and the demo output appeared immediately.
- Valid routes had no console or page errors. The intentional HTTP 404 navigation produced only Chromium's expected failed-resource console diagnostic.
- Complete landing → demo → replay → reset → leave request logging observed only `https://docx-markdown-fidelity-report.sociobot.in`. No cookies were set. Reset and leave removed an injected `demo:` key while preserving a `real:` sentinel.
- `/opt/fleet/lib/verify-url.sh` passed in 709 ms with title/lang/main/H1/alt checks and zero load errors. Machine output is `.factory/qa-4/verify-url/verify.json`.
- `robots.txt`, the four-route sitemap, icons, social image, privacy/terms links, and the external Param Factory link all returned expected success statuses.

## Headers, caching, offline, endpoints, and performance

- HTML/404: `Cache-Control: public, must-revalidate, max-age=30`.
- Hashed assets: `Cache-Control: public, max-age=31536000, immutable`.
- Live responses include header-delivered CSP with `frame-ancestors 'none'`, HSTS, `nosniff`, `Referrer-Policy`, and `Permissions-Policy`.
- The service worker activated as `/sw.js`, created versioned cache `docx-fidelity-shell-index-3NirrwxN.js`, updated successfully, and reloaded `/demo` offline with its heading/banner and no errors.
- There is no product backend or sign-in. The retained Sociobot verify endpoint accepted 30 rapid invalid-token requests; request 31 returned 429 with `Retry-After: 2`. The checkout endpoint is unavailable as noted above.
- Production sizes: JS 14,549 bytes / 5,295 gzip; CSS 12,742 bytes / 3,667 gzip; self-hosted fonts 38,024 bytes; hero WebP 162,082 bytes.
- Lighthouse 13.4.1 mobile: performance 99, accessibility 100, best practices 100, SEO 100; FCP 1.1 s, LCP 2.0 s, TBT 70 ms, CLS 0.001, speed index 1.1 s.

## Required retest

1. Escape source text without escaping converter-authored Markdown, and add regressions for headings, links, lists, code, blockquotes, and table syntax originating as plain Word text.
2. Parse `numbering.xml` and preserve ordered/unordered list type and nesting, or add a source-located loss finding whenever exact list semantics cannot be represented.
3. Make the declared Rust minimum compile from the locked package and test that exact toolchain.
4. Inventory every page/README claim in `.factory/claims.json`; add one observable tagged test per claim or narrow the copy.
5. Allow the privacy H1 and other long text to wrap without clipping at 200% text size on 390 px screens.
6. Decide whether the researched one-time purchase remains intentionally deferred after factory registration.
