use crate::filesystem::workspace::state::{ResolverMethod, Screen, WorkspaceFile, WorkspaceState};
use chrono::{DateTime, Local};
use hashbrown::HashMap;
use serde_derive::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinifiedWorkspaceState {
    pub manifest: String,
    pub viewport: Screen,
    pub plugins: HashMap<String, bool>,
    pub cache_dir: PathBuf,
    pub assets_dirs: Vec<PathBuf>,
    pub resolver_method: ResolverMethod,
    pub last_update: DateTime<Local>,
    pub dirty: bool,
    pub files: HashMap<PathBuf, MinifiedWorkspaceFile>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinifiedWorkspaceFile {
    pub path: PathBuf,
}

impl WorkspaceFile {
    pub fn store(&self) -> PathBuf {
        let mut tmp_path = self.path.clone();

        let name = self.path.file_name().unwrap().to_str().unwrap();

        // Remove file name
        debug!("Temp path: {:?}", tmp_path);

        tmp_path.push(".noot");
        tmp_path.push(".cache");
        tmp_path.set_file_name(format!("{}.cache",name));
        debug!("Temp file: {:?}", tmp_path);

        tmp_path
    }
}

impl MinifiedWorkspaceState {
    pub fn from_state(s: WorkspaceState) -> Self {
        Self {
            manifest: toml::to_string(&s.manifest).unwrap(),
            viewport: s.viewport.clone(),
            plugins: s.plugins.clone(),
            cache_dir: s.cache_dir.clone(),
            assets_dirs: s.assets_dirs.clone(),
            resolver_method: s.resolver_method.clone(),
            last_update: s.last_update.clone(),
            dirty: s.dirty.clone(),
            files: MinifiedWorkspaceFile::from_state(s),
        }
    }
}


impl MinifiedWorkspaceFile {
    pub fn from_state(s: WorkspaceState) -> HashMap<PathBuf, MinifiedWorkspaceFile> {
        let mut files = HashMap::new();

        for (path, file) in s.files {
            files.insert(path, Self::from_file(file));
        }

        files
    }

    pub fn from_file(file: WorkspaceFile) -> Self {
        Self {
            path: file.path,
        }
    }
}