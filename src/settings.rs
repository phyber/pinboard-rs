extern crate config;
extern crate shellexpand;

use config::{Config, ConfigError, File, FileFormat};

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
    pub fn new(config_file: &str) -> Result<Self, ConfigError> {
        // Get a new config and set defaults.
        let mut c = Config::new();
        c.set_default("debug", false)?;

        // Merge config from external sources
        let cf = shellexpand::tilde(config_file);
        c.merge(File::new(&cf, FileFormat::Yaml))?;

        // Derive
        c.try_into()
    }
}
