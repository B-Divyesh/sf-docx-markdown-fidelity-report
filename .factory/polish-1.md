# Polish 1 — review finding closure

**Repair commit:** `5de8753f30bfa01b322876becc9165efaacc14c0`  
**Deployment:** `167aa991-a3ec-4f3a-b3a4-16dc8e317202`  
**Live URL:** <https://docx-markdown-fidelity-report.sociobot.in>

| Finding | Change made | Evidence |
| --- | --- | --- |
| F-1-1 | Formally revised the researched scope from one-time to `free-open-source` in `.factory/brief.json`. The prior factory checkout registration still returns its documented 404, so the release now honestly ships the complete free MIT CLI instead of an unavailable purchase path. | `@regression:free-scope`; clean-clone `npm test`; checkout probe recorded before repair: `GET /api/v1/products/docx-markdown-fidelity-report/checkout` → 404 `enabled factory product`; live landing has no dead checkout link. |
| F-1-2 | Replaced “Docx Fidelity” everywhere visitor-facing with “Docx Markdown Fidelity Report”; added route-specific full-name titles, social metadata, header, and footer verification. | `@regression:review-copy`; live `/` title is `Docx Markdown Fidelity Report — review DOCX`; live `/?demo=1` title is `Demo — Docx Markdown Fidelity Report`. |
| F-1-3 | Rewrote all cited metaphor/vague headings in literal language while retaining the cartographic illustration and clipped survey-sheet interface: “Convert DOCX and list review issues”, “Review the sample fidelity report”, “How conversion works”, “Run the bundled CLI demo”, “What this CLI does not change”, and “Page not found.” | `@regression:review-copy`; screenshots `/tmp/docx-fidelity-verify-9tlUBf/screenshot-mobile.png`; cold live checks of `/`, `/?demo=1`, and `/not-a-route`. |
| F-1-4 | Standardized the output name as **fidelity report** across the landing, demo, README, claims, and terminology audit. First-use copy now explains that it identifies the document section to check. | `@claim:fidelity-report`; `@regression:review-copy`; live `/?demo=1` H1 “See the sample DOCX conversion report.” |
| Work-order demo requirement | Added a one-click `/?demo=1` entry in addition to `/demo`. It seeds only `demo:` storage, shows the persistent banner, reloads the sample on reset, and clears all `demo:` keys when starting for real. | `@claim:demo-isolation`; live cold browser evidence: banner visible, reset kept `real:review`, and Start for real removed `demo:` state. |
| Work-order routing/metadata/mobile requirement | Kept `/demo`, `/privacy`, `/terms`, designed HTTP 404, focus-on-navigation, canonical/social metadata, and route titles; repaired long demo-heading wrapping at 200% text size. | `npm test` (31 tests); `@mobile @regression:text-resize`; live route statuses: `/`, `/?demo=1`, `/demo`, `/privacy`, `/terms` = 200; `/not-a-route` = 404. |

## Claim evidence

From clean clone `/tmp/docx-fidelity-clean-n1H2B6` after `npm ci`, every manifest command passed independently: `demo-conversion`, `local-processing`, `fidelity-report`, `source-fidelity`, `batch-conversion`, `ci-policy`, `safe-input`, `demo-isolation`, `review-checklist`, `single-binary`, `rust-toolchain`, `scope-boundaries`, and `mit-license`. Logs: `/tmp/claim-*.log`.

## Live cold check

Fresh Chromium context at the live URL verified the query demo title/H1/canonical/banner, demo-only reset and leave behavior, all-same-origin demo requests, and the real HTTP 404 title/H1. Valid routes logged no console errors; the only two console entries were Chromium's expected failed-resource messages for the deliberately loaded 404.
