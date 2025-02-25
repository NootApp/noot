use serde_derive::{Serialize, Deserialize};
//use chrono::{DateTime, Local};
use std::collections::HashMap;


#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all="kebab-case")]
pub struct WorkspaceManifest {
    pub asset_dir: String,
    pub plugin_dir: String,
    pub plugin_kv: HashMap<String, bool>,
    //pub db
}
