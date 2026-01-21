use crate::error::{ValidationError, ValidationResult};
use crate::schema::json_schema::{
    JsonSchema2020, JsonSchemaAdditionalProperties, JsonSchemaItems, JsonSchemaObject,
    JsonSchemaType,
};
use regex::Regex;
use serde_json::Value;
use std::collections::HashMap;

/// Valide une instance JSON contre un schéma JSON Schema 2020-12
pub fn validate_json_schema(
    schema: &JsonSchema2020,
    instance: &Value,
    root_defs: Option<&HashMap<String, Box<JsonSchema2020>>>,
) -> ValidationResult {
    validate_json_schema_with_path(schema, instance, "", "", root_defs)
}

fn validate_json_schema_with_path(
    schema: &JsonSchema2020,
    instance: &Value,
    instance_path: &str,
    schema_path: &str,
    root_defs: Option<&HashMap<String, Box<JsonSchema2020>>>,
) -> ValidationResult {
    match schema {
        JsonSchema2020::Boolean(true) => Ok(()),
        JsonSchema2020::Boolean(false) => Err(vec![ValidationError {
            instance_path: instance_path.to_string(),
            schema_path: schema_path.to_string(),
        }]),
        JsonSchema2020::Object(obj) => {
            validate_json_schema_object(obj, instance, instance_path, schema_path, root_defs)
        }
    }
}

fn validate_json_schema_object(
    schema: &JsonSchemaObject,
    instance: &Value,
    instance_path: &str,
    schema_path: &str,
    root_defs: Option<&HashMap<String, Box<JsonSchema2020>>>,
) -> ValidationResult {
    let mut errors = Vec::new();
    
    // Gérer $ref en premier
    if let Some(ref ref_path) = schema.ref_ {
        if let Some(defs) = root_defs {
            match crate::schema::json_schema_parser::resolve_ref(ref_path, Some(defs)) {
                Ok(ref_schema) => {
                    return validate_json_schema_with_path(
                        ref_schema,
                        instance,
                        instance_path,
                        &format!("{}/$ref", schema_path),
                        root_defs,
                    );
                }
                Err(_) => {
                    // Si la référence n'est pas résolue, continuer avec le reste du schéma
                }
            }
        }
    }
    
    // Gérer allOf
    if let Some(ref all_of) = schema.all_of {
        for (i, sub_schema) in all_of.iter().enumerate() {
            let sub_path = format!("{}/allOf/{}", schema_path, i);
            if let Err(mut errs) = validate_json_schema_with_path(
                sub_schema,
                instance,
                instance_path,
                &sub_path,
                root_defs,
            ) {
                errors.append(&mut errs);
            }
        }
    }
    
    // Gérer anyOf
    if let Some(ref any_of) = schema.any_of {
        let mut any_valid = false;
        let mut any_errors = Vec::new();
        for (i, sub_schema) in any_of.iter().enumerate() {
            let sub_path = format!("{}/anyOf/{}", schema_path, i);
            match validate_json_schema_with_path(
                sub_schema,
                instance,
                instance_path,
                &sub_path,
                root_defs,
            ) {
                Ok(()) => any_valid = true,
                Err(mut errs) => any_errors.append(&mut errs),
            }
        }
        if !any_valid {
            errors.append(&mut any_errors);
        }
    }
    
    // Gérer oneOf
    if let Some(ref one_of) = schema.one_of {
        let mut valid_count = 0;
        let mut all_errors = Vec::new();
        for (i, sub_schema) in one_of.iter().enumerate() {
            let sub_path = format!("{}/oneOf/{}", schema_path, i);
            match validate_json_schema_with_path(
                sub_schema,
                instance,
                instance_path,
                &sub_path,
                root_defs,
            ) {
                Ok(()) => valid_count += 1,
                Err(mut errs) => all_errors.append(&mut errs),
            }
        }
        if valid_count != 1 {
            errors.push(ValidationError {
                instance_path: instance_path.to_string(),
                schema_path: format!("{}/oneOf", schema_path),
            });
        }
    }
    
    // Gérer not
    if let Some(ref not) = schema.not {
        let not_path = format!("{}/not", schema_path);
        if validate_json_schema_with_path(not, instance, instance_path, &not_path, root_defs)
            .is_ok()
        {
            errors.push(ValidationError {
                instance_path: instance_path.to_string(),
                schema_path: not_path,
            });
        }
    }
    
    // Gérer if/then/else
    if let Some(ref if_schema) = schema.if_ {
        let if_path = format!("{}/if", schema_path);
        let condition_met = validate_json_schema_with_path(
            if_schema,
            instance,
            instance_path,
            &if_path,
            root_defs,
        )
        .is_ok();
        
        if condition_met {
            if let Some(ref then) = schema.then {
                let then_path = format!("{}/then", schema_path);
                if let Err(mut errs) = validate_json_schema_with_path(
                    then,
                    instance,
                    instance_path,
                    &then_path,
                    root_defs,
                ) {
                    errors.append(&mut errs);
                }
            }
        } else {
            if let Some(ref else_) = schema.else_ {
                let else_path = format!("{}/else", schema_path);
                if let Err(mut errs) = validate_json_schema_with_path(
                    else_,
                    instance,
                    instance_path,
                    &else_path,
                    root_defs,
                ) {
                    errors.append(&mut errs);
                }
            }
        }
    }
    
    // Gérer type
    if let Some(ref type_) = schema.type_ {
        if !validate_type(type_, instance, instance_path, schema_path) {
            errors.push(ValidationError {
                instance_path: instance_path.to_string(),
                schema_path: format!("{}/type", schema_path),
            });
        }
    }
    
    // Gérer enum
    if let Some(ref enum_) = schema.enum_ {
        if !enum_.contains(instance) {
            errors.push(ValidationError {
                instance_path: instance_path.to_string(),
                schema_path: format!("{}/enum", schema_path),
            });
        }
    }
    
    // Gérer const
    if let Some(ref const_) = schema.const_ {
        if instance != const_ {
            errors.push(ValidationError {
                instance_path: instance_path.to_string(),
                schema_path: format!("{}/const", schema_path),
            });
        }
    }
    
    // Gérer les tableaux (prefixItems, items, etc.)
    if instance.is_array() {
        if let Err(mut arr_errors) = validate_array(
            schema,
            instance,
            instance_path,
            schema_path,
            root_defs,
        ) {
            errors.append(&mut arr_errors);
        }
    }
    
    // Gérer les objets (properties, patternProperties, etc.)
    if instance.is_object() {
        if let Err(mut obj_errors) = validate_object(
            schema,
            instance,
            instance_path,
            schema_path,
            root_defs,
        ) {
            errors.append(&mut obj_errors);
        }
    }
    
    // Gérer les chaînes (minLength, maxLength, pattern)
    if instance.is_string() {
        if let Err(mut str_errors) = validate_string(schema, instance, instance_path, schema_path) {
            errors.append(&mut str_errors);
        }
    }
    
    // Gérer les nombres (minimum, maximum, multipleOf, etc.)
    if instance.is_number() {
        if let Err(mut num_errors) = validate_number(schema, instance, instance_path, schema_path) {
            errors.append(&mut num_errors);
        }
    }
    
    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

fn validate_type(
    type_: &JsonSchemaType,
    instance: &Value,
    _instance_path: &str,
    _schema_path: &str,
) -> bool {
    match type_ {
        JsonSchemaType::Null => instance.is_null(),
        JsonSchemaType::Boolean => instance.is_boolean(),
        JsonSchemaType::Object => instance.is_object(),
        JsonSchemaType::Array => instance.is_array(),
        JsonSchemaType::Number => instance.is_number(),
        JsonSchemaType::String => instance.is_string(),
        JsonSchemaType::Integer => {
            if instance.as_i64().is_some() {
                true
            } else if let Some(n) = instance.as_f64() {
                n.fract() == 0.0
            } else {
                false
            }
        }
    }
}

fn validate_array(
    schema: &JsonSchemaObject,
    instance: &Value,
    instance_path: &str,
    schema_path: &str,
    root_defs: Option<&HashMap<String, Box<JsonSchema2020>>>,
) -> ValidationResult {
    let arr = instance.as_array().unwrap();
    let mut errors = Vec::new();
    
    // Valider prefixItems
    if let Some(ref prefix_items) = schema.prefix_items {
        for (i, item_schema) in prefix_items.iter().enumerate() {
            if let Some(item) = arr.get(i) {
                let item_path = format!("{}/{}", instance_path, i);
                let item_schema_path = format!("{}/prefixItems/{}", schema_path, i);
                if let Err(mut errs) = validate_json_schema_with_path(
                    item_schema,
                    item,
                    &item_path,
                    &item_schema_path,
                    root_defs,
                ) {
                    errors.append(&mut errs);
                }
            }
        }
    }
    
    // Valider items pour les éléments restants
    let prefix_len = schema.prefix_items.as_ref().map(|v| v.len()).unwrap_or(0);
    if let Some(ref items) = schema.items {
        match items {
            JsonSchemaItems::Schema(item_schema) => {
                for i in prefix_len..arr.len() {
                    if let Some(item) = arr.get(i) {
                        let item_path = format!("{}/{}", instance_path, i);
                        let item_schema_path = format!("{}/items", schema_path);
                        if let Err(mut errs) = validate_json_schema_with_path(
                            item_schema,
                            item,
                            &item_path,
                            &item_schema_path,
                            root_defs,
                        ) {
                            errors.append(&mut errs);
                        }
                    }
                }
            }
            JsonSchemaItems::Boolean(false) => {
                // Interdit les éléments supplémentaires
                if arr.len() > prefix_len {
                    for i in prefix_len..arr.len() {
                        errors.push(ValidationError {
                            instance_path: format!("{}/{}", instance_path, i),
                            schema_path: format!("{}/items", schema_path),
                        });
                    }
                }
            }
            JsonSchemaItems::Boolean(true) => {
                // Accepte tous les éléments supplémentaires (comportement par défaut)
            }
        }
    }
    
    // Valider minItems
    if let Some(min_items) = schema.min_items {
        if arr.len() < min_items as usize {
            errors.push(ValidationError {
                instance_path: instance_path.to_string(),
                schema_path: format!("{}/minItems", schema_path),
            });
        }
    }
    
    // Valider maxItems
    if let Some(max_items) = schema.max_items {
        if arr.len() > max_items as usize {
            errors.push(ValidationError {
                instance_path: instance_path.to_string(),
                schema_path: format!("{}/maxItems", schema_path),
            });
        }
    }
    
    // Valider uniqueItems
    if let Some(true) = schema.unique_items {
        let mut seen = std::collections::HashSet::new();
        for (i, item) in arr.iter().enumerate() {
            // Comparaison simple basée sur la représentation JSON
            let item_str = serde_json::to_string(item).unwrap_or_default();
            if !seen.insert(item_str) {
                errors.push(ValidationError {
                    instance_path: format!("{}/{}", instance_path, i),
                    schema_path: format!("{}/uniqueItems", schema_path),
                });
            }
        }
    }
    
    // Valider contains
    if let Some(ref contains) = schema.contains {
        let mut found = false;
        for (i, item) in arr.iter().enumerate() {
            let item_path = format!("{}/{}", instance_path, i);
            let contains_path = format!("{}/contains", schema_path);
            if validate_json_schema_with_path(
                contains,
                item,
                &item_path,
                &contains_path,
                root_defs,
            )
            .is_ok()
            {
                found = true;
                break;
            }
        }
        if !found {
            errors.push(ValidationError {
                instance_path: instance_path.to_string(),
                schema_path: format!("{}/contains", schema_path),
            });
        }
    }
    
    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

fn validate_object(
    schema: &JsonSchemaObject,
    instance: &Value,
    instance_path: &str,
    schema_path: &str,
    root_defs: Option<&HashMap<String, Box<JsonSchema2020>>>,
) -> ValidationResult {
    let obj = instance.as_object().unwrap();
    let mut errors = Vec::new();
    let mut evaluated_keys = std::collections::HashSet::new();
    
    // Valider properties
    if let Some(ref properties) = schema.properties {
        for (key, prop_schema) in properties {
            evaluated_keys.insert(key.clone());
            if let Some(value) = obj.get(key) {
                let prop_path = format!("{}/{}", instance_path, escape_json_pointer(key));
                let prop_schema_path = format!("{}/properties/{}", schema_path, escape_json_pointer(key));
                if let Err(mut errs) = validate_json_schema_with_path(
                    prop_schema,
                    value,
                    &prop_path,
                    &prop_schema_path,
                    root_defs,
                ) {
                    errors.append(&mut errs);
                }
            } else if let Some(ref required) = schema.required {
                if required.contains(key) {
                    errors.push(ValidationError {
                        instance_path: instance_path.to_string(),
                        schema_path: format!("{}/properties/{}", schema_path, escape_json_pointer(key)),
                    });
                }
            }
        }
    }
    
    // Valider optionalProperties (extension JTD)
    if let Some(ref optional_properties) = schema.optional_properties {
        for (key, opt_schema) in optional_properties {
            evaluated_keys.insert(key.clone());
            if let Some(value) = obj.get(key) {
                let opt_path = format!("{}/{}", instance_path, escape_json_pointer(key));
                let opt_schema_path = format!("{}/optionalProperties/{}", schema_path, escape_json_pointer(key));
                if let Err(mut errs) = validate_json_schema_with_path(
                    opt_schema,
                    value,
                    &opt_path,
                    &opt_schema_path,
                    root_defs,
                ) {
                    errors.append(&mut errs);
                }
            }
        }
    }
    
    // Valider patternProperties
    if let Some(ref pattern_properties) = schema.pattern_properties {
        for (pattern_str, pattern_schema) in pattern_properties {
            let pattern = match Regex::new(pattern_str) {
                Ok(re) => re,
                Err(_) => {
                    errors.push(ValidationError {
                        instance_path: instance_path.to_string(),
                        schema_path: format!("{}/patternProperties/{}", schema_path, pattern_str),
                    });
                    continue;
                }
            };
            
            for (key, value) in obj {
                if pattern.is_match(key) {
                    evaluated_keys.insert(key.clone());
                    let prop_path = format!("{}/{}", instance_path, escape_json_pointer(key));
                    let prop_schema_path = format!("{}/patternProperties/{}", schema_path, pattern_str);
                    if let Err(mut errs) = validate_json_schema_with_path(
                        pattern_schema,
                        value,
                        &prop_path,
                        &prop_schema_path,
                        root_defs,
                    ) {
                        errors.append(&mut errs);
                    }
                }
            }
        }
    }
    
    // Valider additionalProperties
    if let Some(ref additional_properties) = schema.additional_properties {
        match additional_properties {
            JsonSchemaAdditionalProperties::Schema(add_schema) => {
                for (key, value) in obj {
                    if !evaluated_keys.contains(key) {
                        let prop_path = format!("{}/{}", instance_path, escape_json_pointer(key));
                        let prop_schema_path = format!("{}/additionalProperties", schema_path);
                        if let Err(mut errs) = validate_json_schema_with_path(
                            add_schema,
                            value,
                            &prop_path,
                            &prop_schema_path,
                            root_defs,
                        ) {
                            errors.append(&mut errs);
                        }
                    }
                }
            }
            JsonSchemaAdditionalProperties::Boolean(false) => {
                // Rejette les propriétés supplémentaires
                for key in obj.keys() {
                    if !evaluated_keys.contains(key) {
                        errors.push(ValidationError {
                            instance_path: format!("{}/{}", instance_path, escape_json_pointer(key)),
                            schema_path: format!("{}/additionalProperties", schema_path),
                        });
                    }
                }
            }
            JsonSchemaAdditionalProperties::Boolean(true) => {
                // Accepte toutes les propriétés supplémentaires
            }
        }
    }
    
    // Valider minProperties
    if let Some(min_properties) = schema.min_properties {
        if obj.len() < min_properties as usize {
            errors.push(ValidationError {
                instance_path: instance_path.to_string(),
                schema_path: format!("{}/minProperties", schema_path),
            });
        }
    }
    
    // Valider maxProperties
    if let Some(max_properties) = schema.max_properties {
        if obj.len() > max_properties as usize {
            errors.push(ValidationError {
                instance_path: instance_path.to_string(),
                schema_path: format!("{}/maxProperties", schema_path),
            });
        }
    }
    
    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

fn validate_string(
    schema: &JsonSchemaObject,
    instance: &Value,
    instance_path: &str,
    schema_path: &str,
) -> ValidationResult {
    let s = instance.as_str().unwrap();
    let mut errors = Vec::new();
    
    // Valider minLength
    if let Some(min_length) = schema.min_length {
        if s.chars().count() < min_length as usize {
            errors.push(ValidationError {
                instance_path: instance_path.to_string(),
                schema_path: format!("{}/minLength", schema_path),
            });
        }
    }
    
    // Valider maxLength
    if let Some(max_length) = schema.max_length {
        if s.chars().count() > max_length as usize {
            errors.push(ValidationError {
                instance_path: instance_path.to_string(),
                schema_path: format!("{}/maxLength", schema_path),
            });
        }
    }
    
    // Valider pattern
    if let Some(ref pattern_str) = schema.pattern {
        let pattern = match Regex::new(pattern_str) {
            Ok(re) => re,
            Err(_) => {
                errors.push(ValidationError {
                    instance_path: instance_path.to_string(),
                    schema_path: format!("{}/pattern", schema_path),
                });
                return if errors.is_empty() { Ok(()) } else { Err(errors) };
            }
        };
        
        if !pattern.is_match(s) {
            errors.push(ValidationError {
                instance_path: instance_path.to_string(),
                schema_path: format!("{}/pattern", schema_path),
            });
        }
    }
    
    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

fn validate_number(
    schema: &JsonSchemaObject,
    instance: &Value,
    instance_path: &str,
    schema_path: &str,
) -> ValidationResult {
    let mut errors = Vec::new();
    
    let num = if let Some(n) = instance.as_f64() {
        n
    } else if let Some(n) = instance.as_i64() {
        n as f64
    } else {
        return Err(vec![ValidationError {
            instance_path: instance_path.to_string(),
            schema_path: format!("{}/type", schema_path),
        }]);
    };
    
    // Valider multipleOf
    if let Some(multiple_of) = schema.multiple_of {
        let remainder = (num / multiple_of).fract();
        if remainder.abs() > f64::EPSILON && (remainder - 1.0).abs() > f64::EPSILON {
            errors.push(ValidationError {
                instance_path: instance_path.to_string(),
                schema_path: format!("{}/multipleOf", schema_path),
            });
        }
    }
    
    // Valider minimum
    if let Some(minimum) = schema.minimum {
        if num < minimum {
            errors.push(ValidationError {
                instance_path: instance_path.to_string(),
                schema_path: format!("{}/minimum", schema_path),
            });
        }
    }
    
    // Valider maximum
    if let Some(maximum) = schema.maximum {
        if num > maximum {
            errors.push(ValidationError {
                instance_path: instance_path.to_string(),
                schema_path: format!("{}/maximum", schema_path),
            });
        }
    }
    
    // Valider exclusiveMinimum
    if let Some(exclusive_minimum) = schema.exclusive_minimum {
        if num <= exclusive_minimum {
            errors.push(ValidationError {
                instance_path: instance_path.to_string(),
                schema_path: format!("{}/exclusiveMinimum", schema_path),
            });
        }
    }
    
    // Valider exclusiveMaximum
    if let Some(exclusive_maximum) = schema.exclusive_maximum {
        if num >= exclusive_maximum {
            errors.push(ValidationError {
                instance_path: instance_path.to_string(),
                schema_path: format!("{}/exclusiveMaximum", schema_path),
            });
        }
    }
    
    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

fn escape_json_pointer(s: &str) -> String {
    s.replace("~", "~0").replace("/", "~1")
}
