use std::collections::BTreeSet;
use semver::Version;

pub struct PluginManifest {
    pub name: String,
    pub version: Version,
    pub description: String,
    pub scopes: BTreeSet<String>
}
