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

/// Stores command line arguments according to their types.
#[derive(Debug)]
pub struct Args {
    reg: Vec<ArgType>,
    flags: Vec<ArgType>,
    kwargs: Vec<ArgType>
}

impl Args {
    #[allow(clippy::manual_strip)] // TODO: REMOVE ME!
    pub fn new() -> Re<Self> {
        let (mut reg, mut flags, mut kwargs) = (
            Vec::new(),
            Vec::new(),
            Vec::new()
        );

        for arg in args().skip(1) {
            if arg.starts_with("--") {
                let parts = Vec::from_iter(arg.split('=').map(String::from));

                if parts.len() != 2 {
                    return err!("Invalid keyword argument: {arg}");
                }

                kwargs.push(ArgType::Kwarg(parts[0][2..].to_owned(), parts[1].to_owned()));
            } else if arg.starts_with('-') {
                flags.push(ArgType::Flag(arg[1..].to_owned()));
            } else {
                reg.push(ArgType::Reg(arg));
            };
        }

        Ok(Args { reg, flags, kwargs })
    }
}

#[cfg(debug_assertions)] // TODO: REMOVE ME!
impl Display for Args {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        <Self as Debug>::fmt(self, f)
    }
}
