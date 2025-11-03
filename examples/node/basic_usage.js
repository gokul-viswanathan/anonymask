#!/usr/bin/env node
/**
 * Basic usage example for anonymask Node.js bindings.
 */

const { Anonymizer } = require('@anonymask/core');

function main() {
    // Initialize anonymizer with desired entity types
    const anonymizer = new Anonymizer(['email', 'phone', 'ssn']);

    // Sample text with PII
    const text = `
    Hello, my name is John Doe. You can contact me at john.doe@email.com
    or call me at 555-123-4567. My SSN is 123-45-6789.
    `;

    console.log('Original text:');
    console.log(text);
    console.log();

    // Anonymize the text
    const result = anonymizer.anonymize(text);

    console.log('Anonymized text:');
    console.log(result.anonymized_text);
    console.log();

    console.log('Detected entities:');
    result.entities.forEach(entity => {
        console.log(`- ${entity.entity_type}: ${entity.value} (positions ${entity.start}-${entity.end})`);
    });
    console.log();

    console.log('Mapping (placeholder -> original):');
    Object.entries(result.mapping).forEach(([placeholder, original]) => {
        console.log(`- ${placeholder} -> ${original}`);
    });
    console.log();

    // Deanonymize back to original
    const deanonymized = anonymizer.deanonymize(result.anonymized_text, result.mapping);
    console.log('Deanonymized text:');
    console.log(deanonymized);
    console.log();

    console.log('Verification - texts match:', text.trim() === deanonymized.trim());
}

main();