use std::str::FromStr;
use std::sync::Mutex;
use crate::filesystem::workspace::global::WorkspaceManifest;
use crate::filesystem::workspace::state::{ResolverMethod, Screen, WorkspaceState};
use hashbrown::HashMap;
use lazy_static::lazy_static;
use crate::filesystem::workspace::global::backups::BackupStrategy;
use crate::filesystem::workspace::manager::WorkspaceError::WorkspaceCheckFailed;
use crate::filesystem::workspace::state::minified::MinifiedWorkspaceState;
use crate::filesystem::workspace::state::plugins::PluginManifest;
use crate::subsystems::cryptography::storage::{retrieve, CONSUMER_MAGIC, ENTERPRISE_MAGIC};

lazy_static!(
  pub static ref MANAGER: Mutex<WorkspaceManager> = Mutex::new(WorkspaceManager::new());  
);


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
            debug!("Ingesting workspace {}", workspace.id.clone().unwrap());
            self.all.insert(workspace.id.clone().unwrap(), workspace);
        }
    }



    pub async fn load_workspace(&mut self, id: String) -> WorkspaceResult<WorkspaceState> {
        debug!("Loading workspace {}", id);

        if let Some(manifest) = self.all.get(&id) {
            let root_dir = manifest.parse_local_path().unwrap();
            debug!("Workspace root path is '{:?}'", &root_dir);
            
            let exists_result = tokio::fs::try_exists(&root_dir).await;
            let mut workspace = WorkspaceState {
                manifest: manifest.clone(),
                viewport: Screen::Empty,
                plugins: HashMap::from([("example".to_string(), PluginManifest {
                    version: "0.0.1".to_string(),
                    author: "nootapp".to_string(),
                    repository: "example_plugin".to_string(),
                    source_name: "github".to_string(),
                })]),
                
                cache_dir: Default::default(),
                assets_dirs: vec![],
                resolver_method: ResolverMethod::Proprietary,
                last_update: Default::default(),
                dirty: false,
                files: Default::default(),
            };

            if let Ok(outcome) = exists_result {
                if outcome {
                    let mut noot_dir = root_dir.clone();
                    noot_dir.push(".noot");

                    let noot_exists = tokio::fs::try_exists(&noot_dir).await;

                    if noot_exists.is_ok() {
                        debug!("Noot dir exists");
                        let manifest_file = noot_dir.join("manifest.toml");

                        if std::fs::exists(&manifest_file).unwrap_or(false) {
                            debug!("Manifest file exists");
                            let mut content = std::fs::read(&manifest_file).unwrap();

                            if [ENTERPRISE_MAGIC, CONSUMER_MAGIC].contains(&content[0]) {
                                debug!("Manifest is encrypted");
                                content = retrieve(&manifest_file)?;
                            }

                            let cstring = String::from_utf8(content).unwrap();

                            let ws2: MinifiedWorkspaceState = toml::from_str(&cstring).unwrap();

                            workspace.manifest = ws2.manifest;
                            workspace.viewport = ws2.viewport;
                            workspace.plugins = ws2.plugins;
                            workspace.cache_dir = ws2.cache_dir;
                            workspace.assets_dirs = ws2.assets_dirs;
                            workspace.resolver_method = ws2.resolver_method;
                            workspace.last_update = ws2.last_update;
                            workspace.dirty = ws2.dirty;
                            workspace.store().unwrap();
                            return Ok(workspace);
                        } else {
                            error!("Manifest file not found at location");
                            error!("Path: {}", manifest_file.display());
                            workspace.store().unwrap();
                            return Ok(workspace);
                        }

                    } else {
                        debug!("Noot dir does not exist - Need to init project");
                    }

                    return Err(WorkspaceCheckFailed("Not Implemented (workspace exists but cannot load)".to_string()))
                } else {
                    if let Some(mut git) = manifest.backup_strategy.clone().unwrap().git.clone() {
                        let outcome = git.fetch(&root_dir);
                        if let Err(e) = outcome {
                            error!("Failed to fetch git repository {:?}", e);
                            return Err(e);
                        }
                    }
                }
            } else {
                return Err(WorkspaceError::WorkspaceCheckFailed(exists_result.unwrap_err().to_string()));
            }

            let _ = workspace.store();


            Ok(workspace)
        } else {
            error!("Workspace not found: {}", id);
            Err(WorkspaceError::WorkspaceManifestNotFound(id.clone()))
        }
    }
}

pub type WorkspaceResult<T> = Result<T, WorkspaceError>;


#[derive(Debug, Clone)]
pub enum WorkspaceError {
    WorkspaceDoesNotExist(String),
    WorkspaceManifestNotFound(String),
    WorkspaceDecryptionFailed(String),
    Unknown(String),
    FailedToFetch(String),
    WorkspaceCheckFailed(String),
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
