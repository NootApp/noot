use crate::subsystems::discord::config::RichPresenceConfig;
use discord_rich_presence::activity::{Activity, ActivityType};
use discord_rich_presence::{DiscordIpc, DiscordIpcClient};
use lazy_static::lazy_static;
use std::sync::Mutex;

pub mod config;

#[derive(Debug)]
pub struct RichPresence {
    connected: bool,
    client: DiscordIpcClient,
    inner_config: Mutex<RichPresenceConfig>,
    workspace_config: Mutex<RichPresenceConfig>,
}

lazy_static! {
    pub static ref RPC_CLIENT: Mutex<RichPresence> =
        Mutex::new(RichPresence::new());
}

impl RichPresence {
    pub fn new() -> RichPresence {
        RichPresence {
            connected: false,
            client: DiscordIpcClient::new("1343225099834101810").unwrap(),
            inner_config: Mutex::new(RichPresenceConfig::default()),
            workspace_config: Mutex::new(RichPresenceConfig::default()),
        }
    }

    pub fn connect(&mut self, client_id: &str) {
        if self.connected {
            return;
        }
        
        self.client = DiscordIpcClient::new(client_id).unwrap();
        
        info!("Connecting to Discord");
        let con_res = self.client.connect();
        if con_res.is_err() {
            error!("Failed to connect to Discord");
            error!("{:?}", con_res.err().unwrap());
        } else {
            info!("Connected to Discord");
            self.client
                .set_activity(
                    Activity::new()
                        .activity_type(ActivityType::Playing)
                        .state("Idling"),
                )
                .unwrap();
        }
    }

    pub fn disconnect(&mut self) {
        if !self.connected {
            return;
        }
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
