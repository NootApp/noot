use serde_derive::{Serialize, Deserialize};
use chrono::{DateTime, Local};


#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all="kebab-case")]
pub enum WorkspaceManifestFormatVersion {
    V001
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all="kebab-case")]
pub struct WorkspaceManifestList  {
    pub format: WorkspaceManifestFormatVersion,
    pub workspaces: Vec<Workspace>,
}


#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all="kebab-case")]
pub struct Workspace {
    pub display_name: String,
    pub disk_path: String,
    pub last_accessed: DateTime<Local>,
    pub checksum: String
}


//
//impl WorkspaceManifestList {
//    pub fn new() -> WorkspaceManifestList {
//        WorkspaceManifestList {
//            format: WorkspaceManifestFormatVersion::V001,
//            workspaces: vec![
//                Workspace {
//                    display_name: "Starter Workspace".to_string(),
//                    disk_path: 
//                }
//            ]
//        }
//    }
//}
