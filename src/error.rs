#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("No relays specified, at least one relay is required!")]
    NoRelay,
}
