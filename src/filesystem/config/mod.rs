use serde_derive::{Deserialize, Serialize};
use tokio::io::AsyncWriteExt;
use std::path::PathBuf;
use log::{info, debug, trace, warn, error};

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub workspace: String,
    pub rpc: bool,
}

const DEFAULT_CONFIG_STRING: &'static str = "{\"workspace\": \"NONE\", \"rpc\": false}";

impl Config {
    pub async fn load_from_disk() -> Config {
        debug!("Loading config from disk");
        let mut cfg_path = get_config_path();
        let cfg_folder = cfg_path.clone();
        debug!("Config path: {:?}", cfg_path);
        cfg_path.push("cfg.json");

        if  !tokio::fs::try_exists(&cfg_path).await.unwrap() {
            warn!("Config file does not exist - making folder");
            tokio::fs::create_dir_all(cfg_folder.clone()).await.unwrap();
            tokio::fs::write(&cfg_path, &DEFAULT_CONFIG_STRING).await.unwrap();
            json5::from_str(DEFAULT_CONFIG_STRING).unwrap()
        } else {
            debug!("Parsing config file");
            let contents = tokio::fs::read_to_string(cfg_path).await.unwrap();
            json5::from_str(&contents).unwrap()
        }

    }

    pub async fn save_to_disk(&self) -> Result<(), std::io::Error> {
        debug!("Saving config to disk");
        let contents = json5::to_string(&self).unwrap();
        let mut cfg_path = get_config_path();

        if !tokio::fs::try_exists(&cfg_path).await? {
            debug!("Config file does not exist");
            tokio::fs::create_dir_all(&cfg_path).await?;
        }
        
        debug!("Config path: {:?}", cfg_path);
        cfg_path.push("cfg.json");
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
    use super::*;

    #[tokio::test]
    async fn test_config_read_from_disk() {
        let cfg = Config::load_from_disk().await;

        assert_eq!(cfg.workspace, "NONE");
    }


    #[tokio::test]
    async fn test_config_save_to_disk() {
        let mut cfg = Config::load_from_disk().await;
        cfg.workspace = "testing".to_string();
        cfg.save_to_disk().await.unwrap();
        let cfg2 = Config::load_from_disk().await;
        assert_eq!(cfg2.workspace, cfg.workspace);
        cfg.workspace = "NONE".to_string();
        cfg.save_to_disk().await.unwrap();
        assert_ne!(cfg.workspace, cfg2.workspace);
    }
}