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
            Command::Test => {
                "Yes, this is test feature.".to_owned()
            },
            Command::Greet => {
                let potential_name = &self.args.inner[0];
                format!("Hello, {}!", potential_name.unwrap_or("User!"))
            }
        };

        Ok(message)
    }
}
