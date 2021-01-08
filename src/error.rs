use std::error::Error;
use std::fmt;
use std::io;

#[derive(Debug, Clone)]
pub enum ActixCliError {
    CommandExecutionFailed,
    CargoCheckFailed,
    CargoNotAvailable,
    FaultTolerantTypeError,
    ProjectNameMustBeDefined,
    EnableRequestLogger,
}

impl fmt::Display for ActixCliError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ActixCliError::CommandExecutionFailed => {
                write!(f, "Rust was not able to execute command")
            }
            ActixCliError::CargoCheckFailed => write!(f, "Failed to check cargo version"),
            ActixCliError::CargoNotAvailable => {
                write!(f, "Cargo is required, please visit https://rustup.rs")
            }
            ActixCliError::FaultTolerantTypeError => {
                write!(f, "`fault-tolerant` should be `true` or `false`")
            }
            ActixCliError::ProjectNameMustBeDefined => write!(f, "Project name is required"),
            ActixCliError::EnableRequestLogger => {
                write!(f, "`request-logger` should be `true` or `false`")
            }
        }
    }
}

impl Error for ActixCliError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(self)
    }
}

impl From<io::Error> for ActixCliError {
    fn from(_: io::Error) -> Self {
        ActixCliError::CommandExecutionFailed
    }
}
