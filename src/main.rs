use anyhow::{Context, Result, bail};
use clap::{Parser, Subcommand};
use git_forge_exit_drill::{
    DrillResult, create_demo_git_mirror, inventory_github, inventory_local, load_mappings,
    read_encrypted_archive, run_drill, verify_team_license, write_portfolio,
};
use serde::Serialize;
use std::{
    env,
    ffi::OsStr,
    fs,
    path::{Path, PathBuf},
    process::ExitCode,
};

#[derive(Parser)]
#[command(
    name = "git-forge-exit-drill",
    version,
    about = "Test a GitHub move before your team cuts over",
    long_about = "Test a GitHub move before your team cuts over. Inventory an authorized export or API repository, encrypt the evidence, map target gaps, and write a restore drill. No data is changed on either forge."
)]
struct Cli {
    /// Print the command result as JSON for scripts.
    #[arg(long, global = true)]
    json: bool,
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Inventory one repository and write its encrypted evidence and reports.
    Drill {
        /// Extracted GitHub export directory. Cannot be used with --repo.
        #[arg(long, conflicts_with = "repo", required_unless_present = "repo")]
        source: Option<PathBuf>,
        /// GitHub repository in owner/name form. Cannot be used with --source.
        #[arg(long, conflicts_with = "source", required_unless_present = "source")]
        repo: Option<String>,
        /// Target mapping, such as forgejo:9.0, gitea:1.22, or gitlab:17.0.
        #[arg(long)]
        target: String,
        /// New or existing directory for evidence.gfed and readiness reports. Must be outside --source.
        #[arg(long, default_value = "exit-drill-output")]
        output: PathBuf,
        /// Environment variable that holds the read-only GitHub token.
        #[arg(long, default_value = "GITHUB_TOKEN")]
        token_env: String,
        /// Environment variable that holds the archive passphrase.
        #[arg(long, default_value = "GFED_PASSPHRASE")]
        passphrase_env: String,
    },
    /// Run a complete drill with bundled sample data in a temporary directory.
    Demo {
        /// Keep demo outputs in this directory instead of a temporary directory.
        #[arg(long)]
        output: Option<PathBuf>,
        /// Target mapping for the sample drill.
        #[arg(long, default_value = "forgejo:9.0")]
        target: String,
    },
    /// Open an evidence archive and verify every file digest.
    Verify {
        /// Path to evidence.gfed.
        archive: PathBuf,
        /// Environment variable that holds the archive passphrase.
        #[arg(long, default_value = "GFED_PASSPHRASE")]
        passphrase_env: String,
    },
    /// List the bundled target versions and mapping date.
    Capabilities,
    /// Run up to ten local drills and write one Team Pack portfolio report.
    Portfolio {
        /// Export directory. Repeat this flag for each repository.
        #[arg(long = "source", required = true, num_args = 1..=10)]
        sources: Vec<PathBuf>,
        /// Target mapping shared by every repository.
        #[arg(long)]
        target: String,
        /// Directory for repository outputs and portfolio.md.
        #[arg(long, default_value = "exit-drill-portfolio")]
        output: PathBuf,
        /// Environment variable that holds the archive passphrase.
        #[arg(long, default_value = "GFED_PASSPHRASE")]
        passphrase_env: String,
        /// Environment variable that holds the Team Pack license.
        #[arg(long, default_value = "GFED_LICENSE")]
        license_env: String,
    },
}

fn main() -> ExitCode {
    let requested_json = env::args_os().any(|argument| argument == OsStr::new("--json"));
    let cli = match Cli::try_parse() {
        Ok(cli) => cli,
        Err(error) => {
            if requested_json && error.exit_code() != 0 {
                println!(
                    "{}",
                    serde_json::json!({
                        "error": error.to_string().trim(),
                        "ok": false,
                    })
                );
            } else {
                let _ = error.print();
            }
            return ExitCode::from(error.exit_code() as u8);
        }
    };
    let json = cli.json;
    match execute(cli) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            if json {
                println!(
                    "{}",
                    serde_json::json!({
                        "error": format!("{error:#}"),
                        "ok": false,
                    })
                );
            } else {
                eprintln!("error: {error:#}");
            }
            ExitCode::FAILURE
        }
    }
}

fn execute(cli: Cli) -> Result<()> {
    match cli.command {
        Command::Drill {
            source,
            repo,
            target,
            output,
            token_env,
            passphrase_env,
        } => {
            let passphrase = required_secret(&passphrase_env, "archive passphrase")?;
            let (inventory, evidence) = match (source, repo) {
                (Some(path), None) => {
                    let inventory = inventory_local(&path)?;
                    validate_output_outside_source(&path, &output)?;
                    inventory
                }
                (None, Some(repository)) => {
                    let token = required_secret(&token_env, "GitHub token")?;
                    inventory_github(&repository, &token)?
                }
                _ => bail!("choose exactly one source: --source or --repo"),
            };
            let result = run_drill(inventory, evidence, &target, &output, &passphrase)?;
            print_result(&result, cli.json)?;
        }
        Command::Demo { output, target } => {
            let root = output.unwrap_or_else(|| {
                env::temp_dir().join(format!(
                    "git-forge-exit-drill-demo-{}-{}",
                    std::process::id(),
                    std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap_or_default()
                        .as_nanos()
                ))
            });
            if root.exists()
                && (!root.is_dir()
                    || fs::read_dir(&root)
                        .with_context(|| {
                            format!("could not inspect demo directory {}", root.display())
                        })?
                        .next()
                        .is_some())
            {
                bail!(
                    "demo output '{}' already contains files; choose a new or empty directory to avoid deleting existing data",
                    root.display()
                );
            }
            let sample = root.join("sample-export");
            fs::create_dir_all(&sample)?;
            let files = [
                (
                    "manifest.json",
                    include_bytes!("../examples/atlas-notes-export/manifest.json").as_slice(),
                ),
                (
                    "issues.json",
                    include_bytes!("../examples/atlas-notes-export/issues.json").as_slice(),
                ),
                (
                    "pull_requests.json",
                    include_bytes!("../examples/atlas-notes-export/pull_requests.json").as_slice(),
                ),
                (
                    "releases.json",
                    include_bytes!("../examples/atlas-notes-export/releases.json").as_slice(),
                ),
                (
                    "workflow_runs.json",
                    include_bytes!("../examples/atlas-notes-export/workflow_runs.json").as_slice(),
                ),
                (
                    "workflows.json",
                    include_bytes!("../examples/atlas-notes-export/workflows.json").as_slice(),
                ),
                (
                    "LICENSES.txt",
                    include_bytes!("../examples/atlas-notes-export/LICENSES.txt").as_slice(),
                ),
            ];
            for (name, bytes) in files {
                fs::write(sample.join(name), bytes)?;
            }
            create_demo_git_mirror(&sample.join("atlas-notes.git"))?;
            let (inventory, evidence) = inventory_local(&sample)?;
            let result = run_drill(
                inventory,
                evidence,
                &target,
                &root.join("result"),
                "demo-only-passphrase",
            )?;
            if cli.json {
                #[derive(Serialize)]
                struct DemoOutput<'a> {
                    mode: &'static str,
                    saved_to_real_workspace: bool,
                    archive_passphrase: &'static str,
                    result: &'a DrillResult,
                }
                println!(
                    "{}",
                    serde_json::to_string_pretty(&DemoOutput {
                        mode: "demo",
                        saved_to_real_workspace: false,
                        archive_passphrase: "demo-only-passphrase",
                        result: &result,
                    })?
                );
            } else {
                println!("Demo — sample data. No workspace files were read.");
                print_result(&result, false)?;
                println!("Demo archive passphrase: demo-only-passphrase");
                println!("Choose a new output directory to run this demo again.");
            }
        }
        Command::Verify {
            archive,
            passphrase_env,
        } => {
            let passphrase = required_secret(&passphrase_env, "archive passphrase")?;
            let payload = read_encrypted_archive(&archive, &passphrase)?;
            if cli.json {
                println!(
                    "{}",
                    serde_json::json!({
                        "valid": true,
                        "repository": payload.inventory.repository,
                        "files": payload.files.len(),
                        "evidence_files": payload.files.iter().map(|file| serde_json::json!({
                            "path": file.path,
                            "sha256": file.sha256,
                        })).collect::<Vec<_>>(),
                        "archive": archive,
                    })
                );
            } else {
                println!("Archive verified: {}", archive.display());
                println!("Repository: {}", payload.inventory.repository);
                println!("Evidence files: {}", payload.files.len());
            }
        }
        Command::Capabilities => {
            let mappings = load_mappings()?;
            if cli.json {
                println!("{}", serde_json::to_string_pretty(&mappings)?);
            } else {
                println!("Capability map updated {}", mappings.updated);
                for target in mappings.targets {
                    println!(
                        "- {}:{} — {} artifact mappings",
                        target.id,
                        target.version,
                        target.capabilities.len()
                    );
                }
            }
        }
        Command::Portfolio {
            sources,
            target,
            output,
            passphrase_env,
            license_env,
        } => {
            if sources.len() > 10 {
                bail!(
                    "portfolio accepts at most 10 export directories; remove {} --source value(s) and try again",
                    sources.len() - 10
                );
            }
            for source in &sources {
                validate_output_outside_source(source, &output)?;
            }
            let license = required_secret(&license_env, "Team Pack license")?;
            verify_team_license(&license)?;
            let passphrase = required_secret(&passphrase_env, "archive passphrase")?;
            let mut results = Vec::new();
            for (index, source) in sources.iter().enumerate() {
                let (inventory, evidence) = inventory_local(source)?;
                let repo_output = output.join(format!("repo-{:02}", index + 1));
                results.push(run_drill(
                    inventory,
                    evidence,
                    &target,
                    &repo_output,
                    &passphrase,
                )?);
            }
            let portfolio = write_portfolio(&results, &target, &output)?;
            if cli.json {
                println!(
                    "{}",
                    serde_json::json!({"portfolio": portfolio, "repositories": results})
                );
            } else {
                println!("Portfolio written: {}", portfolio.display());
                for result in &results {
                    println!("- {} — {}", result.repository, result.outcome);
                }
            }
        }
    }
    Ok(())
}

fn validate_output_outside_source(source: &Path, output: &Path) -> Result<()> {
    let source = fs::canonicalize(source).with_context(|| {
        format!(
            "could not resolve source '{}'; check --source and try again",
            source.display()
        )
    })?;
    let output = resolve_path_for_boundary(output)?;
    if output.starts_with(&source) {
        bail!(
            "output '{}' is inside selected source '{}'; choose an output directory outside the export so the source stays read-only",
            output.display(),
            source.display()
        );
    }
    Ok(())
}

fn resolve_path_for_boundary(path: &Path) -> Result<PathBuf> {
    let absolute = if path.is_absolute() {
        path.to_path_buf()
    } else {
        env::current_dir()
            .context("could not determine the current directory")?
            .join(path)
    };
    let normalized = normalize_path(&absolute);
    let mut existing = normalized.clone();
    let mut unresolved = Vec::new();
    while !existing.exists() {
        let component = existing.file_name().ok_or_else(|| {
            anyhow::anyhow!(
                "could not resolve output path '{}'; choose another directory",
                path.display()
            )
        })?;
        unresolved.push(component.to_os_string());
        if !existing.pop() {
            bail!(
                "could not resolve output path '{}'; choose another directory",
                path.display()
            );
        }
    }
    let mut resolved = fs::canonicalize(&existing).with_context(|| {
        format!(
            "could not resolve output path '{}'; choose another directory",
            path.display()
        )
    })?;
    for component in unresolved.iter().rev() {
        resolved.push(component);
    }
    Ok(resolved)
}

fn normalize_path(path: &Path) -> PathBuf {
    let mut normalized = PathBuf::new();
    for component in path.components() {
        match component {
            std::path::Component::CurDir => {}
            std::path::Component::ParentDir => {
                normalized.pop();
            }
            _ => normalized.push(component.as_os_str()),
        }
    }
    normalized
}

fn required_secret(variable: &str, description: &str) -> Result<String> {
    env::var(variable)
        .with_context(|| format!("{description} is missing; set {variable} and try again"))
}

fn print_result(result: &DrillResult, json: bool) -> Result<()> {
    if json {
        println!("{}", serde_json::to_string_pretty(result)?);
    } else {
        println!("Repository: {}", result.repository);
        println!("Target: {}", result.target);
        println!("Outcome: {}", result.outcome.to_uppercase());
        println!("Report: {}", result.markdown_report.display());
        println!("JSON: {}", result.json_report.display());
        println!("Encrypted evidence: {}", result.evidence_archive.display());
        println!("Evidence SHA-256: {}", result.archive_sha256);
    }
    Ok(())
}
