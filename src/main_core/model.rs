use std::fs::OpenOptions;
use std::io::prelude::*;
use std::fs;
use convert_case::{Case, Casing};
use crate::error::ActixCliError;
use crate::config::crud::CrudConfig;

const DERIVE: &'static str = "
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]";
const fn modules() -> &'static str {
    r#"pub mod model;
pub mod database;
    "#
}

fn model_modules(name: &str) -> String {
    String::from("pub mod ") + name + ";"
}

pub fn create_crud_model_rs(name: String, model_config: Option<CrudConfig>) -> Result<(), ActixCliError> {
    let _name = name.replace("-", "_");
    fs::create_dir(format!("./{}/src/{}", name, _name))?;

    let mut file = OpenOptions::new().write(true)
        .create_new(true)
        .open(format!("./{}/src/{}/mod.rs", name, _name))?;

    if let Err(e) = writeln!(file, "{}", modules()) {
        eprintln!("Couldn't create src/{}/mod.rs: {}",name, e);
    }

    fs::create_dir(format!("./{}/src/{}/model", name, _name))?;

    let mut file = OpenOptions::new().write(true)
        .create_new(true)
        .open(format!("./{}/src/{}/model/mod.rs", name, _name))?;

    if let Err(e) = writeln!(file, "{}", model_modules(&_name)) {
        eprintln!("Couldn't create src/{}/model/mod.rs: {}",name, e);
    }

    
    let mut file = OpenOptions::new().write(true)
        .create_new(true)
        .open(format!("./{}/src/{}/model/{}.rs", name, _name, _name))?;

    if model_config.is_some() { 
        let model = model_config.clone().unwrap().model;
        if let Err(e) = writeln!(file, "{} ", "use serde::{Deserialize, Serialize};") {
            eprintln!("Couldn't create src/{}/model/{}.rs: {}", _name, _name, e);
        }

        if let Err(e) = writeln!(file, "{} ", serde_obj(model.clone())) {
            eprintln!("Couldn't create src/{}/model/{}.rs: {}", _name, _name, e);
        }

        if let Err(e) = writeln!(file, "{} ", serde_obj_update(&model_config.clone().unwrap().model_name, model.clone()))
             {
            eprintln!("Couldn't create src/{}/model/{}.rs: {}", _name,  _name, e);
        }

        if let Err(e) = writeln!(file, "{} ", serde_obj_response(&model_config.unwrap().model_name))
             {
            eprintln!("Couldn't create src/{}/model/{}.rs: {}", _name,  _name, e);
        }
    }

    Ok(())
}

fn serde_obj(model: String) -> String {
    String::from(DERIVE)
    + &model
}

fn serde_obj_update(name: &str, model: String) -> String {
    let update = name.to_case(Case::Pascal) + "Update";
    String::from(DERIVE)
    + &model.replace(": ", ": Option<")
    .replace(",", ">,")
    .replace(&name.to_case(Case::Pascal), &update)
}

fn serde_obj_response(name: &str) -> String {
    let response = name.to_case(Case::Pascal) + "Response";
    String::from(DERIVE) + "
pub struct " + &response + "
{pub object: "+ &name.to_case(Case::Pascal) + ", pub id: String,}"
}

