use crate::filesystem::workspace::global::WorkspaceManifest;
use crate::filesystem::workspace::global::flags::WorkspaceFlags;
use crate::filesystem::workspace::state::minified::MinifiedWorkspaceState;
use crate::filesystem::workspace::state::plugins::PluginManifest;
use chrono::{DateTime, Local};
use hashbrown::HashMap;
use serde_derive::{Deserialize, Serialize};
use std::fs::{File};
use std::io::Write;
use std::path::PathBuf;
use std::sync::Arc;

pub mod minified;
pub mod plugins;

#[derive(Debug, Clone)]
pub struct WorkspaceState {
    // We do not want to serialize the workspace manifest into the workspace state, as this is a separate entity
    pub manifest: Arc<WorkspaceManifest>,
    pub viewport: Screen,
    pub plugins: HashMap<String, PluginManifest>,
    pub cache_dir: PathBuf,
    pub assets_dirs: Vec<PathBuf>,
    pub resolver_method: ResolverMethod,
    pub last_update: DateTime<Local>,
    pub dirty: bool,
    pub files: HashMap<PathBuf, WorkspaceFile>,
    // pub media: MediaConfig,
}

impl WorkspaceState {
    // pub fn create_directories(&mut self) -> Result<(), std::io::Error> {
    //     let root = self.manifest.parse_local_path().unwrap();
    //     let workspace = root.join(".noot");
    //     let cache_dir = root.join(".noot/cache");
    //     let plugin_dir = root.join(".noot/plugins");
    //     let asset_dir = root.join(".noot/assets");
    //     let asset_dir_primary = root.join("assets");
    // 
    //     if !self.cache_dir.exists() {
    //         self.cache_dir = cache_dir;
    //         create_dir_all(&self.cache_dir)?
    //     }
    // 
    //     Ok(())
    // }

    // pub fn load_plugins(&mut self) {}

    // pub fn resolve_path(&self) -> PathBuf {
    //     match self.resolver_method {
    //         ResolverMethod::Proprietary => {
    //
    //         }
    //     }
    // }

    pub fn store(&self) -> Result<(), std::io::Error> {
        info!("Storing workspace state");
        let path = self.manifest.parse_local_path().unwrap();
        let mini = MinifiedWorkspaceState::from_state(self.clone());
        let serial = toml::to_string(&mini).unwrap();

        let flags =
            WorkspaceFlags::from(self.manifest.flags.unwrap_or_default());
        let mut wsp_path = path.clone();

        wsp_path.push(".noot");

        if std::fs::exists(wsp_path.clone()).unwrap_or(false) {
            debug!("Workspace data dir exists ({})", wsp_path.display());
        } else {
            debug!(
                "Workspace data dir does not exist ({})",
                wsp_path.display()
            );
            let outcome = std::fs::create_dir_all(&wsp_path);
            if let Ok(_) = outcome {
                debug!("Workspace data dir created ({})", wsp_path.display());
            }
        }

        let manifest_path = wsp_path.join("manifest.toml");

        debug!("Workspace manifest path: {}", manifest_path.display());

        if flags.contains(WorkspaceFlags::ENCRYPTED) {
            info!("Workspace is encrypted");
            let enterprise = flags.contains(WorkspaceFlags::ENTERPRISE);
            crate::subsystems::cryptography::storage::store(
                &manifest_path,
                serial.as_bytes(),
                enterprise,
            )?;
            info!(
                "Wrote encrypted manifest file ({})",
                manifest_path.display()
            );
        } else {
            info!("Workspace is not encrypted");

            let mut handle = File::options()
                .write(true)
                .create(true)
                .truncate(true)
                .open(&manifest_path)?;
            handle.write_all(serial.as_bytes())?;
            handle.sync_all()?;

            info!("Wrote manifest file ({})", manifest_path.display());
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ResolverMethod {
    Smart,
    Proprietary,
    Spec,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Screen {
    Split(Box<(Screen, Screen)>),
    Editor(PathBuf),
    Welcome,
    Empty,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceFile {
    /// TODO: Implement history
    history: Vec<()>,
    /// TODO: Implement redo
    redo_queue: Vec<()>,
    content: Vec<u8>,
    mime_type: String,
    path: PathBuf,
}



#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct MediaConfig {

}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum MediaSyncStrategy {
    LocalHashed,
    Local,
    Remote,
}

impl WorkspaceState {
    pub async fn open_workspace_from_manifest(
        manifest: WorkspaceManifest,
    ) -> WorkspaceState {
        let workspace_path = manifest.parse_local_path().unwrap();
        let workspace_exists = tokio::fs::try_exists(&workspace_path).await;
        let mut viewport: Screen = Screen::Empty;
        match workspace_exists {
            Ok(exists) => {
                if !exists {
                    viewport = Screen::Welcome;
                    let create_dir_result =
                        tokio::fs::create_dir_all(&workspace_path).await;
                    if create_dir_result.is_err() {
                        error!(
                            "Failed to create workspace directory: {:?}",
                            &workspace_path
                        );
                        error!("{:?}", create_dir_result.unwrap_err());
                        panic!(
                            "Failed to create workspace directory - See log for details"
                        );
                    }

                    return match manifest.backup_strategy {
                        // Git(bs) => {
                        //
                        // }
                        _ => Self::create_empty_workspace(manifest).await,
                    };
                }
            }
            Err(e) => {
                error!("Failed to open workspace: {}", e);
            }
        };

        info!("Attempting to ");

        WorkspaceState {
            manifest: Arc::new(manifest),
            viewport,
            plugins: Default::default(),
            cache_dir: Default::default(),
            assets_dirs: vec![],
            resolver_method: ResolverMethod::Proprietary,
            last_update: Default::default(),
            dirty: false,
            files: Default::default(),
            // media: MediaConfig {},
        }
    }

    pub async fn create_empty_workspace(
        manifest: WorkspaceManifest,
    ) -> WorkspaceState {
        let workspace_path = manifest.parse_local_path().unwrap();
        let asset_dir = workspace_path.join(".assets");
        let _manifest_dir = workspace_path.join(".manifest");
        let cache_dir = workspace_path.join(".cache");

        // let manifest_core_file = manifest_dir.join(".noot.wsp");

        let temporary_state = WorkspaceState {
            manifest: Arc::new(manifest),
            viewport: Screen::Welcome,
            plugins: HashMap::new(),
            cache_dir,
            assets_dirs: vec![asset_dir],
            resolver_method: ResolverMethod::Proprietary,
            last_update: Default::default(),
            dirty: false,
            files: HashMap::new(),
            // media: MediaConfig {},
        };

        for asset_directory in temporary_state.assets_dirs.iter() {
            debug!("Creating asset directory: {:?}", asset_directory);
            let asset_dir_create_result =
                tokio::fs::create_dir_all(asset_directory).await;
            if asset_dir_create_result.is_err() {
                error!(
                    "Failed to create asset directory: {:?}",
                    asset_directory
                );
            } else {
                debug!("Created asset directory: {:?}", asset_directory);
            }
        }

        debug!("Creating cache directory: {:?}", &temporary_state.cache_dir);
        let cache_dir_create_result =
            tokio::fs::create_dir_all(&temporary_state.cache_dir).await;
        if cache_dir_create_result.is_err() {
            error!(
                "Failed to create cache directory: {:?}",
                &temporary_state.cache_dir
            );
        }

        temporary_state
    }

    // Store the current workspace state, saving all file contents to temp files and encrypting as required
    // pub async fn store_workspace_state(&self) -> Result<(), tokio::io::Error> {
    //     // Check if we should use encryption
    //     let encrypt = self.manifest.flags.contains(WorkspaceFlags::ENCRYPTED);
    //
    //     for (key, file) in &self.files {
    //         debug!("Saving workspace file: {:?}", key);
    //         let key =
    //         file.save(encrypt).await?;
    //     }
    //
    //     if self.manifest.flags.contains(WorkspaceFlags::ENCRYPTED) {
    //
    //     } else {
    //
    //     }
    //     Ok(())
    // }
}

impl WorkspaceFile {
    // pub async fn open_from_cache(path: PathBuf) -> WorkspaceFile {
    //     let buffer = retrieve(&path).unwrap();
    //     String::from_utf8(buffer).unwrap();
    //
    //     let file = toml::from_str()
    //
    // }

    pub async fn open(path: PathBuf) -> WorkspaceFile {
        let handle = tokio::fs::read(&path).await;

        if handle.is_err() {
            error!("Failed to open file: {:?}", &path);
            error!("{:?}", handle.unwrap_err());
            panic!("Failed to open file: {:?}", &path);
        }

        let buffer = handle.unwrap();
        let mime_type =
            infer::get(&buffer).expect("Failed to extract mime type");

        WorkspaceFile {
            history: vec![],
            redo_queue: vec![],
            content: buffer,
            mime_type: mime_type.to_string(),
            path,
        }
    }

    // pub async fn save(&self, encrypt: bool, key: String) -> Result<(), tokio::io::Error> {
    //
    //     let mut content: Vec<u8> = vec![];
    //
    //
    //     Ok(())
    // }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     #[tokio::test]
//     pub async fn test_workspace_file_storage() {
//         let file = WorkspaceFile::open().await;
//     }
// }
