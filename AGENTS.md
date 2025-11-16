# Agent Instructions for Anonymask

## Build/Lint/Test Commands

### Rust (anonymask-core)
- **Build**: `cd anonymask-core && cargo build`
- **Test all**: `cd anonymask-core && cargo test`
- **Test single**: `cd anonymask-core && cargo test test_function_name`
- **Benchmarks**: `cd anonymask-core && cargo bench`
- **Format**: `cd anonymask-core && cargo fmt`

### Node.js (anonymask-node)
- **Install deps**: `cd anonymask-node && npm install`
- **Build**: `cd anonymask-node && npm run build`
- **Test all**: `cd anonymask-node && npm test`
- **Test single**: `cd anonymask-node && npx jest -t "test name"`
- **Format**: `cd anonymask-node && npm run format`

### Python (anonymask-py)
- **Build**: `cd anonymask-py && maturin develop`
- **Test all**: `cd anonymask-py && pytest tests/test_anonymask.py`
- **Test single**: `cd anonymask-py && pytest tests/test_anonymask.py::TestClass::test_method`

## Code Style Guidelines

### General
- **Indentation**: 2 spaces (configured in .editorconfig)
- **Line endings**: LF
- **Encoding**: UTF-8
- **Trim trailing whitespace**: Yes
- **Final newline**: Yes

### Rust
- **Formatting**: rustfmt with 2 spaces
- **Error handling**: Use `thiserror` crate with custom error types
- **Serialization**: Use `serde` with derive macros
- **Naming**: snake_case for functions/variables, PascalCase for types
- **Imports**: Group std imports first, then external crates, then local modules

### JavaScript/TypeScript
- **Formatting**: Prettier (configured in package.json)
- **Testing**: Jest with describe/test structure
- **Naming**: camelCase for functions/variables, PascalCase for classes/types
- **Imports**: Use ES6 imports, group external imports first

### Python
- **Testing**: pytest with class-based test structure
- **Naming**: snake_case for functions/variables, PascalCase for classes
- **Imports**: Standard library first, then third-party, then local modules

### Error Handling
- **Rust**: Return `Result<T, AnonymaskError>` for fallible operations
- **JavaScript**: Use try/catch or promise rejections
- **Python**: Raise custom exceptions or return error tuples

### Security
- Never log or expose sensitive PII data in error messages
- Use secure random generation for placeholders
- Validate all inputs before processing</content>
<parameter name="filePath">/home/gokul/Documents/projects/anonymask/AGENTS.md