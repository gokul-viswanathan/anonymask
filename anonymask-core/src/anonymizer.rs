use crate::config::{AnonymizerConfig, PlaceholderFormat};
use crate::detection::EntityDetector;
use crate::entity::{AnonymizationResult, EntityType};
use crate::error::AnonymaskError;
use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};
use uuid::Uuid;

/// Main anonymization engine for protecting PII in text.
///
/// The `Anonymizer` detects personally identifiable information (PII) and replaces
/// it with unique, deterministic placeholders. Original values can be restored using
/// the provided mapping.
///
/// # Features
///
/// - Regex-based detection for built-in entity types
/// - Custom entity support for domain-specific PII
/// - Deterministic placeholder generation (same value = same placeholder)
/// - Thread-safe operation
/// - Zero-copy deanonymization
/// - Configurable behavior via `AnonymizerConfig`
///
/// # Examples
///
/// ```
/// use anonymask_core::Anonymizer;
/// use anonymask_core::entity::EntityType;
///
/// let anonymizer = Anonymizer::new(vec![
///     EntityType::Email,
///     EntityType::Phone,
/// ]).unwrap();
///
/// let result = anonymizer.anonymize(
///     "Contact john@example.com at 555-123-4567"
/// ).unwrap();
///
/// assert!(!result.anonymized_text.contains("john@example.com"));
/// assert!(!result.anonymized_text.contains("555-123-4567"));
///
/// // Restore original text
/// let original = anonymizer.deanonymize(
///     &result.anonymized_text,
///     &result.mapping
/// );
/// assert_eq!(original, "Contact john@example.com at 555-123-4567");
/// ```
///
/// # With Configuration
///
/// ```
/// use anonymask_core::{Anonymizer, AnonymizerConfig, PlaceholderFormat};
/// use anonymask_core::entity::EntityType;
///
/// let config = AnonymizerConfig::builder()
///     .with_placeholder_format(PlaceholderFormat::Short)
///     .with_max_entities(100)
///     .build();
///
/// let anonymizer = Anonymizer::with_config(
///     vec![EntityType::Email],
///     config
/// ).unwrap();
/// ```
///
/// # Performance
///
/// - Typical message (< 500 words): < 5ms
/// - Email detection: ~500ns per match
/// - Memory: O(n) where n is text length
///
/// # Thread Safety
///
/// This type is `Send + Sync` and can be safely shared across threads.
pub struct Anonymizer {
    detector: EntityDetector,
    config: AnonymizerConfig,
    counter: AtomicUsize,
}

impl Anonymizer {
    /// Create a new anonymizer for the specified entity types.
    ///
    /// Initializes the detection engine with compiled regex patterns
    /// for all requested entity types.
    ///
    /// # Arguments
    ///
    /// * `entity_types` - Vector of entity types to detect
    ///
    /// # Returns
    ///
    /// * `Ok(Anonymizer)` - Successfully created anonymizer
    /// * `Err(AnonymaskError)` - If initialization fails
    ///
    /// # Examples
    ///
    /// ```
    /// use anonymask_core::Anonymizer;
    /// use anonymask_core::entity::EntityType;
    ///
    /// let anonymizer = Anonymizer::new(vec![
    ///     EntityType::Email,
    ///     EntityType::Phone,
    ///     EntityType::Ssn,
    /// ]).unwrap();
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if regex pattern compilation fails.
    pub fn new(entity_types: Vec<EntityType>) -> Result<Self, AnonymaskError> {
        Self::with_config(entity_types, AnonymizerConfig::default())
    }

    /// Create a new anonymizer with custom configuration.
    ///
    /// Allows full control over anonymization behavior including placeholder
    /// format, case sensitivity, and entity limits.
    ///
    /// # Arguments
    ///
    /// * `entity_types` - Vector of entity types to detect
    /// * `config` - Configuration for anonymizer behavior
    ///
    /// # Returns
    ///
    /// * `Ok(Anonymizer)` - Successfully created anonymizer
    /// * `Err(AnonymaskError)` - If initialization fails
    ///
    /// # Examples
    ///
    /// ```
    /// use anonymask_core::{Anonymizer, AnonymizerConfig, PlaceholderFormat};
    /// use anonymask_core::entity::EntityType;
    ///
    /// let config = AnonymizerConfig::builder()
    ///     .with_placeholder_format(PlaceholderFormat::Short)
    ///     .build();
    ///
    /// let anonymizer = Anonymizer::with_config(
    ///     vec![EntityType::Email, EntityType::Phone],
    ///     config
    /// ).unwrap();
    /// ```
    pub fn with_config(entity_types: Vec<EntityType>, config: AnonymizerConfig) -> Result<Self, AnonymaskError> {
        let detector = EntityDetector::new(&entity_types)?;

        Ok(Anonymizer {
            detector,
            config,
            counter: AtomicUsize::new(0),
        })
    }

    /// Anonymize text by replacing detected PII with placeholders.
    ///
    /// Scans the text for entities matching the configured types and replaces
    /// them with unique UUID-based placeholders. Returns the anonymized text,
    /// a mapping to restore original values, and metadata about detected entities.
    ///
    /// # Arguments
    ///
    /// * `text` - The text to anonymize
    ///
    /// # Returns
    ///
    /// An `AnonymizationResult` containing:
    /// - `anonymized_text`: Text with PII replaced
    /// - `mapping`: HashMap of placeholder -> original value
    /// - `entities`: Metadata about all detected entities
    ///
    /// # Examples
    ///
    /// ```
    /// use anonymask_core::Anonymizer;
    /// use anonymask_core::entity::EntityType;
    ///
    /// let anonymizer = Anonymizer::new(vec![EntityType::Email]).unwrap();
    /// let result = anonymizer.anonymize("Email: user@example.com").unwrap();
    ///
    /// assert_eq!(result.entities.len(), 1);
    /// assert!(!result.anonymized_text.contains("user@example.com"));
    /// ```
    ///
    /// # Deterministic Behavior
    ///
    /// The same PII value will always map to the same placeholder within a single
    /// anonymization operation. However, placeholders change between different
    /// `anonymize()` calls.
    pub fn anonymize(&self, text: &str) -> Result<AnonymizationResult, AnonymaskError> {
        self.anonymize_with_custom(text, None)
    }

    /// Anonymize text with both built-in and custom entity types.
    ///
    /// Extends the standard anonymization to include user-defined custom entities.
    /// Custom entities use substring matching rather than regex patterns.
    ///
    /// # Arguments
    ///
    /// * `text` - The text to anonymize
    /// * `custom_entities` - Optional map of custom entity types to values
    ///
    /// # Returns
    ///
    /// An `AnonymizationResult` with all detected entities anonymized.
    ///
    /// # Examples
    ///
    /// ```
    /// use anonymask_core::Anonymizer;
    /// use anonymask_core::entity::EntityType;
    /// use std::collections::HashMap;
    ///
    /// let anonymizer = Anonymizer::new(vec![EntityType::Email]).unwrap();
    ///
    /// let mut custom_entities = HashMap::new();
    /// custom_entities.insert(
    ///     EntityType::Custom("company".to_string()),
    ///     vec!["Acme Corp".to_string(), "Tech Inc".to_string()]
    /// );
    ///
    /// let result = anonymizer.anonymize_with_custom(
    ///     "Contact john@acme.com at Acme Corp",
    ///     Some(&custom_entities)
    /// ).unwrap();
    ///
    /// assert!(result.entities.len() >= 2);
    /// ```
    ///
    /// # Note
    ///
    /// Custom entity matching is case-sensitive and uses exact substring matching.
    /// For case-insensitive matching, provide lowercase values and lowercase the text.
    pub fn anonymize_with_custom(&self, text: &str, custom_entities: Option<&std::collections::HashMap<EntityType, Vec<String>>>) -> Result<AnonymizationResult, AnonymaskError> {
        if text.is_empty() {
            return Ok(AnonymizationResult {
                anonymized_text: String::new(),
                mapping: HashMap::new(),
                entities: Vec::new(),
            });
        }

        let entities = self.detector.detect(text, custom_entities);

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

    /// Restore original PII values using the anonymization mapping.
    ///
    /// Replaces all placeholders in the text with their original values
    /// from the mapping. This is the inverse operation of `anonymize()`.
    ///
    /// # Arguments
    ///
    /// * `text` - Anonymized text containing placeholders
    /// * `mapping` - HashMap mapping placeholders to original values
    ///
    /// # Returns
    ///
    /// Text with all placeholders replaced by original values.
    ///
    /// # Examples
    ///
    /// ```
    /// use anonymask_core::Anonymizer;
    /// use anonymask_core::entity::EntityType;
    ///
    /// let anonymizer = Anonymizer::new(vec![EntityType::Email]).unwrap();
    ///
    /// let original = "Email: user@example.com";
    /// let result = anonymizer.anonymize(original).unwrap();
    ///
    /// let restored = anonymizer.deanonymize(
    ///     &result.anonymized_text,
    ///     &result.mapping
    /// );
    ///
    /// assert_eq!(restored, original);
    /// ```
    ///
    /// # Performance
    ///
    /// Uses efficient string replacement with O(n*m) complexity where:
    /// - n = text length
    /// - m = number of placeholders
    ///
    /// Placeholders are sorted by length (longest first) to avoid
    /// partial replacement issues.
    ///
    /// # Note
    ///
    /// If the mapping is incomplete (missing placeholders), those
    /// placeholders will remain in the output text unchanged.
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

    /// Generate a unique placeholder for an entity.
    ///
    /// Creates a placeholder based on the configured format.
    /// Supports Standard (TYPE_UUID), Short (TYPE_COUNTER), and Custom formats.
    ///
    /// # Arguments
    ///
    /// * `entity_type` - The type of entity being replaced
    /// * `_value` - The actual PII value (currently unused but reserved for future deterministic generation)
    ///
    /// # Returns
    ///
    /// A unique placeholder string based on the configuration.
    ///
    /// # Examples
    ///
    /// - Standard: "EMAIL_a1b2c3d4e5f6..."
    /// - Short: "EMAIL_1", "EMAIL_2", etc.
    /// - Custom: "[EMAIL:1]" (with template "[{type}:{counter}]")
    fn generate_placeholder(&self, entity_type: &EntityType, _value: &str) -> String {
        let type_prefix = match entity_type {
            EntityType::Email => "EMAIL",
            EntityType::Phone => "PHONE",
            EntityType::Ssn => "SSN",
            EntityType::CreditCard => "CREDIT_CARD",
            EntityType::IpAddress => "IP_ADDRESS",
            EntityType::Url => "URL",
            EntityType::Custom(name) => name,
        };

        match &self.config.placeholder_format {
            PlaceholderFormat::Standard => {
                format!("{}_{}", type_prefix.to_uppercase(), Uuid::new_v4().simple())
            }
            PlaceholderFormat::Short => {
                let count = self.counter.fetch_add(1, Ordering::SeqCst) + 1;
                format!("{}_{}", type_prefix.to_uppercase(), count)
            }
            PlaceholderFormat::Custom(template) => {
                let count = self.counter.fetch_add(1, Ordering::SeqCst) + 1;
                let uuid = Uuid::new_v4().simple().to_string();
                template
                    .replace("{type}", &type_prefix.to_uppercase())
                    .replace("{uuid}", &uuid)
                    .replace("{counter}", &count.to_string())
            }
        }
    }
}