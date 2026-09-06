# Handoff — verification 8

## Status: FAIL

Independent QA found one claims-contract issue and no runtime defect. The README promises a machine-readable `--json` command summary, but `.factory/claims.json` has no matching claim entry or dedicated tagged test. The required result is therefore **FAIL: 1 finding, 1 untested public claim**.

Implementation reviewed: `4d5864947bd41e6d5f7cad4b20a0f3c5ff702fb4`.

Documentation baseline reviewed: `62e91bcc6b3d02f407de15f59c602e5049d3389d`.

Full evidence and disposition: [verification-8.md](./verification-8.md).

## What was verified

- All 16 declared claim commands passed independently from a fresh GitHub clone after `npm ci`.
- `npm test`, `npm run lint`, `npm run build`, `npm run package`, and `cargo +1.88.0 test --locked` passed.
- The packaged crate installed in a new consumer root. The installed CLI completed normal, invalid, overwrite recovery, JSON, and demo paths while leaving the source unchanged.
- Production HTML, JS, and CSS matched the implementation build byte-for-byte.
- Fresh desktop and phone checks passed for the first screen, one-click sample, persistent label, reset isolation, keyboard controls, reduced motion, 200% text, and 44 px targets.
- Valid routes, titles, legal pages, link crawl, the designed HTTP 404, offline reload/update, privacy request logging, headers, and cache policy passed.
- Axe reported no violations on desktop routes and no serious or critical violations on phone routes. `verify-url.sh` passed without browser errors.
- Live mobile Lighthouse scored 100 in Performance, Accessibility, Best Practices, and SEO. LCP was 1.06 s and CLS was 0.0012.
- Every earlier review and verification finding was rechecked and remains fixed.

## Finding to resolve

Add a `json-output` entry to `.factory/claims.json` and one `@claim:json-output` test that proves stdout is parseable, contains the documented fields, and contains no extra prose. Removing the public JSON promise is the alternative if it is not meant to be supported.

No product code was changed during verification.

## Reproduce

```sh
npm ci
npm test
npm run lint
npm run build
npm run package
cargo +1.88.0 test --locked
```
