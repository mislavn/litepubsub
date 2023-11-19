use std::sync::Arc;

#[tarpc::service]
pub trait Publisher {
    async fn send(path: Arc<String>, message: Arc<String>);
}

#[tarpc::service]
pub trait Receiver {
    async fn recv(path: Arc<String>, message: Arc<String>);
}
