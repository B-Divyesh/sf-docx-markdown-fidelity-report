# Docx Markdown Fidelity Report

Convert DOCX to Markdown and list every item that needs review.

This local CLI is for teams moving Word-heavy documentation. It creates readable Markdown, extracts images, and writes a source-located fidelity ledger beside each result. It never executes macros or embedded objects.

## Try the bundled sample

```sh
cargo run -- demo
```

The command uses a fresh temporary directory and prints its path. The same flow is recorded at <https://docx-markdown-fidelity-report.sociobot.in/demo>.

## Install

Build the single binary with stable Rust 1.80 or newer:

```sh
cargo install --path .
docx-fidelity --help
```

No office suite, account, or conversion service is required.

## Usage

Convert one file:

```sh
docx-fidelity convert handbook.docx --output migration/
```

Convert every `.docx` file in a directory:

```sh
docx-fidelity convert word-files/ --output migration/
```

Write a machine-readable command summary to stdout:

```sh
docx-fidelity --json convert handbook.docx --output migration/
```

Each input produces:

- `<name>.md`: converted headings, paragraphs, lists, links, tables, notes, and image references.
- `<name>.fidelity.json`: structured findings with category, severity, source part, paragraph, table, row, and cell.
- `<name>.fidelity.md`: a review checklist for people.
- `<name>.media/`: safely named extracted images, when the DOCX contains images.

The report covers tables, comments, tracked revisions, embedded objects, footnotes, non-default styles, and every image extraction outcome. Unknown or lossy constructs stay visible as findings.

## Exit codes

- `0`: conversion completed. Findings may still need review.
- `2`: bad arguments, unsafe input, or a read/write failure.
- `3`: a licensed CI policy threshold was met.
- `4`: the supplied team license is invalid or cannot be checked.

## Team license

All conversion and report data stays free. A $29 one-time team license adds `--fail-on warning|error`, so CI can stop on a chosen risk level.

Buy at <https://api.sociobot.in/api/v1/products/docx-markdown-fidelity-report/checkout>. The hosted checkout returns a license token. Sociobot/Dodo is the merchant of record and handles refunds.

```sh
export DOCX_FIDELITY_LICENSE="your-token"
docx-fidelity convert docs/ --output out/ --fail-on warning
```

The CLI sends only the token to Sociobot for verification. It never sends document names or contents. You can also verify a token with `docx-fidelity license verify TOKEN`.

## Scope and safety

The converter reads ZIP and XML parts with bounded file sizes. It rejects unsafe archive paths and never runs macros, links, or embedded files. It does not perform OCR, edit DOCX files, or round-trip PDF.

## Develop and verify

```sh
npm install
npm test
npm run build
npm run package
```

`npm test` runs Rust unit and integration tests plus browser claim tests. `npm run build` produces the release binary and the static site at `dist/site/`. `npm run package` creates the publishable crate without sending it to a registry.

## Repository map

- `src/`: Rust CLI and conversion library.
- `examples/`: bundled sample DOCX and expected demo behavior.
- `site/`: Vite landing, docs, demo, privacy, terms, and 404 routes.
- `.factory/`: brief, design system, demo contract, claims, audits, and handoff.

## Privacy and license

There is no telemetry. See the [privacy page](https://docx-markdown-fidelity-report.sociobot.in/privacy) and [terms](https://docx-markdown-fidelity-report.sociobot.in/terms). Source code is MIT licensed.
