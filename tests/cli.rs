use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn fakes_commands() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("fakemall")?;

    cmd.arg("exec")
        .arg("tests/test_config.toml")
        .arg("dmidecode");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("this is a sample output"));

    Ok(())
}

#[test]
fn set_doesnt_exist() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("fakemall")?;

    cmd.arg("exec").arg("test/file/doesnt/exist").arg("kekw");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("No such file or directory"));

    Ok(())
}

#[test]
fn command_doesnt_exist() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("fakemall")?;

    cmd.arg("exec").arg("tests/test_config.toml").arg("asdf");
    cmd.assert()
        .failure()
        .stdout(predicate::str::contains("asdf: command not found"));

    Ok(())
}
