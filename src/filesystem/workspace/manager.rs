use std::str::FromStr;
use crate::filesystem::workspace::global::WorkspaceManifest;
use crate::filesystem::workspace::state::WorkspaceState;
use hashbrown::HashMap;

#[derive(Debug, Clone)]
pub struct WorkspaceManager {
    pub all: HashMap<String, WorkspaceManifest>,
    pub active: Option<WorkspaceState>,
}

impl WorkspaceManager {
    pub fn new() -> Self {
        WorkspaceManager {
            all: HashMap::new(),
            active: None,
        }
    }

    pub fn ingest_config(&mut self, workspaces: Vec<WorkspaceManifest>) {
        debug!("Ingesting workspace manifests from config file");
        for workspace in workspaces {
            debug!("Ingesting workspace {}", workspace.id.clone());
            self.all.insert(workspace.id.clone(), workspace);
        }
    }



    pub async fn load_workspace(&mut self, id: String) -> WorkspaceResult<WorkspaceState> {
        debug!("Loading workspace {}", id);

        if let Some(manifest) = self.all.get(&id) {
            let root_dir = manifest.parse_local_path().unwrap();
            debug!("Workspace root path is '{:?}'", &root_dir);
            
            let exists_result = tokio::fs::try_exists(&root_dir).await;
            
            if let Ok(outcome) = exists_result {
                
            } else {
                //
            }
            
            Err("".into())
        } else {
            error!("Workspace not found: {}", id);
            Err(WorkspaceError::WorkspaceManifestNotFound(id.clone()))
        }

        // debug!("Previous workspace referenced, checking manifests");
        // let workspace_manifest = cfg.workspaces.iter().filter(|p| {
        //     debug!("Checking workspace {} ({} - {})", p.name, p.id, &prev_wsp);
        //     if p.id == prev_wsp {
        //         info!("Previous workspace {} ({})", p.name, prev_wsp);
        //         return true
        //     }
        //     warn!("Workspace does not match");
        //     false
        // }).next();
        //
        // if let Some(workspace_manifest) = workspace_manifest {
        //     debug!("Workspace manifest found - Attempting to load");
        //     return Task::perform(WorkspaceState::open_workspace_from_manifest(workspace_manifest.clone()), Message::WorkspaceLoaded);
        // } else {
        //     warn!("Workspace manifest not found - Defaulting to LandingView");
        // }
    }
}

pub type WorkspaceResult<T> = Result<T, WorkspaceError>;


#[derive(Debug, Clone)]
pub enum WorkspaceError {
    WorkspaceDoesNotExist(String),
    WorkspaceManifestNotFound(String),
    WorkspaceDecryptionFailed(String),
    Unknown(String)
}

impl Into<WorkspaceError> for &'static str{
    fn into(self) -> WorkspaceError {
        WorkspaceError::Unknown(format!("{:?}", self))
    }
}

impl FromStr for WorkspaceError {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(s.to_string()))
    }
}

impl From<String> for WorkspaceError {
    fn from(value: String) -> Self {
        WorkspaceError::Unknown(value)
    }
}
