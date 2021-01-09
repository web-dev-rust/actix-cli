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
    FailedToParseConfigToml,
    ConfigTableFormat,
    // ConfigRequiresModelAndRoutesKey,
    CrudNameIsRequired,
    CrudStructType,
    UnknwonCrudItem,
    CrudRouteShouldBeString,
}

impl fmt::Display for ActixCliError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ActixCliError::CommandExecutionFailed => {
                write!(f, "Rust was not able to execute IO command")
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
            },
            ActixCliError::FailedToParseConfigToml => {
                write!(f, "Failed to parse Config.toml")
            },
            ActixCliError::ConfigTableFormat => {
                write!(f, "Config.toml should be table based")
            },
            // ActixCliError::ConfigRequiresModelAndRoutesKey => {
            //     write!(f, "Config.toml CRUD must contain `model` and `routes` keys")
            // },
            ActixCliError::CrudNameIsRequired => {
                write!(f, "Config.toml CRUD must contain a String for `name`")
            },
            ActixCliError::CrudStructType => {
                write!(f, "Config.toml CRUD model types must be of String type")
            },
            ActixCliError::CrudRouteShouldBeString => {
                write!(f, "Config.toml CRUD rute value should be a string")
            },
            ActixCliError::UnknwonCrudItem => {
                write!(f, "Config.toml CRUD routes contains unknown key")
            },
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

impl From<toml::de::Error> for ActixCliError {
    fn from(_: toml::de::Error) -> Self {
        ActixCliError::FailedToParseConfigToml
    }
}