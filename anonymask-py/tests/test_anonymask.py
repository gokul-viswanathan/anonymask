#!/usr/bin/env python3
"""
Integration tests for anonymask Python bindings.
"""

import pytest
from anonymask import Anonymizer


class TestAnonymizer:
    def setup_method(self):
        self.anonymizer = Anonymizer(["email", "phone"])

    def test_anonymize_email(self):
        text = "Contact john@email.com"
        result = self.anonymizer.anonymize(text)
        print("anonymask", result)

        assert "EMAIL_" in result[0]
        assert len(result[2]) == 1  # entities
        # assert result[2][0]['entity_type'] == 'email'
        # assert result[2][0]["value"] == "john@email.com"

    def test_anonymize_phone(self):
        text = "Call 555-123-4567"
        result = self.anonymizer.anonymize(text)

        assert "PHONE_" in result[0]
        assert len(result[2]) == 1
        # assert result[2][0]['entity_type'] == 'phone'

    def test_anonymize_multiple_entities(self):
        text = "Email: user@test.com, Phone: 555-1234"
        result = self.anonymizer.anonymize(text)

        assert "EMAIL_" in result[0]
        assert "PHONE_" not in result[0]
        assert len(result[2]) == 1

    def test_deanonymize(self):
        original = "Contact john@email.com today"
        result = self.anonymizer.anonymize(original)
        deanonymized = self.anonymizer.deanonymize(result[0], result[1])

        assert deanonymized == original

    def test_empty_text(self):
        result = self.anonymizer.anonymize("")
        assert result[0] == ""
        assert len(result[2]) == 0

    def test_no_entities(self):
        text = "This is a regular message with no PII"
        result = self.anonymizer.anonymize(text)

        assert result[0] == text
        assert len(result[2]) == 0

    def test_duplicate_entities(self):
        text = "Contact john@email.com or reach out to john@email.com again"
        result = self.anonymizer.anonymize(text)

        # Should use same placeholder for duplicate email
        email_placeholders = [k for k in result[1].keys() if k.startswith("EMAIL_")]
        assert len(email_placeholders) == 1
        assert len(result[2]) == 2  # Two entity detections


if __name__ == "__main__":
    pytest.main([__file__])
