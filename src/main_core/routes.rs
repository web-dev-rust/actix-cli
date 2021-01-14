use std::fs::OpenOptions;
use std::io::prelude::*;
use std::fs;
use crate::error::ActixCliError;
use crate::config::crud::CrudConfig;

const fn routes() -> &'static str {
    r#"
use actix_web::{web, HttpResponse};
use crate::{project_name}_web::controllers::{ pong, readiness };
{project_use}

pub fn app_routes(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/")
            {projet_scopes}
            .route("ping", web::get().to(pong))
            .route("~/ready", web::get().to(readiness))
            .route("", web::get().to(|| HttpResponse::NotFound())),
    );
}
"#
}

const fn modules() -> &'static str {
    r#"
    pub mod controllers;
    pub mod routes;
    //pub mod middleware;
    "#
}

pub fn create_routes_rs(name: String, routes_config: Option<CrudConfig>) -> Result<(), ActixCliError> {
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

    let crud_routes = if routes_config.is_some() { 
        routes()
            .replace("{projet_scopes}", &routes_config.clone().unwrap().routes) 
            .replace("{project_use}", &routes_config.unwrap().project_use)
    } else { 
        routes().replace("{projet_scopes}", "").replace("{project_use}", "")
    }.replace("{project_name}", &_name);

    if let Err(e) = writeln!(file, "{}", crud_routes) {
        eprintln!("Couldn't create src/{}_web/routes.rs: {}", name, e);
    }

    Ok(())
}