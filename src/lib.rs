pub mod entity;
pub mod detection;
pub mod anonymizer;
pub mod error;

pub use anonymizer::Anonymizer;
pub use entity::{AnonymizationResult, Entity, EntityType};
pub use error::AnonymaskError;

#[cfg(feature = "python")]
use pyo3::prelude::*;
#[cfg(feature = "python")]
use pyo3::exceptions::PyValueError;
#[cfg(feature = "python")]
use pyo3::Bound;

#[cfg(feature = "node")]
use napi_derive::napi;

#[cfg(feature = "python")]
#[pyclass(name = "Anonymizer")]
struct PyAnonymizer {
    inner: Anonymizer,
}

#[cfg(feature = "python")]
#[pymethods]
impl PyAnonymizer {
    #[new]
    fn new(entity_types: Vec<String>) -> PyResult<Self> {
        let entity_types: Result<Vec<EntityType>, _> = entity_types
            .into_iter()
            .map(|s| EntityType::from_str(&s))
            .collect();
        let entity_types = entity_types.map_err(|e| PyValueError::new_err(e.to_string()))?;
        let inner = Anonymizer::new(entity_types).map_err(|e| PyValueError::new_err(e.to_string()))?;
        Ok(PyAnonymizer { inner })
    }

    fn anonymize(&self, text: &str) -> PyResult<(String, std::collections::HashMap<String, String>, Vec<PyEntity>)> {
        let result = self.inner.anonymize(text).map_err(|e| PyValueError::new_err(e.to_string()))?;
        let entities: Vec<PyEntity> = result.entities.into_iter().map(|e| PyEntity {
            entity_type: format!("{:?}", e.entity_type).to_lowercase(),
            value: e.value,
            start: e.start,
            end: e.end,
        }).collect();
        Ok((result.anonymized_text, result.mapping, entities))
    }

    fn deanonymize(&self, text: &str, mapping: std::collections::HashMap<String, String>) -> String {
        self.inner.deanonymize(text, &mapping)
    }
}

#[cfg(feature = "python")]
#[pyclass(name = "Entity")]
#[derive(Clone)]
struct PyEntity {
    #[pyo3(get)]
    entity_type: String,
    #[pyo3(get)]
    value: String,
    #[pyo3(get)]
    start: usize,
    #[pyo3(get)]
    end: usize,
}

#[cfg(feature = "python")]
#[pymodule]
fn _anonymask(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyAnonymizer>()?;
    m.add_class::<PyEntity>()?;
    Ok(())
}

#[cfg(feature = "node")]
#[napi]
struct JsAnonymizer {
    inner: Anonymizer,
}

#[cfg(feature = "node")]
#[napi]
impl JsAnonymizer {
    #[napi(constructor)]
    pub fn new(entity_types: Vec<String>) -> napi::Result<Self> {
        let entity_types: Result<Vec<EntityType>, _> = entity_types
            .into_iter()
            .map(|s| EntityType::from_str(&s))
            .collect();
        let entity_types = entity_types.map_err(|e| napi::Error::from_reason(e.to_string()))?;
        let inner = Anonymizer::new(entity_types).map_err(|e| napi::Error::from_reason(e.to_string()))?;
        Ok(JsAnonymizer { inner })
    }

    #[napi]
    pub fn anonymize(&self, text: String) -> napi::Result<JsAnonymizationResult> {
        let result = self.inner.anonymize(&text).map_err(|e| napi::Error::from_reason(e.to_string()))?;
        Ok(JsAnonymizationResult {
            anonymized_text: result.anonymized_text,
            mapping: result.mapping,
            entities: result.entities.into_iter().map(|e| JsEntity {
                entity_type: format!("{:?}", e.entity_type).to_lowercase(),
                value: e.value,
                start: e.start as u32,
                end: e.end as u32,
            }).collect(),
        })
    }

    #[napi]
    pub fn deanonymize(&self, text: String, mapping: std::collections::HashMap<String, String>) -> String {
        self.inner.deanonymize(&text, &mapping)
    }
}

#[cfg(feature = "node")]
#[napi(object)]
#[derive(Clone)]
struct JsEntity {
    pub entity_type: String,
    pub value: String,
    pub start: u32,
    pub end: u32,
}

#[cfg(feature = "node")]
#[napi(object)]
struct JsAnonymizationResult {
    pub anonymized_text: String,
    pub mapping: std::collections::HashMap<String, String>,
    pub entities: Vec<JsEntity>,
}