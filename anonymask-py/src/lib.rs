use anonymask_core::*;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::Bound;

// Alias the core types to avoid conflict
use anonymask_core::Anonymizer as CoreAnonymizer;
use anonymask_core::AnonymizerConfig as CoreConfig;
use anonymask_core::PlaceholderFormat as CorePlaceholderFormat;

/// Configuration for anonymizer behavior.
///
/// Provides fine-grained control over how PII is detected and replaced.
///
/// Placeholder formats:
/// - "standard": TYPE_UUID (e.g., "EMAIL_a1b2c3d4...")
/// - "short": TYPE_COUNTER (e.g., "EMAIL_1", "EMAIL_2")
/// - Custom template string with {type}, {uuid}, {counter} placeholders
#[pyclass(name = "AnonymizerConfig")]
#[derive(Clone)]
struct PyAnonymizerConfig {
    #[pyo3(get, set)]
    pub case_sensitive: bool,
    #[pyo3(get, set)]
    pub word_boundary_check: bool,
    #[pyo3(get, set)]
    pub max_entities: usize,
    #[pyo3(get, set)]
    pub placeholder_format: String,
}

#[pymethods]
impl PyAnonymizerConfig {
    /// Create a new configuration.
    ///
    /// Args:
    ///     case_sensitive: Whether custom entity matching is case-sensitive (default: True)
    ///     word_boundary_check: Check word boundaries for custom entities (default: False)
    ///     placeholder_format: Format for placeholders - "standard", "short", or custom template (default: "standard")
    ///     max_entities: Maximum entities to detect, 0 for unlimited (default: 0)
    ///
    /// Examples:
    ///     >>> config = AnonymizerConfig()
    ///     >>> config = AnonymizerConfig(placeholder_format="short")
    ///     >>> config = AnonymizerConfig(placeholder_format="[{type}:{counter}]")
    #[new]
    #[pyo3(signature = (case_sensitive=true, word_boundary_check=false, placeholder_format="standard".to_string(), max_entities=0))]
    fn new(
        case_sensitive: bool,
        word_boundary_check: bool,
        placeholder_format: String,
        max_entities: usize,
    ) -> Self {
        PyAnonymizerConfig {
            case_sensitive,
            word_boundary_check,
            placeholder_format,
            max_entities,
        }
    }

    fn __repr__(&self) -> String {
        format!(
            "AnonymizerConfig(case_sensitive={}, word_boundary_check={}, placeholder_format='{}', max_entities={})",
            self.case_sensitive, self.word_boundary_check, self.placeholder_format, self.max_entities
        )
    }
}

impl PyAnonymizerConfig {
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
            max_entities: self.max_entities,
        }
    }
}

#[pyclass(name = "Anonymizer")]
struct Anonymizer {
    inner: CoreAnonymizer,
}

#[pymethods]
impl Anonymizer {
    #[new]
    #[pyo3(signature = (entity_types, config=None))]
    fn new(entity_types: Vec<String>, config: Option<PyAnonymizerConfig>) -> PyResult<Self> {
        let entity_types: Result<Vec<EntityType>, _> = entity_types
            .into_iter()
            .map(|s| EntityType::from_str(&s))
            .collect();
        let entity_types = entity_types.map_err(|e| PyValueError::new_err(e.to_string()))?;

        let inner = if let Some(cfg) = config {
            CoreAnonymizer::with_config(entity_types, cfg.to_core())
                .map_err(|e| PyValueError::new_err(e.to_string()))?
        } else {
            CoreAnonymizer::new(entity_types).map_err(|e| PyValueError::new_err(e.to_string()))?
        };

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
                entity_type: match &e.entity_type {
                    EntityType::Custom(name) => name.clone(),
                    _ => format!("{:?}", e.entity_type).to_lowercase(),
                },
                value: e.value,
                start: e.start,
                end: e.end,
            })
            .collect();
        Ok((result.anonymized_text, result.mapping, entities))
    }

    #[pyo3(signature = (text, custom_entities=None))]
    fn anonymize_with_custom(
        &self,
        text: &str,
        custom_entities: Option<std::collections::HashMap<String, Vec<String>>>,
    ) -> PyResult<(
        String,
        std::collections::HashMap<String, String>,
        Vec<PyEntity>,
    )> {
        // Convert string entity types to EntityType enum
        let custom_entities = match custom_entities {
            Some(map) => {
                let mut converted_map = std::collections::HashMap::new();
                for (entity_type_str, values) in map {
                    let entity_type = EntityType::from_str(&entity_type_str)
                        .map_err(|e| PyValueError::new_err(e.to_string()))?;
                    converted_map.insert(entity_type, values);
                }
                Some(converted_map)
            }
            None => None,
        };

        let result = self
            .inner
            .anonymize_with_custom(text, custom_entities.as_ref())
            .map_err(|e| PyValueError::new_err(e.to_string()))?;
        let entities: Vec<PyEntity> = result
            .entities
            .into_iter()
            .map(|e| PyEntity {
                entity_type: match &e.entity_type {
                    EntityType::Custom(name) => name.clone(),
                    _ => format!("{:?}", e.entity_type).to_lowercase(),
                },
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
    m.add_class::<PyAnonymizerConfig>()?;
    Ok(())
}