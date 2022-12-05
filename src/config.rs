use std::fs::File;
use std::str::FromStr;
use serde::{Deserialize, Serialize};
use toml_edit::{Document, easy, value};

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Config {
    pub(crate) user_id: Option<String>,
    pub(crate) endpoint: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            user_id: None,
            endpoint: Some(String::from("https://push.learningman.top")),
        }
    }
}

pub(crate) fn update_config(user_id: String, endpoint: Option<String>) {
    let config_path = dirs::home_dir().unwrap().join(".znotify").join("config.toml");
    let config_dir = config_path.parent().unwrap_or_else(|| {
        panic!("Cannot get config directory");
    });
    if !config_dir.exists() {
        std::fs::create_dir_all(config_dir).unwrap_or_else(|_| {
            panic!("Failed to create config directory: {}", config_dir.to_str().unwrap());
        });
    }
    if !config_path.exists() {
        File::create(config_path.clone()).unwrap_or_else(|_| {
            panic!("Failed to create config file: {}", config_path.to_str().unwrap());
        });
    }
    let mut config = Config { user_id: Some(user_id), ..Default::default() };
    if endpoint.is_some() {
        config.endpoint = endpoint;
    }

    // update exist file
    let exist_config = std::fs::read_to_string(config_path.clone()).unwrap_or_default();
    let mut doc = Document::from_str(&exist_config).unwrap_or_else(|_| {
        panic!("Failed to parse config file: {}", config_path.to_str().unwrap());
    });
    doc["user_id"] = value(config.user_id.unwrap());
    doc["endpoint"] = value(config.endpoint.unwrap());
    let new_config = doc.to_string();
    std::fs::write(config_path.clone(), new_config).unwrap_or_else(|_| {
        panic!("Failed to write config file: {}", config_path.to_str().unwrap());
    });
}

pub(crate) fn get_config() -> Config {
    let config_path = dirs::home_dir().unwrap().join(".znotify").join("config.toml");
    if !config_path.exists() {
        return Config::default();
    }
    let config_str = std::fs::read_to_string(config_path).unwrap_or_else(|_| {
        eprintln!("Failed to read config file");
        String::new()
    });
    if config_str.is_empty() {
        return Config::default();
    }

    let config: Config = easy::from_str(&config_str).unwrap_or_else(|_| {
        eprintln!("Failed to parse config file");
        Config::default()
    });
    config
}