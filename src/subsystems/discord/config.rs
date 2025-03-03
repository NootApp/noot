use serde_derive::{Deserialize, Serialize};

/// RichPresenceConfig provides a configuration interface for the
/// rich presence provided within the application.
///
/// Workspaces may override the ability to enable rich presence
/// on an individual level or (in the future) perhaps through
/// group policy
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct RichPresenceConfig {
    pub enable: bool,
    pub client_id: Option<String>,
    pub enable_idle: bool,
    pub show_current_workspace: bool,
    pub show_current_file: bool,
}
