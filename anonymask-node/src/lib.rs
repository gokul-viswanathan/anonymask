use anonymask_core::*;
use napi_derive::napi;

#[napi(object)]
#[derive(Clone)]
struct JsEntity {
    pub entity_type: String,
    pub value: String,
    pub start: u32,
    pub end: u32,
}

#[napi(object)]
struct JsAnonymizationResult {
    pub anonymized_text: String,
    pub mapping: std::collections::HashMap<String, String>,
    pub entities: Vec<JsEntity>,
}

#[napi]
struct JsAnonymizer {
    inner: Anonymizer,
}

#[napi]
impl JsAnonymizer {
    #[napi(constructor)]
    pub fn new(entity_types: Vec<String>) -> napi::Result<Self> {
        let entity_types: Result<Vec<EntityType>, _> = entity_types
            .into_iter()
            .map(|s| EntityType::from_str(&s))
            .collect();
        let entity_types = entity_types.map_err(|e| napi::Error::from_reason(e.to_string()))?;
        let inner =
            Anonymizer::new(entity_types).map_err(|e| napi::Error::from_reason(e.to_string()))?;
        Ok(JsAnonymizer { inner })
    }

    #[napi]
    pub fn anonymize(&self, text: String) -> napi::Result<JsAnonymizationResult> {
        let result = self
            .inner
            .anonymize(&text)
            .map_err(|e| napi::Error::from_reason(e.to_string()))?;
        Ok(JsAnonymizationResult {
            anonymized_text: result.anonymized_text,
            mapping: result.mapping,
            entities: result
                .entities
                .into_iter()
                .map(|e| JsEntity {
                    entity_type: format!("{:?}", e.entity_type).to_lowercase(),
                    value: e.value,
                    start: e.start as u32,
                    end: e.end as u32,
                })
                .collect(),
        })
    }

    #[napi]
    pub fn deanonymize(
        &self,
        text: String,
        mapping: std::collections::HashMap<String, String>,
    ) -> String {
        self.inner.deanonymize(&text, &mapping)
    }
}

