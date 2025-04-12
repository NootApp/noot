use std::collections::BTreeMap;
use std::sync::Arc;
use crossbeam_queue::ArrayQueue;
use crate::config::Config;
use crate::runtime::workers::Job;
use crate::storage::process::ProcessStorageManager;
use crate::storage::process::structs::workspace::Workspace;

/// A global runtime state that can be modified by anyone with a copy of it at any time.
#[derive(Debug)]
pub struct AppState {
    /// The configuration data as it has been loaded from the disk.
    /// > :warning: Caution: This is currently loaded via `Default::default()` and is not persisted to disk.
    pub config: Config,

    /// The storage manager for the current application. Manages the information within the main database file.
    pub store: ProcessStorageManager,

    /// A map of the available workspaces (as found by the `ProcessStoreManager`),
    /// this contains the ID of the workspace as its key, and stores a
    /// partial workspace entry for referencing in the GUI.
    pub workspaces: BTreeMap<String, Workspace>,

    /// Whether IPC is running. (always false at this time)
    pub run_ipc: bool,

    /// The current process ID. (as assigned by the OS)
    /// This is used alongside the IPC subsystem to route messages.
    pub pid: u32,

    /// The workspace ID that should be opened when a new editor is called.
    pub open_workspace: Option<String>,

    /// A mirror value of the CLI argument
    pub skip_splash: bool,

    /// A mirror value of the CLI argument
    pub load_workspace: Option<String>,

    /// Job queue
    pub queue: Arc<ArrayQueue<Job>>,
}