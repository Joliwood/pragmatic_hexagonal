#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Custom error: {0}")]
    Custom(String),
}

pub type Result<T> = std::result::Result<T, Error>;
