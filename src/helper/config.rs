use serde::{Deserialize, Serialize};
use std::fs::{self, read_to_string, File};
use std::path::{PathBuf};
use std::str::FromStr;
use toml_edit::{de, value, DocumentMut};

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

fn get_config_dir() -> PathBuf {
    let mut path = dirs::home_dir().unwrap();
    path.push(".znotify");
    path
}

fn get_config_path() -> PathBuf {
    let mut path = get_config_dir();
    path.push("config.toml");
    path
}

fn read_config_string() -> String {
    if !get_config_dir().exists() || !get_config_dir().is_dir() || !get_config_path().exists() {
        return String::new();
    }

    read_to_string(get_config_path().clone()).unwrap_or_else(|err| {
        panic!(
            "Failed to read config file: {}\n {}",
            get_config_path().to_str().unwrap(),
            err
        );
    })
}

fn write_config_string(s:String) {
    if !get_config_dir().exists() {
        fs::create_dir(get_config_dir().clone()).unwrap_or_else(|err| {
            panic!(
                "Failed to create config dir: {}\n{}",
                get_config_dir().to_str().unwrap(),
                err
            );
        });
    }

    if !get_config_dir().is_dir() {
        panic!(
            "Failed to create config dir: {}{}",
            get_config_dir().to_str().unwrap(),
            " not a directory"
        );
    }

    if !get_config_path().exists() {
        File::create(get_config_path().clone()).unwrap_or_else(|err| {
            panic!(
                "Failed to create config file: {}\n{}",
                get_config_path().to_str().unwrap(),
                err
            );
        });
    }

    fs::write(get_config_path().clone(), s).unwrap_or_else(|err| {
        panic!(
            "Failed to write config file: {}\n{}",
            get_config_path().to_str().unwrap(),
            err
        );
    });
}

pub(crate) fn update_config(cfg: Config) {
    // update exist file
    let exist_config_string = read_config_string();
    let mut doc = DocumentMut::from_str(&exist_config_string).unwrap_or_else(|_| {
        panic!(
            "Failed to parse config file: {}",
            get_config_path().to_str().unwrap()
        );
    });
    doc["user_secret"] = value(cfg.user_secret.unwrap());
    doc["endpoint"] = value(cfg.endpoint.unwrap());
    let new_config = doc.to_string();
    write_config_string(new_config);
}

pub(crate) fn get_config() -> Config {
    let config_str = read_config_string();
    if config_str.is_empty() {
        return Config::default();
    }

    let config: Config = de::from_str(&config_str).unwrap_or_else(|err| {
        eprintln!("Failed to parse config file");
        panic!("{}", err)
    });
    config
}
