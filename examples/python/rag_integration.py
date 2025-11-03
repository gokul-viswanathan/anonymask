#!/usr/bin/env python3
"""
RAG (Retrieval-Augmented Generation) integration example.
Shows how to use anonymask to protect PII before sending to LLMs.
"""

from anonymask import Anonymizer
import json

class RAGSystem:
    def __init__(self):
        # Initialize anonymizer for common PII types
        self.anonymizer = Anonymizer(['email', 'phone', 'ssn', 'credit_card'])

    def process_user_query(self, user_query: str) -> dict:
        """
        Process user query: anonymize, send to LLM, deanonymize response.
        """
        print("User query:", user_query)

        # Step 1: Anonymize the user query
        result = self.anonymizer.anonymize(user_query)
        anonymized_query = result[0]
        mapping = result[1]

        print("Anonymized query:", anonymized_query)

        # Step 2: Simulate sending to LLM and getting response
        # In real implementation, this would call your LLM API
        llm_response = self.simulate_llm_response(anonymized_query)

        print("LLM response (anonymized):", llm_response)

        # Step 3: Deanonymize the LLM response
        original_response = self.anonymizer.deanonymize(llm_response, mapping)

        print("LLM response (original):", original_response)

        return {
            "original_query": user_query,
            "anonymized_query": anonymized_query,
            "llm_response": original_response,
            "mapping": mapping
        }

    def simulate_llm_response(self, anonymized_query: str) -> str:
        """
        Simulate LLM response. In real implementation, replace with actual LLM call.
        """
        # Mock responses based on query content
        if "email" in anonymized_query.lower():
            return "I can see you've provided contact information. I'll help you with EMAIL_A1B2C3D4."
        elif "phone" in anonymized_query.lower():
            return "Your phone number PHONE_X1Y2Z3A4 has been noted."
        else:
            return "I understand your query. How can I assist you further?"

def main():
    rag = RAGSystem()

    # Example queries
    queries = [
        "Contact me at john.doe@email.com for updates",
        "My phone is 555-123-4567 and SSN is 123-45-6789",
        "I have a credit card ending in 1234"
    ]

    for i, query in enumerate(queries, 1):
        print(f"\n=== Query {i} ===")
        result = rag.process_user_query(query)
        print()

if __name__ == "__main__":
    main()