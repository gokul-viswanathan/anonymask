use anonymask_core::{Anonymizer, EntityType};

fn main() {
    let entity_types = vec![EntityType::Email, EntityType::Phone];
    // define custom entities as mapping
    let anonymizer = Anonymizer::new(entity_types).unwrap();

    let text = "Contact john@email.com or call 333-555-1234";
    let result = anonymizer.anonymize(text).unwrap();

    println!("Original: {}", text);
    println!("Anonymized: {}", result.anonymized_text);
    println!("Mapping: {:?}", result.mapping);
    println!("Entities: {:?}", result.entities);

    let deanonymized = anonymizer.deanonymize(&result.anonymized_text, &result.mapping);
    println!("Deanonymized: {}", deanonymized);

    // Test with empty string
    let empty_result = anonymizer.anonymize("").unwrap();
    println!(
        "Empty input - Anonymized: '{}'",
        empty_result.anonymized_text
    );
}
