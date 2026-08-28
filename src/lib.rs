use aes_gcm::{Aes256Gcm, KeyInit, Nonce, aead::Aead};
use anyhow::{Context, Result, anyhow, bail};
use argon2::Argon2;
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};
use rand::{RngCore, rngs::OsRng};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    env, fs,
    io::{Read, Write},
    path::{Path, PathBuf},
    process::{Command, Stdio},
    time::{SystemTime, UNIX_EPOCH},
};
use zeroize::Zeroize;

const ARCHIVE_MAGIC: &[u8; 8] = b"GFEDv001";
const MAX_FILE_BYTES: u64 = 25 * 1024 * 1024;
const MAX_ARCHIVE_BYTES: u64 = 250 * 1024 * 1024;
const BILLING_BASE: &str = "https://api.sociobot.in";
const PRODUCT_SLUG: &str = "git-forge-exit-drill";

pub const ARTIFACT_ORDER: &[&str] = &[
    "git_repository",
    "issues",
    "pull_requests",
    "releases",
    "release_assets",
    "actions_workflows",
    "actions_runs",
    "branch_protection",
    "webhooks",
    "secrets",
    "packages",
    "discussions",
    "git_lfs",
];

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Inventory {
    pub repository: String,
    pub source: String,
    pub captured_at_unix: u64,
    pub artifacts: BTreeMap<String, u64>,
    #[serde(default)]
    pub unavailable: BTreeMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvidenceFile {
    pub path: String,
    pub bytes: u64,
    pub sha256: String,
    pub content_base64: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvidencePayload {
    pub schema_version: u8,
    pub tool_version: String,
    pub inventory: Inventory,
    pub files: Vec<EvidenceFile>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Capability {
    pub status: String,
    pub note: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Target {
    pub id: String,
    pub version: String,
    pub label: String,
    pub capabilities: BTreeMap<String, Capability>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TargetMappings {
    pub schema_version: u8,
    pub updated: String,
    pub targets: Vec<Target>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Finding {
    pub artifact: String,
    pub captured: bool,
    pub count: Option<u64>,
    pub target_support: String,
    pub critical: bool,
    pub result: String,
    pub next_step: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReadinessReport {
    pub schema_version: u8,
    pub repository: String,
    pub target: String,
    pub mapping_updated: String,
    pub outcome: String,
    pub findings: Vec<Finding>,
    pub unavailable: BTreeMap<String, String>,
    pub restore_checklist: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DrillResult {
    pub repository: String,
    pub target: String,
    pub outcome: String,
    pub evidence_archive: PathBuf,
    pub markdown_report: PathBuf,
    pub json_report: PathBuf,
    pub archive_sha256: String,
    pub finding_counts: BTreeMap<String, usize>,
}

#[derive(Debug, Deserialize)]
struct LocalManifest {
    repository: Option<String>,
    artifacts: Option<BTreeMap<String, u64>>,
}

pub fn load_mappings() -> Result<TargetMappings> {
    serde_json::from_str(include_str!("../mappings/targets.json"))
        .context("the bundled target mapping is invalid; reinstall the CLI")
}

pub fn resolve_target(spec: &str) -> Result<Target> {
    let normalized = spec.replace('@', ":").to_ascii_lowercase();
    let mappings = load_mappings()?;
    let mut matches = mappings.targets.into_iter().filter(|target| {
        normalized == target.id || normalized == format!("{}:{}", target.id, target.version)
    });
    let first = matches.next();
    if first.is_none() {
        let choices = load_mappings()?
            .targets
            .into_iter()
            .map(|target| format!("{}:{}", target.id, target.version))
            .collect::<Vec<_>>()
            .join(", ");
        bail!("target '{spec}' is not mapped; choose one of: {choices}")
    }
    Ok(first.expect("target exists"))
}

pub fn inventory_local(root: &Path) -> Result<(Inventory, Vec<EvidenceFile>)> {
    if !root.exists() {
        bail!(
            "export directory '{}' does not exist; check --source and try again",
            root.display()
        );
    }
    if !root.is_dir() {
        bail!(
            "source '{}' is not a directory; pass the extracted export directory",
            root.display()
        );
    }

    let paths = collect_files(root)?;
    if paths.is_empty() {
        bail!(
            "export directory '{}' is empty; extract the GitHub export and try again",
            root.display()
        );
    }

    let mut artifacts = BTreeMap::new();
    let manifest_path = root.join("manifest.json");
    let mut repository = root
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("local-export")
        .to_owned();

    if manifest_path.is_file() {
        let body = fs::read_to_string(&manifest_path)
            .with_context(|| format!("could not read {}", manifest_path.display()))?;
        let manifest: LocalManifest = serde_json::from_str(&body).with_context(|| {
            format!(
                "{} is not a valid manifest; fix its JSON and try again",
                manifest_path.display()
            )
        })?;
        if let Some(name) = manifest.repository {
            repository = name;
        }
        if let Some(explicit) = manifest.artifacts {
            artifacts.extend(explicit);
        }
    }
    // A manifest is metadata, not repository evidence. Never let a claimed count
    // turn into a successful repository capture without real Git object bytes.
    artifacts.remove("git_repository");

    let mut evidence = Vec::with_capacity(paths.len());
    let mut total = 0_u64;
    for path in &paths {
        let metadata = fs::metadata(path)?;
        if metadata.len() > MAX_FILE_BYTES {
            bail!(
                "'{}' exceeds the 25 MB per-file limit; split large binary assets from the metadata export",
                path.display()
            );
        }
        total += metadata.len();
        if total > MAX_ARCHIVE_BYTES {
            bail!(
                "the export exceeds the 250 MB evidence limit; archive large repository objects separately"
            );
        }
        let bytes = fs::read(path).with_context(|| format!("could not read {}", path.display()))?;
        let relative = path
            .strip_prefix(root)
            .unwrap_or(path)
            .to_string_lossy()
            .replace('\\', "/");
        infer_artifact(&relative, &bytes, &mut artifacts);
        evidence.push(evidence_file(relative, bytes));
    }

    if has_valid_git_evidence(root, &paths)? {
        artifacts.insert("git_repository".to_owned(), 1);
    }

    if artifacts.is_empty() {
        bail!(
            "no supported GitHub artifacts were found; add manifest.json or common export JSON files"
        );
    }

    Ok((
        Inventory {
            repository,
            source: format!("local export: {}", root.display()),
            captured_at_unix: now_unix(),
            artifacts,
            unavailable: BTreeMap::new(),
        },
        evidence,
    ))
}

fn collect_files(root: &Path) -> Result<Vec<PathBuf>> {
    fn visit(directory: &Path, out: &mut Vec<PathBuf>) -> Result<()> {
        let mut entries = fs::read_dir(directory)
            .with_context(|| format!("could not read directory {}", directory.display()))?
            .collect::<std::result::Result<Vec<_>, _>>()?;
        entries.sort_by_key(|entry| entry.path());
        for entry in entries {
            let file_type = entry.file_type()?;
            if file_type.is_symlink() {
                continue;
            }
            if file_type.is_dir() {
                visit(&entry.path(), out)?;
            } else if file_type.is_file() {
                out.push(entry.path());
            }
        }
        Ok(())
    }
    let mut files = Vec::new();
    visit(root, &mut files)?;
    Ok(files)
}

fn infer_artifact(path: &str, bytes: &[u8], artifacts: &mut BTreeMap<String, u64>) {
    let name = path.to_ascii_lowercase();
    let kind = if name.contains("pull_request") || name.contains("pulls") {
        Some("pull_requests")
    } else if name.contains("workflow_run") || name.contains("action_run") {
        Some("actions_runs")
    } else if name.contains("workflow") || name.ends_with(".github/workflows") {
        Some("actions_workflows")
    } else if name.contains("release_asset") {
        Some("release_assets")
    } else if name.contains("release") {
        Some("releases")
    } else if name.contains("issue") {
        Some("issues")
    } else if name.contains("branch_protection") || name.contains("ruleset") {
        Some("branch_protection")
    } else if name.contains("webhook") || name.contains("hooks") {
        Some("webhooks")
    } else if name.contains("secret") {
        Some("secrets")
    } else if name.contains("package") {
        Some("packages")
    } else if name.contains("discussion") {
        Some("discussions")
    } else if name.contains("lfs") {
        Some("git_lfs")
    } else {
        None
    };
    if let Some(kind) = kind {
        artifacts
            .entry(kind.to_owned())
            .or_insert_with(|| json_count(bytes));
    }
}

/// Prove that an export contains an object database Git itself can read. A
/// manifest, refs without objects, and a directory named `repository.git` are
/// deliberately insufficient: they cannot restore repository history.
fn has_valid_git_evidence(root: &Path, paths: &[PathBuf]) -> Result<bool> {
    let mut git_dirs = BTreeSet::new();
    let dot_git = root.join(".git");
    if is_git_dir(&dot_git) {
        git_dirs.insert(dot_git);
    }
    if is_git_dir(root) {
        git_dirs.insert(root.to_path_buf());
    }

    for path in paths {
        if path.extension().is_some_and(|extension| extension == "bundle")
            && validate_git_bundle(path)?
        {
            return Ok(true);
        }
        let mut parent = path.parent();
        while let Some(directory) = parent {
            if !directory.starts_with(root) {
                break;
            }
            if directory
                .file_name()
                .and_then(|name| name.to_str())
                .is_some_and(|name| name == ".git" || name.ends_with(".git"))
                && is_git_dir(directory)
            {
                git_dirs.insert(directory.to_path_buf());
            }
            if directory == root {
                break;
            }
            parent = directory.parent();
        }
    }

    for directory in git_dirs {
        if validate_git_dir(&directory)? {
            return Ok(true);
        }
    }
    Ok(false)
}

fn is_git_dir(directory: &Path) -> bool {
    directory.join("HEAD").is_file()
        && directory.join("objects").is_dir()
        && directory.join("refs").is_dir()
}

fn validate_git_dir(directory: &Path) -> Result<bool> {
    let output = Command::new("git")
        .arg(format!("--git-dir={}", directory.display()))
        .args(["fsck", "--no-reflogs", "--no-dangling", "--no-progress"])
        .output()
        .context("could not run Git to validate repository object bytes; install Git and try again")?;
    Ok(output.status.success())
}

fn validate_git_bundle(bundle: &Path) -> Result<bool> {
    let staging = unique_temp_path("git-forge-exit-drill-bundle");
    let output = Command::new("git")
        .args(["clone", "--mirror", "--quiet"])
        .arg(bundle)
        .arg(&staging)
        .output()
        .context("could not run Git to validate bundle object bytes; install Git and try again")?;
    let valid = output.status.success() && is_git_dir(&staging) && validate_git_dir(&staging)?;
    if staging.exists() {
        fs::remove_dir_all(&staging)
            .with_context(|| format!("could not remove temporary bundle check {}", staging.display()))?;
    }
    Ok(valid)
}

fn unique_temp_path(prefix: &str) -> PathBuf {
    let mut random = [0_u8; 8];
    OsRng.fill_bytes(&mut random);
    env::temp_dir().join(format!("{prefix}-{}-{:x}", std::process::id(), u64::from_le_bytes(random)))
}

/// Create the small, valid bare mirror used by the isolated CLI demo. The
/// source files and commit are bundled in this binary; no workspace export or
/// network source is read.
pub fn create_demo_git_mirror(directory: &Path) -> Result<()> {
    let initialized = Command::new("git")
        .args(["init", "--bare", "--quiet"])
        .arg(directory)
        .status()
        .context("could not run Git for the bundled demo; install Git and try again")?;
    if !initialized.success() {
        bail!("could not create the bundled demo Git mirror")
    }

    let content = "Atlas Notes sample source\n";
    let message = "Seed Atlas Notes history\n";
    let stream = format!(
        "blob\nmark :1\ndata {}\n{}commit refs/heads/main\nauthor Demo Owner <demo@example.invalid> 0 +0000\ncommitter Demo Owner <demo@example.invalid> 0 +0000\ndata {}\n{}M 100644 :1 README.md\n\ndone\n",
        content.len(), content, message.len(), message
    );
    let mut child = Command::new("git")
        .arg(format!("--git-dir={}", directory.display()))
        .args(["fast-import", "--quiet"])
        .stdin(Stdio::piped())
        .spawn()
        .context("could not create the bundled demo Git objects")?;
    child
        .stdin
        .take()
        .expect("piped stdin")
        .write_all(stream.as_bytes())?;
    let imported = child.wait()?;
    if !imported.success() || !validate_git_dir(directory)? {
        bail!("could not validate the bundled demo Git objects")
    }
    Ok(())
}

fn json_count(bytes: &[u8]) -> u64 {
    let Ok(value) = serde_json::from_slice::<Value>(bytes) else {
        return 1;
    };
    match value {
        Value::Array(items) => items.len() as u64,
        Value::Object(map) => map
            .get("total_count")
            .and_then(Value::as_u64)
            .or_else(|| {
                map.values()
                    .find_map(|value| value.as_array().map(|items| items.len() as u64))
            })
            .unwrap_or(1),
        _ => 1,
    }
}

pub fn inventory_github(repository: &str, token: &str) -> Result<(Inventory, Vec<EvidenceFile>)> {
    if repository.split('/').count() != 2 {
        bail!("repository must use owner/name; for example acme/widgets")
    }
    if token.trim().is_empty() {
        bail!("the GitHub token is empty; set the token environment variable and try again")
    }
    let api_base =
        env::var("GFED_GITHUB_API_BASE").unwrap_or_else(|_| "https://api.github.com".to_owned());
    let mut artifacts = BTreeMap::new();
    let mut unavailable = BTreeMap::new();
    let mut evidence = Vec::new();

    let metadata = api_get(&api_base, &format!("/repos/{repository}"), token).context(
        "GitHub rejected the repository request; check the name and read-only token scope",
    )?;
    evidence.push(evidence_file("api/repository.json".to_owned(), metadata));
    unavailable.insert(
        "git_repository".to_owned(),
        "GitHub API mode inventories metadata only. Provide --source with a validated mirror or bundle to prove Git object capture.".to_owned(),
    );

    let endpoints = [
        (
            "issues",
            format!("/repos/{repository}/issues?state=all&per_page=100"),
        ),
        (
            "pull_requests",
            format!("/repos/{repository}/pulls?state=all&per_page=100"),
        ),
        (
            "releases",
            format!("/repos/{repository}/releases?per_page=100"),
        ),
        (
            "actions_workflows",
            format!("/repos/{repository}/actions/workflows?per_page=100"),
        ),
        (
            "actions_runs",
            format!("/repos/{repository}/actions/runs?per_page=100"),
        ),
        (
            "webhooks",
            format!("/repos/{repository}/hooks?per_page=100"),
        ),
        (
            "branch_protection",
            format!("/repos/{repository}/rulesets?per_page=100"),
        ),
    ];
    for (kind, path) in endpoints {
        match api_get_paginated(&api_base, &path, token) {
            Ok(bytes) => {
                let mut count = json_count(&bytes);
                if kind == "issues"
                    && let Ok(Value::Array(items)) = serde_json::from_slice::<Value>(&bytes)
                {
                    count = items
                        .iter()
                        .filter(|item| item.get("pull_request").is_none())
                        .count() as u64;
                }
                if kind == "releases"
                    && let Ok(Value::Array(items)) = serde_json::from_slice::<Value>(&bytes)
                {
                    let asset_count = items
                        .iter()
                        .filter_map(|item| item.get("assets")?.as_array())
                        .map(Vec::len)
                        .sum::<usize>();
                    artifacts.insert("release_assets".to_owned(), asset_count as u64);
                }
                artifacts.insert(kind.to_owned(), count);
                evidence.push(evidence_file(format!("api/{kind}.json"), bytes));
            }
            Err(error) => {
                unavailable.insert(
                    kind.to_owned(),
                    format!("API scope or endpoint unavailable: {error}"),
                );
            }
        }
    }

    Ok((
        Inventory {
            repository: repository.to_owned(),
            source: format!("GitHub API: {repository}"),
            captured_at_unix: now_unix(),
            artifacts,
            unavailable,
        },
        evidence,
    ))
}

fn api_get(base: &str, path: &str, token: &str) -> Result<Vec<u8>> {
    let url = format!("{}{}", base.trim_end_matches('/'), path);
    let response = ureq::get(&url)
        .set("Accept", "application/vnd.github+json")
        .set("Authorization", &format!("Bearer {token}"))
        .set("User-Agent", "git-forge-exit-drill/0.1")
        .call()
        .map_err(|error| anyhow!("{error}"))?;
    let mut bytes = Vec::new();
    response
        .into_reader()
        .take(MAX_FILE_BYTES)
        .read_to_end(&mut bytes)?;
    Ok(bytes)
}

fn api_get_paginated(base: &str, path: &str, token: &str) -> Result<Vec<u8>> {
    let first = api_get(base, path, token)?;
    let mut value: Value = serde_json::from_slice(&first)?;
    let (array_key, first_count, total_count) = match &value {
        Value::Array(items) => (None, items.len(), None),
        Value::Object(map) => {
            let key = ["workflow_runs", "workflows"]
                .into_iter()
                .find(|key| map.get(*key).and_then(Value::as_array).is_some());
            let count = key
                .and_then(|key| map.get(key))
                .and_then(Value::as_array)
                .map(Vec::len)
                .unwrap_or(0);
            let total = map.get("total_count").and_then(Value::as_u64);
            (key.map(str::to_owned), count, total)
        }
        _ => return Ok(first),
    };
    if first_count < 100 && total_count.is_none_or(|total| total as usize <= first_count) {
        return Ok(first);
    }

    for page in 2..=100 {
        let page_path = format!("{path}&page={page}");
        let page_bytes = api_get(base, &page_path, token)?;
        let page_value: Value = serde_json::from_slice(&page_bytes)?;
        let page_items = match (&array_key, page_value) {
            (None, Value::Array(items)) => items,
            (Some(key), Value::Object(mut map)) => map
                .remove(key)
                .and_then(|items| items.as_array().cloned())
                .unwrap_or_default(),
            _ => Vec::new(),
        };
        let page_count = page_items.len();
        match (&array_key, &mut value) {
            (None, Value::Array(items)) => items.extend(page_items),
            (Some(key), Value::Object(map)) => {
                if let Some(Value::Array(items)) = map.get_mut(key) {
                    items.extend(page_items);
                }
            }
            _ => {}
        }
        let current_count = match (&array_key, &value) {
            (None, Value::Array(items)) => items.len(),
            (Some(key), Value::Object(map)) => map
                .get(key)
                .and_then(Value::as_array)
                .map(Vec::len)
                .unwrap_or(0),
            _ => 0,
        };
        if page_count < 100 || total_count.is_some_and(|total| current_count >= total as usize) {
            break;
        }
    }
    let combined = serde_json::to_vec(&value)?;
    if combined.len() as u64 > MAX_FILE_BYTES {
        bail!("an API artifact exceeds the 25 MB evidence limit; use a local GitHub export instead")
    }
    Ok(combined)
}

fn evidence_file(path: String, bytes: Vec<u8>) -> EvidenceFile {
    EvidenceFile {
        path,
        bytes: bytes.len() as u64,
        sha256: hex_digest(&bytes),
        content_base64: BASE64.encode(bytes),
    }
}

pub fn build_report(
    inventory: &Inventory,
    target: &Target,
    mapping_updated: &str,
) -> ReadinessReport {
    let critical_artifacts = [
        "git_repository",
        "issues",
        "pull_requests",
        "releases",
        "actions_workflows",
        "actions_runs",
    ];
    let mut findings = Vec::new();
    let mut blocked = false;
    let mut review = !inventory.unavailable.is_empty();

    for artifact in ARTIFACT_ORDER {
        let count = inventory.artifacts.get(*artifact).copied();
        let captured = count.is_some();
        let critical = critical_artifacts.contains(artifact);
        let capability = target
            .capabilities
            .get(*artifact)
            .cloned()
            .unwrap_or(Capability {
                status: "unknown".to_owned(),
                note: "No mapping exists. Check the target documentation before cutover."
                    .to_owned(),
            });
        let (result, next_step) = if !captured {
            if critical {
                blocked = true;
            } else {
                review = true;
            }
            (
                "missing evidence".to_owned(),
                format!(
                    "Export {} or record why this repository has none.",
                    label(artifact)
                ),
            )
        } else if capability.status == "unsupported" {
            if critical {
                blocked = true;
            } else {
                review = true;
            }
            ("target gap".to_owned(), capability.note.clone())
        } else if capability.status == "manual" || capability.status == "unknown" {
            review = true;
            ("restore test required".to_owned(), capability.note.clone())
        } else {
            ("mapped".to_owned(), capability.note.clone())
        };
        findings.push(Finding {
            artifact: (*artifact).to_owned(),
            captured,
            count,
            target_support: capability.status,
            critical,
            result,
            next_step,
        });
    }

    let outcome = if blocked {
        "blocked"
    } else if review {
        "review"
    } else {
        "ready"
    }
    .to_owned();
    let restore_checklist = vec![
        "Restore into a disposable target project, never the planned production namespace.".to_owned(),
        "Compare the default branch and every tag against the source refs.".to_owned(),
        "Open one old issue and pull request; confirm author attribution, dates, comments, and attachments.".to_owned(),
        "Download one restored release asset and compare its checksum.".to_owned(),
        "Run one build from a pinned commit and save its logs plus artifact checksums.".to_owned(),
        "Review third-party license files before copying packages or release assets.".to_owned(),
        "Record the rollback owner, cutover window, and the old forge read-only date.".to_owned(),
    ];
    ReadinessReport {
        schema_version: 1,
        repository: inventory.repository.clone(),
        target: target.label.clone(),
        mapping_updated: mapping_updated.to_owned(),
        outcome,
        findings,
        unavailable: inventory.unavailable.clone(),
        restore_checklist,
    }
}

pub fn run_drill(
    inventory: Inventory,
    evidence: Vec<EvidenceFile>,
    target_spec: &str,
    output: &Path,
    passphrase: &str,
) -> Result<DrillResult> {
    validate_passphrase(passphrase)?;
    let mappings = load_mappings()?;
    let target = resolve_target(target_spec)?;
    let report = build_report(&inventory, &target, &mappings.updated);
    fs::create_dir_all(output).with_context(|| format!("could not create {}", output.display()))?;

    let payload = EvidencePayload {
        schema_version: 1,
        tool_version: env!("CARGO_PKG_VERSION").to_owned(),
        inventory,
        files: evidence,
    };
    let archive_path = output.join("evidence.gfed");
    write_encrypted_archive(&archive_path, &payload, passphrase)?;
    let archive_sha256 = hex_digest(&fs::read(&archive_path)?);

    let json_path = output.join("readiness.json");
    let markdown_path = output.join("readiness.md");
    write_atomic(&json_path, &serde_json::to_vec_pretty(&report)?)?;
    write_atomic(
        &markdown_path,
        render_markdown(&report, &archive_sha256).as_bytes(),
    )?;
    let mut finding_counts = BTreeMap::new();
    for finding in &report.findings {
        *finding_counts.entry(finding.result.clone()).or_insert(0) += 1;
    }
    Ok(DrillResult {
        repository: report.repository,
        target: report.target,
        outcome: report.outcome,
        evidence_archive: archive_path,
        markdown_report: markdown_path,
        json_report: json_path,
        archive_sha256,
        finding_counts,
    })
}

fn render_markdown(report: &ReadinessReport, archive_sha256: &str) -> String {
    let mut output = format!(
        "# Exit drill: {}\n\n**Target:** {}  \n**Outcome:** {}  \n**Capability map:** {}  \n**Evidence SHA-256:** `{}`\n\n",
        report.repository,
        report.target,
        report.outcome.to_uppercase(),
        report.mapping_updated,
        archive_sha256
    );
    output.push_str(
        "## Findings\n\n| Artifact | Captured | Target | Result |\n| --- | ---: | --- | --- |\n",
    );
    for finding in &report.findings {
        let captured = finding
            .count
            .map_or_else(|| "No".to_owned(), |count| format!("Yes ({count})"));
        output.push_str(&format!(
            "| {} | {} | {} | {} |\n",
            label(&finding.artifact),
            captured,
            finding.target_support,
            finding.result
        ));
    }
    output.push_str("\n## Actions before cutover\n\n");
    for finding in report
        .findings
        .iter()
        .filter(|finding| finding.result != "mapped")
    {
        output.push_str(&format!(
            "- [ ] **{}:** {}\n",
            label(&finding.artifact),
            finding.next_step
        ));
    }
    if !report.unavailable.is_empty() {
        output.push_str("\n## API evidence not available\n\n");
        for (artifact, reason) in &report.unavailable {
            output.push_str(&format!("- **{}:** {}\n", label(artifact), reason));
        }
    }
    output.push_str("\n## Restore drill\n\n");
    for item in &report.restore_checklist {
        output.push_str(&format!("- [ ] {item}\n"));
    }
    output.push_str("\nGenerated locally by Git Forge Exit Drill. Review this report with the people who own builds, releases, and access rules.\n");
    output
}

fn validate_passphrase(passphrase: &str) -> Result<()> {
    if passphrase.chars().count() < 12 {
        bail!(
            "the archive passphrase must contain at least 12 characters; set a longer value and try again"
        )
    }
    Ok(())
}

pub fn write_encrypted_archive(
    path: &Path,
    payload: &EvidencePayload,
    passphrase: &str,
) -> Result<()> {
    validate_passphrase(passphrase)?;
    let plaintext = serde_json::to_vec(payload)?;
    let mut salt = [0_u8; 16];
    let mut nonce_bytes = [0_u8; 12];
    OsRng.fill_bytes(&mut salt);
    OsRng.fill_bytes(&mut nonce_bytes);
    let mut key = [0_u8; 32];
    Argon2::default()
        .hash_password_into(passphrase.as_bytes(), &salt, &mut key)
        .map_err(|error| anyhow!("could not derive the archive key: {error}"))?;
    let cipher = Aes256Gcm::new_from_slice(&key).expect("32-byte key");
    let ciphertext = cipher
        .encrypt(Nonce::from_slice(&nonce_bytes), plaintext.as_ref())
        .map_err(|_| anyhow!("could not encrypt the evidence archive"))?;
    key.zeroize();
    let mut archive =
        Vec::with_capacity(ARCHIVE_MAGIC.len() + salt.len() + nonce_bytes.len() + ciphertext.len());
    archive.extend_from_slice(ARCHIVE_MAGIC);
    archive.extend_from_slice(&salt);
    archive.extend_from_slice(&nonce_bytes);
    archive.extend_from_slice(&ciphertext);
    write_atomic(path, &archive)
}

pub fn read_encrypted_archive(path: &Path, passphrase: &str) -> Result<EvidencePayload> {
    let archive = fs::read(path).with_context(|| format!("could not read {}", path.display()))?;
    if archive.len() < 52 || &archive[..8] != ARCHIVE_MAGIC {
        bail!("this is not a Git Forge Exit Drill archive")
    }
    let salt = &archive[8..24];
    let nonce = &archive[24..36];
    let ciphertext = &archive[36..];
    let mut key = [0_u8; 32];
    Argon2::default()
        .hash_password_into(passphrase.as_bytes(), salt, &mut key)
        .map_err(|error| anyhow!("could not derive the archive key: {error}"))?;
    let cipher = Aes256Gcm::new_from_slice(&key).expect("32-byte key");
    let plaintext = cipher
        .decrypt(Nonce::from_slice(nonce), ciphertext)
        .map_err(|_| {
            anyhow!("the archive could not be opened; check the passphrase and file integrity")
        })?;
    key.zeroize();
    let payload: EvidencePayload =
        serde_json::from_slice(&plaintext).context("the decrypted archive payload is invalid")?;
    for file in &payload.files {
        let content = BASE64
            .decode(&file.content_base64)
            .with_context(|| format!("{} has invalid encoded content", file.path))?;
        if content.len() as u64 != file.bytes || hex_digest(&content) != file.sha256 {
            bail!("archive integrity check failed for {}", file.path)
        }
    }
    Ok(payload)
}

pub fn verify_team_license(token: &str) -> Result<()> {
    if token.trim().is_empty() {
        bail!("Team Pack needs a license; set GFED_LICENSE or use the free drill command")
    }
    let token_hash = hex_digest(token.as_bytes());
    if let Some(cache_path) = license_cache_path()
        && let Ok(cache_bytes) = fs::read(&cache_path)
        && let Ok(cache) = serde_json::from_slice::<LicenseCache>(&cache_bytes)
        && cache.token_hash == token_hash
        && now_unix().saturating_sub(cache.checked_at) < 86_400
    {
        if cache.valid {
            return Ok(());
        }
        bail!("the Team Pack license is not active; restore or buy a license on the product site")
    }
    let base = env::var("GFED_BILLING_BASE").unwrap_or_else(|_| BILLING_BASE.to_owned());
    let url = format!(
        "{}/api/v1/products/{PRODUCT_SLUG}/verify",
        base.trim_end_matches('/')
    );
    let response = ureq::get(&url)
        .query("license", token)
        .call()
        .map_err(|_| {
            anyhow!("the license could not be checked; connect to the internet and try again")
        })?;
    let verdict: LicenseVerdict = response
        .into_json()
        .context("the license service returned an unreadable response")?;
    if let Some(cache_path) = license_cache_path() {
        if let Some(parent) = cache_path.parent() {
            let _ = fs::create_dir_all(parent);
        }
        let cache = LicenseCache {
            token_hash,
            valid: verdict.valid,
            checked_at: now_unix(),
        };
        if let Ok(bytes) = serde_json::to_vec(&cache) {
            let _ = write_atomic(&cache_path, &bytes);
        }
    }
    if !verdict.valid {
        bail!(
            "the Team Pack license is not active ({}); restore or buy a license on the product site",
            verdict.reason.unwrap_or_else(|| "invalid".to_owned())
        )
    }
    Ok(())
}

#[derive(Serialize, Deserialize)]
struct LicenseCache {
    token_hash: String,
    valid: bool,
    checked_at: u64,
}

#[derive(Deserialize)]
struct LicenseVerdict {
    valid: bool,
    reason: Option<String>,
}

fn license_cache_path() -> Option<PathBuf> {
    if let Ok(base) = env::var("XDG_CONFIG_HOME") {
        return Some(
            PathBuf::from(base)
                .join(PRODUCT_SLUG)
                .join("license-cache.json"),
        );
    }
    env::var("HOME").ok().map(|base| {
        PathBuf::from(base)
            .join(".config")
            .join(PRODUCT_SLUG)
            .join("license-cache.json")
    })
}

pub fn write_portfolio(results: &[DrillResult], target: &str, output: &Path) -> Result<PathBuf> {
    let path = output.join("portfolio.md");
    let mut body = format!(
        "# Exit drill portfolio\n\n**Target:** {target}  \n**Repositories:** {}\n\n",
        results.len()
    );
    body.push_str("| Repository | Outcome | Missing evidence | Target gaps | Restore tests |\n| --- | --- | ---: | ---: | ---: |\n");
    for result in results {
        body.push_str(&format!(
            "| {} | {} | {} | {} | {} |\n",
            result.repository,
            result.outcome,
            result
                .finding_counts
                .get("missing evidence")
                .copied()
                .unwrap_or(0),
            result
                .finding_counts
                .get("target gap")
                .copied()
                .unwrap_or(0),
            result
                .finding_counts
                .get("restore test required")
                .copied()
                .unwrap_or(0)
        ));
    }
    body.push_str(
        "\nOpen each repository report before scheduling cutover. Resolve blocked items first.\n",
    );
    fs::create_dir_all(output)?;
    write_atomic(&path, body.as_bytes())?;
    Ok(path)
}

fn write_atomic(path: &Path, bytes: &[u8]) -> Result<()> {
    let file_name = path
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("output");
    let temporary = path.with_file_name(format!(".{file_name}.tmp"));
    fs::write(&temporary, bytes)
        .with_context(|| format!("could not write {}", temporary.display()))?;
    fs::rename(&temporary, path).with_context(|| format!("could not finish {}", path.display()))?;
    Ok(())
}

fn label(artifact: &str) -> String {
    match artifact {
        "git_repository" => "Git repository".to_owned(),
        "pull_requests" => "Pull requests".to_owned(),
        "actions_workflows" => "Actions workflows".to_owned(),
        "actions_runs" => "Actions run history".to_owned(),
        "release_assets" => "Release assets".to_owned(),
        "branch_protection" => "Branch protection".to_owned(),
        "git_lfs" => "Git LFS objects".to_owned(),
        other => {
            let mut value = other.replace('_', " ");
            if let Some(first) = value.get_mut(0..1) {
                first.make_ascii_uppercase();
            }
            value
        }
    }
}

fn hex_digest(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

fn now_unix() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn target_aliases_resolve() {
        assert_eq!(resolve_target("forgejo").unwrap().version, "9.0");
        assert_eq!(resolve_target("gitea@1.22").unwrap().id, "gitea");
        assert!(resolve_target("unknown:1").is_err());
    }

    #[test]
    fn encrypted_archive_round_trip_and_rejects_wrong_key() {
        let directory = tempdir().unwrap();
        let payload = EvidencePayload {
            schema_version: 1,
            tool_version: "test".to_owned(),
            inventory: Inventory {
                repository: "acme/test".to_owned(),
                source: "fixture".to_owned(),
                captured_at_unix: 1,
                artifacts: BTreeMap::from([("issues".to_owned(), 2)]),
                unavailable: BTreeMap::new(),
            },
            files: vec![evidence_file(
                "issues.json".to_owned(),
                b"secret evidence".to_vec(),
            )],
        };
        let path = directory.path().join("evidence.gfed");
        write_encrypted_archive(&path, &payload, "correct horse battery").unwrap();
        let raw = fs::read(&path).unwrap();
        assert!(
            !raw.windows(b"secret evidence".len())
                .any(|window| window == b"secret evidence")
        );
        assert!(read_encrypted_archive(&path, "wrong password here").is_err());
        let restored = read_encrypted_archive(&path, "correct horse battery").unwrap();
        assert_eq!(restored.files[0].path, "issues.json");
    }

    #[test]
    fn missing_critical_evidence_blocks_cutover() {
        let inventory = Inventory {
            repository: "acme/test".to_owned(),
            source: "fixture".to_owned(),
            captured_at_unix: 1,
            artifacts: BTreeMap::from([("git_repository".to_owned(), 1)]),
            unavailable: BTreeMap::new(),
        };
        let mappings = load_mappings().unwrap();
        let report = build_report(
            &inventory,
            &resolve_target("forgejo:9.0").unwrap(),
            &mappings.updated,
        );
        assert_eq!(report.outcome, "blocked");
        assert!(report.findings.iter().any(|finding| finding.artifact == "issues" && finding.result == "missing evidence"));
    }
}
