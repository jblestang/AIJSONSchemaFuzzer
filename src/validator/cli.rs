use crate::error::JtdError;
use crate::schema::unified_parser::{parse_schema_file_auto, UnifiedSchema};
use crate::validator::validate::validate;
use crate::validator::json_schema_validate::validate_json_schema;
use serde_json::Value;
use std::fs;
use std::path::Path;

pub fn validate_files(schema_path: &Path, instance_path: &Path) {
    match run_validation(schema_path, instance_path) {
        Ok(()) => {
            println!("✓ Validation successful");
        }
        Err(e) => {
            eprintln!("✗ Validation failed: {}", e);
            std::process::exit(1);
        }
    }
}

fn run_validation(schema_path: &Path, instance_path: &Path) -> Result<(), JtdError> {
    // Vérifier que les fichiers existent
    if !schema_path.exists() {
        return Err(JtdError::IoError(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("Le fichier schéma '{}' n'existe pas", schema_path.display())
        )));
    }
    if !instance_path.exists() {
        return Err(JtdError::IoError(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("Le fichier instance '{}' n'existe pas", instance_path.display())
        )));
    }
    
    let schema = parse_schema_file_auto(schema_path)?;
    let instance_content = fs::read_to_string(instance_path)?;
    let instance: Value = serde_json::from_str(&instance_content)?;
    
    match schema {
        UnifiedSchema::JTD(jtd_schema) => {
            match validate(&jtd_schema, &instance, "", "") {
                Ok(()) => Ok(()),
                Err(errors) => {
                    for error in errors {
                        eprintln!("  - {}", error);
                    }
                    Err(JtdError::SchemaSyntaxError("Validation failed".to_string()))
                }
            }
        }
        UnifiedSchema::JsonSchema2020(json_schema) => {
            let root_defs = match &json_schema {
                crate::schema::json_schema::JsonSchema2020::Object(obj) => obj.defs.as_ref(),
                _ => None,
            };
            match validate_json_schema(&json_schema, &instance, root_defs) {
                Ok(()) => Ok(()),
                Err(errors) => {
                    for error in errors {
                        eprintln!("  - {}", error);
                    }
                    Err(JtdError::SchemaSyntaxError("Validation failed".to_string()))
                }
            }
        }
    }
}
