use std::collections::BTreeMap;
use bincode::{Decode, Encode};
use interprocess::local_socket::prelude::*;
use crate::runtime::GLOBAL_STATE;

#[derive(Debug)]
pub enum RpcOperation {
    Ping = 1,
    Pong = 2,
}

pub const MAGIC: u32 = 0b01100111011100111001101100111111;
pub const VERSION: u16 = 1;


#[derive(Debug, Decode, Encode)]
pub struct IpcFrame {
    pub magic: u32,
    pub version: u32,
    pub from: u32,
    pub to: u32,
    pub operation: RpcOperation,
}

pub enum ConnectionState {
    Offline,
    Connected,
    Pending,
}

pub struct IPCManager {
    clients: BTreeMap<u32, ConnectionState>,
}

pub fn create_ipc_thread() {
    let state = GLOBAL_STATE.clone();
    tokio::spawn(async move {
        state.lock().unwrap().run_ipc = true;

        loop {

        }
    });
}