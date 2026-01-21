use crate::error::JtdError;
use crate::schema::ast::{JtdSchema, SchemaForm};
use std::collections::HashSet;

/// Vérifie la syntaxe d'un schéma JTD selon la RFC 8927
pub fn validate_schema_syntax(schema: &JtdSchema, is_root: bool) -> Result<(), JtdError> {
    // 1. Vérifier que definitions n'apparaît qu'au niveau racine
    if !is_root && schema.definitions.is_some() {
        return Err(JtdError::SchemaSyntaxError(
            "definitions can only appear at root level".to_string(),
        ));
    }
    
    // 2. Vérifier que les formes sont mutuellement exclusives
    validate_form_exclusivity(schema)?;
    
    // 3. Vérifier les contraintes spécifiques à chaque forme
    match &schema.form {
        SchemaForm::Ref { ref_name: _ } => {
            // ref peut être utilisé à n'importe quel niveau, il doit juste référencer
            // une définition dans definitions au niveau racine
            // La vérification de l'existence de la référence est faite dans validate_all_references
        }
        SchemaForm::Type { type_name: _ } => {
            // Le type est déjà validé par la désérialisation
        }
        SchemaForm::Enum { enum_values } => {
            validate_enum(enum_values)?;
        }
        SchemaForm::Elements { elements } => {
            validate_schema_syntax(elements, false)?;
        }
        SchemaForm::Values { values } => {
            validate_schema_syntax(values, false)?;
        }
        SchemaForm::Properties {
            properties,
            optional_properties,
            ..
        } => {
            for (_, prop_schema) in properties {
                validate_schema_syntax(prop_schema, false)?;
            }
            if let Some(opt_props) = optional_properties {
                for (_, opt_schema) in opt_props {
                    validate_schema_syntax(opt_schema, false)?;
                }
            }
        }
        SchemaForm::Discriminator { mapping, .. } => {
            for (_, map_schema) in mapping {
                validate_schema_syntax(map_schema, false)?;
            }
        }
        SchemaForm::Empty {} => {}
    }
    
    // 4. Vérifier les définitions récursivement
    if let Some(definitions) = &schema.definitions {
        for (name, def_schema) in definitions {
            validate_schema_syntax(def_schema, false)?;
        }
    }
    
    Ok(())
}

/// Vérifie que les formes sont mutuellement exclusives
fn validate_form_exclusivity(schema: &JtdSchema) -> Result<(), JtdError> {
    let mut form_count = 0;
    
    if matches!(schema.form, SchemaForm::Empty {}) {
        form_count += 1;
    }
    if matches!(schema.form, SchemaForm::Ref { .. }) {
        form_count += 1;
    }
    if matches!(schema.form, SchemaForm::Type { .. }) {
        form_count += 1;
    }
    if matches!(schema.form, SchemaForm::Enum { .. }) {
        form_count += 1;
    }
    if matches!(schema.form, SchemaForm::Elements { .. }) {
        form_count += 1;
    }
    if matches!(schema.form, SchemaForm::Values { .. }) {
        form_count += 1;
    }
    if matches!(schema.form, SchemaForm::Properties { .. }) {
        form_count += 1;
    }
    if matches!(schema.form, SchemaForm::Discriminator { .. }) {
        form_count += 1;
    }
    
    if form_count != 1 {
        return Err(JtdError::SchemaSyntaxError(
            "Schema must match exactly one form".to_string(),
        ));
    }
    
    Ok(())
}

/// Valide qu'un enum est non vide et sans doublons
fn validate_enum(enum_values: &[String]) -> Result<(), JtdError> {
    if enum_values.is_empty() {
        return Err(JtdError::InvalidEnum("enum must be non-empty".to_string()));
    }
    
    // Vérifier les doublons (après normalisation des échappements)
    let mut seen = HashSet::new();
    for value in enum_values {
        // Normaliser les échappements selon RFC 8259
        let normalized = normalize_string(value);
        if !seen.insert(normalized) {
            return Err(JtdError::InvalidEnum(
                format!("enum contains duplicate value: {}", value),
            ));
        }
    }
    
    Ok(())
}

/// Normalise une chaîne pour la comparaison (gère les échappements)
fn normalize_string(s: &str) -> String {
    // Simplification: pour une implémentation complète, il faudrait
    // parser les échappements JSON selon RFC 8259
    s.to_string()
}

/// Vérifie qu'une référence existe dans les définitions
pub fn validate_reference(
    ref_name: &str,
    definitions: Option<&std::collections::HashMap<String, Box<JtdSchema>>>,
) -> Result<(), JtdError> {
    if let Some(defs) = definitions {
        if !defs.contains_key(ref_name) {
            return Err(JtdError::ReferenceNotFound(ref_name.to_string()));
        }
    } else {
        return Err(JtdError::ReferenceNotFound(format!(
            "Reference '{}' not found: no definitions available",
            ref_name
        )));
    }
    Ok(())
}

/// Détecte les références circulaires dans un schéma
#[allow(dead_code)]
pub fn detect_circular_references(
    schema: &JtdSchema,
    visited: &mut HashSet<String>,
) -> Result<(), JtdError> {
    detect_circular_references_with_definitions(schema, schema.definitions.as_ref(), visited)
}

/// Détecte les références circulaires avec les définitions racine
pub fn detect_circular_references_with_definitions(
    schema: &JtdSchema,
    root_definitions: Option<&std::collections::HashMap<String, Box<JtdSchema>>>,
    visited: &mut HashSet<String>,
) -> Result<(), JtdError> {
    match &schema.form {
        SchemaForm::Ref { ref_name } => {
            if visited.contains(ref_name) {
                return Err(JtdError::CircularReference(format!(
                    "Circular reference detected: {}",
                    ref_name
                )));
            }
            visited.insert(ref_name.clone());
            
            // Utiliser les définitions racine pour résoudre la référence
            if let Some(definitions) = root_definitions {
                if let Some(ref_schema) = definitions.get(ref_name) {
                    detect_circular_references_with_definitions(ref_schema, Some(definitions), visited)?;
                }
            }
            visited.remove(ref_name);
        }
        SchemaForm::Elements { elements } => {
            detect_circular_references_with_definitions(elements, root_definitions, visited)?;
        }
        SchemaForm::Values { values } => {
            detect_circular_references_with_definitions(values, root_definitions, visited)?;
        }
        SchemaForm::Properties {
            properties,
            optional_properties,
            ..
        } => {
            for prop_schema in properties.values() {
                detect_circular_references_with_definitions(prop_schema, root_definitions, visited)?;
            }
            if let Some(opt_props) = optional_properties {
                for opt_schema in opt_props.values() {
                    detect_circular_references_with_definitions(opt_schema, root_definitions, visited)?;
                }
            }
        }
        SchemaForm::Discriminator { mapping, .. } => {
            for map_schema in mapping.values() {
                detect_circular_references_with_definitions(map_schema, root_definitions, visited)?;
            }
        }
        _ => {}
    }
    
    // Vérifier les définitions récursivement (seulement au niveau racine)
    if let Some(definitions) = &schema.definitions {
        for (_, def_schema) in definitions {
            detect_circular_references_with_definitions(def_schema, Some(definitions), visited)?;
        }
    }
    
    Ok(())
}
