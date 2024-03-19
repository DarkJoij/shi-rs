use crate::fs::config::{Config, save_config};
use crate::io::args::{ArgCast, Args};
use crate::utils::{parse_into_bool, Re};
use super::reprs::Command;

pub struct Executor {
    args: Args
}

impl Executor {
    pub fn new(args: Args) -> Self {
        Executor { args }
    }

    pub fn execute(&self, config: &mut Config) -> Re<String> {
        let message = match self.args.command {
            // TODO: SAVE CORE INFO IN TOML AND PARSE IT HERE!
            Command::Verbose => "V0.24.320",
            Command::Settings => {
                let mut result = "";

                for arg in &self.args.inner {
                    let mark = parse_into_bool(&arg.cell_2)?;

                    match arg.cast {
                        ArgCast::AllowGit => {
                            if config.git_allowed != mark {
                                config.git_allowed = mark;
                            }
                        },
                        ArgCast::AllowLogging => {
                            if config.logging_allowed != mark {
                                config.logging_allowed = mark;
                            }
                        },
                        _ => result = "Invalid keyword argument provided."
                    };
                }

                if result.is_empty() {
                    save_config(config)?;
                    result = "Settings saved successfully!";
                }

                result
            }
        };

        Ok(message.to_owned())
    }
}
