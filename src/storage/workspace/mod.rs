use std::collections::BTreeMap;
use std::fmt::Debug;
use std::fs::create_dir_all;
use std::path::PathBuf;
use std::sync::{Arc, MutexGuard};
use bincode::{Decode, Encode};
use chrono::Local;
use crossbeam_queue::ArrayQueue;
use rusqlite::Connection;
use iced::window::Id;
use rusqlite::fallible_iterator::FallibleIterator;
use url::Url;
use crate::runtime::{state::AppState, GLOBAL_STATE};
use crate::runtime::workers::{Job, JobType};
use crate::storage::process::structs::setting::Setting;
use crate::storage::process::structs::workspace::Workspace;
use crate::utils::cryptography::hashing::hash_str;
use crate::assets::{AssetManager};
use self::buffer::Buffer;

const WORKSPACE_SEED: &'static str = include_str!("../../../database/workspace.sql");

pub mod buffer;

#[derive(Debug)]
pub struct WorkspaceManager {
    db: Connection,
    pub source_window: Option<Id>,
    pub source: Workspace,
    pub tree: Vec<FileEntry>,
    pub(crate) assets: AssetManager,
    pub(crate) buffers: BTreeMap<String, Buffer>,
    pub(crate) queue: Arc<ArrayQueue<Job>>,
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
    RootNotFound(String),
    BufferNotFound(String),
}

impl WorkspaceManager {
    pub fn new(source: Workspace, temp_lock: MutexGuard<AppState>) -> WorkspaceResult<WorkspaceManager> {
        info!("Creating workspace manager instance for {}", source.id);
        let workspace_dir = PathBuf::from(&source.disk_path);
        info!("Workspace dir: {:?}", workspace_dir);

        if workspace_dir.exists() && workspace_dir.is_dir() {
            info!("Workspace dir already exists");
            // The workspace is a folder and does exist
            let noot_dir = workspace_dir.join(".noot");
            let connection = Connection::open(&noot_dir.join("workspace.db")).unwrap();

            temp_lock.store.update_workspace(&source.id, Local::now());
            let queue = temp_lock.queue.clone();

            Ok(Self {
                db: connection,
                source_window: None,
                source,
                assets: AssetManager::new(),
                tree: Vec::new(),
                buffers: Default::default(),
                queue
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
        let source_id = nanoid!(5);

        let source = Workspace {
            id: source_id.clone(),
            long_id: hash_str(source_id),
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

        let tx = connection.transaction().unwrap();

        tx.execute_batch(WORKSPACE_SEED).unwrap();

        tx.commit().unwrap();

        connection.close().unwrap();

        let mut mgr = WorkspaceManager::new(source, GLOBAL_STATE.lock().unwrap())?;

        mgr.set_setting("plugins.enable", false)
            .set_setting("plugins.allow-unpacked", false)
            .set_setting("assets.cache-strategy", AssetCachingStrategy::Blob)
            .set_setting("assets.fetch-remote", RemoteDataStrategy::All);


        Ok(mgr)

    }

    pub fn preload(&mut self) -> WorkspaceResult<()> {

        // TODO: Implement file indexing
        self.queue.push(Job::new(JobType::BuildTree(self.source.id.to_string(), PathBuf::from(&self.source.disk_path), self.source_window.unwrap(), true))).unwrap();


        // TODO: Implement asset caching on workspace open
        // TODO: Implement buffer pre-rendering to improve performance

        info!("Workspace Loaded");
        Ok(())
    }

    pub fn set_window_id(&mut self, id: Id) {
        self.source_window = Some(id);
    }

    pub fn open_buffer(&mut self, id: Url) -> WorkspaceResult<()> {
        info!("{}",&id);
        info!("Is Internal? {}", id.scheme() == "noot" && id.host_str().unwrap() == "internal");
       if id.scheme() == "noot" && id.host_str().unwrap() == "internal" {
           let internal_id = id.path();
           info!("Opening internal buffer '{}'", id);
           let maybe_buffer = match internal_id {
               "/test" => self.open_buffer_from_bytes(include_bytes!("../../../static/experiences/test.md"), id.to_string(), id.to_string()),
               _ => Err(WorkspaceError::BufferNotFound(id.to_string()))
           };

           if let Ok(buffer) = maybe_buffer {
               self.buffers.insert(id.to_string(), buffer);
               Ok(())
           } else {
               Err(WorkspaceError::BufferNotFound(id.to_string()))
           }
       } else if let None = self.buffers.get(&id.to_string()) {
           let b = self.open_buffer_from_bytes(&std::fs::read(id.to_file_path().unwrap()).unwrap(), "test.md".to_string(), id.to_string())?;
           self.buffers.insert(id.to_string(), b);
           Ok(())
       } else {
           Err(WorkspaceError::BufferNotFound(id.to_string()))
       }

    }

    fn open_buffer_from_bytes(&self, bytes: &[u8], url: String, name: String) -> WorkspaceResult<Buffer> {
        let b = Buffer::from_md(name, self.source.id.clone(), url, String::from_utf8_lossy(bytes).to_string());

        Ok(b)
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

    pub fn set_setting<T: Encode + Decode<()> + Debug>(&mut self, key: impl Into<String>, value: T) -> &mut Self {
        let k = key.into();
        let setting = Setting::new(k, value);
        setting.store(&mut self.db).unwrap();
        self
    }
}

// pub fn watch_dir() -> impl Stream<Item = Message> {
//     let state = GLOBAL_STATE.clone();
//
//
//     stream::channel(1, move |mut output| {
//         let (tx, rx) = std::sync::mpsc::channel::<notify::Result<notify::Event>>();
//         let state = state;
//
//         let mut temp_lock = state.lock().unwrap();
//         let mut workspace_id = temp_lock.open_workspace.clone().unwrap_or_default();
//
//         while workspace_id.len() == 0 {
//             drop(temp_lock);
//             sleep(Duration::from_millis(200));
//             temp_lock = state.lock().unwrap();
//             workspace_id = temp_lock.open_workspace.clone().unwrap_or_default()
//         }
//
//         let mut path = temp_lock.workspaces.get(&workspace_id).cloned().unwrap().disk_path;
//         drop(temp_lock);
//
//
//         let mut watcher = notify::recommended_watcher(tx).unwrap();
//
//         watcher.watch(Path::new(&path), RecursiveMode::Recursive).unwrap();
//
//         loop {
//             let try_result = rx.try_recv();
//             if let Ok(result) = try_result {
//                 match result {
//                     Ok(event) => {
//                         info!("{:?}", event);
//                         output.try_send(Message::tick()).unwrap()
//                     },
//                     Err(err) => error!("{:?}", err)
//                 }
//             }
//
//             let temp_lock = state.lock().unwrap();
//             let new_workspace_id = temp_lock.open_workspace.clone().unwrap();
//             let new_path = temp_lock.workspaces.get(&new_workspace_id).cloned().unwrap().disk_path;
//             drop(temp_lock);
//
//             if path != new_path {
//                 warn!("Watcher path changed");
//
//                 watcher.unwatch(Path::new(&path)).unwrap();
//                 path = new_path;
//                 watcher.watch(Path::new(&path), RecursiveMode::Recursive).unwrap();
//             }
//
//             sleep(Duration::from_millis(50));
//         }
//     })
// }


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
