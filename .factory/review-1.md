# Adversarial first-read review 1 — FAIL

**Product:** Docx Markdown Fidelity Report  
**Live URL:** <https://docx-markdown-fidelity-report.sociobot.in>  
**Reviewed:** 2026-08-29 UTC  
**Viewport:** fresh Chromium contexts at 390 × 844 and 1440 × 900

## Verdict: FAIL

The core conversion demo is clear and works. However, the brief's one-time monetization is still absent, despite being a carried-forward known gap, and the product has several plain-language and identity defects. This review can pass only with zero findings.

## Cold first read

Before scrolling, at both tested sizes, I understood:

- **What it does:** converts a DOCX to Markdown and identifies conversion risks that need review.
- **Who it is for:** teams moving Word documentation.
- **What to click first:** **Try it with sample data** to see a complex file convert and inspect its ledger.

The relevant first-screen text is: “Convert DOCX. Map every review risk.”, “For teams moving Word documentation, it shows where Markdown needs a human check.”, and “Try it with sample data”. This gate passes. The 390 px screen has the primary action, its result text, and all three facts above the fold.

## Findings

### F-1-1 — BLOCKING — The brief's one-time product is not available

**Location / evidence:** `.factory/brief.json` sets `"monetization": "one-time"`. The live landing has no price, purchase action, license activation, or description of a paid tier. Its former paid-policy area instead says “CI included”. `.factory/handoff.md` and `.factory/verification-5.md` both record the same unresolved one-time-purchase gap.

**Why this fails:** This is an earlier unresolved finding, so the history rule requires it to return as a blocker. A visitor cannot obtain the stated product model or tell what a one-time purchase includes. Removing the broken checkout avoided a false promise, but did not implement the brief.

**Concrete fix:** Register the product with the Sociobot billing API, add a truthful paid-tier section with exact one-time price and unlocked capability, and provide a tested purchase, return, and license-verification flow. If registration remains unavailable, formally revise the researched brief and product scope rather than leaving the monetization contract unmet.

### F-1-2 — P2 — The product has two names

**Location / quote:** The required product name is **“Docx Markdown Fidelity Report”** in the brief, repository, and README. The live wordmark, titles, canonical social title, and footer call it **“Docx Fidelity”**; for example, `<title>Docx Fidelity — convert DOCX and map review risks</title>`.

**Why this fails:** A first-time visitor cannot reliably connect the site to the named product or repository. It also violates the required title pattern because the title does not use the product name supplied by the brief.

**Concrete fix:** Use one name everywhere. For example, use **Docx Markdown Fidelity Report** for the wordmark and `Docx Markdown Fidelity Report — find conversion risks` for the landing title; update route titles, Open Graph/Twitter titles, README heading, footer, and package-facing documentation together.

### F-1-3 — P2 — Landing headings use map metaphors or vague slogans instead of section names

**Location / quotes:**

- Landing H1: “Convert DOCX. **Map** every review risk.”
- Sample-report H2: “**See the handoff before you publish**”.
- Eyebrow: “**Three field steps**”.
- Step H3s: “**Point at files**” and “**Keep the useful parts**”.
- Install H2: “**Run the real demo**”.
- Boundaries H2: “**Know what stays untouched**”.
- Footer: “Convert DOCX and **map** every review risk.”
- Demo H1: “Watch one DOCX **map** its risks.”
- 404 H1: “This path is outside the survey.”

**Why this fails:** “Map”, “field”, “handoff”, and “survey” are visual-theme language, not the job or section subject. “Useful” is subjective. Several headings make sense only after learning the product's map metaphor, contrary to the plain-words rule for a mobile visitor and screen-reader heading list.

**Concrete fix:** Keep the cartographic art but use literal headings. Suggested replacements are:

| Current copy | Replacement |
| --- | --- |
| Convert DOCX. Map every review risk. | Convert DOCX and list review issues. |
| See the handoff before you publish | Review the sample fidelity report |
| Three field steps | How conversion works |
| Point at files | Choose a file or folder |
| Keep the useful parts | Get Markdown, images, and review files |
| Run the real demo | Run the bundled CLI demo |
| Know what stays untouched | What this CLI does not change |
| Watch one DOCX map its risks. | See the sample DOCX conversion report. |
| This path is outside the survey. | Page not found. |

### F-1-4 — P2 — One output is called a “risk ledger”, “fidelity ledger”, and “report”

**Location / quotes:** The hero result says “inspect its **risk ledger**”; the sample label says “Sample **report**”; the demo says “Generated **fidelity ledger**”; README says “source-located **fidelity ledger**”; the product title says “Fidelity **Report**”. “Source-located” also appears repeatedly without a plain-language definition on the first screen.

**Why this fails:** The same output has three names, which makes the primary result sound more specialised than it is. “Source-located” is compact internal jargon for an audience that may be migrating documentation rather than writing converters.

**Concrete fix:** Choose **fidelity report** throughout. On first use, write “a fidelity report that names the file and paragraph to check”; thereafter use “report”. For example, change “inspect its risk ledger” to “inspect its fidelity report”.

## Copy audit

Counts treat hyphenated terms, paths, and commands as one word. Labels and headings are included because they are audible interface copy. No audited sentence exceeds 22 words. Flags above identify the metaphors, vague adjectives, and inconsistent terms; necessary technical nouns such as DOCX, Markdown, JSON, and CI are not flagged merely for being technical.

### Landing page

| Copy | Words | Result |
| --- | ---: | --- |
| Docx Fidelity | 2 | Flag F-1-2 |
| Demo | 1 | Pass |
| Install | 1 | Pass |
| Privacy | 1 | Pass |
| Local conversion field report · v0.1 | 5 | Flag F-1-3: theme label, not useful section information |
| Convert DOCX. | 2 | Flag F-1-3 when paired with the next sentence |
| Map every review risk. | 4 | Flag F-1-3: metaphor |
| For teams moving Word documentation, it shows where Markdown needs a human check. | 13 | Pass |
| Try it with sample data | 5 | Pass |
| See a complex file convert, then inspect its risk ledger. | 10 | Flag F-1-4 |
| Runs on your computer. | 4 | Pass |
| Document data stays local. | 4 | Pass |
| CI policy gates are included. | 5 | Pass; covered by the CI claim |
| Every pin becomes a source-located finding. | 7 | Flag F-1-4: jargon/metaphor |
| Sample report · North Ridge guide | 5 | Flag F-1-4: output name differs |
| See the handoff before you publish | 7 | Flag F-1-3 |
| The converter keeps useful Markdown and names each place that lost meaning. | 11 | Flag F-1-3: “useful” is vague |
| Blocked | 1 | Pass |
| 9 findings | 2 | Pass |
| Map ref | 2 | Flag F-1-3: map metaphor |
| Finding | 1 | Pass |
| Source | 1 | Pass |
| Action | 1 | Pass |
| Tracked edit | 2 | Pass |
| Confirm accepted text | 3 | Pass |
| 3×2 table | 1 | Pass |
| Check merged cells | 3 | Pass |
| Image extracted | 2 | Pass |
| Add useful alt text | 4 | Pass |
| Embedded file | 2 | Pass |
| Inspect separately | 2 | Pass |
| Three field steps | 3 | Flag F-1-3 |
| Convert a whole document set | 5 | Pass |
| Point at files | 3 | Flag F-1-3 |
| Choose one DOCX or a directory. | 6 | Pass |
| One command converts every DOCX in that directory. | 8 | Pass |
| Keep the useful parts | 4 | Flag F-1-3 |
| Get Markdown, extracted images, a JSON ledger, and a human checklist. | 11 | Flag F-1-4: ledger term |
| Review exact locations | 3 | Pass |
| Open each paragraph, table, row, or package part named in the report. | 12 | Pass |
| Single Rust binary | 3 | Pass |
| Run the real demo | 4 | Flag F-1-3 |
| Build locally, then convert the bundled field guide in a fresh temporary directory. | 13 | Pass |
| Copy commands | 2 | Pass |
| Converted: field-guide.docx → field-guide.md | 3 | Pass |
| Report: blocked · 9 findings | 4 | Pass |
| Sandbox: /tmp/docx-fidelity-demo-… | 2 | Pass |
| Clear boundaries | 2 | Flag F-1-3: vague heading |
| Know what stays untouched | 4 | Flag F-1-3 |
| No OCR | 2 | Pass |
| No document editing | 3 | Pass |
| No PDF round-trip | 3 | Pass |
| No macro execution | 3 | Pass |
| No document upload | 3 | Pass |
| The CLI rejects ZIP or XML parts larger than 32 MiB. | 10 | Pass |
| Embedded files are reported, never opened. | 6 | Pass |
| CI included | 2 | Pass |
| Policy gates | 2 | Pass |
| Stop CI on review risks | 5 | Pass |
| Use `--fail-on warning\|error` to fail a migration check at the risk level your team chooses. | 15 | Pass |
| Policy gates run locally with the conversion. | 7 | Pass |
| No account or network connection is needed. | 7 | Pass |
| Install the CLI | 3 | Pass |
| Convert DOCX and map every review risk. | 7 | Flag F-1-3 |
| Terms | 1 | Pass |
| Built by Param Factory | 4 | Pass |

### README

| Copy | Words | Result |
| --- | ---: | --- |
| Docx Markdown Fidelity Report | 4 | Flag F-1-2: differs from live name |
| Convert DOCX to Markdown and list every item that needs review. | 10 | Pass |
| This local CLI is for teams moving Word-heavy documentation. | 9 | Pass |
| It creates readable Markdown, extracts images, and writes a source-located fidelity ledger beside each result. | 15 | Flag F-1-4: “readable”, jargon, and ledger term |
| It never executes macros or embedded objects. | 7 | Pass |
| Try the bundled sample | 4 | Pass |
| The command uses a fresh temporary directory and prints its path. | 10 | Pass |
| The same flow is recorded at the demo URL. | 9 | Pass |
| Install | 1 | Pass |
| Build the single binary with stable Rust 1.88 or newer. | 10 | Pass |
| No account or conversion service is required. | 7 | Pass |
| Usage | 1 | Pass |
| Convert one file | 3 | Pass |
| Convert every `.docx` file in a directory | 7 | Pass |
| Write a machine-readable command summary to stdout | 8 | Pass |
| Each input produces: | 3 | Pass |
| converted headings, paragraphs, lists, links, tables, notes, and image references. | 9 | Pass |
| structured findings with category, severity, source part, paragraph, table, row, and cell. | 11 | Pass |
| a review checklist for people. | 5 | Pass |
| safely named extracted images, when the DOCX contains images. | 8 | Pass |
| The report covers tables, comments, tracked revisions, embedded objects, footnotes, non-default styles, and every image extraction outcome. | 17 | Pass |
| Unknown or lossy constructs stay visible as findings. | 8 | Pass |
| Plain Word text that looks like Markdown stays literal in the output. | 11 | Pass |
| Exit codes | 2 | Pass |
| conversion completed; findings may still need review. | 7 | Pass |
| bad arguments, unsafe input, or a read/write failure. | 8 | Pass |
| a CI policy threshold was met. | 6 | Pass |
| CI policy gates | 3 | Pass |
| All conversion, reports, and CI policy gates run locally. | 9 | Pass |
| Use `--fail-on warning\|error` to make CI stop at the selected risk level. | 12 | Pass |
| The command does not require an account or network connection. | 10 | Pass |
| Scope and safety | 3 | Pass |
| The converter rejects ZIP or XML parts larger than 32 MiB and rejects unsafe archive paths. | 15 | Pass |
| It reports, but never extracts or runs, macros and embedded files. | 11 | Pass |
| It leaves source DOCX files unchanged and accepts DOCX rather than PDF or image input. | 15 | Pass |
| Develop and verify | 3 | Pass |
| `npm test` runs Rust unit and integration tests plus browser claim tests. | 11 | Pass |
| `npm run build` produces the release binary and the static site at `dist/site/`. | 12 | Pass |
| `npm run package` creates the publishable crate without sending it to a registry. | 13 | Pass |
| Repository map | 2 | Pass |
| bundled sample DOCX and expected demo behavior. | 7 | Pass |
| Vite landing, docs, demo, privacy, terms, and 404 routes. | 9 | Pass |
| brief, design system, demo contract, claims, audits, and handoff. | 9 | Pass |
| Privacy | 1 | Pass |
| There is no telemetry. | 4 | Pass; covered by `local-processing` request-log test |
| See the privacy page and terms. | 6 | Pass |
| Source code is MIT licensed. | 5 | Pass |

## Demo, sandbox, privacy, and CLI checks

- **One-click demo:** Pass. The first action opens `/demo`; its first mobile screen has the persistent “Demo — sample data, nothing is saved” banner, Reset demo, Start for real, the bundled-field-guide heading, and the live terminal recording. The terminal identifies `field-guide.docx`, its Markdown report, nine findings, and all seven reported risk categories.
- **Isolation:** Pass. In a fresh browser context I seeded `demo:review` and `real:review`. Reset demo removed only `demo:review`; Start for real again removed only `demo:review` and navigated home.
- **Privacy:** Pass. Fresh landing and demo request logs contained only `https://docx-markdown-fidelity-report.sociobot.in` (document, local JS/CSS, self-hosted fonts, and local art). No third-party request, upload, analytics, or provider key was observed.
- **CLI sandbox:** Pass. `./dist/bin/docx-fidelity --json demo` created its source, Markdown, JSON report, and checklist under a new `/tmp/docx-fidelity-demo-*` directory, not the working directory.

## Claims gate

All 12 entries in `.factory/claims.json` have exactly one tagged test in `tests/claims.spec.ts`. From this checkout after `npm ci`, I ran every declared command independently; all passed:

| Claim ID | Result |
| --- | --- |
| demo-conversion | Pass |
| local-processing | Pass |
| risk-ledger | Pass |
| source-fidelity | Pass |
| batch-conversion | Pass |
| ci-policy | Pass |
| safe-input | Pass |
| demo-isolation | Pass |
| review-checklist | Pass |
| single-binary | Pass |
| rust-toolchain | Pass |
| scope-boundaries | Pass |

`npm test` also passed all 29 tests. `npm run build`, `npm run lint`, and `cargo +1.88.0 test --locked --lib` pass. I found no unlisted claim-like sentence on the landing or README after matching their observable claims to the manifest entries.

## Structure, accessibility, and history

- `/`, `/demo`, `/privacy`, and `/terms` return 200. An unknown route returns the designed page with HTTP 404. Each checked route has one `<main>`, one `<h1>`, a route title, description, canonical, and Open Graph title.
- All live navigational links return 200, apart from intentional `mailto:` links and the unknown-route skip-link target, which correctly inherits its 404 route status. The Param Factory footer uses the certificate-valid `https://sociobot.in/` URL.
- `/privacy` and `/terms` are present in the header/footer. Back/forward and focus-on-route-change are covered by the passing Playwright suite. The live console has no errors on valid routes; the observed 404 network message is only the deliberately loaded 404 document.
- The self-hosted type, warm-paper palette, contour art, clipped panels, and vermilion review pins match the product-specific cartography thesis rather than a generic SaaS template.
- Earlier findings verified fixed: the unavailable checkout **link** was removed; hashed assets use immutable cache policy in the deployed configuration; the invalid `www.sociobot.in` factory link is replaced; source Markdown/list fidelity, Rust 1.88 support, comprehensive claims, and mobile 200% text regressions pass. The earlier missing one-time product itself remains open as F-1-1.

## What would make this perfect

Deliver the promised one-time license through Sociobot with end-to-end test coverage, then use one product name and literal output/section vocabulary throughout. The existing demo, local-first behavior, fidelity checks, and cartographic visual system can remain unchanged.
