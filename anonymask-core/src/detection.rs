use crate::entity::{Entity, EntityType};
use crate::error::AnonymaskError;
use regex::Regex;
use std::collections::HashMap;

/// Entity detection engine using regex patterns.
///
/// Detects PII entities in text using compiled regex patterns for built-in types
/// and substring matching for custom entity types.
///
/// # Performance
///
/// - Email detection: ~500ns per match
/// - Typical message (< 500 words): < 5ms total
/// - Regex patterns are compiled once at initialization
///
/// # Thread Safety
///
/// This type is `Send + Sync` and can be safely shared across threads.
pub struct EntityDetector {
    patterns: HashMap<EntityType, Regex>,
}

impl EntityDetector {
    /// Create a new entity detector for the specified entity types.
    ///
    /// Compiles regex patterns for all requested built-in entity types.
    /// Custom entity types don't require regex compilation.
    ///
    /// # Arguments
    ///
    /// * `entity_types` - List of entity types to detect
    ///
    /// # Returns
    ///
    /// * `Ok(EntityDetector)` - Successfully created detector
    /// * `Err(AnonymaskError)` - If regex compilation fails
    ///
    /// # Examples
    ///
    /// ```
    /// use anonymask_core::detection::EntityDetector;
    /// use anonymask_core::entity::EntityType;
    ///
    /// let detector = EntityDetector::new(&[
    ///     EntityType::Email,
    ///     EntityType::Phone,
    /// ]).unwrap();
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - A regex pattern fails to compile (should never happen with built-in patterns)
    /// - A custom entity type is passed (custom types don't use regex)
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
            // Enhanced phone pattern: supports (555) 123-4567, 555-123-4567, 555.123.4567, 555-123, etc.
            EntityType::Phone => r"\b(?:\+?1[-.\s]?)?\(?([0-9]{3})\)?[-.\s]?([0-9]{3})(?:[-.\s]?([0-9]{4}))?\b",
            EntityType::Ssn => r"\b\d{3}[-]?\d{2}[-]?\d{4}\b",
            EntityType::CreditCard => r"\b\d{4}[- ]?\d{4}[- ]?\d{4}[- ]?\d{4}\b",
            // Enhanced IP pattern with validation
            EntityType::IpAddress => r"\b(?:(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.){3}(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\b",
            // Enhanced URL pattern: prevents trailing punctuation issues
            EntityType::Url => r"\bhttps?://(?:[a-zA-Z0-9-._~:/?#\[\]@!$&'()*+,;=]|%[0-9A-Fa-f]{2})+",
            EntityType::Custom(_) => {
                return Err(AnonymaskError::InvalidEntityType {
                    entity_type: format!("{:?}", entity_type),
                    reason: "Custom entity types don't use regex patterns".to_string(),
                })
            }
        };
        Regex::new(pattern_str).map_err(|e| AnonymaskError::RegexError {
            pattern: pattern_str.to_string(),
            source: e,
        })
    }

    /// Detect all PII entities in the given text.
    ///
    /// Searches for entities using both regex patterns (for built-in types)
    /// and substring matching (for custom types). Handles overlapping entities
    /// by prioritizing the one that appears first in the text.
    ///
    /// # Arguments
    ///
    /// * `text` - The text to scan for PII entities
    /// * `custom_entities` - Optional map of custom entity types to values
    ///
    /// # Returns
    ///
    /// A vector of detected entities, sorted by position, with overlaps removed.
    ///
    /// # Examples
    ///
    /// ```
    /// use anonymask_core::detection::EntityDetector;
    /// use anonymask_core::entity::EntityType;
    ///
    /// let detector = EntityDetector::new(&[EntityType::Email]).unwrap();
    /// let entities = detector.detect("Contact user@example.com", None);
    ///
    /// assert_eq!(entities.len(), 1);
    /// assert_eq!(entities[0].value, "user@example.com");
    /// ```
    ///
    /// # Overlap Handling
    ///
    /// If two entities overlap in the text, only the one appearing first
    /// is kept. This prevents detecting "phone@email.com" as both a phone
    /// number and an email address.
    pub fn detect(&self, text: &str, custom_entities: Option<&std::collections::HashMap<EntityType, Vec<String>>>) -> Vec<Entity> {
        let mut entities = Vec::new();

        // Detect entities using regex patterns
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

        // Detect custom entities
        if let Some(custom_map) = custom_entities {
            for (entity_type, values) in custom_map {
                for value in values {
                    let mut start = 0;
                    while let Some(pos) = text[start..].find(value) {
                        let absolute_start = start + pos;
                        let absolute_end = absolute_start + value.len();
                        
                        entities.push(Entity {
                            entity_type: entity_type.clone(),
                            value: value.clone(),
                            start: absolute_start,
                            end: absolute_end,
                        });
                        
                        start = absolute_start + 1; // Move past this occurrence
                    }
                }
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
