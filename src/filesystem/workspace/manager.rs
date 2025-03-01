use hashbrown::HashMap;
use crate::filesystem::workspace::global::WorkspaceManifest;
use crate::filesystem::workspace::state::WorkspaceState;

#[derive(Debug, Clone)]
pub struct WorkspaceManager {
    pub all: HashMap<String, WorkspaceManifest>,
    pub active: Option<WorkspaceState>
}


impl WorkspaceManager {
    pub fn new() -> Self {
        WorkspaceManager {
            all: HashMap::new(),
            active: None,
        }
    }
}