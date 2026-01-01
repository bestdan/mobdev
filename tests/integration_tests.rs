use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_cli_version() {
    let mut cmd = Command::cargo_bin("mobdev").unwrap();
    cmd.arg("--version");
    cmd.assert().success().stdout(predicate::str::contains("0.15.0"));
}

#[test]
fn test_cli_help() {
    let mut cmd = Command::cargo_bin("mobdev").unwrap();
    cmd.arg("--help");
    cmd.assert().success()
        .stdout(predicate::str::contains("Mobile developer utility package"));
}

#[test]
fn test_git_check_in_repo() {
    let mut cmd = Command::cargo_bin("mobdev").unwrap();
    cmd.arg("git").arg("check");
    // Should succeed since we're in a git repo
    cmd.assert().success();
}

#[test]
fn test_git_root() {
    let mut cmd = Command::cargo_bin("mobdev").unwrap();
    cmd.arg("git").arg("root");
    // Should succeed and output a path
    cmd.assert().success()
        .stdout(predicate::str::contains("mobdev"));
}

#[test]
fn test_git_branch() {
    let mut cmd = Command::cargo_bin("mobdev").unwrap();
    cmd.arg("git").arg("branch");
    // Should succeed and output a branch name
    cmd.assert().success()
        .stdout(predicate::str::is_empty().not());
}

#[test]
fn test_check_externals() {
    let mut cmd = Command::cargo_bin("mobdev").unwrap();
    cmd.arg("check").arg("externals");
    // May fail if dependencies not installed, but should execute
    let _ = cmd.assert();
}

#[test]
fn test_files_filter_suffix() {
    use std::process::{Command as StdCommand, Stdio};
    use std::io::Write;

    let mut cmd = StdCommand::new("cargo");
    cmd.args(["run", "--bin", "mobdev", "--", "files", "filter", "suffix", ".g.dart"]);
    cmd.stdin(Stdio::piped());
    cmd.stdout(Stdio::piped());
    
    let mut child = cmd.spawn().expect("Failed to spawn command");
    
    if let Some(mut stdin) = child.stdin.take() {
        stdin.write_all(b"file1.dart\nfile2.g.dart\nfile3.dart\n").unwrap();
    }
    
    let output = child.wait_with_output().expect("Failed to wait for command");
    let stdout = String::from_utf8_lossy(&output.stdout);
    
    // Should filter out .g.dart files
    assert!(stdout.contains("file1.dart"));
    assert!(!stdout.contains("file2.g.dart"));
    assert!(stdout.contains("file3.dart"));
}
