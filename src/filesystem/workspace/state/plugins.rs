use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct PluginManifest {
    pub version: String,
    pub author: String,
    pub repository: String,
    pub source_name: String,
}
