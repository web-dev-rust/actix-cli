use std::fs::OpenOptions;
use std::io::prelude::*;
use toml::from_str;
use crate::error::ActixCliError;

pub mod crud;

pub fn read_config_toml(path: String) -> Result<toml::Value, ActixCliError> {
    let mut file = OpenOptions::new().read(true).open(&path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let toml_content: toml::Value = from_str(&contents)?;
    Ok(toml_content)
}