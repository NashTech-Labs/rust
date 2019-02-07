use std::env;
use config::{ConfigError, Config, File, Environment};

#[derive(Debug, Deserialize)]
pub struct Settings {
    debug: bool,
    database_port_address: &'static str,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let mut settings: Config = Config::default();
        settings.merge(vec![config::File::with_name("Settings"),
                            config::File::from(Path::new("conf/configuration.toml"))]).unwrap()
            .merge(config::Environment::with_prefix("APP")).unwrap()

    }
}