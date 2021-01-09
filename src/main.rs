use structopt::StructOpt;

mod cargo;
mod main_core;
mod error;
mod input;
mod setup;
mod config;
mod database;

use input::Opt;

fn main() {
    let opt = Opt::from_args();

    // Cargo
    let _ = cargo::check_cargo_version().unwrap();
    let _ = cargo::create_cargo(opt.fault_tolerant, opt.name.clone());

    // Config.toml
    let path: String = opt.config_file.unwrap().into_os_string().into_string().unwrap();
    let toml = config::read_config_toml(path).unwrap();
    let configs = config::crud::create_crud_info(toml, opt.name.clone()).unwrap();

    // main.rs
    let _ = main_core::create_main(opt.name.clone(), opt.fault_tolerant, opt.request_logger).unwrap();
    let _ = main_core::routes::create_routes_rs(opt.name.clone(), configs.clone()).unwrap();
    let _ = main_core::controller::create_controllers_rs(opt.name.clone(), configs.clone()).unwrap();
    if configs.is_some() {
        let _ = main_core::controller::create_crud_controller_rs(opt.name.clone(), configs.clone().unwrap().model_name);
    }

    let _ = main_core::model::create_crud_model_rs(opt.name.clone(), configs.clone()).unwrap();
    let _ = database::create_crud_model_rs(opt.name.clone(), configs.clone()).unwrap();

    // build
    let _ = setup::build().unwrap();
}
