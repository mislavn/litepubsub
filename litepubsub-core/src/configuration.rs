use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct LitePubSubConfiguration {
    channel_size: Option<usize>,
    retry_count: usize,
    tarpc_port: u16,
    capnp_port: u16,
}
