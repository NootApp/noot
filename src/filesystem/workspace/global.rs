use crate::subsystems::discord::config::RichPresenceConfig;
use chrono::{DateTime, Local};

use serde_derive::{Deserialize, Serialize};
use std::path::PathBuf;

use crate::filesystem::utils::traits::{Configuration, ValidationError};
pub(crate) use crate::filesystem::workspace::global::backups::WorkspaceBackupStrategy;
use lazy_static::lazy_static;
use regex::Regex;
// pub(crate) use crate::filesystem::workspace::global::flags::{WorkspaceFlags, serialize_flags, deserialize_flags};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct WorkspaceManifest {
    pub id: Option<String>,
    pub name: Option<String>,
    pub local_path: Option<String>,
    pub cd: Option<DateTime<Local>>,
    pub le: Option<DateTime<Local>>,
    pub backup_strategy: Option<WorkspaceBackupStrategy>,
    pub rpc: Option<RichPresenceConfig>,
    // #[serde(serialize_with = "serialize_flags", deserialize_with = "deserialize_flags")]
    pub flags: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct S3BackupStrategy {
    pub bucket: Option<String>,
    pub region: Option<String>,
    pub root_path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct RsyncBackupStrategy {}




pub type PathResult<T> = Result<T, PathError>;

#[derive(Debug, Clone)]
pub struct PathError {
    reason: String,
    code: u8,
}

impl PathError {
    pub fn new<S: Into<String>>(reason: S, code: u8) -> Self {
        Self {
            reason: reason.into(),
            code,
        }
    }
}

lazy_static! {
    pub static ref PATH_PARSER: Regex =
        Regex::new("([A-Za-z0-9-_+~@:.]*)/{0,2}").unwrap();
}

impl WorkspaceManifest {
    pub fn parse_local_path(&self) -> PathResult<PathBuf> {
        debug!("Parsing local path - {:?}", self.local_path);

        if let Some(local_path) = &self.local_path {
            let mut parsed_parts: Vec<String> = vec![];
            let parts = PATH_PARSER.captures_iter(local_path);

            let mut index = 0;
            for c1 in parts {
                let part = c1.get(1).unwrap().as_str();

                match part {
                    ":WSP_DIR:" => {
                        if index > 0 {
                            return Err(PathError::new(
                                "Cannot add workspace directory anywhere but start of path",
                                0b00000010,
                            ));
                        }

                        let mut doc_dir = dirs::document_dir().unwrap();
                        doc_dir.push("noot");
                        parsed_parts
                            .push(doc_dir.to_str().unwrap().to_string());
                    }
                    x => parsed_parts.push(x.trim().to_string()),
                }

                index += 1;
            }

            return Ok(PathBuf::from_iter(parsed_parts.into_iter()));
        }

        Err(PathError {
            reason: "Cannot parse empty path".to_string(),
            code: 0b00000001,
        })
    }
}

impl Configuration for WorkspaceManifest {
    fn validate(&self, prefix: &str) -> Vec<ValidationError> {
        let errors = Vec::new();

        errors
    }

    fn repair(&mut self) {
        todo!()
    }
}

pub mod backups;
pub mod flags;

#[cfg(test)]
mod tests {

    use super::*;
    use crate::filesystem::workspace::global::backups::git::GitBackupStrategy;

    #[test]
    fn test_parse_local_path_wsp_dir() {
        let mut doc_dir = dirs::document_dir().unwrap();
        doc_dir.push("noot");
        doc_dir.push("test_workspace");

        let test_manifest = WorkspaceManifest {
            id: None,
            name: None,
            local_path: Some(":WSP_DIR:/test_workspace".to_string()),
            cd: None,
            le: None,
            backup_strategy: None,
            rpc: None,
            flags: None,
        };

        let local_path = test_manifest.parse_local_path().unwrap();

        assert_eq!(local_path, doc_dir)
    }

    #[test]
    fn test_parse_local_path_invalid_wsp_dir() {
        let test_manifest = WorkspaceManifest {
            id: None,
            name: None,
            local_path: Some("invalid/:WSP_DIR:/test_workspace".to_string()),
            cd: None,
            le: None,
            backup_strategy: None,
            rpc: None,
            flags: None,
        };

        let local_path = test_manifest.parse_local_path().unwrap_err();

        assert_eq!(local_path.code, 0b00000010);
    }

    #[test]
    fn test_parse_local_path_non_wsp_dir() {
        let test_manifest = WorkspaceManifest {
            id: None,
            name: None,
            local_path: Some("noot/test_workspace".to_string()),
            cd: None,
            le: None,
            backup_strategy: None,
            rpc: None,
            flags: None,
        };

        let local_path = test_manifest.parse_local_path().unwrap();

        assert_eq!(local_path, PathBuf::from("noot/test_workspace"))
    }
}
