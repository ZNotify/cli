use std::fs::{self, File, read_to_string};
use std::str::FromStr;
use serde::{Deserialize, Serialize};
use toml_edit::{Document, value, de};

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Config {
    pub(crate) user_secret: Option<String>,
    pub(crate) endpoint: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            user_secret: None,
            endpoint: Some(String::from("https://push.learningman.top")),
        }
    }
}

pub(crate) fn update_config(cfg: Config) {
    let config_path = dirs::home_dir().unwrap().join(".znotify").join("config.toml");
    let config_dir = config_path.parent().unwrap_or_else(|| {
        panic!("Cannot get config directory");
    });
    if !config_dir.exists() {
        fs::create_dir_all(config_dir).unwrap_or_else(|_| {
            panic!("Failed to create config directory: {}", config_dir.to_str().unwrap());
        });
    }
    if !config_path.exists() {
        File::create(config_path.clone()).unwrap_or_else(|_| {
            panic!("Failed to create config file: {}", config_path.to_str().unwrap());
        });
    }

    // update exist file
    let exist_config = read_to_string(config_path.clone()).unwrap_or_default();
    let mut doc = Document::from_str(&exist_config).unwrap_or_else(|_| {
        panic!("Failed to parse config file: {}", config_path.to_str().unwrap());
    });
    doc["user_secret"] = value(cfg.user_secret.unwrap());
    doc["endpoint"] = value(cfg.endpoint.unwrap());
    let new_config = doc.to_string();
    fs::write(config_path.clone(), new_config).unwrap_or_else(|_| {
        panic!("Failed to write config file: {}", config_path.to_str().unwrap());
    });
}

pub(crate) fn get_config() -> Config {
    let config_path = dirs::home_dir().unwrap().join(".znotify").join("config.toml");
    if !config_path.exists() {
        return Config::default();
    }
    let config_str = read_to_string(config_path).unwrap_or_else(|err| {
        eprintln!("Failed to read config file");
        panic!("{}", err)
    });
    if config_str.is_empty() {
        return Config::default();
    }

    let config: Config = de::from_str(&config_str).unwrap_or_else(|err| {
        eprintln!("Failed to parse config file");
        panic!("{}", err)
    });
    config
}
