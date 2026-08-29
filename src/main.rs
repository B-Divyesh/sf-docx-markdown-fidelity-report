use anyhow::{bail, Context, Result};
use clap::{Parser, Subcommand, ValueEnum};
use docx_fidelity::{convert_paths, ConversionResult, ConvertOptions, Severity};
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::time::{SystemTime, UNIX_EPOCH};

const DEMO_DOCX: &[u8] = include_bytes!("../examples/field-guide.docx");
const VERIFY_URL: &str =
    "https://api.sociobot.in/api/v1/products/docx-markdown-fidelity-report/verify";

#[derive(Parser)]
#[command(name = "docx-fidelity", version, about = "Convert DOCX to Markdown and list fidelity issues", long_about = None)]
struct Cli {
    /// Print one JSON command summary to stdout.
    #[arg(long, global = true)]
    json: bool,
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Convert one DOCX or every DOCX in a directory.
    Convert {
        /// A .docx file or a directory containing .docx files.
        input: PathBuf,
        /// Directory for Markdown, reports, and extracted media.
        #[arg(short, long, default_value = "docx-fidelity-output")]
        output: PathBuf,
        /// Replace existing output files.
        #[arg(long)]
        overwrite: bool,
        /// Exit 3 when a finding meets this level. Runs locally.
        #[arg(long, value_enum)]
        fail_on: Option<FailOn>,
    },
    /// Convert the bundled complex sample in a fresh temporary directory.
    Demo,
    /// Check an existing license token without sending document data.
    License {
        #[command(subcommand)]
        command: LicenseCommand,
    },
}

#[derive(Subcommand)]
enum LicenseCommand {
    /// Verify a token with Sociobot.
    Verify { token: String },
}

#[derive(Clone, Copy, Debug, ValueEnum)]
enum FailOn {
    Warning,
    Error,
}

#[derive(Serialize)]
struct CommandSummary<'a> {
    status: &'a str,
    converted: usize,
    outputs: Vec<OutputSummary>,
}

#[derive(Serialize)]
struct OutputSummary {
    source: String,
    markdown: PathBuf,
    report: PathBuf,
    status: String,
    findings: usize,
}

#[derive(Debug, Deserialize, Serialize)]
struct LicenseVerdict {
    valid: bool,
    reason: String,
    expires_at: Option<String>,
}

fn main() -> ExitCode {
    match run() {
        Ok(code) => ExitCode::from(code),
        Err(error) => {
            eprintln!("Error: {error:#}\nNext: run `docx-fidelity --help` and check the input and output paths.");
            ExitCode::from(2)
        }
    }
}

fn run() -> Result<u8> {
    let cli = Cli::parse();
    match cli.command {
        Command::Convert {
            input,
            output,
            overwrite,
            fail_on,
        } => {
            let results = convert_all(&input, &output, overwrite)?;
            print_results(&results, cli.json)?;
            if let Some(level) = fail_on {
                let tripped =
                    results
                        .iter()
                        .flat_map(|r| &r.report.findings)
                        .any(|f| match level {
                            FailOn::Warning => f.severity >= Severity::Warning,
                            FailOn::Error => f.severity >= Severity::Error,
                        });
                if tripped {
                    return Ok(3);
                }
            }
            Ok(0)
        }
        Command::Demo => {
            let dir = demo_directory()?;
            let input = dir.join("field-guide.docx");
            let output = dir.join("report");
            fs::write(&input, DEMO_DOCX)?;
            let results = convert_all(&input, &output, false)?;
            if cli.json {
                print_results(&results, true)?;
            } else {
                println!("Demo — sample data, nothing was read from your files");
                println!("Converted: {}", input.display());
                println!("Markdown: {}", results[0].markdown_path.display());
                println!(
                    "Fidelity report: {}",
                    results[0].markdown_report_path.display()
                );
                println!("Sandbox: {}", dir.display());
            }
            Ok(0)
        }
        Command::License {
            command: LicenseCommand::Verify { token },
        } => {
            let verdict = match verify_license(&token) {
                Ok(verdict) => verdict,
                Err(error) => {
                    eprintln!("License could not be checked: {error:#}");
                    return Ok(4);
                }
            };
            if cli.json {
                println!("{}", serde_json::to_string(&verdict)?);
            } else if verdict.valid {
                println!("License is active.");
            } else {
                println!("License is not active: {}.", verdict.reason);
            }
            Ok(if verdict.valid { 0 } else { 4 })
        }
    }
}

fn convert_all(input: &Path, output: &Path, overwrite: bool) -> Result<Vec<ConversionResult>> {
    let options = ConvertOptions { overwrite };
    convert_paths(input, output, &options)
}

fn print_results(results: &[ConversionResult], json: bool) -> Result<()> {
    if json {
        let outputs = results
            .iter()
            .map(|r| OutputSummary {
                source: r.report.source.clone(),
                markdown: r.markdown_path.clone(),
                report: r.json_report_path.clone(),
                status: format!("{:?}", r.report.status).to_ascii_lowercase(),
                findings: r.report.findings.len(),
            })
            .collect();
        println!(
            "{}",
            serde_json::to_string(&CommandSummary {
                status: "converted",
                converted: results.len(),
                outputs
            })?
        );
    } else {
        for r in results {
            println!(
                "Converted {} → {}",
                r.input.display(),
                r.markdown_path.display()
            );
            println!(
                "Report: {} ({:?}, {} findings)",
                r.markdown_report_path.display(),
                r.report.status,
                r.report.findings.len()
            );
        }
    }
    Ok(())
}

fn demo_directory() -> Result<PathBuf> {
    let stamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
    let dir = env::temp_dir().join(format!("docx-fidelity-demo-{}-{stamp}", std::process::id()));
    if dir.exists() {
        bail!("demo directory already exists: {}", dir.display());
    }
    fs::create_dir_all(&dir)?;
    Ok(dir)
}

fn verify_license(token: &str) -> Result<LicenseVerdict> {
    if token.trim().is_empty() {
        bail!("license token is empty");
    }
    let url = env::var("DOCX_FIDELITY_VERIFY_URL").unwrap_or_else(|_| VERIFY_URL.into());
    let response = ureq::get(&url)
        .query("license", token)
        .timeout(std::time::Duration::from_secs(8))
        .call()
        .context("license check failed; check the connection and try again")?;
    response
        .into_json()
        .context("license service returned an unreadable response")
}
