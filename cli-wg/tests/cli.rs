#[cfg(test)]
mod cli_test {
  extern crate assert_cmd;

  use std::process::Command;
  use std::error;
  use std::io::Write;

  use assert_cmd::prelude::*;
  use predicates::prelude::*;
  use tempfile::NamedTempFile;

  #[test]
  fn file_deesnt_exist() -> Result<(), Box<dyn error::Error>> {
    let mut cmd = Command::cargo_bin("cli_wg")?;

    cmd
      .arg("foobar")
      .arg("test/file/doesnt/exist");

    cmd
      .assert()
      .failure()
      .stderr(predicate::str::contains("The system cannot find the path specified"));

    Ok(())
  }

  #[test]
  fn find_content_in_file() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = NamedTempFile::new()?;
    writeln!(file, "A test\nActual content\nMore content\nAnother test")?;

    let mut cmd = Command::cargo_bin("cli_wg")?;

    cmd
      .arg("test")
      .arg(file.path());

    cmd
      .assert()
      .success()
      .stdout(predicate::str::contains("test\nAnother test"));

    Ok(())
  }

  // fn empty_pattern_test() 
}