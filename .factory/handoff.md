# Handoff — review 4

## Status: FAIL

Review 4 tested the live site and packaged CLI without modifying product code. The implementation works, all 13 declared claim commands pass, and the live deployment matches the candidate. Two P2 findings remain: two public actions lack required manifest tests, and one subjective public sentence cannot be tested.

## Deliverables

- `.factory/review-4.md`: complete independent review, earlier-finding disposition, and FAIL verdict.
- `/work/.evidence/qa-report.md`: required report copy.
- `/work/.evidence/qa-result.json`: machine-readable verdict.
- `/work/.evidence/review-4/`: screenshots, live audit, claim logs, build logs, consumer results, headers, byte comparison, reduced-motion evidence, and Lighthouse JSON.

## Verification completed

From clean checkout `/tmp/docx-review4-clean` at documentation SHA `5e5c46410a5161ebf33311df6603fc4eba03984d`:

```sh
npm ci
# Each of the 13 exact commands in .factory/claims.json ran independently.
npm test
npm run build
npm run lint
npm run package
cargo +1.88.0 test --locked
```

Results: 9 Rust unit tests, 1 doctest, and 31 Playwright tests passed. The release binary, static site, lint, typecheck, and package gates passed. The packaged crate installed and ran in a clean consumer root. Live home/demo verifier checks, two-viewport axe scans, keyboard, focus, 200% text, reduced motion, offline reload/update, route/link checks, and mobile Lighthouse all passed.

Implementation candidate: `28b411e814f354f249356038fbd4d60b69448aad`. Later commit `69cf982` changes README/test documentation only; later commits through `5e5c464` are report/evidence changes. All 16 public build files matched live byte for byte.

## Work left

1. Add `.factory/claims.json` entries and one exact tagged test each for **Copy commands** and **Replay recording**, or remove those actions.
2. Replace “The Markdown stays usable” with a concrete tested statement, or remove it.
3. Rerun every manifest command and the full gates. PASS requires zero findings and zero untested claims.
