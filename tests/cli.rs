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
    assert!(fs::read_to_string(output.join("result/readiness.md"))
        .unwrap()
        .contains("| Git repository | Yes (1) | native | mapped |"));
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
