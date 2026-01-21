use crate::error::JtdError;
use crate::schema::json_schema::{JsonSchema2020, JsonSchemaObject};
use serde_json::Value;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Parse un schéma JSON Schema 2020-12 depuis un fichier
pub fn parse_json_schema_file(path: &Path) -> Result<JsonSchema2020, JtdError> {
    let content = fs::read_to_string(path)?;
    parse_json_schema(&content)
}

/// Parse un schéma JSON Schema 2020-12 depuis une chaîne JSON
pub fn parse_json_schema(json: &str) -> Result<JsonSchema2020, JtdError> {
    let value: Value = serde_json::from_str(json)?;
    
    // Détecter si c'est un schéma JSON Schema 2020-12
    if !JsonSchema2020::is_json_schema_2020(&value) {
        return Err(JtdError::SchemaSyntaxError(
            "Not a JSON Schema 2020-12 schema".to_string(),
        ));
    }
    
    // Parser le schéma
    let schema: JsonSchema2020 = serde_json::from_value(value)?;
    
    // Valider la syntaxe de base
    validate_json_schema_syntax(&schema)?;
    
    Ok(schema)
}

/// Valide la syntaxe de base d'un schéma JSON Schema 2020-12
fn validate_json_schema_syntax(schema: &JsonSchema2020) -> Result<(), JtdError> {
    match schema {
        JsonSchema2020::Boolean(_) => Ok(()),
        JsonSchema2020::Object(obj) => {
            // Vérifier que $schema pointe vers 2020-12 si présent
            if let Some(ref schema_uri) = obj.schema {
                if !schema_uri.contains("2020-12") && !schema_uri.contains("draft/2020-12") {
                    return Err(JtdError::SchemaSyntaxError(format!(
                        "Schema URI '{}' does not point to JSON Schema 2020-12",
                        schema_uri
                    )));
                }
            }
            
            // Valider les définitions récursivement
            if let Some(ref defs) = obj.defs {
                for (_name, def_schema) in defs {
                    validate_json_schema_syntax(def_schema)?;
                }
            }
            
            // Valider les sous-schémas dans allOf, anyOf, etc.
            if let Some(ref all_of) = obj.all_of {
                for sub_schema in all_of {
                    validate_json_schema_syntax(sub_schema)?;
                }
            }
            
            if let Some(ref any_of) = obj.any_of {
                for sub_schema in any_of {
                    validate_json_schema_syntax(sub_schema)?;
                }
            }
            
            if let Some(ref one_of) = obj.one_of {
                for sub_schema in one_of {
                    validate_json_schema_syntax(sub_schema)?;
                }
            }
            
            if let Some(ref not) = obj.not {
                validate_json_schema_syntax(not)?;
            }
            
            if let Some(ref if_) = obj.if_ {
                validate_json_schema_syntax(if_)?;
            }
            
            if let Some(ref then) = obj.then {
                validate_json_schema_syntax(then)?;
            }
            
            if let Some(ref else_) = obj.else_ {
                validate_json_schema_syntax(else_)?;
            }
            
            // Valider prefixItems
            if let Some(ref prefix_items) = obj.prefix_items {
                for item_schema in prefix_items {
                    validate_json_schema_syntax(item_schema)?;
                }
            }
            
            // Valider items
            if let Some(ref items) = obj.items {
                match items {
                    crate::schema::json_schema::JsonSchemaItems::Schema(s) => {
                        validate_json_schema_syntax(s)?;
                    }
                    crate::schema::json_schema::JsonSchemaItems::Boolean(_) => {}
                }
            }
            
            // Valider contains
            if let Some(ref contains) = obj.contains {
                validate_json_schema_syntax(contains)?;
            }
            
            // Valider unevaluatedItems
            if let Some(ref unevaluated_items) = obj.unevaluated_items {
                validate_json_schema_syntax(unevaluated_items)?;
            }
            
            // Valider properties
            if let Some(ref properties) = obj.properties {
                for (_, prop_schema) in properties {
                    validate_json_schema_syntax(prop_schema)?;
                }
            }
            
            // Valider optionalProperties (extension JTD)
            if let Some(ref optional_properties) = obj.optional_properties {
                for (_, opt_schema) in optional_properties {
                    validate_json_schema_syntax(opt_schema)?;
                }
            }
            
            // Valider patternProperties
            if let Some(ref pattern_properties) = obj.pattern_properties {
                for (_, pattern_schema) in pattern_properties {
                    validate_json_schema_syntax(pattern_schema)?;
                }
            }
            
            // Valider additionalProperties
            if let Some(ref additional_properties) = obj.additional_properties {
                match additional_properties {
                    crate::schema::json_schema::JsonSchemaAdditionalProperties::Schema(s) => {
                        validate_json_schema_syntax(s)?;
                    }
                    crate::schema::json_schema::JsonSchemaAdditionalProperties::Boolean(_) => {}
                }
            }
            
            // Valider dependentSchemas
            if let Some(ref dependent_schemas) = obj.dependent_schemas {
                for (_, dep_schema) in dependent_schemas {
                    validate_json_schema_syntax(dep_schema)?;
                }
            }
            
            // Valider unevaluatedProperties
            if let Some(ref unevaluated_properties) = obj.unevaluated_properties {
                validate_json_schema_syntax(unevaluated_properties)?;
            }
            
            Ok(())
        }
    }
}

/// Résout une référence dans les définitions
pub fn resolve_ref<'a>(
    ref_path: &str,
    root_defs: Option<&'a HashMap<String, Box<JsonSchema2020>>>,
) -> Result<&'a JsonSchema2020, JtdError> {
    // Pour l'instant, on supporte seulement les références vers $defs
    // Format attendu: "#/$defs/Name" ou "Name"
    let ref_name = if ref_path.starts_with("#/$defs/") {
        &ref_path[8..]
    } else if ref_path.starts_with("#/") {
        return Err(JtdError::ReferenceNotFound(format!(
            "Only references to $defs are supported: {}",
            ref_path
        )));
    } else {
        ref_path
    };
    
    if let Some(defs) = root_defs {
        if let Some(schema) = defs.get(ref_name) {
            return Ok(schema);
        }
    }
    
    Err(JtdError::ReferenceNotFound(format!(
        "Reference '{}' not found in $defs",
        ref_name
    )))
}
