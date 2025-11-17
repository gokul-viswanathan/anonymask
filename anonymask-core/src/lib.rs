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
    fn test_anonymize_phone_short_format() {
        let anonymizer = Anonymizer::new(vec![EntityType::Phone]).unwrap();
        let result = anonymizer.anonymize("Call 555-123").unwrap();
        assert!(result.anonymized_text.contains("PHONE_"));
        assert_eq!(result.entities.len(), 1);
    }

    #[test]
    fn test_anonymize_phone_multiple_formats() {
        let anonymizer = Anonymizer::new(vec![EntityType::Phone]).unwrap();
        let result = anonymizer.anonymize("Call 555-123-4567 or 555-123").unwrap();
        assert!(result.anonymized_text.contains("PHONE_"));
        assert_eq!(result.entities.len(), 2);
    }

    #[test]
    fn test_anonymize_phone_with_dots() {
        let anonymizer = Anonymizer::new(vec![EntityType::Phone]).unwrap();
        let result = anonymizer.anonymize("Call 555.123.4567 or 555.123").unwrap();
        assert!(result.anonymized_text.contains("PHONE_"));
        assert_eq!(result.entities.len(), 2);
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

    #[test]
    fn test_anonymize_with_custom_entities() {
        let anonymizer = Anonymizer::new(vec![EntityType::Email]).unwrap();
        let mut custom_entities = std::collections::HashMap::new();
        custom_entities.insert(EntityType::Phone, vec!["555-123-4567".to_string()]);
        
        let result = anonymizer
            .anonymize_with_custom("Contact john@email.com or call 555-123-4567", Some(&custom_entities))
            .unwrap();
        
        assert!(result.anonymized_text.contains("EMAIL_"));
        assert!(result.anonymized_text.contains("PHONE_"));
        assert_eq!(result.entities.len(), 2);
    }

    #[test]
    fn test_anonymize_custom_entities_only() {
        let anonymizer = Anonymizer::new(vec![]).unwrap();
        let mut custom_entities = std::collections::HashMap::new();
        custom_entities.insert(EntityType::Email, vec!["custom@example.com".to_string()]);
        
        let result = anonymizer
            .anonymize_with_custom("Send to custom@example.com", Some(&custom_entities))
            .unwrap();
        
        assert!(result.anonymized_text.contains("EMAIL_"));
        assert_eq!(result.entities.len(), 1);
        assert_eq!(result.entities[0].value, "custom@example.com");
    }

    #[test]
    fn test_anonymize_duplicate_custom_entities() {
        let anonymizer = Anonymizer::new(vec![]).unwrap();
        let mut custom_entities = std::collections::HashMap::new();
        custom_entities.insert(EntityType::Email, vec!["test@example.com".to_string()]);
        
        let result = anonymizer
            .anonymize_with_custom("Email test@example.com and test@example.com", Some(&custom_entities))
            .unwrap();
        
        assert!(result.anonymized_text.contains("EMAIL_"));
        // Should have same placeholder for same email
        let parts: Vec<&str> = result.anonymized_text.split("EMAIL_").collect();
        assert_eq!(parts.len(), 3); // "Email ", "xxx and ", "xxx"
    }

    #[test]
    fn test_backward_compatibility() {
        let anonymizer = Anonymizer::new(vec![EntityType::Email]).unwrap();
        let result1 = anonymizer.anonymize("Contact john@email.com").unwrap();
        let result2 = anonymizer.anonymize_with_custom("Contact john@email.com", None).unwrap();
        
        // Check that both results have the same structure
        assert!(result1.anonymized_text.contains("EMAIL_"));
        assert!(result2.anonymized_text.contains("EMAIL_"));
        assert_eq!(result1.entities.len(), result2.entities.len());
        assert_eq!(result1.mapping.len(), result2.mapping.len());
        assert_eq!(result1.entities[0].entity_type, result2.entities[0].entity_type);
        assert_eq!(result1.entities[0].value, result2.entities[0].value);
    }
}

