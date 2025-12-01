use napi_derive::napi;
use std::collections::HashMap;

use anonymask_core::{
    Anonymizer as CoreAnonymizer, AnonymizerConfig as CoreConfig,
    EntityType, PlaceholderFormat as CorePlaceholderFormat,
};

#[napi(object)]
#[derive(Clone)]
pub struct Entity {
  pub entity_type: String,
  pub value: String,
  pub start: u32,
  pub end: u32,
}

#[napi(object)]
pub struct AnonymizationResult {
  pub anonymized_text: String,
  pub mapping: HashMap<String, String>,
  pub entities: Vec<Entity>,
}

/// Configuration for anonymizer behavior.
///
/// Provides fine-grained control over how PII is detected and replaced.
///
/// Placeholder formats:
/// - "standard": TYPE_UUID (e.g., "EMAIL_a1b2c3d4...")
/// - "short": TYPE_COUNTER (e.g., "EMAIL_1", "EMAIL_2")
/// - Custom template string with {type}, {uuid}, {counter} placeholders
#[napi(object)]
#[derive(Clone)]
pub struct AnonymizerConfig {
  /// Whether custom entity matching is case-sensitive (default: true)
  pub case_sensitive: bool,
  /// Check word boundaries for custom entities (default: false)
  pub word_boundary_check: bool,
  /// Format for placeholders - "standard", "short", or custom template (default: "standard")
  pub placeholder_format: String,
  /// Maximum entities to detect, 0 for unlimited (default: 0)
  pub max_entities: u32,
}

impl Default for AnonymizerConfig {
  fn default() -> Self {
    Self {
      case_sensitive: true,
      word_boundary_check: false,
      placeholder_format: "standard".to_string(),
      max_entities: 0,
    }
  }
}

impl AnonymizerConfig {
  fn to_core(&self) -> CoreConfig {
    let placeholder_format = match self.placeholder_format.as_str() {
      "standard" => CorePlaceholderFormat::Standard,
      "short" => CorePlaceholderFormat::Short,
      template => CorePlaceholderFormat::Custom(template.to_string()),
    };

    CoreConfig {
      case_sensitive: self.case_sensitive,
      word_boundary_check: self.word_boundary_check,
      placeholder_format,
      max_entities: self.max_entities as usize,
    }
  }
}

#[napi]
pub struct Anonymizer {
  inner: CoreAnonymizer,
}

#[napi]
impl Anonymizer {
  /// Create a new anonymizer with optional configuration.
  ///
  /// Examples:
  /// ```js
  /// // Without config (uses defaults)
  /// const anonymizer = new Anonymizer(['email', 'phone']);
  ///
  /// // With config
  /// const config = {
  ///   caseSensitive: true,
  ///   wordBoundaryCheck: false,
  ///   placeholderFormat: 'short',
  ///   maxEntities: 100
  /// };
  /// const anonymizer = new Anonymizer(['email'], config);
  /// ```
  #[napi(constructor)]
  pub fn new(entity_types: Vec<String>, config: Option<AnonymizerConfig>) -> napi::Result<Self> {
    let entity_types: Result<Vec<EntityType>, _> = entity_types
      .into_iter()
      .map(|s| EntityType::from_str(&s))
      .collect();

    let entity_types = entity_types.map_err(|e| napi::Error::from_reason(e.to_string()))?;

    let inner = if let Some(cfg) = config {
      CoreAnonymizer::with_config(entity_types, cfg.to_core())
        .map_err(|e| napi::Error::from_reason(e.to_string()))?
    } else {
      CoreAnonymizer::new(entity_types).map_err(|e| napi::Error::from_reason(e.to_string()))?
    };

    Ok(Self { inner })
  }

  #[napi]
  pub fn anonymize(&self, text: String) -> napi::Result<AnonymizationResult> {
    let result = self
      .inner
      .anonymize(&text)
      .map_err(|e| napi::Error::from_reason(e.to_string()))?;

    Ok(AnonymizationResult {
      anonymized_text: result.anonymized_text,
      mapping: result.mapping,
      entities: result
        .entities
        .into_iter()
        .map(|e| Entity {
          entity_type: match &e.entity_type {
            EntityType::Custom(name) => name.clone(),
            _ => format!("{:?}", e.entity_type).to_lowercase(),
          },
          value: e.value,
          start: e.start as u32,
          end: e.end as u32,
        })
        .collect(),
    })
  }

  #[napi]
  pub fn anonymize_with_custom(
    &self,
    text: String,
    custom_entities: Option<HashMap<String, Vec<String>>>,
  ) -> napi::Result<AnonymizationResult> {
    // Convert string entity types to EntityType enum
    let custom_entities = match custom_entities {
      Some(map) => {
        let mut converted_map = HashMap::new();
        for (entity_type_str, values) in map {
          let entity_type = EntityType::from_str(&entity_type_str)
            .map_err(|e| napi::Error::from_reason(e.to_string()))?;
          converted_map.insert(entity_type, values);
        }
        Some(converted_map)
      }
      None => None,
    };

    let result = self
      .inner
      .anonymize_with_custom(&text, custom_entities.as_ref())
      .map_err(|e| napi::Error::from_reason(e.to_string()))?;

    Ok(AnonymizationResult {
      anonymized_text: result.anonymized_text,
      mapping: result.mapping,
      entities: result
        .entities
        .into_iter()
        .map(|e| Entity {
          entity_type: match &e.entity_type {
            EntityType::Custom(name) => name.clone(),
            _ => format!("{:?}", e.entity_type).to_lowercase(),
          },
          value: e.value,
          start: e.start as u32,
          end: e.end as u32,
        })
        .collect(),
    })
  }

  #[napi]
  pub fn deanonymize(&self, text: String, mapping: HashMap<String, String>) -> String {
    self.inner.deanonymize(&text, &mapping)
  }
}
