use serde_derive::{Deserialize, Serialize};
use tokio::io::AsyncWriteExt;
use std::path::PathBuf;
use log::{info, debug, trace, warn, error};
use crate::filesystem::workspace::global::WorkspaceManifest;
use crate::subsystems::discord::config::RichPresenceConfig;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Config {
    #[serde(rename = "workspace")]
    pub workspaces: Vec<WorkspaceManifest>,
    pub last_open: Option<String>,
    pub rpc: RichPresenceConfig,
}



/// The default config is imported at compile time
/// from the sample_config.toml in the root directory
/// of the repository
const DEFAULT_CONFIG_STRING: &'static str = include_str!("../../../sample_config.toml");

impl Config {
    pub async fn load_from_disk() -> Config {
        debug!("Loading config from disk");
        let mut cfg_path = get_config_path();
        let cfg_folder = cfg_path.clone();
        debug!("Config path: {:?}", cfg_path);
        cfg_path.push("cfg.toml");

        if  !tokio::fs::try_exists(&cfg_path).await.unwrap() {
            warn!("Config file does not exist - making folder");
            tokio::fs::create_dir_all(cfg_folder.clone()).await.unwrap();
            tokio::fs::write(&cfg_path, &DEFAULT_CONFIG_STRING).await.unwrap();
            toml::from_str(DEFAULT_CONFIG_STRING).unwrap()
        } else {
            debug!("Parsing config file");
            let contents = tokio::fs::read_to_string(cfg_path).await.unwrap();
            toml::from_str(&contents).unwrap()
        }

    }

    pub async fn save_to_disk(&self) -> Result<(), std::io::Error> {
        debug!("Saving config to disk");
        let contents = toml::to_string(&self).unwrap();
        let mut cfg_path = get_config_path();

        if !tokio::fs::try_exists(&cfg_path).await? {
            debug!("Config file does not exist");
            tokio::fs::create_dir_all(&cfg_path).await?;
        }

        debug!("Config path: {:?}", cfg_path);
        cfg_path.push("cfg.toml");
        let mut handle = tokio::fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(cfg_path).await?;

        handle.write_all(contents.as_bytes()).await?;
        handle.flush().await?;

        debug!("Config file successfully saved");
        Ok(())
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
    use std::thread::current;
    use nanoid::nanoid;
    use super::*;

    #[tokio::test]
    async fn test_config_read_from_disk() {
        let cfg = Config::load_from_disk().await;

        assert_eq!(cfg.workspaces.len(), 1);
    }


    #[tokio::test]
    async fn test_config_save_to_disk() {
        let mut cfg = Config::load_from_disk().await;
        
        let new_workspace_id = nanoid!(10);
        
        cfg.last_open = Some(new_workspace_id.clone());
        cfg.save_to_disk().await.unwrap();
        let cfg2 = Config::load_from_disk().await;
        assert_eq!(cfg2.last_open, Some(new_workspace_id));
    }
}