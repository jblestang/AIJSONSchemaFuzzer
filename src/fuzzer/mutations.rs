use crate::schema::ast::{JtdSchema, SchemaForm, TypeName};
use rand::Rng;
use serde_json::Value;

/// Types d'altérations possibles
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MutationType {
    /// Altération syntaxique : JSON mal formé
    Syntax,
    /// Altération sémantique : violation du schéma
    Semantic,
}

/// Génère un JSON invalide selon le type d'altération
pub fn generate_invalid_json(
    schema: &JtdSchema,
    mutation_type: MutationType,
    mutation_name: Option<&str>,
) -> Result<String, String> {
    match mutation_type {
        MutationType::Syntax => {
            let syntax_name = mutation_name.and_then(|n| {
                crate::fuzzer::mutation_names::SyntaxMutationName::from_str(n)
            });
            generate_syntax_invalid(schema, syntax_name)
        }
        MutationType::Semantic => {
            let semantic_name = mutation_name.and_then(|n| {
                crate::fuzzer::mutation_names::SemanticMutationName::from_str(n)
            });
            generate_semantic_invalid(schema, semantic_name)
        }
    }
}

/// Génère un JSON syntaxiquement invalide
pub fn generate_syntax_invalid(schema: &JtdSchema, mutation_name: Option<crate::fuzzer::mutation_names::SyntaxMutationName>) -> Result<String, String> {
    let valid_json = crate::fuzzer::generator::generate_valid_json(schema);
    
    // Pour mixed-indentation, on a besoin d'un JSON formaté
    let needs_formatted = mutation_name == Some(crate::fuzzer::mutation_names::SyntaxMutationName::MixedIndentation);
    let json_str = if needs_formatted {
        serde_json::to_string_pretty(&valid_json)
            .map_err(|e| format!("Erreur de sérialisation: {}", e))?
    } else {
        serde_json::to_string(&valid_json)
            .map_err(|e| format!("Erreur de sérialisation: {}", e))?
    };
    
    let mutation = if let Some(name) = mutation_name {
        match name {
            crate::fuzzer::mutation_names::SyntaxMutationName::MissingClosingBrace => 0,
            crate::fuzzer::mutation_names::SyntaxMutationName::MissingOpeningBrace => 1,
            crate::fuzzer::mutation_names::SyntaxMutationName::InvalidCharacter => 2,
            crate::fuzzer::mutation_names::SyntaxMutationName::CommaToSemicolon => 3,
            crate::fuzzer::mutation_names::SyntaxMutationName::RemoveQuotes => 4,
            crate::fuzzer::mutation_names::SyntaxMutationName::TrailingComma => 5,
            crate::fuzzer::mutation_names::SyntaxMutationName::ColonToEquals => 6,
            crate::fuzzer::mutation_names::SyntaxMutationName::TruncatedJson => 7,
            crate::fuzzer::mutation_names::SyntaxMutationName::MixedIndentation => 8,
        }
    } else {
        let mut rng = rand::thread_rng();
        rng.gen_range(0..=8)
    };
    
    let invalid = match mutation {
        0 => {
            // Supprimer une accolade fermante
            if json_str.ends_with('}') || json_str.ends_with(']') {
                json_str[..json_str.len() - 1].to_string()
            } else {
                format!("{}}}", json_str)
            }
        }
        1 => {
            // Supprimer une accolade ouvrant
            if json_str.starts_with('{') || json_str.starts_with('[') {
                json_str[1..].to_string()
            } else {
                format!("{{{}", json_str)
            }
        }
        2 => {
            // Ajouter un caractère invalide
            format!("{}x", json_str)
        }
        3 => {
            // Remplacer une virgule par un point-virgule
            json_str.replace(',', ";")
        }
        4 => {
            // Supprimer les guillemets d'une clé
            json_str.replace("\"", "")
        }
        5 => {
            // Ajouter une virgule trailing
            if json_str.ends_with('}') {
                format!("{},", &json_str[..json_str.len() - 1])
            } else if json_str.ends_with(']') {
                format!("{},]", &json_str[..json_str.len() - 1])
            } else {
                format!("{},", json_str)
            }
        }
        6 => {
            // Remplacer : par =
            json_str.replace(':', "=")
        }
        7 => {
            // JSON tronqué
            if json_str.len() > 10 {
                json_str[..json_str.len() / 2].to_string()
            } else {
                json_str
            }
        }
        8 => {
            // Mutation 8: Mélange tabulations et espaces dans l'indentation
            // Génère un JSON avec un formatage mixte (tabs et espaces)
            let mut result = String::new();
            let mut in_string = false;
            let mut escape_next = false;
            let mut rng = rand::thread_rng();
            let mut chars = json_str.chars().peekable();
            let mut skip_indent = false;
            
            while let Some(ch) = chars.next() {
                if escape_next {
                    result.push(ch);
                    escape_next = false;
                    continue;
                }
                
                match ch {
                    '"' => {
                        in_string = !in_string;
                        result.push(ch);
                    }
                    '\\' if in_string => {
                        escape_next = true;
                        result.push(ch);
                    }
                    '\n' if !in_string => {
                        result.push(ch);
                        skip_indent = true;
                        // Après un saut de ligne, compter l'indentation existante
                        let mut indent_count = 0;
                        let mut peek_chars = chars.clone();
                        while let Some(&next_ch) = peek_chars.peek() {
                            if next_ch == ' ' || next_ch == '\t' {
                                indent_count += 1;
                                peek_chars.next();
                            } else {
                                break;
                            }
                        }
                        
                        // Générer une indentation mixte (tabs et espaces)
                        // Mélanger aléatoirement tabs et espaces pour le même niveau
                        for _ in 0..indent_count {
                            if rng.gen_bool(0.5) {
                                result.push('\t');
                            } else {
                                result.push(' ');
                            }
                        }
                    }
                    ' ' | '\t' if !in_string && skip_indent => {
                        // Ignorer les espaces/tabs existants après un saut de ligne
                        // (on les a déjà remplacés par l'indentation mixte)
                        continue;
                    }
                    _ => {
                        skip_indent = false;
                        result.push(ch);
                    }
                }
            }
            result
        }
        _ => json_str,
    };
    
    Ok(invalid)
}

/// Génère un JSON sémantiquement invalide (valide syntaxiquement mais viole le schéma)
pub fn generate_semantic_invalid(schema: &JtdSchema, mutation_name: Option<crate::fuzzer::mutation_names::SemanticMutationName>) -> Result<String, String> {
    let invalid_value = generate_semantic_invalid_value(schema, &schema.definitions, mutation_name.as_ref());
    serde_json::to_string_pretty(&invalid_value)
        .map_err(|e| format!("Erreur de sérialisation: {}", e))
}

fn generate_semantic_invalid_value(
    schema: &JtdSchema,
    root_definitions: &Option<std::collections::HashMap<String, Box<JtdSchema>>>,
    mutation_name: Option<&crate::fuzzer::mutation_names::SemanticMutationName>,
) -> Value {
    let mut rng = rand::thread_rng();
    
    // Si un nom de mutation est spécifié, l'utiliser, sinon sélectionner aléatoirement
    let mutation = if let Some(name) = mutation_name {
        map_mutation_name_to_index(schema, name)
    } else {
        // Sélection aléatoire par défaut
        match &schema.form {
            SchemaForm::Type { .. } => rng.gen_range(0..=5),
            SchemaForm::Enum { .. } => rng.gen_range(0..=10),
            SchemaForm::Elements { .. } => rng.gen_range(0..=11),
            SchemaForm::Values { .. } => rng.gen_range(0..=5),
            SchemaForm::Properties { .. } => rng.gen_range(0..=20),
            SchemaForm::Discriminator { .. } => rng.gen_range(0..=9),
            SchemaForm::Ref { .. } => rng.gen_range(0..=1),
            SchemaForm::Empty { .. } => 0,
        }
    };
    
    match &schema.form {
        SchemaForm::Empty {} => {
            // Pour empty, on peut générer n'importe quoi, donc on génère quelque chose d'invalide
            // en ajoutant des propriétés non autorisées (mais empty accepte tout, donc on génère null si nullable=false)
            if schema.nullable == Some(false) {
                Value::Null
            } else {
                generate_arbitrary_value()
            }
        }
        SchemaForm::Ref { ref_name } => {
            if let Some(defs) = root_definitions {
                if let Some(ref_schema) = defs.get(ref_name) {
                    generate_semantic_invalid_value(ref_schema, root_definitions, mutation_name)
                } else {
                    // Référence invalide
                    Value::String("invalid_ref".to_string())
                }
            } else {
                Value::Null
            }
        }
        SchemaForm::Type { type_name } => {
            match mutation {
                0..=2 => {
                    // Type incorrect
                    generate_wrong_type_value(type_name)
                }
                3..=4 => {
                    // Valeur hors plage
                    generate_out_of_range_value(type_name)
                }
                5 => {
                    // Null si nullable=false
                    if schema.nullable == Some(false) {
                        Value::Null
                    } else {
                        generate_wrong_type_value(type_name)
                    }
                }
                _ => generate_wrong_type_value(type_name),
            }
        }
        SchemaForm::Enum { enum_values } => {
            // Mutations sémantiques pour Enum:
            // - Valeur non dans l'enum (chaîne aléatoire)
            // - Chaîne similaire mais différente (variations)
            if enum_values.is_empty() {
                Value::String("invalid".to_string())
            } else {
                match mutation {
                    0..=7 => {
                        // Valeur non dans l'enum (chaîne aléatoire)
                        Value::String(format!("not_in_enum_{}", rng.gen_range(0..1000)))
                    }
                    8..=9 => {
                        // Chaîne similaire mais différente (variations de casse, espaces, etc.)
                        if let Some(first_value) = enum_values.first() {
                            let mut similar = first_value.clone();
                            // Ajouter un préfixe/suffixe
                            similar = format!("{}_modified", similar);
                            Value::String(similar)
                        } else {
                            Value::String("invalid_enum_value".to_string())
                        }
                    }
                    10 => {
                        // Chaîne vide si l'enum ne contient pas de chaîne vide
                        if !enum_values.contains(&"".to_string()) {
                            Value::String("".to_string())
                        } else {
                            Value::String("not_in_enum".to_string())
                        }
                    }
                    _ => Value::String("not_in_enum".to_string()),
                }
            }
        }
        SchemaForm::Elements { elements } => {
            // Mutations sémantiques pour Elements (tableaux):
            // 0-2:   Type incorrect (pas un tableau)
            // 3-4:   Élément invalide (un seul élément qui viole le schéma)
            // 5-6:   Types mixtes dans le tableau (mélange d'éléments valides et invalides)
            // 7-8:   Tous les éléments invalides
            // 9-10:  Tableau avec types complètement différents (string, number, bool, object, array)
            // 11:    Tableau vide (peut être valide selon le schéma, mais testons)
            // Note: mutation est déjà calculée plus haut
            
            match mutation {
                0..=2 => {
                    // Mutation 0-2: Pas un tableau (type incorrect)
                    Value::String("not_an_array".to_string())
                }
                3..=4 => {
                    // Mutation 3-4: Un seul élément invalide
                    let mut arr = vec![];
                    arr.push(generate_semantic_invalid_value(elements, root_definitions, None));
                    Value::Array(arr)
                }
                5..=6 => {
                    // Mutation 5-6: Types mixtes - mélange d'éléments valides et invalides
                    let mut arr = vec![];
                    let size = rng.gen_range(3..=8);
                    for i in 0..size {
                        if i % 2 == 0 {
                            // Éléments valides
                            let valid = crate::fuzzer::generator::generate_with_definitions(elements, root_definitions);
                            arr.push(valid);
                        } else {
                            // Éléments invalides (types mixtes)
                            arr.push(generate_semantic_invalid_value(elements, root_definitions, None));
                        }
                    }
                    Value::Array(arr)
                }
                7..=8 => {
                    // Mutation 7-8: Tous les éléments invalides
                    let mut arr = vec![];
                    let size = rng.gen_range(2..=5);
                    for _ in 0..size {
                        arr.push(generate_semantic_invalid_value(elements, root_definitions, None));
                    }
                    Value::Array(arr)
                }
                9..=10 => {
                    // Mutation 9-10: Tableau avec types complètement différents
                    // Mélange de string, number, bool, object, array, null
                    let mut arr = vec![];
                    let size = rng.gen_range(3..=6);
                    for i in 0..size {
                        match i % 6 {
                            0 => arr.push(Value::String("mixed_type_string".to_string())),
                            1 => arr.push(Value::Number(serde_json::Number::from(42))),
                            2 => arr.push(Value::Bool(true)),
                            3 => arr.push(Value::Object(serde_json::Map::new())),
                            4 => arr.push(Value::Array(vec![])),
                            5 => arr.push(Value::Null),
                            _ => arr.push(Value::Null),
                        }
                    }
                    Value::Array(arr)
                }
                11 => {
                    // Mutation 11: Tableau vide (peut être valide, mais testons)
                    Value::Array(vec![])
                }
                _ => Value::Object(serde_json::Map::new()),
            }
        }
        SchemaForm::Values { values } => {
            // Mutations sémantiques pour Values (objets avec valeurs uniformes):
            // 0-2:   Type incorrect (pas un objet)
            // 3-5:   Valeur invalide (une ou plusieurs clés avec valeurs invalides)
            // Note: mutation est déjà calculée plus haut
            
            match mutation {
                0..=2 => {
                    // Mutation 0-2: Pas un objet (type incorrect)
                    Value::Array(vec![])
                }
                3..=4 => {
                    // Mutation 3-4: Une clé avec valeur invalide
                    let mut obj = serde_json::Map::new();
                    obj.insert("key".to_string(), generate_semantic_invalid_value(values, root_definitions, None));
                    Value::Object(obj)
                }
                5 => {
                    // Mutation 5: Plusieurs clés avec valeurs invalides
                    let mut obj = serde_json::Map::new();
                    let count = rng.gen_range(2..=4);
                    for i in 0..count {
                        obj.insert(format!("key_{}", i), generate_semantic_invalid_value(values, root_definitions, None));
                    }
                    Value::Object(obj)
                }
                _ => Value::Null,
            }
        }
        SchemaForm::Properties {
            properties,
            optional_properties,
            additional_properties,
        } => {
            // Mutations sémantiques pour Properties (21 types de mutations):
            // 0-2:   Type incorrect (pas un objet)
            // 3-4:   Toutes les propriétés requises manquantes
            // 5-6:   Une propriété requise manquante (aléatoire)
            // 7-8:   Propriétés supplémentaires (si additionalProperties=false)
            // 9-10:  Valeur invalide pour une propriété requise
            // 11-12: Toutes les valeurs de propriétés requises invalides
            // 13-14: Valeur invalide pour une propriété optionnelle
            // 15-16: Propriété requise avec null (si nullable=false)
            // 17-18: Propriété requise manquante + propriété supplémentaire
            // 19:    Objet vide (toutes propriétés requises manquantes)
            // 20:    Objet null (si nullable=false) ou propriétés partielles
            // Note: mutation est déjà calculée plus haut
            
            match mutation {
                0..=2 => {
                    // Mutation 0-2: Pas un objet (type incorrect)
                    Value::Array(vec![])
                }
                3..=4 => {
                    // Mutation 3-4: Toutes les propriétés requises manquantes
                    let mut obj = serde_json::Map::new();
                    // Ajouter seulement les propriétés optionnelles
                    if let Some(opt_props) = optional_properties {
                        for (key, opt_schema) in opt_props {
                            if rng.gen_bool(0.5) {
                                let value = crate::fuzzer::generator::generate_with_definitions(opt_schema, root_definitions);
                                obj.insert(key.clone(), value);
                            }
                        }
                    }
                    Value::Object(obj)
                }
                5..=6 => {
                    // Mutation 5-6: Une propriété requise manquante (aléatoire)
                    let mut obj = serde_json::Map::new();
                    let keys: Vec<&String> = properties.keys().collect();
                    if !keys.is_empty() {
                        let skip_idx = rng.gen_range(0..keys.len());
                        for (i, (key, prop_schema)) in properties.iter().enumerate() {
                            if i != skip_idx {
                                let value = crate::fuzzer::generator::generate_with_definitions(prop_schema, root_definitions);
                                obj.insert(key.clone(), value);
                            }
                        }
                        // Ajouter les propriétés optionnelles
                        if let Some(opt_props) = optional_properties {
                            for (key, opt_schema) in opt_props {
                                if rng.gen_bool(0.7) {
                                    let value = crate::fuzzer::generator::generate_with_definitions(opt_schema, root_definitions);
                                    obj.insert(key.clone(), value);
                                }
                            }
                        }
                    }
                    Value::Object(obj)
                }
                7..=8 => {
                    // Mutation 7-8: Propriété supplémentaire si additionalProperties=false
                    if additional_properties.unwrap_or(false) == false {
                        let mut obj = serde_json::Map::new();
                        // Ajouter toutes les propriétés requises
                        for (key, prop_schema) in properties {
                            let value = crate::fuzzer::generator::generate_with_definitions(prop_schema, root_definitions);
                            obj.insert(key.clone(), value);
                        }
                        // Ajouter les propriétés optionnelles
                        if let Some(opt_props) = optional_properties {
                            for (key, opt_schema) in opt_props {
                                if rng.gen_bool(0.5) {
                                    let value = crate::fuzzer::generator::generate_with_definitions(opt_schema, root_definitions);
                                    obj.insert(key.clone(), value);
                                }
                            }
                        }
                        // Ajouter des propriétés supplémentaires invalides
                        let extra_count = rng.gen_range(1..=3);
                        for i in 0..extra_count {
                            obj.insert(format!("extra_property_{}", i), Value::String("invalid".to_string()));
                        }
                        Value::Object(obj)
                    } else {
                        // Si additionalProperties=true, on génère une propriété manquante
                        let mut obj = serde_json::Map::new();
                        let keys: Vec<&String> = properties.keys().collect();
                        if !keys.is_empty() {
                            let skip = rng.gen_range(0..keys.len());
                            for (i, (key, prop_schema)) in properties.iter().enumerate() {
                                if i != skip {
                                    let value = crate::fuzzer::generator::generate_with_definitions(prop_schema, root_definitions);
                                    obj.insert(key.clone(), value);
                                }
                            }
                        }
                        Value::Object(obj)
                    }
                }
                9..=10 => {
                    // Mutation 9-10: Valeur de propriété requise invalide (une seule)
                    let mut obj = serde_json::Map::new();
                    let keys: Vec<&String> = properties.keys().collect();
                    if !keys.is_empty() {
                        let invalid_key_idx = rng.gen_range(0..keys.len());
                        for (i, (key, prop_schema)) in properties.iter().enumerate() {
                            if i == invalid_key_idx {
                                // Valeur invalide pour cette propriété
                                let value = generate_semantic_invalid_value(prop_schema, root_definitions, None);
                                obj.insert(key.clone(), value);
                            } else {
                                // Valeurs valides pour les autres
                                let value = crate::fuzzer::generator::generate_with_definitions(prop_schema, root_definitions);
                                obj.insert(key.clone(), value);
                            }
                        }
                        // Ajouter les propriétés optionnelles valides
                        if let Some(opt_props) = optional_properties {
                            for (key, opt_schema) in opt_props {
                                if rng.gen_bool(0.5) {
                                    let value = crate::fuzzer::generator::generate_with_definitions(opt_schema, root_definitions);
                                    obj.insert(key.clone(), value);
                                }
                            }
                        }
                    }
                    Value::Object(obj)
                }
                11..=12 => {
                    // Mutation 11-12: Toutes les valeurs de propriétés requises invalides
                    let mut obj = serde_json::Map::new();
                    for (key, prop_schema) in properties {
                        let value = generate_semantic_invalid_value(prop_schema, root_definitions, None);
                        obj.insert(key.clone(), value);
                    }
                    // Ajouter les propriétés optionnelles valides
                    if let Some(opt_props) = optional_properties {
                        for (key, opt_schema) in opt_props {
                            if rng.gen_bool(0.5) {
                                let value = crate::fuzzer::generator::generate_with_definitions(opt_schema, root_definitions);
                                obj.insert(key.clone(), value);
                            }
                        }
                    }
                    Value::Object(obj)
                }
                13..=14 => {
                    // Mutation 13-14: Valeur de propriété optionnelle invalide
                    let mut obj = serde_json::Map::new();
                    // Toutes les propriétés requises valides
                    for (key, prop_schema) in properties {
                        let value = crate::fuzzer::generator::generate_with_definitions(prop_schema, root_definitions);
                        obj.insert(key.clone(), value);
                    }
                    // Une propriété optionnelle avec valeur invalide
                    if let Some(opt_props) = optional_properties {
                        let opt_keys: Vec<&String> = opt_props.keys().collect();
                        if !opt_keys.is_empty() {
                            let invalid_opt_idx = rng.gen_range(0..opt_keys.len());
                            for (i, (key, opt_schema)) in opt_props.iter().enumerate() {
                                if i == invalid_opt_idx {
                                    let value = generate_semantic_invalid_value(opt_schema, root_definitions, None);
                                    obj.insert(key.clone(), value);
                                } else if rng.gen_bool(0.5) {
                                    let value = crate::fuzzer::generator::generate_with_definitions(opt_schema, root_definitions);
                                    obj.insert(key.clone(), value);
                                }
                            }
                        }
                    }
                    Value::Object(obj)
                }
                15..=16 => {
                    // Mutation 15-16: Propriété requise avec type null (si nullable=false)
                    let mut obj = serde_json::Map::new();
                    for (key, prop_schema) in properties {
                        if prop_schema.nullable != Some(true) {
                            // Mettre null pour une propriété non-nullable
                            obj.insert(key.clone(), Value::Null);
                        } else {
                            let value = crate::fuzzer::generator::generate_with_definitions(prop_schema, root_definitions);
                            obj.insert(key.clone(), value);
                        }
                    }
                    // Ajouter les propriétés optionnelles
                    if let Some(opt_props) = optional_properties {
                        for (key, opt_schema) in opt_props {
                            if rng.gen_bool(0.5) {
                                let value = crate::fuzzer::generator::generate_with_definitions(opt_schema, root_definitions);
                                obj.insert(key.clone(), value);
                            }
                        }
                    }
                    Value::Object(obj)
                }
                17..=18 => {
                    // Mutation 17-18: Propriété requise manquante + propriété supplémentaire
                    let mut obj = serde_json::Map::new();
                    let keys: Vec<&String> = properties.keys().collect();
                    if !keys.is_empty() {
                        let skip = rng.gen_range(0..keys.len());
                        for (i, (key, prop_schema)) in properties.iter().enumerate() {
                            if i != skip {
                                let value = crate::fuzzer::generator::generate_with_definitions(prop_schema, root_definitions);
                                obj.insert(key.clone(), value);
                            }
                        }
                    }
                    // Ajouter propriété supplémentaire si non autorisée
                    if additional_properties.unwrap_or(false) == false {
                        obj.insert("unexpected_key".to_string(), Value::String("invalid".to_string()));
                    }
                    Value::Object(obj)
                }
                19 => {
                    // Mutation 19: Objet vide (toutes les propriétés requises manquantes)
                    Value::Object(serde_json::Map::new())
                }
                20 => {
                    // Mutation 20: Objet null (si nullable=false)
                    if schema.nullable != Some(true) {
                        Value::Null
                    } else {
                        // Sinon, générer un objet avec propriété manquante
                        let mut obj = serde_json::Map::new();
                        let keys: Vec<&String> = properties.keys().collect();
                        if !keys.is_empty() && keys.len() > 1 {
                            // Garder seulement la première propriété
                            if let Some((first_key, first_schema)) = properties.iter().next() {
                                let value = crate::fuzzer::generator::generate_with_definitions(first_schema, root_definitions);
                                obj.insert(first_key.clone(), value);
                            }
                        }
                        Value::Object(obj)
                    }
                }
                _ => Value::Null,
            }
        }
        SchemaForm::Discriminator {
            discriminator,
            mapping,
        } => {
            // Mutations sémantiques pour Discriminator (10 types de mutations):
            // 0-2:   Type incorrect (pas un objet)
            // 3-4:   Tag manquant
            // 5-6:   Tag invalide (pas dans mapping)
            // 7-8:   Tag non-string
            // 9:     Instance invalide selon le schéma du mapping
            let mutation = rng.gen_range(0..=9);
            
            match mutation {
                0..=2 => {
                    // Mutation 0-2: Pas un objet (type incorrect)
                    Value::Array(vec![])
                }
                3..=4 => {
                    // Mutation 3-4: Tag manquant
                    let mut obj = serde_json::Map::new();
                    if let Some((_tag, tag_schema)) = mapping.iter().next() {
                        let tag_obj = crate::fuzzer::generator::generate_with_definitions(tag_schema, root_definitions);
                        if let Value::Object(tag_map) = tag_obj {
                            obj.extend(tag_map);
                        }
                    }
                    Value::Object(obj)
                }
                5..=6 => {
                    // Mutation 5-6: Tag invalide (pas dans mapping)
                    let mut obj = serde_json::Map::new();
                    obj.insert(discriminator.clone(), Value::String("invalid_tag".to_string()));
                    Value::Object(obj)
                }
                7..=8 => {
                    // Mutation 7-8: Tag non-string
                    let mut obj = serde_json::Map::new();
                    if mutation == 7 {
                        obj.insert(discriminator.clone(), Value::Number(serde_json::Number::from(42)));
                    } else {
                        obj.insert(discriminator.clone(), Value::Bool(true));
                    }
                    Value::Object(obj)
                }
                9 => {
                    // Mutation 9: Instance invalide selon le schéma du mapping
                    let tags: Vec<&String> = mapping.keys().collect();
                    if let Some(tag) = tags.get(0) {
                        let mut obj = serde_json::Map::new();
                        obj.insert(discriminator.clone(), Value::String((*tag).clone()));
                        if let Some(tag_schema) = mapping.get(*tag) {
                            let invalid_tag_obj = generate_semantic_invalid_value(tag_schema, root_definitions, None);
                            if let Value::Object(tag_map) = invalid_tag_obj {
                                obj.extend(tag_map);
                            }
                        }
                        Value::Object(obj)
                    } else {
                        Value::Null
                    }
                }
                _ => Value::Null,
            }
        }
    }
}

fn generate_wrong_type_value(expected_type: &TypeName) -> Value {
    let mut rng = rand::thread_rng();
    match expected_type {
        TypeName::Boolean => Value::String("not_a_boolean".to_string()),
        TypeName::String => Value::Bool(true),
        TypeName::Timestamp => Value::String("not-a-valid-timestamp".to_string()),
        TypeName::Float32 | TypeName::Float64 => Value::String("not_a_number".to_string()),
        TypeName::Int8 | TypeName::Uint8 | TypeName::Int16 | TypeName::Uint16
        | TypeName::Int32 | TypeName::Uint32 => {
            if rng.gen_bool(0.5) {
                Value::String("not_an_integer".to_string())
            } else {
                Value::Number(serde_json::Number::from(999999999999i64))
            }
        }
    }
}

fn generate_out_of_range_value(type_name: &TypeName) -> Value {
    match type_name {
        TypeName::Int8 => Value::Number(serde_json::Number::from(200)),
        TypeName::Uint8 => Value::Number(serde_json::Number::from(-1)),
        TypeName::Int16 => Value::Number(serde_json::Number::from(40000)),
        TypeName::Uint16 => Value::Number(serde_json::Number::from(-1)),
        TypeName::Int32 => Value::Number(serde_json::Number::from(3000000000i64)),
        TypeName::Uint32 => Value::Number(serde_json::Number::from(-1)),
        TypeName::Float32 => Value::Number(
            serde_json::Number::from_f64(1e50).unwrap_or(serde_json::Number::from(0)),
        ),
        _ => Value::String("out_of_range".to_string()),
    }
}

fn generate_arbitrary_value() -> Value {
    let mut rng = rand::thread_rng();
    match rng.gen_range(0..=4) {
        0 => Value::Null,
        1 => Value::Bool(rng.gen()),
        2 => Value::Number(serde_json::Number::from(rng.gen_range(0..=100))),
        3 => Value::String("arbitrary".to_string()),
        4 => Value::Array(vec![]),
        _ => Value::Null,
    }
}

/// Mappe un nom de mutation à un index pour la forme de schéma donnée
fn map_mutation_name_to_index(schema: &JtdSchema, name: &crate::fuzzer::mutation_names::SemanticMutationName) -> u32 {
    match &schema.form {
        SchemaForm::Type { .. } => {
            match name {
                crate::fuzzer::mutation_names::SemanticMutationName::WrongType => 0,
                crate::fuzzer::mutation_names::SemanticMutationName::OutOfRange => 3,
                crate::fuzzer::mutation_names::SemanticMutationName::NullForNonNullable => 5,
                _ => 0, // Par défaut
            }
        }
        SchemaForm::Enum { .. } => {
            match name {
                crate::fuzzer::mutation_names::SemanticMutationName::NotInEnum => 0,
                crate::fuzzer::mutation_names::SemanticMutationName::SimilarButDifferent => 8,
                crate::fuzzer::mutation_names::SemanticMutationName::EmptyString => 10,
                _ => 0,
            }
        }
        SchemaForm::Elements { .. } => {
            match name {
                crate::fuzzer::mutation_names::SemanticMutationName::NotAnArray => 0,
                crate::fuzzer::mutation_names::SemanticMutationName::SingleInvalidElement => 3,
                crate::fuzzer::mutation_names::SemanticMutationName::MixedTypes => 5,
                crate::fuzzer::mutation_names::SemanticMutationName::AllInvalidElements => 7,
                crate::fuzzer::mutation_names::SemanticMutationName::CompletelyDifferentTypes => 9,
                crate::fuzzer::mutation_names::SemanticMutationName::EmptyArray => 11,
                _ => 0,
            }
        }
        SchemaForm::Values { .. } => {
            match name {
                crate::fuzzer::mutation_names::SemanticMutationName::NotAnObject => 0,
                crate::fuzzer::mutation_names::SemanticMutationName::SingleInvalidValue => 3,
                crate::fuzzer::mutation_names::SemanticMutationName::MultipleInvalidValues => 5,
                _ => 0,
            }
        }
        SchemaForm::Properties { .. } => {
            match name {
                crate::fuzzer::mutation_names::SemanticMutationName::NotAnObjectProp => 0,
                crate::fuzzer::mutation_names::SemanticMutationName::AllRequiredMissing => 3,
                crate::fuzzer::mutation_names::SemanticMutationName::OneRequiredMissing => 5,
                crate::fuzzer::mutation_names::SemanticMutationName::AdditionalProperties => 7,
                crate::fuzzer::mutation_names::SemanticMutationName::SingleInvalidProperty => 9,
                crate::fuzzer::mutation_names::SemanticMutationName::AllInvalidProperties => 11,
                crate::fuzzer::mutation_names::SemanticMutationName::InvalidOptionalProperty => 13,
                crate::fuzzer::mutation_names::SemanticMutationName::NullForNonNullableProp => 15,
                crate::fuzzer::mutation_names::SemanticMutationName::MissingPlusAdditional => 17,
                crate::fuzzer::mutation_names::SemanticMutationName::EmptyObject => 19,
                crate::fuzzer::mutation_names::SemanticMutationName::NullObject => 20,
                _ => 0,
            }
        }
        SchemaForm::Discriminator { .. } => {
            match name {
                crate::fuzzer::mutation_names::SemanticMutationName::NotAnObjectDisc => 0,
                crate::fuzzer::mutation_names::SemanticMutationName::MissingTag => 3,
                crate::fuzzer::mutation_names::SemanticMutationName::InvalidTag => 5,
                crate::fuzzer::mutation_names::SemanticMutationName::TagNotString => 7,
                crate::fuzzer::mutation_names::SemanticMutationName::InvalidInstance => 9,
                _ => 0,
            }
        }
        SchemaForm::Ref { .. } => {
            match name {
                crate::fuzzer::mutation_names::SemanticMutationName::InvalidReference => 0,
                crate::fuzzer::mutation_names::SemanticMutationName::NonExistentReference => 1,
                _ => 0,
            }
        }
        SchemaForm::Empty { .. } => {
            match name {
                crate::fuzzer::mutation_names::SemanticMutationName::NullForEmpty => 0,
                _ => 0,
            }
        }
    }
}
