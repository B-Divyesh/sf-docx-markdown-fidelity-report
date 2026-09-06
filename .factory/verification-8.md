# Convert DOCX and list review issues — verification 8

- **Verdict:** FAIL
- **Findings:** 1
- **Untested public claims:** 1
- **Implementation candidate:** `4d5864947bd41e6d5f7cad4b20a0f3c5ff702fb4`
- **Documentation baseline:** `62e91bcc6b3d02f407de15f59c602e5049d3389d`
- **Live URL:** <https://docx-markdown-fidelity-report.sociobot.in>
- **Verified:** 2026-09-06 UTC from a fresh GitHub clone

## Verdict

**FAIL.** The conversion job, live demo, installed CLI, accessibility checks, and all 16 declared claims pass. The README also promises a machine-readable `--json` command summary, but that public outcome is absent from `.factory/claims.json` and has no dedicated `@claim:` test. The contract permits PASS only with zero findings and zero untested public claims.

## Job, audience, and first action

Before scrolling in fresh 1440 × 900 desktop and 390 × 844 phone contexts:

- Job: **“Convert DOCX and list review issues.”**
- Audience: **“For teams moving Word documentation, it shows where Markdown needs a human check.”**
- First action: **“Try it with sample data.”**
- Stated result: **“Convert the bundled field guide and open its fidelity report.”**

The action and all three product facts fit in both first screens.

## Finding

### F-8-1 — P2 — The documented JSON output has no declared claim

README line 38 says **“Write a machine-readable command summary to stdout”** and gives `docx-fidelity --json convert …`. The installed CLI also exposes `--json` as **“Print one JSON command summary to stdout.”** None of the 16 entries in `.factory/claims.json` names this outcome, and no test is tagged `@claim:json-output` or an equivalent ID.

The behavior worked during this review: the clean installed binary returned parseable JSON for `demo` and `convert`. That does not satisfy the attached claims contract, which requires each public promise to be listed and owned by one dedicated claim test. Existing tests use JSON as a helper while proving other claims; they do not inventory this README promise.

Required resolution: add a JSON-output claim and one tagged test that runs the installed or release CLI in a fresh directory, parses stdout, checks the documented fields, and confirms stdout contains only the machine-readable summary. Alternatively, remove the public promise and option copy if JSON output is not supported as a product contract.

## Declared claims

After `npm ci` in a fresh clone at the documentation baseline, every exact command in `.factory/claims.json` was run independently.

| Claim | Exact command | Result |
| --- | --- | --- |
| `demo-conversion` | `npm test -- --grep @claim:demo-conversion` | PASS |
| `local-processing` | `npm test -- --grep @claim:local-processing` | PASS |
| `fidelity-report` | `npm test -- --grep @claim:fidelity-report` | PASS |
| `source-fidelity` | `npm test -- --grep @claim:source-fidelity` | PASS |
| `batch-conversion` | `npm test -- --grep @claim:batch-conversion` | PASS |
| `ci-policy` | `npm test -- --grep @claim:ci-policy` | PASS |
| `safe-input` | `npm test -- --grep @claim:safe-input` | PASS |
| `demo-isolation` | `npm test -- --grep @claim:demo-isolation` | PASS |
| `copy-commands` | `npm test -- --grep @claim:copy-commands` | PASS |
| `replay-recording` | `npm test -- --grep @claim:replay-recording` | PASS |
| `sample-markdown` | `npm test -- --grep @claim:sample-markdown` | PASS |
| `review-checklist` | `npm test -- --grep @claim:review-checklist` | PASS |
| `single-binary` | `npm test -- --grep @claim:single-binary` | PASS |
| `rust-toolchain` | `npm test -- --grep @claim:rust-toolchain` | PASS |
| `scope-boundaries` | `npm test -- --grep @claim:scope-boundaries` | PASS |
| `mit-license` | `npm test -- --grep @claim:mit-license` | PASS |

The manifest/tag audit found 16 claim IDs and exactly one matching `@claim:<id>` test for each, with no extra claim tags. The separate README audit found F-8-1.

## Clean build and installed CLI

The following passed from the fresh clone:

```sh
npm ci
npm test
npm run lint
npm run build
npm run package
cargo +1.88.0 test --locked
```

- `npm test`: 9 Rust unit tests, 1 doctest, and 34 Playwright tests passed.
- `npm run build`: produced `dist/bin/docx-fidelity` and `dist/site`.
- `npm run package`: produced `docx-markdown-fidelity-report-0.1.5.crate`.
- The crate was extracted and installed with `cargo install --locked --path … --root …` in a new consumer directory.
- The installed binary reported `docx-fidelity 0.1.5`; `--help`, `--version`, JSON demo, and JSON conversion worked.
- The normal sample wrote Markdown, JSON fidelity report, human checklist, and extracted SVG. Its report was blocked with nine findings in all seven promised categories and included source locations.
- The Markdown contained the promised heading, link, table, image reference, and footnote.
- The source DOCX hash was unchanged.
- Repeating conversion without `--overwrite` exited 2 with recovery guidance; `--overwrite` then exited 0. A non-DOCX input exited 2 with an actionable error.
- Claim and regression tests covered normalized filename collisions, incomplete XML, stale media cleanup, Markdown-looking literal text, decimal and nested lists, unknown numbering, unsafe paths, 32 MiB limits, 10,001 ZIP entries, macros, and embedded objects.

## Live desktop and phone checks

- Production HTML, hashed JavaScript, and CSS matched the clean candidate build byte-for-byte. The later `62e91bc` commit changes only `.factory/handoff.md`, so `4d58649` is the implementation reviewed.
- `/`, `/?demo=1`, `/demo`, `/privacy`, and `/terms` returned 200 with their own titles, one H1, `lang="en"`, and one main landmark.
- `/not-a-route` returned the expected HTTP 404 and rendered the designed recovery page. This is not a defect.
- One click loaded the bundled field-guide sample. The persistent demo label remained visible after scrolling. The recording named `field-guide.docx`, its Markdown and fidelity report, nine findings, seven categories, source locations, and a temporary sandbox.
- Reset demo removed only seeded `demo:` data, restored `demo:sample=field-guide`, and preserved a seeded `real:` value. Start for real removed demo state and preserved the real sentinel.
- Copy commands and Replay recording both worked from the keyboard on production.
- The skip link was first in tab order, had a 3 px visible focus ring, and moved focus to `#main`. Route changes and browser back moved focus to the route H1.
- All visible links and buttons measured at least 44 × 44 CSS pixels on the phone. Every route fit at 390 px with text set to 200%.
- Reduced motion showed the complete recording without waiting for animated output.
- Axe found zero violations on every desktop route and zero serious or critical violations on every phone route. `verify-url.sh` passed with no console or page errors.
- The service worker activated, completed `registration.update()`, and reloaded the demo offline with its H1 and demo label.
- All crawled site links succeeded, including the Param Factory link. `robots.txt` and `sitemap.xml` list the valid public routes.

## Privacy, headers, and performance

- The complete live sample flow made same-origin requests only. No analytics, third-party script, document upload, or cookie was observed.
- CSP is delivered as a response header and includes `connect-src 'self'`, `object-src 'none'`, and `frame-ancestors 'none'`. HSTS, `nosniff`, referrer policy, and permissions policy are present.
- Hashed JS and CSS use `max-age=31536000, immutable`; HTML and the service worker use 30-second revalidation.
- Initial JavaScript is 14,786 bytes raw and 5,351 bytes gzip. CSS is 12,973 bytes raw and 3,732 bytes gzip. Self-hosted fonts total 38,024 bytes; the hero image is 162,082 bytes.
- Live mobile Lighthouse: Performance 100, Accessibility 100, Best Practices 100, SEO 100; FCP 1.06 s, LCP 1.06 s, TBT 0 ms, CLS 0.0012.
- This is a static site plus local CLI. It has no product backend, tenant data, database, health endpoint, or live request allowance, so backend isolation, restart persistence, and 429 checks do not apply.

## Earlier finding disposition

| Earlier finding | Current evidence |
| --- | --- |
| Unavailable paid checkout and absent paid product | Closed by the source-of-truth `free-open-source` scope. The complete CLI is MIT licensed and the site makes no paid offer. |
| Hashed assets had short caching | Fixed. Live hashed assets are immutable for one year. |
| Param Factory footer link failed TLS | Fixed. `https://sociobot.in/` returned 200. |
| Normalized batch names overwrote results | Fixed. The collision claim produced distinct output paths. |
| Malformed document XML reported clear | Fixed. The incomplete-XML regression fails closed. |
| Overwrite retained stale media | Fixed. The stale-media regression passed. |
| Controls were below 44 px | Fixed. No undersized control was found on any live route. |
| Demo category counts disagreed | Fixed. Seven categories sum to nine findings on production and in the installed sample. |
| Unknown routes returned 200 | Fixed. The designed unknown route returns HTTP 404. |
| Safe-input and batch claim tests were too narrow | Fixed. Current claims cover collisions, traversal, embedded binaries, size, and entry count. |
| Release versions disagreed | Fixed. Site, crate, changelog, and installed binary identify v0.1.5. |
| Literal Word text became Markdown syntax | Fixed. The source-fidelity claim covers headings, links, lists, code, quotes, and table syntax. |
| Decimal lists became bullets | Fixed. Decimal and nested lists are preserved; unknown numbering is source-located. |
| Rust 1.80 support was false | Fixed. Rust 1.88 built and tested the locked package. |
| Privacy text clipped at 200% | Fixed. All live routes fit at 390 px and each H1 stayed within its box. |
| Product names, metaphor headings, report terms, and CI wording were inconsistent | Fixed. Current copy uses one name, literal headings, “fidelity report,” and explains automated checks. |
| Copy commands, Replay recording, and “The Markdown stays usable” | Fixed. The first two have exact claims; the third was replaced by tested sample structures. |

No earlier product defect regressed. F-8-1 is a newly identified claim-inventory gap.

## Missed feature check

No additional feature is required by the researched job. OCR, hosted sync, document editing, and AI processing would conflict with the stated local conversion and review scope. The current import, Markdown output, image extraction, JSON report, and human checklist cover the useful handoff.

## Reproduce

```sh
npm ci
# Run each command in .factory/claims.json independently.
npm test
npm run lint
npm run build
npm run package
cargo +1.88.0 test --locked
./dist/bin/docx-fidelity --json demo
```
