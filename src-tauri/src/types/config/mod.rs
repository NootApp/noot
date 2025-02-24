use serde_derive::{Deserialize, Serialize};
use std::io::Write;
use std::path::PathBuf;

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct Config {
    pub workspace: String,
}

const DEFAULT_CONFIG_STRING: &'static str = "{\"workspace\": \"NONE\"}";

impl Config {
    pub fn load_from_disk() -> Config {    
        let mut cfg_path = get_config_path();


        cfg_path.push("cfg.json");

        if !std::fs::exists(&cfg_path).unwrap() {
            return json5::from_str(DEFAULT_CONFIG_STRING).unwrap();
        } else {
            let contents = std::fs::read_to_string(cfg_path).unwrap();
            json5::from_str(&contents).unwrap()
        }

    }

    pub fn save_to_disk(&self) -> Result<(), std::io::Error> {
        let contents = json5::to_string(&self).unwrap();
        let mut cfg_path = get_config_path();

        if(!std::fs::exists(&cfg_path)?) {
            std::fs::create_dir_all(&cfg_path)?;
        }

        cfg_path.push("cfg.json");
        let mut handle = std::fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(cfg_path)?;

        handle.write_all(contents.as_bytes())?;
        handle.flush()?;

        Ok(())
    }
}

#[cfg(debug_assertions)]
pub fn get_config_path() -> PathBuf {
    let cfg_dir = dirs::config_local_dir();
    let mut cfg_path = cfg_dir.unwrap();
    cfg_path.push("noot-dev");
    dbg!(&cfg_path);
    return cfg_path;
}

#[cfg(not(debug_assertions))]
pub fn get_config_path() -> PathBuf {
    let cfg_dir = dirs::config_local_dir();
    let mut cfg_path = cfg_dir.unwrap();
    cfg_path.push("noot");
    dbg!(&cfg_path);
    return cfg_path;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_read_from_disk() {
        let cfg = Config::load_from_disk();

        assert_eq!(cfg.workspace, "NONE");
    }


    #[test]
    fn test_config_save_to_disk() {
        let mut cfg = Config::load_from_disk();
        cfg.workspace = "testing".to_string();
        cfg.save_to_disk().unwrap();
        let cfg2 = Config::load_from_disk();
        assert_eq!(cfg2.workspace, cfg.workspace);
        cfg.workspace = "NONE".to_string();
        cfg.save_to_disk();
        assert_ne!(cfg.workspace, cfg2.workspace);
    }
}
