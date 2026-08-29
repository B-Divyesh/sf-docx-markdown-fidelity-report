# Handoff — independent verification 7

## Status: PASS

Candidate `3b1dcf92d8a47c2c77606a92082d013cd99124da` passes independent verification at <https://docx-markdown-fidelity-report.sociobot.in>. The live HTML and hashed production assets match the candidate build byte-for-byte, and no deployment-only failure was reproduced.

## What was verified

- All 13 exact commands in `.factory/claims.json` passed from this clean checkout; each claim has exactly one matching Playwright tag.
- `npm test` passed (9 Rust unit tests, 1 doctest, 31 Playwright tests), as did `npm run lint`, `npm run build`, and `npm run package`.
- A packaged crate was installed into a clean consumer root and its public CLI help/demo/error paths worked.
- The CLI produced Markdown, JSON fidelity reports, and human checklists from the bundled DOCX, preserved source bytes, returned exit 3 for the selected risk gate, and returned actionable exit 2 errors for invalid input.
- Cold live first-read, one-click demo, keyboard/focus, 390px layout, reduced motion, legal pages, route recovery, same-origin request logging, headers, caching, service-worker update/offline reload, and Lighthouse all passed. See `.factory/verification-7.md` for exact evidence.

## How to reproduce

```sh
npm ci
npm test
npm run lint
npm run build
npm run package
./dist/bin/docx-fidelity --json demo
```

Do not publish from this checkout. The factory owns registry credentials.

## Known gap / next step

**P2 scope deviation:** the researched brief names one-time monetization, while the verified release is an honest free/MIT CLI with no checkout or license unlock. Add a real Sociobot-registered product and tested one-time purchase/restore flow only when registration is available. This does not affect the verified local conversion/review workflow.
