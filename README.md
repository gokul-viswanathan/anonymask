# anonymask

Secure anonymization/de-anonymization library for protecting Personally Identifiable Information (PII) before sending data to Large Language Models (LLMs).

## Features

- **Fast regex-based entity detection** for EMAIL, PHONE, SSN, CREDIT_CARD, IP_ADDRESS, URL
- **Thread-safe in-memory storage** with TTL support (optional)
- **Deterministic UUID-based placeholders** for consistent anonymization
- **Multi-language bindings** for Python and Node.js
- **Zero-copy deanonymization** for performance
- **Comprehensive error handling** and edge case support

## Installation

### Python

```bash
pip install anonymask
```

### Node.js

```bash
npm install @anonymask/core
```

## Quick Start

### Python

```python
from anonymask import Anonymizer

# Initialize
anonymizer = Anonymizer(['email', 'phone'])

# Anonymize
result = anonymizer.anonymize("Contact john@email.com or 555-1234")
print(result[0])  # "Contact EMAIL_abc123 or 555-1234"

# Deanonymize
original = anonymizer.deanonymize(result[0], result[1])
print(original)  # "Contact john@email.com or 555-1234"
```

### Node.js

```javascript
const { Anonymizer } = require("@anonymask/core");

const anonymizer = new Anonymizer(["email", "phone"]);
const result = anonymizer.anonymize("Contact john@email.com or 555-1234");
console.log(result.anonymized_text); // "Contact EMAIL_abc123 or 555-1234"

const original = anonymizer.deanonymize(result.anonymized_text, result.mapping);
console.log(original); // "Contact john@email.com or 555-1234"
```

## Supported Entity Types

- `email` - Email addresses (e.g., user@domain.com)
- `phone` - Phone numbers (e.g., 555-123-4567, 555-1234)
- `ssn` - Social Security Numbers (e.g., 123-45-6789)
- `credit_card` - Credit card numbers (e.g., 1234-5678-9012-3456)
- `ip_address` - IP addresses (e.g., 192.168.1.1)
- `url` - URLs (e.g., https://example.com)

## Architecture

anonymask is built with Rust for performance and safety:

```
User Input → Entity Detection → Placeholder Generation → Mapping Storage → Anonymized Output
                                      ↓
LLM Processing ← Deanonymization ← Mapping Retrieval ← Placeholder Matching
```

### Core Components

1. **Entity Detection**: Regex-based pattern matching for structured data
2. **Anonymization**: Replace detected entities with unique placeholders
3. **Storage**: In-memory mapping with optional TTL (user-managed)
4. **Deanonymization**: Restore original values using stored mappings

## Performance

- **Typical message (< 500 words)**: < 5ms processing time
- **Memory efficient**: No external dependencies for core functionality
- **Thread-safe**: Concurrent access supported

## Security

- **No data persistence**: Mappings stored in memory only
- **Deterministic placeholders**: Same input always produces same output
- **Secure UUID generation**: Cryptographically secure random IDs
- **Input validation**: Comprehensive error handling for malformed data

## Examples

See the `examples/` directory for complete usage examples:

- **Python**: Basic usage and RAG integration
- **Node.js**: Basic usage and Express middleware

## Development

### Building from Source

```bash
# Clone repository
git clone https://github.com/yourusername/anonymask.git
cd anonymask

# Build Rust library
cargo build --release

# Build Python bindings
cd bindings/python
maturin develop

# Build Node.js bindings
cd ../node
npm run build
```

### Running Tests

```bash
cargo test                    # Unit tests
cargo bench                   # Benchmarks
```

## License

MIT License - see LICENSE file for details.

## Contributing

1. Fork the repository
2. Create a feature branch
3. Add tests for new functionality
4. Ensure all tests pass
5. Submit a pull request

## Roadmap

- [ ] NER-based entity detection (PERSON, ORG, LOCATION)
- [ ] Custom regex patterns
- [ ] External storage backends
- [ ] Additional language bindings (Go, Java, C#)

