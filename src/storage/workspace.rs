use std::fmt::Debug;
use std::fs::create_dir_all;
use std::ops::{Deref, DerefMut};
use std::path::PathBuf;
use std::sync::{Arc, Mutex, MutexGuard};
use bincode::{config, encode_to_vec, Decode, Encode};
use chrono::Local;
use rusqlite::{params, Connection};
use crate::runtime::{AppState, GLOBAL_STATE};
use crate::storage::process::structs::setting::Setting;
use crate::storage::process::structs::workspace::Workspace;


const WORKSPACE_SEED: &'static str = include_str!("../../database/workspace.sql");

#[derive(Debug)]
pub struct WorkspaceManager {
    state: Arc<Mutex<AppState>>,
    db: Connection,
    pub source: Workspace,
    pub tree: Vec<FileEntry>
}

#[derive(Debug)]
pub enum FileEntry {
    File(PathBuf),
    Dir(PathBuf, Vec<FileEntry>),
}

#[derive(Encode, Decode, Debug, Copy, Clone)]
pub enum AssetCachingStrategy {
    /// Store remote assets in memory.
    /// Requires a constant network connection
    /// to ensure consistent availability
    Memory,

    /// Store remote assets in blob storage
    /// Prevents accidental leaking of data via filesystem
    /// all assets will be stored within the database in an encrypted format
    Blob,

    /// Store remote assets on the disk in the `.assets` directory
    /// these assets are not encrypted, for compatability reasons
    Disk
}

#[derive(Encode, Decode, Debug, Copy, Clone)]
pub enum RemoteDataStrategy {
    /// Do not fetch remote data, be it images, videos, or other assets
    None,

    /// Only fetch remote data from specified domains
    AllowSpecific,

    /// Fetch all remote data, regardless of domain
    All
}




pub type WorkspaceResult<T> = Result<T, WorkspaceError>;
#[derive(Debug)]
pub enum WorkspaceError {
    WorkspaceInvalid(String),
    WorkspaceNotFound(String),
    RootNotFound(String)
}

impl WorkspaceManager {
    pub fn new(source: Workspace) -> WorkspaceResult<WorkspaceManager> {
        let workspace_dir = PathBuf::from(&source.disk_path);

        if workspace_dir.exists() && workspace_dir.is_dir() {
            // The workspace is a folder and does exist
            let noot_dir = workspace_dir.join(".noot");
            let connection = Connection::open(&noot_dir.join("workspace.db")).unwrap();

            Ok(Self {
                state: GLOBAL_STATE.clone(),
                db: connection,
                source,
                tree: Vec::new()
            })
        } else if workspace_dir.exists() && !workspace_dir.is_dir() {
            // The workspace exists but is not a folder
            Err(WorkspaceError::WorkspaceInvalid(format!("The path '{}' is not a directory", workspace_dir.display())))
        } else {
            // The workspace does not exist
            Err(WorkspaceError::WorkspaceNotFound(workspace_dir.display().to_string()))
        }
    }


    /// Utility method to create a new empty workspace given a name and drive path
    pub fn create(name: String, path: String) -> WorkspaceResult<WorkspaceManager> {
        let source = Workspace {
            id: nanoid!(),
            name,
            disk_path: path.clone(),
            last_accessed: Local::now(),
        };

        let root = PathBuf::from(path);

        info!("Root folder: {}", root.display());

        if !root.exists() {
            let maybe_create = create_dir_all(&root);

            if maybe_create.is_err() {
                error!("Failed to create root directory for workspace");
                return Err(WorkspaceError::RootNotFound("Could not create workspace root directory".to_string()))
            }


        }

        let state = GLOBAL_STATE.clone();

        let lock = state.lock().unwrap();

        lock.store.create_workspace(source.clone());

        drop(lock);

        let mut noot_path = root.clone();
        noot_path.push(".noot");
        info!("Noot path: {}", noot_path.display());

        if !noot_path.exists() {
            create_dir_all(noot_path.clone()).unwrap();
        }

        noot_path.push(".empty-file");
        info!("Noot DB path: {}", noot_path.with_file_name("workspace.db").display());

        let mut connection = Connection::open(noot_path.with_file_name("workspace.db")).unwrap();

        let mut tx = connection.transaction().unwrap();

        tx.execute_batch(WORKSPACE_SEED).unwrap();

        tx.commit().unwrap();

        connection.close().unwrap();

        let mut mgr = WorkspaceManager::new(source)?;

        mgr.set_setting("plugins.enable", None::<()>, false)
            .set_setting("plugins.allow-unpacked", None::<()>, false)
            .set_setting("assets.cache-strategy", Some(AssetCachingStrategy::Blob), true)
            .set_setting("assets.fetch-remote", Some(RemoteDataStrategy::All), false);


        Ok(mgr)

    }

    pub fn get_setting<T: Encode + Decode<()> + Debug>(&mut self, key: impl Into<String>) -> Option<Setting<T>> {
        let k = key.into();
        let mut stmt = self.db.prepare("SELECT * FROM settings WHERE id = ?").unwrap();
        let outcome = stmt.query_row([&k], |r| Ok(Some(Setting::from(r))));

        if let Ok(outcome) = outcome {
            outcome
        } else {
            error!("Setting didnt exist: '{}'", k);
            None
        }
    }

    pub fn set_setting<T: Encode + Decode<()> + Debug>(&mut self, key: impl Into<String>, value: Option<T>, enabled: bool) -> &mut Self {
        let k = key.into();
        let mut v = None;
        if let Some(value) = value {
            v = Some(encode_to_vec(value, config::standard()).unwrap());
        }
        let mut stmt = self.db.prepare("UPDATE settings SET value = ?, enabled = ? WHERE id = ?").unwrap();
        stmt.execute((v, enabled, k)).unwrap();
        drop(stmt);
        self
    }
}


pub fn render_directory(path: String, workspace_directory: PathBuf) -> PathBuf {
    let mut workspace_dir = PathBuf::from(workspace_directory);

    if path.starts_with("$PROJECT_DIR") {
        let parts = path.split("$PROJECT_DIR/").collect::<Vec<&str>>();
        workspace_dir.push("noot");
        workspace_dir.push(parts[1]);
    }

    workspace_dir
}

pub fn minify_directory(path: String, workspace_directory: PathBuf) -> String {
    let workspace_dir = workspace_directory.as_os_str().to_str().unwrap();

    if path.starts_with(workspace_dir) {
        let parts = path.split(workspace_dir).collect::<Vec<&str>>();
        return format!("$PROJECT_DIR/{}", parts[1]);
    }
    path
}