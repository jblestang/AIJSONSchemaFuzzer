use crate::schema::json_schema::{JsonSchema2020, JsonSchemaObject, JsonSchemaItems};
use crate::fuzzer::mutation_names::SyntaxMutationName;
use rand::Rng;
use serde_json::Value;

/// Génère un JSON syntaxiquement invalide pour JSON Schema 2020-12
pub fn generate_json_schema_syntax_invalid(
    schema: &JsonSchema2020,
    mutation_name: Option<&str>,
) -> Result<String, String> {
    // Générer un JSON valide basique
    let valid_json = generate_json_schema_valid_value(schema);
    
    // Pour mixed-indentation, on a besoin d'un JSON formaté
    let syntax_name = mutation_name.and_then(|n| SyntaxMutationName::from_str(n));
    let needs_formatted = syntax_name == Some(SyntaxMutationName::MixedIndentation);
    let json_str = if needs_formatted {
        serde_json::to_string_pretty(&valid_json)
            .map_err(|e| format!("Erreur de sérialisation: {}", e))?
    } else {
        serde_json::to_string(&valid_json)
            .map_err(|e| format!("Erreur de sérialisation: {}", e))?
    };
    
    // Appliquer la mutation syntaxique (même logique que pour JTD)
    let mutation = if let Some(name) = syntax_name {
        match name {
            SyntaxMutationName::MissingClosingBrace => 0,
            SyntaxMutationName::MissingOpeningBrace => 1,
            SyntaxMutationName::InvalidCharacter => 2,
            SyntaxMutationName::CommaToSemicolon => 3,
            SyntaxMutationName::RemoveQuotes => 4,
            SyntaxMutationName::TrailingComma => 5,
            SyntaxMutationName::ColonToEquals => 6,
            SyntaxMutationName::TruncatedJson => 7,
            SyntaxMutationName::MixedIndentation => 8,
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
            // Mélange tabulations et espaces dans l'indentation
            // Utiliser la même logique que pour JTD
            let mut result = String::new();
            let mut in_string = false;
            let mut escape_next = false;
            let mut rng = rand::thread_rng();
            let mut chars = json_str.chars().peekable();
            
            while let Some(ch) = chars.next() {
                if escape_next {
                    result.push(ch);
                    escape_next = false;
                    continue;
                }
                
                match ch {
                    '\\' => {
                        result.push(ch);
                        escape_next = true;
                    }
                    '"' => {
                        result.push(ch);
                        in_string = !in_string;
                    }
                    '\n' => {
                        result.push(ch);
                        // Ajouter indentation mixte après un saut de ligne
                        if !in_string {
                            while let Some(&next) = chars.peek() {
                                if next == ' ' || next == '\t' {
                                    chars.next();
                                    if rng.gen_bool(0.5) {
                                        result.push('\t');
                                    } else {
                                        result.push(' ');
                                    }
                                } else {
                                    break;
                                }
                            }
                        }
                    }
                    _ => result.push(ch),
                }
            }
            result
        }
        _ => json_str,
    };
    
    Ok(invalid)
}

/// Génère un JSON sémantiquement invalide pour JSON Schema 2020-12
pub fn generate_json_schema_semantic_invalid(
    schema: &JsonSchema2020,
    mutation_name: Option<&str>,
) -> Result<String, String> {
    let invalid_value = generate_json_schema_invalid_value(schema, mutation_name);
    serde_json::to_string_pretty(&invalid_value)
        .map_err(|e| format!("Erreur de sérialisation: {}", e))
}

fn generate_json_schema_invalid_value(
    schema: &JsonSchema2020,
    mutation_name: Option<&str>,
) -> Value {
    match schema {
        JsonSchema2020::Boolean(true) => {
            // Pour un schéma qui accepte tout, on génère null (qui peut être rejeté selon le contexte)
            Value::Null
        }
        JsonSchema2020::Boolean(false) => {
            // Pour un schéma qui rejette tout, on génère n'importe quoi (sera toujours invalide)
            Value::String("invalid".to_string())
        }
        JsonSchema2020::Object(obj) => {
            generate_json_schema_object_invalid(obj, mutation_name)
        }
    }
}

fn generate_json_schema_object_invalid(
    schema: &JsonSchemaObject,
    mutation_name: Option<&str>,
) -> Value {
    let mut rng = rand::thread_rng();
    
    // Priorité aux mutations spécifiques
    if let Some(name) = mutation_name {
        return generate_specific_mutation(schema, name);
    }
    
    // Mutations pour prefixItems (priorité si présent)
    if schema.prefix_items.is_some() {
        match rng.gen_range(0..6) {
            0 => {
                // Mauvais type à une position
                return generate_prefix_items_wrong_type(schema);
            }
            1 => {
                // Trop d'éléments quand items: false
                if let Some(ref items) = schema.items {
                    if let JsonSchemaItems::Boolean(false) = items {
                        return generate_prefix_items_extra_items(schema);
                    }
                }
            }
            2 => {
                // Pas assez d'éléments
                return generate_prefix_items_too_few(schema);
            }
            3 => {
                // Mauvais type dans items (après prefixItems)
                if let Some(ref items) = schema.items {
                    if let JsonSchemaItems::Schema(items_schema) = items {
                        return generate_prefix_items_with_invalid_items(schema, items_schema);
                    }
                }
            }
            4 => {
                // Violation minItems avec prefixItems
                if let Some(min_items) = schema.min_items {
                    let prefix_len = schema.prefix_items.as_ref().map(|v| v.len()).unwrap_or(0);
                    if min_items > prefix_len as u64 {
                        return generate_prefix_items_min_items_violation(schema, min_items);
                    }
                }
            }
            5 => {
                // Violation maxItems avec prefixItems
                if let Some(max_items) = schema.max_items {
                    let prefix_len = schema.prefix_items.as_ref().map(|v| v.len()).unwrap_or(0);
                    if max_items < prefix_len as u64 {
                        return generate_prefix_items_max_items_violation(schema, max_items);
                    }
                }
            }
            _ => {}
        }
    }
    
    // Mutations pour patternProperties
    if schema.pattern_properties.is_some() {
        match rng.gen_range(0..2) {
            0 => {
                // Clé qui match le pattern mais valeur invalide
                return generate_pattern_properties_invalid_value(schema);
            }
            _ => {}
        }
    }
    
    // Mutations pour allOf
    if let Some(ref all_of) = schema.all_of {
        if !all_of.is_empty() {
            // Générer une valeur qui viole un des sous-schémas
            let invalid_schema = &all_of[0];
            return generate_json_schema_invalid_value(invalid_schema, None);
        }
    }
    
    // Mutations pour anyOf
    if let Some(ref any_of) = schema.any_of {
        if !any_of.is_empty() {
            // Générer une valeur qui viole tous les sous-schémas
            // Pour simplifier, on génère une valeur qui viole le premier
            let invalid_schema = &any_of[0];
            return generate_json_schema_invalid_value(invalid_schema, None);
        }
    }
    
    // Mutations pour oneOf
    if let Some(ref one_of) = schema.one_of {
        if one_of.len() >= 2 {
            // Générer une valeur qui satisfait plusieurs sous-schémas (devrait être invalide)
            // Pour simplifier, on génère une valeur valide pour le premier
            return generate_json_schema_valid_value(&one_of[0]);
        }
    }
    
    // Mutations pour not
    if let Some(ref not) = schema.not {
        // Générer une valeur qui satisfait le schéma not (devrait être invalide)
        return generate_json_schema_valid_value(not);
    }
    
    // Mutations pour if/then/else
    if let Some(ref if_schema) = schema.if_ {
        match rng.gen_range(0..2) {
            0 => {
                // Condition vraie mais then invalide
                if let Some(ref then) = schema.then {
                    // Générer une valeur qui satisfait if mais viole then
                    let valid_for_if = generate_json_schema_valid_value(if_schema);
                    // Pour simplifier, on génère une valeur invalide pour then
                    return generate_json_schema_invalid_value(then, None);
                }
            }
            _ => {
                // Condition fausse mais else invalide
                if let Some(ref else_) = schema.else_ {
                    // Générer une valeur qui ne satisfait pas if mais viole else
                    return generate_json_schema_invalid_value(else_, None);
                }
            }
        }
    }
    
    // Mutations pour type
    if let Some(ref type_) = schema.type_ {
        // Générer un type incorrect
        return generate_type_violation(type_);
    }
    
    // Mutations pour enum
    if let Some(ref enum_) = schema.enum_ {
        if !enum_.is_empty() {
            // Générer une valeur non dans l'enum
            return generate_enum_violation(enum_);
        }
    }
    
    // Mutations pour const
    if let Some(ref const_value) = schema.const_ {
        // Générer une valeur différente de la constante
        return generate_different_value(const_value);
    }
    
    // Mutations pour required
    if let Some(ref required) = schema.required {
        if !required.is_empty() {
            // Générer un objet sans une propriété requise
            return generate_missing_required_property(schema, required);
        }
    }
    
    // Mutations pour additionalProperties
    if let Some(ref additional_properties) = schema.additional_properties {
        match additional_properties {
            crate::schema::json_schema::JsonSchemaAdditionalProperties::Boolean(false) => {
                // Générer un objet avec une propriété supplémentaire (sera rejetée)
                return generate_additional_properties_violation(schema);
            }
            crate::schema::json_schema::JsonSchemaAdditionalProperties::Schema(_) => {
                // Générer un objet avec une propriété supplémentaire invalide selon le schéma
                return generate_additional_properties_violation(schema);
            }
            crate::schema::json_schema::JsonSchemaAdditionalProperties::Boolean(true) => {
                // Pas de violation possible si true
            }
        }
    }
    
    // Mutations pour optionalProperties
    if let Some(ref optional_properties) = schema.optional_properties {
        if !optional_properties.is_empty() {
            // Générer un objet avec une propriété optionnelle invalide
            return generate_optional_properties_invalid(schema);
        }
    }
    
    // Mutations pour uniqueItems
    if let Some(true) = schema.unique_items {
        // Générer un tableau avec des doublons
        return generate_unique_items_violation(schema);
    }
    
    // Mutations pour contains
    if let Some(ref contains) = schema.contains {
        // Générer un tableau sans élément qui satisfait contains
        return generate_contains_violation(schema, contains);
    }
    
    // Mutations pour minItems (seulement si pas de prefixItems, car géré ci-dessus)
    if schema.prefix_items.is_none() {
        if let Some(min_items) = schema.min_items {
            if min_items > 0 {
                // Générer un tableau trop court avec des valeurs valides selon le schéma
                return generate_min_items_violation(schema, min_items);
            }
        }
    }
    
    // Mutations pour maxItems (seulement si pas de prefixItems, car géré ci-dessus)
    if schema.prefix_items.is_none() {
        if let Some(max_items) = schema.max_items {
            // Générer un tableau trop long avec des valeurs valides selon le schéma
            return generate_max_items_violation(schema, max_items);
        }
    }
    
    // Mutations pour minLength
    if let Some(min_length) = schema.min_length {
        if min_length > 0 {
            // Générer une chaîne trop courte
            return Value::String("x".repeat((min_length - 1) as usize));
        }
    }
    
    // Mutations pour maxLength
    if let Some(max_length) = schema.max_length {
        // Générer une chaîne trop longue
        return Value::String("x".repeat((max_length + 1) as usize));
    }
    
    // Mutations pour pattern
    if let Some(ref pattern_str) = schema.pattern {
        // Générer une chaîne qui ne match pas le pattern
        return Value::String("does_not_match_pattern".to_string());
    }
    
    // Mutations pour exclusiveMinimum
    if let Some(exclusive_minimum) = schema.exclusive_minimum {
        // Générer un nombre trop petit (<= exclusiveMinimum)
        return Value::Number(serde_json::Number::from_f64(exclusive_minimum).unwrap_or(serde_json::Number::from(0)));
    }
    
    // Mutations pour exclusiveMaximum
    if let Some(exclusive_maximum) = schema.exclusive_maximum {
        // Générer un nombre trop grand (>= exclusiveMaximum)
        return Value::Number(serde_json::Number::from_f64(exclusive_maximum).unwrap_or(serde_json::Number::from(0)));
    }
    
    // Mutations pour minimum
    if let Some(minimum) = schema.minimum {
        // Générer un nombre trop petit
        return Value::Number(serde_json::Number::from_f64(minimum - 1.0).unwrap_or(serde_json::Number::from(0)));
    }
    
    // Mutations pour maximum
    if let Some(maximum) = schema.maximum {
        // Générer un nombre trop grand
        return Value::Number(serde_json::Number::from_f64(maximum + 1.0).unwrap_or(serde_json::Number::from(0)));
    }
    
    // Mutations pour multipleOf
    if let Some(multiple_of) = schema.multiple_of {
        // Générer un nombre qui n'est pas un multiple
        return Value::Number(serde_json::Number::from_f64(multiple_of + 0.5).unwrap_or(serde_json::Number::from(1)));
    }
    
    // Mutations pour $ref
    if let Some(ref ref_path) = schema.ref_ {
        // Générer une instance invalide selon la référence
        // Pour simplifier, on génère null (sera invalide si la référence attend autre chose)
        return Value::Null;
    }
    
    // Par défaut, générer une valeur invalide basique
    Value::String("invalid".to_string())
}

fn generate_specific_mutation(schema: &JsonSchemaObject, name: &str) -> Value {
    match name {
        "prefix-items-wrong-type" => generate_prefix_items_wrong_type(schema),
        "prefix-items-extra" => generate_prefix_items_extra_items(schema),
        "prefix-items-too-few" => generate_prefix_items_too_few(schema),
        "prefix-items-invalid-items" => {
            if let Some(ref items) = schema.items {
                if let JsonSchemaItems::Schema(items_schema) = items {
                    return generate_prefix_items_with_invalid_items(schema, items_schema);
                }
            }
            Value::Null
        }
        "prefix-items-min-items-violation" => {
            if let Some(min_items) = schema.min_items {
                return generate_prefix_items_min_items_violation(schema, min_items);
            }
            Value::Null
        }
        "prefix-items-max-items-violation" => {
            if let Some(max_items) = schema.max_items {
                return generate_prefix_items_max_items_violation(schema, max_items);
            }
            Value::Null
        }
        "pattern-properties-invalid-value" => generate_pattern_properties_invalid_value(schema),
        "all-of-invalid" => {
            if let Some(ref all_of) = schema.all_of {
                if !all_of.is_empty() {
                    return generate_json_schema_invalid_value(&all_of[0], None);
                }
            }
            Value::Null
        }
        "any-of-all-invalid" => {
            if let Some(ref any_of) = schema.any_of {
                if !any_of.is_empty() {
                    return generate_json_schema_invalid_value(&any_of[0], None);
                }
            }
            Value::Null
        }
        "one-of-multiple-valid" => {
            if let Some(ref one_of) = schema.one_of {
                if one_of.len() >= 2 {
                    return generate_json_schema_valid_value(&one_of[0]);
                }
            }
            Value::Null
        }
        "not-satisfied" => {
            if let Some(ref not) = schema.not {
                return generate_json_schema_valid_value(not);
            }
            Value::Null
        }
        "if-then-invalid" => {
            if let Some(ref if_schema) = schema.if_ {
                if let Some(ref then) = schema.then {
                    return generate_json_schema_invalid_value(then, None);
                }
            }
            Value::Null
        }
        "if-else-invalid" => {
            if let Some(ref else_) = schema.else_ {
                return generate_json_schema_invalid_value(else_, None);
            }
            Value::Null
        }
        "const-different" => {
            if let Some(ref const_value) = schema.const_ {
                return generate_different_value(const_value);
            }
            Value::Null
        }
        "missing-required" => {
            if let Some(ref required) = schema.required {
                if !required.is_empty() {
                    return generate_missing_required_property(schema, required);
                }
            }
            Value::Null
        }
        "min-items-violation" => {
            if let Some(min_items) = schema.min_items {
                if min_items > 0 {
                    return generate_min_items_violation(schema, min_items);
                }
            }
            Value::Null
        }
        "max-items-violation" => {
            if let Some(max_items) = schema.max_items {
                return generate_max_items_violation(schema, max_items);
            }
            Value::Null
        }
        "min-length-violation" => {
            if let Some(min_length) = schema.min_length {
                if min_length > 0 {
                    return Value::String("x".repeat((min_length - 1) as usize));
                }
            }
            Value::Null
        }
        "max-length-violation" => {
            if let Some(max_length) = schema.max_length {
                return Value::String("x".repeat((max_length + 1) as usize));
            }
            Value::Null
        }
        "pattern-violation" => {
            return Value::String("does_not_match_pattern".to_string());
        }
        "minimum-violation" => {
            if let Some(minimum) = schema.minimum {
                return Value::Number(serde_json::Number::from_f64(minimum - 1.0).unwrap_or(serde_json::Number::from(0)));
            }
            Value::Null
        }
        "maximum-violation" => {
            if let Some(maximum) = schema.maximum {
                return Value::Number(serde_json::Number::from_f64(maximum + 1.0).unwrap_or(serde_json::Number::from(0)));
            }
            Value::Null
        }
        "multiple-of-violation" => {
            if let Some(multiple_of) = schema.multiple_of {
                return Value::Number(serde_json::Number::from_f64(multiple_of + 0.5).unwrap_or(serde_json::Number::from(1)));
            }
            Value::Null
        }
        "type-violation" => {
            if let Some(ref type_) = schema.type_ {
                return generate_type_violation(type_);
            }
            Value::Null
        }
        "enum-violation" => {
            if let Some(ref enum_) = schema.enum_ {
                if !enum_.is_empty() {
                    return generate_enum_violation(enum_);
                }
            }
            Value::Null
        }
        "unique-items-violation" => {
            if let Some(true) = schema.unique_items {
                return generate_unique_items_violation(schema);
            }
            Value::Null
        }
        "contains-violation" => {
            if let Some(ref contains) = schema.contains {
                return generate_contains_violation(schema, contains);
            }
            Value::Null
        }
        "exclusive-minimum-violation" => {
            if let Some(exclusive_minimum) = schema.exclusive_minimum {
                return Value::Number(serde_json::Number::from_f64(exclusive_minimum).unwrap_or(serde_json::Number::from(0)));
            }
            Value::Null
        }
        "exclusive-maximum-violation" => {
            if let Some(exclusive_maximum) = schema.exclusive_maximum {
                return Value::Number(serde_json::Number::from_f64(exclusive_maximum).unwrap_or(serde_json::Number::from(0)));
            }
            Value::Null
        }
        "ref-invalid" => {
            // Générer une instance invalide selon la référence
            return Value::Null;
        }
        "additional-properties-violation" => {
            return generate_additional_properties_violation(schema);
        }
        "optional-properties-invalid" => {
            return generate_optional_properties_invalid(schema);
        }
        _ => Value::Null,
    }
}

fn generate_prefix_items_wrong_type(schema: &JsonSchemaObject) -> Value {
    if let Some(ref prefix_items) = schema.prefix_items {
        if !prefix_items.is_empty() {
            let mut arr = Vec::new();
            // Générer le bon type pour le premier élément, mais mauvais pour les autres
            for (i, item_schema) in prefix_items.iter().enumerate() {
                if i == 0 {
                    // Premier élément valide
                    arr.push(generate_json_schema_valid_value(item_schema));
                } else {
                    // Autres éléments invalides
                    arr.push(generate_json_schema_invalid_value(item_schema, None));
                }
            }
            return Value::Array(arr);
        }
    }
    Value::Array(vec![])
}

fn generate_prefix_items_extra_items(schema: &JsonSchemaObject) -> Value {
    if let Some(ref prefix_items) = schema.prefix_items {
        let mut arr = Vec::new();
        // Générer tous les éléments prefixItems valides
        for item_schema in prefix_items {
            arr.push(generate_json_schema_valid_value(item_schema));
        }
        // Ajouter un élément supplémentaire (interdit si items: false)
        arr.push(Value::String("extra".to_string()));
        return Value::Array(arr);
    }
    Value::Array(vec![])
}

fn generate_prefix_items_too_few(schema: &JsonSchemaObject) -> Value {
    if let Some(ref prefix_items) = schema.prefix_items {
        if prefix_items.len() > 1 {
            // Générer moins d'éléments que requis
            let mut arr = Vec::new();
            for item_schema in prefix_items.iter().take(prefix_items.len() - 1) {
                arr.push(generate_json_schema_valid_value(item_schema));
            }
            return Value::Array(arr);
        } else if !prefix_items.is_empty() {
            // Si un seul élément requis, générer un tableau vide
            return Value::Array(vec![]);
        }
    }
    Value::Array(vec![])
}

fn generate_prefix_items_with_invalid_items(schema: &JsonSchemaObject, items_schema: &JsonSchema2020) -> Value {
    if let Some(ref prefix_items) = schema.prefix_items {
        let mut arr = Vec::new();
        // Générer tous les éléments prefixItems valides
        for item_schema in prefix_items {
            arr.push(generate_json_schema_valid_value(item_schema));
        }
        // Ajouter un élément supplémentaire avec un type invalide pour items
        arr.push(generate_json_schema_invalid_value(items_schema, None));
        return Value::Array(arr);
    }
    Value::Array(vec![])
}

fn generate_prefix_items_min_items_violation(schema: &JsonSchemaObject, min_items: u64) -> Value {
    if let Some(ref prefix_items) = schema.prefix_items {
        let prefix_len = prefix_items.len() as u64;
        if min_items > prefix_len {
            // Générer un tableau avec prefixItems valides mais pas assez d'éléments totaux
            let mut arr = Vec::new();
            // Générer tous les prefixItems
            for item_schema in prefix_items {
                arr.push(generate_json_schema_valid_value(item_schema));
            }
            // Ajouter des éléments selon items si présent, mais pas assez pour atteindre minItems
            if let Some(ref items) = schema.items {
                if let JsonSchemaItems::Schema(items_schema) = items {
                    let needed = (min_items - prefix_len - 1) as usize;
                    for _ in 0..needed {
                        arr.push(generate_json_schema_valid_value(items_schema));
                    }
                }
            }
            return Value::Array(arr);
        }
    }
    Value::Array(vec![])
}

fn generate_prefix_items_max_items_violation(schema: &JsonSchemaObject, max_items: u64) -> Value {
    if let Some(ref prefix_items) = schema.prefix_items {
        let prefix_len = prefix_items.len() as u64;
        if max_items >= prefix_len {
            // Générer un tableau avec prefixItems + trop d'éléments selon items
            let mut arr = Vec::new();
            // Générer tous les prefixItems
            for item_schema in prefix_items {
                arr.push(generate_json_schema_valid_value(item_schema));
            }
            // Ajouter des éléments selon items jusqu'à dépasser maxItems
            if let Some(ref items) = schema.items {
                if let JsonSchemaItems::Schema(items_schema) = items {
                    let extra = (max_items - prefix_len + 1) as usize;
                    for _ in 0..extra {
                        arr.push(generate_json_schema_valid_value(items_schema));
                    }
                }
            }
            return Value::Array(arr);
        }
    }
    Value::Array(vec![])
}

fn generate_min_items_violation(schema: &JsonSchemaObject, min_items: u64) -> Value {
    // Générer un tableau trop court avec des valeurs valides selon le schéma
    let mut arr = Vec::new();
    
    // Si items est présent, générer des valeurs selon items
    if let Some(ref items) = schema.items {
        if let JsonSchemaItems::Schema(items_schema) = items {
            for _ in 0..(min_items - 1) {
                arr.push(generate_json_schema_valid_value(items_schema));
            }
        }
    } else {
        // Par défaut, générer des nulls
        for _ in 0..(min_items - 1) {
            arr.push(Value::Null);
        }
    }
    
    Value::Array(arr)
}

fn generate_max_items_violation(schema: &JsonSchemaObject, max_items: u64) -> Value {
    // Générer un tableau trop long avec des valeurs valides selon le schéma
    let mut arr = Vec::new();
    
    // Si items est présent, générer des valeurs selon items
    if let Some(ref items) = schema.items {
        if let JsonSchemaItems::Schema(items_schema) = items {
            for _ in 0..(max_items + 1) {
                arr.push(generate_json_schema_valid_value(items_schema));
            }
        }
    } else {
        // Par défaut, générer des nulls
        for _ in 0..(max_items + 1) {
            arr.push(Value::Null);
        }
    }
    
    Value::Array(arr)
}

fn generate_pattern_properties_invalid_value(schema: &JsonSchemaObject) -> Value {
    if let Some(ref pattern_properties) = schema.pattern_properties {
        let mut obj = serde_json::Map::new();
        // Prendre le premier pattern
        if let Some((pattern_str, pattern_schema)) = pattern_properties.iter().next() {
            // Générer une clé qui match le pattern
            let key = if pattern_str.starts_with('^') && pattern_str.ends_with('$') {
                // Pattern simple, générer une clé qui match
                "x-foo".to_string()
            } else {
                "x-foo".to_string()
            };
            // Mais valeur invalide
            let invalid_value = generate_json_schema_invalid_value(pattern_schema, None);
            obj.insert(key, invalid_value);
            return Value::Object(obj);
        }
    }
    Value::Object(serde_json::Map::new())
}

fn generate_missing_required_property(schema: &JsonSchemaObject, required: &[String]) -> Value {
    let mut obj = serde_json::Map::new();
    
    // Ajouter toutes les propriétés sauf une requise
    if let Some(ref properties) = schema.properties {
        for (key, prop_schema) in properties {
            if key != &required[0] {
                let value = generate_json_schema_valid_value(prop_schema);
                obj.insert(key.clone(), value);
            }
        }
    }
    
    Value::Object(obj)
}

fn generate_different_value(const_value: &Value) -> Value {
    match const_value {
        Value::String(_) => Value::String("different".to_string()),
        Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                Value::Number(serde_json::Number::from(i + 1))
            } else if let Some(f) = n.as_f64() {
                Value::Number(serde_json::Number::from_f64(f + 1.0).unwrap_or(serde_json::Number::from(0)))
            } else {
                Value::Bool(true)
            }
        }
        Value::Bool(b) => Value::Bool(!b),
        Value::Null => Value::String("not_null".to_string()),
        Value::Array(_) => Value::Array(vec![]),
        Value::Object(_) => Value::Object(serde_json::Map::new()),
    }
}

fn generate_json_schema_valid_value(schema: &JsonSchema2020) -> Value {
    match schema {
        JsonSchema2020::Boolean(true) => Value::Null,
        JsonSchema2020::Boolean(false) => Value::Null,
        JsonSchema2020::Object(obj) => {
            // Génération basique d'une valeur valide
            if let Some(ref type_) = obj.type_ {
                match type_ {
                    crate::schema::json_schema::JsonSchemaType::String => Value::String("valid".to_string()),
                    crate::schema::json_schema::JsonSchemaType::Number => Value::Number(serde_json::Number::from(42)),
                    crate::schema::json_schema::JsonSchemaType::Integer => Value::Number(serde_json::Number::from(42)),
                    crate::schema::json_schema::JsonSchemaType::Boolean => Value::Bool(true),
                    crate::schema::json_schema::JsonSchemaType::Null => Value::Null,
                    crate::schema::json_schema::JsonSchemaType::Array => Value::Array(vec![]),
                    crate::schema::json_schema::JsonSchemaType::Object => Value::Object(serde_json::Map::new()),
                }
            } else if let Some(ref const_value) = obj.const_ {
                const_value.clone()
            } else if let Some(ref enum_) = obj.enum_ {
                if !enum_.is_empty() {
                    enum_[0].clone()
                } else {
                    Value::Null
                }
            } else {
                Value::Null
            }
        }
    }
}

fn generate_type_violation(type_: &crate::schema::json_schema::JsonSchemaType) -> Value {
    // Générer un type différent de celui attendu
    match type_ {
        crate::schema::json_schema::JsonSchemaType::String => Value::Number(serde_json::Number::from(42)),
        crate::schema::json_schema::JsonSchemaType::Number => Value::String("not_a_number".to_string()),
        crate::schema::json_schema::JsonSchemaType::Integer => Value::String("not_an_integer".to_string()),
        crate::schema::json_schema::JsonSchemaType::Boolean => Value::String("not_a_boolean".to_string()),
        crate::schema::json_schema::JsonSchemaType::Null => Value::String("not_null".to_string()),
        crate::schema::json_schema::JsonSchemaType::Array => Value::String("not_an_array".to_string()),
        crate::schema::json_schema::JsonSchemaType::Object => Value::String("not_an_object".to_string()),
    }
}

fn generate_enum_violation(enum_: &[serde_json::Value]) -> Value {
    // Générer une valeur qui n'est pas dans l'enum
    // Si l'enum contient des strings, générer une string différente
    // Sinon, générer un type différent
    if let Some(Value::String(_)) = enum_.first() {
        // Générer une string qui n'est pas dans l'enum
        let mut candidate = "not_in_enum_0".to_string();
        let mut counter = 0;
        while enum_.contains(&Value::String(candidate.clone())) {
            candidate = format!("not_in_enum_{}", counter);
            counter += 1;
        }
        Value::String(candidate)
    } else if let Some(Value::Number(_)) = enum_.first() {
        // Générer un nombre qui n'est pas dans l'enum
        Value::String("not_a_number_in_enum".to_string())
    } else {
        // Par défaut, générer une string
        Value::String("not_in_enum".to_string())
    }
}

fn generate_unique_items_violation(schema: &JsonSchemaObject) -> Value {
    // Générer un tableau avec des éléments dupliqués
    let mut arr = Vec::new();
    
    // Générer un élément valide selon items ou prefixItems
    let valid_item = if let Some(ref items) = schema.items {
        if let JsonSchemaItems::Schema(items_schema) = items {
            generate_json_schema_valid_value(items_schema)
        } else {
            Value::String("item".to_string())
        }
    } else if let Some(ref prefix_items) = schema.prefix_items {
        if !prefix_items.is_empty() {
            generate_json_schema_valid_value(&prefix_items[0])
        } else {
            Value::String("item".to_string())
        }
    } else {
        Value::String("item".to_string())
    };
    
    // Ajouter le même élément deux fois
    arr.push(valid_item.clone());
    arr.push(valid_item);
    
    Value::Array(arr)
}

fn generate_contains_violation(schema: &JsonSchemaObject, contains: &JsonSchema2020) -> Value {
    // Générer un tableau sans élément qui satisfait contains
    let mut arr = Vec::new();
    
    // Générer des éléments valides selon items ou prefixItems, mais qui ne satisfont pas contains
    let invalid_for_contains = generate_json_schema_invalid_value(contains, None);
    
    // Ajouter quelques éléments invalides pour contains
    arr.push(invalid_for_contains.clone());
    arr.push(invalid_for_contains);
    
    Value::Array(arr)
}

fn generate_additional_properties_violation(schema: &JsonSchemaObject) -> Value {
    use crate::schema::json_schema::JsonSchemaAdditionalProperties;
    
    let mut obj = serde_json::Map::new();
    
    // Ajouter les propriétés valides selon properties
    if let Some(ref properties) = schema.properties {
        for (key, prop_schema) in properties {
            let value = generate_json_schema_valid_value(prop_schema);
            obj.insert(key.clone(), value);
        }
    }
    
    // Ajouter une propriété supplémentaire invalide
    if let Some(ref additional_properties) = schema.additional_properties {
        match additional_properties {
            JsonSchemaAdditionalProperties::Boolean(false) => {
                // Si additionalProperties: false, ajouter n'importe quelle propriété supplémentaire
                obj.insert("extra_property".to_string(), Value::String("invalid".to_string()));
            }
            JsonSchemaAdditionalProperties::Schema(add_schema) => {
                // Si additionalProperties: <schema>, ajouter une propriété qui viole ce schéma
                let invalid_value = generate_json_schema_invalid_value(add_schema, None);
                obj.insert("extra_property".to_string(), invalid_value);
            }
            JsonSchemaAdditionalProperties::Boolean(true) => {
                // Si additionalProperties: true, on ne peut pas violer, donc générer null
                return Value::Null;
            }
        }
    } else {
        // Si additionalProperties n'est pas défini, par défaut c'est true, donc on ne peut pas violer
        return Value::Null;
    }
    
    Value::Object(obj)
}

fn generate_optional_properties_invalid(schema: &JsonSchemaObject) -> Value {
    let mut obj = serde_json::Map::new();
    
    // Ajouter les propriétés requises valides
    if let Some(ref properties) = schema.properties {
        for (key, prop_schema) in properties {
            let value = generate_json_schema_valid_value(prop_schema);
            obj.insert(key.clone(), value);
        }
    }
    
    // Ajouter une propriété optionnelle invalide
    if let Some(ref optional_properties) = schema.optional_properties {
        if let Some((key, opt_schema)) = optional_properties.iter().next() {
            // Générer une valeur invalide pour cette propriété optionnelle
            let invalid_value = generate_json_schema_invalid_value(opt_schema, None);
            obj.insert(key.clone(), invalid_value);
        }
    } else {
        // Si pas d'optionalProperties, on ne peut pas violer
        return Value::Null;
    }
    
    Value::Object(obj)
}
