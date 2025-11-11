# Anonymask Python Package

This package provides Python bindings for the Anonymask core library, enabling secure anonymization and de-anonymization of PII data.

## Installation

```bash
pip install anonymask
```

## Building from Source

1. Ensure you have Rust and Python installed.
2. Clone the repository and navigate to the `anonymask-py` directory.
3. Install dependencies:

```bash
pip install maturin
```

4. Build the package:

```bash
maturin build --release --sdist
```

This will compile the Rust code and generate the Python wheel.

## Usage

```python
from anonymask import Anonymizer

anonymizer = Anonymizer(['email', 'phone'])
result = anonymizer.anonymize('Contact john@email.com or call 555-123-4567')

print(result[0])  # "Contact EMAIL_xxx or call PHONE_xxx"
print(result[1])  # {'EMAIL_xxx': 'john@email.com', 'PHONE_xxx': '555-123-4567'}
```

## Publishing to PyPI

1. Ensure you have a PyPI account and are configured (use `~/.pypirc` or environment variables).
2. Update the version in `pyproject.toml`.
3. Build the package: `maturin build --release`.
4. Publish: `maturin publish`.

Note: Maturin handles the build and upload process. Make sure to have your PyPI credentials set up.

