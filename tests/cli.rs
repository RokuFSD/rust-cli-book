use assert_cmd::prelude::*; // Add methods on commands
use assert_fs::prelude::*;
use predicates::prelude::*; // Used for writing assertions
use std::process::Command; // Run programs

#[test]
fn file_doesnt_exist() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("rust-cli-book")?;
    cmd.arg("test/file/doesnt/exist")
        .arg("--pattern")
        .arg("Rust");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Error opening file"));
    Ok(())
}

#[test]
fn find_content_in_file() -> Result<(), Box<dyn std::error::Error>> {
    let file = assert_fs::NamedTempFile::new("sample.txt")?;
    file.write_str("Rust is awesome")?;

    let mut cmd = Command::cargo_bin("rust-cli-book")?;
    cmd.arg(file.path()).arg("--pattern").arg("Rust");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Rust is awesome"));
    Ok(())
}

#[test]
fn pattern_is_empty() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("rust-cli-book")?;
    cmd.arg("test/file/doesnt/exist").arg("--pattern");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("a value is required for"));
    Ok(())
}
