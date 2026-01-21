use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Représente un schéma JTD selon la RFC 8927
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JtdSchema {
    /// Les 8 formes de schéma (mutuellement exclusives)
    #[serde(flatten)]
    pub form: SchemaForm,
    
    /// Si true, l'instance peut être null
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nullable: Option<bool>,
    
    /// Metadata (ignorée lors de la validation)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
    
    /// Définitions (uniquement au niveau racine)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub definitions: Option<HashMap<String, Box<JtdSchema>>>,
}

/// Les 8 formes de schéma mutuellement exclusives
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SchemaForm {
    /// Forme "ref" - référence à une définition
    Ref {
        #[serde(rename = "ref")]
        ref_name: String,
    },
    
    /// Forme "type" - type primitif
    Type {
        #[serde(rename = "type")]
        type_name: TypeName,
    },
    
    /// Forme "enum" - énumération de chaînes
    Enum {
        #[serde(rename = "enum")]
        enum_values: Vec<String>,
    },
    
    /// Forme "elements" - tableau d'éléments
    Elements {
        elements: Box<JtdSchema>,
    },
    
    /// Forme "values" - objet avec valeurs uniformes
    Values {
        values: Box<JtdSchema>,
    },
    
    /// Forme "properties" - objet avec propriétés nommées
    Properties {
        properties: HashMap<String, Box<JtdSchema>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "optionalProperties")]
        optional_properties: Option<HashMap<String, Box<JtdSchema>>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "additionalProperties")]
        additional_properties: Option<bool>,
    },
    
    /// Forme "discriminator" - objet avec tag discriminant
    Discriminator {
        discriminator: String,
        mapping: HashMap<String, Box<JtdSchema>>,
    },
    
    /// Forme "empty" - accepte tout (doit être en dernier pour l'ordre de désérialisation)
    /// Note: Empty ne doit correspondre qu'à un objet vide ou avec uniquement nullable/metadata
    Empty {},
}

/// Types primitifs supportés par JTD
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TypeName {
    Boolean,
    String,
    Timestamp,
    Float32,
    Float64,
    Int8,
    Uint8,
    Int16,
    Uint16,
    Int32,
    Uint32,
}

impl TypeName {
    /// Vérifie si un type est valide selon la RFC 8927
    #[allow(dead_code)]
    pub fn is_valid(s: &str) -> bool {
        matches!(
            s,
            "boolean" | "string" | "timestamp" | "float32" | "float64"
                | "int8" | "uint8" | "int16" | "uint16" | "int32" | "uint32"
        )
    }
}

impl JtdSchema {
    /// Crée un schéma "empty"
    #[allow(dead_code)]
    pub fn empty() -> Self {
        Self {
            form: SchemaForm::Empty {},
            nullable: None,
            metadata: None,
            definitions: None,
        }
    }
    
    /// Vérifie si le schéma est de forme "empty"
    pub fn is_empty(&self) -> bool {
        matches!(self.form, SchemaForm::Empty {})
    }
}
