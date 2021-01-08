use structopt::StructOpt;

mod cargo;
mod error;
mod input;

use input::Opt;

fn main() {
    let opt = Opt::from_args();

    // Cargo
    let _ = cargo::check_cargo_version().unwrap();
    let _ = cargo::create_cargo(opt.fault_tolerant, opt.name);
}
