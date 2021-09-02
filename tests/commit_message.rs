use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*; // Used for writing assertions
use std::process::Command; // Run programs

#[test]
fn display_help() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("commit_message")?;

    cmd.arg("--help");
    cmd.assert().success();
    Ok(())
}

#[test]
fn add_message() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("commit_message")?;

    cmd.args(&["--message", "fix issue with lint error"]);
    cmd.assert().success();
    cmd.assert()
        .stdout("feat: initial_working_version fix issue with lint error\n");
    Ok(())
}
