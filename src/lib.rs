use aes_gcm::{Aes256Gcm, KeyInit, Nonce, aead::Aead};
use anyhow::{Context, Result, anyhow, bail};
use argon2::Argon2;
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};
use rand::{RngCore, rngs::OsRng};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
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
    /// Recognized export files or manifest declarations that could not be
    /// backed by parseable records. These are deliberately separate from a
    /// missing artifact: they tell the operator that a file or declared total
    /// exists, but it is not evidence they can rely on for cutover.
    #[serde(default)]
    pub incomplete: BTreeMap<String, String>,
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
    #[serde(default)]
    pub incomplete: BTreeMap<String, String>,
    #[serde(default)]
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

    let mut declared = BTreeMap::new();
    let mut artifacts = BTreeMap::new();
    let mut incomplete = BTreeMap::new();
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
            declared = explicit;
        }
    }

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
        if let Some(kind) = artifact_kind(&relative) {
            match parsed_artifact_counts(kind, &bytes) {
                Ok(parsed) => {
                    for (artifact, count) in parsed.counts {
                        *artifacts.entry(artifact).or_insert(0) += count;
                    }
                    for (artifact, reason) in parsed.incomplete {
                        incomplete.entry(artifact.clone()).or_insert_with(|| {
                            format!(
                                "{relative} contains invalid {} evidence: {reason}",
                                label(&artifact)
                            )
                        });
                    }
                }
                Err(reason) => {
                    incomplete.entry(kind.to_owned()).or_insert_with(|| {
                        format!(
                            "{} is not valid {} evidence: {reason}",
                            relative,
                            label(kind)
                        )
                    });
                }
            }
        }
        evidence.push(evidence_file(relative, bytes));
    }

    if has_valid_git_evidence(root, &paths)? {
        artifacts.insert("git_repository".to_owned(), 1);
    }

    for (artifact, expected) in declared {
        if artifact == "git_repository" {
            continue;
        }
        if !ARTIFACT_ORDER.contains(&artifact.as_str()) {
            continue;
        }
        let actual = artifacts.get(&artifact).copied();
        if actual != Some(expected) {
            incomplete.entry(artifact.clone()).or_insert_with(|| match actual {
                Some(found) => format!(
                    "manifest.json declares {expected} {} but parseable export records contain {found}",
                    label(&artifact)
                ),
                None => format!(
                    "manifest.json declares {expected} {} but no corresponding parseable export records were found",
                    label(&artifact)
                ),
            });
        }
    }

    if artifacts.is_empty() && incomplete.is_empty() {
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
            incomplete,
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

fn artifact_kind(path: &str) -> Option<&'static str> {
    let name = path.to_ascii_lowercase();
    if !name.ends_with(".json") || name.ends_with("manifest.json") {
        return None;
    }
    if name.contains("pull_request") || name.contains("pulls") {
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
    }
}

/// Parse only records that are actually present in a recognized export file.
/// A `total_count` is useful metadata, but it is never a substitute for the
/// records that a restore drill needs to preserve. A record is evidence only
/// after its artifact-specific identity and restoration fields validate.
#[derive(Debug, Default)]
struct ParsedArtifactCounts {
    counts: BTreeMap<String, u64>,
    incomplete: BTreeMap<String, String>,
}

fn parsed_artifact_counts(
    kind: &str,
    bytes: &[u8],
) -> std::result::Result<ParsedArtifactCounts, String> {
    let value: Value = serde_json::from_slice(bytes).map_err(|error| error.to_string())?;
    let records = artifact_records(kind, value)?;

    let mut parsed = ParsedArtifactCounts::default();
    let mut valid_records = 0_u64;
    let mut release_assets = 0_u64;
    for (index, record) in records.iter().enumerate() {
        match validate_artifact_record(kind, record) {
            Ok(asset_count) => {
                valid_records += 1;
                release_assets += asset_count;
            }
            Err(reason) => {
                let message = format!("record {} {reason}", index + 1);
                parsed
                    .incomplete
                    .entry(kind.to_owned())
                    .or_insert_with(|| message.clone());
                if kind == "releases" {
                    parsed
                        .incomplete
                        .entry("release_assets".to_owned())
                        .or_insert(message);
                }
            }
        }
    }

    // An explicit empty collection is valid evidence of zero records. A
    // wholly invalid collection gets no count at all; a mixed one exposes the
    // number of individually valid records while remaining incomplete.
    if !parsed.incomplete.contains_key(kind) || valid_records > 0 {
        parsed.counts.insert(kind.to_owned(), valid_records);
    }
    if kind == "releases"
        && (!parsed.incomplete.contains_key("release_assets") || release_assets > 0)
    {
        parsed
            .counts
            .insert("release_assets".to_owned(), release_assets);
    }
    Ok(parsed)
}

fn artifact_records(kind: &str, value: Value) -> std::result::Result<Vec<Value>, String> {
    let records = match value {
        Value::Array(items) => items,
        Value::Object(map) => {
            let collection_keys: &[&str] = match kind {
                "actions_workflows" => &["workflows", "items"],
                "actions_runs" => &["workflow_runs", "items"],
                "pull_requests" => &["pull_requests", "pulls", "items"],
                "issues" => &["issues", "items"],
                "releases" => &["releases", "items"],
                _ => &["items"],
            };
            if let Some(records) = collection_keys
                .iter()
                .find_map(|key| map.get(*key).and_then(Value::as_array))
            {
                records.clone()
            } else if map.contains_key("id")
                || map.contains_key("number")
                || map.contains_key("tag_name")
            {
                vec![Value::Object(map)]
            } else if map.get("total_count").is_some_and(Value::is_u64) {
                Vec::new()
            } else {
                return Err("it does not contain an array of export records".to_owned());
            }
        }
        _ => return Err("it must contain an array or object of export records".to_owned()),
    };
    Ok(records)
}

fn validate_artifact_record(kind: &str, record: &Value) -> std::result::Result<u64, String> {
    if kind == "release_assets" {
        return validate_release_asset(record).map(|_| 0);
    }

    let map = record
        .as_object()
        .ok_or_else(|| "must be a JSON object".to_owned())?;
    match kind {
        "issues" | "pull_requests" => {
            require_identifier(map, &["id", "number"])?;
            require_text(map, &["title"], "a title")?;
            require_author(map)?;
        }
        "releases" => {
            require_text(map, &["tag_name"], "a release tag")?;
            let assets = map
                .get("assets")
                .and_then(Value::as_array)
                .ok_or_else(|| "must include an assets array".to_owned())?;
            for asset in assets {
                validate_release_asset(asset)?;
            }
            return Ok(assets.len() as u64);
        }
        "actions_workflows" => {
            require_identifier(map, &["id"])?;
            require_text(map, &["name"], "a workflow name")?;
            require_text(map, &["path"], "a workflow path")?;
        }
        "actions_runs" => {
            require_identifier(map, &["id"])?;
            require_text(map, &["name"], "a run name")?;
            require_text(map, &["head_sha"], "a commit SHA")?;
        }
        "branch_protection" => {
            let has_identity = has_identifier(map, &["id", "name", "pattern"]);
            let has_policy = [
                "required_status_checks",
                "required_pull_request_reviews",
                "enforce_admins",
                "restrictions",
                "rules",
            ]
            .iter()
            .any(|field| map.contains_key(*field));
            if !has_identity && !has_policy {
                return Err("must include a rule identity or a branch protection policy".to_owned());
            }
        }
        "webhooks" => {
            require_identifier(map, &["id"])?;
            if !map.get("events").is_some_and(Value::is_array) {
                return Err("must include an events array".to_owned());
            }
            if !map.get("config").is_some_and(Value::is_object) {
                return Err("must include a webhook config object".to_owned());
            }
        }
        "secrets" => {
            require_text(map, &["name"], "a secret name")?;
        }
        "packages" => {
            require_identifier(map, &["id"])?;
            require_text(map, &["name"], "a package name")?;
            require_text(map, &["package_type", "type"], "a package type")?;
        }
        "discussions" => {
            require_identifier(map, &["id", "number"])?;
            require_text(map, &["title"], "a discussion title")?;
            require_author(map)?;
        }
        "git_lfs" => {
            require_text(map, &["oid"], "an LFS object ID")?;
            if !map.get("size").is_some_and(Value::is_u64) {
                return Err("must include an LFS object size".to_owned());
            }
        }
        _ => {
            require_identifier(map, &["id", "name"])?;
        }
    }
    Ok(0)
}

fn validate_release_asset(asset: &Value) -> std::result::Result<(), String> {
    if asset.as_str().is_some_and(|name| !name.trim().is_empty()) {
        return Ok(());
    }
    let map = asset
        .as_object()
        .ok_or_else(|| "must be a named release asset object or name string".to_owned())?;
    require_text(map, &["name"], "a release asset name")?;
    require_identifier(map, &["id", "browser_download_url"])
}

fn require_identifier(
    map: &Map<String, Value>,
    fields: &[&str],
) -> std::result::Result<(), String> {
    if has_identifier(map, fields) {
        Ok(())
    } else {
        Err(format!("must include {}", joined_fields(fields)))
    }
}

fn has_identifier(map: &Map<String, Value>, fields: &[&str]) -> bool {
    fields.iter().any(|field| {
        map.get(*field).is_some_and(|value| {
            value.is_number() || value.as_str().is_some_and(|text| !text.trim().is_empty())
        })
    })
}

fn require_text(
    map: &Map<String, Value>,
    fields: &[&str],
    description: &str,
) -> std::result::Result<(), String> {
    if fields.iter().any(|field| {
        map.get(*field)
            .and_then(Value::as_str)
            .is_some_and(|text| !text.trim().is_empty())
    }) {
        Ok(())
    } else {
        Err(format!("must include {description}"))
    }
}

fn require_author(map: &Map<String, Value>) -> std::result::Result<(), String> {
    let has_author = ["author", "user", "creator"].iter().any(|field| {
        map.get(*field).is_some_and(|value| match value {
            Value::String(text) => !text.trim().is_empty(),
            Value::Object(person) => ["login", "username", "name", "id"]
                .iter()
                .any(|key| has_identifier(person, &[*key])),
            _ => false,
        })
    });
    if has_author {
        Ok(())
    } else {
        Err("must include an issue or pull request author".to_owned())
    }
}

fn joined_fields(fields: &[&str]) -> String {
    match fields {
        [] => "an identifier".to_owned(),
        ["id", "number"] => "an id or number".to_owned(),
        ["id", "browser_download_url"] => "an id or browser download URL".to_owned(),
        [field] => format!("a {field}"),
        [first, second] => format!("a {first} or {second}"),
        _ => fields.join(" or "),
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
        if path
            .extension()
            .is_some_and(|extension| extension == "bundle")
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
    // Alternates make a repository depend on object bytes outside the selected
    // export. That is not self-contained migration evidence.
    let alternates = directory.join("objects/info/alternates");
    if alternates.is_file() && fs::metadata(&alternates)?.len() > 0 {
        return Ok(false);
    }

    let fsck = Command::new("git")
        .arg(format!("--git-dir={}", directory.display()))
        .args(["fsck", "--no-reflogs", "--no-dangling", "--no-progress"])
        .env_remove("GIT_ALTERNATE_OBJECT_DIRECTORIES")
        .env_remove("GIT_OBJECT_DIRECTORY")
        .output()
        .context(
            "could not run Git to validate repository object bytes; install Git and try again",
        )?;
    if !fsck.status.success() {
        return Ok(false);
    }

    // `git fsck` exits successfully for a newly initialized repository. A
    // migration export must have a commit reachable from a real ref.
    let reachable_commit = Command::new("git")
        .arg(format!("--git-dir={}", directory.display()))
        .args(["rev-list", "--max-count=1", "--all"])
        .env_remove("GIT_ALTERNATE_OBJECT_DIRECTORIES")
        .env_remove("GIT_OBJECT_DIRECTORY")
        .output()
        .context("could not inspect Git refs; install Git and try again")?;
    if !reachable_commit.status.success()
        || reachable_commit.stdout.iter().all(u8::is_ascii_whitespace)
    {
        return Ok(false);
    }

    // Count only bytes in this object database. This excludes an unborn repo
    // and prevents metadata or refs backed solely by external alternates from
    // being reported as captured history.
    let object_count = Command::new("git")
        .arg(format!("--git-dir={}", directory.display()))
        .args(["count-objects", "-v"])
        .env_remove("GIT_ALTERNATE_OBJECT_DIRECTORIES")
        .env_remove("GIT_OBJECT_DIRECTORY")
        .output()
        .context("could not count Git object bytes; install Git and try again")?;
    if !object_count.status.success() {
        return Ok(false);
    }
    let counts = String::from_utf8_lossy(&object_count.stdout);
    let local_objects = counts.lines().filter_map(|line| {
        let (key, value) = line.split_once(": ")?;
        matches!(key, "count" | "in-pack")
            .then(|| value.parse::<u64>().ok())
            .flatten()
    });
    Ok(local_objects.sum::<u64>() > 0)
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
        fs::remove_dir_all(&staging).with_context(|| {
            format!(
                "could not remove temporary bundle check {}",
                staging.display()
            )
        })?;
    }
    Ok(valid)
}

fn unique_temp_path(prefix: &str) -> PathBuf {
    let mut random = [0_u8; 8];
    OsRng.fill_bytes(&mut random);
    env::temp_dir().join(format!(
        "{prefix}-{}-{:x}",
        std::process::id(),
        u64::from_le_bytes(random)
    ))
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
        content.len(),
        content,
        message.len(),
        message
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

fn valid_non_pull_request_issue_count(bytes: &[u8]) -> Option<u64> {
    let value = serde_json::from_slice::<Value>(bytes).ok()?;
    let records = artifact_records("issues", value).ok()?;
    Some(
        records
            .iter()
            .filter(|record| {
                validate_artifact_record("issues", record).is_ok()
                    && record
                        .as_object()
                        .is_none_or(|map| !map.contains_key("pull_request"))
            })
            .count() as u64,
    )
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
    let mut incomplete = BTreeMap::new();
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
                match parsed_artifact_counts(kind, &bytes) {
                    Ok(parsed) => {
                        let has_valid_issue_count = parsed.counts.contains_key("issues");
                        for (artifact, count) in parsed.counts {
                            artifacts.insert(artifact, count);
                        }
                        for (artifact, reason) in parsed.incomplete {
                            incomplete.entry(artifact.clone()).or_insert_with(|| {
                                format!(
                                    "GitHub API response contains invalid {} evidence: {reason}",
                                    label(&artifact)
                                )
                            });
                        }
                        if kind == "issues"
                            && has_valid_issue_count
                            && let Some(count) = valid_non_pull_request_issue_count(&bytes)
                        {
                            artifacts.insert("issues".to_owned(), count);
                        }
                    }
                    Err(reason) => {
                        incomplete.entry(kind.to_owned()).or_insert_with(|| {
                            format!(
                                "GitHub API response is not valid {} evidence: {reason}",
                                label(kind)
                            )
                        });
                    }
                }
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
            incomplete,
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
        let incomplete = inventory.incomplete.get(*artifact);
        let captured = count.is_some() && incomplete.is_none();
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
        let (result, next_step) = if let Some(reason) = incomplete {
            if critical {
                blocked = true;
            } else {
                review = true;
            }
            ("incomplete evidence".to_owned(), reason.clone())
        } else if !captured {
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
        incomplete: inventory.incomplete.clone(),
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
        let captured = if finding.captured {
            format!("Yes ({})", finding.count.unwrap_or(0))
        } else if let Some(count) = finding.count {
            format!("No ({} valid records)", count)
        } else {
            "No".to_owned()
        };
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
    if !report.incomplete.is_empty() {
        output.push_str("\n## Incomplete evidence\n\n");
        for (artifact, reason) in &report.incomplete {
            output.push_str(&format!("- **{}:** {}\n", label(artifact), reason));
        }
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
    body.push_str("| Repository | Outcome | Evidence gaps | Target gaps | Restore tests |\n| --- | --- | ---: | ---: | ---: |\n");
    for result in results {
        body.push_str(&format!(
            "| {} | {} | {} | {} | {} |\n",
            result.repository,
            result.outcome,
            result
                .finding_counts
                .get("missing evidence")
                .copied()
                .unwrap_or(0)
                + result
                    .finding_counts
                    .get("incomplete evidence")
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
                incomplete: BTreeMap::new(),
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
            incomplete: BTreeMap::new(),
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
