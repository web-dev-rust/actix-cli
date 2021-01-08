use std::fs::OpenOptions;
use std::io::prelude::*;
use std::fs;
use crate::error::ActixCliError;

const fn routes() -> &'static str {
    r#"
//{project_use}
use actix_web::{web, HttpResponse};

pub fn app_routes(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/")
            // {projet_scopes}
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
    //pub mod controller;
    //pub mod schema;
    pub mod routes;
    //pub mod middleware;
    "#
}

pub fn create_routes_rs(name: String) -> Result<(), ActixCliError> {
    let _name = name.replace("-", "_");
    fs::create_dir(format!("./{}/src/{}_web", name, _name))?;

    let mut file = OpenOptions::new().write(true)
        .create_new(true)
        .open(format!("./{}/src/{}_web/mod.rs", name, _name))?;

    if let Err(e) = writeln!(file, "{}", modules()) {
        eprintln!("Couldn't create src/{}_web/mod.rs: {}",name, e);
    }

    let mut file = OpenOptions::new().write(true)
        .create_new(true)
        .open(format!("./{}/src/{}_web/routes.rs", name, _name))?;

    if let Err(e) = writeln!(file, "{}", routes()) {
        eprintln!("Couldn't create src/{}_web/routes.rs: {}", name, e);
    }

    Ok(())
}