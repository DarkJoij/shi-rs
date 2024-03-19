use crate::fs::config::{Config, save_config};
use crate::io::git_enabled;
use crate::io::logn_macros::set_up_logger;

use serde_json::Error as SerdeJsonError;

use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::io::Error as IOError;

/// Configures the capabilities needed when the program is running.
pub fn set_up(config: &mut Config) -> Re<()> {
    set_up_logger(config);

    if git_enabled() && matches!(config.git_enabled, false) {
        config.git_enabled = true;
        save_config(config)?;
    }
    
    Ok(())
}

/// Made for errors compatibility.
///
/// Note
/// ----
/// Use only the properties from the
/// [`Debug`] trait to format this enum.
pub enum Error {
    System(String),
    IO(IOError),
    Serde(SerdeJsonError)
}

impl From<String> for Error {
    fn from(value: String) -> Self {
        Self::System(value)
    }
}

impl From<IOError> for Error {
    fn from(value: IOError) -> Self {
        Self::IO(value)
    }
}

impl From<SerdeJsonError> for Error {
    fn from(value: SerdeJsonError) -> Self {
        Self::Serde(value)
    }
}

// We need to check, maybe there is an easier way
impl Debug for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", match self {
            Error::System(message) => message.to_owned(),
            Error::IO(io_err) => io_err.to_string(),
            Error::Serde(serde_err) => serde_err.to_string()
        })
    }
}

/// Not RegExp! Shortened from [`Result`]
pub type Re<T> = Result<T, Error>;
