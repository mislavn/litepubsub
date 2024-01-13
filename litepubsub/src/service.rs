use std::sync::Arc;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Packet {
    path: String,
    message: String,
}

#[tarpc::service]
pub trait Publisher {
    async fn send(packet: Arc<Packet>);
}

#[tarpc::service]
pub trait Receiver {
    async fn recv(packet: Arc<Packet>);
}
