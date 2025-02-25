use std::thread;
use std::sync::mpsc::{channel, Sender, Receiver};
use std::sync::{Mutex, Arc};
use discord_rich_presence::{DiscordIpcClient, DiscordIpc, activity::*};
use serde;
use serde_derive::Deserialize;
use lazy_static::lazy_static;
use chrono::{Utc, DateTime};

#[derive(Debug, Clone)]
pub enum RpcCommand {
    Start,
    Stop,
    Set(WorkspaceForStatus),
    Report
}

lazy_static! {
    static ref CHANNEL: (Sender<RpcCommand>, Arc<Mutex<Receiver<RpcCommand>>>) = {
        let (tx, rx) = channel();
        (tx, Arc::new(Mutex::new(rx)))
    };
    static ref RUNNING: Mutex<bool> = Mutex::new(false);
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct WorkspaceForStatus {
    pub name: String,
    pub path: String,
    pub last_change: DateTime<Utc>,
    pub configuration: WorkspaceConfig,
    pub editors: Vec<EditorFile>
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct WorkspaceConfig {
    pub rpc: WorkspaceRpcConfig
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct WorkspaceRpcConfig {
    pub enable: bool,
    pub show_file_in_status: bool,
    pub show_time_in_status: bool,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum EditorKind {
    File(EditorFile),
    Split(Vec<EditorKind>)
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EditorFile {
    pub file: String,
    pub name: String,
    pub opened: DateTime<Utc>,
    pub changed: DateTime<Utc>,
    pub has_pending_changes: bool,
    pub content: String
}

pub fn workspace_to_status<'a>(ws: WorkspaceForStatus, client: &mut DiscordIpcClient) -> () {
    let mut a = Activity::new().activity_type(ActivityType::Playing);
    let mut state = String::from("Idling");
    let mut details = String::from("No files open");
    




    if ws.configuration.rpc.show_file_in_status && ws.editors.len() > 0 {
        details = format!("{} | {}", ws.editors[0].name, ws.name);
    }


    a = a.state(state.as_str())
        .details(details.as_str())
        .timestamps(
            Timestamps::new()
                .start(ws.last_change.to_utc().timestamp())
            );

    client.set_activity(a).unwrap()
}


#[tauri::command]
pub fn start_rich_presence() {
    println!("Sending RPC start command");

    dbg!(CHANNEL.0.send(RpcCommand::Start));
    println!("RPC start command sent");
}

#[tauri::command]
pub fn set_rich_presence_activity(ws: String) {

    println!("Received payload: {:?}", &ws);

    let result = serde_json::from_str(&ws);

    if result.is_ok() {
        println!("\x1b42mWORKSPACE PARSE WAS GOOD - NO PANIC\x1b[0m");
        CHANNEL.0.send(RpcCommand::Set(result.unwrap())).unwrap();
    } else {
        println!("{:?}",result.unwrap_err());
    }
}

pub fn start_rpc_thread() {
    println!("Spawning RPC thread");

    let receiver = CHANNEL.1.clone();
    let sender = CHANNEL.0.clone();

    thread::spawn(move || {
        println!("Starting RPC thread");
        
        let mut client = DiscordIpcClient::new("1343225099834101810").unwrap();
        
        let mut bl = false;

        sender.send(RpcCommand::Report).unwrap();

        while !bl {
            let guard = receiver.lock();
            let try_rx = guard.unwrap().try_recv();

            // If there is nothing on the receiver, 
            if try_rx.is_err() {
                continue;
            }

            let received = try_rx.unwrap();

            match received {
                RpcCommand::Start => {
                    println!("Attempting to connect to discord IPC");
                    let conres = client.connect();

                    if conres.is_ok() {
                        println!("Connected successfully");
                    } else {
                        let err = conres.unwrap_err();
                        println!("Failed to connect to IPC with following error:");
                        println!("{:?}", err);
                    }
                },
                RpcCommand::Stop => {
                    println!("Clearing activity and shutting down discord IPC connection");
                    client.clear_activity().unwrap();
                    client.close().unwrap();
                    bl = true
                },
                RpcCommand::Set(ws) => {
                    println!("Setting activity to below struct");
                    dbg!(&ws);
                    let activity = workspace_to_status(ws, &mut client);
                }
                _ => {
                    println!("Unrecognized RPC request");
                    dbg!(received);
                }
            }
        }
    });
}


pub fn shutdown_rpc() {
    CHANNEL.0.send(RpcCommand::Stop).unwrap();
}




