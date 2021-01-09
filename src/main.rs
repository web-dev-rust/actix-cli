use structopt::StructOpt;

mod cargo;
mod main_core;
mod error;
mod input;
mod setup;
mod config;

use input::Opt;

fn main() {
    let opt = Opt::from_args();

    // // Cargo
    // let _ = cargo::check_cargo_version().unwrap();
    // let _ = cargo::create_cargo(opt.fault_tolerant, opt.name.clone());

    // Config.toml
    let toml = config::read_config_toml().unwrap();
    let configs = config::crud::create_crud_info(toml, opt.name.clone()).unwrap();

    // // main.rs
    // let _ = main_core::create_main(opt.name.clone(), opt.fault_tolerant, opt.request_logger).unwrap();
    // let _ = main_core::routes::create_routes_rs(opt.name, configs).unwrap();

    // // build
    // let _ = setup::build().unwrap();
}
