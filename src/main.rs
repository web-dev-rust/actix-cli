use structopt::StructOpt;

mod cargo;
mod main_core;
mod error;
mod input;

use input::Opt;

fn main() {
    let opt = Opt::from_args();

    // Cargo
    let _ = cargo::check_cargo_version().unwrap();
    let _ = cargo::create_cargo(opt.fault_tolerant, opt.name.clone());

    // main.rs
    let _ = main_core::create_main(opt.name, opt.fault_tolerant, opt.request_logger);//.unwrap();
}
