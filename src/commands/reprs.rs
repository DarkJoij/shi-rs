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
    Test,
    Greet,
}

impl Command {
    pub fn from(value: String) -> Re<Self> {
        // There could be a lowercase check here...
        let parsed = match value.as_str() {
            "test" => Command::Test,
            "greet" => Command::Greet,
            _ => return err!("Invalid command: \"{value}\"")
        };

        Ok(parsed)
    }

    pub fn description(&self) -> String {
        let description = match self {
            Command::Test => "Greet you.",
            Command::Greet => "Test feature."
        };

        description.to_owned()
    }

    pub fn aliases(&self) -> Option<Vec<String>> {
        let aliases = match self {
            Command::Test => Vec::new(),
            Command::Greet => vec!["hello".to_owned()]
        };

        if aliases.is_empty() { None } else { Some(aliases) }
    }
}
