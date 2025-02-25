use serde_derive::{Serialize, Deserialize};
use std::path::PathBuf;
use chrono::{DateTime, Local};
use nanoid::nanoid;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all="kebab-case")]
pub enum WorkspaceManifestFormatVersion {
    V001
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all="kebab-case")]
pub struct WorkspaceManifestList  {
    pub format: WorkspaceManifestFormatVersion,
    pub last_opened: String,
    pub workspaces: Vec<Workspace>,
}


#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all="kebab-case")]
pub struct Workspace {
    pub id: String,
    pub display_name: String,
    pub disk_path: String,
    pub last_accessed: DateTime<Local>,
    pub checksum: Option<String>
}

impl WorkspaceManifestList {
    pub fn new() -> WorkspaceManifestList {
        let document_dir = get_workspace_path();
        let mut workspace_dir = document_dir.clone();
        let workspace_id = nanoid!(10);
        workspace_dir.push("starter-workspace");
        WorkspaceManifestList {
            format: WorkspaceManifestFormatVersion::V001,
            last_opened: workspace_id.clone(),
            workspaces: vec![
                Workspace {
                    id: workspace_id,
                    display_name: "Starter Workspace".to_string(),
                    disk_path: workspace_dir.to_str().unwrap().to_string(),
                    last_accessed: Local::now(),
                    checksum: None
                }
            ]
        }
    }
}

// TODO: Introduce workspace methods for creating new workspaces
impl Workspace {}

#[tauri::command]
pub fn get_workspace_config() -> WorkspaceManifestList {
    WorkspaceManifestList::new()
}

#[tauri::command]
pub fn get_active_workspace() -> Option<Workspace> {
    let manifest = WorkspaceManifestList::new();
    let mut mr: Option<Workspace> = None;

    for workspace in manifest.workspaces.iter() {
        if workspace.id == manifest.last_opened {
            mr = Some(workspace.clone());
        }
    }

    mr
}


#[cfg(debug_assertions)]
pub fn get_workspace_path() -> PathBuf {
    let doc_dir = dirs::document_dir();
    let mut doc_path = doc_dir.unwrap();
    doc_path.push("noot-dev");
    dbg!(&doc_path);
    return doc_path;
}

#[cfg(not(debug_assertions))]
pub fn get_workspace_path() -> PathBuf {
    let doc_dir = dirs::document_dir();
    let mut doc_path = doc_dir.unwrap();
    doc_path.push("noot");
    dbg!(&doc_path);
    return doc_path;
}
