use crate::error::JtdError;
use crate::schema::ast::JtdSchema;
use crate::schema::syntax_checks::{detect_circular_references_with_definitions, validate_reference, validate_schema_syntax};
use std::collections::HashSet;
use std::fs;
use std::path::Path;

/// Parse un schéma JTD depuis un fichier JSON
pub fn parse_schema_file(path: &Path) -> Result<JtdSchema, JtdError> {
    let content = fs::read_to_string(path)?;
    parse_schema(&content)
}

/// Parse un schéma JTD depuis une chaîne JSON
pub fn parse_schema(json: &str) -> Result<JtdSchema, JtdError> {
    let schema: JtdSchema = serde_json::from_str(json)?;
    
    // Valider la syntaxe
    validate_schema_syntax(&schema, true)?;
    
    // Vérifier toutes les références
    validate_all_references(&schema)?;
    
    // Détecter les références circulaires
    // On doit passer les définitions racine pour détecter correctement les cycles
    let mut visited = HashSet::new();
    detect_circular_references_with_definitions(&schema, schema.definitions.as_ref(), &mut visited)?;
    
    Ok(schema)
}

/// Valide que toutes les références dans le schéma existent
fn validate_all_references(schema: &JtdSchema) -> Result<(), JtdError> {
    validate_references_recursive(schema, schema.definitions.as_ref(), true)
}

fn validate_references_recursive(
    schema: &JtdSchema,
    root_definitions: Option<&std::collections::HashMap<String, Box<JtdSchema>>>,
    _is_root: bool,
) -> Result<(), JtdError> {
    match &schema.form {
        crate::schema::ast::SchemaForm::Ref { ref_name } => {
            validate_reference(ref_name, root_definitions)?;
            
            // Valider récursivement le schéma référencé
            if let Some(defs) = root_definitions {
                if let Some(ref_schema) = defs.get(ref_name) {
                    validate_references_recursive(ref_schema, Some(defs), false)?;
                }
            }
        }
        crate::schema::ast::SchemaForm::Elements { elements } => {
            validate_references_recursive(elements, root_definitions, false)?;
        }
        crate::schema::ast::SchemaForm::Values { values } => {
            validate_references_recursive(values, root_definitions, false)?;
        }
        crate::schema::ast::SchemaForm::Properties {
            properties,
            optional_properties,
            ..
        } => {
            for prop_schema in properties.values() {
                validate_references_recursive(prop_schema, root_definitions, false)?;
            }
            if let Some(opt_props) = optional_properties {
                for opt_schema in opt_props.values() {
                    validate_references_recursive(opt_schema, root_definitions, false)?;
                }
            }
        }
        crate::schema::ast::SchemaForm::Discriminator { mapping, .. } => {
            for map_schema in mapping.values() {
                validate_references_recursive(map_schema, root_definitions, false)?;
            }
        }
        _ => {}
    }
    
    Ok(())
}
