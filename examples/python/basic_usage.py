#!/usr/bin/env python3
"""
Basic usage example for anonymask Python bindings.
"""

from anonymask import Anonymizer

def main():
    # Initialize anonymizer with desired entity types
    anonymizer = Anonymizer(['email', 'phone', 'ssn'])

    # Sample text with PII
    text = """
    Hello, my name is John Doe. You can contact me at john.doe@email.com
    or call me at 555-123-4567. My SSN is 123-45-6789.
    """

    print("Original text:")
    print(text)
    print()

    # Anonymize the text
    result = anonymizer.anonymize(text)

    print("Anonymized text:")
    print(result[0])  # anonymized_text
    print()

    print("Detected entities:")
    for entity in result[2]:  # entities
        print(f"- {entity['entity_type']}: {entity['value']} (positions {entity['start']}-{entity['end']})")
    print()

    print("Mapping (placeholder -> original):")
    for placeholder, original in result[1].items():  # mapping
        print(f"- {placeholder} -> {original}")
    print()

    # Deanonymize back to original
    deanonymized = anonymizer.deanonymize(result[0], result[1])
    print("Deanonymized text:")
    print(deanonymized)
    print()

    print("Verification - texts match:", text.strip() == deanonymized.strip())

if __name__ == "__main__":
    main()