#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Custom inventory error: {0}")]
    CustomInventory(String),
}

pub type Result<T> = std::result::Result<T, Error>;
