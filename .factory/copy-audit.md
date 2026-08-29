# Landing page copy audit

Audited 2026-08-29 for v0.1.4. Counts treat commands, paths, and hyphenated terms as one word. Interface labels are included because assistive technology reads them. Commands are listed separately from prose and are not sentence-counted.

| Copy | Words | Result |
| --- | ---: | --- |
| Docx Markdown Fidelity Report | 4 | Pass |
| Demo | 1 | Pass |
| Install | 1 | Pass |
| Privacy | 1 | Pass |
| Local DOCX conversion · v0.1 | 5 | Pass |
| Convert DOCX and list review issues. | 6 | Pass |
| For teams moving Word documentation, it shows where Markdown needs a human check. | 13 | Pass |
| Try it with sample data | 5 | Pass |
| Convert the bundled field guide and open its fidelity report. | 10 | Pass |
| Runs on your computer. | 4 | Pass |
| Document data stays local. | 4 | Pass |
| Stop automated checks when reports find selected risks. | 8 | Pass |
| Review markers point to document sections that need checking. | 9 | Pass |
| Sample fidelity report · North Ridge guide | 6 | Pass |
| Review the sample fidelity report | 5 | Pass |
| The converter produces Markdown and a fidelity report for each section that needs checking. | 14 | Pass |
| Blocked | 1 | Pass |
| 9 findings | 2 | Pass |
| word/document.xml | 1 | Pass |
| Reference | 1 | Pass |
| Finding | 1 | Pass |
| Source | 1 | Pass |
| Action | 1 | Pass |
| Tracked edit | 2 | Pass |
| Paragraph 4 | 2 | Pass |
| Confirm accepted text | 3 | Pass |
| 3×2 table | 2 | Pass |
| Table 1 | 2 | Pass |
| Check merged cells | 3 | Pass |
| Image extracted | 2 | Pass |
| Paragraph 11 | 2 | Pass |
| Add alt text | 3 | Pass |
| Embedded file | 2 | Pass |
| Package part | 2 | Pass |
| Inspect separately | 2 | Pass |
| Three steps | 2 | Pass |
| How conversion works | 3 | Pass |
| Choose a file or folder | 5 | Pass |
| Choose one DOCX or a directory. | 6 | Pass |
| One command converts every DOCX in that directory. | 8 | Pass |
| Get conversion files | 3 | Pass |
| Get Markdown, extracted images, a JSON report, and a human checklist. | 11 | Pass |
| Review exact locations | 3 | Pass |
| Open each paragraph, table, row, or package part named in the report. | 12 | Pass |
| Single Rust binary | 3 | Pass |
| Run the bundled CLI demo | 5 | Pass |
| Build locally, then convert the bundled field guide in a fresh temporary directory. | 13 | Pass |
| Copy commands | 2 | Pass |
| Converted: field-guide.docx → field-guide.md | 3 | Pass |
| Report: blocked · 9 findings | 4 | Pass |
| Sandbox: /tmp/docx-fidelity-demo-… | 2 | Pass |
| Limits | 1 | Pass |
| What this CLI does not change | 6 | Pass |
| No OCR | 2 | Pass |
| No document editing | 3 | Pass |
| No PDF round-trip | 3 | Pass |
| No macro execution | 3 | Pass |
| No document upload | 3 | Pass |
| The CLI rejects ZIP or XML parts larger than 32 MiB. | 10 | Pass |
| Embedded files are reported, never opened. | 6 | Pass |
| Included workflow | 2 | Pass |
| Stop automated checks at selected risk levels | 7 | Pass |
| Use `--fail-on warning\|error` to stop an automated check at the selected risk level. | 13 | Pass |
| Automated checks run locally with each conversion. | 7 | Pass |
| Install the CLI | 3 | Pass |
| Convert DOCX and list review issues. | 6 | Pass |
| Terms | 1 | Pass |
| Built by Param Factory (external) | 5 | Pass |
| v0.1.4 · build 2026.08.29 | 4 | Pass |

No landing sentence exceeds 22 words or contains a banned word. The first screen states the job, audience, first action, action result, and three tested facts. “CI” does not appear on the landing page. The README expands “continuous integration (CI)” before using the abbreviation.

## Command examples

- `docx-fidelity convert word-files/ --output migration/`
- `cargo install --path .`
- `docx-fidelity demo`
- `docx-fidelity convert docs/ --output out/ --fail-on warning`

## Terminology table

| Concept | One term used |
| --- | --- |
| Microsoft Word package | DOCX |
| Converted text file | Markdown |
| Conversion output that lists issues | fidelity report |
| One report entry | finding |
| Human decision needed | review |
| Bundled isolated try-out | demo |
| Automated build threshold | automated check |
| Original place in the DOCX | source location |
