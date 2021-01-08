use std::process::Command;
use crate::error::ActixCliError;

pub fn build() -> Result<(), ActixCliError> {
    Command::new("git")
        .arg("init")
        .output()?
        .stdout;

    Command::new("cargo")
        .arg("build")
        .output()?
        .stdout;

    Ok(())
}