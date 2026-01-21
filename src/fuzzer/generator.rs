use crate::schema::ast::{JtdSchema, SchemaForm, TypeName};
use rand::Rng;
use serde_json::Value;

/// Génère un JSON valide selon un schéma JTD
pub fn generate_valid_json(schema: &JtdSchema) -> Value {
    generate_with_definitions(schema, &schema.definitions)
}

pub(crate) fn generate_with_definitions(
    schema: &JtdSchema,
    root_definitions: &Option<std::collections::HashMap<String, Box<JtdSchema>>>,
) -> Value {
    match &schema.form {
        SchemaForm::Empty {} => {
            // Génère une valeur arbitraire
            generate_arbitrary_value()
        }
        SchemaForm::Ref { ref_name } => {
            if let Some(defs) = root_definitions {
                if let Some(ref_schema) = defs.get(ref_name) {
                    generate_with_definitions(ref_schema, root_definitions)
                } else {
                    Value::Null
                }
            } else {
                Value::Null
            }
        }
        SchemaForm::Type { type_name } => generate_type_value(type_name),
        SchemaForm::Enum { enum_values } => {
            if enum_values.is_empty() {
                Value::Null
            } else {
                let idx = rand::thread_rng().gen_range(0..enum_values.len());
                Value::String(enum_values[idx].clone())
            }
        }
        SchemaForm::Elements { elements } => {
            let size = rand::thread_rng().gen_range(0..=5);
            let arr: Vec<Value> = (0..size)
                .map(|_| generate_with_definitions(elements, root_definitions))
                .collect();
            Value::Array(arr)
        }
        SchemaForm::Values { values } => {
            let size = rand::thread_rng().gen_range(0..=5);
            let mut obj = serde_json::Map::new();
            for i in 0..size {
                let key = format!("key_{}", i);
                let value = generate_with_definitions(values, root_definitions);
                obj.insert(key, value);
            }
            Value::Object(obj)
        }
        SchemaForm::Properties {
            properties,
            optional_properties,
            ..
        } => {
            let mut obj = serde_json::Map::new();
            
            // Propriétés requises
            for (key, prop_schema) in properties {
                let value = generate_with_definitions(prop_schema, root_definitions);
                obj.insert(key.clone(), value);
            }
            
            // Propriétés optionnelles (50% de chance)
            if let Some(opt_props) = optional_properties {
                for (key, opt_schema) in opt_props {
                    if rand::thread_rng().gen_bool(0.5) {
                        let value = generate_with_definitions(opt_schema, root_definitions);
                        obj.insert(key.clone(), value);
                    }
                }
            }
            
            Value::Object(obj)
        }
        SchemaForm::Discriminator {
            discriminator,
            mapping,
        } => {
            let mut obj = serde_json::Map::new();
            
            // Choisir un tag aléatoire
            let tags: Vec<&String> = mapping.keys().collect();
            if let Some(tag) = tags.get(rand::thread_rng().gen_range(0..tags.len())) {
                obj.insert(discriminator.clone(), Value::String((*tag).clone()));
                
                // Générer selon le schéma du mapping
                if let Some(tag_schema) = mapping.get(*tag) {
                    let tag_obj = generate_with_definitions(tag_schema, root_definitions);
                    if let Value::Object(tag_map) = tag_obj {
                        obj.extend(tag_map);
                    }
                }
            }
            
            Value::Object(obj)
        }
    }
}

fn generate_type_value(type_name: &TypeName) -> Value {
    let mut rng = rand::thread_rng();
    
    match type_name {
        TypeName::Boolean => Value::Bool(rng.gen()),
        TypeName::String => {
            let len = rng.gen_range(0..=20);
            let chars: String = (0..len)
                .map(|_| rng.gen_range(b'a'..=b'z') as char)
                .collect();
            Value::String(chars)
        }
        TypeName::Timestamp => {
            // Génère un timestamp RFC3339 valide
            let year = rng.gen_range(2000..=2100);
            let month = rng.gen_range(1..=12);
            let day = rng.gen_range(1..=28);
            let hour = rng.gen_range(0..=23);
            let minute = rng.gen_range(0..=59);
            let second = rng.gen_range(0..=59);
            Value::String(format!(
                "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}Z",
                year, month, day, hour, minute, second
            ))
        }
        TypeName::Float32 => Value::Number(
            serde_json::Number::from_f64(rng.gen_range(-1000.0..1000.0) as f64)
                .unwrap_or(serde_json::Number::from(0)),
        ),
        TypeName::Float64 => Value::Number(
            serde_json::Number::from_f64(rng.gen_range(-1000.0..1000.0))
                .unwrap_or(serde_json::Number::from(0)),
        ),
        TypeName::Int8 => Value::Number(serde_json::Number::from(rng.gen_range(-128i64..=127))),
        TypeName::Uint8 => Value::Number(serde_json::Number::from(rng.gen_range(0u64..=255))),
        TypeName::Int16 => Value::Number(serde_json::Number::from(rng.gen_range(-32768i64..=32767))),
        TypeName::Uint16 => Value::Number(serde_json::Number::from(rng.gen_range(0u64..=65535))),
        TypeName::Int32 => Value::Number(serde_json::Number::from(
            rng.gen_range(-2147483648i64..=2147483647),
        )),
        TypeName::Uint32 => Value::Number(serde_json::Number::from(rng.gen_range(0u64..=4294967295))),
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
