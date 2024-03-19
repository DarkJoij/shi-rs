use crate::utils::Re;

use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string_pretty};

use std::fs::{File, read_to_string, remove_file, write};
use std::io::Write;

const CONFIG_FILE_NAME: &str = "./config.json";

/// A structure representing a config without
/// a high-level wrapper (no need here).
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Config {
    pub git_enabled: bool,
    pub git_allowed: bool,
    pub logging_allowed: bool
}

/// Creates a file and returns [`File`] if it does not
/// exist (if there is a file, it will not be killed ;) )
/// TODO
/// ----
/// * It is also possible to replace this function with a
/// universal tool for generating default files of similar
/// content.
fn create_config() -> Re<String> {
    let mut file = File::create(CONFIG_FILE_NAME)?;
    let default_content = to_string_pretty(&Config::default())?;

    file.write_all(default_content.as_bytes())?;
    Ok(default_content.to_owned())
}

/// Recreates the config (deleting the old one) if filling
/// in the old one caused errors or other problems.
fn regenerate_config() -> Re<Config> {
    remove_file(CONFIG_FILE_NAME)?;

    let content = create_config()?;
    let config: Config = from_str(&content)?;

    drop(content); // TODO: MUST BE CHECKED FOR SAFETY!
    Ok(config)
}

/// Reads the file data and returns the deserialized [`Config`].
/// If there is no file, calls [`create_config`].
pub fn read_config() -> Re<Config> {
    let content = match read_to_string(CONFIG_FILE_NAME) {
        Ok(file) => file,
        Err(..) => create_config()?
    };
    let config: Config = match from_str(&content) {
        Ok(config) => config,
        Err(..) => regenerate_config()?
    };

    drop(content); // TODO: MUST BE CHECKED FOR SAFETY!
    Ok(config)
}

/// Writes the serialized [`Config`] to a file.
pub fn save_config(config: &Config) -> Re<()> {
    let serialized = to_string_pretty(config)?;
    write(CONFIG_FILE_NAME, serialized)?;

    Ok(())
}
