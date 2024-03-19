use crate::io::args::Args;
use crate::utils::Re;
use super::reprs::Command;

pub struct Executor {
    args: Args
}

impl Executor {
    pub fn new(args: Args) -> Self {
        Executor { args }
    }

    pub fn execute(&self) -> Re<String> {
        let message = match self.args.command {
            // TODO: SAVE CORE INFO IN TOML AND PARSE IT HERE!
            Command::Verbose => "V.0.24.320"
        };

        Ok(message.to_owned())
    }
}
