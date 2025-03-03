use crate::filesystem::workspace::state::{ResolverMethod, Screen};
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
    pub password: Vec<u8>,
}
