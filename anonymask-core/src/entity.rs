use serde::{Deserialize, Serialize};
use crate::error::AnonymaskError;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EntityType {
    Email,
    Phone,
    Ssn,
    CreditCard,
    IpAddress,
    Url,
    // NER types to be added later
    // Person,
    // Org,
    // Location,
}

impl EntityType {
    pub fn from_str(s: &str) -> Result<Self, AnonymaskError> {
        match s.to_lowercase().as_str() {
            "email" => Ok(EntityType::Email),
            "phone" => Ok(EntityType::Phone),
            "ssn" => Ok(EntityType::Ssn),
            "credit_card" => Ok(EntityType::CreditCard),
            "ip_address" => Ok(EntityType::IpAddress),
            "url" => Ok(EntityType::Url),
            _ => Err(AnonymaskError::InvalidEntityType(s.to_string())),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entity {
    pub entity_type: EntityType,
    pub value: String,
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnonymizationResult {
    pub anonymized_text: String,
    pub mapping: std::collections::HashMap<String, String>, // placeholder -> original
    pub entities: Vec<Entity>,
}