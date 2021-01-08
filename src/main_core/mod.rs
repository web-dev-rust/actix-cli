use std::fs::OpenOptions;
use std::io::prelude::*;
use crate::error::ActixCliError;

pub mod routes;

const fn main_content() -> &'static str {
    r#"
use actix_web::middleware::{DefaultHeaders, Logger};
use actix_web::{App, HttpServer};
{bastion_use}
use uuid::Uuid;

//mod {name};
mod {name}_web;

use {name}_web::routes::app_routes;
//{project_use}

#[actix_rt::main]
async fn {bastion_main_fn}() -> Result<(), std::io::Error> {
    {not_bastion}

    HttpServer::new(|| {
        App::new()
        //{data}
        {logger}
        .configure(app_routes)
    })
    .workers(num_cpus::get() + 2)
    .bind("0.0.0.0:4000")
    .unwrap()
    .run()
    .await
}

{bastion_main}
"#
}

const fn bastion_main() -> &'static str {
    r#"
#[fort::root]
async fn main(_: BastionContext) -> Result<(), ()> {
    std::env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();

    let _ = web_main();

    Ok(())
}"#
}

const fn common_main() -> &'static str {
    r#"
std::env::set_var("RUST_LOG", "actix_web=debug");
env_logger::init();
"#
}

const fn logger() -> &'static str {
    r#"
    .wrap(DefaultHeaders::new().header("x-request-id", Uuid::new_v4().to_string()))
    .wrap(Logger::new("IP:%a DATETIME:%t REQUEST:\"%r\" STATUS: %s DURATION:%D X-REQUEST-ID:%{x-request-id}o"))

    "#
}

pub fn create_main(name: String, bastion: bool, request_logger: bool) -> Result<(), ActixCliError> {
    let mut main = String::from(main_content());
    
    if let Some(idx) = main.find("{bastion_main}") {
        if bastion {
            main.replace_range(idx..=idx+14, bastion_main());
        } else {
            main.replace_range(idx..=idx+14, "");
        }
    }

    if let Some(idx) = main.find("{bastion_main_fn}") {
        if bastion {
            main.replace_range(idx..=idx+16, "web_main");
        } else {
            main.replace_range(idx..=idx+16, "main");
        }
    }

    if let Some(idx) = main.find("{bastion_use}") {
        if bastion {
            main.replace_range(idx..=idx+13, "use bastion::prelude::*;\n");
        } else {
            main.replace_range(idx..=idx+13, "");
        }
    }

    if let Some(idx) = main.find("{not_bastion}") {
        if bastion {
            main.replace_range(idx..=idx+13, "");
        } else {
            main.replace_range(idx..=idx+13, common_main());
        }
    }

    if let Some(idx) = main.find("{logger}") {
        if request_logger {
            main.replace_range(idx..=idx+8, logger());
        } else {
            main.replace_range(idx..=idx+8, "");
        }
    }

    let main = main.replace("{name}", &name.replace("-", "_"));
    let mut file = OpenOptions::new()
        .write(true)
        .open(format!("./{}/src/main.rs", name))?;

    if let Err(e) = writeln!(file, "{}", main) {
        eprintln!("Couldn't create src/main.rs: {}", e);
    }

    Ok(())
}