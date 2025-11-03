#!/usr/bin/env node
/**
 * Express middleware example for anonymask.
 * Shows how to integrate anonymization into an Express.js API.
 */

const express = require('express');
const { Anonymizer } = require('@anonymask/core');

const app = express();
const port = 3000;

// Initialize anonymizer
const anonymizer = new Anonymizer(['email', 'phone', 'ssn', 'credit_card']);

// Middleware to parse JSON
app.use(express.json());

// Anonymization middleware
function anonymizeMiddleware(req, res, next) {
    if (req.body && req.body.text) {
        console.log('Anonymizing request body...');

        // Anonymize the incoming text
        const result = anonymizer.anonymize(req.body.text);

        // Store original data for later deanonymization
        req.anonymizationData = {
            original: req.body.text,
            anonymized: result.anonymized_text,
            mapping: result.mapping,
            entities: result.entities
        };

        // Replace request body with anonymized version
        req.body.text = result.anonymized_text;
    }
    next();
}

// Deanonymization middleware for responses
function deanonymizeMiddleware(req, res, next) {
    const originalSend = res.send;

    res.send = function(data) {
        if (req.anonymizationData && typeof data === 'string') {
            console.log('Deanonymizing response...');

            // Deanonymize the response
            const deanonymized = anonymizer.deanonymize(data, req.anonymizationData.mapping);
            data = deanonymized;
        }

        originalSend.call(this, data);
    };

    next();
}

// Routes
app.post('/chat', anonymizeMiddleware, deanonymizeMiddleware, (req, res) => {
    // Simulate LLM processing
    const userMessage = req.body.text;
    console.log('Processing anonymized message:', userMessage);

    // Mock LLM response (in real app, call OpenAI, Anthropic, etc.)
    let response = simulateLLMResponse(userMessage);

    // Response will be automatically deanonymized by middleware
    res.json({
        response: response,
        entities_found: req.anonymizationData ? req.anonymizationData.entities.length : 0
    });
});

app.get('/health', (req, res) => {
    res.json({ status: 'ok', service: 'anonymized-chat-api' });
});

// Mock LLM response function
function simulateLLMResponse(anonymizedMessage) {
    if (anonymizedMessage.includes('EMAIL_')) {
        return "I see you've provided an email address. I'll make sure to EMAIL_A1B2C3D4 it to you.";
    } else if (anonymizedMessage.includes('PHONE_')) {
        return "Your phone number PHONE_X1Y2Z3A4 has been noted for callback.";
    } else {
        return "Thank you for your message. How can I help you today?";
    }
}

// Error handling
app.use((err, req, res, next) => {
    console.error('Error:', err);
    res.status(500).json({ error: 'Internal server error' });
});

// Start server
app.listen(port, () => {
    console.log(`Anonymized chat API listening at http://localhost:${port}`);
    console.log('Try: curl -X POST http://localhost:3000/chat -H "Content-Type: application/json" -d \'{"text": "Contact me at john@email.com"}\'');
});

// Graceful shutdown
process.on('SIGINT', () => {
    console.log('Shutting down server...');
    process.exit(0);
});