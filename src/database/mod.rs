use std::fs::OpenOptions;
use std::io::prelude::*;
use std::fs;
use convert_case::{Case, Casing};

pub mod context;

use crate::config::crud::CrudConfig;
use crate::error::ActixCliError;

pub fn create_crud_model_rs(name: String, model_config: Option<CrudConfig>) -> Result<(), ActixCliError> {
    let _name = name.replace("-", "_");
    fs::create_dir(format!("./{}/src/{}/database", name, _name))?;

    let mut file = OpenOptions::new().write(true)
        .create_new(true)
        .open(format!("./{}/src/{}/database/mod.rs", name, _name))?;

    if model_config.is_some() {
        if let Err(e) = writeln!(file, "{}", &database_info(&_name, &model_config.unwrap().model_name)) {
            eprintln!("Couldn't create src/{}/database/mod.rs: {}",name, e);
        }
    }

    Ok(())
}

fn database_info(project_name: &str, module_name: &str) -> String {
    let obj = module_name.to_case(Case::Pascal);
"use crate::{project_name}::model::{project_name}::{@, @Update, @Response};
use std::collections::HashMap;
pub struct Context(HashMap<String, @>);

impl Context {
    pub fn new(map: HashMap<String, @>) -> Self {
        Context{0: map}
    }

    pub fn create(&mut self, key: String, value: @) -> bool {
        match self.0.insert(key, value) {
            None => true,
            Some(_) => false,
        }
    }

    pub fn get(&mut self, key: String) -> Option<@>{
        match self.0.get(&key) {
            None => None,
            Some(s) => Some(s.to_owned())
        }
    }

    pub fn delete(&mut self, key: String) -> bool {
        match self.0.remove(&key) {
            None => false,
            Some(_) => true
        }
    }

    pub fn all(&mut self) -> Vec<ObjectResponse> {
        self.0
            .iter()
            .map(|(k, v)| ObjectResponse {
                object: v.clone(),
                id: k.to_string(),
            })
            .collect::<Vec<ObjectResponse>>()
    }

    pub fn update(&mut self, key: String, value: ObjectUpdate) -> bool {
        if let Some(x) = self.0.get_mut(&key) {
            // EXAMPLE CODE, update to your needs
            if value.age.is_none() && value.name.is_none() && value.school.is_none() {
                return false;
            }

            if value.age.is_some() {
                x.age = value.age.unwrap();
            }

            if value.name.is_some() {
                x.name = value.name.unwrap();
            }

            if value.school.is_some() {
                x.school = value.school.unwrap();
            }

            true
        } else {
            false
        }
    }
}
".replace("@", &obj).replace("{project_name}", project_name)
}

