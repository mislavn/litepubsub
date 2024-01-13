use std::{
    net::SocketAddr,
    ops::Deref,
    sync::{Arc, Mutex},
};

use async_rwlock::RwLock;
use futures::{future, prelude::*};
use tarpc::{
    client,
    context::Context,
    server::{self, incoming::Incoming, BaseChannel, Channel},
    transport::channel,
};

use crate::{
    error::PubSubError,
    service::{self, Publisher, PublisherClient, Receiver},
};

#[derive(Debug, Clone)]
pub struct PubSub {
    arc: Arc<PubSubArc>,
}

#[derive(Debug)]
pub struct PubSubArc {
    publishers: RwLock<Vec<service::PublisherClient>>,
    tcp: Option<SocketAddr>,
    pub channel_publisher: Mutex<Option<service::PublisherClient>>,
}
impl Deref for PubSub {
    type Target = PubSubArc;

    fn deref(&self) -> &Self::Target {
        &self.arc
    }
}

async fn spawn(fut: impl futures::Future<Output = ()> + Send + 'static) {
    tokio::spawn(fut);
}

#[derive(Debug, Default, Clone)]
pub struct PubSubBuilder {
    channel_size: Option<usize>,
    tcp: Option<SocketAddr>,
    uds: Option<String>,
}

impl PubSubBuilder {
    pub fn new() -> PubSubBuilder {
        PubSubBuilder::default()
    }

    pub fn channel(mut self, size: Option<usize>) -> PubSubBuilder {
        self.channel_size = size;
        self
    }

    pub fn tcp(mut self, address: SocketAddr) -> PubSubBuilder {
        self.tcp = Some(address);
        self
    }

    pub fn unix_socket(mut self, path: String) -> PubSubBuilder {
        self.uds = Some(path);
        self
    }

    pub async fn build(self) -> Result<PubSub, PubSubError> {
        let mut arc = PubSubArc {
            publishers: RwLock::new(vec![]),
            tcp: self.tcp.clone(),
            channel_publisher: Mutex::new(None),
        };

        let server = PubSub { arc: Arc::new(arc) };

        match self.channel_size {
            Some(size) => {}
            None => {
                let (tx, rx) = channel::unbounded();
                tokio::spawn(
                    BaseChannel::with_defaults(rx)
                        .execute(server.clone().serve())
                        .for_each(spawn),
                );
                let client = PublisherClient::new(client::Config::default(), tx).spawn();
            }
        }

        Ok(server)
    }
}

impl service::Publisher for PubSub {
    async fn send(self, ctx: Context, packet: Arc<service::Packet>) {
        tokio::spawn(async move {
            let mut failed_publishers = vec![];
            for (index, publisher) in self.publishers.read().await.iter().enumerate() {
                match publisher.send(ctx, packet.clone()).await {
                    Ok(_) => {}
                    Err(e) => {
                        failed_publishers.push(index);
                        tracing::error!("Failed to send packet with error {}", e.to_string());
                    }
                }
            }

            if failed_publishers.is_empty() {
                return;
            }

            let mut publishers = self.publishers.write().await;
            for index in failed_publishers.iter().rev() {
                publishers.remove(*index);
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use tokio;

    use super::*;

    #[tokio::test]
    async fn test_pubsub_tcp() {
        let pubsub = PubSubBuilder::new()
            .tcp("127.0.0.1:0".parse().unwrap())
            .build()
            .await
            .unwrap();
    }
}
