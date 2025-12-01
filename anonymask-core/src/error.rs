use thiserror::Error;

/// Errors that can occur during anonymization operations.
///
/// All errors provide detailed context to help diagnose issues quickly.
#[derive(Error, Debug)]
pub enum AnonymaskError {
    /// Invalid entity type was provided
    ///
    /// # Examples
    /// - Using a custom entity type with regex pattern detection
    /// - Providing an unsupported entity type name
    #[error("Invalid entity type '{entity_type}': {reason}")]
    InvalidEntityType {
        /// The entity type that was invalid
        entity_type: String,
        /// Explanation of why it's invalid
        reason: String,
    },

    /// Mapping not found during deanonymization
    ///
    /// This occurs when trying to deanonymize text with a placeholder
    /// that doesn't exist in the provided mapping.
    #[error("Mapping not found for placeholder '{placeholder}' at position {position}")]
    MappingNotFound {
        /// The placeholder that couldn't be found
        placeholder: String,
        /// Position in the text where it occurred
        position: usize,
    },

    /// Regex compilation or execution error
    ///
    /// This indicates an issue with the pattern matching system.
    #[error("Regex error for pattern '{pattern}': {source}")]
    RegexError {
        /// The regex pattern that failed
        pattern: String,
        /// The underlying regex error
        #[source]
        source: regex::Error,
    },

    /// Storage backend error
    ///
    /// Occurs when interacting with persistent storage for mappings.
    #[error("Storage error: {0}")]
    StorageError(String),

    /// General anonymization operation failure
    ///
    /// Used for unexpected errors during anonymization.
    #[error("Anonymization failed: {0}")]
    AnonymizationError(String),
}

impl AnonymaskError {
    /// Helper to suggest valid entity types when an invalid one is provided
    pub fn suggest_entity_type(invalid: &str) -> &'static str {
        let supported = [
            "email", "phone", "ssn", "credit_card", "ip_address", "url",
        ];

        let invalid_lower = invalid.to_lowercase();
        for supported_type in &supported {
            if supported_type.contains(&invalid_lower) || invalid_lower.contains(supported_type) {
                return supported_type;
            }
        }

        // Common typos
        match invalid_lower.as_str() {
            "mail" | "e-mail" => "email",
            "telephone" | "tel" | "mobile" => "phone",
            "social_security" | "social_security_number" => "ssn",
            "cc" | "card" | "credit" => "credit_card",
            "ip" | "ipaddress" | "ip_addr" => "ip_address",
            "link" | "uri" => "url",
            _ => "email",
        }
    }
}