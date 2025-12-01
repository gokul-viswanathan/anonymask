pub mod anonymizer;
pub mod config;
pub mod detection;
pub mod entity;
pub mod error;

pub use anonymizer::Anonymizer;
pub use config::{AnonymizerConfig, AnonymizerConfigBuilder, PlaceholderFormat};
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
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), EntityType::Custom("invalid".to_string()));
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

    #[test]
    fn test_custom_entity_type() {
        let anonymizer = Anonymizer::new(vec![]).unwrap();
        let mut custom_entities = std::collections::HashMap::new();
        custom_entities.insert(EntityType::Custom("name".to_string()), vec!["John Doe".to_string()]);

        let result = anonymizer
            .anonymize_with_custom("My name is John Doe", Some(&custom_entities))
            .unwrap();

        assert!(result.anonymized_text.contains("NAME_"));
        assert_eq!(result.entities.len(), 1);
        assert_eq!(result.entities[0].entity_type, EntityType::Custom("name".to_string()));
        assert_eq!(result.entities[0].value, "John Doe");
    }

    // Property-based tests for regression prevention
    #[cfg(test)]
    mod property_tests {
        use super::*;
        use proptest::prelude::*;

        proptest! {
            /// Property: anonymize then deanonymize is identity function
            ///
            /// For any arbitrary text, if we anonymize it and then deanonymize it,
            /// we should get back the original text exactly.
            #[test]
            fn prop_anonymize_deanonymize_is_identity(text in "\\PC{0,1000}") {
                let anonymizer = Anonymizer::new(vec![
                    EntityType::Email,
                    EntityType::Phone,
                    EntityType::Url,
                ]).unwrap();

                let result = anonymizer.anonymize(&text).unwrap();
                let restored = anonymizer.deanonymize(&result.anonymized_text, &result.mapping);

                prop_assert_eq!(text, restored);
            }

            /// Property: no PII leakage in anonymized text
            ///
            /// When we detect and anonymize an email, the original email
            /// should not appear anywhere in the anonymized output.
            #[test]
            fn prop_no_email_leakage(
                local in "[a-z]{5,15}",
                domain in "[a-z]{5,15}",
                tld in "(com|org|net|io|dev)"
            ) {
                let email = format!("{}@{}.{}", local, domain, tld);
                let text = format!("Contact me at {}", email);

                let anonymizer = Anonymizer::new(vec![EntityType::Email]).unwrap();
                let result = anonymizer.anonymize(&text).unwrap();

                // Original email should not appear in anonymized text
                prop_assert!(!result.anonymized_text.contains(&email));

                // But should be in the mapping values
                prop_assert!(result.mapping.values().any(|v| v == &email));
            }

            /// Property: empty input produces empty output
            #[test]
            fn prop_empty_input_empty_output(_entity_types in prop::collection::vec(
                prop::sample::select(vec![
                    EntityType::Email,
                    EntityType::Phone,
                    EntityType::Ssn,
                ]),
                0..5
            )) {
                let anonymizer = Anonymizer::new(_entity_types).unwrap();
                let result = anonymizer.anonymize("").unwrap();

                prop_assert_eq!(result.anonymized_text, "");
                prop_assert!(result.entities.is_empty());
                prop_assert!(result.mapping.is_empty());
            }

            /// Property: text without PII remains unchanged
            #[test]
            fn prop_no_pii_no_change(text in "[A-Za-z\\s]{10,100}") {
                // Generate text that definitely doesn't contain PII patterns
                let safe_text = text.replace("@", "").replace(".", "").replace("-", "");

                let anonymizer = Anonymizer::new(vec![
                    EntityType::Email,
                    EntityType::Phone,
                ]).unwrap();
                let result = anonymizer.anonymize(&safe_text).unwrap();

                // If no entities detected, text should be unchanged
                if result.entities.is_empty() {
                    prop_assert_eq!(result.anonymized_text, safe_text);
                }
            }

            /// Property: duplicate values get same placeholder
            #[test]
            fn prop_duplicate_values_same_placeholder(
                email in "[a-z]{5}@[a-z]{5}\\.com"
            ) {
                let text = format!("Email {} and {}", email, email);

                let anonymizer = Anonymizer::new(vec![EntityType::Email]).unwrap();
                let result = anonymizer.anonymize(&text).unwrap();

                // Should only have one unique placeholder for duplicate emails
                prop_assert_eq!(result.mapping.len(), 1);
            }

            /// Property: all detected entities are in range
            #[test]
            fn prop_entity_positions_valid(text in "\\PC{10,500}") {
                let anonymizer = Anonymizer::new(vec![
                    EntityType::Email,
                    EntityType::Phone,
                    EntityType::Url,
                ]).unwrap();
                let result = anonymizer.anonymize(&text).unwrap();

                for entity in result.entities {
                    // Start must be before end
                    prop_assert!(entity.start < entity.end);
                    // End must be within text bounds
                    prop_assert!(entity.end <= text.len());
                    // Extracted value must match
                    prop_assert_eq!(&text[entity.start..entity.end], entity.value);
                }
            }

            /// Property: mapping is bijective (one-to-one)
            #[test]
            fn prop_mapping_bijective(text in "\\PC{10,500}") {
                let anonymizer = Anonymizer::new(vec![
                    EntityType::Email,
                    EntityType::Phone,
                ]).unwrap();
                let result = anonymizer.anonymize(&text).unwrap();

                // Each placeholder should map to exactly one value
                let placeholder_count = result.mapping.len();
                let unique_values: std::collections::HashSet<_> = result.mapping.values().collect();

                prop_assert_eq!(placeholder_count, unique_values.len());
            }

            /// Property: valid IPv4 addresses are detected
            #[test]
            fn prop_valid_ipv4_detected(
                a in 0u8..=255,
                b in 0u8..=255,
                c in 0u8..=255,
                d in 0u8..=255
            ) {
                let ip = format!("{}.{}.{}.{}", a, b, c, d);
                let text = format!("Server IP is {}", ip);

                let anonymizer = Anonymizer::new(vec![EntityType::IpAddress]).unwrap();
                let result = anonymizer.anonymize(&text).unwrap();

                // Should detect exactly one IP address
                prop_assert_eq!(result.entities.len(), 1);
                prop_assert_eq!(&result.entities[0].value, &ip);
            }

            /// Property: invalid IPv4 addresses (octets > 255) are not detected
            #[test]
            fn prop_invalid_ipv4_not_detected(invalid_octet in 256u32..=999) {
                let ip = format!("{}.1.1.1", invalid_octet);
                let text = format!("Invalid IP: {}", ip);

                let anonymizer = Anonymizer::new(vec![EntityType::IpAddress]).unwrap();
                let result = anonymizer.anonymize(&text).unwrap();

                // Should not detect invalid IP
                prop_assert_eq!(result.entities.len(), 0);
            }

            /// Property: phone numbers with parentheses are detected
            #[test]
            fn prop_phone_with_parens_detected(
                area in 200u16..=999,
                exchange in 200u16..=999,
                line in 1000u16..=9999
            ) {
                let phone = format!("({}) {}-{}", area, exchange, line);
                let text = format!("Call {}", phone);

                let anonymizer = Anonymizer::new(vec![EntityType::Phone]).unwrap();
                let result = anonymizer.anonymize(&text).unwrap();

                // Should detect the phone number
                prop_assert!(result.entities.len() >= 1);
                prop_assert!(result.entities.iter().any(|e| e.entity_type == EntityType::Phone));
            }
        }
    }
}

