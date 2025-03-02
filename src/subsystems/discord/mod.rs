use discord_rich_presence::{DiscordIpc, DiscordIpcClient};
use iced::Task;
use lazy_static::lazy_static;
use crate::events::types::Message;
use log::{info, debug, trace, warn, error};
use std::sync::Mutex;
use discord_rich_presence::activity::{Activity, ActivityType};
use iced::widget::text::Rich;
use crate::subsystems::discord::config::RichPresenceConfig;

pub mod config;


#[derive(Debug)]
pub struct RichPresence {
    client: DiscordIpcClient,
    inner_config: Mutex<RichPresenceConfig>,
    workspace_config: Mutex<RichPresenceConfig>,
}


lazy_static!(
    pub static ref RPC_CLIENT: Mutex<RichPresence> = Mutex::new(RichPresence::new());
);


impl RichPresence {
    pub fn new() -> RichPresence {
        RichPresence {
            client: DiscordIpcClient::new("1343225099834101810").unwrap(),
            inner_config: Mutex::new(RichPresenceConfig {
                enable: false,
                client_id: None,
                enable_idle: false,
                show_current_workspace: false,
            }),
            workspace_config: Mutex::new(RichPresenceConfig {
                enable: false,
                client_id: None,
                enable_idle: false,
                show_current_workspace: false,
            }),
        }
    }

    pub fn connect(&mut self) {
        info!("Connecting to Discord");
        let con_res = self.client.connect();
        if con_res.is_err() {
            error!("Failed to connect to Discord");
            error!("{:?}", con_res.err().unwrap());
        } else {
            info!("Connected to Discord");
            self.client.set_activity(
                Activity::new()
                    .activity_type(ActivityType::Playing)
                    .state("Idling")
            ).unwrap();
        }
    }

    pub fn disconnect(&mut self) {
        info!("Disconnecting");
        let close_res = self.client.close();
        if close_res.is_err() {
            error!("Failed to disconnect");
            error!("{:?}", close_res.err().unwrap());
        } else {
            info!("Disconnected from Discord");
        }
    }
}



