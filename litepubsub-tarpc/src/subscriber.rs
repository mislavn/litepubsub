#[tarpc::service]
trait Subscriber {
    /// Returns a greeting for name.
    async fn recv(path: String, message: String);
}
