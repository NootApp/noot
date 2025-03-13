use regex::Regex;
use serde_derive::{Deserialize, Serialize};
use crate::filesystem::utils::traits::{Configuration, ValidationError};

/// RichPresenceConfig provides a configuration interface for the
/// rich presence provided within the application.
///
/// Workspaces may override the ability to enable rich presence
/// on an individual level or (in the future) perhaps through
/// group policy
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct RichPresenceConfig {
    pub enable: Option<bool>,
    pub client_id: Option<String>,
    pub enable_idle: Option<bool>,
    pub show_current_workspace: Option<bool>,
    pub show_current_file: Option<bool>,
}

impl RichPresenceConfig {
    pub fn is_enabled(&self) -> bool {
        self.enable.unwrap_or(false)
    }

    pub fn client_id(&self) -> String {
        self.client_id.clone().unwrap_or("1343225099834101810".to_string())
    }

    pub fn is_enable_idle(&self) -> bool {
        self.enable_idle.unwrap_or(false)
    }

    pub fn can_show_current_workspace(&self) -> bool {
        self.show_current_workspace.unwrap_or(false)
    }

    pub fn can_show_current_file(&self) -> bool {
        self.show_current_file.unwrap_or(false)
    }
}

impl Default for RichPresenceConfig {
    fn default() -> Self {
        Self {
            enable: None,
            client_id: None,
            enable_idle: None,
            show_current_workspace: None,
            show_current_file: None,
        }
    }
}



impl Configuration for RichPresenceConfig {

    /// Validation will always pass for rich presence config
    fn validate(&self, prefix: &str) -> Vec<ValidationError> {
        let mut errors = Vec::new();

        let cid = self.client_id();

        if !Regex::new("[0-9]{18}").unwrap().is_match(&cid) {
            errors.push(
                ValidationError::new(
                    &format!("{}::client_id", prefix),
                    &format!("Invalid CID: '{}' - Client IDs must be 18 digit numbers provided in string notation", cid),
                    true
                )
            )
        }

        errors
    }

    /// Repair is an empty method on this struct, as there is nothing to repair
    fn repair(&mut self) {}
}