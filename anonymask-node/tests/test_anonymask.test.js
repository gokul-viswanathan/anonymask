#!/usr/bin/env node
/**
 * Integration tests for anonymask Node.js bindings.
 */

const { Anonymizer } = require("../index.js");

describe("Anonymizer", () => {
  let anonymizer;

  beforeEach(() => {
    anonymizer = new Anonymizer(["email", "phone"]);
  });

  test("anonymizes email", () => {
    const text = "Contact john@email.com";
    const result = anonymizer.anonymize(text);

    expect(result.anonymizedText).toContain("EMAIL_");
    expect(result.entities).toHaveLength(1);
    expect(result.entities[0].entityType).toBe("email");
    expect(result.entities[0].value).toBe("john@email.com");
  });

  test("anonymizes phone", () => {
    const text = "Call 555-123-4567";
    const result = anonymizer.anonymize(text);

    expect(result.anonymizedText).toContain("PHONE_");
    expect(result.entities).toHaveLength(1);
    expect(result.entities[0].entityType).toBe("phone");
  });

  test("anonymizes multiple entities", () => {
    const text = "Email: user@test.com, Phone: 555-1234";
    const result = anonymizer.anonymize(text);

    expect(result.anonymizedText).toContain("EMAIL_");
    expect(result.anonymizedText).not.toContain("PHONE_");
    expect(result.entities).toHaveLength(1);
  });

  test("deanonymizes correctly", () => {
    const original = "Contact john@email.com today";
    const result = anonymizer.anonymize(original);
    const deanonymized = anonymizer.deanonymize(
      result.anonymizedText,
      result.mapping,
    );

    expect(deanonymized).toBe(original);
  });

  test("handles empty text", () => {
    const result = anonymizer.anonymize("");

    expect(result.anonymizedText).toBe("");
    expect(result.entities).toHaveLength(0);
  });

  test("handles text with no entities", () => {
    const text = "This is a regular message with no PII";
    const result = anonymizer.anonymize(text);

    expect(result.anonymizedText).toBe(text);
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

  test("anonymizes phone short format", () => {
    const text = "Call 555-123";
    const result = anonymizer.anonymize(text);

    expect(result.anonymizedText).toContain("PHONE_");
    expect(result.entities).toHaveLength(1);
    expect(result.entities[0].entityType).toBe("phone");
  });

  test("anonymizes phone multiple formats", () => {
    const text = "Call 555-123-4567 or 555-123";
    const result = anonymizer.anonymize(text);

    expect(result.anonymizedText).toContain("PHONE_");
    expect(result.entities).toHaveLength(2);
  });

  test("anonymizes with custom entities", () => {
    const customEntities = {
      phone: ["555-999-0000"],
      email: ["custom@example.com"]
    };
    const result = anonymizer.anonymizeWithCustom("Contact custom@example.com or call 555-999-0000", customEntities);

    expect(result.anonymizedText).toContain("EMAIL_");
    expect(result.anonymizedText).toContain("PHONE_");
    expect(result.entities).toHaveLength(2);
  });

  test("anonymizes custom entities only", () => {
    const anonymizer = new Anonymizer([]);
    const customEntities = {
      email: ["secret@company.com"]
    };
    const result = anonymizer.anonymizeWithCustom("Send to secret@company.com", customEntities);

    expect(result.anonymizedText).toContain("EMAIL_");
    expect(result.entities).toHaveLength(1);
    expect(result.entities[0].value).toBe("secret@company.com");
  });

  test("backward compatibility", () => {
    const text = "Contact john@email.com";
    const result1 = anonymizer.anonymize(text);
    const result2 = anonymizer.anonymizeWithCustom(text, null);

    expect(result1.anonymizedText).toContain("EMAIL_");
    expect(result2.anonymizedText).toContain("EMAIL_");
    expect(result1.entities.length).toBe(result2.entities.length);
  });

  test("custom entity types", () => {
    const anonymizer = new Anonymizer([]);
    const customEntities = {
      name: ["John Doe"],
      company: ["Acme Corp"]
    };
    const result = anonymizer.anonymizeWithCustom("John Doe works at Acme Corp", customEntities);

    expect(result.anonymizedText).toContain("NAME_");
    expect(result.anonymizedText).toContain("COMPANY_");
    expect(result.entities).toHaveLength(2);
    expect(result.entities[0].entityType).toBe("name");
    expect(result.entities[0].value).toBe("John Doe");
    expect(result.entities[1].entityType).toBe("company");
    expect(result.entities[1].value).toBe("Acme Corp");
  });
});

describe("AnonymizerConfig (v2.0.0 features)", () => {
  test("creates config with default values", () => {
    const config = {
      caseSensitive: true,
      wordBoundaryCheck: false,
      placeholderFormat: "standard",
      maxEntities: 0
    };
    const anonymizer = new Anonymizer(["email"], config);
    const result = anonymizer.anonymize("test@example.com");

    expect(result.anonymizedText).toContain("EMAIL_");
    expect(result.entities).toHaveLength(1);
  });

  test("uses short placeholder format", () => {
    const config = {
      caseSensitive: true,
      wordBoundaryCheck: false,
      placeholderFormat: "short",
      maxEntities: 0
    };
    const anonymizer = new Anonymizer(["email"], config);
    const result = anonymizer.anonymize("Contact user@example.com");

    // Short format should use counter: EMAIL_1, EMAIL_2, etc.
    expect(result.anonymizedText).toContain("EMAIL_1");
    expect(result.entities).toHaveLength(1);
  });

  test("uses custom placeholder template", () => {
    const config = {
      caseSensitive: true,
      wordBoundaryCheck: false,
      placeholderFormat: "[{type}:{counter}]",
      maxEntities: 0
    };
    const anonymizer = new Anonymizer(["email"], config);
    const result = anonymizer.anonymize("Email: test@example.com");

    // Custom format should match template
    expect(result.anonymizedText).toContain("[EMAIL:1]");
    expect(result.entities).toHaveLength(1);
  });

  test("handles case sensitivity in custom entities", () => {
    const configSensitive = {
      caseSensitive: true,
      wordBoundaryCheck: false,
      placeholderFormat: "standard",
      maxEntities: 0
    };
    const anonymizer = new Anonymizer([], configSensitive);
    const customEntities = { name: ["John"] };
    const result = anonymizer.anonymizeWithCustom("john and John are here", customEntities);

    // Should only match "John" (case-sensitive)
    const nameCount = (result.anonymizedText.match(/NAME_/g) || []).length;
    expect(nameCount).toBe(1);
  });

  test("supports different formats for multiple entities", () => {
    const text = "Email: user@test.com, Phone: 555-123-4567";

    // Standard format
    const anonymizer1 = new Anonymizer(["email", "phone"], {
      caseSensitive: true,
      wordBoundaryCheck: false,
      placeholderFormat: "standard",
      maxEntities: 0
    });
    const result1 = anonymizer1.anonymize(text);
    expect(result1.anonymizedText).toContain("EMAIL_");
    expect(result1.anonymizedText).toContain("PHONE_");

    // Short format
    const anonymizer2 = new Anonymizer(["email", "phone"], {
      caseSensitive: true,
      wordBoundaryCheck: false,
      placeholderFormat: "short",
      maxEntities: 0
    });
    const result2 = anonymizer2.anonymize(text);
    const hasEmail1or2 = result2.anonymizedText.includes("EMAIL_1") || result2.anonymizedText.includes("EMAIL_2");
    const hasPhone1or2 = result2.anonymizedText.includes("PHONE_1") || result2.anonymizedText.includes("PHONE_2");
    expect(hasEmail1or2).toBe(true);
    expect(hasPhone1or2).toBe(true);

    // Custom format
    const anonymizer3 = new Anonymizer(["email", "phone"], {
      caseSensitive: true,
      wordBoundaryCheck: false,
      placeholderFormat: "<{type}-{counter}>",
      maxEntities: 0
    });
    const result3 = anonymizer3.anonymize(text);
    expect(result3.anonymizedText).toContain("<EMAIL-");
    expect(result3.anonymizedText).toContain("<PHONE-");
  });

  test("maintains backward compatibility without config", () => {
    // Old way (still supported)
    const anonymizerOld = new Anonymizer(["email"]);

    // New way with explicit default config
    const anonymizerNew = new Anonymizer(["email"], {
      caseSensitive: true,
      wordBoundaryCheck: false,
      placeholderFormat: "standard",
      maxEntities: 0
    });

    const text = "Contact user@example.com";
    const resultOld = anonymizerOld.anonymize(text);
    const resultNew = anonymizerNew.anonymize(text);

    // Both should work and detect email
    expect(resultOld.anonymizedText).toContain("EMAIL_");
    expect(resultNew.anonymizedText).toContain("EMAIL_");
    expect(resultOld.entities).toHaveLength(1);
    expect(resultNew.entities).toHaveLength(1);
  });

  test("custom template with UUID placeholder", () => {
    const config = {
      caseSensitive: true,
      wordBoundaryCheck: false,
      placeholderFormat: "<<{type}_{uuid}>>",
      maxEntities: 0
    };
    const anonymizer = new Anonymizer(["email"], config);
    const result = anonymizer.anonymize("test@example.com");

    // Should contain template structure
    expect(result.anonymizedText).toContain("<<EMAIL_");
    expect(result.anonymizedText).toContain(">>");
  });
});

// Run tests if this file is executed directly
if (require.main === module) {
  // Simple test runner (in real app, use Jest or similar)
  console.log("Running anonymask Node.js tests...");

  const tests = [
    () => {
      const anonymizer = new Anonymizer(["email"]);
      const result = anonymizer.anonymize("test@example.com");
      if (!result.anonymizedText.includes("EMAIL_"))
        throw new Error("Email not anonymized");
      console.log("✓ Email anonymization test passed");
    },
    () => {
      const anonymizer = new Anonymizer(["email"]);
      const original = "Contact test@example.com";
      const result = anonymizer.anonymize(original);
      const deanonymized = anonymizer.deanonymize(
        result.anonymizedText,
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
