const { Anonymizer } = require('./index.js');

const anonymizer = new Anonymizer(['email', 'phone']);

const text = 'Contact john@email.com or call 555-1234';
const result = anonymizer.anonymize(text);

console.log('Original:', text);
console.log('Anonymized:', result.anonymized_text);
console.log('Mapping:', result.mapping);

const original = anonymizer.deanonymize(result.anonymized_text, result.mapping);
console.log('Deanonymized:', original);