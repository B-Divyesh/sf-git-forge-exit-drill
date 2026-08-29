use assert_cmd::Command;
use predicates::prelude::*;
use std::{fs, io::Write, path::Path};
use tempfile::tempdir;

fn create_valid_mirror(source: &Path) {
    let mirror = source.join("mirror.git");
    assert!(
        std::process::Command::new("git")
            .args(["init", "--bare", "--quiet"])
            .arg(&mirror)
            .status()
            .unwrap()
            .success()
    );
    let mut import = std::process::Command::new("git")
        .arg(format!("--git-dir={}", mirror.display()))
        .args(["fast-import", "--quiet"])
        .stdin(std::process::Stdio::piped())
        .spawn()
        .unwrap();
    import
        .stdin
        .take()
        .unwrap()
        .write_all(b"blob\nmark :1\ndata 5\nhello\ncommit refs/heads/main\nauthor Test <test@example.invalid> 0 +0000\ncommitter Test <test@example.invalid> 0 +0000\ndata 5\nseed\nM 100644 :1 README.md\n\ndone\n")
        .unwrap();
    assert!(import.wait().unwrap().success());
}

#[test]
fn demo_runs_end_to_end() {
    let directory = tempdir().unwrap();
    let output = directory.path().join("demo");
    Command::cargo_bin("git-forge-exit-drill")
        .unwrap()
        .args(["demo", "--output", output.to_str().unwrap()])
        .assert()
        .success()
        .stdout(predicate::str::contains("Demo — sample data"))
        .stdout(predicate::str::contains("Outcome: BLOCKED"));
    assert!(output.join("result/evidence.gfed").is_file());
    assert!(output.join("result/readiness.md").is_file());
    assert!(output.join("result/readiness.json").is_file());
    assert!(
        fs::read_to_string(output.join("result/readiness.md"))
            .unwrap()
            .contains("| Git repository | Yes (1) | native | mapped |")
    );
}

#[test]
fn demo_refuses_non_empty_output_and_preserves_existing_files() {
    let directory = tempdir().unwrap();
    let output = directory.path().join("existing-output");
    fs::create_dir(&output).unwrap();
    let sentinel = output.join("important.txt");
    fs::write(&sentinel, "do-not-delete").unwrap();

    Command::cargo_bin("git-forge-exit-drill")
        .unwrap()
        .args(["demo", "--output", output.to_str().unwrap()])
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "already contains files; choose a new or empty directory",
        ));

    assert_eq!(fs::read_to_string(sentinel).unwrap(), "do-not-delete");
}

#[test]
fn manifest_cannot_claim_git_repository_without_object_bytes() {
    let directory = tempdir().unwrap();
    let source = directory.path().join("metadata-only-export");
    let output = directory.path().join("result");
    fs::create_dir(&source).unwrap();
    fs::write(
        source.join("manifest.json"),
        r#"{"repository":"acme/metadata-only","artifacts":{"git_repository":1,"issues":1}}"#,
    )
    .unwrap();
    fs::write(source.join("issues.json"), "[]").unwrap();

    Command::cargo_bin("git-forge-exit-drill")
        .unwrap()
        .env("GFED_PASSPHRASE", "correct horse battery")
        .args([
            "drill",
            "--source",
            source.to_str().unwrap(),
            "--target",
            "forgejo:9.0",
            "--output",
            output.to_str().unwrap(),
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("Outcome: BLOCKED"));

    let report = fs::read_to_string(output.join("readiness.json")).unwrap();
    assert!(report.contains("\"artifact\": \"git_repository\""));
    assert!(report.contains("\"captured\": false"));
    assert!(report.contains("\"outcome\": \"blocked\""));
}

#[test]
fn malformed_recognized_json_is_incomplete_not_captured() {
    let directory = tempdir().unwrap();
    let source = directory.path().join("invalid-json-export");
    let output = directory.path().join("result");
    fs::create_dir(&source).unwrap();
    fs::write(source.join("issues.json"), "this is not json").unwrap();

    Command::cargo_bin("git-forge-exit-drill")
        .unwrap()
        .env("GFED_PASSPHRASE", "correct horse battery")
        .args([
            "drill",
            "--source",
            source.to_str().unwrap(),
            "--target",
            "forgejo:9.0",
            "--output",
            output.to_str().unwrap(),
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("Outcome: BLOCKED"));

    let report: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(output.join("readiness.json")).unwrap()).unwrap();
    let issues = report["findings"]
        .as_array()
        .unwrap()
        .iter()
        .find(|finding| finding["artifact"] == "issues")
        .unwrap();
    assert_eq!(issues["captured"], false);
    assert_eq!(issues["count"], serde_json::Value::Null);
    assert_eq!(issues["result"], "incomplete evidence");
    assert!(
        report["incomplete"]["issues"]
            .as_str()
            .unwrap()
            .contains("not valid Issues evidence")
    );
    Command::cargo_bin("git-forge-exit-drill")
        .unwrap()
        .env("GFED_PASSPHRASE", "correct horse battery")
        .args(["verify", output.join("evidence.gfed").to_str().unwrap()])
        .assert()
        .success();
}

#[test]
fn null_critical_records_are_incomplete_not_captured() {
    let directory = tempdir().unwrap();
    let source = directory.path().join("null-record-export");
    let output = directory.path().join("result");
    fs::create_dir(&source).unwrap();
    fs::write(
        source.join("manifest.json"),
        r#"{"repository":"acme/null-records","artifacts":{"issues":1,"pull_requests":1,"releases":1,"actions_workflows":1,"actions_runs":1}}"#,
    )
    .unwrap();
    for name in [
        "issues.json",
        "pull_requests.json",
        "releases.json",
        "workflows.json",
        "workflow_runs.json",
    ] {
        fs::write(source.join(name), "[null]").unwrap();
    }
    create_valid_mirror(&source);

    Command::cargo_bin("git-forge-exit-drill")
        .unwrap()
        .env("GFED_PASSPHRASE", "correct horse battery")
        .args([
            "drill",
            "--source",
            source.to_str().unwrap(),
            "--target",
            "forgejo:9.0",
            "--output",
            output.to_str().unwrap(),
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("Outcome: BLOCKED"));

    let report: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(output.join("readiness.json")).unwrap()).unwrap();
    for artifact in [
        "issues",
        "pull_requests",
        "releases",
        "actions_workflows",
        "actions_runs",
    ] {
        let finding = report["findings"]
            .as_array()
            .unwrap()
            .iter()
            .find(|finding| finding["artifact"] == artifact)
            .unwrap();
        assert_eq!(finding["captured"], false, "{artifact}");
        assert_eq!(finding["count"], serde_json::Value::Null, "{artifact}");
        assert_eq!(finding["result"], "incomplete evidence", "{artifact}");
        assert!(
            report["incomplete"][artifact]
                .as_str()
                .unwrap()
                .contains("record 1 must be a JSON object"),
            "{artifact} should identify the invalid record"
        );
    }
    assert_eq!(report["outcome"], "blocked");
    Command::cargo_bin("git-forge-exit-drill")
        .unwrap()
        .env("GFED_PASSPHRASE", "correct horse battery")
        .args(["verify", output.join("evidence.gfed").to_str().unwrap()])
        .assert()
        .success();
}

#[test]
fn invalid_and_mixed_issue_records_count_only_valid_records() {
    let directory = tempdir().unwrap();
    let source = directory.path().join("invalid-issue-records");
    fs::create_dir(&source).unwrap();
    fs::write(
        source.join("manifest.json"),
        r#"{"repository":"acme/invalid-issues","artifacts":{"issues":1}}"#,
    )
    .unwrap();
    create_valid_mirror(&source);

    for (name, content, expected_reason) in [
        ("null", "[null]", "must be a JSON object"),
        ("scalar", "[9]", "must be a JSON object"),
        ("unrelated", r#"[{"id":9}]"#, "must include a title"),
        (
            "missing-identity",
            r#"[{"title":"No identity","author":"mira"}]"#,
            "must include an id or number",
        ),
        (
            "missing-author",
            r#"[{"number":9,"title":"No author"}]"#,
            "must include an issue or pull request author",
        ),
    ] {
        fs::write(source.join("issues.json"), content).unwrap();
        let output = directory.path().join(name);
        Command::cargo_bin("git-forge-exit-drill")
            .unwrap()
            .env("GFED_PASSPHRASE", "correct horse battery")
            .args([
                "drill",
                "--source",
                source.to_str().unwrap(),
                "--target",
                "forgejo:9.0",
                "--output",
                output.to_str().unwrap(),
            ])
            .assert()
            .success();
        let report: serde_json::Value =
            serde_json::from_str(&fs::read_to_string(output.join("readiness.json")).unwrap())
                .unwrap();
        let issues = report["findings"]
            .as_array()
            .unwrap()
            .iter()
            .find(|finding| finding["artifact"] == "issues")
            .unwrap();
        assert_eq!(issues["captured"], false, "{name}");
        assert_eq!(issues["count"], serde_json::Value::Null, "{name}");
        assert_eq!(issues["result"], "incomplete evidence", "{name}");
        assert!(
            report["incomplete"]["issues"]
                .as_str()
                .unwrap()
                .contains(expected_reason),
            "{name} should explain its invalid record"
        );
    }

    fs::write(
        source.join("issues.json"),
        r#"[
          {"number": 81, "title": "Valid evidence", "author": "mira"},
          null,
          4,
          {"number": 82, "title": "Missing author"}
        ]"#,
    )
    .unwrap();
    let output = directory.path().join("mixed");
    Command::cargo_bin("git-forge-exit-drill")
        .unwrap()
        .env("GFED_PASSPHRASE", "correct horse battery")
        .args([
            "drill",
            "--source",
            source.to_str().unwrap(),
            "--target",
            "forgejo:9.0",
            "--output",
            output.to_str().unwrap(),
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("Outcome: BLOCKED"));
    let report: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(output.join("readiness.json")).unwrap()).unwrap();
    let issues = report["findings"]
        .as_array()
        .unwrap()
        .iter()
        .find(|finding| finding["artifact"] == "issues")
        .unwrap();
    assert_eq!(issues["captured"], false);
    assert_eq!(issues["count"], 1);
    assert_eq!(issues["result"], "incomplete evidence");
    assert!(
        report["incomplete"]["issues"]
            .as_str()
            .unwrap()
            .contains("record 2 must be a JSON object")
    );
}

#[test]
fn manifest_totals_without_records_are_incomplete_not_captured() {
    let directory = tempdir().unwrap();
    let source = directory.path().join("manifest-only-export");
    let output = directory.path().join("result");
    fs::create_dir(&source).unwrap();
    fs::write(
        source.join("manifest.json"),
        r#"{"repository":"acme/manifest-only","artifacts":{"issues":999,"pull_requests":888,"releases":777,"actions_workflows":666,"actions_runs":555}}"#,
    )
    .unwrap();
    let mirror = source.join("mirror.git");
    assert!(
        std::process::Command::new("git")
            .args(["init", "--bare", "--quiet"])
            .arg(&mirror)
            .status()
            .unwrap()
            .success()
    );
    let mut import = std::process::Command::new("git")
        .arg(format!("--git-dir={}", mirror.display()))
        .args(["fast-import", "--quiet"])
        .stdin(std::process::Stdio::piped())
        .spawn()
        .unwrap();
    use std::io::Write;
    import
        .stdin
        .take()
        .unwrap()
        .write_all(b"blob\nmark :1\ndata 5\nhello\ncommit refs/heads/main\nauthor Test <test@example.invalid> 0 +0000\ncommitter Test <test@example.invalid> 0 +0000\ndata 5\nseed\nM 100644 :1 README.md\n\ndone\n")
        .unwrap();
    assert!(import.wait().unwrap().success());

    Command::cargo_bin("git-forge-exit-drill")
        .unwrap()
        .env("GFED_PASSPHRASE", "correct horse battery")
        .args([
            "drill",
            "--source",
            source.to_str().unwrap(),
            "--target",
            "forgejo:9.0",
            "--output",
            output.to_str().unwrap(),
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("Outcome: BLOCKED"));

    let report: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(output.join("readiness.json")).unwrap()).unwrap();
    for artifact in [
        "issues",
        "pull_requests",
        "releases",
        "actions_workflows",
        "actions_runs",
    ] {
        let finding = report["findings"]
            .as_array()
            .unwrap()
            .iter()
            .find(|finding| finding["artifact"] == artifact)
            .unwrap();
        assert_eq!(finding["captured"], false, "{artifact}");
        assert_eq!(finding["result"], "incomplete evidence", "{artifact}");
    }
    assert_eq!(report["outcome"], "blocked");
}

#[test]
fn bundled_sample_counts_match_its_parseable_evidence() {
    let directory = tempdir().unwrap();
    let output = directory.path().join("result");
    let sample =
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("examples/atlas-notes-export");
    Command::cargo_bin("git-forge-exit-drill")
        .unwrap()
        .env("GFED_PASSPHRASE", "correct horse battery")
        .args([
            "drill",
            "--source",
            sample.to_str().unwrap(),
            "--target",
            "forgejo:9.0",
            "--output",
            output.to_str().unwrap(),
        ])
        .assert()
        .success();
    let report: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(output.join("readiness.json")).unwrap()).unwrap();
    for (artifact, count) in [
        ("issues", 2),
        ("pull_requests", 2),
        ("releases", 1),
        ("release_assets", 2),
        ("actions_workflows", 1),
        ("actions_runs", 1),
    ] {
        let finding = report["findings"]
            .as_array()
            .unwrap()
            .iter()
            .find(|finding| finding["artifact"] == artifact)
            .unwrap();
        assert_eq!(finding["captured"], true, "{artifact}");
        assert_eq!(finding["count"], count, "{artifact}");
    }
}

#[test]
fn empty_bare_repository_is_not_captured_as_git_history() {
    let directory = tempdir().unwrap();
    let source = directory.path().join("empty-repository-export");
    let output = directory.path().join("result");
    fs::create_dir(&source).unwrap();
    fs::write(
        source.join("manifest.json"),
        r#"{"repository":"acme/empty-repository","artifacts":{"git_repository":1,"issues":1}}"#,
    )
    .unwrap();
    fs::write(source.join("issues.json"), "[]").unwrap();
    std::process::Command::new("git")
        .args(["init", "--bare", "--quiet"])
        .arg(source.join("empty.git"))
        .status()
        .unwrap();

    Command::cargo_bin("git-forge-exit-drill")
        .unwrap()
        .env("GFED_PASSPHRASE", "correct horse battery")
        .args([
            "--json",
            "drill",
            "--source",
            source.to_str().unwrap(),
            "--target",
            "forgejo:9.0",
            "--output",
            output.to_str().unwrap(),
        ])
        .assert()
        .success();

    let report: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(output.join("readiness.json")).unwrap()).unwrap();
    let git_finding = report["findings"]
        .as_array()
        .unwrap()
        .iter()
        .find(|finding| finding["artifact"] == "git_repository")
        .unwrap();
    assert_eq!(git_finding["captured"], false);
    assert_eq!(git_finding["count"], serde_json::Value::Null);
    assert_eq!(git_finding["result"], "missing evidence");
    assert_eq!(report["outcome"], "blocked");
}

#[test]
fn portfolio_rejects_eleven_total_sources_before_license_or_output() {
    let directory = tempdir().unwrap();
    let output = directory.path().join("portfolio");
    let sample =
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("examples/atlas-notes-export");
    let mut arguments = vec!["portfolio".to_owned()];
    for _ in 0..11 {
        arguments.push("--source".to_owned());
        arguments.push(sample.to_string_lossy().into_owned());
    }
    arguments.extend([
        "--target".to_owned(),
        "forgejo:9.0".to_owned(),
        "--output".to_owned(),
        output.to_string_lossy().into_owned(),
    ]);

    Command::cargo_bin("git-forge-exit-drill")
        .unwrap()
        .args(arguments)
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "portfolio accepts at most 10 export directories; remove 1 --source value(s) and try again",
        ))
        .stderr(predicate::str::contains("Team Pack needs a license").not());
    assert!(!output.exists());
}

#[test]
fn local_drill_is_scriptable_and_verifiable() {
    let directory = tempdir().unwrap();
    let output = directory.path().join("result");
    let sample =
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("examples/atlas-notes-export");
    Command::cargo_bin("git-forge-exit-drill")
        .unwrap()
        .env("GFED_PASSPHRASE", "correct horse battery")
        .args([
            "--json",
            "drill",
            "--source",
            sample.to_str().unwrap(),
            "--target",
            "gitlab:17.0",
            "--output",
            output.to_str().unwrap(),
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("archive_sha256"));
    let archive = output.join("evidence.gfed");
    Command::cargo_bin("git-forge-exit-drill")
        .unwrap()
        .env("GFED_PASSPHRASE", "correct horse battery")
        .args(["verify", archive.to_str().unwrap()])
        .assert()
        .success()
        .stdout(predicate::str::contains("Archive verified"));
    let encrypted = fs::read(archive).unwrap();
    assert!(
        !encrypted
            .windows(b"Keep author attribution".len())
            .any(|window| window == b"Keep author attribution")
    );
}

#[test]
fn empty_export_explains_the_next_step() {
    let directory = tempdir().unwrap();
    Command::cargo_bin("git-forge-exit-drill")
        .unwrap()
        .env("GFED_PASSPHRASE", "correct horse battery")
        .args([
            "drill",
            "--source",
            directory.path().to_str().unwrap(),
            "--target",
            "forgejo:9.0",
        ])
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "is empty; extract the GitHub export and try again",
        ));
}

#[test]
fn help_names_the_real_job() {
    Command::cargo_bin("git-forge-exit-drill")
        .unwrap()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Test a GitHub move"))
        .stdout(predicate::str::contains("portfolio"));
}
