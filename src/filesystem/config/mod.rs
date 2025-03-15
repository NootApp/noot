use crate::filesystem::utils::traits::{Configuration, ValidationError};
use crate::filesystem::workspace::global::WorkspaceManifest;
use crate::subsystems::discord::config::RichPresenceConfig;
use serde_derive::{Deserialize, Serialize};
use std::path::PathBuf;
use std::fs::{create_dir_all, exists, read_to_string, OpenOptions};
use std::io::Write;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Config {
    #[serde(rename = "workspace")]
    pub workspaces: Option<Vec<WorkspaceManifest>>,
    pub last_open: Option<String>,
    pub rpc: Option<RichPresenceConfig>,
    pub performance: Option<performance::PerformanceConfiguration>,
}

/// The default config is imported at compile time
/// from the sample_config.toml in the root directory
/// of the repository
const DEFAULT_CONFIG_STRING: &'static str =
    include_str!("../../../sample_config.toml");

impl Config {
    pub fn load_from_disk() -> (Config, bool) {
        debug!("Loading config from disk");
        let mut cfg_path = get_config_path();
        let cfg_folder = cfg_path.clone();
        debug!("Config path: {:?}", cfg_path);
        cfg_path.push("cfg.toml");

        if !exists(&cfg_path).unwrap() {
            warn!("Config file does not exist - making folder");
            let tmp: Config = Config::default();
            tmp.validate("");
            tmp.save_to_disk().unwrap();

            (tmp, true)
        } else {
            debug!("Parsing config file");
            let contents = read_to_string(cfg_path).unwrap();
            let tmp: Config = toml::from_str(&contents).unwrap();
            tmp.validate("");
            (tmp, false)
        }
    }

    pub fn save_to_disk(&self) -> Result<(), std::io::Error> {
        debug!("Saving config to disk");
        let contents = toml::to_string(&self).unwrap();
        let mut cfg_path = get_config_path();

        if !exists(&cfg_path)? {
            debug!("Config file does not exist");
            create_dir_all(&cfg_path)?;
        }

        debug!("Config path: {:?}", cfg_path);
        cfg_path.push("cfg.toml");
        let mut handle = OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(cfg_path)?;

        handle.write_all(contents.as_bytes())?;
        handle.flush()?;

        debug!("Config file successfully saved");
        Ok(())
    }
}

impl Default for Config {
    fn default() -> Config {
        toml::from_str(DEFAULT_CONFIG_STRING).unwrap()
    }
}

impl Configuration for Config {
    fn validate(&self, _prefix: &str) -> Vec<ValidationError> {
        let mut errors = Vec::new();

        dbg!(&self);

        if self.rpc.is_none() {
            errors.push(ValidationError::new(
                "config.rpc",
                "Rich Presence configuration not provided, will use default",
                true,
            ));
        } else {
            let mut rpc = self.rpc.as_ref().unwrap().validate("config.rpc");
            errors.append(&mut rpc);
        }

        if self.performance.is_none() {
            errors.push(ValidationError::new(
                "config.performance",
                "Performance configuration not provided, will use default",
                true,
            ));
        } else {
            let mut performance = self
                .performance
                .as_ref()
                .unwrap()
                .clone()
                .validate("config.performance");
            errors.append(&mut performance);
        }

        if self.workspaces.is_none() {
            errors.push(ValidationError::new(
                "config.workspaces",
                "Workspaces list not provided, will use default",
                true,
            ));
        } else {
            let mut used_ids = Vec::new();
            for workspace in self.workspaces.as_ref().unwrap().iter() {
                if used_ids.contains(&workspace.id) {
                    errors.push(ValidationError::new(
                        &format!("config.workspace.{}(2)", workspace.id.clone().unwrap()),
                        "More than one workspace has the same ID. This will cause the app to fail to start.",
                        false
                    ))
                } else {
                    used_ids.push(workspace.id.clone());
                }
                let mut wsp = workspace.validate(&format!(
                    "config.workspace.{}",
                    workspace.id.clone().unwrap()
                ));
                errors.append(&mut wsp);
            }
        }

        errors
    }

    fn repair(&mut self) {
        if self.rpc.is_none() {
            self.rpc = Some(RichPresenceConfig::default());
        }

        if self.performance.is_none() {
            self.performance =
                Some(performance::PerformanceConfiguration::default());
        }

        if self.workspaces.is_none() {
            self.workspaces = Some(Vec::new());
        }
    }
}

#[cfg(debug_assertions)]
pub fn get_config_path() -> PathBuf {
    let cfg_dir = dirs::config_dir();
    let mut cfg_path = cfg_dir.unwrap();
    cfg_path.push("noot-dev");
    return cfg_path;
}

#[cfg(not(debug_assertions))]
pub fn get_config_path() -> PathBuf {
    let cfg_dir = dirs::config_local_dir();
    let mut cfg_path = cfg_dir.unwrap();
    cfg_path.push("noot");
    return cfg_path;
}

#[cfg(test)]
mod tests {
    use super::*;
    use nanoid::nanoid;

    fn test_config_read_from_disk() {
        let (cfg, _) = Config::load_from_disk();

        assert_eq!(cfg.workspaces.unwrap().len(), 1);
    }

    fn test_config_save_to_disk() {
        let (mut cfg, _) = Config::load_from_disk();

        let new_workspace_id = nanoid!(10);

        cfg.last_open = Some(new_workspace_id.clone());
        cfg.save_to_disk().unwrap();
        let (cfg2, _) = Config::load_from_disk();
        assert_eq!(cfg2.last_open, Some(new_workspace_id));
    }
}

mod performance;
