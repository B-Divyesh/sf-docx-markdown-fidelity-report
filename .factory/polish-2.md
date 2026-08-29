# Polish 2 — cumulative finding closure

**Product commits:** `28b411e814f354f249356038fbd4d60b69448aad`, `69cf9825d8a24a0c5a5da45d79fce897e6c87a1d`  
**Deployment:** `cda6ea00-f70f-4f9f-aae1-403ee93549b2`  
**Live URL:** <https://docx-markdown-fidelity-report.sociobot.in>

## Finding map

| Finding | Change made | Evidence |
| --- | --- | --- |
| F-1-1 | Preserved the formal `free-open-source` scope correction after the Sociobot product registration returned its documented 404. The complete CLI remains free and MIT licensed, with no unavailable checkout or paid promise. | `@regression:free-scope`; `@claim:mit-license`; clean-clone logs `/tmp/polish2-final-claim-mit-license.log`; live link crawl in `.factory/polish-2/live/live-check.json`; screenshot `.factory/polish-2/live/home/screenshot-mobile.png`; live `/` rechecked. |
| F-1-2 | Preserved **Docx Markdown Fidelity Report** as the single name in the wordmark, footer, README, package documentation, route titles, and social metadata. | `@regression:review-copy`; all route titles in `.factory/polish-2/live/live-check.json`; screenshot `.factory/polish-2/live/home/screenshot-desktop.png`; live `/`, `/?demo=1`, `/privacy`, `/terms`, and the 404 rechecked. |
| F-1-3 | Preserved literal headings and first-screen wording while retaining the topographic art: “Convert DOCX and list review issues”, “How conversion works”, “Run the bundled CLI demo”, “What this CLI does not change”, and “Page not found”. | `@regression:review-copy`; `.factory/copy-audit.md`; screenshots `.factory/polish-2/live/home/screenshot-mobile.png` and `.factory/polish-2/live/not-found-mobile.png`; live `/` and `/not-a-route` rechecked. |
| F-1-4 | Preserved **fidelity report** as the single output name. The first use explains that the report names the document section to check. | `@claim:fidelity-report`; `@regression:review-copy`; clean-clone log `/tmp/polish2-final-claim-fidelity-report.log`; screenshot `.factory/polish-2/live/demo/screenshot-mobile.png`; live `/?demo=1` rechecked. |
| F-2-1 | Replaced every visitor-facing “CI policy” phrase in Home, Privacy, Terms, and README. The first-screen fact now says “Stop automated checks when reports find selected risks.” The section explains the option and selected level; README expands “continuous integration (CI)” before using the abbreviation. Added exact site and README regression assertions. | `@regression:review-copy`; `@claim:ci-policy`; clean-clone log `/tmp/polish2-final-claim-ci-policy.log`; `.factory/copy-audit.md`; screenshot `.factory/polish-2/live/home/screenshot-mobile.png`; live `/` rechecked with retired-copy rejection in `.factory/polish-2/live-check.mjs`. |

## Required acceptance work

| Requirement | Implementation | Evidence |
| --- | --- | --- |
| One-click isolated sample | The first action opens `/?demo=1`. The persistent banner has Reset demo and Start for real. Both operations clear only `demo:` keys; real keys survive. README now points to the query entry. | `@claim:demo-isolation`; `.factory/demo.md`; `.factory/polish-2/live/demo-mobile.png`; live storage assertions in `.factory/polish-2/live/live-check.json`. |
| Every claim tested | `.factory/claims.json` has 13 entries and each ID occurs in exactly one `@claim:<id>` test. Every manifest command passed independently from the clean clone at `69cf9825d8a24a0c5a5da45d79fce897e6c87a1d`. | `/tmp/polish2-final-claim-*.log`; manifest/tag audit reported 13 claims, one tag each, and no extra tags. |
| Titles, metadata, routing, focus, 404, legal links | Real routes retain distinct titles, canonical/social metadata, one H1/main, focus-on-route-change, back navigation, Privacy/Terms links, and an HTTP 404 page. | `@regression:real-404`; `@regression:param-factory-footer-link`; accessible route tests; `.factory/polish-2/live/live-check.json`. |
| Mobile and text resize | First action and all three facts fit at 390 × 844. Controls remain at least 44 px. Every route has no horizontal overflow at 200% text. | `@mobile @regression:touch-targets`; `@mobile @regression:text-resize`; `.factory/polish-2/live/home/screenshot-mobile.png`; `.factory/polish-2/live/demo-text-200.png`. |
| Accessibility, privacy, offline | All routes have zero WCAG 2 A/AA axe violations. Demo requests are same-origin, no browser errors occur, and the shell reloads offline. | Playwright axe route suite; `@claim:local-processing`; `@offline-update`; `.factory/polish-2/live/live-check.json`; verifier reports under `.factory/polish-2/live/`. |
| Catalog copy | Updated the catalog line to “Convert DOCX to Markdown and flag every section that needs a human check.” | `.factory/catalog-description.txt`: verb-first, 73 characters. |

## Verification summary

- `npm test`: 9 Rust unit tests, 1 Rust doc test, and 31 browser tests passed.
- `npm run lint`, `npm run build`, and `npm run package`: passed; `dist/bin/docx-fidelity` and `dist/site/` were produced.
- Clean clone: `/tmp/docx-fidelity-polish2-clean-UKsoLo` at `69cf9825d8a24a0c5a5da45d79fce897e6c87a1d`; all 13 exact claim commands passed independently.
- Factory verifier: live Home and `/?demo=1` returned 200 with no console errors, one H1/main, language, alt text, and named buttons.
- Live route audit: valid routes returned 200, `/not-a-route` returned 404, all crawled links returned 200, and all six pages had zero axe violations.
- Live mobile Lighthouse on `/?demo=1`: Performance 100, Accessibility 100, Best Practices 100, SEO 100; LCP 1.1 s, CLS 0.001, total blocking time 10 ms.
- Deployment `cda6ea00-f70f-4f9f-aae1-403ee93549b2` completed successfully and the custom-domain cold check passed.

No finding from review 1 or review 2 remains open.
