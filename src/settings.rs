extern crate config;

use std::env;
use std::path::PathBuf;
use config::{Config, ConfigError, File, FileFormat};

// Path to the config file in the .config directory.
// This should be replaced with some XDG stuff in the future.
static DOT_CONFIG_FILE: &'static str = ".config/pinboard/config.yaml";

#[derive(Debug, Deserialize)]
pub struct API {
    pub token: String,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    debug: bool,
    pub api: API,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        // Get a new config and set defaults.
        let mut c = Config::new();
        c.set_default("debug", false)?;

        // Merge config from external sources
        let cf = config_file_path();
        c.merge(File::new(&cf, FileFormat::Yaml))?;

        // Derive
        c.try_into()
    }
}

// Attempt to get a home directory. If it fails, an empty string is returned.
// This will become the equivalent of looking for a config in the current
// directory.
fn config_file_path() -> String {
    let home_dir = match env::home_dir() {
        Some(path) => path,
        None => PathBuf::from(""),
    };

    String::from(home_dir.join(DOT_CONFIG_FILE).to_str().unwrap())
}
