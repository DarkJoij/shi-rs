use crate::err;
use crate::fs::config::{Config, save_config};
use crate::io::is_git_enabled;
use crate::io::logn_macros::set_up_logger;

use serde_json::Error as SerdeJsonError;

use core::str::ParseBoolError;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::io::Error as IOError;
use std::str::FromStr;

/// Not RegExp! Shortened from [`Result`]
pub type Re<T> = Result<T, Error>;

/// Configures the capabilities needed when the program is running.
pub fn set_up(config: &mut Config) -> Re<()> {
    set_up_logger(config);
    let mark = is_git_enabled();

    if config.git_enabled != mark {
        config.git_enabled = mark;
        save_config(config)?;
    }

    // This code causes the config to be overwritten every
    // time it is run, which may negatively affect performance
    // in the future:

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
    Serde(SerdeJsonError),
    BooleanParsing(ParseBoolError)
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

impl From<ParseBoolError> for Error {
    fn from(value: ParseBoolError) -> Self {
        Self::BooleanParsing(value)
    }
}

// We need to check, maybe there is an easier way
impl Debug for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", match self {
            Error::System(message) => message.to_owned(),
            Error::IO(io_err) => io_err.to_string(),
            Error::Serde(serde_err) => serde_err.to_string(),
            Error::BooleanParsing(parse_bool_err) => parse_bool_err.to_string()
        })
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        <Self as Debug>::fmt(self, f)
    }
}

/// Parses the string into the [`bool`] type based on the content.
pub fn parse_into_bool(string: &Option<String>) -> Re<bool> {
    let string = match string {
        Some(string_bool) => string_bool,
        None => return err!("'rvalue' was not provided for keyword argument.")
    };
    let parsed = bool::from_str(string)?;

    Ok(parsed)
}
