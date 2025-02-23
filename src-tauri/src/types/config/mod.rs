use serde_derive::{Deserialize, Serialize};
use std::io::Write;

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct Config {
    pub workspace: String,
}

const DEFAULT_CONFIG_STRING: &'static str = "{\"workspace\": \"NONE\"}";

impl Config {
    pub fn load_from_disk() -> Config {
        let cfg_dir = dirs::config_local_dir();
        let mut cfg_path = cfg_dir.unwrap();
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

        let cfg_dir = dirs::config_local_dir();
        let mut cfg_path = cfg_dir.unwrap();
        cfg_path.push("cfg.json");
        let mut handle = std::fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(cfg_path)?;

        handle.write_all(contents.as_bytes())?;
        handle.flush()?;

        Ok(())
    }
}
