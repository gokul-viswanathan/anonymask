use crate::detection::EntityDetector;
use crate::entity::{AnonymizationResult, EntityType};
use crate::error::AnonymaskError;
use std::collections::HashMap;
use uuid::Uuid;

pub struct Anonymizer {
    detector: EntityDetector,
}

impl Anonymizer {
    pub fn new(entity_types: Vec<EntityType>) -> Result<Self, AnonymaskError> {
        let detector = EntityDetector::new(&entity_types)?;

        Ok(Anonymizer {
            detector,
        })
    }

    pub fn anonymize(&self, text: &str) -> Result<AnonymizationResult, AnonymaskError> {
        if text.is_empty() {
            return Ok(AnonymizationResult {
                anonymized_text: String::new(),
                mapping: HashMap::new(),
                entities: Vec::new(),
            });
        }

        let entities = self.detector.detect(text);

        let mut placeholder_to_original = HashMap::new();
        let mut anonymized_text = text.to_string();

        // Collect unique values and generate placeholders
        let mut unique_values = HashMap::new();
        for entity in &entities {
            if !unique_values.contains_key(&entity.value) {
                let placeholder = self.generate_placeholder(&entity.entity_type, &entity.value);
                unique_values.insert(entity.value.clone(), placeholder);
            }
        }

        // Build placeholder to original mapping
        for (original, placeholder) in &unique_values {
            placeholder_to_original.insert(placeholder.clone(), original.clone());
        }

        // Replace in text
        for (original, placeholder) in &unique_values {
            anonymized_text = anonymized_text.replace(original, placeholder);
        }

        Ok(AnonymizationResult {
            anonymized_text,
            mapping: placeholder_to_original,
            entities,
        })
    }

    pub fn deanonymize(&self, text: &str, mapping: &HashMap<String, String>) -> String {
        let mut deanonymized_text = text.to_string();

        // Sort placeholders by length descending to avoid partial replacements
        let mut placeholders: Vec<_> = mapping.keys().collect();
        placeholders.sort_by_key(|p| std::cmp::Reverse(p.len()));

        for placeholder in placeholders {
            if let Some(original) = mapping.get(placeholder) {
                deanonymized_text = deanonymized_text.replace(placeholder, original);
            }
        }

        deanonymized_text
    }

    fn generate_placeholder(&self, entity_type: &EntityType, _value: &str) -> String {
        let type_prefix = match entity_type {
            EntityType::Email => "EMAIL",
            EntityType::Phone => "PHONE",
            EntityType::Ssn => "SSN",
            EntityType::CreditCard => "CREDIT_CARD",
            EntityType::IpAddress => "IP_ADDRESS",
            EntityType::Url => "URL",
        };
        format!("{}_{}", type_prefix, Uuid::new_v4().simple())
    }
}