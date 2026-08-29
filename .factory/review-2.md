# Adversarial first-read review 2 — FAIL

**Product:** Docx Markdown Fidelity Report  
**Live URL:** <https://docx-markdown-fidelity-report.sociobot.in>  
**Reviewed:** 2026-08-29 UTC  
**Method:** fresh Chromium contexts at 390 × 844 and 1440 × 900; fresh clean clone at `bd6292b8deaf2d091eb978c0844ddb32e85c1ccf`.

## Verdict: FAIL

One P2 wording finding remains. The sample path, claims, privacy, routes, and earlier-review repairs all verify, but the landing page still uses unexplained CI jargon for an audience described as teams moving Word documentation. PASS requires zero findings.

## Cold first read

Before scrolling, on mobile and desktop, I understood:

- **What it does:** converts a DOCX and lists the places that need review before using the Markdown.
- **For whom:** teams moving Word documentation.
- **What to click first:** **Try it with sample data** to convert the bundled field guide and open its fidelity report.

The first-screen evidence was: “Convert DOCX and list review issues.”, “For teams moving Word documentation, it shows where Markdown needs a human check.”, and “Try it with sample data”. This gate passes. At 390 px, the action, its result text, and all three facts are visible without scrolling.

## Findings

### F-2-1 — P2 — “CI policy” is unexplained jargon in visible landing copy

**Location / exact quote:** The first-screen fact says “CI policy checks are included.” The later section is headed “Stop CI on review issues” and says “Use `--fail-on warning|error` to fail a migration check at the risk level your team chooses.”

**Why this fails:** The stated audience is teams moving Word-heavy documentation, not only software engineers. A first-time visitor who does not already know “CI” cannot use the third first-screen fact or understand the later benefit. “Policy” also does not say what will happen. This violates the required plain-language copy audit even though the underlying, tested feature works.

**Concrete fix:** Replace the fact with “Stop automated checks when reports find selected risks.” Replace the heading with “Stop automated checks at selected risk levels.” In the section body, define the abbreviation once if it must remain: “Use `--fail-on warning|error` to stop a continuous-integration (CI) check at the selected risk level.” Add a copy regression assertion for the expanded first use.

## Copy audit

Counts treat a hyphenated term, command option, path, number, and product name as one word. This lists all visible landing prose, headings, labels, table cells, and buttons; command examples are excluded because they are commands rather than sentences. README commands are likewise excluded. No audited item exceeds 22 words. Required task terms such as DOCX and Markdown are not flagged; F-2-1 flags the only audience-mismatched unexplained workflow jargon.

### Landing page

| Copy | Words | Result |
| --- | ---: | --- |
| Docx Markdown Fidelity Report | 4 | Pass |
| Demo | 1 | Pass |
| Install | 1 | Pass |
| Privacy | 1 | Pass |
| Local DOCX conversion · v0.1 | 5 | Pass |
| Convert DOCX and list review issues. | 6 | Pass |
| For teams moving Word documentation, it shows where Markdown needs a human check. | 13 | Pass |
| Try it with sample data | 5 | Pass |
| Convert the bundled field guide and open its fidelity report. | 10 | Pass |
| Runs on your computer. | 4 | Pass |
| Document data stays local. | 4 | Pass |
| CI policy checks are included. | 5 | Flag F-2-1 |
| Review markers point to document sections that need checking. | 9 | Pass |
| Sample fidelity report · North Ridge guide | 6 | Pass |
| Review the sample fidelity report | 5 | Pass |
| The converter produces Markdown and a fidelity report for each section that needs checking. | 14 | Pass |
| Blocked | 1 | Pass |
| 9 findings | 2 | Pass |
| word/document.xml | 1 | Pass |
| Reference | 1 | Pass |
| Finding | 1 | Pass |
| Source | 1 | Pass |
| Action | 1 | Pass |
| 01 / Tracked edit / Paragraph 4 / Confirm accepted text | 8 | Pass |
| 02 / 3×2 table / Table 1 / Check merged cells | 9 | Pass |
| 03 / Image extracted / Paragraph 11 / Add alt text | 8 | Pass |
| 04 / Embedded file / Package part / Inspect separately | 7 | Pass |
| Three steps | 2 | Pass |
| How conversion works | 3 | Pass |
| Choose a file or folder | 5 | Pass |
| Choose one DOCX or a directory. | 6 | Pass |
| One command converts every DOCX in that directory. | 8 | Pass |
| Get conversion files | 3 | Pass |
| Get Markdown, extracted images, a JSON report, and a human checklist. | 11 | Pass |
| Review exact locations | 3 | Pass |
| Open each paragraph, table, row, or package part named in the report. | 12 | Pass |
| Single Rust binary | 3 | Pass |
| Run the bundled CLI demo | 5 | Pass |
| Build locally, then convert the bundled field guide in a fresh temporary directory. | 13 | Pass |
| Copy commands | 2 | Pass |
| Converted: field-guide.docx → field-guide.md | 3 | Pass |
| Report: blocked · 9 findings | 4 | Pass |
| Sandbox: /tmp/docx-fidelity-demo-… | 2 | Pass |
| Limits | 1 | Pass |
| What this CLI does not change | 6 | Pass |
| No OCR | 2 | Pass |
| No document editing | 3 | Pass |
| No PDF round-trip | 3 | Pass |
| No macro execution | 3 | Pass |
| No document upload | 3 | Pass |
| The CLI rejects ZIP or XML parts larger than 32 MiB. | 10 | Pass |
| Embedded files are reported, never opened. | 6 | Pass |
| Included workflow | 2 | Pass |
| Stop CI on review issues | 5 | Flag F-2-1 |
| Use `--fail-on warning\|error` to fail a migration check at the risk level your team chooses. | 16 | Flag F-2-1 |
| Policy checks run locally with the conversion. | 7 | Flag F-2-1 |
| Install the CLI | 3 | Pass |
| Convert DOCX and list review issues. | 6 | Pass |
| Terms | 1 | Pass |
| Built by Param Factory (external) | 5 | Pass |
| v0.1.3 · build 2026.08.29 | 4 | Pass |

### README

| Copy | Words | Result |
| --- | ---: | --- |
| Docx Markdown Fidelity Report | 4 | Pass |
| Convert DOCX to Markdown and list every item that needs review. | 10 | Pass |
| This local CLI is for teams moving Word-heavy documentation. | 9 | Pass |
| It creates Markdown, extracts images, and writes a fidelity report beside each result. | 13 | Pass |
| The report names the file part and paragraph to check. | 10 | Pass |
| It never executes macros or embedded objects. | 7 | Pass |
| Try the bundled sample | 4 | Pass |
| The command uses a fresh temporary directory and prints its path. | 10 | Pass |
| The same flow is recorded at the demo URL. | 9 | Pass |
| Install | 1 | Pass |
| Build the single binary with stable Rust 1.88 or newer. | 10 | Pass |
| Usage | 1 | Pass |
| Convert one file | 3 | Pass |
| Convert every `.docx` file in a directory | 7 | Pass |
| Write a machine-readable command summary to stdout | 8 | Pass |
| Each input produces: | 3 | Pass |
| converted headings, paragraphs, lists, links, tables, notes, and image references. | 9 | Pass |
| structured findings with category, severity, source part, paragraph, table, row, and cell. | 11 | Pass |
| a review checklist for people. | 5 | Pass |
| safely named extracted images, when the DOCX contains images. | 8 | Pass |
| The fidelity report covers tables, comments, tracked revisions, embedded objects, footnotes, non-default styles, and every image extraction outcome. | 17 | Pass |
| Unknown or lossy constructs stay visible as findings. | 8 | Pass |
| Plain Word text that looks like Markdown stays literal in the output. | 11 | Pass |
| Exit codes | 2 | Pass |
| conversion completed. | 2 | Pass |
| Findings may still need review. | 5 | Pass |
| bad arguments, unsafe input, or a read/write failure. | 8 | Pass |
| a CI policy threshold was met. | 6 | Flag F-2-1 |
| CI policy gates | 3 | Flag F-2-1 |
| All conversion, fidelity reports, and CI policy checks run locally. | 9 | Flag F-2-1 |
| Use `--fail-on warning\|error` to make CI stop at the selected risk level. | 12 | Flag F-2-1 |
| Scope and safety | 3 | Pass |
| The converter rejects ZIP or XML parts larger than 32 MiB and rejects unsafe archive paths. | 15 | Pass |
| It reports, but never extracts or runs, macros and embedded files. | 11 | Pass |
| It leaves source DOCX files unchanged and accepts DOCX rather than PDF or image input. | 15 | Pass |
| Develop and verify | 3 | Pass |
| `npm test` runs Rust unit and integration tests plus browser claim tests. | 11 | Pass |
| `npm run build` produces the release binary and the static site at `dist/site/`. | 12 | Pass |
| `npm run package` creates the publishable crate without sending it to a registry. | 13 | Pass |
| Deploy | 1 | Pass |
| The factory deploys the static site from `dist/site/`. | 8 | Pass |
| Build it with `npm run build` before the factory deployment step. | 11 | Pass |
| Repository map | 2 | Pass |
| bundled sample DOCX and expected demo behavior. | 7 | Pass |
| Vite landing, docs, demo, privacy, terms, and 404 routes. | 9 | Pass |
| brief, design system, demo contract, claims, audits, and handoff. | 9 | Pass |
| Privacy | 1 | Pass |
| There is no telemetry. | 4 | Pass |
| See the privacy page and terms. | 6 | Pass |
| Source code is MIT licensed. | 5 | Pass |

## Demo, sandbox, claims, and privacy

- **One-click demo:** Pass. `/?demo=1` is the landing action and its initial 390 px screen already shows the sample DOCX conversion recording, realistic source locations, Markdown/report filenames, nine findings, and seven issue categories.
- **Demo sandbox:** Pass. The persistent banner reads “Demo — sample data, nothing is saved to your files.” After seeding `real:review` and `demo:old`, Reset removed the old demo key and re-seeded only `demo:sample`; Start for real removed demo keys and preserved `real:review`.
- **CLI sandbox:** Pass. From an unrelated fresh temporary directory, `target/debug/docx-fidelity --json demo` wrote the source, Markdown, JSON report, and checklist under a fresh `/tmp/docx-fidelity-demo-*` path; no output appeared in the invoking directory.
- **Privacy:** Pass. Request logs for fresh landing and demo contexts contain only the product origin (document, local JS/CSS, fonts, and art). There are no uploads, analytics, third-party scripts, or provider-key requests.
- **Claims:** Pass. Every one of the 13 exact commands in `.factory/claims.json` passed independently in clean clone `/tmp/docx-fidelity-review2-N11crc`: `demo-conversion`, `local-processing`, `fidelity-report`, `source-fidelity`, `batch-conversion`, `ci-policy`, `safe-input`, `demo-isolation`, `review-checklist`, `single-binary`, `rust-toolchain`, `scope-boundaries`, and `mit-license`. Logs are in `/tmp/review2-claim-logs/`.
- **Unlisted claims:** None found. The landing and README’s observable behavior statements map to the manifest entries; non-claim instructional copy was not treated as a product promise.

## Earlier-review verification

All four findings from `review-1.md` are actually fixed in the live deployment and code:

| Earlier finding | Verification |
| --- | --- |
| F-1-1 | The current brief formally and truthfully specifies `free-open-source`; the live site has no unavailable paid promise or checkout link, and README/terms state MIT scope. |
| F-1-2 | Live wordmark, footer, titles, canonical social titles, README, and code use **Docx Markdown Fidelity Report**. |
| F-1-3 | Former map/mood headings are now literal: “Convert DOCX and list review issues.”, “How conversion works”, “Run the bundled CLI demo”, “What this CLI does not change”, and “Page not found.” |
| F-1-4 | The live hero, sample, demo, README, claim manifest, and code consistently call the output a **fidelity report**; first-use copy says it names the document section to check. |

## Structure, accessibility, and visual review

- `/`, `/?demo=1`, `/demo`, `/privacy`, and `/terms` return 200. `/not-a-route` returns a designed HTTP 404. Each checked route has one `<main>`, one `<h1>`, a route-specific title, description, canonical, and Open Graph title.
- All live navigation links returned 200 or were intentional same-page anchors/mail links. Header/footer are consistent and include Privacy and Terms. `robots.txt`, `sitemap.xml`, SVG favicon, apple touch icon, canonical URLs, OG/Twitter card, and security headers are present.
- A fresh live console had no errors. Browser axe checks at 390 px found no WCAG 2 A/AA violations on landing, demo, privacy, terms, or 404. The live CSP contains header-delivered `frame-ancestors 'none'` and permits only self assets.
- The warm paper, original contour-map art, self-hosted Fraunces/Atkinson pairing, clipped map-sheet panels, and vermilion source pins match `.factory/design.md` and are distinct from a generic SaaS template.
- The brief does not imply an AI, sync, or additional import/export step beyond the present DOCX input plus Markdown/JSON/checklist outputs. No decorative AI feature or embedded provider key is present.

## What would make this perfect

Resolve F-2-1 by expanding or replacing the visible CI-policy language, then add the stated copy regression test. No functional, sandbox, privacy, route, claim, or visual-system change is indicated by this review.
