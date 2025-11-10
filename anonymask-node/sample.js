const {
  Anonymizer,
} = require("../anonymask-node/anonymask.linux-x64-gnu.node");

const anonymizer = new Anonymizer(["email", "phone"]);

const text = "Contact john@email.com or call 555-1234";
const result = anonymizer.anonymize(text);

console.log("Result:", result);
console.log("Original:", text);
console.log("Anonymized:", result.anonymizedText);
console.log("Mapping:", result.mapping);

const original = anonymizer.deanonymize(result.anonymizedText, result.mapping);
console.log("Deanonymized:", original);
