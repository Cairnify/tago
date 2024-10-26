use chrono::{DateTime, Utc};
use std::collections::HashMap;
use std::fs;

pub fn get_config_path() -> std::path::PathBuf {
    let mut config_path = home::home_dir().expect("Could not retrieve home directory");
    config_path.push(".config/tago/config.toml");
    config_path
}

pub fn load_config() -> HashMap<String, DateTime<Utc>> {
    let config_path = get_config_path();
    if config_path.exists() {
        let content = fs::read_to_string(config_path).expect("Could not read config file");
        toml::from_str(&content).expect("Could not parse config")
    } else {
        HashMap::new()
    }
}

pub fn write_config(config: &HashMap<String, DateTime<Utc>>) {
    let config_path = get_config_path();

    if let Some(parent) = config_path.parent() {
        fs::create_dir_all(parent).expect("Could not create config directory");
    }

    let toml_data = toml::to_string(config).expect("Could not serialize data to TOML format");

    fs::write(&config_path, toml_data).expect("Could not write to config file");

}
