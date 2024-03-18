use crate::utils::Re;

use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string_pretty};

use std::fs::{File, read_to_string, write};
use std::io::Write;

const CONFIG_FILE_NAME: &str = "./config.json";

/// A structure representing a config without
/// a high-level wrapper (no need here).
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Config {
    pub logging: bool,
    pub rust_shell: bool
}

/// Creates a file and returns [`File`] if it does not
/// exist (if there is a file, it will not be killed ;) )
fn create_config() -> Re<String> {
    let mut file = File::create(CONFIG_FILE_NAME)?;
    let default_content = to_string_pretty(&Config::default())?;

    file.write_all(default_content.as_bytes())?;
    Ok(default_content.to_owned())
}

/// Reads the file data and returns the deserialized [`Config`].
/// If there is no file, calls [`create_config`].
pub fn read_config() -> Re<Config> {
    let content = match read_to_string(CONFIG_FILE_NAME) {
        Ok(file) => file,
        Err(..) => create_config()? // Recursive asking...
    };

    let config: Config = from_str(&content)?;
    drop(content); // TODO: MUST BE CHECKED FOR SAFETY!

    Ok(config)
}

/// Writes the serialized [`Config`] to a file.
fn write_file(config: &Config) -> Re<()> {
    let serialized = to_string_pretty(config)?;
    write(CONFIG_FILE_NAME, serialized)?;

    Ok(())
}
