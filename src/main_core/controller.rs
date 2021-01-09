use std::fs::OpenOptions;
use std::io::prelude::*;
use std::fs;
use crate::error::ActixCliError;
use crate::config::crud::CrudConfig;

const fn routes() -> &'static str {
    r#"
{project_use}
use actix_web::{web, HttpResponse};

pub fn app_routes(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/")
            {projet_scopes}
            .route("ping", web::get().to(pong))
            .route("~/ready", web::get().to(readiness))
            .route("", web::get().to(|| HttpResponse::NotFound())),
    );
}
// Move to controller/mod.rs
// use actix_web::{HttpResponse, Responder};
use actix_web::Responder;

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

const fn modules() -> &'static str {
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

pub fn create_controllers_rs(name: String, routes_config: Option<CrudConfig>) -> Result<(), ActixCliError> {
    let _name = name.replace("-", "_");
    fs::create_dir(format!("./{}/src/{}_web/controllers", name, _name))?;

    let mut file = OpenOptions::new().write(true)
        .create_new(true)
        .open(format!("./{}/src/{}_web/controllers/mod.rs", name, _name))?;

    if let Err(e) = writeln!(file, "{}", 
        if routes_config.is_some() {
            module_path!().replace("{crud}", "pub mod crud;")
        } 
        else {modules().replace("{crud}", "")}) 
    {
        eprintln!("Couldn't create src/{}_web/controllers/mod.rs: {}",name, e);
    }

    // let mut file = OpenOptions::new().write(true)
    //     .create_new(true)
    //     .open(format!("./{}/src/{}_web/routes.rs", name, _name))?;

    // let crud_routes = if routes_config.is_some() { 
    //     routes()
    //         .replace("{projet_scopes}", &routes_config.clone().unwrap().routes) 
    //         .replace("{project_use}", &routes_config.unwrap().project_use)
    // } else { 
    //     routes().replace("{projet_scopes}", "").replace("{project_use}", "")
    // };

    // if let Err(e) = writeln!(file, "{}", crud_routes) {
    //     eprintln!("Couldn't create src/{}_web/routes.rs: {}", name, e);
    // }

    Ok(())
}