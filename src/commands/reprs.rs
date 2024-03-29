use crate::err;
use crate::utils::Re;

/// It is NOT currently in use, but
/// it may be used in the future.
enum _CommandCast {
    Regular,
    Irregular
}

/// The enum branches of which are the current "system" commands.
#[derive(Debug)]
pub enum Command {
    Verbose,
    Settings
}

impl Command {
    pub fn from(value: String) -> Re<Self> {
        // There could be a lowercase check here...
        let parsed = match value.as_str() {
            "-v" | "verbose" => Command::Verbose,
            "-s" | "settings" => Command::Settings,
            _ => return err!("Invalid command: \"{value}\".")
        };

        Ok(parsed)
    }

    pub fn description(&self) -> String {
        let description = match self {
            Command::Verbose => "Shows core information.",
            Command::Settings => "Enable/disable selected core settings."
        };

        description.to_owned()
    }
}
