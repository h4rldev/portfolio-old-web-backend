use serde::{Deserialize, Serialize};
use std::{
    env::current_dir,
    fs::{metadata, read_to_string, File},
    io::Write,
};

#[derive(Deserialize, Serialize)]
pub struct Config {
    ip: String,
    port: String,
    custom_header_key: Option<String>,
    custom_header_value: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            ip: "0.0.0.0".to_string(),
            port: "8080".to_string(),
            custom_header_key: None,
            custom_header_value: None,
        }
    }
}

impl Config {
    pub fn get() -> Self {
        let current_dir = current_dir().expect("Can't get current directory");
        let config_dir = current_dir.join("backend-config.toml");

        if !config_dir.is_file() {
            return Self::new();
        }

        if metadata(&config_dir).expect("Can't get metadata").len() == 0 {
            return Self::new();
        }

        let content = read_to_string(config_dir).expect("Can't read file to string.");
        toml::from_str(&content).expect("Can't parse toml.")
    }

    fn new() -> Self {
        let current_dir = current_dir().expect("Can't get current directory");
        let config_dir = current_dir.join("backend-config.toml");
        let toml_content = toml::to_string(&Self::default()).expect("Can't serialize to string.");

        let mut file = File::create(config_dir).expect("Can't create file?");
        file.write_all(toml_content.as_bytes())
            .expect("Can't write to toml");
        Self::default()
    }

    pub fn ip(&self) -> String {
        self.ip.clone()
    }

    pub fn port(&self) -> String {
        self.port.clone()
    }

    pub fn custom_header_key(&self) -> Option<String> {
        self.custom_header_key.clone()
    }

    pub fn custom_header_value(&self) -> Option<String> {
        self.custom_header_value.clone()
    }
}
