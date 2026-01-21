use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Représente un schéma JSON Schema 2020-12
/// Basé sur https://json-schema.org/draft/2020-12/json-schema-core
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum JsonSchema2020 {
    /// Schéma booléen (true = accepte tout, false = rejette tout)
    Boolean(bool),
    
    /// Schéma objet
    Object(JsonSchemaObject),
}

/// Schéma JSON Schema 2020-12 sous forme d'objet
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JsonSchemaObject {
    /// Identifie la version du schéma ($schema)
    #[serde(rename = "$schema")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,
    
    /// Identifiant du schéma ($id)
    #[serde(rename = "$id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    
    /// Ancre pour références ($anchor)
    #[serde(rename = "$anchor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anchor: Option<String>,
    
    /// Ancre dynamique ($dynamicAnchor)
    #[serde(rename = "$dynamicAnchor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_anchor: Option<String>,
    
    /// Référence ($ref)
    #[serde(rename = "$ref")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ref_: Option<String>,
    
    /// Référence dynamique ($dynamicRef)
    #[serde(rename = "$dynamicRef")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_ref: Option<String>,
    
    /// Définitions ($defs)
    #[serde(rename = "$defs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub defs: Option<HashMap<String, Box<JsonSchema2020>>>,
    
    /// Type primitif
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<JsonSchemaType>,
    
    /// Énumération
    #[serde(rename = "enum")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enum_: Option<Vec<serde_json::Value>>,
    
    /// Constantes
    #[serde(rename = "const")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub const_: Option<serde_json::Value>,
    
    // === Applicators ===
    
    /// Tous doivent être valides (allOf)
    #[serde(rename = "allOf")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_of: Option<Vec<Box<JsonSchema2020>>>,
    
    /// Au moins un doit être valide (anyOf)
    #[serde(rename = "anyOf")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub any_of: Option<Vec<Box<JsonSchema2020>>>,
    
    /// Un seul doit être valide (oneOf)
    #[serde(rename = "oneOf")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub one_of: Option<Vec<Box<JsonSchema2020>>>,
    
    /// Ne doit pas être valide (not)
    #[serde(rename = "not")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not: Option<Box<JsonSchema2020>>,
    
    /// Si/then/else (if/then/else)
    #[serde(rename = "if")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub if_: Option<Box<JsonSchema2020>>,
    
    #[serde(rename = "then")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub then: Option<Box<JsonSchema2020>>,
    
    #[serde(rename = "else")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub else_: Option<Box<JsonSchema2020>>,
    
    // === Arrays ===
    
    /// Schémas pour les premiers éléments (prefixItems) - JSON Schema 2020-12
    #[serde(rename = "prefixItems")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix_items: Option<Vec<Box<JsonSchema2020>>>,
    
    /// Schéma pour les éléments restants (items) - JSON Schema 2020-12
    /// Peut être un schéma ou false (interdit les éléments supplémentaires)
    #[serde(rename = "items")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<JsonSchemaItems>,
    
    /// Contient au moins un élément qui satisfait ce schéma (contains)
    #[serde(rename = "contains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contains: Option<Box<JsonSchema2020>>,
    
    /// Nombre minimum d'éléments (minItems)
    #[serde(rename = "minItems")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_items: Option<u64>,
    
    /// Nombre maximum d'éléments (maxItems)
    #[serde(rename = "maxItems")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<u64>,
    
    /// Éléments non évalués (unevaluatedItems)
    #[serde(rename = "unevaluatedItems")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unevaluated_items: Option<Box<JsonSchema2020>>,
    
    /// Unique items (uniqueItems)
    #[serde(rename = "uniqueItems")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unique_items: Option<bool>,
    
    // === Objects ===
    
    /// Propriétés requises (properties)
    #[serde(rename = "properties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<HashMap<String, Box<JsonSchema2020>>>,
    
    /// Propriétés optionnelles (pas dans JSON Schema standard, mais compatible)
    #[serde(rename = "optionalProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub optional_properties: Option<HashMap<String, Box<JsonSchema2020>>>,
    
    /// Propriétés avec patterns (patternProperties)
    #[serde(rename = "patternProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pattern_properties: Option<HashMap<String, Box<JsonSchema2020>>>,
    
    /// Propriétés supplémentaires (additionalProperties)
    /// Peut être un schéma, true (accepte tout), ou false (rejette tout)
    #[serde(rename = "additionalProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<JsonSchemaAdditionalProperties>,
    
    /// Propriétés requises (required)
    #[serde(rename = "required")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<Vec<String>>,
    
    /// Propriétés dépendantes (dependentSchemas)
    #[serde(rename = "dependentSchemas")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dependent_schemas: Option<HashMap<String, Box<JsonSchema2020>>>,
    
    /// Propriétés non évaluées (unevaluatedProperties)
    #[serde(rename = "unevaluatedProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unevaluated_properties: Option<Box<JsonSchema2020>>,
    
    /// Nombre minimum de propriétés (minProperties)
    #[serde(rename = "minProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_properties: Option<u64>,
    
    /// Nombre maximum de propriétés (maxProperties)
    #[serde(rename = "maxProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_properties: Option<u64>,
    
    // === Strings ===
    
    /// Longueur minimale (minLength)
    #[serde(rename = "minLength")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_length: Option<u64>,
    
    /// Longueur maximale (maxLength)
    #[serde(rename = "maxLength")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_length: Option<u64>,
    
    /// Pattern regex (pattern)
    #[serde(rename = "pattern")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pattern: Option<String>,
    
    /// Format (format)
    #[serde(rename = "format")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    
    // === Numbers ===
    
    /// Multiple de (multipleOf)
    #[serde(rename = "multipleOf")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multiple_of: Option<f64>,
    
    /// Minimum (minimum)
    #[serde(rename = "minimum")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum: Option<f64>,
    
    /// Maximum (maximum)
    #[serde(rename = "maximum")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum: Option<f64>,
    
    /// Minimum exclusif (exclusiveMinimum)
    #[serde(rename = "exclusiveMinimum")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclusive_minimum: Option<f64>,
    
    /// Maximum exclusif (exclusiveMaximum)
    #[serde(rename = "exclusiveMaximum")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclusive_maximum: Option<f64>,
    
    // === Autres ===
    
    /// Commentaire ($comment)
    #[serde(rename = "$comment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    
    /// Exemples (examples)
    #[serde(rename = "examples")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub examples: Option<Vec<serde_json::Value>>,
    
    /// Description (description)
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    
    /// Titre (title)
    #[serde(rename = "title")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    
    /// Default (default)
    #[serde(rename = "default")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<serde_json::Value>,
    
    /// Deprecated (deprecated)
    #[serde(rename = "deprecated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecated: Option<bool>,
    
    /// Read only (readOnly)
    #[serde(rename = "readOnly")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
    
    /// Write only (writeOnly)
    #[serde(rename = "writeOnly")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub write_only: Option<bool>,
}

/// Type JSON Schema 2020-12
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum JsonSchemaType {
    Null,
    Boolean,
    Object,
    Array,
    Number,
    String,
    Integer,
}

/// Items peut être un schéma ou false
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum JsonSchemaItems {
    Schema(Box<JsonSchema2020>),
    Boolean(bool),
}

/// AdditionalProperties peut être un schéma, true, ou false
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum JsonSchemaAdditionalProperties {
    Schema(Box<JsonSchema2020>),
    Boolean(bool),
}

impl JsonSchema2020 {
    /// Détecte si un JSON est un schéma JSON Schema 2020-12
    pub fn is_json_schema_2020(json: &serde_json::Value) -> bool {
        if let Some(obj) = json.as_object() {
            // Vérifier la présence de $schema pointant vers 2020-12
            if let Some(schema_val) = obj.get("$schema") {
                if let Some(schema_str) = schema_val.as_str() {
                    if schema_str.contains("2020-12") || schema_str.contains("draft/2020-12") {
                        return true;
                    }
                }
            }
            
            // Vérifier la présence de mots-clés spécifiques à JSON Schema 2020-12
            if obj.contains_key("prefixItems") || 
               obj.contains_key("$defs") ||
               obj.contains_key("unevaluatedItems") ||
               obj.contains_key("unevaluatedProperties") ||
               obj.contains_key("$dynamicRef") ||
               obj.contains_key("$dynamicAnchor") {
                return true;
            }
        }
        false
    }
}
