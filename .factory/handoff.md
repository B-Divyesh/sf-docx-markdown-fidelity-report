# Handoff — perfection loop round 2

## Status: complete

All findings in `.factory/review-1.md` and `.factory/review-2.md` are closed. The v0.1.4 static site and Rust CLI are built, tested, pushed, deployed, and cold-checked at <https://docx-markdown-fidelity-report.sociobot.in>.

## What changed

- Replaced unexplained “CI policy” wording across Home, Privacy, Terms, README, and the claim manifest with plain automated-check language.
- Added regression assertions for the first-screen fact, plain section heading, option explanation, README expansion, retired wording, and the canonical `/?demo=1` README link.
- Updated `.factory/copy-audit.md`, `.factory/catalog-description.txt`, `CHANGELOG.md`, and v0.1.4 package metadata.
- Preserved and reverified the earlier free/MIT scope correction, one product name, literal headings, consistent fidelity-report terminology, isolated demo, route metadata, legal pages, and cartographic visual system.
- Added a repeatable cold-live audit at `.factory/polish-2/live-check.mjs` and stored mobile/desktop, text-resize, 404, URL-verifier, and Lighthouse evidence under `.factory/polish-2/`.

## Exact verification

- `npm test` passed: 9 Rust unit tests, 1 doc test, and 31 Playwright tests. The browser suite covers every claim, all routes, keyboard focus, 44 px targets, 200% text, demo isolation, privacy requests, offline reload, and axe.
- `npm run lint` passed: formatting, Clippy with warnings denied, and TypeScript checks.
- `npm run build` passed and produced `dist/bin/docx-fidelity` plus `dist/site/`.
- `npm run package` passed and created the publishable crate without registry upload.
- Build budgets: initial JS 14.73 KB raw / 5.33 KB gzip; CSS 12.97 KB raw / 3.71 KB gzip.
- Manifest audit found 13 claims, exactly one matching `@claim:<id>` tag for each, and no extra claim tags.
- Every exact command in `.factory/claims.json` passed independently from clean clone `/tmp/docx-fidelity-polish2-clean-UKsoLo` at commit `69cf9825d8a24a0c5a5da45d79fce897e6c87a1d`. Logs are `/tmp/polish2-final-claim-<id>.log`.
- `/opt/fleet/lib/verify-url.sh` passed on live `/` and `/?demo=1` with no console errors. Reports and screenshots are under `.factory/polish-2/live/home/` and `.factory/polish-2/live/demo/`.
- `node .factory/polish-2/live-check.mjs` passed: five valid entries returned 200, the unknown route returned 404, every route had its title/H1 and zero WCAG A/AA axe violations, all links returned 200, same-origin privacy held, demo state remained isolated, focus/back worked, and offline reload worked. Result: `.factory/polish-2/live/live-check.json`.
- Lighthouse 13 mobile on live `/?demo=1`: Performance 100, Accessibility 100, Best Practices 100, SEO 100; LCP 1.1 s, CLS 0.001, total blocking time 10 ms. Report: `.factory/polish-2/live/lighthouse.json`.

## Release and deployment

- Repair commits: `28b411e814f354f249356038fbd4d60b69448aad`, `69cf9825d8a24a0c5a5da45d79fce897e6c87a1d`.
- Both commits are pushed to `origin/main`.
- Static deployment command: `/opt/fleet/lib/deploy-static.sh docx-markdown-fidelity-report dist/site`.
- Azure deployment ID: `cda6ea00-f70f-4f9f-aae1-403ee93549b2`.
- The custom-domain TLS check returned 200 after deployment.

## Reproduce

```sh
npm ci
npm test
npm run lint
npm run build
npm run package
node .factory/polish-2/live-check.mjs
```

Run each `test` command in `.factory/claims.json` independently from a fresh clone for the claims gate.

## Known gaps and next steps

None. No finding or deferred minor item remains. Registry publishing is intentionally left to the factory owner, as required for CLI artifacts.
