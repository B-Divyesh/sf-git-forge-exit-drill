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
