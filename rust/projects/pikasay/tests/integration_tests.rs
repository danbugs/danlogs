use std::process::Command;
use assert_cmd::prelude::*;
use predicates::prelude::*;

#[test]
fn run_with_defaults() -> Result<(), Box<dyn std::error::Error>> {
    Command::cargo_bin("pikasay")
        .expect("binary exists")
        .assert()
        .stdout(predicate::str::contains("Created by danlogs!"))
        .success();
    
    Ok(())
}

#[test]
fn fail_on_non_existing_file() -> Result<(), Box<dyn std::error::Error>>{
    Command::cargo_bin("pikasay")
    .expect("binary exists")
    .args(&["-f", "no/such/file.txt"])
    .assert()
    .failure();

    Ok(())
}