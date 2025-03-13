use crate::filesystem::config::Config;
use crate::filesystem::workspace::state::WorkspaceState;
use std::io;
use std::io::Bytes;
use discord_rich_presence::activity::Activity;
use iced::{Application, Task};
use nanoid::nanoid;
use crate::filesystem::workspace::manager::WorkspaceResult;
use crate::Noot;

#[derive(Debug, Clone)]
pub enum Message {
    /*
        Configuration events
    */


    /// Emitted when the configuration file is loaded
    ConfigLoaded(Config),
    // CreateNewWorkspace,


    /*
        Workspace Events
    */
    /// Emitted when the workspace manager attempts to load a workspace
    WorkspaceLoadResult(WorkspaceResult<WorkspaceState>),

    /// Emitted when the manifest list needs to be updated
    WorkspaceIngestManifests,

    WorkspaceLoadStart,


    /*
        Rich Presence Events
    */
    /// Emitted when it is time for the RPC client to initialize
    RPCInit,

    /// Emitted when the RPC client connects to Discord
    RPCConnected,

    /// Emitted when the RPC client disconnects from Discord
    RPCDisconnected,

    /// Emitted when the RPC client modifies the activity
    RPCModified,

    // Emitted when the RPC client should update the activity
    // RPCModifyTrigger(), // TODO: Figure out how we implement this.

    /*
        Thread pool Events
        Jobs which are handed out to a background thread pool to handle
    */
    TPSpawn,
    TPKill,
    
    // TPEncrypt(Vec<u8>),
    // TPEncrypt(Vec<u8>),
    // TPDecrypt(Vec<u8>),
    // TPDecryptResult(Vec<u8>),
    
    

    /*
        Plugin Events
    */
    /// Emitted when a plugin is loaded.
    /// **Params**:
    /// - 1 - String - Plugin name
    /// - 2 - String - Plugin Version
    PluginLoaded(String, String),

    /// Emitted when a plugin is unloaded.
    /// **Params**:
    /// - 1 - String - Plugin name
    PluginUnloaded(String),

    /// Emitted when a plugin is updated.
    /// **Params**:
    /// - 1 - String - Plugin name
    /// - 2 - String - Plugin Version
    /// - 3 - String - Old Plugin Version
    PluginUpdated(String, String, String),

    /// Emitted when a plugin runs into an error.
    /// **Params**:
    /// - 1 - String - Plugin name
    /// - 2 - String - Error Message
    PluginError(String, String),


    /*
        Reactivity Events
    */

    /// Emitted when the content of a form element changes.
    /// Contains the ID of the form field which was changed, as well as the new content
    FormContentChanged(String, String),


    // ActionPerformed(text_editor::Action),
    // ThemeSelected(highlighter::Theme),
    // WordWrapToggled(bool),
    // NewFile,
    // OpenFile,
    // FileOpened(Result<(PathBuf, Arc<String>), Error>),
    // SaveFile,
    // FileSaved(Result<PathBuf, Error>),
}

#[derive(Debug, Clone)]
pub enum Error {
    DialogClosed,
    IoError(io::ErrorKind),
}


pub struct EventQueue {
    id: String,
    queue: Vec<Message>,
}

impl EventQueue {
    pub fn new() -> EventQueue {
        EventQueue {
            id: nanoid!(10),
            queue: Vec::new()
        }
    }

    pub fn add(&mut self, msg: Message) {
        debug!("Adding event to queue ({})", self.id);
        self.queue.push(msg);
    }

    pub fn drain(&mut self, noot: &mut Noot) -> Task<Message> {
        debug!("Draining event queue ({})", self.id);

        let mut tasks: Vec<Task<Message>> = Vec::new();

        for event in self.queue.drain(..) {
            debug!("Draining event ({}): {:?}", self.id, event);
            tasks.push(noot.update(event))
        }

        debug!("Event queue drained ({})", self.id);
        Task::batch(tasks)
    }
}


pub struct ThreadPoolEvent<T> {
    id: String,
    kind: ThreadPoolMessage,
    fulfilled: bool,
    channel: tokio::sync::mpsc::Sender<T>,
}

impl <T> ThreadPoolEvent<T> {
    // TODO! Implement threadpool event promise structure
}


pub enum ThreadPoolMessage {
    Kill
}