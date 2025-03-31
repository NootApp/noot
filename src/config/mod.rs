use std::fmt::Debug;
use std::io::Read;
use std::path::PathBuf;
use serde_derive::{Deserialize, Serialize};
use crate::consts::APP_NAME;

pub mod performance;

pub type ConfigResult = Result<(), ConfigurationError>;


#[derive(Debug)]
pub enum ConfigurationError {
    InvalidField(String, Box<dyn Debug>),
    NotExist,
    Passthrough(std::io::Error),
    Decode(toml::de::Error),
}


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
pub struct Config {
    pub workspace_directory: PathBuf,
    pub performance: performance::PerformanceConfiguration
}


impl Default for Config {
    fn default() -> Config {
        let mut document_directory = dirs::document_dir();

        if document_directory.is_none() {
            document_directory = dirs::desktop_dir();
        }

        let workspace_directory = document_directory.unwrap();

        Config {
            workspace_directory,
            performance: Default::default()
        }
    }
}

impl Config {
    pub fn load(&mut self) -> ConfigResult {
        let mut config_path = locate_config_dir()?;
        config_path.set_file_name("config");
        config_path.set_extension("toml");

        let handle_outcome = std::fs::File::options()
            .read(true)
            .write(false)
            .create(false)
            .truncate(false)
            .open(config_path);

        if let Err(error) = handle_outcome {
            return Err(ConfigurationError::Passthrough(error))
        }

        let mut handle = handle_outcome.unwrap();

        let mut raw_toml = String::new();

        handle.read_to_string(&mut raw_toml).unwrap();

        let cfg = toml::from_str::<Config>(&raw_toml);

        if let Err(error) = cfg {
            return Err(ConfigurationError::Decode(error))
        }


        Ok(())
    }

    pub fn save(&self) -> ConfigResult {
        let mut config_path = locate_config_dir()?;
        config_path.set_file_name("config");
        config_path.set_extension("toml");

        let handle_outcome = std::fs::File::options()
            .read(false)
            .write(true)
            .create(true)
            .truncate(true)
            .open(config_path);

        if let Err(error) = handle_outcome {
            return Err(ConfigurationError::Passthrough(error))
        }


        Ok(())
    }
}


pub fn locate_config_dir() -> Result<PathBuf, ConfigurationError> {
    let dir = dirs::config_local_dir();

    if dir.is_none() {
        return Err(ConfigurationError::NotExist)
    }

    let mut path = dir.unwrap();

    path.push(APP_NAME);

    let might_exist = std::fs::exists(&path);

    if let Ok(config_dir_exists) = might_exist {
        if config_dir_exists {
            debug!("Config path exists");
        } else {
            debug!("Config path does not exist");
            let did_make_dir = std::fs::create_dir_all(&path);

            if did_make_dir.is_err() {
                return Err(ConfigurationError::Passthrough(did_make_dir.unwrap_err()));
            }
        }
    } else {
        return Err(ConfigurationError::Passthrough(might_exist.unwrap_err()))
    }


    Ok(path)
}
