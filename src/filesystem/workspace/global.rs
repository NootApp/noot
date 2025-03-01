use std::path::PathBuf;
use chrono::{DateTime, Local, Utc};
use lazy_static::lazy_static;
use nanoid::nanoid;
use serde_derive::{Deserialize, Serialize};
use crate::filesystem::config::Config;
use crate::subsystems::discord::config::RichPresenceConfig;


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct WorkspaceManifest {
    pub id: String,
    pub name: String,
    pub store: Option<String>,
    pub local_path: Option<String>,
    pub cd: DateTime<Local>,
    pub le: DateTime<Local>,
    pub backup_strategy: WorkspaceBackupStrategy,
    pub rpc: WorkspaceRichPresenceConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct WorkspaceRichPresenceConfig {
    pub enable: bool,
    pub enable_idle: bool,
    pub show_current_file: bool,
    pub show_workspace_name: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum WorkspaceBackupStrategy {
    S3(S3BackupStrategy),
    Rsync(RsyncBackupStrategy),
    Git(GitBackupStrategy),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct GitBackupStrategy {
    permit_remotes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct S3BackupStrategy {
    bucket: String,
    region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct RsyncBackupStrategy {}


impl WorkspaceManifest {
    pub fn parse_local_path(&self) -> PathBuf {
        debug!("Parsing local path - {:?}", self.local_path);
        let mut doc_dir = dirs::document_dir().unwrap();
        
        doc_dir.push("noot");
        
        
        if let Some(p) = &self.local_path {
            if p.contains(":WSP_DIR:") {
                let p2 = p.replace(":WSP_DIR:", "");
                
                doc_dir.push(p2);
            }
        } else {
            doc_dir.push(self.local_path.clone().unwrap());
        }
        
        debug!("Local path: {:?}", doc_dir);
        
        doc_dir
    }
}


pub struct WorkspaceFlags {
    
}