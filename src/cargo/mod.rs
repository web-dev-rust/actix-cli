use std::fs::OpenOptions;
use std::io::prelude::*;
use std::process::Command;

use crate::error::ActixCliError;

pub fn check_cargo_version() -> Result<usize, ActixCliError> {
    let version_bytes = Command::new("cargo").arg("-V").output()?.stdout;

    let version_str = std::str::from_utf8(version_bytes.as_slice())
        .map_err(|_| ActixCliError::CargoCheckFailed)?;
    let version = version_str.split(" ").collect::<Vec<&str>>()[1];
    let version = version.split(".").collect::<Vec<&str>>()[1]
        .parse::<usize>()
        .unwrap_or(0);

    if version > 45 {
        Ok(version)
    } else {
        Err(ActixCliError::CargoNotAvailable)
    }
}

pub fn create_cargo(enable_bastion: bool, name: String) -> Result<(), ActixCliError> {
    Command::new("cargo")
        .arg("new")
        .arg(&name)
        .arg("--bin")
        .output()?
        .stdout;

    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(format!("./{}/Cargo.toml", name))
        .unwrap();

    vec![
        "actix = \"0.10.0\"",
        "actix-web = \"3.3.2\"",
        "actix-rt = \"1.1.1\"",
        "num_cpus = \"1.0\"",
        "log = \"0.4.11\"",
        "env_logger = \"0.8.2\"",
        "uuid = \"0.8.1\"",
        "serde = { version = \"1.0.104\", features = [\"derive\"] }",
        "serde_json = \"1.0.44\"",
        "serde_derive = \"1.0.104\"",
    ]
    .into_iter()
    .for_each(|line| {
        if let Err(e) = writeln!(file, "{}", line) {
            eprintln!("Couldn't add actix crates to Cargo.toml: {}", e);
        }
    });

    if enable_bastion {
        vec!["fort = \"0.4\"", "bastion = \"0.4.3\""]
            .into_iter()
            .for_each(|line| {
                if let Err(e) = writeln!(file, "{}", line) {
                    eprintln!("Couldn't add bastion crates to Cargo.toml: {}", e);
                }
            });
    }

    Ok(())
}
