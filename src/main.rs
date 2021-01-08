use structopt::StructOpt;

mod cargo;
mod main_core;
mod error;
mod input;
mod setup;

use input::Opt;

fn main() {
    let opt = Opt::from_args();

    // Cargo
    let _ = cargo::check_cargo_version().unwrap();
    let _ = cargo::create_cargo(opt.fault_tolerant, opt.name.clone());

    // main.rs
    let _ = main_core::create_main(opt.name.clone(), opt.fault_tolerant, opt.request_logger).unwrap();
    let _ = main_core::routes::create_routes_rs(opt.name).unwrap();

    // build
    let _ = setup::build().unwrap();
}
