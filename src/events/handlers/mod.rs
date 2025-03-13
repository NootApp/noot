use iced::Task;
use crate::{events, Noot};
use crate::events::types::Message;
use crate::events::types::Message::*;


pub mod thread_pool;
pub mod config;
pub mod workspace;
pub mod rpc;

pub(crate) fn core(noot: &mut Noot, message: Message) -> Task<Message> {
    match message {
        ConfigLoaded(cfg) => config::on_load(noot, cfg),

        WorkspaceIngestManifests => workspace::on_ingest(noot),
        WorkspaceLoadStart => workspace::on_load_start(noot),
        WorkspaceLoadResult(outcome) => workspace::on_load(noot, outcome),

        

        RPCConnected => rpc::on_connect(),
        RPCDisconnected => rpc::on_disconnect(),
        RPCModified => rpc::on_change(),
        RPCInit => rpc::on_init(noot),
        _ => {
            warn!("Received an unknown message payload");
            dbg!(message);
            Task::none()
        }
    }
}