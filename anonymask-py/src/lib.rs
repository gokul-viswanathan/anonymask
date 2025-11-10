use anonymask_core::*;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::Bound;

// Alias the core Anonymizer to avoid conflict
use anonymask_core::Anonymizer as CoreAnonymizer;

#[pyclass(name = "Anonymizer")]
struct Anonymizer {
    inner: CoreAnonymizer,
}

#[pymethods]
impl Anonymizer {
    #[new]
    fn new(entity_types: Vec<String>) -> PyResult<Self> {
        let entity_types: Result<Vec<EntityType>, _> = entity_types
            .into_iter()
            .map(|s| EntityType::from_str(&s))
            .collect();
        let entity_types = entity_types.map_err(|e| PyValueError::new_err(e.to_string()))?;
        let inner =
            CoreAnonymizer::new(entity_types).map_err(|e| PyValueError::new_err(e.to_string()))?;
        Ok(Anonymizer { inner })
    }

    fn anonymize(
        &self,
        text: &str,
    ) -> PyResult<(
        String,
        std::collections::HashMap<String, String>,
        Vec<PyEntity>,
    )> {
        let result = self
            .inner
            .anonymize(text)
            .map_err(|e| PyValueError::new_err(e.to_string()))?;
        let entities: Vec<PyEntity> = result
            .entities
            .into_iter()
            .map(|e| PyEntity {
                entity_type: format!("{:?}", e.entity_type).to_lowercase(),
                value: e.value,
                start: e.start,
                end: e.end,
            })
            .collect();
        Ok((result.anonymized_text, result.mapping, entities))
    }

    fn deanonymize(
        &self,
        text: &str,
        mapping: std::collections::HashMap<String, String>,
    ) -> String {
        self.inner.deanonymize(text, &mapping)
    }
}

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

#[pymodule]
fn _anonymask(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Anonymizer>()?;
    m.add_class::<PyEntity>()?;
    Ok(())
}