#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Custom inventory error: {0}")]
    CustomInventory(String),

    #[error("MongoDB error: {0}")]
    MongoDB(#[from] mongodb::error::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
