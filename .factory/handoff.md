# Handoff — adversarial first-read review 2

## Status: FAIL

This reviewer made no product-code changes. The evidence and one remaining P2 copy finding are in `.factory/review-2.md`.

## What was verified

- Reviewed the live site cold in fresh mobile (390 × 844) and desktop contexts.
- Used a clean clone at `bd6292b8deaf2d091eb978c0844ddb32e85c1ccf`; `npm ci`, all 13 exact claim commands, `npm test` (31 browser tests plus Rust tests), `npm run build`, and `npm run lint` passed.
- Confirmed `/?demo=1` immediately shows sample conversion data. Reset and Start for real remove only `demo:` storage; the direct CLI demo writes only to a fresh temp directory.
- Confirmed same-origin-only demo requests, no console errors, valid route metadata and 404, live links, headers, and zero axe WCAG 2 A/AA violations on all checked routes.
- Rechecked all four findings in `review-1.md`; each is actually fixed.

## Remaining work

Resolve **F-2-1**: replace or define the visible “CI policy” wording for the stated Word-documentation audience, and add the proposed copy regression. No other product change is requested by this review.

## How to verify after repair

```sh
npm ci
npm test
npm run build
npm run lint
```

Then perform a fresh mobile visit to `/`, confirm the expanded automated-check wording, and re-run the 13 commands in `.factory/claims.json`.
