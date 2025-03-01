use hashbrown::HashMap;
use std::path::PathBuf;
use chrono::{DateTime, Local};
use serde_derive::{Deserialize, Serialize};
use WorkspaceBackupStrategy::Git;
use crate::filesystem::workspace::global::{WorkspaceBackupStrategy, WorkspaceManifest};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct WorkspaceState {
    // We do not want to serialize the workspace manifest into the workspace state, as this is a seperate entity
    #[serde(skip)]
    pub manifest: WorkspaceManifest,
    pub viewport: Screen,
    pub plugins: HashMap<String, bool>,
    pub assets_dirs: Vec<PathBuf>,
    pub resolver_method: ResolverMethod,
    pub last_update: DateTime<Local>,
    pub dirty: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ResolverMethod {
    Proprietary,
    Spec
}


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Screen {
    Split(Box<(Screen, Screen)>),
    Editor(WorkspaceFile)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceFile {
    /// TODO: Implement history
    history: Vec<()>,
    /// TODO: Implement redo
    redo_queue: Vec<()>,
    content: String,
    path: PathBuf,
}



impl WorkspaceState {
    pub async fn open_workspace_from_manifest(manifest: WorkspaceManifest) -> WorkspaceState {
        let workspace_path = manifest.parse_local_path();
        let workspace_exists = tokio::fs::try_exists(&workspace_path).await;

        match workspace_exists {
            Ok(exists) => {
                if !exists {
                    let create_dir_result = tokio::fs::create_dir_all(&workspace_path).await;
                    if create_dir_result.is_err() {
                        error!("Failed to create workspace directory: {:?}", &workspace_path);
                        error!("{:?}", create_dir_result.unwrap_err());
                        panic!("Failed to create workspace directory - See log for details");
                    }

                    match manifest.backup_strategy {
                        // Git(bs) => {
                        // 
                        // }
                        _ => {
                            return Self::create_empty_workspace(manifest).await;
                        }
                    }
                }
            },
            Err(e) => {
                error!("Failed to open workspace: {}", e);
            }
        };

        info!("Attempting to ");

        ()
    }

    pub async fn create_empty_workspace(manifest: WorkspaceManifest) -> WorkspaceState {
        WorkspaceState {
            manifest,
            viewport: Screen::Editor(WorkspaceFile::),
            plugins: HashMap::new(),
            assets_dirs: vec![],
            resolver_method: ResolverMethod::Proprietary,
            last_update: Default::default(),
            dirty: false,
        }
    }

}


impl WorkspaceFile {
    pub fn open(path: PathBuf) -> WorkspaceFile {
        let 
    }
}