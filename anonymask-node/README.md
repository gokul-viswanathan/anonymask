# Anonymask Node.js Package

This package provides Node.js bindings for the Anonymask core library, enabling secure anonymization and de-anonymization of PII data.

## Installation

```bash
npm install @anonymask/node
```

## Building from Source

1. Ensure you have Rust and Node.js installed.
2. Clone the repository and navigate to the `anonymask-node` directory.
3. Install dependencies:

```bash
npm install
```

4. Build the package:

```bash
npm run build
```

This will compile the Rust code and generate the native Node.js module.

## Usage

```javascript
const { Anonymizer } = require('@anonymask/node');

const anonymizer = new Anonymizer(['email', 'phone']);
const result = anonymizer.anonymize('Contact john@email.com or call 555-123-4567');

console.log(result.anonymized_text); // "Contact EMAIL_xxx or call PHONE_xxx"
console.log(result.mapping); // { "EMAIL_xxx": "john@email.com", "PHONE_xxx": "555-123-4567" }
```

## Publishing to npm

1. Ensure you have an npm account and are logged in (`npm login`).
2. Update the version in `package.json`.
3. Build the package: `npm run build`.
4. Publish: `npm publish`.

Note: The package will automatically build before publishing due to the `prepublishOnly` script.