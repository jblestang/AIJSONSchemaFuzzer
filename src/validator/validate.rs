use crate::error::{ValidationError, ValidationResult};
use crate::schema::ast::{JtdSchema, SchemaForm, TypeName};
use serde_json::Value;
use std::collections::HashMap;

/// Valide une instance JSON contre un schéma JTD
pub fn validate(
    schema: &JtdSchema,
    instance: &Value,
    instance_path: &str,
    schema_path: &str,
) -> ValidationResult {
    validate_with_definitions(schema, instance, instance_path, schema_path, &schema.definitions)
}

/// Valide avec les définitions du schéma racine
fn validate_with_definitions(
    schema: &JtdSchema,
    instance: &Value,
    instance_path: &str,
    schema_path: &str,
    root_definitions: &Option<HashMap<String, Box<JtdSchema>>>,
) -> ValidationResult {
    let mut errors = Vec::new();
    
    // Gérer nullable
    if instance.is_null() {
        if schema.nullable == Some(true) {
            return Ok(());
        } else if !schema.is_empty() {
            errors.push(ValidationError {
                instance_path: instance_path.to_string(),
                schema_path: schema_path.to_string(),
            });
            return Err(errors);
        }
    }
    
    // Valider selon la forme du schéma
    match &schema.form {
        SchemaForm::Empty {} => {
            // Forme empty accepte tout
            Ok(())
        }
        SchemaForm::Ref { ref_name } => {
            validate_ref(root_definitions, instance, instance_path, schema_path, ref_name)
        }
        SchemaForm::Type { type_name } => {
            validate_type(type_name, instance, instance_path, schema_path)
        }
        SchemaForm::Enum { enum_values } => {
            validate_enum(enum_values, instance, instance_path, schema_path)
        }
        SchemaForm::Elements { elements } => {
            validate_elements(elements, instance, instance_path, schema_path, root_definitions)
        }
        SchemaForm::Values { values } => {
            validate_values(values, instance, instance_path, schema_path, root_definitions)
        }
        SchemaForm::Properties {
            properties,
            optional_properties,
            additional_properties,
        } => {
            validate_properties(
                properties,
                optional_properties,
                additional_properties.unwrap_or(false),
                instance,
                instance_path,
                schema_path,
                root_definitions,
            )
        }
        SchemaForm::Discriminator {
            discriminator,
            mapping,
        } => {
            validate_discriminator(
                discriminator,
                mapping,
                instance,
                instance_path,
                schema_path,
                root_definitions,
            )
        }
    }
}

fn validate_ref(
    root_definitions: &Option<HashMap<String, Box<JtdSchema>>>,
    instance: &Value,
    instance_path: &str,
    schema_path: &str,
    ref_name: &str,
) -> ValidationResult {
    if let Some(definitions) = root_definitions {
        if let Some(ref_schema) = definitions.get(ref_name) {
            validate_with_definitions(ref_schema, instance, instance_path, schema_path, root_definitions)
        } else {
            Err(vec![ValidationError {
                instance_path: instance_path.to_string(),
                schema_path: format!("{}/definitions/{}", schema_path, ref_name),
            }])
        }
    } else {
        Err(vec![ValidationError {
            instance_path: instance_path.to_string(),
            schema_path: schema_path.to_string(),
        }])
    }
}

fn validate_type(
    type_name: &TypeName,
    instance: &Value,
    instance_path: &str,
    schema_path: &str,
) -> ValidationResult {
    match type_name {
        TypeName::Boolean => {
            if instance.is_boolean() {
                Ok(())
            } else {
                Err(vec![ValidationError {
                    instance_path: instance_path.to_string(),
                    schema_path: schema_path.to_string(),
                }])
            }
        }
        TypeName::String => {
            if instance.is_string() {
                Ok(())
            } else {
                Err(vec![ValidationError {
                    instance_path: instance_path.to_string(),
                    schema_path: schema_path.to_string(),
                }])
            }
        }
        TypeName::Timestamp => {
            if let Some(s) = instance.as_str() {
                // Vérifier format RFC3339
                if chrono::DateTime::parse_from_rfc3339(s).is_ok() {
                    Ok(())
                } else {
                    Err(vec![ValidationError {
                        instance_path: instance_path.to_string(),
                        schema_path: schema_path.to_string(),
                    }])
                }
            } else {
                Err(vec![ValidationError {
                    instance_path: instance_path.to_string(),
                    schema_path: schema_path.to_string(),
                }])
            }
        }
        TypeName::Float32 | TypeName::Float64 => {
            if instance.is_number() {
                // Vérifier les limites selon le type
                if let Some(n) = instance.as_f64() {
                    match type_name {
                        TypeName::Float32 => {
                            // Vérifier si dans la plage float32
                            if n >= f32::MIN as f64 && n <= f32::MAX as f64 {
                                Ok(())
                            } else {
                                Err(vec![ValidationError {
                                    instance_path: instance_path.to_string(),
                                    schema_path: schema_path.to_string(),
                                }])
                            }
                        }
                        TypeName::Float64 => Ok(()),
                        _ => unreachable!(),
                    }
                } else {
                    Err(vec![ValidationError {
                        instance_path: instance_path.to_string(),
                        schema_path: schema_path.to_string(),
                    }])
                }
            } else {
                Err(vec![ValidationError {
                    instance_path: instance_path.to_string(),
                    schema_path: schema_path.to_string(),
                }])
            }
        }
        TypeName::Int8 => validate_integer_range(instance, instance_path, schema_path, -128, 127),
        TypeName::Uint8 => validate_integer_range(instance, instance_path, schema_path, 0, 255),
        TypeName::Int16 => {
            validate_integer_range(instance, instance_path, schema_path, -32768, 32767)
        }
        TypeName::Uint16 => {
            validate_integer_range(instance, instance_path, schema_path, 0, 65535)
        }
        TypeName::Int32 => {
            validate_integer_range(instance, instance_path, schema_path, -2147483648, 2147483647)
        }
        TypeName::Uint32 => {
            validate_integer_range(instance, instance_path, schema_path, 0, 4294967295)
        }
    }
}

fn validate_integer_range(
    instance: &Value,
    instance_path: &str,
    schema_path: &str,
    min: i64,
    max: i64,
) -> ValidationResult {
    if let Some(n) = instance.as_i64() {
        if n >= min && n <= max {
            Ok(())
        } else {
            Err(vec![ValidationError {
                instance_path: instance_path.to_string(),
                schema_path: schema_path.to_string(),
            }])
        }
    } else {
        Err(vec![ValidationError {
            instance_path: instance_path.to_string(),
            schema_path: schema_path.to_string(),
        }])
    }
}

fn validate_enum(
    enum_values: &[String],
    instance: &Value,
    instance_path: &str,
    schema_path: &str,
) -> ValidationResult {
    if let Some(s) = instance.as_str() {
        if enum_values.contains(&s.to_string()) {
            Ok(())
        } else {
            Err(vec![ValidationError {
                instance_path: instance_path.to_string(),
                schema_path: schema_path.to_string(),
            }])
        }
    } else {
        Err(vec![ValidationError {
            instance_path: instance_path.to_string(),
            schema_path: schema_path.to_string(),
        }])
    }
}

fn validate_elements(
    elements: &JtdSchema,
    instance: &Value,
    instance_path: &str,
    schema_path: &str,
    root_definitions: &Option<HashMap<String, Box<JtdSchema>>>,
) -> ValidationResult {
    if let Some(arr) = instance.as_array() {
        let mut errors = Vec::new();
        for (i, item) in arr.iter().enumerate() {
            let item_path = format!("{}/{}", instance_path, i);
            let item_schema_path = format!("{}/elements", schema_path);
            if let Err(mut errs) = validate_with_definitions(elements, item, &item_path, &item_schema_path, root_definitions) {
                errors.append(&mut errs);
            }
        }
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    } else {
        Err(vec![ValidationError {
            instance_path: instance_path.to_string(),
            schema_path: format!("{}/elements", schema_path),
        }])
    }
}

fn validate_values(
    values: &JtdSchema,
    instance: &Value,
    instance_path: &str,
    schema_path: &str,
    root_definitions: &Option<HashMap<String, Box<JtdSchema>>>,
) -> ValidationResult {
    if let Some(obj) = instance.as_object() {
        let mut errors = Vec::new();
        for (key, value) in obj {
            let value_path = format!("{}/{}", instance_path, escape_json_pointer(key));
            let value_schema_path = format!("{}/values", schema_path);
            if let Err(mut errs) = validate_with_definitions(values, value, &value_path, &value_schema_path, root_definitions) {
                errors.append(&mut errs);
            }
        }
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    } else {
        Err(vec![ValidationError {
            instance_path: instance_path.to_string(),
            schema_path: format!("{}/values", schema_path),
        }])
    }
}

fn validate_properties(
    properties: &HashMap<String, Box<JtdSchema>>,
    optional_properties: &Option<HashMap<String, Box<JtdSchema>>>,
    additional_properties: bool,
    instance: &Value,
    instance_path: &str,
    schema_path: &str,
    root_definitions: &Option<HashMap<String, Box<JtdSchema>>>,
) -> ValidationResult {
    if let Some(obj) = instance.as_object() {
        let mut errors = Vec::new();
        
        // Vérifier les propriétés requises
        for (prop_name, prop_schema) in properties {
            if let Some(prop_value) = obj.get(prop_name) {
                let prop_path = format!("{}/{}", instance_path, escape_json_pointer(prop_name));
                let prop_schema_path = format!("{}/properties/{}", schema_path, escape_json_pointer(prop_name));
                if let Err(mut errs) = validate_with_definitions(prop_schema, prop_value, &prop_path, &prop_schema_path, root_definitions) {
                    errors.append(&mut errs);
                }
            } else {
                errors.push(ValidationError {
                    instance_path: instance_path.to_string(),
                    schema_path: format!("{}/properties/{}", schema_path, escape_json_pointer(prop_name)),
                });
            }
        }
        
        // Vérifier les propriétés optionnelles
        if let Some(opt_props) = optional_properties {
            for (opt_name, opt_schema) in opt_props {
                if let Some(opt_value) = obj.get(opt_name) {
                    let opt_path = format!("{}/{}", instance_path, escape_json_pointer(opt_name));
                    let opt_schema_path = format!("{}/optionalProperties/{}", schema_path, escape_json_pointer(opt_name));
                    if let Err(mut errs) = validate_with_definitions(opt_schema, opt_value, &opt_path, &opt_schema_path, root_definitions) {
                        errors.append(&mut errs);
                    }
                }
            }
        }
        
        // Vérifier les propriétés supplémentaires
        // additionalProperties: false (défaut) rejette les propriétés supplémentaires
        // additionalProperties: true accepte les propriétés supplémentaires
        if !additional_properties {
            let mut allowed_keys: std::collections::HashSet<&String> = properties.keys().collect();
            if let Some(opt_props) = optional_properties {
                allowed_keys.extend(opt_props.keys());
            }
            
            for key in obj.keys() {
                if !allowed_keys.contains(key) {
                    errors.push(ValidationError {
                        instance_path: format!("{}/{}", instance_path, escape_json_pointer(key)),
                        schema_path: schema_path.to_string(),
                    });
                }
            }
        }
        // Si additional_properties est true, on n'ajoute pas d'erreur pour les propriétés supplémentaires
        
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    } else {
        Err(vec![ValidationError {
            instance_path: instance_path.to_string(),
            schema_path: format!("{}/properties", schema_path),
        }])
    }
}

fn validate_discriminator(
    discriminator: &str,
    mapping: &HashMap<String, Box<JtdSchema>>,
    instance: &Value,
    instance_path: &str,
    schema_path: &str,
    root_definitions: &Option<HashMap<String, Box<JtdSchema>>>,
) -> ValidationResult {
    if let Some(obj) = instance.as_object() {
        // L'instance doit avoir la clé discriminator
        if let Some(tag_value) = obj.get(discriminator) {
            if let Some(tag_str) = tag_value.as_str() {
                // Le tag doit être dans le mapping
                if let Some(tag_schema) = mapping.get(tag_str) {
                    // Valider l'instance contre le schéma du mapping
                    // Note: selon la RFC, le tag discriminator est "exempt" de validation
                    // dans le schéma du mapping (discriminator tag exemption)
                    // On crée une copie de l'instance sans le tag pour la validation
                    let mut instance_without_tag = instance.clone();
                    if let Some(obj) = instance_without_tag.as_object_mut() {
                        obj.remove(discriminator);
                    }
                    validate_with_definitions(tag_schema, &instance_without_tag, instance_path, &format!("{}/mapping/{}", schema_path, escape_json_pointer(tag_str)), root_definitions)
                } else {
                    Err(vec![ValidationError {
                        instance_path: format!("{}/{}", instance_path, escape_json_pointer(discriminator)),
                        schema_path: format!("{}/mapping", schema_path),
                    }])
                }
            } else {
                Err(vec![ValidationError {
                    instance_path: format!("{}/{}", instance_path, escape_json_pointer(discriminator)),
                    schema_path: format!("{}/discriminator", schema_path),
                }])
            }
        } else {
            Err(vec![ValidationError {
                instance_path: instance_path.to_string(),
                schema_path: format!("{}/discriminator", schema_path),
            }])
        }
    } else {
        Err(vec![ValidationError {
            instance_path: instance_path.to_string(),
            schema_path: format!("{}/discriminator", schema_path),
        }])
    }
}

/// Échappe une clé pour un JSON Pointer selon RFC 6901
fn escape_json_pointer(s: &str) -> String {
    s.replace("~", "~0").replace("/", "~1")
}
