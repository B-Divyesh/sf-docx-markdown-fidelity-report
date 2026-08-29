# Adversarial first-read review 3 — PASS

**Product:** Docx Markdown Fidelity Report  
**Live URL:** <https://docx-markdown-fidelity-report.sociobot.in>  
**Reviewed:** 2026-08-29 UTC  
**Method:** fresh Chromium contexts at 390 × 844 and 1440 × 900; clean clone at `5191511fa58b1c76c358dd252de6090a45055250`.

## Verdict: PASS

Zero blocking findings, zero minor findings, and zero untested product claims remain.

## Cold first read

Before scrolling, at both sizes, I understood:

- **What it does:** converts DOCX to Markdown and lists the places that need human review.
- **For whom:** teams moving Word documentation.
- **What to click first:** **Try it with sample data** to convert the bundled field guide and open its fidelity report.

The exact first-screen evidence is “Convert DOCX and list review issues.”, “For teams moving Word documentation, it shows where Markdown needs a human check.”, and “Try it with sample data”. The adjacent result text says “Convert the bundled field guide and open its fidelity report.” All three product facts remain visible at 390 × 844 without scrolling. This gate passes.

## Findings

None.

## Copy audit

Counts use whitespace-delimited words after stripping Markdown syntax and standalone separators such as `·` and `→`. Commands, paths, and hyphenated terms count as one word. Interface labels and headings are included because the review also requires useful headings and result-naming controls. Commands are listed separately and are not treated as prose sentences.

Every item below passes. No item exceeds 22 words. No banned marketing adjective, unexplained jargon, metaphor heading, inconsistent product term, or non-result-naming button remains.

### Landing page

| Copy | Words |
| --- | ---: |
| Docx Markdown Fidelity Report | 4 |
| Demo | 1 |
| Install | 1 |
| Privacy | 1 |
| Local DOCX conversion · v0.1 | 4 |
| Convert DOCX and list review issues. | 6 |
| For teams moving Word documentation, it shows where Markdown needs a human check. | 13 |
| Try it with sample data | 5 |
| Convert the bundled field guide and open its fidelity report. | 10 |
| Runs on your computer. | 4 |
| Document data stays local. | 4 |
| Stop automated checks when reports find selected risks. | 8 |
| Review markers point to document sections that need checking. | 9 |
| Sample fidelity report · North Ridge guide | 6 |
| Review the sample fidelity report | 5 |
| The converter produces Markdown and a fidelity report for each section that needs checking. | 14 |
| Blocked | 1 |
| 9 findings | 2 |
| word/document.xml | 1 |
| Reference | 1 |
| Finding | 1 |
| Source | 1 |
| Action | 1 |
| Tracked edit | 2 |
| Paragraph 4 | 2 |
| Confirm accepted text | 3 |
| 3×2 table | 2 |
| Table 1 | 2 |
| Check merged cells | 3 |
| Image extracted | 2 |
| Paragraph 11 | 2 |
| Add alt text | 3 |
| Embedded file | 2 |
| Package part | 2 |
| Inspect separately | 2 |
| Three steps | 2 |
| How conversion works | 3 |
| Choose a file or folder | 5 |
| Choose one DOCX or a directory. | 6 |
| One command converts every DOCX in that directory. | 8 |
| Get conversion files | 3 |
| Get Markdown, extracted images, a JSON report, and a human checklist. | 11 |
| Review exact locations | 3 |
| Open each paragraph, table, row, or package part named in the report. | 12 |
| Single Rust binary | 3 |
| Run the bundled CLI demo | 5 |
| Build locally, then convert the bundled field guide in a fresh temporary directory. | 13 |
| Copy commands | 2 |
| Converted: field-guide.docx → field-guide.md | 3 |
| Report: blocked · 9 findings | 4 |
| Sandbox: /tmp/docx-fidelity-demo-… | 2 |
| Limits | 1 |
| What this CLI does not change | 6 |
| No OCR | 2 |
| No document editing | 3 |
| No PDF round-trip | 3 |
| No macro execution | 3 |
| No document upload | 3 |
| The CLI rejects ZIP or XML parts larger than 32 MiB. | 11 |
| Embedded files are reported, never opened. | 6 |
| Included workflow | 2 |
| Stop automated checks at selected risk levels | 7 |
| Use `--fail-on warning\|error` to stop an automated check at the selected risk level. | 13 |
| Automated checks run locally with each conversion. | 7 |
| Install the CLI | 3 |
| Convert DOCX and list review issues. | 6 |
| Terms | 1 |
| Built by Param Factory (external) | 5 |
| v0.1.4 · build 2026.08.29 | 3 |

Landing command examples: `docx-fidelity convert word-files/ --output migration/`, `cargo install --path .`, `docx-fidelity demo`, and `docx-fidelity convert docs/ --output out/ --fail-on warning`.

### README

| Copy | Words |
| --- | ---: |
| Docx Markdown Fidelity Report | 4 |
| Convert DOCX to Markdown and list every item that needs review. | 11 |
| This local CLI is for teams moving Word-heavy documentation. | 9 |
| It creates Markdown, extracts images, and writes a fidelity report beside each result. | 13 |
| The report names the file part and paragraph to check. | 10 |
| It never executes macros or embedded objects. | 7 |
| Try the bundled sample | 4 |
| The command uses a fresh temporary directory and prints its path. | 11 |
| The same flow is recorded at `https://docx-markdown-fidelity-report.sociobot.in/?demo=1`. | 7 |
| Install | 1 |
| Build the single binary with stable Rust 1.88 or newer. | 10 |
| Usage | 1 |
| Convert one file | 3 |
| Convert every `.docx` file in a directory | 7 |
| Write a machine-readable command summary to stdout | 7 |
| Each input produces: | 3 |
| converted headings, paragraphs, lists, links, tables, notes, and image references. | 10 |
| structured findings with category, severity, source part, paragraph, table, row, and cell. | 12 |
| a review checklist for people. | 5 |
| safely named extracted images, when the DOCX contains images. | 9 |
| The fidelity report covers tables, comments, tracked revisions, embedded objects, footnotes, non-default styles, and every image extraction outcome. | 18 |
| Unknown or lossy constructs stay visible as findings. | 8 |
| Plain Word text that looks like Markdown stays literal in the output. | 12 |
| Exit codes | 2 |
| conversion completed. | 2 |
| Findings may still need review. | 5 |
| bad arguments, unsafe input, or a read/write failure. | 8 |
| an automated check reached the selected risk level. | 8 |
| Stop automated checks at selected risk levels | 7 |
| Continuous integration (CI) runs automated checks when a team changes files. | 11 |
| Use `--fail-on warning\|error` to stop a check at the selected risk level. | 12 |
| The conversion, fidelity report, and check all run locally. | 9 |
| Scope and safety | 3 |
| The converter rejects ZIP or XML parts larger than 32 MiB and rejects unsafe archive paths. | 16 |
| It reports, but never extracts or runs, macros and embedded files. | 11 |
| It leaves source DOCX files unchanged and accepts DOCX rather than PDF or image input. | 15 |
| Develop and verify | 3 |
| `npm test` runs Rust unit and integration tests plus browser claim tests. | 12 |
| `npm run build` produces the release binary and the static site at `dist/site/`. | 13 |
| `npm run package` creates the publishable crate without sending it to a registry. | 13 |
| Deploy | 1 |
| The factory deploys the static site from `dist/site/`. | 8 |
| Build it with `npm run build` before the factory deployment step. | 11 |
| Repository map | 2 |
| Rust CLI and conversion library. | 5 |
| bundled sample DOCX and expected demo behavior. | 7 |
| Vite landing, docs, demo, privacy, terms, and 404 routes. | 9 |
| brief, design system, demo contract, claims, audits, and handoff. | 9 |
| Privacy | 1 |
| There is no telemetry. | 4 |
| See the privacy page and terms. | 6 |
| Source code is MIT licensed. | 5 |

README command examples are the documented `cargo`, `docx-fidelity`, and `npm` invocations; each result-bearing command was exercised during this review. Development and deployment instructions describe repository operation rather than adding unsupported product behavior.

### Terminology

| Concept | Term used consistently |
| --- | --- |
| Microsoft Word package | DOCX |
| Converted text file | Markdown |
| Output that lists issues | fidelity report |
| One report entry | finding |
| Human decision needed | review |
| Bundled isolated try-out | demo |
| Build threshold | automated check |
| Original place in the DOCX | source location |

## Demo and sandbox

- The primary first-screen action opens `/?demo=1` in one click.
- The first 390 px demo screen shows the persistent banner, sample DOCX heading, privacy explanation, and the running terminal recording. The completed recording names `field-guide.docx`, Markdown and report outputs, nine findings, seven issue categories, source locations, and the temporary sandbox.
- With `real:review3=keep`, `demo:old=discard`, and `demo:changed=yes` seeded, **Reset demo** removed only `demo:` state and re-seeded `demo:sample`. **Start for real** cleared all `demo:` keys, preserved `real:review3`, and returned home.
- The live landing/demo request log contained only the product origin. No analytics, third-party script, upload, provider request, or embedded key was observed.
- From an unrelated temporary working directory, `dist/bin/docx-fidelity --json demo` wrote only to `/tmp/docx-fidelity-demo-9603-1788025986/`: the sample DOCX, Markdown, JSON report, checklist, and extracted image. The invoking directory stayed empty.

## Claims

`.factory/claims.json` contains 13 entries, and each ID occurs exactly once as `@claim:<id>` in `tests/claims.spec.ts`. Every exact manifest command passed independently after `npm ci` in the clean clone `/tmp/docx-review3-clean-YPYUTG/repo`.

| Claim ID | Result |
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

The landing and README product promises map to these entries: conversion/output to `demo-conversion`, `fidelity-report`, `source-fidelity`, and `review-checklist`; directory conversion to `batch-conversion`; local/privacy statements to `local-processing` and `demo-isolation`; check thresholds to `ci-policy`; limits to `safe-input` and `scope-boundaries`; installation to `single-binary` and `rust-toolchain`; licensing to `mit-license`. No unlisted product claim remains.

The full clean-clone gate also passed: 9 Rust unit tests, 1 doctest, and 31 Playwright tests. `npm run build`, `npm run lint`, and `npm run package` passed. The built site JavaScript is 5.33 kB gzip.

## Earlier findings

Every finding in `review-1.md` and `review-2.md`, plus the related `polish-1.md`, `polish-2.md`, and prior handoff assertions, was checked again on the live site and in the current code.

| Earlier ID | Result | Direct verification |
| --- | --- | --- |
| F-1-1 | Fixed | The source-of-truth brief formally specifies `free-open-source`; live copy has no checkout or unavailable paid promise; MIT license tests pass. |
| F-1-2 | Fixed | Wordmark, footer, route/social titles, README, package, and code use **Docx Markdown Fidelity Report**. |
| F-1-3 | Fixed | Live headings are literal: “Convert DOCX and list review issues.”, “How conversion works”, “Run the bundled CLI demo”, “What this CLI does not change”, and “Page not found.” |
| F-1-4 | Fixed | Landing, demo, README, manifest, and code consistently use **fidelity report** and explain the source locations it contains. |
| F-2-1 | Fixed | Landing copy contains no “CI” or “CI policy”; it says “automated checks”. README defines “Continuous integration (CI)” before using the abbreviation. |

No finding is half-fixed or regressed.

## Structure, accessibility, and visual identity

- `/`, `/?demo=1`, `/demo`, `/privacy`, and `/terms` return 200. `/not-a-route` returns HTTP 404 with the designed “Page not found.” route and a working home action.
- Each checked route has `lang="en"`, one `<main>`, one `<h1>`, a route-specific title, description, canonical, Open Graph/Twitter metadata, favicon, consistent header/footer, and Privacy/Terms links.
- The landing title is `Docx Markdown Fidelity Report — review DOCX` (53 characters). Other routes use `Demo/Privacy/Terms — Docx Markdown Fidelity Report` and the 404 uses `Page not found — Docx Markdown Fidelity Report`.
- All crawled HTTP links return 200; hash and `mailto:` links are intentional. Valid routes produced no console errors. The 404 produces only the expected failed-document message for its HTTP 404 response.
- Client navigation uses real URLs. Privacy navigation focused its H1; browser Back returned home and focused the landing H1. Deep links and reloads preserve their routes.
- The live CSP and other security policies arrive as response headers. The CSP limits scripts, styles, fonts, images, and connections to self, and sets `frame-ancestors 'none'` as a header.
- The full Playwright suite found no serious or critical axe violations on the landing, demo, privacy, terms, or 404 routes. Its 390 px checks passed keyboard activation, visible route focus, 44 px targets, no horizontal overflow, and 200% text resizing. Reduced-motion rules disable drawing and transitions.
- The warm survey-paper palette, self-hosted Fraunces/Atkinson type, original contour illustration, map-sheet cuts, source-location table, and vermilion markers implement `.factory/design.md`. The result is product-specific and not a generic SaaS card/gradient layout.

## Missed leverage

No finding. The brief calls for local DOCX input and Markdown, image, JSON report, and checklist outputs; all are present. An AI step would add document disclosure and network dependence without being implied by the job. Hosted sync would conflict with the local-first boundary. No decorative AI feature, provider credential, or direct model-provider request exists.

## What would make this perfect

Nothing remains to change for the reviewed scope. Preserve the current one-click isolated demo, literal terminology, claim tests, route behavior, and local-only conversion guarantees in future releases.
