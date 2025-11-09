#!/usr/bin/env node
/**
 * Integration tests for anonymask Node.js bindings.
 */

const { JsAnonymizer } = require("../anonymask.node");

describe("Anonymizer", () => {
  let anonymizer;

  beforeEach(() => {
    anonymizer = new JsAnonymizer(["email", "phone"]);
  });

  test("anonymizes email", () => {
    const text = "Contact john@email.com";
    const result = anonymizer.anonymize(text);

    expect(result.anonymized_text).toContain("EMAIL_");
    expect(result.entities).toHaveLength(1);
    expect(result.entities[0].entity_type).toBe("email");
    expect(result.entities[0].value).toBe("john@email.com");
  });

  test("anonymizes phone", () => {
    const text = "Call 555-123-4567";
    const result = anonymizer.anonymize(text);

    expect(result.anonymized_text).toContain("PHONE_");
    expect(result.entities).toHaveLength(1);
    expect(result.entities[0].entity_type).toBe("phone");
  });

  test("anonymizes multiple entities", () => {
    const text = "Email: user@test.com, Phone: 555-1234";
    const result = anonymizer.anonymize(text);

    expect(result.anonymized_text).toContain("EMAIL_");
    expect(result.anonymized_text).toContain("PHONE_");
    expect(result.entities).toHaveLength(2);
  });

  test("deanonymizes correctly", () => {
    const original = "Contact john@email.com today";
    const result = anonymizer.anonymize(original);
    const deanonymized = anonymizer.deanonymize(
      result.anonymized_text,
      result.mapping,
    );

    expect(deanonymized).toBe(original);
  });

  test("handles empty text", () => {
    const result = anonymizer.anonymize("");

    expect(result.anonymized_text).toBe("");
    expect(result.entities).toHaveLength(0);
  });

  test("handles text with no entities", () => {
    const text = "This is a regular message with no PII";
    const result = anonymizer.anonymize(text);

    expect(result.anonymized_text).toBe(text);
    expect(result.entities).toHaveLength(0);
  });

  test("handles duplicate entities", () => {
    const text = "Contact john@email.com or reach out to john@email.com again";
    const result = anonymizer.anonymize(text);

    // Should use same placeholder for duplicate email
    const emailPlaceholders = Object.keys(result.mapping).filter((k) =>
      k.startsWith("EMAIL_"),
    );
    expect(emailPlaceholders).toHaveLength(1);
    expect(result.entities).toHaveLength(2); // Two detections
  });
});

// Run tests if this file is executed directly
if (require.main === module) {
  // Simple test runner (in real app, use Jest or similar)
  console.log("Running anonymask Node.js tests...");

  const tests = [
    () => {
      const anonymizer = new JsAnonymizer(["email"]);
      const result = anonymizer.anonymize("test@example.com");
      if (!result.anonymized_text.includes("EMAIL_"))
        throw new Error("Email not anonymized");
      console.log("✓ Email anonymization test passed");
    },
    () => {
      const anonymizer = new JsAnonymizer(["email"]);
      const original = "Contact test@example.com";
      const result = anonymizer.anonymize(original);
      const deanonymized = anonymizer.deanonymize(
        result.anonymized_text,
        result.mapping,
      );
      if (deanonymized !== original) throw new Error("Deanonymization failed");
      console.log("✓ Deanonymization test passed");
    },
  ];

  tests.forEach((test) => {
    try {
      test();
    } catch (error) {
      console.error("✗ Test failed:", error.message);
      process.exit(1);
    }
  });

  console.log("All tests passed!");
}
