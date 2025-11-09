use anonymask_core::{Anonymizer, EntityType};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn benchmark_anonymize_email(c: &mut Criterion) {
    let anonymizer = Anonymizer::new(vec![EntityType::Email]).unwrap();
    let text =
        "Contact john.doe@email.com for more information. Also reach out to jane.smith@company.org";

    c.bench_function("anonymize_email", |b| {
        b.iter(|| anonymizer.anonymize(black_box(text)))
    });
}

fn benchmark_anonymize_multiple_entities(c: &mut Criterion) {
    let anonymizer =
        Anonymizer::new(vec![EntityType::Email, EntityType::Phone, EntityType::Ssn]).unwrap();
    let text = "User john@email.com with phone 555-123-4567 and SSN 123-45-6789 contacted support";

    c.bench_function("anonymize_multiple", |b| {
        b.iter(|| anonymizer.anonymize(black_box(text)))
    });
}

fn benchmark_deanonymize(c: &mut Criterion) {
    let anonymizer = Anonymizer::new(vec![EntityType::Email]).unwrap();
    let text = "Contact john@email.com";
    let result = anonymizer.anonymize(text).unwrap();

    c.bench_function("deanonymize", |b| {
        b.iter(|| {
            anonymizer.deanonymize(
                black_box(&result.anonymized_text),
                black_box(&result.mapping),
            )
        })
    });
}

fn benchmark_large_text(c: &mut Criterion) {
    let anonymizer = Anonymizer::new(vec![EntityType::Email, EntityType::Phone]).unwrap();
    let text = "Large text with multiple emails: user1@email.com, user2@domain.org, user3@test.net and phones: 555-123-4567, 555-987-6543, 555-111-2222. ".repeat(10);

    c.bench_function("anonymize_large_text", |b| {
        b.iter(|| anonymizer.anonymize(black_box(&text)))
    });
}

criterion_group!(
    benches,
    benchmark_anonymize_email,
    benchmark_anonymize_multiple_entities,
    benchmark_deanonymize,
    benchmark_large_text
);
criterion_main!(benches);

