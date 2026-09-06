# Demo sandbox

- Browser entry: `https://docx-markdown-fidelity-report.sociobot.in/?demo=1` (the persistent `/demo` route is also supported)
- CLI entry: `docx-fidelity demo`
- Sample: `examples/field-guide.docx` contains headings, a table, an image, a footnote, a comment, a tracked insertion, and a custom style.
- CLI behavior: copies the sample into a fresh operating-system temporary directory, converts it there, prints report paths, and does not read or write the current directory.
- Browser behavior: plays a recording produced from the same bundled sample. **Replay recording** starts that recording again from the command line. It stores no document data. Demo state uses only the `demo:` local-storage prefix; **Reset demo** clears that prefix and reloads the bundled sample. **Start for real** leaves demo mode and discards demo state.
- Verification starts from a fresh browser context or fresh temp directory and uses only these entry points.
