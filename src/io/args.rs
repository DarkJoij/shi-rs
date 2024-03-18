use crate::commands::reprs::Command;
use crate::err;
use crate::utils::Re;

use std::env::args;
use std::fmt::{Debug, Display, Formatter, Result};

/// Represents command line argument
/// in parsed view with separations.
#[derive(Debug)]
pub enum ArgType {
    Reg(String),
    Flag(String),
    Kwarg(String, String)
}

impl ArgType {
    pub fn unwrap_or(&self, fail_variant: &str) -> String {
        match self {
            ArgType::Reg(arg) => arg.to_owned(),
            ArgType::Flag(arg) => arg.to_owned(),
            _ => fail_variant.to_owned()
        }
    }

    pub fn unwrap_kwarg_or(&self, fail_lvalue: &str, fail_rvalue: &str) -> (String, String) {
        match self {
            ArgType::Kwarg(lvalue, rvalue) => (lvalue.to_owned(), rvalue.to_owned()),
            _ => (fail_lvalue.to_owned(), fail_rvalue.to_owned())
        }
    }
}

impl Display for ArgType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", match self {
            ArgType::Reg(arg) => arg.to_owned(),
            ArgType::Flag(arg) => format!("-{arg}"),
            ArgType::Kwarg(lvalue, rvalue) => format!("--{lvalue}={rvalue}")
        })
    }
}

/// Stores command line arguments according to their types.
#[derive(Debug)]
pub struct Args {
    pub command: Command,
    pub inner: Vec<ArgType>
}

impl Args {
    #[allow(clippy::manual_strip)] // TODO: REMOVE ME!
    pub fn new() -> Re<Self> {
        let mut inner = Vec::new();
        let argv: Vec<String> = args().collect();
        
        let string_command = match argv.get(1) {
            Some(string) => string,
            None => "No command provided."
        };
        let command = Command::from(string_command.to_owned())?;

        // It is necessary to sort out the sorting of arguments!
        for arg in &argv[2..] {
            let argument = if arg.starts_with("--") {
                let parts = Vec::from_iter(arg.split('=').map(String::from));

                if parts.len() != 2 {
                    return err!("Invalid keyword argument: {arg}");
                }

                ArgType::Kwarg(parts[0][2..].to_owned(), parts[1].to_owned())
            } else if arg.starts_with('-') {
                ArgType::Flag(arg[1..].to_owned())
            } else {
                ArgType::Reg(arg.to_owned())
            };

            inner.push(argument);
        }

        Ok(Args { command, inner })
    }
}
