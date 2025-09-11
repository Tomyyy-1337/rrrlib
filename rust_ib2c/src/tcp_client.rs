use std::{net::{IpAddr, Ipv4Addr}, sync::mpsc};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TCP_Data {
    source_id: u32,
    data: String,
}

pub struct TCP_Client {
    address: IpAddr,
    port: u16,
    sender: mpsc::Sender<serde_json::Value>,
    receiver: mpsc::Receiver<serde_json::Value>,
}

impl TCP_Client {
    pub fn new() -> Self {
        let (tx, rx) = std::sync::mpsc::channel();
        Self {
            address: IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
            port: 8080,
            sender: tx,
            receiver: rx,
        }
    }

    pub fn get_sender(&self) -> mpsc::Sender<serde_json::Value> {
        self.sender.clone()
    }
}
