//! DOCX-to-Markdown conversion with a source-located fidelity ledger.
//!
//! The small public API mirrors the CLI:
//!
//! ```no_run
//! use docx_fidelity::{convert_path, ConvertOptions};
//! let result = convert_path("handbook.docx", "out", &ConvertOptions::default())?;
//! assert!(!result.markdown_path.as_os_str().is_empty());
//! # Ok::<(), anyhow::Error>(())
//! ```

use anyhow::{bail, Context, Result};
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap, HashSet};
use std::fs::{self, File};
use std::io::{Read, Seek};
use std::path::{Component, Path, PathBuf};
use zip::ZipArchive;

const MAX_ENTRY_BYTES: u64 = 32 * 1024 * 1024;
const MAX_TOTAL_BYTES: u64 = 128 * 1024 * 1024;
const MAX_ENTRIES: usize = 10_000;

#[derive(Debug, Clone, Default)]
pub struct ConvertOptions {
    pub overwrite: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversionResult {
    pub input: PathBuf,
    pub markdown_path: PathBuf,
    pub json_report_path: PathBuf,
    pub markdown_report_path: PathBuf,
    pub media_dir: PathBuf,
    pub report: FidelityReport,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FidelityReport {
    pub schema_version: u8,
    pub source: String,
    pub status: ReportStatus,
    pub counts: BTreeMap<String, usize>,
    pub findings: Vec<Finding>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ReportStatus {
    Clear,
    Review,
    Blocked,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Severity {
    Info,
    Warning,
    Error,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceLocation {
    pub part: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paragraph: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub row: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cell: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Finding {
    pub category: String,
    pub severity: Severity,
    pub summary: String,
    pub action: String,
    pub location: SourceLocation,
}

#[derive(Default)]
struct ParseState {
    markdown: String,
    findings: Vec<Finding>,
    paragraph: usize,
    table: usize,
    row: usize,
    cell: usize,
    in_paragraph: bool,
    in_text: bool,
    in_deleted_text: bool,
    deleted_depth: usize,
    in_table: bool,
    in_cell: bool,
    paragraph_text: String,
    paragraph_style: Option<String>,
    paragraph_list: Option<ParagraphList>,
    list_counters: HashMap<(String, usize), usize>,
    table_rows: Vec<Vec<String>>,
    table_row: Vec<String>,
    cell_text: String,
    revision_seen: HashSet<String>,
    comment_seen: HashSet<String>,
    embedded_seen: HashSet<String>,
    unsupported_seen: HashSet<String>,
    hyperlink: Option<(usize, String, bool)>,
    run_start: Option<(usize, bool)>,
    run_bold: bool,
    run_italic: bool,
    image_index: usize,
    footnote_refs: Vec<String>,
}

/// Numbering definitions available to paragraphs in `word/document.xml`.
///
/// Word stores the paragraph's `numId` separately from the abstract list's
/// level and number format. Keeping that indirection means a decimal list is
/// not silently mistaken for a bullet list.
#[derive(Default)]
struct Numbering {
    num_to_abstract: HashMap<String, String>,
    formats: HashMap<(String, usize), ListDefinition>,
    starts: HashMap<(String, usize), usize>,
}

impl Numbering {
    fn definition(&self, num_id: &str, level: usize) -> Option<ListDefinition> {
        let abstract_id = self.num_to_abstract.get(num_id)?;
        let mut definition = self.formats.get(&(abstract_id.clone(), level))?.clone();
        if let Some(start) = self.starts.get(&(num_id.into(), level)) {
            definition.start = *start;
        }
        Some(definition)
    }
}

#[derive(Clone)]
struct ListDefinition {
    format: String,
    start: usize,
}

#[derive(Clone)]
struct ParagraphList {
    num_id: Option<String>,
    level: usize,
    format: Option<String>,
    start: usize,
}

pub fn convert_path(
    input: impl AsRef<Path>,
    output_dir: impl AsRef<Path>,
    options: &ConvertOptions,
) -> Result<ConversionResult> {
    let input = input.as_ref();
    let output_dir = output_dir.as_ref();
    let stem = safe_stem(
        input
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("document"),
    );
    let paths = output_paths(output_dir, &stem);
    preflight_outputs(std::slice::from_ref(&paths), options.overwrite)?;
    convert_path_to(input, output_dir, &stem, options)
}

/// Convert one DOCX or every DOCX in a directory with collision-free output names.
///
/// The complete batch is planned before conversion starts. Inputs whose safe names
/// normalize to the same stem receive a numeric suffix, so no result can replace
/// another result even when `overwrite` is enabled.
pub fn convert_paths(
    input: impl AsRef<Path>,
    output_dir: impl AsRef<Path>,
    options: &ConvertOptions,
) -> Result<Vec<ConversionResult>> {
    let inputs = discover_inputs(input)?;
    let output_dir = output_dir.as_ref();
    let stems = unique_output_stems(&inputs);
    let paths: Vec<_> = stems
        .iter()
        .map(|stem| output_paths(output_dir, stem))
        .collect();
    preflight_outputs(&paths, options.overwrite)?;
    inputs
        .iter()
        .zip(stems)
        .map(|(path, stem)| {
            convert_path_to(path, output_dir, &stem, options)
                .with_context(|| format!("conversion failed for {}", path.display()))
        })
        .collect()
}

fn convert_path_to(
    input: &Path,
    output_dir: &Path,
    stem: &str,
    options: &ConvertOptions,
) -> Result<ConversionResult> {
    if input
        .extension()
        .and_then(|s| s.to_str())
        .map(str::to_ascii_lowercase)
        .as_deref()
        != Some("docx")
    {
        bail!("input must be a .docx file: {}", input.display());
    }
    let metadata =
        fs::metadata(input).with_context(|| format!("cannot read {}", input.display()))?;
    if !metadata.is_file() {
        bail!("input is not a file: {}", input.display());
    }
    fs::create_dir_all(output_dir)
        .with_context(|| format!("cannot create {}", output_dir.display()))?;
    let final_paths = output_paths(output_dir, stem);

    let file = File::open(input)?;
    let mut archive = ZipArchive::new(file).context("input is not a valid DOCX ZIP archive")?;
    validate_archive(&mut archive)?;
    let document =
        read_part(&mut archive, "word/document.xml")?.context("DOCX has no word/document.xml")?;
    validate_document_xml(&document).context("word/document.xml is malformed")?;
    let rels = read_part(&mut archive, "word/_rels/document.xml.rels")?
        .map(|xml| parse_relationships(&xml))
        .transpose()?
        .unwrap_or_default();
    let styles = read_part(&mut archive, "word/styles.xml")?
        .map(|xml| parse_styles(&xml))
        .transpose()?
        .unwrap_or_default();
    let numbering = read_part(&mut archive, "word/numbering.xml")?
        .map(|xml| parse_numbering(&xml))
        .transpose()?
        .unwrap_or_default();
    let comments = read_part(&mut archive, "word/comments.xml")?
        .map(|xml| parse_notes(&xml, "comment"))
        .transpose()?
        .unwrap_or_default();
    let footnotes = read_part(&mut archive, "word/footnotes.xml")?
        .map(|xml| parse_notes(&xml, "footnote"))
        .transpose()?
        .unwrap_or_default();

    let staging = StagingDir::create(output_dir)?;
    let staged_paths = output_paths(&staging.path, stem);
    fs::create_dir_all(&staged_paths.media_dir)?;
    let mut state = parse_document(
        &document,
        &rels,
        &styles,
        &numbering,
        &comments,
        &footnotes,
        &mut archive,
        &staged_paths.media_dir,
    )?;
    for (id, text) in &comments {
        if !state.comment_seen.contains(id) {
            state.findings.push(Finding {
                category: "comments".into(),
                severity: Severity::Warning,
                summary: format!(
                    "Comment {id} has no source anchor. “{}”",
                    truncate(text, 100)
                ),
                action: "Find and resolve this comment in Word before publishing.".into(),
                location: SourceLocation {
                    part: "word/comments.xml".into(),
                    paragraph: None,
                    table: None,
                    row: None,
                    cell: None,
                },
            });
        }
    }
    append_package_findings(&mut archive, &mut state)?;
    if !state.footnote_refs.is_empty() {
        state.markdown.push_str("\n## Notes\n\n");
        for id in &state.footnote_refs {
            let text = footnotes
                .get(id)
                .cloned()
                .unwrap_or_else(|| "Footnote text was not found.".into());
            state
                .markdown
                .push_str(&format!("[^{id}]: {}\n", escape_markdown_source(&text)));
        }
    }
    if fs::read_dir(&staged_paths.media_dir)?.next().is_none() {
        fs::remove_dir(&staged_paths.media_dir)?;
    }
    let report = build_report(input, state.findings);
    fs::write(
        &staged_paths.markdown,
        state.markdown.trim_end().to_owned() + "\n",
    )?;
    fs::write(
        &staged_paths.json_report,
        serde_json::to_string_pretty(&report)? + "\n",
    )?;
    fs::write(&staged_paths.markdown_report, render_report(&report))?;
    commit_outputs(&staged_paths, &final_paths, options.overwrite)?;
    Ok(ConversionResult {
        input: input.to_path_buf(),
        markdown_path: final_paths.markdown,
        json_report_path: final_paths.json_report,
        markdown_report_path: final_paths.markdown_report,
        media_dir: final_paths.media_dir,
        report,
    })
}

#[derive(Clone)]
struct OutputPaths {
    markdown: PathBuf,
    json_report: PathBuf,
    markdown_report: PathBuf,
    media_dir: PathBuf,
}

fn output_paths(output_dir: &Path, stem: &str) -> OutputPaths {
    OutputPaths {
        markdown: output_dir.join(format!("{stem}.md")),
        json_report: output_dir.join(format!("{stem}.fidelity.json")),
        markdown_report: output_dir.join(format!("{stem}.fidelity.md")),
        media_dir: output_dir.join(format!("{stem}.media")),
    }
}

fn unique_output_stems(inputs: &[PathBuf]) -> Vec<String> {
    let mut used = HashSet::new();
    inputs
        .iter()
        .map(|input| {
            let base = safe_stem(
                input
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("document"),
            );
            let mut candidate = base.clone();
            let mut suffix = 2usize;
            while !used.insert(candidate.to_ascii_lowercase()) {
                candidate = format!("{base}-{suffix}");
                suffix += 1;
            }
            candidate
        })
        .collect()
}

fn preflight_outputs(paths: &[OutputPaths], overwrite: bool) -> Result<()> {
    if overwrite {
        return Ok(());
    }
    for outputs in paths {
        for path in [
            &outputs.markdown,
            &outputs.json_report,
            &outputs.markdown_report,
            &outputs.media_dir,
        ] {
            if path.exists() {
                bail!(
                    "output exists: {} (pass --overwrite to replace it)",
                    path.display()
                );
            }
        }
    }
    Ok(())
}

fn commit_outputs(staged: &OutputPaths, final_paths: &OutputPaths, overwrite: bool) -> Result<()> {
    for (source, destination) in [
        (&staged.markdown, &final_paths.markdown),
        (&staged.json_report, &final_paths.json_report),
        (&staged.markdown_report, &final_paths.markdown_report),
    ] {
        replace_path(source, destination, overwrite)?;
    }
    if staged.media_dir.exists() {
        replace_path(&staged.media_dir, &final_paths.media_dir, overwrite)?;
    } else if overwrite && final_paths.media_dir.exists() {
        remove_exact_path(&final_paths.media_dir)?;
    }
    Ok(())
}

fn replace_path(source: &Path, destination: &Path, overwrite: bool) -> Result<()> {
    if destination.exists() {
        if !overwrite {
            bail!(
                "output exists: {} (pass --overwrite to replace it)",
                destination.display()
            );
        }
        remove_exact_path(destination)?;
    }
    fs::rename(source, destination)
        .with_context(|| format!("cannot move completed output to {}", destination.display()))
}

fn remove_exact_path(path: &Path) -> Result<()> {
    let metadata = fs::symlink_metadata(path)?;
    if metadata.file_type().is_dir() && !metadata.file_type().is_symlink() {
        fs::remove_dir_all(path)?;
    } else {
        fs::remove_file(path)?;
    }
    Ok(())
}

struct StagingDir {
    path: PathBuf,
}

impl StagingDir {
    fn create(parent: &Path) -> Result<Self> {
        for attempt in 0..1000 {
            let path = parent.join(format!(
                ".docx-fidelity-stage-{}-{attempt}",
                std::process::id()
            ));
            match fs::create_dir(&path) {
                Ok(()) => return Ok(Self { path }),
                Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => continue,
                Err(error) => return Err(error.into()),
            }
        }
        bail!(
            "cannot create a unique staging directory in {}",
            parent.display()
        )
    }
}

impl Drop for StagingDir {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.path);
    }
}

pub fn discover_inputs(input: impl AsRef<Path>) -> Result<Vec<PathBuf>> {
    let input = input.as_ref();
    if input.is_file() {
        return Ok(vec![input.to_path_buf()]);
    }
    if !input.is_dir() {
        bail!("input does not exist: {}", input.display());
    }
    let mut files: Vec<_> = fs::read_dir(input)?
        .filter_map(|entry| entry.ok().map(|e| e.path()))
        .filter(|path| {
            path.extension()
                .and_then(|s| s.to_str())
                .map(str::to_ascii_lowercase)
                .as_deref()
                == Some("docx")
        })
        .collect();
    files.sort();
    if files.is_empty() {
        bail!("no .docx files found in {}", input.display());
    }
    Ok(files)
}

fn validate_archive<R: Read + Seek>(archive: &mut ZipArchive<R>) -> Result<()> {
    if archive.len() > MAX_ENTRIES {
        bail!("DOCX contains too many archive entries");
    }
    let mut total = 0u64;
    for index in 0..archive.len() {
        let entry = archive.by_index(index)?;
        if entry.enclosed_name().is_none() {
            bail!("DOCX contains an unsafe archive path");
        }
        if entry.size() > MAX_ENTRY_BYTES {
            bail!("DOCX part is larger than 32 MB: {}", entry.name());
        }
        total = total.saturating_add(entry.size());
        if total > MAX_TOTAL_BYTES {
            bail!("DOCX expands beyond the 128 MB safety limit");
        }
    }
    Ok(())
}

fn read_part<R: Read + Seek>(archive: &mut ZipArchive<R>, name: &str) -> Result<Option<Vec<u8>>> {
    let Ok(mut entry) = archive.by_name(name) else {
        return Ok(None);
    };
    let mut bytes = Vec::with_capacity(entry.size() as usize);
    entry.read_to_end(&mut bytes)?;
    Ok(Some(bytes))
}

fn validate_document_xml(xml: &[u8]) -> Result<()> {
    let mut reader = Reader::from_reader(xml);
    let mut stack: Vec<Vec<u8>> = Vec::new();
    let mut saw_document = false;
    let mut saw_body = false;
    loop {
        match reader.read_event()? {
            Event::Start(event) => {
                let name = local_name(event.name().as_ref()).to_vec();
                if stack.is_empty() {
                    if saw_document || name != b"document" {
                        bail!("expected one `document` root element");
                    }
                    saw_document = true;
                } else if stack.len() == 1 && name == b"body" {
                    saw_body = true;
                }
                stack.push(name);
            }
            Event::Empty(event) => {
                let name = local_name(event.name().as_ref()).to_vec();
                if stack.is_empty() {
                    bail!("the `document` root element cannot be empty");
                }
                if stack.len() == 1 && name == b"body" {
                    saw_body = true;
                }
            }
            Event::End(event) => {
                let name = local_name(event.name().as_ref()).to_vec();
                let Some(open_name) = stack.pop() else {
                    bail!("found a closing element without an opening element");
                };
                if open_name != name {
                    bail!(
                        "closing element `{}` does not match `{}`",
                        String::from_utf8_lossy(&name),
                        String::from_utf8_lossy(&open_name)
                    );
                }
            }
            Event::Text(text) if stack.is_empty() && !text.unescape()?.trim().is_empty() => {
                bail!("found text outside the `document` root element");
            }
            Event::DocType(_) => bail!("document type declarations are not allowed"),
            Event::Eof => break,
            _ => {}
        }
    }
    if !saw_document {
        bail!("the `document` root element is missing");
    }
    if !saw_body {
        bail!("the `document` body element is missing");
    }
    if let Some(open_name) = stack.last() {
        bail!(
            "the `{}` element is not closed",
            String::from_utf8_lossy(open_name)
        );
    }
    Ok(())
}

fn parse_relationships(xml: &[u8]) -> Result<HashMap<String, String>> {
    let mut reader = Reader::from_reader(xml);
    reader.config_mut().trim_text(true);
    let mut map = HashMap::new();
    loop {
        match reader.read_event()? {
            Event::Empty(e) | Event::Start(e)
                if local_name(e.name().as_ref()) == b"Relationship" =>
            {
                let id = attr(&e, b"Id");
                let target = attr(&e, b"Target");
                if let (Some(id), Some(target)) = (id, target) {
                    map.insert(id, target);
                }
            }
            Event::Eof => break,
            _ => {}
        }
    }
    Ok(map)
}

fn parse_styles(xml: &[u8]) -> Result<HashMap<String, String>> {
    let mut reader = Reader::from_reader(xml);
    let mut map = HashMap::new();
    let mut id = None;
    loop {
        match reader.read_event()? {
            Event::Start(e) if local_name(e.name().as_ref()) == b"style" => {
                id = attr_local(&e, b"styleId")
            }
            Event::Empty(e) if local_name(e.name().as_ref()) == b"name" => {
                if let (Some(style_id), Some(name)) = (id.clone(), attr_local(&e, b"val")) {
                    map.insert(style_id, name);
                }
            }
            Event::End(e) if local_name(e.name().as_ref()) == b"style" => id = None,
            Event::Eof => break,
            _ => {}
        }
    }
    Ok(map)
}

fn parse_numbering(xml: &[u8]) -> Result<Numbering> {
    let mut reader = Reader::from_reader(xml);
    let mut numbering = Numbering::default();
    let mut abstract_id = None;
    let mut num_id = None;
    let mut level = None;
    let mut level_start = 1usize;
    let mut override_level = None;
    loop {
        match reader.read_event()? {
            Event::Start(e) if local_name(e.name().as_ref()) == b"abstractNum" => {
                abstract_id = attr_local(&e, b"abstractNumId");
            }
            Event::End(e) if local_name(e.name().as_ref()) == b"abstractNum" => {
                abstract_id = None;
            }
            Event::Start(e) if local_name(e.name().as_ref()) == b"num" => {
                num_id = attr_local(&e, b"numId");
            }
            Event::End(e) if local_name(e.name().as_ref()) == b"num" => {
                num_id = None;
            }
            Event::Start(e) if local_name(e.name().as_ref()) == b"lvl" => {
                level = attr_local(&e, b"ilvl").and_then(|value| value.parse().ok());
                level_start = 1;
            }
            Event::End(e) if local_name(e.name().as_ref()) == b"lvl" => level = None,
            Event::Start(e) if local_name(e.name().as_ref()) == b"lvlOverride" => {
                override_level = attr_local(&e, b"ilvl").and_then(|value| value.parse().ok());
            }
            Event::End(e) if local_name(e.name().as_ref()) == b"lvlOverride" => {
                override_level = None;
            }
            Event::Empty(e) | Event::Start(e) if local_name(e.name().as_ref()) == b"start" => {
                level_start = attr_local(&e, b"val")
                    .and_then(|value| value.parse().ok())
                    .unwrap_or(1);
            }
            Event::Empty(e) | Event::Start(e)
                if local_name(e.name().as_ref()) == b"startOverride" =>
            {
                if let (Some(num), Some(override_level), Some(start)) = (
                    num_id.clone(),
                    override_level,
                    attr_local(&e, b"val").and_then(|value| value.parse().ok()),
                ) {
                    numbering.starts.insert((num, override_level), start);
                }
            }
            Event::Empty(e) | Event::Start(e)
                if local_name(e.name().as_ref()) == b"abstractNumId" =>
            {
                if let (Some(num), Some(abstract_num)) = (num_id.clone(), attr_local(&e, b"val")) {
                    numbering.num_to_abstract.insert(num, abstract_num);
                }
            }
            Event::Empty(e) | Event::Start(e) if local_name(e.name().as_ref()) == b"numFmt" => {
                if let (Some(abstract_num), Some(list_level), Some(format)) =
                    (abstract_id.clone(), level, attr_local(&e, b"val"))
                {
                    numbering.formats.insert(
                        (abstract_num, list_level),
                        ListDefinition {
                            format,
                            start: level_start,
                        },
                    );
                }
            }
            Event::Eof => break,
            _ => {}
        }
    }
    Ok(numbering)
}

fn parse_notes(xml: &[u8], element: &str) -> Result<HashMap<String, String>> {
    let mut reader = Reader::from_reader(xml);
    let mut notes = HashMap::new();
    let mut current = None;
    let mut text = String::new();
    let mut in_text = false;
    loop {
        match reader.read_event()? {
            Event::Start(e) if local_name(e.name().as_ref()) == element.as_bytes() => {
                current = attr_local(&e, b"id");
                text.clear();
            }
            Event::Start(e) if local_name(e.name().as_ref()) == b"t" => in_text = true,
            Event::Text(e) if in_text && current.is_some() => text.push_str(&e.unescape()?),
            Event::End(e) if local_name(e.name().as_ref()) == b"t" => in_text = false,
            Event::End(e) if local_name(e.name().as_ref()) == element.as_bytes() => {
                if let Some(id) = current.take() {
                    notes.insert(id, text.trim().to_owned());
                }
            }
            Event::Eof => break,
            _ => {}
        }
    }
    Ok(notes)
}

#[allow(clippy::too_many_arguments)]
fn parse_document<R: Read + Seek>(
    xml: &[u8],
    rels: &HashMap<String, String>,
    styles: &HashMap<String, String>,
    numbering: &Numbering,
    comments: &HashMap<String, String>,
    _footnotes: &HashMap<String, String>,
    archive: &mut ZipArchive<R>,
    media_dir: &Path,
) -> Result<ParseState> {
    let mut reader = Reader::from_reader(xml);
    reader.config_mut().trim_text(false);
    let mut s = ParseState::default();
    loop {
        match reader
            .read_event()
            .context("word/document.xml is malformed")?
        {
            Event::Start(e) => handle_start(
                &e, &mut s, rels, styles, numbering, comments, archive, media_dir,
            )?,
            Event::Empty(e) => handle_empty(
                &e, &mut s, rels, styles, numbering, comments, archive, media_dir,
            )?,
            Event::Text(e) if s.in_text && !s.in_deleted_text && s.deleted_depth == 0 => {
                let value = escape_markdown_source(&e.unescape()?);
                if s.in_cell {
                    s.cell_text.push_str(&value);
                } else {
                    s.paragraph_text.push_str(&value);
                }
            }
            Event::End(e) => handle_end(local_name(e.name().as_ref()), &mut s, styles),
            Event::Eof => break,
            _ => {}
        }
    }
    Ok(s)
}

#[allow(clippy::too_many_arguments)]
fn handle_start<R: Read + Seek>(
    e: &BytesStart<'_>,
    s: &mut ParseState,
    rels: &HashMap<String, String>,
    styles: &HashMap<String, String>,
    numbering: &Numbering,
    comments: &HashMap<String, String>,
    archive: &mut ZipArchive<R>,
    media_dir: &Path,
) -> Result<()> {
    let qualified_name = e.name();
    let name = local_name(qualified_name.as_ref());
    match name {
        b"p" => {
            s.paragraph += 1;
            s.in_paragraph = true;
            s.paragraph_text.clear();
            s.paragraph_style = None;
            s.paragraph_list = None;
        }
        b"r" => {
            let length = if s.in_cell {
                s.cell_text.len()
            } else {
                s.paragraph_text.len()
            };
            s.run_start = Some((length, s.in_cell));
            s.run_bold = false;
            s.run_italic = false;
        }
        b"t" => s.in_text = true,
        b"numPr" => start_list(s),
        b"numId" => set_list_num_id(e, s, numbering),
        b"ilvl" => set_list_level(e, s, numbering),
        b"delText" => s.in_deleted_text = true,
        b"tbl" => {
            s.table += 1;
            s.row = 0;
            s.in_table = true;
            s.table_rows.clear();
        }
        b"tr" => {
            s.row += 1;
            s.table_row.clear();
        }
        b"tc" => {
            s.cell += 1;
            s.in_cell = true;
            s.cell_text.clear();
        }
        b"ins" | b"moveTo" => revision_finding(name, e, s),
        b"del" | b"moveFrom" => {
            revision_finding(name, e, s);
            s.deleted_depth += 1;
        }
        b"rPrChange" | b"pPrChange" | b"tblPrChange" => revision_finding(name, e, s),
        b"hyperlink" => hyperlink_start(e, s, rels),
        b"commentRangeStart" => comment_finding(e, s, comments),
        b"object" | b"oleObject" | b"altChunk" => embedded_finding(name, s),
        b"blip" | b"imagedata" => image_finding(e, s, rels, archive, media_dir)?,
        b"footnoteReference" => footnote_finding(e, s),
        b"b" => s.run_bold = attr_is_on(e),
        b"i" => s.run_italic = attr_is_on(e),
        b"u" => inline_loss_finding("underline", s),
        b"oMath" | b"oMathPara" => inline_loss_finding("equation", s),
        b"txbxContent" => inline_loss_finding("text box", s),
        b"fldSimple" | b"instrText" => inline_loss_finding("field", s),
        b"pict" => inline_loss_finding("legacy drawing", s),
        _ => {
            let _ = (styles, numbering);
        }
    }
    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn handle_empty<R: Read + Seek>(
    e: &BytesStart<'_>,
    s: &mut ParseState,
    rels: &HashMap<String, String>,
    styles: &HashMap<String, String>,
    numbering: &Numbering,
    comments: &HashMap<String, String>,
    archive: &mut ZipArchive<R>,
    media_dir: &Path,
) -> Result<()> {
    let qualified_name = e.name();
    let name = local_name(qualified_name.as_ref());
    match name {
        b"pStyle" => s.paragraph_style = attr_local(e, b"val"),
        b"rStyle" => {
            if let Some(style_id) = attr_local(e, b"val") {
                let style_name = styles.get(&style_id).cloned().unwrap_or(style_id);
                if !is_supported_style(&style_name, &style_name) {
                    s.findings.push(finding(
                        "styles",
                        Severity::Warning,
                        format!("Text uses the character style “{style_name}”."),
                        "Check the styled text in the Markdown.",
                        s,
                    ));
                }
            }
        }
        b"numPr" => start_list(s),
        b"numId" => set_list_num_id(e, s, numbering),
        b"ilvl" => set_list_level(e, s, numbering),
        b"tab" => {
            if s.in_cell {
                s.cell_text.push('\t')
            } else {
                s.paragraph_text.push('\t')
            }
        }
        b"br" | b"cr" => {
            if s.in_cell {
                s.cell_text.push_str("<br>")
            } else {
                s.paragraph_text.push_str("  \n")
            }
        }
        b"ins" | b"del" | b"moveFrom" | b"moveTo" | b"rPrChange" | b"pPrChange"
        | b"tblPrChange" => revision_finding(name, e, s),
        b"commentRangeStart" => comment_finding(e, s, comments),
        b"object" | b"oleObject" | b"altChunk" => embedded_finding(name, s),
        b"blip" | b"imagedata" => image_finding(e, s, rels, archive, media_dir)?,
        b"footnoteReference" => footnote_finding(e, s),
        b"b" => s.run_bold = attr_is_on(e),
        b"i" => s.run_italic = attr_is_on(e),
        b"u" => inline_loss_finding("underline", s),
        b"oMath" | b"oMathPara" => inline_loss_finding("equation", s),
        b"txbxContent" => inline_loss_finding("text box", s),
        b"fldSimple" | b"instrText" => inline_loss_finding("field", s),
        b"pict" => inline_loss_finding("legacy drawing", s),
        _ => {
            let _ = (styles, numbering);
        }
    }
    Ok(())
}

fn handle_end(name: &[u8], s: &mut ParseState, styles: &HashMap<String, String>) {
    match name {
        b"t" => s.in_text = false,
        b"delText" => s.in_deleted_text = false,
        b"del" | b"moveFrom" => s.deleted_depth = s.deleted_depth.saturating_sub(1),
        b"r" => finish_run(s),
        b"hyperlink" => finish_hyperlink(s),
        b"p" => {
            let text = if s.in_cell {
                s.cell_text.trim().to_owned()
            } else {
                s.paragraph_text.trim().to_owned()
            };
            if !s.in_cell && !text.is_empty() {
                let style = s.paragraph_style.clone();
                let list = s.paragraph_list.clone();
                let prefix = paragraph_prefix(style.as_deref(), list.as_ref(), s);
                s.markdown.push_str(&format!("{prefix}{text}\n\n"));
            }
            if let Some(style_id) = s.paragraph_style.take() {
                let name = styles
                    .get(&style_id)
                    .cloned()
                    .unwrap_or_else(|| style_id.clone());
                if !is_supported_style(&style_id, &name) {
                    s.findings.push(finding(
                        "styles",
                        Severity::Warning,
                        format!("Paragraph uses the non-default style “{name}”."),
                        "Check emphasis, spacing, and meaning in the Markdown.",
                        s,
                    ));
                }
            }
            s.paragraph_list = None;
            s.in_paragraph = false;
        }
        b"tc" => {
            s.table_row.push(s.cell_text.trim().replace('\n', " "));
            s.cell_text.clear();
            s.in_cell = false;
        }
        b"tr" => s.table_rows.push(std::mem::take(&mut s.table_row)),
        b"tbl" => {
            render_table(s);
            s.in_table = false;
        }
        _ => {}
    }
}

fn paragraph_prefix(
    style: Option<&str>,
    list: Option<&ParagraphList>,
    s: &mut ParseState,
) -> String {
    if let Some(list) = list {
        let indent = "    ".repeat(list.level);
        return match list.format.as_deref() {
            Some("bullet") => format!("{indent}- "),
            Some("decimal") => format!("{indent}{}. ", next_list_number(s, list)),
            Some(format) => {
                s.findings.push(finding(
                    "lists",
                    Severity::Warning,
                    format!(
                        "List format `{}` was rendered as a decimal Markdown list.",
                        safe_display_name(format)
                    ),
                    "Check list markers and nesting against the Word document.",
                    s,
                ));
                format!("{indent}{}. ", next_list_number(s, list))
            }
            None => {
                let num_id = list.num_id.as_deref().unwrap_or("unknown");
                s.findings.push(finding(
                    "lists",
                    Severity::Warning,
                    format!(
                        "List numId `{}` at level {} has no recognized numbering definition; it was rendered as a bullet list.",
                        safe_display_name(num_id),
                        list.level
                    ),
                    "Check list order, marker type, and nesting against the Word document.",
                    s,
                ));
                format!("{indent}- ")
            }
        };
    }
    match style.unwrap_or("").to_ascii_lowercase().as_str() {
        "heading1" | "title" => "# ".into(),
        "heading2" => "## ".into(),
        "heading3" => "### ".into(),
        "heading4" => "#### ".into(),
        "heading5" => "##### ".into(),
        "heading6" => "###### ".into(),
        _ => "".into(),
    }
}

fn next_list_number(s: &mut ParseState, list: &ParagraphList) -> usize {
    let key = (
        list.num_id.clone().unwrap_or_else(|| "unknown".into()),
        list.level,
    );
    let next = s.list_counters.entry(key).or_insert(0);
    if *next == 0 {
        *next = list.start.saturating_sub(1);
    }
    *next += 1;
    *next
}

fn start_list(s: &mut ParseState) {
    s.paragraph_list = Some(ParagraphList {
        num_id: None,
        level: 0,
        format: None,
        start: 1,
    });
}

fn set_list_num_id(e: &BytesStart<'_>, s: &mut ParseState, numbering: &Numbering) {
    if s.paragraph_list.is_none() {
        start_list(s);
    }
    let num_id = attr_local(e, b"val");
    if num_id.as_deref() == Some("0") {
        s.paragraph_list = None;
        return;
    }
    if let Some(list) = s.paragraph_list.as_mut() {
        list.num_id = num_id;
    }
    resolve_list_format(s, numbering);
}

fn set_list_level(e: &BytesStart<'_>, s: &mut ParseState, numbering: &Numbering) {
    if s.paragraph_list.is_none() {
        start_list(s);
    }
    if let Some(list) = s.paragraph_list.as_mut() {
        list.level = attr_local(e, b"val")
            .and_then(|value| value.parse().ok())
            .unwrap_or(0);
    }
    resolve_list_format(s, numbering);
}

fn resolve_list_format(s: &mut ParseState, numbering: &Numbering) {
    let Some(list) = s.paragraph_list.as_mut() else {
        return;
    };
    let definition = list
        .num_id
        .as_deref()
        .and_then(|num_id| numbering.definition(num_id, list.level));
    list.format = definition
        .as_ref()
        .map(|definition| definition.format.clone());
    list.start = definition.map(|definition| definition.start).unwrap_or(1);
}

fn render_table(s: &mut ParseState) {
    if s.table_rows.is_empty() {
        return;
    }
    let columns = s.table_rows.iter().map(Vec::len).max().unwrap_or(0);
    if columns == 0 {
        return;
    }
    s.markdown.push('|');
    for i in 0..columns {
        s.markdown.push_str(&format!(
            " {} |",
            s.table_rows[0].get(i).cloned().unwrap_or_default()
        ));
    }
    s.markdown.push('\n');
    s.markdown.push('|');
    for _ in 0..columns {
        s.markdown.push_str(" --- |");
    }
    s.markdown.push('\n');
    for row in s.table_rows.iter().skip(1) {
        s.markdown.push('|');
        for i in 0..columns {
            s.markdown
                .push_str(&format!(" {} |", row.get(i).cloned().unwrap_or_default()));
        }
        s.markdown.push('\n');
    }
    s.markdown.push('\n');
    let rows = s.table_rows.len();
    s.findings.push(finding(
        "tables",
        Severity::Warning,
        format!(
            "Table {} was flattened to a {rows}×{columns} Markdown table.",
            s.table
        ),
        "Check merged cells, widths, borders, and reading order.",
        s,
    ));
}

fn revision_finding(name: &[u8], e: &BytesStart<'_>, s: &mut ParseState) {
    let id = attr_local(e, b"id").unwrap_or_else(|| format!("at-{}", s.paragraph));
    let key = format!("{}:{id}", String::from_utf8_lossy(name));
    if s.revision_seen.insert(key) {
        let kind = match name {
            b"ins" => "insertion",
            b"del" => "deletion",
            b"moveFrom" => "move-from revision",
            b"moveTo" => "move-to revision",
            _ => "format revision",
        };
        s.findings.push(finding(
            "revisions",
            Severity::Warning,
            format!("Tracked {kind} {id} is present."),
            "Confirm the accepted text before publishing.",
            s,
        ));
    }
}

fn comment_finding(e: &BytesStart<'_>, s: &mut ParseState, comments: &HashMap<String, String>) {
    let id = attr_local(e, b"id").unwrap_or_else(|| "unknown".into());
    if s.comment_seen.insert(id.clone()) {
        let note = comments
            .get(&id)
            .filter(|x| !x.is_empty())
            .map(|x| format!(" “{}”", truncate(x, 100)))
            .unwrap_or_default();
        s.findings.push(finding(
            "comments",
            Severity::Warning,
            format!("Comment {id} is not carried into Markdown.{note}"),
            "Resolve or copy this comment before publishing.",
            s,
        ));
    }
}

fn embedded_finding(name: &[u8], s: &mut ParseState) {
    let kind = String::from_utf8_lossy(name);
    let key = format!(
        "{}:{}",
        s.paragraph,
        if kind == "oleObject" { "object" } else { &kind }
    );
    if !s.embedded_seen.insert(key) {
        return;
    }
    s.findings.push(finding(
        "embedded_objects",
        Severity::Error,
        format!("Embedded object `{kind}` cannot be converted."),
        "Open the DOCX safely and replace the object with a supported file or text.",
        s,
    ));
}

fn inline_loss_finding(kind: &str, s: &mut ParseState) {
    let key = format!("{kind}:{}", s.paragraph);
    if !s.unsupported_seen.insert(key) {
        return;
    }
    let (category, severity, summary, action) = match kind {
        "underline" => (
            "styles",
            Severity::Warning,
            "Underlined text has no direct Markdown equivalent.",
            "Check whether emphasis or a link was intended.",
        ),
        "equation" => (
            "embedded_objects",
            Severity::Error,
            "An equation cannot be converted to Markdown notation.",
            "Rewrite the equation in a supported math format.",
        ),
        "text box" => (
            "styles",
            Severity::Warning,
            "A text box may change reading order in Markdown.",
            "Check this paragraph against the visual Word layout.",
        ),
        "field" => (
            "styles",
            Severity::Warning,
            "A Word field was reduced to its displayed text.",
            "Check dates, references, and generated values.",
        ),
        _ => (
            "images",
            Severity::Warning,
            "A legacy drawing may not retain its layout.",
            "Check the drawing and its position.",
        ),
    };
    s.findings
        .push(finding(category, severity, summary, action, s));
}

fn hyperlink_start(e: &BytesStart<'_>, s: &mut ParseState, rels: &HashMap<String, String>) {
    let target = attr_local(e, b"id")
        .and_then(|id| rels.get(&id).cloned())
        .or_else(|| attr_local(e, b"anchor").map(|anchor| format!("#{anchor}")));
    let Some(target) = target else {
        return;
    };
    if !(target.starts_with("https://")
        || target.starts_with("http://")
        || target.starts_with("mailto:")
        || target.starts_with('#'))
    {
        s.findings.push(finding(
            "embedded_objects",
            Severity::Error,
            format!(
                "Link target `{}` uses an unsafe or unsupported scheme.",
                truncate(&target, 80)
            ),
            "Replace the link with an HTTPS, HTTP, email, or document anchor target.",
            s,
        ));
        return;
    }
    let start = if s.in_cell {
        s.cell_text.len()
    } else {
        s.paragraph_text.len()
    };
    s.hyperlink = Some((start, target, s.in_cell));
}

fn finish_hyperlink(s: &mut ParseState) {
    let Some((start, target, in_cell)) = s.hyperlink.take() else {
        return;
    };
    let buffer = if in_cell {
        &mut s.cell_text
    } else {
        &mut s.paragraph_text
    };
    if start > buffer.len() {
        return;
    }
    let label = buffer[start..].to_owned();
    buffer.truncate(start);
    buffer.push_str(&format!(
        "[{label}]({})",
        target.replace(' ', "%20").replace(')', "%29")
    ));
}

fn finish_run(s: &mut ParseState) {
    let Some((start, in_cell)) = s.run_start.take() else {
        return;
    };
    if !s.run_bold && !s.run_italic {
        return;
    }
    let buffer = if in_cell {
        &mut s.cell_text
    } else {
        &mut s.paragraph_text
    };
    if start >= buffer.len() {
        return;
    }
    let content = buffer[start..].to_owned();
    buffer.truncate(start);
    let marker = if s.run_bold && s.run_italic {
        "***"
    } else if s.run_bold {
        "**"
    } else {
        "*"
    };
    buffer.push_str(marker);
    buffer.push_str(&content);
    buffer.push_str(marker);
}

fn footnote_finding(e: &BytesStart<'_>, s: &mut ParseState) {
    let id = attr_local(e, b"id").unwrap_or_else(|| "unknown".into());
    if !s.footnote_refs.contains(&id) {
        s.footnote_refs.push(id.clone());
    }
    let marker = format!("[^{id}]");
    if s.in_cell {
        s.cell_text.push_str(&marker)
    } else {
        s.paragraph_text.push_str(&marker)
    }
    s.findings.push(finding(
        "footnotes",
        Severity::Info,
        format!("Footnote {id} was moved to a Markdown note."),
        "Check note numbering and backlinks.",
        s,
    ));
}

fn image_finding<R: Read + Seek>(
    e: &BytesStart<'_>,
    s: &mut ParseState,
    rels: &HashMap<String, String>,
    archive: &mut ZipArchive<R>,
    media_dir: &Path,
) -> Result<()> {
    s.image_index += 1;
    let id = attr_local(e, b"embed")
        .or_else(|| attr_local(e, b"link"))
        .or_else(|| attr_local(e, b"id"));
    let Some(id) = id else {
        s.findings.push(finding(
            "images",
            Severity::Error,
            "Image has no package relationship.",
            "Replace or relink the image in Word.",
            s,
        ));
        return Ok(());
    };
    let Some(target) = rels.get(&id) else {
        s.findings.push(finding(
            "images",
            Severity::Error,
            format!("Image relationship {id} is missing."),
            "Replace or relink the image in Word.",
            s,
        ));
        return Ok(());
    };
    let archive_name = normalize_word_target(target);
    let file_name = safe_media_name(
        Path::new(target)
            .file_name()
            .and_then(|x| x.to_str())
            .unwrap_or("image.bin"),
        s.image_index,
    );
    let output = media_dir.join(&file_name);
    match archive.by_name(&archive_name) {
        Ok(mut entry) => {
            let mut out = File::create(&output)?;
            std::io::copy(&mut entry, &mut out)?;
            let reference = format!(
                "![]({}/{})",
                media_dir.file_name().unwrap().to_string_lossy(),
                file_name
            );
            if s.in_cell {
                s.cell_text.push_str(&reference)
            } else {
                s.paragraph_text.push_str(&reference)
            }
            s.findings.push(finding(
                "images",
                Severity::Info,
                format!("Image {id} was extracted as `{file_name}`."),
                "Add useful alt text and check the image position.",
                s,
            ));
        }
        Err(_) => s.findings.push(finding(
            "images",
            Severity::Error,
            format!("Image {id} points to missing part `{archive_name}`."),
            "Replace or relink the image in Word.",
            s,
        )),
    }
    Ok(())
}

fn append_package_findings<R: Read + Seek>(
    archive: &mut ZipArchive<R>,
    s: &mut ParseState,
) -> Result<()> {
    let mut names = Vec::new();
    for i in 0..archive.len() {
        names.push(archive.by_index(i)?.name().to_owned());
    }
    for name in names {
        if name.starts_with("word/embeddings/") && !name.ends_with('/') {
            s.findings.push(Finding {
                category: "embedded_objects".into(),
                severity: Severity::Error,
                summary: format!(
                    "Package contains embedded file `{}`.",
                    safe_display_name(&name)
                ),
                action: "Inspect it separately. This tool never opens embedded files.".into(),
                location: SourceLocation {
                    part: name,
                    paragraph: None,
                    table: None,
                    row: None,
                    cell: None,
                },
            });
        } else if name == "word/vbaProject.bin" {
            s.findings.push(Finding {
                category: "embedded_objects".into(),
                severity: Severity::Error,
                summary: "Package contains a macro project.".into(),
                action: "Review macros separately. This tool never runs them.".into(),
                location: SourceLocation {
                    part: name,
                    paragraph: None,
                    table: None,
                    row: None,
                    cell: None,
                },
            });
        } else if (name.starts_with("word/header") || name.starts_with("word/footer"))
            && name.ends_with(".xml")
        {
            s.findings.push(package_finding(
                "styles",
                Severity::Warning,
                &name,
                "Header or footer content is not included in Markdown.",
                "Copy any required running text into the document body.",
            ));
        } else if name == "word/endnotes.xml" {
            s.findings.push(package_finding(
                "footnotes",
                Severity::Warning,
                &name,
                "Endnotes are not included in Markdown.",
                "Copy or convert the endnotes before publishing.",
            ));
        } else if (name.starts_with("word/charts/")
            || name.starts_with("word/diagrams/")
            || name.starts_with("word/activeX/")
            || name.starts_with("customXml/"))
            && !name.ends_with('/')
        {
            s.findings.push(package_finding(
                "embedded_objects",
                Severity::Error,
                &name,
                "A non-text package part cannot be converted.",
                "Inspect this part separately and replace it with text or an image.",
            ));
        }
    }
    Ok(())
}

fn package_finding(
    category: &str,
    severity: Severity,
    part: &str,
    summary: &str,
    action: &str,
) -> Finding {
    Finding {
        category: category.into(),
        severity,
        summary: summary.into(),
        action: action.into(),
        location: SourceLocation {
            part: part.into(),
            paragraph: None,
            table: None,
            row: None,
            cell: None,
        },
    }
}

fn build_report(input: &Path, findings: Vec<Finding>) -> FidelityReport {
    let mut counts = BTreeMap::new();
    for f in &findings {
        *counts.entry(f.category.clone()).or_insert(0) += 1;
    }
    for category in [
        "tables",
        "comments",
        "revisions",
        "embedded_objects",
        "footnotes",
        "styles",
        "images",
    ] {
        counts.entry(category.into()).or_insert(0);
    }
    let status = if findings.iter().any(|f| f.severity == Severity::Error) {
        ReportStatus::Blocked
    } else if findings.iter().any(|f| f.severity == Severity::Warning) {
        ReportStatus::Review
    } else {
        ReportStatus::Clear
    };
    FidelityReport {
        schema_version: 1,
        source: input
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .into_owned(),
        status,
        counts,
        findings,
    }
}

fn render_report(report: &FidelityReport) -> String {
    let mut out = format!(
        "# Fidelity report: {}\n\n**Status:** {:?}\n\n",
        report.source, report.status
    );
    out.push_str("## Ledger\n\n| Category | Count |\n| --- | ---: |\n");
    for (category, count) in &report.counts {
        out.push_str(&format!("| {} | {} |\n", category.replace('_', " "), count));
    }
    out.push_str("\n## Review checklist\n\n");
    if report.findings.is_empty() {
        out.push_str("No fidelity risks were found. Review the Markdown before publishing.\n");
    }
    for finding in &report.findings {
        out.push_str(&format!(
            "- [ ] **{:?} · {} · {}** — {} _{}._\n",
            finding.severity,
            finding.category.replace('_', " "),
            location_label(&finding.location),
            finding.summary,
            finding.action
        ));
    }
    out
}

fn finding(
    category: &str,
    severity: Severity,
    summary: impl Into<String>,
    action: impl Into<String>,
    s: &ParseState,
) -> Finding {
    Finding {
        category: category.into(),
        severity,
        summary: summary.into(),
        action: action.into(),
        location: SourceLocation {
            part: "word/document.xml".into(),
            paragraph: Some(s.paragraph),
            table: s.in_table.then_some(s.table),
            row: s.in_table.then_some(s.row),
            cell: s.in_cell.then_some(s.cell),
        },
    }
}

fn location_label(l: &SourceLocation) -> String {
    let mut bits = vec![l.part.clone()];
    if let Some(x) = l.paragraph {
        bits.push(format!("paragraph {x}"));
    }
    if let Some(x) = l.table {
        bits.push(format!("table {x}"));
    }
    if let Some(x) = l.row {
        bits.push(format!("row {x}"));
    }
    if let Some(x) = l.cell {
        bits.push(format!("cell {x}"));
    }
    bits.join(", ")
}

fn attr(e: &BytesStart<'_>, wanted: &[u8]) -> Option<String> {
    e.attributes()
        .flatten()
        .find(|a| a.key.as_ref() == wanted)
        .and_then(|a| String::from_utf8(a.value.into_owned()).ok())
}

fn attr_local(e: &BytesStart<'_>, wanted: &[u8]) -> Option<String> {
    e.attributes()
        .flatten()
        .find(|a| local_name(a.key.as_ref()) == wanted)
        .and_then(|a| String::from_utf8(a.value.into_owned()).ok())
}

fn attr_is_on(e: &BytesStart<'_>) -> bool {
    !matches!(
        attr_local(e, b"val").as_deref(),
        Some("0" | "false" | "off")
    )
}

fn local_name(name: &[u8]) -> &[u8] {
    name.rsplit(|b| *b == b':').next().unwrap_or(name)
}
fn safe_stem(name: &str) -> String {
    let cleaned: String = name
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || matches!(c, '-' | '_') {
                c
            } else {
                '-'
            }
        })
        .collect();
    let trimmed = cleaned.trim_matches('-');
    if trimmed.is_empty() {
        "document".into()
    } else {
        trimmed.chars().take(80).collect()
    }
}
fn safe_media_name(name: &str, index: usize) -> String {
    let ext = Path::new(name)
        .extension()
        .and_then(|x| x.to_str())
        .unwrap_or("bin")
        .to_ascii_lowercase();
    let allowed = [
        "png", "jpg", "jpeg", "gif", "webp", "svg", "emf", "wmf", "tif", "tiff",
    ];
    let ext = if allowed.contains(&ext.as_str()) {
        ext
    } else {
        "bin".into()
    };
    format!("image-{index:03}.{ext}")
}
fn normalize_word_target(target: &str) -> String {
    let path = Path::new("word").join(target);
    let mut parts = Vec::new();
    for c in path.components() {
        match c {
            Component::Normal(p) => parts.push(p.to_string_lossy().into_owned()),
            Component::ParentDir => {
                parts.pop();
            }
            _ => {}
        }
    }
    parts.join("/")
}
fn is_supported_style(id: &str, name: &str) -> bool {
    let id = id.to_ascii_lowercase();
    let name = name.to_ascii_lowercase();
    id == "normal"
        || id == "title"
        || id.starts_with("heading")
        || id.starts_with("list")
        || name == "normal"
        || name.starts_with("heading")
        || name.starts_with("list")
}
/// Escape text that came from a Word text node before it is inserted into
/// converter-authored Markdown. Formatting, links, image references, and
/// headings are composed later, so their Markdown syntax remains active while
/// a literal Word paragraph can never become a heading, link, list, quote,
/// code span, or table by accident.
fn escape_markdown_source(text: &str) -> String {
    let mut escaped = String::with_capacity(text.len());
    for character in text.chars() {
        if character == '\0' {
            continue;
        }
        if matches!(
            character,
            '\\' | '`'
                | '*'
                | '_'
                | '{'
                | '}'
                | '['
                | ']'
                | '<'
                | '>'
                | '#'
                | '+'
                | '-'
                | '.'
                | '!'
                | '|'
                | '~'
                | '='
                | '('
                | ')'
        ) {
            escaped.push('\\');
        }
        escaped.push(character);
    }
    escaped
}
fn truncate(text: &str, max: usize) -> String {
    if text.chars().count() <= max {
        text.into()
    } else {
        format!("{}…", text.chars().take(max).collect::<String>())
    }
}
fn safe_display_name(name: &str) -> String {
    name.chars().filter(|c| !c.is_control()).take(120).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::tempdir;
    use zip::write::SimpleFileOptions;

    fn write_docx(path: &Path, document_xml: &str, parts: &[(&str, &[u8])]) {
        let file = File::create(path).unwrap();
        let mut archive = zip::ZipWriter::new(file);
        let options = SimpleFileOptions::default();
        archive.start_file("word/document.xml", options).unwrap();
        archive.write_all(document_xml.as_bytes()).unwrap();
        for (name, contents) in parts {
            archive.start_file(*name, options).unwrap();
            archive.write_all(contents).unwrap();
        }
        archive.finish().unwrap();
    }

    fn document_with_text(text: &str) -> String {
        format!(
            r#"<w:document xmlns:w="urn:word"><w:body><w:p><w:r><w:t>{text}</w:t></w:r></w:p></w:body></w:document>"#
        )
    }

    #[test]
    fn safe_names_cannot_escape() {
        assert_eq!(safe_stem("../../Quarter 1"), "Quarter-1");
        assert_eq!(safe_media_name("../../payload.exe", 2), "image-002.bin");
    }

    #[test]
    fn relationship_targets_stay_in_word() {
        assert_eq!(
            normalize_word_target("media/image1.png"),
            "word/media/image1.png"
        );
        assert_eq!(
            normalize_word_target("../custom/item.xml"),
            "custom/item.xml"
        );
    }

    #[test]
    fn batch_output_names_are_disambiguated_before_writing() {
        let root = tempdir().unwrap();
        let input = root.path().join("input");
        let output = root.path().join("output");
        fs::create_dir(&input).unwrap();
        write_docx(
            &input.join("Plan Q1.docx"),
            &document_with_text("FIRST"),
            &[],
        );
        write_docx(
            &input.join("Plan-Q1.docx"),
            &document_with_text("SECOND"),
            &[],
        );

        let results = convert_paths(&input, &output, &ConvertOptions { overwrite: true }).unwrap();

        assert_eq!(results.len(), 2);
        assert_ne!(results[0].markdown_path, results[1].markdown_path);
        let markdown: HashSet<_> = results
            .iter()
            .map(|result| fs::read_to_string(&result.markdown_path).unwrap())
            .collect();
        assert_eq!(
            markdown,
            HashSet::from(["FIRST\n".into(), "SECOND\n".into()])
        );
    }

    #[test]
    fn incomplete_document_xml_fails_without_outputs() {
        let root = tempdir().unwrap();
        let input = root.path().join("broken.docx");
        let output = root.path().join("output");
        write_docx(&input, "<w:document><broken>", &[]);

        let error = convert_path(&input, &output, &ConvertOptions::default()).unwrap_err();

        assert!(format!("{error:#}").contains("word/document.xml is malformed"));
        assert!(!output.join("broken.md").exists());
        assert!(!output.join("broken.fidelity.json").exists());
    }

    #[test]
    fn overwrite_removes_media_from_the_previous_document() {
        let root = tempdir().unwrap();
        let input = root.path().join("guide.docx");
        let output = root.path().join("output");
        let relationships =
            br#"<Relationships><Relationship Id="rId1" Target="media/old.svg"/></Relationships>"#;
        let with_image = r#"<w:document xmlns:w="urn:word" xmlns:a="urn:drawing" xmlns:r="urn:rels"><w:body><w:p><w:r><a:blip r:embed="rId1"/></w:r></w:p></w:body></w:document>"#;
        write_docx(
            &input,
            with_image,
            &[
                ("word/_rels/document.xml.rels", relationships),
                ("word/media/old.svg", b"<svg/>"),
            ],
        );
        convert_path(&input, &output, &ConvertOptions::default()).unwrap();
        assert!(output.join("guide.media/image-001.svg").exists());

        write_docx(&input, &document_with_text("NO IMAGE"), &[]);
        convert_path(&input, &output, &ConvertOptions { overwrite: true }).unwrap();

        assert!(!output.join("guide.media").exists());
        assert_eq!(
            fs::read_to_string(output.join("guide.md")).unwrap(),
            "NO IMAGE\n"
        );
    }

    #[test]
    fn embedded_objects_are_reported_but_never_extracted() {
        let root = tempdir().unwrap();
        let input = root.path().join("embedded.docx");
        let output = root.path().join("output");
        let document = r#"<w:document xmlns:w="urn:word"><w:body><w:p><w:object/></w:p></w:body></w:document>"#;
        write_docx(
            &input,
            document,
            &[("word/embeddings/payload.bin", b"DO NOT EXTRACT")],
        );

        let result = convert_path(&input, &output, &ConvertOptions::default()).unwrap();

        assert_eq!(result.report.counts["embedded_objects"], 2);
        assert!(!output.join("embedded.media").exists());
        assert!(!output.join("payload.bin").exists());
    }

    #[test]
    fn plain_word_markdown_syntax_is_escaped_without_muting_converter_markup() {
        let root = tempdir().unwrap();
        let input = root.path().join("plain-markdown-syntax.docx");
        let output = root.path().join("output");
        let document = r#"<w:document xmlns:w="urn:word"><w:body>
          <w:p><w:r><w:t># Plain Word paragraph</w:t></w:r></w:p>
          <w:p><w:r><w:t>[payroll](https://attacker.example)</w:t></w:r></w:p>
          <w:p><w:r><w:t>- Plain dash paragraph</w:t></w:r></w:p>
          <w:p><w:r><w:t>`literal code`</w:t></w:r></w:p>
          <w:p><w:r><w:t>&gt; Plain quote</w:t></w:r></w:p>
          <w:p><w:r><w:t>| Source | table syntax |</w:t></w:r></w:p>
          <w:p><w:r><w:t>1. Plain ordered marker</w:t></w:r></w:p>
          <w:p><w:pPr><w:pStyle w:val="Heading1"/></w:pPr><w:r><w:t>Semantic heading</w:t></w:r></w:p>
        </w:body></w:document>"#;
        write_docx(&input, document, &[]);

        let result = convert_path(&input, &output, &ConvertOptions::default()).unwrap();
        let markdown = fs::read_to_string(&result.markdown_path).unwrap();

        assert!(markdown.contains(r"\# Plain Word paragraph"));
        assert!(markdown.contains(r"\[payroll\]\(https://attacker\.example\)"));
        assert!(markdown.contains(r"\- Plain dash paragraph"));
        assert!(markdown.contains(r"\`literal code\`"));
        assert!(markdown.contains(r"\> Plain quote"));
        assert!(markdown.contains(r"\| Source \| table syntax \|"));
        assert!(markdown.contains(r"1\. Plain ordered marker"));
        assert!(!markdown.contains("\n# Plain Word paragraph"));
        assert!(markdown.contains("# Semantic heading"));
        assert_eq!(result.report.status, ReportStatus::Clear);
        assert!(result.report.findings.is_empty());
    }

    #[test]
    fn decimal_numbering_and_nesting_are_preserved_from_numbering_xml() {
        let root = tempdir().unwrap();
        let input = root.path().join("decimal-list.docx");
        let output = root.path().join("output");
        let document = r#"<w:document xmlns:w="urn:word"><w:body>
          <w:p><w:pPr><w:numPr><w:ilvl w:val="0"/><w:numId w:val="42"/></w:numPr></w:pPr><w:r><w:t>First required step</w:t></w:r></w:p>
          <w:p><w:pPr><w:numPr><w:ilvl w:val="0"/><w:numId w:val="42"/></w:numPr></w:pPr><w:r><w:t>Second required step</w:t></w:r></w:p>
          <w:p><w:pPr><w:numPr><w:ilvl w:val="1"/><w:numId w:val="42"/></w:numPr></w:pPr><w:r><w:t>Nested check</w:t></w:r></w:p>
        </w:body></w:document>"#;
        let numbering = br#"<w:numbering xmlns:w="urn:word">
          <w:abstractNum w:abstractNumId="7"><w:lvl w:ilvl="0"><w:start w:val="3"/><w:numFmt w:val="decimal"/></w:lvl><w:lvl w:ilvl="1"><w:numFmt w:val="bullet"/></w:lvl></w:abstractNum>
          <w:num w:numId="42"><w:abstractNumId w:val="7"/></w:num>
        </w:numbering>"#;
        write_docx(&input, document, &[("word/numbering.xml", numbering)]);

        let result = convert_path(&input, &output, &ConvertOptions::default()).unwrap();
        let markdown = fs::read_to_string(&result.markdown_path).unwrap();

        assert!(markdown.contains("3. First required step\n\n4. Second required step"));
        assert!(markdown.contains("    - Nested check"));
        assert!(!markdown.contains("- First required step"));
        assert_eq!(result.report.status, ReportStatus::Clear);
        assert!(result.report.findings.is_empty());
    }

    #[test]
    fn unknown_numbering_is_reported_at_its_source_location() {
        let root = tempdir().unwrap();
        let input = root.path().join("unknown-list.docx");
        let output = root.path().join("output");
        let document = r#"<w:document xmlns:w="urn:word"><w:body><w:p><w:pPr><w:numPr><w:numId w:val="999"/></w:numPr></w:pPr><w:r><w:t>Check this marker</w:t></w:r></w:p></w:body></w:document>"#;
        write_docx(&input, document, &[]);

        let result = convert_path(&input, &output, &ConvertOptions::default()).unwrap();

        assert_eq!(result.report.status, ReportStatus::Review);
        assert!(result.report.findings.iter().any(|finding| {
            finding.category == "lists"
                && finding.location.part == "word/document.xml"
                && finding.location.paragraph == Some(1)
        }));
    }
}
