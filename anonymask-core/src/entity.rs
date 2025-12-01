use serde::{Deserialize, Serialize};
use crate::error::AnonymaskError;

/// Type of personally identifiable information (PII) entity.
///
/// Defines both built-in entity types with regex-based detection
/// and custom entity types for user-defined patterns.
///
/// # Examples
///
/// ```
/// use anonymask_core::entity::EntityType;
///
/// // Built-in types
/// let email_type = EntityType::Email;
/// let phone_type = EntityType::Phone;
///
/// // Custom types
/// let name_type = EntityType::Custom("name".to_string());
/// let company_type = EntityType::Custom("company".to_string());
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EntityType {
    /// Email addresses (user@domain.com)
    Email,
    /// Phone numbers (555-123-4567, (555) 123-4567, etc.)
    Phone,
    /// Social Security Numbers (123-45-6789)
    Ssn,
    /// Credit card numbers (1234-5678-9012-3456)
    CreditCard,
    /// IPv4 addresses (192.168.1.1)
    IpAddress,
    /// URLs (https://example.com)
    Url,
    /// Custom user-defined entity types
    ///
    /// Used for domain-specific entities like names, companies, etc.
    /// Custom entities use substring matching rather than regex.
    Custom(String),
    // NER types to be added later
    // Person,
    // Org,
    // Location,
}

impl EntityType {
    /// Parse an entity type from a string.
    ///
    /// Built-in types are case-insensitive. Any unrecognized string
    /// is treated as a custom entity type.
    ///
    /// # Arguments
    ///
    /// * `s` - The entity type string to parse
    ///
    /// # Returns
    ///
    /// Always returns `Ok` with either a built-in or custom entity type.
    ///
    /// # Examples
    ///
    /// ```
    /// use anonymask_core::entity::EntityType;
    ///
    /// let email = EntityType::from_str("email").unwrap();
    /// assert_eq!(email, EntityType::Email);
    ///
    /// let custom = EntityType::from_str("company").unwrap();
    /// assert_eq!(custom, EntityType::Custom("company".to_string()));
    /// ```
    pub fn from_str(s: &str) -> Result<Self, AnonymaskError> {
        match s.to_lowercase().as_str() {
            "email" => Ok(EntityType::Email),
            "phone" => Ok(EntityType::Phone),
            "ssn" => Ok(EntityType::Ssn),
            "credit_card" => Ok(EntityType::CreditCard),
            "ip_address" => Ok(EntityType::IpAddress),
            "url" => Ok(EntityType::Url),
            _ => Ok(EntityType::Custom(s.to_string())),
        }
    }
}

/// A detected PII entity in text with its location.
///
/// Contains the entity type, value, and position information
/// for a piece of PII found in the input text.
///
/// # Fields
///
/// * `entity_type` - The type of PII detected
/// * `value` - The actual PII value found
/// * `start` - Starting byte index in the original text
/// * `end` - Ending byte index in the original text
///
/// # Examples
///
/// ```
/// use anonymask_core::entity::{Entity, EntityType};
///
/// let entity = Entity {
///     entity_type: EntityType::Email,
///     value: "user@example.com".to_string(),
///     start: 0,
///     end: 16,
/// };
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entity {
    /// The type of entity detected
    pub entity_type: EntityType,
    /// The actual value of the PII
    pub value: String,
    /// Starting position in the text (byte index)
    pub start: usize,
    /// Ending position in the text (byte index)
    pub end: usize,
}

/// Result of an anonymization operation.
///
/// Contains the anonymized text, the mapping to restore original values,
/// and metadata about detected entities.
///
/// # Fields
///
/// * `anonymized_text` - Text with PII replaced by placeholders
/// * `mapping` - HashMap mapping placeholders back to original values
/// * `entities` - List of all detected entities with positions
///
/// # Examples
///
/// ```
/// use anonymask_core::Anonymizer;
/// use anonymask_core::entity::EntityType;
///
/// let anonymizer = Anonymizer::new(vec![EntityType::Email]).unwrap();
/// let result = anonymizer.anonymize("Contact user@example.com").unwrap();
///
/// println!("Anonymized: {}", result.anonymized_text);
/// println!("Found {} entities", result.entities.len());
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnonymizationResult {
    /// Text with all detected PII replaced by placeholders
    pub anonymized_text: String,
    /// Mapping from placeholder to original value
    ///
    /// Used to restore original values during deanonymization.
    /// Keys are placeholders (e.g., "EMAIL_abc123"), values are original PII.
    pub mapping: std::collections::HashMap<String, String>,
    /// All entities detected in the original text
    ///
    /// Includes entity type, value, and position information.
    pub entities: Vec<Entity>,
}