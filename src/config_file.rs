use crate::console_ui::input_string;
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::{fs, path::PathBuf};
use toml::de::Error;

#[cfg(target_os = "linux")]
static CONFIG_PATH: &str = ".local/pwd-manager.toml";

#[cfg(target_os = "windows")]
static CONFIG_PATH: &str = "AppData\\Local\\pwd-manager.toml";

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub pwd_file_path: String,
}

impl Config {
    pub async fn get_storage_path() -> Self {
        let conf_path = Config::get_conf_path();
        let mut conf = Config {
            pwd_file_path: "".to_string(),
        };
        match fs::read_to_string(conf_path) {
            Ok(conf_str) => {
                let config_res: Result<Config, Error> = toml::from_str(&conf_str);
                if config_res.is_err() {
                    println!("Error parsing config, regenerating it...");
                    conf.regen_config().await;
                    conf
                } else {
                    let storage_path = config_res.unwrap().pwd_file_path;
                    let parent = Path::new(&storage_path).parent().unwrap_or(Path::new("*"));
                    if parent.to_string_lossy() == "" || parent.exists() {
                        Config {
                            pwd_file_path: storage_path,
                        }
                    } else {
                        conf.regen_config().await;
                        conf
                    }
                }
            }
            Err(e) => {
                if e.kind() == std::io::ErrorKind::NotFound {
                    conf.regen_config().await;
                    conf
                } else {
                    panic!("Error while trying to access config file: {:?}", e);
                }
            }
        }
    }
    async fn set_storage_path(&self, path: &str) {
        let config = toml::to_string(&Config {
            pwd_file_path: path.to_string(),
        })
        .unwrap();
        let conf_path = Config::get_conf_path();
        std::fs::write(conf_path, config).expect("Failed to save config");
    }

    async fn regen_config(&mut self) {
        let path = loop {
            let user_input = input_string("Input path to password storage file: ").await;
            let path = Path::new(user_input.trim());
            let Some(dir_path) = path.parent() else {
                continue;
            };

            if dir_path.to_string_lossy() != "" && !dir_path.exists() {
                continue;
            }
            break path.to_string_lossy().to_string();
        };
        self.set_storage_path(&path).await;
        self.pwd_file_path = path.clone();
    }

    fn get_conf_path() -> PathBuf {
        match home::home_dir() {
            Some(path) if !path.as_os_str().is_empty() => path.join(CONFIG_PATH),
            _ => {
                println!("Unable to get your home dir!");
                Path::new(CONFIG_PATH).to_path_buf()
            }
        }
    }
}
