# @anonymask/core

![CI](https://github.com/gokul-viswanathan/anonymask/workflows/CI/badge.svg)
![NPM Version](https://img.shields.io/npm/v/@anonymask/core)
![License](https://img.shields.io/npm/l/@anonymask/core)

> Secure anonymization/de-anonymization library for protecting Personally Identifiable Information (PII) in Node.js applications. Built with Rust for maximum performance.

## ✨ Features

- **🚀 Blazing Fast**: Rust-powered core with < 5ms processing time
- **🔍 Comprehensive Detection**: EMAIL, PHONE, SSN, CREDIT_CARD, IP_ADDRESS, URL
- **🔒 Secure Placeholders**: Deterministic UUID-based anonymization
- **🛡️ Type-Safe**: Full TypeScript support with detailed definitions
- **⚡ Zero Dependencies**: No external runtime dependencies
- **🧵 Thread-Safe**: Safe for concurrent use

## 📦 Installation

```bash
npm install @anonymask/core
```

## 🚀 Quick Start

```javascript
const { Anonymizer } = require("@anonymask/core");

// Initialize with desired entity types
const anonymizer = new Anonymizer(["email", "phone", "ssn"]);

// Anonymize text
const text = "Contact john@email.com or call 555-123-4567. SSN: 123-45-6789";
const result = anonymizer.anonymize(text);

console.log(result.anonymizedText);
// "Contact EMAIL_xxx or call PHONE_xxx. SSN: SSN_xxx"

console.log(result.mapping);
// { EMAIL_xxx: 'john@email.com', PHONE_xxx: '555-123-4567', SSN_xxx: '123-45-6789' }

console.log(result.entities);
// [
//   { entity_type: 'email', value: 'john@email.com', start: 8, end: 22 },
//   { entity_type: 'phone', value: '555-123-4567', start: 31, end: 43 },
//   { entity_type: 'ssn', value: '123-45-6789', start: 50, end: 60 }
// ]

// Deanonymize back to original
const original = anonymizer.deanonymize(result.anonymized_text, result.mapping);
console.log(original);
// "Contact john@email.com or call 555-123-4567. SSN: 123-45-6789"
```

## 🎯 Supported Entity Types

| Type          | Description             | Examples                                            |
| ------------- | ----------------------- | --------------------------------------------------- |
| `email`       | Email addresses         | `user@domain.com`, `john.doe@company.co.uk`         |
| `phone`       | Phone numbers           | `555-123-4567`, `555-123`, `(555) 123-4567`, `555.123.4567` |
| `ssn`         | Social Security Numbers | `123-45-6789`, `123456789`                          |
| `credit_card` | Credit card numbers     | `1234-5678-9012-3456`, `1234567890123456`           |
| `ip_address`  | IP addresses            | `192.168.1.1`, `2001:0db8:85a3::8a2e:0370:7334`     |
| `url`         | URLs                    | `https://example.com`, `http://sub.domain.org/path` |

## 📚 API Reference

### Constructor

```javascript
const anonymizer = new Anonymizer(entityTypes: string[])
```

- `entityTypes`: Array of entity types to detect (see supported types above)

### Methods

#### `anonymize(text: string) => AnonymizationResult`

Anonymizes the input text using automatic detection and returns detailed result.

**Returns:**

```typescript
interface AnonymizationResult {
  anonymizedText: string; // Text with PII replaced by placeholders
  mapping: Record<string, string>; // Placeholder -> original value mapping
  entities: Entity[]; // Array of detected entities with metadata
}

interface Entity {
  entity_type: string; // Type of entity (email, phone, etc.)
  value: string; // Original detected value
  start: number; // Start position in original text
  end: number; // End position in original text
}
```

#### `anonymizeWithCustom(text: string, customEntities?: Record<string, string[]>) => AnonymizationResult`

Anonymizes the input text using both automatic detection and custom entities.

**Parameters:**
- `text`: The input text to anonymize
- `customEntities`: Optional map of entity types to arrays of custom values to anonymize

**Example:**
```javascript
const customEntities = {
  email: ["secret@company.com", "admin@internal.org"],
  phone: ["555-999-0000"]
};

const result = anonymizer.anonymizeWithCustom(text, customEntities);
```

#### `deanonymize(text: string, mapping: Record<string, string>) => string`

Restores original text using the provided mapping.

## 💡 Use Cases

### Express Middleware

```javascript
const express = require("express");
const { Anonymizer } = require("@anonymask/core");

const app = express();
const anonymizer = new Anonymizer(["email", "phone", "ssn"]);

// Middleware to anonymize request bodies
app.use(express.json());
app.use((req, res, next) => {
  if (req.body && req.body.text) {
    const result = anonymizer.anonymize(req.body.text);
    req.body.anonymized_text = result.anonymized_text;
    req.body.pii_mapping = result.mapping;
  }
  next();
});

app.post("/api/chat", (req, res) => {
  // Send req.body.anonymized_text to LLM
  // Store req.body.pii_mapping for deanonymization
  res.json({ message: "Processed securely" });
});
```

### LLM Integration

```javascript
const { Anonymizer } = require("@anonymask/core");

class SecureLLMClient {
  constructor() {
    this.anonymizer = new Anonymizer(["email", "phone", "ssn", "credit_card"]);
  }

  async processMessage(userMessage, customEntities = null) {
    // Anonymize user input with optional custom entities
    const result = customEntities 
      ? this.anonymizer.anonymizeWithCustom(userMessage, customEntities)
      : this.anonymizer.anonymize(userMessage);

    // Send anonymized message to LLM
    const llmResponse = await this.callLLM(result.anonymized_text);

    // Deanonymize LLM response
    const safeResponse = this.anonymizer.deanonymize(
      llmResponse,
      result.mapping,
    );

    return safeResponse;
  }
}
```

### Custom Entity Anonymization

```javascript
const { Anonymizer } = require("@anonymask/core");

// Initialize with basic detection
const anonymizer = new Anonymizer(["email"]);

// Define custom entities to anonymize
const customEntities = {
  email: ["internal@company.com", "admin@secure.org"],
  phone: ["555-999-0000", "555-888-1111"],
  // You can even specify entity types not in the initial list
  ssn: ["123-45-6789"]
};

const text = "Contact internal@company.com or call 555-999-0000";
const result = anonymizer.anonymizeWithCustom(text, customEntities);

console.log(result.anonymizedText);
// "Contact EMAIL_xxx or call PHONE_xxx"

console.log(result.mapping);
// { EMAIL_xxx: 'internal@company.com', PHONE_xxx: '555-999-0000' }
```

### Batch Processing

```javascript
const { Anonymizer } = require("@anonymask/core");
const fs = require("fs").promises;

async function processDataset(filePath) {
  const anonymizer = new Anonymizer(["email", "phone", "ssn"]);
  const data = await fs.readFile(filePath, "utf8");
  const records = JSON.parse(data);

  const processedRecords = records.map((record) => {
    const result = anonymizer.anonymize(record.text);
    return {
      ...record,
      original_text: record.text,
      anonymized_text: result.anonymized_text,
      pii_mapping: result.mapping,
      entities_detected: result.entities.length,
    };
  });

  await fs.writeFile(
    "processed_data.json",
    JSON.stringify(processedRecords, null, 2),
  );
  return processedRecords;
}
```

## 🧪 Testing

```bash
# Install dependencies
npm install

# Run tests
npm test

# Run tests with coverage
npm run test:coverage

# Build the package
npm run build

# Run benchmarks
npm run bench
```

## 🔧 Development

### Building from Source

```bash
# Clone the repository
git clone https://github.com/gokul-viswanathan/anonymask.git
cd anonymask/anonymask-node

# Install dependencies
npm install

# Build the native addon
npm run build

# Run tests
npm test
```

### Project Structure

```
anonymask-node/
├── src/
│   └── lib.rs              # Rust NAPI bindings
├── index.js                # JavaScript entry point
├── index.d.ts              # TypeScript definitions
├── tests/
│   └── test_anonymask.test.js  # Test suite
├── package.json
└── README.md
```

## 🏗️ Architecture

This package uses NAPI-RS to create high-performance Node.js bindings from the Rust core library:

```
JavaScript/TypeScript → NAPI-RS → Rust Core → Native Performance
```

The Rust core provides:

- **Memory Safety**: No buffer overflows or memory leaks
- **Performance**: Near-native execution speed
- **Concurrency**: Thread-safe operations
- **Reliability**: Robust error handling

## 📊 Performance

- **Processing Speed**: < 5ms for typical messages (< 500 words)
- **Memory Usage**: Minimal footprint with zero-copy operations
- **Startup Time**: Fast initialization with lazy loading
- **Concurrency**: Safe for use in multi-threaded environments

## 🔒 Security

- **Cryptographically Secure**: UUID v4 for unique placeholder generation
- **Deterministic**: Same input always produces same output
- **No Data Leakage**: Secure handling of PII throughout the process
- **Input Validation**: Comprehensive validation and error handling

## 📄 License

MIT License - see [LICENSE](../LICENSE) file for details.

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Add tests for new functionality
4. Ensure all tests pass (`npm test`)
5. Follow the existing code style
6. Submit a pull request

## 🗺️ Roadmap

- [ ] Streaming API for large texts
- [ ] Custom entity pattern support
- [ ] Persistent mapping storage
- [ ] Performance optimizations
- [ ] Additional entity types

## 📞 Support

- 📖 [Documentation](https://github.com/gokul-viswanathan/anonymask#readme)
- 🐛 [Issue Tracker](https://github.com/gokul-viswanathan/anonymask/issues)
- 💬 [Discussions](https://github.com/gokul-viswanathan/anonymask/discussions)

---

**Version**: 0.4.5 | **Built with ❤️ using Rust and NAPI-RS**
