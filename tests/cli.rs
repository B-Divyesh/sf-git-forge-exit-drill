use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::tempdir;

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
