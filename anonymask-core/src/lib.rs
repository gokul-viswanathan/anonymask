pub mod anonymizer;
pub mod detection;
pub mod entity;
pub mod error;

pub use anonymizer::Anonymizer;
pub use entity::{AnonymizationResult, Entity, EntityType};
pub use error::AnonymaskError;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_anonymize_email() {
        let anonymizer = Anonymizer::new(vec![EntityType::Email]).unwrap();
        let result = anonymizer.anonymize("Contact john@email.com").unwrap();
        assert!(result.anonymized_text.contains("EMAIL_"));
        assert_eq!(result.entities.len(), 1);
        assert_eq!(result.entities[0].entity_type, EntityType::Email);
    }

    #[test]
    fn test_anonymize_phone() {
        let anonymizer = Anonymizer::new(vec![EntityType::Phone]).unwrap();
        let result = anonymizer.anonymize("Call 555-123-4567").unwrap();
        assert!(result.anonymized_text.contains("PHONE_"));
        assert_eq!(result.entities.len(), 1);
    }

    #[test]
    fn test_anonymize_multiple_entities() {
        let anonymizer = Anonymizer::new(vec![EntityType::Email, EntityType::Phone]).unwrap();
        let result = anonymizer
            .anonymize("Contact john@email.com or call 555-123-4567")
            .unwrap();
        assert!(result.anonymized_text.contains("EMAIL_"));
        assert!(result.anonymized_text.contains("PHONE_"));
        assert_eq!(result.entities.len(), 2);
    }

    #[test]
    fn test_anonymize_duplicate_entities() {
        let anonymizer = Anonymizer::new(vec![EntityType::Email]).unwrap();
        let result = anonymizer
            .anonymize("Email john@email.com and jane@email.com")
            .unwrap();
        assert!(result.anonymized_text.contains("EMAIL_"));
        // Should have same placeholder for same email
        let parts: Vec<&str> = result.anonymized_text.split("EMAIL_").collect();
        assert_eq!(parts.len(), 3); // "Email ", "xxx and ", "xxx"
    }

    #[test]
    fn test_deanonymize() {
        let anonymizer = Anonymizer::new(vec![EntityType::Email]).unwrap();
        let original = "Contact john@email.com";
        let result = anonymizer.anonymize(original).unwrap();
        let deanonymized = anonymizer.deanonymize(&result.anonymized_text, &result.mapping);
        assert_eq!(deanonymized, original);
    }

    #[test]
    fn test_anonymize_empty_string() {
        let anonymizer = Anonymizer::new(vec![EntityType::Email]).unwrap();
        let result = anonymizer.anonymize("").unwrap();
        assert_eq!(result.anonymized_text, "");
        assert!(result.entities.is_empty());
    }

    #[test]
    fn test_invalid_entity_type() {
        let result = EntityType::from_str("invalid");
        assert!(result.is_err());
    }
}

