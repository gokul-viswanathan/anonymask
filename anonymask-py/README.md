# anonymask

![PyPI Version](https://img.shields.io/pypi/v/anonymask)
![Python Versions](https://img.shields.io/pypi/pyversions/anonymask)
![License](https://img.shields.io/pypi/l/anonymask)
![Downloads](https://img.shields.io/pypi/dm/anonymask)

> Secure anonymization/de-anonymization library for protecting Personally Identifiable Information (PII) in Python applications. Built with Rust for maximum performance.

## ✨ Features

- **🚀 Blazing Fast**: Rust-powered core with < 5ms processing time
- **🔍 Comprehensive Detection**: EMAIL, PHONE, SSN, CREDIT_CARD, IP_ADDRESS, URL
- **🔒 Secure Placeholders**: Deterministic UUID-based anonymization
- **🐍 Pythonic API**: Clean, intuitive Python interface
- **⚡ Zero Dependencies**: No external runtime dependencies
- **🧵 Thread-Safe**: Safe for concurrent use in multi-threaded applications

## 📦 Installation

```bash
pip install anonymask
```

## 🚀 Quick Start

```python
from anonymask import Anonymizer

# Initialize with desired entity types
anonymizer = Anonymizer(['email', 'phone', 'ssn'])

# Anonymize text
text = "Contact john@email.com or call 555-123-4567. SSN: 123-45-6789"
result = anonymizer.anonymize(text)

# Result is a tuple: (anonymized_text, mapping, entities)
print(result[0])
# "Contact EMAIL_xxx or call PHONE_xxx. SSN: SSN_xxx"

print(result[1])
# {'EMAIL_xxx': 'john@email.com', 'PHONE_xxx': '555-123-4567', 'SSN_xxx': '123-45-6789'}

print(result[2])
# [
#   {'entity_type': 'email', 'value': 'john@email.com', 'start': 8, 'end': 22},
#   {'entity_type': 'phone', 'value': '555-123-4567', 'start': 31, 'end': 43},
#   {'entity_type': 'ssn', 'value': '123-45-6789', 'start': 50, 'end': 60}
# ]

# Deanonymize back to original
original = anonymizer.deanonymize(result[0], result[1])
print(original)
# "Contact john@email.com or call 555-123-4567. SSN: 123-45-6789"
```

## 🎯 Supported Entity Types

| Type | Description | Examples |
|------|-------------|----------|
| `email` | Email addresses | `user@domain.com`, `john.doe@company.co.uk` |
| `phone` | Phone numbers | `555-123-4567`, `555-123`, `(555) 123-4567`, `555.123.4567` |
| `ssn` | Social Security Numbers | `123-45-6789`, `123456789` |
| `credit_card` | Credit card numbers | `1234-5678-9012-3456`, `1234567890123456` |
| `ip_address` | IP addresses | `192.168.1.1`, `2001:0db8:85a3::8a2e:0370:7334` |
| `url` | URLs | `https://example.com`, `http://sub.domain.org/path` |

## 📚 API Reference

### Constructor

```python
anonymizer = Anonymizer(entity_types: List[str])
```

- `entity_types`: List of entity types to detect (see supported types above)

### Methods

#### `anonymize(text: str) -> Tuple[str, Dict[str, str], List[Dict]]`

Anonymizes the input text using automatic detection and returns detailed result.

**Returns:**
- `str`: Text with PII replaced by placeholders
- `Dict[str, str]`: Placeholder -> original value mapping
- `List[Dict]`: Array of detected entities with metadata

Each entity dictionary contains:
- `entity_type`: Type of entity (email, phone, etc.)
- `value`: Original detected value
- `start`: Start position in original text
- `end`: End position in original text

#### `anonymize_with_custom(text: str, custom_entities: Optional[Dict[str, List[str]]] = None) -> Tuple[str, Dict[str, str], List[Dict]]`

Anonymizes the input text using both automatic detection and custom entities.

**Parameters:**
- `text`: The input text to anonymize
- `custom_entities`: Optional dictionary mapping entity types to lists of custom values

**Example:**
```python
custom_entities = {
    "email": ["secret@company.com", "admin@internal.org"],
    "phone": ["555-999-0000"]
}

result = anonymizer.anonymize_with_custom(text, custom_entities)
```

#### `deanonymize(text: str, mapping: Dict[str, str]) -> str`

Restores original text using the provided mapping.

## 💡 Use Cases

### RAG Applications

```python
from anonymask import Anonymizer
import chromadb  # or any vector store

class SecureRAG:
    def __init__(self):
        self.anonymizer = Anonymizer(['email', 'phone', 'ssn', 'credit_card'])
        self.vector_store = chromadb.Client()
    
    def add_document(self, doc_id: str, text: str):
        # Anonymize before storing
        result = self.anonymizer.anonymize(text)
        safe_text = result[0]
        
        # Store anonymized text and mapping
        self.vector_store.add(
            documents=[safe_text],
            metadatas=[{'mapping': result[1], 'entities': result[2]}],
            ids=[doc_id]
        )
    
    def query(self, query: str):
        # Anonymize query
        result = self.anonymizer.anonymize(query)
        safe_query = result[0]
        
        # Search with anonymized query
        results = self.vector_store.query(query_texts=[safe_query])
        
        # Deanonymize results
        deanonymized_results = []
        for doc, metadata in zip(results['documents'][0], results['metadatas'][0]):
            original = self.anonymizer.deanonymize(doc, metadata['mapping'])
            deanonymized_results.append(original)
        
        return deanonymized_results
```

### FastAPI Integration

```python
from fastapi import FastAPI, HTTPException
from pydantic import BaseModel
from anonymask import Anonymizer

app = FastAPI()
anonymizer = Anonymizer(['email', 'phone', 'ssn'])

class TextInput(BaseModel):
    text: str

class AnonymizedOutput(BaseModel):
    anonymized_text: str
    entities_count: int
    entities: list

@app.post("/anonymize", response_model=AnonymizedOutput)
async def anonymize_text(input_data: TextInput):
    try:
        result = anonymizer.anonymize(input_data.text)
        return AnonymizedOutput(
            anonymized_text=result[0],
            entities_count=len(result[2]),
            entities=result[2]
        )
    except Exception as e:
        raise HTTPException(status_code=500, detail=str(e))

@app.post("/deanonymize")
async def deanonymize_text(anonymized_text: str, mapping: dict):
    try:
        original = anonymizer.deanonymize(anonymized_text, mapping)
        return {"original_text": original}
    except Exception as e:
        raise HTTPException(status_code=500, detail=str(e))
```

### Data Processing Pipeline

```python
import pandas as pd
from anonymask import Anonymizer
from typing import List, Dict

class DataProcessor:
    def __init__(self, entity_types: List[str]):
        self.anonymizer = Anonymizer(entity_types)
    
    def process_dataframe(self, df: pd.DataFrame, text_column: str) -> pd.DataFrame:
        """Process a pandas DataFrame with text data"""
        processed_data = []
        
        for idx, row in df.iterrows():
            text = row[text_column]
            result = self.anonymizer.anonymize(text)
            
            processed_row = row.copy()
            processed_row['original_text'] = text
            processed_row['anonymized_text'] = result[0]
            processed_row['pii_mapping'] = result[1]
            processed_row['entities_found'] = len(result[2])
            processed_row['entities'] = result[2]
            
            processed_data.append(processed_row)
        
        return pd.DataFrame(processed_data)
    
    def batch_process(self, texts: List[str]) -> List[Dict]:
        """Process a list of texts"""
        results = []
        
        for text in texts:
            result = self.anonymizer.anonymize(text)
            results.append({
                'original': text,
                'anonymized': result[0],
                'mapping': result[1],
                'entities': result[2],
                'entity_count': len(result[2])
            })
        
        return results

# Usage example
processor = DataProcessor(['email', 'phone', 'ssn'])
df = pd.read_csv('customer_data.csv')
processed_df = processor.process_dataframe(df, 'customer_message')
```

### LLM Integration

```python
from anonymask import Anonymizer
import openai

class SecureLLMClient:
    def __init__(self, api_key: str):
        self.client = openai.OpenAI(api_key=api_key)
        self.anonymizer = Anonymizer(['email', 'phone', 'ssn', 'credit_card'])
    
    def secure_chat_completion(self, messages: list, custom_entities: dict = None) -> str:
        # Anonymize all user messages
        anonymized_messages = []
        mappings = []
        
        for message in messages:
            if message['role'] == 'user':
                if custom_entities:
                    result = self.anonymizer.anonymize_with_custom(message['content'], custom_entities)
                else:
                    result = self.anonymizer.anonymize(message['content'])
                anonymized_messages.append({
                    'role': 'user',
                    'content': result[0]
                })
                mappings.append(result[1])
            else:
                anonymized_messages.append(message)
        
        # Get LLM response
        response = self.client.chat.completions.create(
            model="gpt-4",
            messages=anonymized_messages
        )
        
        llm_response = response.choices[0].message.content
        
        # Deanonymize response using the last mapping
        if mappings:
            safe_response = self.anonymizer.deanonymize(llm_response, mappings[-1])
        else:
            safe_response = llm_response
        
        return safe_response
```

### Custom Entity Anonymization

```python
from anonymask import Anonymizer

# Initialize with basic detection
anonymizer = Anonymizer(['email'])

# Define custom entities to anonymize
custom_entities = {
    'email': ['internal@company.com', 'admin@secure.org'],
    'phone': ['555-999-0000', '555-888-1111'],
    # You can even specify entity types not in the initial list
    'ssn': ['123-45-6789']
}

text = "Contact internal@company.com or call 555-999-0000"
result = anonymizer.anonymize_with_custom(text, custom_entities)

print(result[0])
# "Contact EMAIL_xxx or call PHONE_xxx"

print(result[1])
# {'EMAIL_xxx': 'internal@company.com', 'PHONE_xxx': '555-999-0000'}
```

## 🧪 Testing

```bash
# Install development dependencies
pip install pytest

# Run tests
pytest tests/test_anonymask.py -v

# Run tests with coverage
pytest tests/test_anonymask.py --cov=anonymask --cov-report=html
```

## 🔧 Development

### Building from Source

1. **Prerequisites**:
   - Python 3.8+
   - Rust (latest stable)
   - Maturin

2. **Setup**:
```bash
# Clone the repository
git clone https://github.com/gokul-viswanathan/anonymask.git
cd anonymask/anonymask-py

# Install development dependencies
pip install maturin pytest

# Build the package in development mode
maturin develop

# Run tests
pytest tests/
```

3. **Build for Release**:
```bash
# Build wheel and source distribution
maturin build --release --sdist

# The built wheels will be in target/wheels/
```

### Project Structure

```
anonymask-py/
├── src/
│   └── lib.rs              # Rust PyO3 bindings
├── python/
│   └── anonymask/
│       ├── __init__.py     # Python package interface
│       └── _anonymask.so   # Compiled Rust extension
├── tests/
│   └── test_anonymask.py   # Test suite
├── pyproject.toml          # Python package configuration
├── Cargo.toml              # Rust project configuration
└── README.md
```

## 🏗️ Architecture

This package uses PyO3 to create high-performance Python bindings from the Rust core library:

```
Python → PyO3 → Rust Core → Native Performance
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
4. Ensure all tests pass (`pytest`)
5. Follow the existing code style
6. Submit a pull request

## 🗺️ Roadmap

- [ ] Async API support
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

**Version**: 0.4.5 | **Built with ❤️ using Rust and PyO3**

