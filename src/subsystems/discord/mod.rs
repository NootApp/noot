use crate::subsystems::discord::config::RichPresenceConfig;
use discord_rich_presence::activity::{Activity, ActivityType};
use discord_rich_presence::{DiscordIpc, DiscordIpcClient};
use lazy_static::lazy_static;
use std::sync::Mutex;

pub mod config;

const MIN_WAIT_TIME: u128 = 2500;

#[derive(Debug)]
#[cfg(feature = "drpc")]
pub struct RichPresence {
    connected: bool,
    client: DiscordIpcClient,
    inner_config: Mutex<RichPresenceConfig>,
    workspace_config: Mutex<RichPresenceConfig>,
    last_change: std::time::Instant,
    has_presence: bool,
}

#[cfg(feature = "drpc")]
lazy_static! {
    pub static ref RPC_CLIENT: Mutex<RichPresence> =
        Mutex::new(RichPresence::new());
}

#[cfg(feature = "drpc")]
impl RichPresence {
    pub fn new() -> RichPresence {
        RichPresence {
            connected: false,
            client: DiscordIpcClient::new("1343225099834101810").unwrap(),
            inner_config: Mutex::new(RichPresenceConfig::default()),
            workspace_config: Mutex::new(RichPresenceConfig::default()),
            last_change: std::time::Instant::now(),
            has_presence: false,
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
            self.connected = true;
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

    pub fn set_activity(&mut self, activity: Activity) {
        if self.connected {
            if self.last_change.elapsed().as_millis() < MIN_WAIT_TIME && self.has_presence {
                warn!("Tried to update status too fast. discarding changes temporarily");
                warn!("(Last change was {}ms ago", self.last_change.elapsed().as_millis());
                return;
            }
            self.client.set_activity(activity).unwrap();
            self.has_presence = true;
        } else {
            error!("Failed to set activity");
            error!("Cannot set activity before logging in");
        }
    }
}
