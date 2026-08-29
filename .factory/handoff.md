# Handoff — repair 2

## Release status: ready

- Repaired candidate: `5dd6ca2fc371e71be22c38a1092cec3a42a633a5`
- Verifier report: `1df6164762ccf3391ecc3430748596eb0f7240e6`
- Repair commit: `e3e8f72`
- Deployed URL: https://docx-markdown-fidelity-report.sociobot.in
- Azure deployment: `80da0fc6-cb76-4c58-9050-be5487b6a56e`

The verifier's only release blocker is repaired. The footer now links to `https://sociobot.in/`, whose certificate covers the hostname, instead of `https://www.sociobot.in`, which fails TLS hostname validation.

## Regression coverage

`@regression:param-factory-footer-link` opens `/`, `/demo`, `/privacy`, `/terms`, and the 404 route. It asserts that every **Built by Param Factory** link is visible, keeps its external-link semantics, and uses the exact certificate-valid URL. The post-deploy crawler followed every distinct HTTP link with normal TLS validation; all returned HTTP 200, including `https://sociobot.in/`.

## Verification evidence

- Clean install: `npm ci` completed with 24 packages and 0 vulnerabilities.
- Full suite: `npm test` passed 17 tests, including Rust unit tests, the doctest, all claim tests, route axe checks, keyboard/mobile coverage, and service-worker offline reload.
- Every `.factory/claims.json` command was also run separately. All six passed: `demo-conversion`, `local-processing`, `risk-ledger`, `batch-conversion`, `ci-policy`, and `safe-input`.
- Static checks: `npm run lint` passed `cargo fmt --check`, Clippy with warnings denied, and TypeScript typechecking.
- Production artifacts: `npm run build` produced `dist/bin/docx-fidelity` and `dist/site`; JS is 14.46 KB raw / 5.28 KB gzip and CSS is 12.40 KB raw / 3.62 KB gzip.
- Package: `npm run package` produced an 83.7 KiB crate (25.6 KiB compressed). A fresh consumer installed it from the crate, ran `--help` and `--json demo`, converted one sample with nine findings, then converted a two-file batch with every promised ledger category.
- Browser: the factory `verify-url.sh` passed with no console errors. `/`, `/demo`, `/privacy`, `/terms`, and an unknown route each returned 200 on desktop and 390×844 mobile, with one `h1`, one `main`, route-specific titles, no horizontal overflow, and zero serious or critical axe findings.
- Keyboard and motion: Tab exposed the skip link; Enter focused `#main`; Enter on the demo action opened `/demo` and focused its heading. Reduced-motion animation and transition durations are `0.00001s`.
- Privacy: the complete `/demo` request log contained only `https://docx-markdown-fidelity-report.sociobot.in`. No analytics, third-party script, or document upload occurred.
- Offline/update: the deployed service worker activated cache `docx-fidelity-shell-index-C1ZRaeIZ.js`; a network-disabled reload rendered the landing heading.
- Headers and caching: HTML and `sw.js` use `public, must-revalidate, max-age=30`; the hashed JS uses `public, max-age=31536000, immutable`. CSP, HSTS, `nosniff`, Referrer-Policy, and Permissions-Policy are present.
- Live identity: 14 deployed files matched `dist/site` byte-for-byte. SHA-256: HTML `e1595ec528f129b6b5075b49d1a55acfdd7678f6834dd921fa5d60ea395ff24a`; JS `c87315d770e22235e48fe7a7f2ad2ac129fb917c6fa71f70744aa9dcf5997b32`; CSS `8f73fd99ed10dfbf13d4304d66fb7a704496567487ba3ca1e5e060f11644da63`.
- Response policy: the retained license verifier returned 200 for 30 invalid synthetic tokens, then 429 with `Retry-After: 3` on request 31.
- Lighthouse 13.4.1 mobile: performance 99, accessibility 100, best practices 100, SEO 100; LCP 1.96 s, CLS 0.0011, total blocking time 0 ms.

## Run and verify

```sh
npm ci
npm test
npm run lint
npm run build
npm run package
cargo run -- demo
```

Deployment uses the work-order command `npm ci && npm run build:site`, output directory `dist/site`, and `/opt/fleet/lib/deploy-static.sh docx-markdown-fidelity-report dist/site`.

## Known gaps and scope boundaries

No release blocker remains. The product intentionally does not do OCR, document editing, PDF round-tripping, macro execution, embedded-object extraction, or hosted document conversion. Registry publishing remains a factory-owner step; this repair only produced and consumer-tested the package.
