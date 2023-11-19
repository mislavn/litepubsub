#[derive(Debug, thiserror::Error)]

pub enum PubSubError {
    #[error("Tarpc transport error")]
    Tarpc(#[from] tarpc::client::RpcError),
}
