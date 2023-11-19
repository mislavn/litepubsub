#[tarpc::service]
trait PublisherService {
    /// Returns a greeting for name.
    async fn send(path: String, message: String);
}

#[derive(Clone)]
struct Publisher {}
