use crate::filesystem::workspace::global::backups::BackupStrategy;
use crate::filesystem::workspace::manager::{WorkspaceError, WorkspaceResult};
use serde_derive::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct GitBackupStrategy {
    pub permit_remotes: Option<Vec<String>>,
    pub repository: Option<String>,
    pub branch: Option<String>,
}

impl BackupStrategy for GitBackupStrategy {
    fn fetch(&mut self, path: &PathBuf) -> WorkspaceResult<()> {
        let clone_attempt = git2::Repository::clone(
            self.repository.clone().unwrap().as_str(),
            path.to_str().unwrap(),
        );

        if clone_attempt.is_err() {
            let err = clone_attempt.err().unwrap();
            error!("Failed to clone repository: {}", err);
            return Err(WorkspaceError::FailedToFetch(err.to_string()));
        }

        let repository = clone_attempt.unwrap();

        let head = repository.head().unwrap();
        let hash = head.shorthand().unwrap();

        debug!("Repository details");
        debug!("Branch: {:?}", head.name().unwrap());
        debug!("Last Commit Hash: {:?}", hash);
        debug!("Path On Disk: {:?}", path);

        Ok(())
    }

    fn save(&mut self, key: &str) -> WorkspaceResult<()> {
        todo!()
    }
}
