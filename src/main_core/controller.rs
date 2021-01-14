use std::fs::OpenOptions;
use std::io::prelude::*;
use std::fs;
use convert_case::{Case, Casing};

use crate::error::ActixCliError;
use crate::config::crud::CrudConfig;

fn crud_controller(name: &str, object: &str) -> String {
    let name = name.replace("-", "_");
    format!("
    use crate::{}::model::{}::*;
    use crate::{}::database::Context;
    use std::sync::{{Arc, Mutex}};

    use actix_web::{{web, HttpResponse, Responder}};
    
    pub async fn create_{}(state: web::Data<Arc<Mutex<Context>>>, info: web::Bytes) -> impl Responder {{
        let id = uuid::Uuid::new_v4();
    
        let bytes_str = match String::from_utf8(info.to_vec()) {{
            Ok(text) => Ok(text),
            Err(_) => Err(\"bytes parse error\")
        }};
    
        let info = serde_json::from_str(&bytes_str.unwrap()).unwrap();
        let mut state = state.lock().unwrap();
        match state.create(id.to_string(), info) {{
            true => HttpResponse::Created().body(id.to_string()),
            false => HttpResponse::Conflict().finish(),
        }}
    }}
    
    pub async fn show_{}(state: web::Data<Arc<Mutex<Context>>>) -> impl Responder {{
        let mut state = state.lock().unwrap();
        let response = state.all();
        HttpResponse::Ok().json(response)
    }}

    pub async fn delete_{}(id: web::Path<String>, state: web::Data<Arc<Mutex<Context>>>) -> impl Responder {{
        let uuid = id.to_string();
    
        if uuid::Uuid::parse_str(&uuid).is_err() {{
            return HttpResponse::BadRequest().body(\"Id must be a Uuid::V4\");
        }}
    
        let mut state = state.lock().unwrap();
        match state.delete(id.to_string()) {{
            true => HttpResponse::Ok().finish(),
            false => HttpResponse::BadRequest().finish(),
        }}
    }}
    
    pub async fn get_{}(id: web::Path<String>, state: web::Data<Arc<Mutex<Context>>>) -> impl Responder {{
        let uuid = id.to_string();
    
        if uuid::Uuid::parse_str(&uuid).is_err() {{
            return HttpResponse::BadRequest().body(\"Id must be a Uuid::V4\");
        }}
    
        let mut state = state.lock().unwrap();
        match state.get(uuid) {{
            Some(body) => HttpResponse::Ok().json(body),
            None => HttpResponse::BadRequest().finish(),
        }}
    }}
    
    pub async fn update_{}(
        id: web::Path<String>,
        info: web::Bytes, 
        state: web::Data<Arc<Mutex<Context>>>) -> impl Responder {{
        let uuid = id.to_string();
    
        if uuid::Uuid::parse_str(&uuid).is_err() {{
            return HttpResponse::BadRequest().body(\"Id must be a Uuid::V4\");
        }}

        let bytes_str = match String::from_utf8(info.to_vec()) {{
            Ok(text) => Ok(text),
            Err(_) => Err(\"bytes parse error\")
        }};
    
        let info: ObjectUpdate = serde_json::from_str(&bytes_str.unwrap()).unwrap();
        let mut state = state.lock().unwrap();
        match state.update(id.to_string(), info) {{
            true => HttpResponse::Ok().finish(),
            false => HttpResponse::BadRequest().finish(),
        }}
    }}
", name, name, name, object, object, object, object, object).replace("@object", &object.to_case(Case::Pascal))
}

const fn module_path() -> &'static str {
    r#"
{crud}

use actix_web::{HttpResponse, Responder};

pub async fn pong() -> impl Responder {
    HttpResponse::Ok().body("pong")
}

pub async fn readiness() -> impl Responder {
    let process = std::process::Command::new("sh")
        .arg("-c")
        .arg("echo hello")
        .output();

    match process {
        Ok(_) => HttpResponse::Accepted(),
        Err(_) => HttpResponse::InternalServerError(),
    }
}
    "#
}

pub fn create_controllers_rs(name: String, crud_config: Option<CrudConfig>) -> Result<(), ActixCliError> {
    let _name = name.replace("-", "_");
    fs::create_dir(format!("./{}/src/{}_web/controllers", name, _name))?;

    let mut file = OpenOptions::new().write(true)
        .create_new(true)
        .open(format!("./{}/src/{}_web/controllers/mod.rs", name, _name))?;

    if let Err(e) = writeln!(file, "{}", 
        if crud_config.is_some() {
            module_path().replace("{crud}", "pub mod crud;")
        } 
        else {module_path().replace("{crud}", "")}) 
    {
        eprintln!("Couldn't create src/{}_web/controllers/mod.rs: {}",name, e);
    }

    Ok(())
}

pub fn create_crud_controller_rs(name: String, crud_name: String) -> Result<(), ActixCliError> {
    let _name = name.replace("-", "_");
    let mut file = OpenOptions::new().write(true)
        .create_new(true)
        .open(format!("./{}/src/{}_web/controllers/crud.rs", name, _name))?;

    if let Err(e) = writeln!(file, "{}", 
        crud_controller(&name, &crud_name)) 
    {
        eprintln!("Couldn't create src/{}_web/controllers/crud.rs: {}",name, e);
    }

    Ok(())
}