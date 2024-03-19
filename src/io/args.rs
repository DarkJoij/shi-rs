use crate::commands::reprs::Command;
use crate::err;
use crate::utils::Re;

use std::env::args;
use std::fmt::{Debug, Display, Formatter, Result};

/// Represents command line argument
/// in parsed view with separations.
#[derive(Debug)]
pub enum ArgCast {
    Reg,
    Force,
    Flag,
    AllowGit,
    AllowLogging,
    Kwarg
}

/// Represents an argument.
pub struct Arg {
    pub cast: ArgCast,  //    cell_1 should be [`Option<String>`]
    pub cell_1: String, // <- because of [`Force`] and [`Verbose`]
    pub cell_2: Option<String>
}

impl Arg {
    #[allow(clippy::manual_strip)] // TODO: REMOVE ME!
    fn from(value: &str) -> Re<Self> {
        // TODO: Might be rewritten because it can be easier 
        // to automize process if definition casts...
        let (cast, cell_1, cell_2) = if value.starts_with("--") {
            let parts = Vec::from_iter(value.split('=').map(String::from));

            if parts.len() > 2 {
                return err!("Invalid keyword argument: {value}");
            }

            let cell_1 = parts[0][2..].to_owned();
            let cell_2 = if parts.len() == 2 { Some(parts[1].to_owned()) } else { None };
            let cast = match cell_1.as_str() {
                "allow-git" => ArgCast::AllowGit,
                "allow-logging" => ArgCast::AllowLogging,
                _ => ArgCast::Kwarg
            };

            (cast, cell_1, cell_2)
        } else if value.starts_with('-') {
            let cast = match value {
                "-f" | "-force" => ArgCast::Force,
                _ => ArgCast::Flag
            };

            (cast, value[1..].to_owned(), None)
        } else {
            (ArgCast::Reg, value.to_owned(), None)
        };

        Ok(Arg{ cast, cell_1, cell_2 })
    }

    pub fn build_string(&self) -> String {
        let rvalue = match &self.cell_2 {
            Some(string) => format!("={string}"),
            None => String::new()
        };

        match self.cast {
            ArgCast::Reg => self.cell_1.clone(),
            ArgCast::Force => "-force".to_owned(),
            ArgCast::Flag => format!("-{}", self.cell_1),
            _ => format!("--{}{}", self.cell_1, rvalue) // TODO: Review all branches.
        }
    }
}

impl Debug for Arg {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "({:?}, {})", self.cast, self.build_string())
    }
}

impl Display for Arg {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.write_str(&self.build_string())
    }
}

/// Stores command line arguments according to their types.
#[derive(Debug)]
pub struct Args {
    pub command: Command,
    pub inner: Vec<Arg>
}

impl Args {
    pub fn new() -> Re<Self> {
        let mut inner = Vec::new();
        let argv: Vec<String> = args().collect();
        
        let string_command = match argv.get(1) {
            Some(string) => string,
            None => "No command provided."
        };
        let command = Command::from(string_command.to_owned())?;

        for arg in &argv[2..] {
            inner.push(Arg::from(arg)?);
        }

        Ok(Args { command, inner })
    }
}
