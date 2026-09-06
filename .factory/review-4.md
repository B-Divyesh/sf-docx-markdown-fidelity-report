# Convert DOCX and list review issues — review 4

**Verdict: FAIL**

**Findings:** 2

**Untested public claims:** 3

**Live URL:** <https://docx-markdown-fidelity-report.sociobot.in>

**Reviewed:** 2026-09-06 UTC

**Implementation candidate:** `28b411e814f354f249356038fbd4d60b69448aad`

**Documentation SHA before this report:** `5e5c46410a5161ebf33311df6603fc4eba03984d`

## Decision

The conversion, demo, packaged CLI, live site, accessibility, privacy, and all 13 declared claims work. The review still fails because two public controls have no required claim entries or tagged tests, and one subjective public claim cannot be tested. PASS requires zero findings and zero untested claims.

## Job, audience, and first action

Before scrolling in fresh 1440 × 900 and 390 × 844 Chromium contexts, the page made these points clear:

- Job: “Convert DOCX and list review issues.”
- Audience: “For teams moving Word documentation, it shows where Markdown needs a human check.”
- First action: “Try it with sample data.”
- Result beside the action: “Convert the bundled field guide and open its fidelity report.”

The action and all three short product facts were visible without scrolling at both sizes.

## Findings

### F-4-1 — P2 — Two public actions are missing from the claims manifest

The live page offers **Copy commands** and **Replay recording**. Neither action is listed in `.factory/claims.json`, and neither has an exact `@claim:` test. The claim contract treats result-naming controls as promises that need an observable sandbox test.

Both actions worked in this manual review. Keyboard Space copied `cargo install --path .` and `docx-fidelity demo` to the clipboard. Keyboard Space also restarted the recording and it completed through the sandbox path. Working once does not replace the required repeatable claim coverage.

Required resolution: add one manifest entry and one exact tagged test for each action, or remove the actions.

Untested public claims: 2.

### F-4-2 — P2 — “The Markdown stays usable” is not a testable claim

The demo result says, “The Markdown stays usable.” “Usable” has no defined observable result, threshold, or matching entry in `.factory/claims.json`. The existing fidelity and source-preservation tests prove specific output properties, but they cannot prove this subjective sentence.

Required resolution: replace it with a concrete tested statement, such as the specific structures preserved by `source-fidelity`, or remove it.

Untested public claims: 1.

## Sample demo and data isolation

- The first action opened `/?demo=1` in one click.
- The sample screen immediately showed the bundled field guide, `field-guide.md`, `field-guide.fidelity.md`, nine findings, all seven issue categories, source locations, and the temporary sandbox.
- The banner “Demo — sample data, nothing is saved to your files” remained present in demo mode and included Reset demo and Start for real.
- With `real:review4=keep` and `demo:review4=discard` seeded, Reset demo removed the demo key, preserved the real key, and restored only `demo:sample`.
- Start for real removed all `demo:` keys, preserved `real:review4`, and returned home.
- Home and demo requests were same-origin only. No analytics, third-party script, upload, or document request occurred.
- The CLI demo ran from an unrelated empty directory and wrote only below its generated `/tmp/docx-fidelity-demo-*` directory.

## Declared claims

The manifest contains 13 entries. Each ID occurs in exactly one `@claim:<id>` test. Every exact command was run independently after `npm ci` in clean checkout `/tmp/docx-review4-clean`.

| Claim | Result |
| --- | --- |
| `demo-conversion` | Pass |
| `local-processing` | Pass |
| `fidelity-report` | Pass |
| `source-fidelity` | Pass |
| `batch-conversion` | Pass |
| `ci-policy` | Pass |
| `safe-input` | Pass |
| `demo-isolation` | Pass |
| `review-checklist` | Pass |
| `single-binary` | Pass |
| `rust-toolchain` | Pass |
| `scope-boundaries` | Pass |
| `mit-license` | Pass |

The three unlisted or untestable statements in F-4-1 and F-4-2 remain outside this otherwise complete manifest.

## CLI and clean consumer checks

The documented prerequisites were installed before runtime measurement: `npm ci` and Rust 1.88.0. The complete gates passed:

- `npm test`: 9 Rust unit tests, 1 doctest, and 31 Playwright tests.
- `npm run build`: release binary and `dist/site/` produced.
- `npm run lint`: formatting, clippy, and TypeScript checks passed.
- `npm run package`: the 0.1.4 crate was produced without publishing.
- `cargo +1.88.0 test --locked`: 9 unit tests and 1 doctest passed.

The crate was extracted and installed with `cargo install --locked --path ... --root ...` into an empty consumer root. The installed `docx-fidelity 0.1.4` passed `--help`, `--version`, and `--json demo`.

Normal, invalid, boundary, and recovery evidence:

- Normal conversion wrote Markdown, JSON, the human checklist, and extracted media; the report was blocked with nine source-located findings. The source DOCX hash was unchanged.
- Two colliding names, `Plan Q1.docx` and `Plan-Q1.docx`, produced distinct `Plan-Q1.*` and `Plan-Q1-2.*` outputs.
- A missing input and a `.pdf` input exited 2 with specific recovery text.
- Reusing an output directory exited 2 and named `--overwrite`; retrying with that option succeeded.
- `--fail-on error` wrote the outputs and exited 3.
- Declared boundary tests rejected archive traversal, a 32 MiB plus one byte part, and 10,001 entries. Embedded objects and macros were reported but not extracted or run.
- Unit regressions passed for malformed XML failure, stale-media cleanup, Markdown escaping, ordered and nested lists, and unknown-list source locations.

## Live pages and accessibility

- `/`, `/?demo=1`, `/demo`, `/privacy`, and `/terms` returned 200. `/not-a-route` returned the expected HTTP 404 and rendered the designed recovery page. Its browser resource error is the expected result of loading that deliberate 404.
- Every route at desktop and phone size had `lang="en"`, one H1, one main landmark, header/navigation/footer landmarks, route-specific title and description, canonical and social metadata, image alt text, and working legal links.
- All HTTP links returned 200. Hash links and `mailto:` links were intentional.
- Axe reported zero violations on all five routes at both viewports. The factory `verify-url.sh` passed on home and the query demo with no console or page errors.
- All visible links and buttons measured at least 44 × 44 CSS pixels. No route overflowed at 390 px or with text enlarged to 200%.
- Tab reached the skip link, navigation, sample action, scrollable report, copy button, and footer. Focus used a 3 px vermilion outline. Enter navigation moved focus to the new H1; Back restored the home H1.
- Reduced motion shortened contour and transition durations to 0.00001 seconds and showed the full terminal result immediately.
- The documented single light paper treatment passed axe contrast checks. No second theme is promised.

## Privacy, offline behavior, and performance

- Live requests during the complete sample path stayed on the product origin. The site has no cookie, analytics, external font, third-party script, or document upload request.
- The CSP is delivered as a response header and limits scripts, styles, fonts, images, and connections to self. `frame-ancestors 'none'`, `object-src 'none'`, HSTS, `nosniff`, referrer policy, and permissions policy are present.
- The service worker controlled a fresh context, offline `/demo` reload retained its title, H1, and sample banner, and `registration.update()` completed after reconnecting.
- Mobile Lighthouse: Performance 100, Accessibility 100, Best Practices 100, SEO 100; LCP 1.20 seconds, CLS 0.00065, total blocking time 1 ms.
- Initial JavaScript is 14.73 KB raw and 5.33 KB gzip. CSS is 12.97 KB raw and 3.73 KB gzip. Fonts total 38.0 KB and the hero image is 162.1 KB.

This is a static site plus local CLI. It has no product backend, account, tenant, database, health endpoint, or hosted rate-limited request path, so backend-only isolation, restart persistence, health, and 429 checks do not apply.

## Live version and candidate identity

The live root, 404 page, JavaScript, source map, CSS, fonts, images, icons, service worker, robots file, sitemap, and test stylesheet matched all 16 public files in the fresh build byte for byte. The deployment configuration is not a public asset.

`28b411e814f354f249356038fbd4d60b69448aad` is the last implementation change. Commit `69cf9825d8a24a0c5a5da45d79fce897e6c87a1d` changed only README demo wording and one test assertion. Later commits through documentation SHA `5e5c46410a5161ebf33311df6603fc4eba03984d` contain reports and evidence. The implementation tree is unchanged after `28b411e`.

## Earlier finding disposition

| Earlier issue | Current evidence |
| --- | --- |
| Broken checkout and absent paid product | Closed by the source-of-truth brief's documented `free-open-source` scope. The complete CLI is MIT licensed and makes no paid promise. |
| Hashed assets lacked long caching | Fixed. Hashed JS returns `max-age=31536000, immutable`; HTML and the service worker revalidate after 30 seconds. |
| Param Factory footer link failed TLS | Fixed. The link is `https://sociobot.in/` and returned 200. |
| Batch output-name collisions lost data | Fixed. Installed consumer and regression test produced two distinct result sets. |
| Malformed XML reported clear | Fixed. The incomplete-document unit regression passed. |
| Overwrite retained stale media | Fixed. The stale-media cleanup regression passed. |
| Controls were below 44 px | Fixed. No undersized control appeared on any route at either viewport. |
| Browser sample counts and categories disagreed | Fixed. The sample shows nine findings and seven categories matching the bundled CLI result. |
| Unknown routes returned 200 | Fixed. The designed route returns HTTP 404. |
| Safety and batch claims had narrow tests | Fixed for the declared claims. Traversal, embedded binaries, size, entry count, and colliding names are asserted. |
| Release versions disagreed | Fixed. Site, crate, and installed binary report 0.1.4. |
| Literal Markdown-looking Word text changed meaning | Fixed. Source-fidelity tests prove escaping. |
| Ordered lists became bullets | Fixed. Decimal and nested list tests pass; unknown numbering gets a source-located finding. |
| Rust 1.80 support was false | Fixed by declaring and testing Rust 1.88. |
| Public claims were missing from the six-item manifest | Improved to 13 correctly tagged claims, but F-4-1 and F-4-2 identify three remaining public statements without compliant coverage. |
| Privacy text clipped at 200% | Fixed. Every route stayed within 390 px and each H1 fit its box. |
| Product name, metaphor headings, output terms, and unexplained CI wording | Fixed. The live copy uses one name, literal headings, “fidelity report,” and “automated checks”; README defines CI before using it. |

## Missed feature check

No additional AI, sync, or import feature is implied. Local DOCX input and Markdown, image, JSON, and checklist outputs complete the stated job. Adding a hosted model step would conflict with the local-document boundary.

## Evidence paths

Detailed browser JSON, screenshots, claim logs, clean-gate logs, consumer output, response headers, byte comparisons, reduced-motion output, and Lighthouse JSON are under `/work/.evidence/review-4/`. The required report copy is `/work/.evidence/qa-report.md`.
