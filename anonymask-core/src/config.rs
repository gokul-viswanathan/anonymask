use serde::{Deserialize, Serialize};

/// Configuration for the anonymizer behavior.
///
/// Provides fine-grained control over how PII is detected and replaced.
///
/// # Examples
///
/// ```
/// use anonymask_core::config::AnonymizerConfig;
///
/// let config = AnonymizerConfig::builder()
///     .with_case_sensitivity(true)
///     .with_word_boundary_check(true)
///     .build();
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnonymizerConfig {
    /// Whether custom entity matching should be case-sensitive
    pub case_sensitive: bool,

    /// Whether to check word boundaries for custom entities
    ///
    /// When true, "John" won't match "Johnson"
    pub word_boundary_check: bool,

    /// Format for placeholder generation
    pub placeholder_format: PlaceholderFormat,

    /// Maximum number of entities to detect (0 = unlimited)
    pub max_entities: usize,
}

/// Format for generated placeholders.
///
/// Controls how anonymized placeholders appear in the output text.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PlaceholderFormat {
    /// Standard format: TYPE_UUID (e.g., "EMAIL_a1b2c3d4...")
    Standard,

    /// Short format: TYPE_COUNTER (e.g., "EMAIL_1", "EMAIL_2")
    ///
    /// Uses sequential numbering instead of UUIDs for brevity.
    Short,

    /// Custom format with template string
    ///
    /// Available placeholders:
    /// - {type}: Entity type (uppercase)
    /// - {uuid}: UUID v4
    /// - {counter}: Sequential number
    ///
    /// Example: "[{type}:{counter}]"
    Custom(String),
}

impl Default for AnonymizerConfig {
    fn default() -> Self {
        Self {
            case_sensitive: true,
            word_boundary_check: false,
            placeholder_format: PlaceholderFormat::Standard,
            max_entities: 0, // unlimited
        }
    }
}

impl AnonymizerConfig {
    /// Create a new configuration builder.
    ///
    /// # Examples
    ///
    /// ```
    /// use anonymask_core::config::AnonymizerConfig;
    ///
    /// let config = AnonymizerConfig::builder()
    ///     .with_case_sensitivity(false)
    ///     .build();
    /// ```
    pub fn builder() -> AnonymizerConfigBuilder {
        AnonymizerConfigBuilder::default()
    }
}

/// Builder for creating `AnonymizerConfig`.
///
/// Provides a fluent API for constructing configuration objects.
///
/// # Examples
///
/// ```
/// use anonymask_core::config::{AnonymizerConfig, PlaceholderFormat};
///
/// let config = AnonymizerConfig::builder()
///     .with_case_sensitivity(false)
///     .with_word_boundary_check(true)
///     .with_placeholder_format(PlaceholderFormat::Short)
///     .with_max_entities(100)
///     .build();
/// ```
#[derive(Default)]
pub struct AnonymizerConfigBuilder {
    case_sensitive: Option<bool>,
    word_boundary_check: Option<bool>,
    placeholder_format: Option<PlaceholderFormat>,
    max_entities: Option<usize>,
}

impl AnonymizerConfigBuilder {
    /// Set whether custom entity matching should be case-sensitive.
    ///
    /// Default: `true`
    pub fn with_case_sensitivity(mut self, case_sensitive: bool) -> Self {
        self.case_sensitive = Some(case_sensitive);
        self
    }

    /// Set whether to check word boundaries for custom entities.
    ///
    /// Default: `false`
    pub fn with_word_boundary_check(mut self, check: bool) -> Self {
        self.word_boundary_check = Some(check);
        self
    }

    /// Set the placeholder format.
    ///
    /// Default: `PlaceholderFormat::Standard`
    pub fn with_placeholder_format(mut self, format: PlaceholderFormat) -> Self {
        self.placeholder_format = Some(format);
        self
    }

    /// Set the maximum number of entities to detect.
    ///
    /// Default: `0` (unlimited)
    pub fn with_max_entities(mut self, max: usize) -> Self {
        self.max_entities = Some(max);
        self
    }

    /// Build the configuration.
    pub fn build(self) -> AnonymizerConfig {
        let default = AnonymizerConfig::default();
        AnonymizerConfig {
            case_sensitive: self.case_sensitive.unwrap_or(default.case_sensitive),
            word_boundary_check: self.word_boundary_check.unwrap_or(default.word_boundary_check),
            placeholder_format: self.placeholder_format.unwrap_or(default.placeholder_format),
            max_entities: self.max_entities.unwrap_or(default.max_entities),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = AnonymizerConfig::default();
        assert!(config.case_sensitive);
        assert!(!config.word_boundary_check);
        assert_eq!(config.placeholder_format, PlaceholderFormat::Standard);
        assert_eq!(config.max_entities, 0);
    }

    #[test]
    fn test_builder_pattern() {
        let config = AnonymizerConfig::builder()
            .with_case_sensitivity(false)
            .with_word_boundary_check(true)
            .with_placeholder_format(PlaceholderFormat::Short)
            .with_max_entities(100)
            .build();

        assert!(!config.case_sensitive);
        assert!(config.word_boundary_check);
        assert_eq!(config.placeholder_format, PlaceholderFormat::Short);
        assert_eq!(config.max_entities, 100);
    }

    #[test]
    fn test_builder_partial() {
        let config = AnonymizerConfig::builder()
            .with_case_sensitivity(false)
            .build();

        assert!(!config.case_sensitive);
        assert!(!config.word_boundary_check); // default
        assert_eq!(config.placeholder_format, PlaceholderFormat::Standard); // default
    }

    #[test]
    fn test_custom_placeholder_format() {
        let format = PlaceholderFormat::Custom("[{type}:{counter}]".to_string());
        let config = AnonymizerConfig::builder()
            .with_placeholder_format(format.clone())
            .build();

        assert_eq!(config.placeholder_format, format);
    }
}
