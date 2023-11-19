use std::sync::Arc;

mod error;
mod tarpc;

pub struct PubSub {
    arc: Arc<PubSubArc>,
}

pub struct PubSubArc {
    publisher: tarpc::PublisherClient,
}

impl PubSub {
    fn new() -> Self {
        todo!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
