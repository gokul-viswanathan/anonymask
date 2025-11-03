pub mod entity;
pub mod detection;
pub mod anonymizer;
pub mod error;

pub use anonymizer::Anonymizer;
pub use entity::{AnonymizationResult, Entity, EntityType};
pub use error::AnonymaskError;