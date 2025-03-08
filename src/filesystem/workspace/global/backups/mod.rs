use std::fmt::Debug;
use std::path::PathBuf;
use hashbrown::HashMap;
use serde::de::DeserializeOwned;
use serde_derive::{Deserialize, Serialize};

pub mod git;




#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct WorkspaceBackupStrategy {
    // pub methods: HashMap<String, Box<dyn BackupStrategy<()>>>,
    // Temporarily disable these two enum variants whilst they are not being worked on or implemented.
    // pub s3: Option<S3BackupStrategy>,
    // pub rsync: Option<rsync::RsyncBackupStrategy>,
    pub git: Option<git::GitBackupStrategy>,
}

use crate::filesystem::workspace::manager::WorkspaceResult;



pub trait BackupStrategy {
    fn fetch(&mut self, path: &PathBuf) -> WorkspaceResult<()>;

    fn save(&mut self, key: &str) -> WorkspaceResult<()>;
}
