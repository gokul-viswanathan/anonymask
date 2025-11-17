# anonymask

Secure anonymization/de-anonymization library for protecting Personally Identifiable Information (PII) before sending data to Large Language Models (LLMs). Built with Rust for performance and safety, with bindings for Python and Node.js.

## ✨ Features

- **🚀 High Performance**: Rust-powered core with < 5ms processing time for typical messages
- **🔍 Comprehensive Detection**: Regex-based entity detection for EMAIL, PHONE, SSN, CREDIT_CARD, IP_ADDRESS, URL
- **🔒 Secure Placeholders**: Deterministic UUID-based placeholders for consistent anonymization
- **🌐 Multi-Language**: Native bindings for Python and Node.js with identical APIs
- **⚡ Zero-Copy Deanonymization**: Efficient restoration of original values
- **🛡️ Robust Error Handling**: Comprehensive error handling and edge case support
- **🧵 Thread-Safe**: Concurrent access supported

## 📦 Installation

### Python

```bash
pip install anonymask
```

### Node.js

```bash
npm install @anonymask/core
```

## 🚀 Quick Start

### Python

```python
from anonymask import Anonymizer

# Initialize anonymizer with desired entity types
anonymizer = Anonymizer(['email', 'phone', 'ssn'])

# Anonymize text
text = "Contact john@email.com or call 555-123-4567. SSN: 123-45-6789"
result = anonymizer.anonymize(text)

# Result is a tuple: (anonymized_text, mapping, entities)
print(result[0])  # "Contact EMAIL_xxx or call PHONE_xxx. SSN: SSN_xxx"
print(result[1])  # {'EMAIL_xxx': 'john@email.com', 'PHONE_xxx': '555-123-4567', 'SSN_xxx': '123-45-6789'}
print(result[2])  # List of detected entities with metadata

# Deanonymize back to original
original = anonymizer.deanonymize(result[0], result[1])
print(original)  # "Contact john@email.com or call 555-123-4567. SSN: 123-45-6789"
```

### Node.js

```javascript
const { Anonymizer } = require("@anonymask/core");

// Initialize anonymizer with desired entity types
const anonymizer = new Anonymizer(["email", "phone", "ssn"]);

// Anonymize text
const text = "Contact john@email.com or call 555-123-4567. SSN: 123-45-6789";
const result = anonymizer.anonymize(text);

// Result is an object: { anonymized_text, mapping, entities }
console.log(result.anonymizedText); // "Contact EMAIL_xxx or call PHONE_xxx. SSN: SSN_xxx"
console.log(result.mapping); // { EMAIL_xxx: 'john@email.com', PHONE_xxx: '555-123-4567', SSN_xxx: '123-45-6789' }
console.log(result.entities); // Array of detected entities with metadata

// Deanonymize back to original
const original = anonymizer.deanonymize(result.anonymized_text, result.mapping);
console.log(original); // "Contact john@email.com or call 555-123-4567. SSN: 123-45-6789"
```

## 🎯 Supported Entity Types

| Entity Type   | Description             | Examples                                                       |
| ------------- | ----------------------- | -------------------------------------------------------------- |
| `email`       | Email addresses         | `user@domain.com`, `john.doe@company.co.uk`                    |
| `phone`       | Phone numbers           | `555-123-4567`, `(555) 123-4567`, `555.123.4567`, `5551234567` |
| `ssn`         | Social Security Numbers | `123-45-6789`, `123456789`                                     |
| `credit_card` | Credit card numbers     | `1234-5678-9012-3456`, `1234567890123456`                      |
| `ip_address`  | IP addresses            | `192.168.1.1`, `2001:0db8:85a3:0000:0000:8a2e:0370:7334`       |
| `url`         | URLs                    | `https://example.com`, `http://sub.domain.org/path`            |
| **Custom**     | Any custom entity type  | User-defined types like `name`, `company`, `address`, etc.        |

## 🏗️ Architecture

anonymask is built with a layered architecture for performance and safety:

```
┌─────────────────────────────────────────────────────────────┐
│                    Language Bindings                        │
│  ┌─────────────────┐    ┌─────────────────────────────────┐ │
│  │   Python        │    │      Node.js                     │ │
│  │   (PyO3)        │    │      (NAPI-RS)                   │ │
│  └─────────────────┘    └─────────────────────────────────┘ │
└─────────────────────────────────────────────────────────────┘
                              │
┌─────────────────────────────────────────────────────────────┐
│                      Core Library                           │
│  ┌─────────────────┐  ┌─────────────────┐  ┌──────────────┐ │
│  │ Entity Detection│  │  Anonymization  │  │Deanonymization│ │
│  │   (Regex)       │  │  (UUID Mapping) │  │ (Zero-Copy)   │ │
│  └─────────────────┘  └─────────────────┘  └──────────────┘ │
└─────────────────────────────────────────────────────────────┘
                              │
┌─────────────────────────────────────────────────────────────┐
│                    Rust Foundation                          │
│           Performance, Memory Safety, Concurrency           │
└─────────────────────────────────────────────────────────────┘
```

### Core Components

1. **Entity Detection**: Fast regex-based pattern matching for structured PII data
2. **Anonymization**: Replace detected entities with unique, deterministic placeholders
3. **Deanonymization**: Restore original values using efficient placeholder-to-value mapping

## 📊 Performance

- **Processing Speed**: < 5ms for typical messages (< 500 words)
- **Memory Efficiency**: Minimal memory footprint with no external dependencies
- **Concurrency**: Thread-safe design for parallel processing
- **Deterministic**: Same input always produces same output for consistency

## 🔒 Security Features

- **Cryptographically Secure**: UUID v4 generation for unique placeholders
- **Deterministic Mapping**: Consistent placeholder generation across sessions
- **Input Validation**: Comprehensive validation and sanitization
- **No Data Leakage**: Secure handling of sensitive information
- **Zero-Trust Design**: No logging or exposure of PII data

## 📚 API Reference

### Python API

```python
from anonymask import Anonymizer

# Initialize
anonymizer = Anonymizer(entity_types=['email', 'phone'])

# Anonymize
result = anonymizer.anonymize(text)
# Returns: (anonymized_text: str, mapping: dict, entities: list)

# Anonymize with custom entities
custom_entities = {
    'name': ['John Doe', 'Jane Smith'],
    'company': ['Acme Corp', 'Tech Inc']
}
result = anonymizer.anonymize_with_custom(text, custom_entities)
# Returns: (anonymized_text: str, mapping: dict, entities: list)

# Deanonymize
original = anonymizer.deanonymize(anonymized_text, mapping)
# Returns: str
```

### Node.js API

```javascript
const { Anonymizer } = require("@anonymask/core");

// Initialize
const anonymizer = new Anonymizer(["email", "phone"]);

// Anonymize
const result = anonymizer.anonymize(text);
// Returns: { anonymized_text: string, mapping: object, entities: array }

// Anonymize with custom entities
const customEntities = {
    name: ['John Doe', 'Jane Smith'],
    company: ['Acme Corp', 'Tech Inc']
};
const result = anonymizer.anonymizeWithCustom(text, customEntities);
// Returns: { anonymized_text: string, mapping: object, entities: array }

// Deanonymize
const original = anonymizer.deanonymize(anonymized_text, mapping);
// Returns: string
```

## 💡 Use Cases

### RAG Applications

```python
# Protect user data before sending to vector stores
anonymizer = Anonymizer(['email', 'phone', 'ssn'])
safe_document = anonymizer.anonymize(user_document)[0]
# Store safe_document in vector database
```

### LLM Chat Applications

```javascript
// Anonymize user messages before sending to LLM
const anonymizer = new Anonymizer(["email", "phone", "ssn", "credit_card"]);
const safeMessage = anonymizer.anonymize(userMessage);
// Send safeMessage.anonymized_text to LLM
```

### Custom Entity Anonymization

```python
# Anonymize custom entities like names and companies
anonymizer = Anonymizer([])  # No predefined entities
custom_entities = {
    'name': ['John Doe', 'Jane Smith'],
    'company': ['Acme Corp', 'Tech Inc'],
    'address': ['123 Main St', '456 Oak Ave']
}
text = "John Doe works at Acme Corp and lives at 123 Main St"
result = anonymizer.anonymize_with_custom(text, custom_entities)
# Result: "NAME_xxx works at COMPANY_xxx and lives at ADDRESS_xxx"
```

```javascript
// Custom entity anonymization in Node.js
const anonymizer = new Anonymizer([]);
const customEntities = {
    name: ['John Doe', 'Jane Smith'],
    company: ['Acme Corp', 'Tech Inc'],
    address: ['123 Main St', '456 Oak Ave']
};
const text = "John Doe works at Acme Corp and lives at 123 Main St";
const result = anonymizer.anonymizeWithCustom(text, customEntities);
// Result: "NAME_xxx works at COMPANY_xxx and lives at ADDRESS_xxx"
```

### Data Processing Pipelines

```python
# Batch processing with anonymization
anonymizer = Anonymizer(['email', 'phone'])
for record in dataset:
    safe_record = anonymizer.anonymize(record.text)[0]
    # Process safe_record
```

## 🧪 Testing

### Python

```bash
cd anonymask-py
pytest tests/test_anonymask.py -v
```

### Node.js

```bash
cd anonymask-node
npm test
```

### Rust Core

```bash
cd anonymask-core
cargo test
cargo bench  # Performance benchmarks
```

## 🔧 Development

### Building from Source

```bash
# Clone repository
git clone https://github.com/gokul-viswanathan/anonymask.git
cd anonymask

# Build Rust core library
cd anonymask-core
cargo build --release

# Build Python bindings
cd ../anonymask-py
maturin develop
maturin build --release --sdist

# Build Node.js bindings
cd ../anonymask-node
npm install
npm run build
```

### Project Structure

```
anonymask/
├── anonymask-core/          # Rust core library
│   ├── src/
│   │   ├── anonymizer.rs    # Main anonymization logic
│   │   ├── detection.rs     # Entity detection patterns
│   │   ├── entity.rs        # Entity types and structures
│   │   └── error.rs         # Error handling
│   └── Cargo.toml
├── anonymask-py/            # Python bindings
│   ├── src/lib.rs          # PyO3 bindings
│   ├── python/anonymask/   # Python package
│   └── pyproject.toml
├── anonymask-node/         # Node.js bindings
│   ├── src/lib.rs          # NAPI-RS bindings
│   ├── index.js            # JavaScript interface
│   └── package.json
└── examples/               # Usage examples
    ├── python/
    └── node/
```

## 📄 License

MIT License - see [LICENSE](LICENSE) file for details.

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Add tests for new functionality
4. Ensure all tests pass (`cargo test && npm test && pytest`)
5. Follow the existing code style and conventions
6. Submit a pull request

## 🗺️ Roadmap

- [x] **Custom Entity Types**: User-defined entity types with any string name
- [ ] **NER-based Detection**: Named Entity Recognition for PERSON, ORG, LOCATION
- [ ] **Custom Patterns**: User-defined regex patterns
- [ ] **External Storage**: Database backends for mapping persistence
- [ ] **More Languages**: Go, Java, C# bindings
- [ ] **Performance Mode**: Optimized batch processing
- [ ] **Fuzzy Matching**: Advanced entity detection with ML
- [ ] **Audit Logging**: Secure audit trail capabilities

## 📞 Support

- 📖 [Documentation](https://github.com/gokul-viswanathan/anonymask#readme)
- 🐛 [Issue Tracker](https://github.com/gokul-viswanathan/anonymask/issues)
- 💬 [Discussions](https://github.com/gokul-viswanathan/anonymask/discussions)

---

**Version**: 0.4.5 | **Built with ❤️ using Rust**
