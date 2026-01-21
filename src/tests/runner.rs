use crate::schema::parser::parse_schema;
use crate::schema::json_schema_parser::parse_json_schema;
use crate::tests::test_cases::all_test_cases;
use crate::tests::json_schema_test_cases::all_json_schema_test_cases;
use crate::validator::validate::validate;
use crate::validator::json_schema_validate::validate_json_schema;

/// Exécute tous les tests de conformité RFC 8927 et JSON Schema 2020-12
pub fn run_all_tests() {
    println!("=== Exécution des tests de conformité ===\n");
    
    // Tests JTD (RFC 8927)
    println!("=== Tests JTD (RFC 8927) ===\n");
    let jtd_cases = all_test_cases();
    let mut jtd_passed = 0;
    let mut jtd_failed = 0;
    
    for case in jtd_cases {
        match run_jtd_test_case(&case) {
            Ok(()) => {
                println!("✓ {}", case.name);
                jtd_passed += 1;
            }
            Err(e) => {
                println!("✗ {} - {}", case.name, e);
                jtd_failed += 1;
            }
        }
    }
    
    println!("\n=== Tests JSON Schema 2020-12 ===\n");
    let json_schema_cases = all_json_schema_test_cases();
    let mut json_schema_passed = 0;
    let mut json_schema_failed = 0;
    
    for case in json_schema_cases {
        match run_json_schema_test_case(&case) {
            Ok(()) => {
                println!("✓ {}", case.name);
                json_schema_passed += 1;
            }
            Err(e) => {
                println!("✗ {} - {}", case.name, e);
                json_schema_failed += 1;
            }
        }
    }
    
    println!("\n=== Résultats ===");
    println!("JTD - Passés: {}, Échoués: {}, Total: {}", jtd_passed, jtd_failed, jtd_passed + jtd_failed);
    println!("JSON Schema 2020-12 - Passés: {}, Échoués: {}, Total: {}", json_schema_passed, json_schema_failed, json_schema_passed + json_schema_failed);
    println!("Total global - Passés: {}, Échoués: {}, Total: {}", 
             jtd_passed + json_schema_passed, 
             jtd_failed + json_schema_failed, 
             jtd_passed + jtd_failed + json_schema_passed + json_schema_failed);
    
    if jtd_failed > 0 || json_schema_failed > 0 {
        std::process::exit(1);
    }
}

fn run_jtd_test_case(case: &crate::tests::test_cases::TestCase) -> Result<(), String> {
    // Parser le schéma
    let schema = parse_schema(case.schema)
        .map_err(|e| format!("Erreur de parsing du schéma: {}", e))?;
    
    // Valider l'instance
    let result = validate(&schema, &case.instance, "", "");
    
    match (result, case.should_be_valid) {
        (Ok(()), true) => {
            // Cas valide, validation réussie - OK
            Ok(())
        }
        (Ok(()), false) => {
            // Cas invalide mais validation réussie - ERREUR
            Err("Validation a réussi alors qu'elle devrait échouer".to_string())
        }
        (Err(errors), true) => {
            // Cas valide mais validation échouée - ERREUR
            Err(format!(
                "Validation a échoué alors qu'elle devrait réussir. Erreurs: {:?}",
                errors
            ))
        }
        (Err(errors), false) => {
            // Cas invalide, validation échouée - OK (mais vérifier les erreurs si spécifiées)
            if let Some(expected) = &case.expected_errors {
                // Vérifier que les erreurs correspondent (simplifié)
                if errors.len() != expected.len() {
                    return Err(format!(
                        "Nombre d'erreurs incorrect: attendu {}, obtenu {}",
                        expected.len(),
                        errors.len()
                    ));
                }
            }
            Ok(())
        }
    }
}

fn run_json_schema_test_case(case: &crate::tests::json_schema_test_cases::JsonSchemaTestCase) -> Result<(), String> {
    // Parser le schéma
    let schema = parse_json_schema(case.schema)
        .map_err(|e| format!("Erreur de parsing du schéma: {}", e))?;
    
    // Obtenir les définitions racine
    let root_defs = match &schema {
        crate::schema::json_schema::JsonSchema2020::Object(obj) => obj.defs.as_ref(),
        _ => None,
    };
    
    // Valider l'instance
    let result = validate_json_schema(&schema, &case.instance, root_defs);
    
    match (result, case.should_be_valid) {
        (Ok(()), true) => {
            // Cas valide, validation réussie - OK
            Ok(())
        }
        (Ok(()), false) => {
            // Cas invalide mais validation réussie - ERREUR
            Err("Validation a réussi alors qu'elle devrait échouer".to_string())
        }
        (Err(errors), true) => {
            // Cas valide mais validation échouée - ERREUR
            Err(format!(
                "Validation a échoué alors qu'elle devrait réussir. Erreurs: {:?}",
                errors
            ))
        }
        (Err(errors), false) => {
            // Cas invalide, validation échouée - OK (mais vérifier les erreurs si spécifiées)
            if let Some(expected) = &case.expected_errors {
                // Vérifier que les erreurs correspondent (simplifié)
                if errors.len() != expected.len() {
                    return Err(format!(
                        "Nombre d'erreurs incorrect: attendu {}, obtenu {}",
                        expected.len(),
                        errors.len()
                    ));
                }
            }
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_basic_validation() {
        let schema = parse_schema(r#"{"type": "string"}"#).unwrap();
        let instance = serde_json::json!("hello");
        assert!(validate(&schema, &instance, "", "").is_ok());
    }
}
