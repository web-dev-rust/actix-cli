use crate::error::ActixCliError;
use structopt::StructOpt;

/// A basic example
#[derive(StructOpt, Debug)]
#[structopt(name = "actix-cli")]
pub struct Opt {
    /// Enables Bastion for fault tolerant system
    #[structopt(short, long, parse(try_from_str = fault_tolerant), default_value = "true")]
    pub fault_tolerant: bool,
    /// Enables request logger as `[IP:%a DATETIME:%t REQUEST:\"%r\" STATUS: %s DURATION:%D X-REQUEST-ID:%{x-request-id}o]
    /// and `"[x-request-id: Uuid::new_v4()]`
    #[structopt(short, long, parse(try_from_str = request_logger), default_value = "true")]
    pub request_logger: bool,
    /// Defines project name in Cargo.toml
    #[structopt(short, long, parse(try_from_str = name))]
    pub name: String,

}

fn fault_tolerant(s: &str) -> Result<bool, ActixCliError> {
    match s {
        "true" => Ok(true),
        "false" => Ok(false),
        _ => Err(ActixCliError::FaultTolerantTypeError),
    }
}

fn request_logger(s: &str) -> Result<bool, ActixCliError> {
    match s {
        "true" => Ok(true),
        "false" => Ok(false),
        _ => Err(ActixCliError::EnableRequestLogger),
    }
}

fn name(s: &str) -> Result<String, ActixCliError> {
    if s.is_empty() {
        Err(ActixCliError::ProjectNameMustBeDefined)
    } else {
        Ok(String::from(s))
    }
}
