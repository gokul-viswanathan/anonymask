use thiserror::Error;

#[derive(Error, Debug)]
pub enum AnonymaskError {
    #[error("Invalid entity type: {0}")]
    InvalidEntityType(String),
    #[error("Mapping not found for ID: {0}")]
    MappingNotFound(String),
    #[error("Regex compilation error: {0}")]
    RegexError(#[from] regex::Error),
    #[error("Storage error: {0}")]
    StorageError(String),
    #[error("Anonymization failed: {0}")]
    AnonymizationError(String),
}