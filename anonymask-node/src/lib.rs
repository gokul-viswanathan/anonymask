use napi_derive::napi;
use std::collections::HashMap;

use anonymask_core::{Anonymizer as CoreAnonymizer, EntityType};

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

#[napi]
pub struct Anonymizer {
  inner: CoreAnonymizer,
}

#[napi]
impl Anonymizer {
  #[napi(constructor)]
  pub fn new(entity_types: Vec<String>) -> napi::Result<Self> {
    let entity_types: Result<Vec<EntityType>, _> = entity_types
      .into_iter()
      .map(|s| EntityType::from_str(&s))
      .collect();

    let entity_types = entity_types.map_err(|e| napi::Error::from_reason(e.to_string()))?;

    let inner =
      CoreAnonymizer::new(entity_types).map_err(|e| napi::Error::from_reason(e.to_string()))?;

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
