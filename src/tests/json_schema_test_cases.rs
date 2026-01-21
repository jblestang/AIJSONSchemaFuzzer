use crate::error::ValidationError;
use serde_json::json;

/// Représente un cas de test pour JSON Schema 2020-12
pub struct JsonSchemaTestCase {
    pub name: &'static str,
    pub schema: &'static str,
    pub instance: serde_json::Value,
    pub should_be_valid: bool,
    pub expected_errors: Option<Vec<ValidationError>>,
}

// Helper pour créer des schémas avec $ sans problème de macro
fn make_schema(s: &str) -> &'static str {
    Box::leak(s.to_string().into_boxed_str())
}

/// Tous les cas de test pour JSON Schema 2020-12
pub fn all_json_schema_test_cases() -> Vec<JsonSchemaTestCase> {
    let mut cases = Vec::new();
    
    // Tests pour prefixItems
    cases.push(JsonSchemaTestCase {
        name: "prefixItems validates tuple correctly",
        schema: r#"{"$schema": "https://json-schema.org/draft/2020-12/schema", "prefixItems": [{"type": "string"}, {"type": "number"}]}"#,
        instance: json!(["hello", 42]),
        should_be_valid: true,
        expected_errors: None,
    });
    
    cases.push(JsonSchemaTestCase {
        name: "prefixItems rejects wrong type at position",
        schema: r#"{"$schema": "https://json-schema.org/draft/2020-12/schema", "prefixItems": [{"type": "string"}, {"type": "number"}]}"#,
        instance: json!([42, "hello"]),
        should_be_valid: false,
        expected_errors: Some(vec![
            ValidationError {
                instance_path: "/0".to_string(),
                schema_path: "/prefixItems/0/type".to_string(),
            },
            ValidationError {
                instance_path: "/1".to_string(),
                schema_path: "/prefixItems/1/type".to_string(),
            },
        ]),
    });
    
    cases.push(JsonSchemaTestCase {
        name: "prefixItems with items false rejects extra items",
        schema: r#"{"$schema": "https://json-schema.org/draft/2020-12/schema", "prefixItems": [{"type": "string"}], "items": false}"#,
        instance: json!(["hello", "extra"]),
        should_be_valid: false,
        expected_errors: Some(vec![ValidationError {
            instance_path: "/1".to_string(),
            schema_path: "/items".to_string(),
        }]),
    });
    
    cases.push(JsonSchemaTestCase {
        name: "prefixItems with items schema validates extra items",
        schema: r#"{"$schema": "https://json-schema.org/draft/2020-12/schema", "prefixItems": [{"type": "string"}], "items": {"type": "number"}}"#,
        instance: json!(["hello", 1, 2, 3]),
        should_be_valid: true,
        expected_errors: None,
    });
    
    // Tests pour patternProperties
    cases.push(JsonSchemaTestCase {
        name: "patternProperties validates matching keys",
        schema: r#"{"$schema": "https://json-schema.org/draft/2020-12/schema", "patternProperties": {"^x-": {"type": "string"}}}"#,
        instance: json!({"x-foo": "bar", "x-baz": "qux"}),
        should_be_valid: true,
        expected_errors: None,
    });
    
    cases.push(JsonSchemaTestCase {
        name: "patternProperties rejects non-matching type",
        schema: r#"{"$schema": "https://json-schema.org/draft/2020-12/schema", "patternProperties": {"^x-": {"type": "string"}}}"#,
        instance: json!({"x-foo": 42}),
        should_be_valid: false,
        expected_errors: Some(vec![ValidationError {
            instance_path: "/x-foo".to_string(),
            schema_path: "/patternProperties/^x-/type".to_string(),
        }]),
    });
    
    // Tests pour allOf
    cases.push(JsonSchemaTestCase {
        name: "allOf validates when all schemas are valid",
        schema: r#"{"$schema": "https://json-schema.org/draft/2020-12/schema", "allOf": [{"type": "string"}, {"minLength": 3}]}"#,
        instance: json!("hello"),
        should_be_valid: true,
        expected_errors: None,
    });
    
    cases.push(JsonSchemaTestCase {
        name: "allOf rejects when one schema is invalid",
        schema: r#"{"$schema": "https://json-schema.org/draft/2020-12/schema", "allOf": [{"type": "string"}, {"minLength": 10}]}"#,
        instance: json!("hi"),
        should_be_valid: false,
        expected_errors: Some(vec![ValidationError {
            instance_path: "".to_string(),
            schema_path: "/allOf/1/minLength".to_string(),
        }]),
    });
    
    // Tests pour anyOf
    cases.push(JsonSchemaTestCase {
        name: "anyOf validates when at least one schema is valid",
        schema: r#"{"$schema": "https://json-schema.org/draft/2020-12/schema", "anyOf": [{"type": "string"}, {"type": "number"}]}"#,
        instance: json!("hello"),
        should_be_valid: true,
        expected_errors: None,
    });
    
    cases.push(JsonSchemaTestCase {
        name: "anyOf rejects when no schema is valid",
        schema: r#"{"$schema": "https://json-schema.org/draft/2020-12/schema", "anyOf": [{"type": "string"}, {"type": "number"}]}"#,
        instance: json!(true),
        should_be_valid: false,
        expected_errors: Some(vec![
            ValidationError {
                instance_path: "".to_string(),
                schema_path: "/anyOf/0/type".to_string(),
            },
            ValidationError {
                instance_path: "".to_string(),
                schema_path: "/anyOf/1/type".to_string(),
            },
        ]),
    });
    
    // Tests pour oneOf
    cases.push(JsonSchemaTestCase {
        name: "oneOf validates when exactly one schema is valid",
        schema: r#"{"$schema": "https://json-schema.org/draft/2020-12/schema", "oneOf": [{"type": "string"}, {"type": "number"}]}"#,
        instance: json!("hello"),
        should_be_valid: true,
        expected_errors: None,
    });
    
    cases.push(JsonSchemaTestCase {
        name: "oneOf rejects when multiple schemas are valid",
        schema: r#"{"$schema": "https://json-schema.org/draft/2020-12/schema", "oneOf": [{"type": "string"}, {"minLength": 1}]}"#,
        instance: json!("hello"),
        should_be_valid: false,
        expected_errors: Some(vec![ValidationError {
            instance_path: "".to_string(),
            schema_path: "/oneOf".to_string(),
        }]),
    });
    
    // Tests pour $ref
    // Utiliser json! pour construire le schéma puis sérialiser pour éviter les problèmes avec $ dans les raw strings
    let ref_schema_value = json!({
        "$schema": "https://json-schema.org/draft/2020-12/schema",
        "$defs": {
            "Person": {
                "type": "object",
                "properties": {
                    "name": {"type": "string"}
                }
            }
        },
        "$ref": "#/$defs/Person"
    });
    let ref_schema_str = serde_json::to_string(&ref_schema_value).unwrap();
    cases.push(JsonSchemaTestCase {
        name: "$ref resolves to $defs correctly",
        schema: Box::leak(ref_schema_str.into_boxed_str()),
        instance: json!({"name": "Alice"}),
        should_be_valid: true,
        expected_errors: None,
    });
    
    // Tests pour minItems/maxItems
    cases.push(JsonSchemaTestCase {
        name: "minItems validates minimum array length",
        schema: r#"{"$schema": "https://json-schema.org/draft/2020-12/schema", "type": "array", "minItems": 2}"#,
        instance: json!([1, 2, 3]),
        should_be_valid: true,
        expected_errors: None,
    });
    
    cases.push(JsonSchemaTestCase {
        name: "minItems rejects array too short",
        schema: r#"{"$schema": "https://json-schema.org/draft/2020-12/schema", "type": "array", "minItems": 2}"#,
        instance: json!([1]),
        should_be_valid: false,
        expected_errors: Some(vec![ValidationError {
            instance_path: "".to_string(),
            schema_path: "/minItems".to_string(),
        }]),
    });
    
    cases.push(JsonSchemaTestCase {
        name: "maxItems validates maximum array length",
        schema: r#"{"$schema": "https://json-schema.org/draft/2020-12/schema", "type": "array", "maxItems": 2}"#,
        instance: json!([1, 2]),
        should_be_valid: true,
        expected_errors: None,
    });
    
    cases.push(JsonSchemaTestCase {
        name: "maxItems rejects array too long",
        schema: r#"{"$schema": "https://json-schema.org/draft/2020-12/schema", "type": "array", "maxItems": 2}"#,
        instance: json!([1, 2, 3]),
        should_be_valid: false,
        expected_errors: Some(vec![ValidationError {
            instance_path: "".to_string(),
            schema_path: "/maxItems".to_string(),
        }]),
    });
    
    // Tests pour pattern
    cases.push(JsonSchemaTestCase {
        name: "pattern validates string matching regex",
        schema: r#"{"$schema": "https://json-schema.org/draft/2020-12/schema", "type": "string", "pattern": "^[a-z]+$"}"#,
        instance: json!("hello"),
        should_be_valid: true,
        expected_errors: None,
    });
    
    cases.push(JsonSchemaTestCase {
        name: "pattern rejects string not matching regex",
        schema: r#"{"$schema": "https://json-schema.org/draft/2020-12/schema", "type": "string", "pattern": "^[a-z]+$"}"#,
        instance: json!("Hello"),
        should_be_valid: false,
        expected_errors: Some(vec![ValidationError {
            instance_path: "".to_string(),
            schema_path: "/pattern".to_string(),
        }]),
    });
    
    // Tests pour minLength/maxLength
    cases.push(JsonSchemaTestCase {
        name: "minLength validates minimum string length",
        schema: r#"{"$schema": "https://json-schema.org/draft/2020-12/schema", "type": "string", "minLength": 3}"#,
        instance: json!("hello"),
        should_be_valid: true,
        expected_errors: None,
    });
    
    cases.push(JsonSchemaTestCase {
        name: "minLength rejects string too short",
        schema: r#"{"$schema": "https://json-schema.org/draft/2020-12/schema", "type": "string", "minLength": 3}"#,
        instance: json!("hi"),
        should_be_valid: false,
        expected_errors: Some(vec![ValidationError {
            instance_path: "".to_string(),
            schema_path: "/minLength".to_string(),
        }]),
    });
    
    cases.push(JsonSchemaTestCase {
        name: "maxLength validates maximum string length",
        schema: r#"{"$schema": "https://json-schema.org/draft/2020-12/schema", "type": "string", "maxLength": 5}"#,
        instance: json!("hello"),
        should_be_valid: true,
        expected_errors: None,
    });
    
    cases.push(JsonSchemaTestCase {
        name: "maxLength rejects string too long",
        schema: r#"{"$schema": "https://json-schema.org/draft/2020-12/schema", "type": "string", "maxLength": 5}"#,
        instance: json!("hello world"),
        should_be_valid: false,
        expected_errors: Some(vec![ValidationError {
            instance_path: "".to_string(),
            schema_path: "/maxLength".to_string(),
        }]),
    });
    
    // Tests pour minimum/maximum
    cases.push(JsonSchemaTestCase {
        name: "minimum validates number >= minimum",
        schema: r#"{"$schema": "https://json-schema.org/draft/2020-12/schema", "type": "number", "minimum": 10}"#,
        instance: json!(15),
        should_be_valid: true,
        expected_errors: None,
    });
    
    cases.push(JsonSchemaTestCase {
        name: "minimum rejects number < minimum",
        schema: r#"{"$schema": "https://json-schema.org/draft/2020-12/schema", "type": "number", "minimum": 10}"#,
        instance: json!(5),
        should_be_valid: false,
        expected_errors: Some(vec![ValidationError {
            instance_path: "".to_string(),
            schema_path: "/minimum".to_string(),
        }]),
    });
    
    cases.push(JsonSchemaTestCase {
        name: "maximum validates number <= maximum",
        schema: r#"{"$schema": "https://json-schema.org/draft/2020-12/schema", "type": "number", "maximum": 10}"#,
        instance: json!(5),
        should_be_valid: true,
        expected_errors: None,
    });
    
    cases.push(JsonSchemaTestCase {
        name: "maximum rejects number > maximum",
        schema: r#"{"$schema": "https://json-schema.org/draft/2020-12/schema", "type": "number", "maximum": 10}"#,
        instance: json!(15),
        should_be_valid: false,
        expected_errors: Some(vec![ValidationError {
            instance_path: "".to_string(),
            schema_path: "/maximum".to_string(),
        }]),
    });
    
    // Tests pour contains
    cases.push(JsonSchemaTestCase {
        name: "contains validates array with matching element",
        schema: r#"{"$schema": "https://json-schema.org/draft/2020-12/schema", "type": "array", "contains": {"type": "string"}}"#,
        instance: json!([1, 2, "hello", 3]),
        should_be_valid: true,
        expected_errors: None,
    });
    
    cases.push(JsonSchemaTestCase {
        name: "contains rejects array without matching element",
        schema: r#"{"$schema": "https://json-schema.org/draft/2020-12/schema", "type": "array", "contains": {"type": "string"}}"#,
        instance: json!([1, 2, 3]),
        should_be_valid: false,
        expected_errors: Some(vec![ValidationError {
            instance_path: "".to_string(),
            schema_path: "/contains".to_string(),
        }]),
    });
    
    // Tests pour uniqueItems
    cases.push(JsonSchemaTestCase {
        name: "uniqueItems validates array with unique items",
        schema: r#"{"$schema": "https://json-schema.org/draft/2020-12/schema", "type": "array", "uniqueItems": true}"#,
        instance: json!([1, 2, 3]),
        should_be_valid: true,
        expected_errors: None,
    });
    
    cases.push(JsonSchemaTestCase {
        name: "uniqueItems rejects array with duplicates",
        schema: r#"{"$schema": "https://json-schema.org/draft/2020-12/schema", "type": "array", "uniqueItems": true}"#,
        instance: json!([1, 2, 1]),
        should_be_valid: false,
        expected_errors: Some(vec![ValidationError {
            instance_path: "/2".to_string(),
            schema_path: "/uniqueItems".to_string(),
        }]),
    });
    
    // Tests pour required
    cases.push(JsonSchemaTestCase {
        name: "required validates object with all required properties",
        schema: r#"{"$schema": "https://json-schema.org/draft/2020-12/schema", "type": "object", "properties": {"name": {"type": "string"}, "age": {"type": "number"}}, "required": ["name", "age"]}"#,
        instance: json!({"name": "Alice", "age": 30}),
        should_be_valid: true,
        expected_errors: None,
    });
    
    cases.push(JsonSchemaTestCase {
        name: "required rejects object missing required property",
        schema: r#"{"$schema": "https://json-schema.org/draft/2020-12/schema", "type": "object", "properties": {"name": {"type": "string"}, "age": {"type": "number"}}, "required": ["name", "age"]}"#,
        instance: json!({"name": "Alice"}),
        should_be_valid: false,
        expected_errors: Some(vec![ValidationError {
            instance_path: "".to_string(),
            schema_path: "/properties/age".to_string(),
        }]),
    });
    
    // Tests pour minProperties/maxProperties
    cases.push(JsonSchemaTestCase {
        name: "minProperties validates object with enough properties",
        schema: r#"{"$schema": "https://json-schema.org/draft/2020-12/schema", "type": "object", "minProperties": 2}"#,
        instance: json!({"a": 1, "b": 2}),
        should_be_valid: true,
        expected_errors: None,
    });
    
    cases.push(JsonSchemaTestCase {
        name: "minProperties rejects object with too few properties",
        schema: r#"{"$schema": "https://json-schema.org/draft/2020-12/schema", "type": "object", "minProperties": 2}"#,
        instance: json!({"a": 1}),
        should_be_valid: false,
        expected_errors: Some(vec![ValidationError {
            instance_path: "".to_string(),
            schema_path: "/minProperties".to_string(),
        }]),
    });
    
    cases.push(JsonSchemaTestCase {
        name: "maxProperties validates object with not too many properties",
        schema: r#"{"$schema": "https://json-schema.org/draft/2020-12/schema", "type": "object", "maxProperties": 2}"#,
        instance: json!({"a": 1, "b": 2}),
        should_be_valid: true,
        expected_errors: None,
    });
    
    cases.push(JsonSchemaTestCase {
        name: "maxProperties rejects object with too many properties",
        schema: r#"{"$schema": "https://json-schema.org/draft/2020-12/schema", "type": "object", "maxProperties": 2}"#,
        instance: json!({"a": 1, "b": 2, "c": 3}),
        should_be_valid: false,
        expected_errors: Some(vec![ValidationError {
            instance_path: "".to_string(),
            schema_path: "/maxProperties".to_string(),
        }]),
    });
    
    // Tests pour multipleOf
    cases.push(JsonSchemaTestCase {
        name: "multipleOf validates number that is multiple",
        schema: r#"{"$schema": "https://json-schema.org/draft/2020-12/schema", "type": "number", "multipleOf": 2}"#,
        instance: json!(4),
        should_be_valid: true,
        expected_errors: None,
    });
    
    cases.push(JsonSchemaTestCase {
        name: "multipleOf rejects number that is not multiple",
        schema: r#"{"$schema": "https://json-schema.org/draft/2020-12/schema", "type": "number", "multipleOf": 2}"#,
        instance: json!(5),
        should_be_valid: false,
        expected_errors: Some(vec![ValidationError {
            instance_path: "".to_string(),
            schema_path: "/multipleOf".to_string(),
        }]),
    });
    
    // Tests pour exclusiveMinimum/exclusiveMaximum
    cases.push(JsonSchemaTestCase {
        name: "exclusiveMinimum validates number > minimum",
        schema: r#"{"$schema": "https://json-schema.org/draft/2020-12/schema", "type": "number", "exclusiveMinimum": 10}"#,
        instance: json!(15),
        should_be_valid: true,
        expected_errors: None,
    });
    
    cases.push(JsonSchemaTestCase {
        name: "exclusiveMinimum rejects number <= minimum",
        schema: r#"{"$schema": "https://json-schema.org/draft/2020-12/schema", "type": "number", "exclusiveMinimum": 10}"#,
        instance: json!(10),
        should_be_valid: false,
        expected_errors: Some(vec![ValidationError {
            instance_path: "".to_string(),
            schema_path: "/exclusiveMinimum".to_string(),
        }]),
    });
    
    cases.push(JsonSchemaTestCase {
        name: "exclusiveMaximum validates number < maximum",
        schema: r#"{"$schema": "https://json-schema.org/draft/2020-12/schema", "type": "number", "exclusiveMaximum": 10}"#,
        instance: json!(5),
        should_be_valid: true,
        expected_errors: None,
    });
    
    cases.push(JsonSchemaTestCase {
        name: "exclusiveMaximum rejects number >= maximum",
        schema: r#"{"$schema": "https://json-schema.org/draft/2020-12/schema", "type": "number", "exclusiveMaximum": 10}"#,
        instance: json!(10),
        should_be_valid: false,
        expected_errors: Some(vec![ValidationError {
            instance_path: "".to_string(),
            schema_path: "/exclusiveMaximum".to_string(),
        }]),
    });
    
    // Tests pour const
    cases.push(JsonSchemaTestCase {
        name: "const validates exact value",
        schema: r#"{"$schema": "https://json-schema.org/draft/2020-12/schema", "const": 42}"#,
        instance: json!(42),
        should_be_valid: true,
        expected_errors: None,
    });
    
    cases.push(JsonSchemaTestCase {
        name: "const rejects different value",
        schema: r#"{"$schema": "https://json-schema.org/draft/2020-12/schema", "const": 42}"#,
        instance: json!(43),
        should_be_valid: false,
        expected_errors: Some(vec![ValidationError {
            instance_path: "".to_string(),
            schema_path: "/const".to_string(),
        }]),
    });
    
    // Tests pour not
    cases.push(JsonSchemaTestCase {
        name: "not validates when schema is invalid",
        schema: r#"{"$schema": "https://json-schema.org/draft/2020-12/schema", "not": {"type": "string"}}"#,
        instance: json!(42),
        should_be_valid: true,
        expected_errors: None,
    });
    
    cases.push(JsonSchemaTestCase {
        name: "not rejects when schema is valid",
        schema: r#"{"$schema": "https://json-schema.org/draft/2020-12/schema", "not": {"type": "string"}}"#,
        instance: json!("hello"),
        should_be_valid: false,
        expected_errors: Some(vec![ValidationError {
            instance_path: "".to_string(),
            schema_path: "/not".to_string(),
        }]),
    });
    
    // Tests pour if/then/else
    cases.push(JsonSchemaTestCase {
        name: "if/then validates when condition is met",
        schema: r#"{"$schema": "https://json-schema.org/draft/2020-12/schema", "if": {"type": "string"}, "then": {"minLength": 3}}"#,
        instance: json!("hello"),
        should_be_valid: true,
        expected_errors: None,
    });
    
    cases.push(JsonSchemaTestCase {
        name: "if/then rejects when condition is met but then fails",
        schema: r#"{"$schema": "https://json-schema.org/draft/2020-12/schema", "if": {"type": "string"}, "then": {"minLength": 10}}"#,
        instance: json!("hi"),
        should_be_valid: false,
        expected_errors: Some(vec![ValidationError {
            instance_path: "".to_string(),
            schema_path: "/then/minLength".to_string(),
        }]),
    });
    
    cases.push(JsonSchemaTestCase {
        name: "if/else validates when condition is not met",
        schema: r#"{"$schema": "https://json-schema.org/draft/2020-12/schema", "if": {"type": "string"}, "else": {"type": "number"}}"#,
        instance: json!(42),
        should_be_valid: true,
        expected_errors: None,
    });
    
    cases.push(JsonSchemaTestCase {
        name: "if/else rejects when condition is not met and else fails",
        schema: r#"{"$schema": "https://json-schema.org/draft/2020-12/schema", "if": {"type": "string"}, "else": {"type": "number"}}"#,
        instance: json!(true),
        should_be_valid: false,
        expected_errors: Some(vec![ValidationError {
            instance_path: "".to_string(),
            schema_path: "/else/type".to_string(),
        }]),
    });
    
    cases
}
