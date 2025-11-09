use crate::entity::{Entity, EntityType};
use crate::error::AnonymaskError;
use regex::Regex;
use std::collections::HashMap;

pub struct EntityDetector {
    patterns: HashMap<EntityType, Regex>,
}

impl EntityDetector {
    pub fn new(entity_types: &[EntityType]) -> Result<Self, AnonymaskError> {
        let mut patterns = HashMap::new();

        for entity_type in entity_types {
            let pattern = Self::get_pattern(entity_type)?;
            patterns.insert(entity_type.clone(), pattern);
        }

        Ok(EntityDetector { patterns })
    }

    fn get_pattern(entity_type: &EntityType) -> Result<Regex, AnonymaskError> {
        let pattern_str = match entity_type {
            EntityType::Email => r"\b[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Z|a-z]{2,}\b",
            EntityType::Phone => r"\b\d{3}[-.]?\d{3}[-.]?\d{4}\b",
            EntityType::Ssn => r"\b\d{3}[-]?\d{2}[-]?\d{4}\b",
            EntityType::CreditCard => r"\b\d{4}[- ]?\d{4}[- ]?\d{4}[- ]?\d{4}\b",
            EntityType::IpAddress => r"\b\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}\b",
            EntityType::Url => r"\bhttps?://[^\s]+\b",
        };
        Regex::new(pattern_str).map_err(AnonymaskError::RegexError)
    }

    pub fn detect(&self, text: &str) -> Vec<Entity> {
        let mut entities = Vec::new();

        for (entity_type, regex) in &self.patterns {
            for mat in regex.find_iter(text) {
                entities.push(Entity {
                    entity_type: entity_type.clone(),
                    value: mat.as_str().to_string(),
                    start: mat.start(),
                    end: mat.end(),
                });
            }
        }

        // Sort by start position to handle overlaps
        entities.sort_by_key(|e| e.start);

        // Remove overlapping entities, prioritizing earlier ones
        let mut filtered: Vec<Entity> = Vec::new();
        for entity in entities {
            if filtered.is_empty() || filtered.last().unwrap().end <= entity.start {
                filtered.push(entity);
            }
        }

        filtered
    }
}

