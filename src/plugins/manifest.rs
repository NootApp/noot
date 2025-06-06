use std::collections::BTreeSet;
use semver::Version;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginManifest {
    pub name: String,
    pub version: Version,
    pub description: String,
    pub authors: Vec<String>,
    pub scopes: BTreeSet<String>
}
