use crate::error::JtdError;
use crate::fuzzer::mutations::{generate_invalid_json, MutationType};
use crate::fuzzer::generator::generate_valid_json;
use crate::fuzzer::json_schema_mutations::generate_json_schema_semantic_invalid;
use crate::schema::parser::parse_schema_file;
use crate::schema::unified_parser::{parse_schema_file_auto, UnifiedSchema};
use std::fs;
use std::path::Path;

/// Options pour le fuzzer
pub struct FuzzerOptions {
    pub count: usize,
    pub mutation_type: MutationType,
    pub mutation_name: Option<String>,
    pub output_dir: Option<String>,
}

/// Exécute le fuzzer sur un schéma
pub fn run_fuzzer(schema_path: &Path, options: FuzzerOptions) -> Result<(), JtdError> {
    // Vérifier que le fichier schéma existe
    if !schema_path.exists() {
        return Err(JtdError::IoError(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("Le fichier schéma '{}' n'existe pas", schema_path.display())
        )));
    }
    
    let unified_schema = parse_schema_file_auto(schema_path)?;
    
    let schema_format = match &unified_schema {
        UnifiedSchema::JTD(_) => "JTD (RFC 8927)",
        UnifiedSchema::JsonSchema2020(_) => "JSON Schema 2020-12",
    };
    
    println!("=== Fuzzer ===");
    println!("Schéma: {}", schema_path.display());
    println!("Format détecté: {}", schema_format);
    println!("Type d'altération: {:?}", options.mutation_type);
    if let Some(ref name) = options.mutation_name {
        println!("Mutation spécifique: {}", name);
    } else {
        println!("Mutation: aléatoire");
    }
    println!("Nombre de cas: {}", options.count);
    println!();
    
    // Créer le répertoire de sortie si nécessaire
    if let Some(ref output_dir) = options.output_dir {
        fs::create_dir_all(output_dir)
            .map_err(|e| JtdError::IoError(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Impossible de créer le répertoire '{}': {}", output_dir, e)
            )))?;
        println!("Répertoire de sortie: {}", output_dir);
    }
    
    for i in 0..options.count {
        match options.mutation_type {
            MutationType::Syntax => {
                // Pour l'instant, les mutations syntaxiques ne sont supportées que pour JTD
                match &unified_schema {
                    UnifiedSchema::JTD(jtd_schema) => {
                        let invalid_json = generate_invalid_json(jtd_schema, MutationType::Syntax, options.mutation_name.as_deref())
                            .map_err(|e| JtdError::SchemaSyntaxError(e))?;
                        
                        if let Some(ref output_dir) = options.output_dir {
                            let file_path = format!("{}/invalid_syntax_{:04}.json", output_dir, i);
                            fs::write(&file_path, &invalid_json)?;
                            println!("✓ Généré: {}", file_path);
                        } else {
                            println!("=== Cas {} (Syntaxiquement invalide) ===", i + 1);
                            println!("{}", invalid_json);
                            println!();
                        }
                    }
                    UnifiedSchema::JsonSchema2020(_) => {
                        eprintln!("⚠ Les mutations syntaxiques ne sont pas encore supportées pour JSON Schema 2020-12");
                        return Err(JtdError::SchemaSyntaxError("Mutations syntaxiques non supportées pour JSON Schema 2020-12".to_string()));
                    }
                }
            }
            MutationType::Semantic => {
                match &unified_schema {
                    UnifiedSchema::JTD(jtd_schema) => {
                        let invalid_json = generate_invalid_json(jtd_schema, MutationType::Semantic, options.mutation_name.as_deref())
                            .map_err(|e| JtdError::SchemaSyntaxError(e))?;
                        
                        if let Some(ref output_dir) = options.output_dir {
                            let file_path = format!("{}/invalid_semantic_{:04}.json", output_dir, i);
                            fs::write(&file_path, &invalid_json)?;
                            println!("✓ Généré: {}", file_path);
                        } else {
                            println!("=== Cas {} (Sémantiquement invalide) ===", i + 1);
                            println!("{}", invalid_json);
                            println!();
                        }
                    }
                    UnifiedSchema::JsonSchema2020(json_schema) => {
                        let invalid_json = generate_json_schema_semantic_invalid(json_schema, options.mutation_name.as_deref())
                            .map_err(|e| JtdError::SchemaSyntaxError(e))?;
                        
                        if let Some(ref output_dir) = options.output_dir {
                            let file_path = format!("{}/invalid_semantic_{:04}.json", output_dir, i);
                            fs::write(&file_path, &invalid_json)?;
                            println!("✓ Généré: {}", file_path);
                        } else {
                            println!("=== Cas {} (Sémantiquement invalide) ===", i + 1);
                            println!("{}", invalid_json);
                            println!();
                        }
                    }
                }
            }
        }
    }
    
    Ok(())
}

/// Génère un JSON valide selon le schéma
pub fn generate_valid(schema_path: &Path, output_path: Option<&Path>) -> Result<(), JtdError> {
    // Vérifier que le fichier schéma existe
    if !schema_path.exists() {
        return Err(JtdError::IoError(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("Le fichier schéma '{}' n'existe pas", schema_path.display())
        )));
    }
    
    let unified_schema = parse_schema_file_auto(schema_path)?;
    
    let valid_json = match &unified_schema {
        UnifiedSchema::JTD(jtd_schema) => {
            generate_valid_json(jtd_schema)
        }
        UnifiedSchema::JsonSchema2020(_) => {
            // Pour l'instant, la génération de JSON valides pour JSON Schema 2020-12 n'est pas implémentée
            return Err(JtdError::SchemaSyntaxError("Génération de JSON valides non supportée pour JSON Schema 2020-12".to_string()));
        }
    };
    
    let json_str = serde_json::to_string_pretty(&valid_json)
        .map_err(|e| JtdError::SchemaSyntaxError(format!("Erreur de sérialisation: {}", e)))?;
    
    if let Some(output_path) = output_path {
        // Créer le répertoire parent si nécessaire
        if let Some(parent) = output_path.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| JtdError::IoError(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("Impossible de créer le répertoire parent '{}': {}", parent.display(), e)
                )))?;
        }
        fs::write(output_path, &json_str)?;
        println!("✓ JSON valide généré: {}", output_path.display());
    } else {
        println!("{}", json_str);
    }
    
    Ok(())
}
