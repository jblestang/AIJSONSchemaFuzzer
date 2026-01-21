use serde::{Deserialize, Serialize};
use std::fmt;

/// Représente une erreur de validation selon la RFC 8927
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ValidationError {
    /// JSON Pointer vers la partie de l'instance qui a causé l'erreur
    pub instance_path: String,
    /// JSON Pointer vers la partie du schéma qui a causé le rejet
    pub schema_path: String,
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "instance: {}, schema: {}",
            self.instance_path, self.schema_path
        )
    }
}

/// Résultat d'une validation
pub type ValidationResult = Result<(), Vec<ValidationError>>;

/// Erreurs internes du parser/validateur
#[derive(Debug, thiserror::Error)]
pub enum JtdError {
    #[error("Invalid JSON: {0}")]
    InvalidJson(#[from] serde_json::Error),
    
    #[error("Schema syntax error: {0}")]
    SchemaSyntaxError(String),
    
    #[error("Reference not found: {0}")]
    ReferenceNotFound(String),
    
    #[error("Circular reference detected: {0}")]
    CircularReference(String),
    
    #[error("Invalid enum: {0}")]
    InvalidEnum(String),
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}
