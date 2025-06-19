use std::fs;
use toml::de::Error;

#[derive(serde::Deserialize)]
pub struct Server {
    pub port: u16,
    pub static_dir: String,
}

#[derive(serde::Deserialize)]
pub struct Config {
    pub server: Server,
}

impl Config {
    pub fn new() -> Result<Self, Error> {
        let config_path = "config.toml";

        let toml_string = fs::read_to_string(config_path)
            .expect("Could not read config file");
        
        let config: Config = toml::from_str(&toml_string)
            .expect("Could not parse config file");
        
        Ok(config)
    }
}
