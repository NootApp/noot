use iced::Task;
use crate::events::types::{EventQueue, Message};
use crate::Noot;
use crate::subsystems::discord::RPC_CLIENT;

pub fn on_connect() -> Task<Message> {
    debug!("Rich Presence Client connected");
    Task::none()
}

pub fn on_disconnect() -> Task<Message> {
    debug!("Rich Presence Client disconnected");
    Task::none()
}

pub fn on_change() -> Task<Message> {
    debug!("Rich Presence Client updated status");
    Task::none()
}

pub fn on_init(noot: &mut Noot) -> Task<Message> {
    debug!("Rich Presence Client initializing");

    let mut queue = EventQueue::new();
    let mut client = RPC_CLIENT.lock().unwrap();
    let config = noot.config.clone();

    if config.is_none() {
        panic!("RPCInit triggered before config was loaded");
    }

    let cfg = config.unwrap();


    debug!("Checking RPC permissions");


    let rpc_config = cfg.rpc.unwrap_or_default();

    if rpc_config.is_enabled() {
        debug!("RPC is enabled in the config");
        client.connect(&rpc_config.client_id());
        queue.add(Message::RPCConnected);
    } else {
        debug!("RPC is not enabled in the config");
        client.disconnect();
        queue.add(Message::RPCDisconnected);
    }

    queue.drain(noot)
}