use crate::error::JtdError;
use crate::schema::json_schema::JsonSchema2020;
use crate::schema::json_schema_parser::parse_json_schema;
use crate::schema::parser::parse_schema;
use crate::schema::ast::JtdSchema;
use serde_json::Value;
use std::path::Path;
use std::fs;

/// Format de schéma détecté
#[derive(Debug, Clone, PartialEq)]
pub enum SchemaFormat {
    JTD,
    JsonSchema2020,
}

/// Schéma unifié (peut être JTD ou JSON Schema 2020-12)
#[derive(Debug, Clone)]
pub enum UnifiedSchema {
    JTD(JtdSchema),
    JsonSchema2020(JsonSchema2020),
}

/// Parse un schéma en détectant automatiquement le format
pub fn parse_schema_auto(json: &str) -> Result<UnifiedSchema, JtdError> {
    let value: Value = serde_json::from_str(json)?;
    
    // Détecter le format
    if JsonSchema2020::is_json_schema_2020(&value) {
        let schema = parse_json_schema(json)?;
        Ok(UnifiedSchema::JsonSchema2020(schema))
    } else {
        let schema = parse_schema(json)?;
        Ok(UnifiedSchema::JTD(schema))
    }
}

/// Parse un schéma depuis un fichier en détectant automatiquement le format
pub fn parse_schema_file_auto(path: &Path) -> Result<UnifiedSchema, JtdError> {
    let content = fs::read_to_string(path)?;
    parse_schema_auto(&content)
}

/// Détecte le format d'un schéma
pub fn detect_schema_format(json: &str) -> Result<SchemaFormat, JtdError> {
    let value: Value = serde_json::from_str(json)?;
    
    if JsonSchema2020::is_json_schema_2020(&value) {
        Ok(SchemaFormat::JsonSchema2020)
    } else {
        Ok(SchemaFormat::JTD)
    }
}
