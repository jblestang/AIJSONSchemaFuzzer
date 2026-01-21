use crate::schema::ast::SchemaForm;

/// Noms des mutations syntaxiques
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SyntaxMutationName {
    MissingClosingBrace,
    MissingOpeningBrace,
    InvalidCharacter,
    CommaToSemicolon,
    RemoveQuotes,
    TrailingComma,
    ColonToEquals,
    TruncatedJson,
    MixedIndentation,
}

impl SyntaxMutationName {
    pub fn all() -> Vec<Self> {
        vec![
            SyntaxMutationName::MissingClosingBrace,
            SyntaxMutationName::MissingOpeningBrace,
            SyntaxMutationName::InvalidCharacter,
            SyntaxMutationName::CommaToSemicolon,
            SyntaxMutationName::RemoveQuotes,
            SyntaxMutationName::TrailingComma,
            SyntaxMutationName::ColonToEquals,
            SyntaxMutationName::TruncatedJson,
            SyntaxMutationName::MixedIndentation,
        ]
    }
    
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "missing-closing-brace" | "missing_closing_brace" => Some(SyntaxMutationName::MissingClosingBrace),
            "missing-opening-brace" | "missing_opening_brace" => Some(SyntaxMutationName::MissingOpeningBrace),
            "invalid-character" | "invalid_character" => Some(SyntaxMutationName::InvalidCharacter),
            "comma-to-semicolon" | "comma_to_semicolon" => Some(SyntaxMutationName::CommaToSemicolon),
            "remove-quotes" | "remove_quotes" => Some(SyntaxMutationName::RemoveQuotes),
            "trailing-comma" | "trailing_comma" => Some(SyntaxMutationName::TrailingComma),
            "colon-to-equals" | "colon_to_equals" => Some(SyntaxMutationName::ColonToEquals),
            "truncated-json" | "truncated_json" => Some(SyntaxMutationName::TruncatedJson),
            "mixed-indentation" | "mixed_indentation" => Some(SyntaxMutationName::MixedIndentation),
            _ => None,
        }
    }
    
    #[allow(dead_code)]
    pub fn to_string(&self) -> &'static str {
        match self {
            SyntaxMutationName::MissingClosingBrace => "missing-closing-brace",
            SyntaxMutationName::MissingOpeningBrace => "missing-opening-brace",
            SyntaxMutationName::InvalidCharacter => "invalid-character",
            SyntaxMutationName::CommaToSemicolon => "comma-to-semicolon",
            SyntaxMutationName::RemoveQuotes => "remove-quotes",
            SyntaxMutationName::TrailingComma => "trailing-comma",
            SyntaxMutationName::ColonToEquals => "colon-to-equals",
            SyntaxMutationName::TruncatedJson => "truncated-json",
            SyntaxMutationName::MixedIndentation => "mixed-indentation",
        }
    }
    
    pub fn description(&self) -> &'static str {
        match self {
            SyntaxMutationName::MissingClosingBrace => "Supprime une accolade fermante (} ou ])",
            SyntaxMutationName::MissingOpeningBrace => "Supprime une accolade ouvrant ({ ou [)",
            SyntaxMutationName::InvalidCharacter => "Ajoute un caractère invalide à la fin",
            SyntaxMutationName::CommaToSemicolon => "Remplace une virgule par un point-virgule",
            SyntaxMutationName::RemoveQuotes => "Supprime les guillemets des clés",
            SyntaxMutationName::TrailingComma => "Ajoute une virgule trailing",
            SyntaxMutationName::ColonToEquals => "Remplace : par =",
            SyntaxMutationName::TruncatedJson => "Tronque le JSON (moitié supprimée)",
            SyntaxMutationName::MixedIndentation => "Mélange tabulations et espaces dans l'indentation",
        }
    }
}

/// Noms des mutations sémantiques par forme de schéma
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SemanticMutationName {
    // Type
    WrongType,
    OutOfRange,
    NullForNonNullable,
    
    // Enum
    NotInEnum,
    SimilarButDifferent,
    EmptyString,
    
    // Elements
    NotAnArray,
    SingleInvalidElement,
    MixedTypes,
    AllInvalidElements,
    CompletelyDifferentTypes,
    EmptyArray,
    
    // Values
    NotAnObject,
    SingleInvalidValue,
    MultipleInvalidValues,
    
    // Properties
    NotAnObjectProp,
    AllRequiredMissing,
    OneRequiredMissing,
    AdditionalProperties,
    SingleInvalidProperty,
    AllInvalidProperties,
    InvalidOptionalProperty,
    NullForNonNullableProp,
    MissingPlusAdditional,
    EmptyObject,
    NullObject,
    
    // Discriminator
    NotAnObjectDisc,
    MissingTag,
    InvalidTag,
    TagNotString,
    InvalidInstance,
    
    // Ref
    InvalidReference,
    NonExistentReference,
    
    // Empty
    NullForEmpty,
}

impl SemanticMutationName {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            // Type
            "wrong-type" | "wrong_type" => Some(SemanticMutationName::WrongType),
            "out-of-range" | "out_of_range" => Some(SemanticMutationName::OutOfRange),
            "null-for-non-nullable" | "null_for_non_nullable" => Some(SemanticMutationName::NullForNonNullable),
            
            // Enum
            "not-in-enum" | "not_in_enum" => Some(SemanticMutationName::NotInEnum),
            "similar-but-different" | "similar_but_different" => Some(SemanticMutationName::SimilarButDifferent),
            "empty-string" | "empty_string" => Some(SemanticMutationName::EmptyString),
            
            // Elements
            "not-an-array" | "not_an_array" => Some(SemanticMutationName::NotAnArray),
            "single-invalid-element" | "single_invalid_element" => Some(SemanticMutationName::SingleInvalidElement),
            "mixed-types" | "mixed_types" => Some(SemanticMutationName::MixedTypes),
            "all-invalid-elements" | "all_invalid_elements" => Some(SemanticMutationName::AllInvalidElements),
            "completely-different-types" | "completely_different_types" => Some(SemanticMutationName::CompletelyDifferentTypes),
            "empty-array" | "empty_array" => Some(SemanticMutationName::EmptyArray),
            
            // Values
            "not-an-object" | "not_an_object" => Some(SemanticMutationName::NotAnObject),
            "single-invalid-value" | "single_invalid_value" => Some(SemanticMutationName::SingleInvalidValue),
            "multiple-invalid-values" | "multiple_invalid_values" => Some(SemanticMutationName::MultipleInvalidValues),
            
            // Properties
            "not-an-object-prop" | "not_an_object_prop" => Some(SemanticMutationName::NotAnObjectProp),
            "all-required-missing" | "all_required_missing" => Some(SemanticMutationName::AllRequiredMissing),
            "one-required-missing" | "one_required_missing" => Some(SemanticMutationName::OneRequiredMissing),
            "additional-properties" | "additional_properties" => Some(SemanticMutationName::AdditionalProperties),
            "single-invalid-property" | "single_invalid_property" => Some(SemanticMutationName::SingleInvalidProperty),
            "all-invalid-properties" | "all_invalid_properties" => Some(SemanticMutationName::AllInvalidProperties),
            "invalid-optional-property" | "invalid_optional_property" => Some(SemanticMutationName::InvalidOptionalProperty),
            "null-for-non-nullable-prop" | "null_for_non_nullable_prop" => Some(SemanticMutationName::NullForNonNullableProp),
            "missing-plus-additional" | "missing_plus_additional" => Some(SemanticMutationName::MissingPlusAdditional),
            "empty-object" | "empty_object" => Some(SemanticMutationName::EmptyObject),
            "null-object" | "null_object" => Some(SemanticMutationName::NullObject),
            
            // Discriminator
            "not-an-object-disc" | "not_an_object_disc" => Some(SemanticMutationName::NotAnObjectDisc),
            "missing-tag" | "missing_tag" => Some(SemanticMutationName::MissingTag),
            "invalid-tag" | "invalid_tag" => Some(SemanticMutationName::InvalidTag),
            "tag-not-string" | "tag_not_string" => Some(SemanticMutationName::TagNotString),
            "invalid-instance" | "invalid_instance" => Some(SemanticMutationName::InvalidInstance),
            
            // Ref
            "invalid-reference" | "invalid_reference" => Some(SemanticMutationName::InvalidReference),
            "non-existent-reference" | "non_existent_reference" => Some(SemanticMutationName::NonExistentReference),
            
            // Empty
            "null-for-empty" | "null_for_empty" => Some(SemanticMutationName::NullForEmpty),
            
            _ => None,
        }
    }
    
    #[allow(dead_code)]
    pub fn to_string(&self) -> &'static str {
        match self {
            SemanticMutationName::WrongType => "wrong-type",
            SemanticMutationName::OutOfRange => "out-of-range",
            SemanticMutationName::NullForNonNullable => "null-for-non-nullable",
            SemanticMutationName::NotInEnum => "not-in-enum",
            SemanticMutationName::SimilarButDifferent => "similar-but-different",
            SemanticMutationName::EmptyString => "empty-string",
            SemanticMutationName::NotAnArray => "not-an-array",
            SemanticMutationName::SingleInvalidElement => "single-invalid-element",
            SemanticMutationName::MixedTypes => "mixed-types",
            SemanticMutationName::AllInvalidElements => "all-invalid-elements",
            SemanticMutationName::CompletelyDifferentTypes => "completely-different-types",
            SemanticMutationName::EmptyArray => "empty-array",
            SemanticMutationName::NotAnObject => "not-an-object",
            SemanticMutationName::SingleInvalidValue => "single-invalid-value",
            SemanticMutationName::MultipleInvalidValues => "multiple-invalid-values",
            SemanticMutationName::NotAnObjectProp => "not-an-object-prop",
            SemanticMutationName::AllRequiredMissing => "all-required-missing",
            SemanticMutationName::OneRequiredMissing => "one-required-missing",
            SemanticMutationName::AdditionalProperties => "additional-properties",
            SemanticMutationName::SingleInvalidProperty => "single-invalid-property",
            SemanticMutationName::AllInvalidProperties => "all-invalid-properties",
            SemanticMutationName::InvalidOptionalProperty => "invalid-optional-property",
            SemanticMutationName::NullForNonNullableProp => "null-for-non-nullable-prop",
            SemanticMutationName::MissingPlusAdditional => "missing-plus-additional",
            SemanticMutationName::EmptyObject => "empty-object",
            SemanticMutationName::NullObject => "null-object",
            SemanticMutationName::NotAnObjectDisc => "not-an-object-disc",
            SemanticMutationName::MissingTag => "missing-tag",
            SemanticMutationName::InvalidTag => "invalid-tag",
            SemanticMutationName::TagNotString => "tag-not-string",
            SemanticMutationName::InvalidInstance => "invalid-instance",
            SemanticMutationName::InvalidReference => "invalid-reference",
            SemanticMutationName::NonExistentReference => "non-existent-reference",
            SemanticMutationName::NullForEmpty => "null-for-empty",
        }
    }
    
    pub fn description(&self) -> &'static str {
        match self {
            SemanticMutationName::WrongType => "Type incorrect",
            SemanticMutationName::OutOfRange => "Valeur hors plage",
            SemanticMutationName::NullForNonNullable => "Null pour type non-nullable",
            SemanticMutationName::NotInEnum => "Valeur non dans l'enum",
            SemanticMutationName::SimilarButDifferent => "Chaîne similaire mais différente",
            SemanticMutationName::EmptyString => "Chaîne vide",
            SemanticMutationName::NotAnArray => "Pas un tableau",
            SemanticMutationName::SingleInvalidElement => "Un seul élément invalide",
            SemanticMutationName::MixedTypes => "Types mixtes (valides et invalides)",
            SemanticMutationName::AllInvalidElements => "Tous les éléments invalides",
            SemanticMutationName::CompletelyDifferentTypes => "Types complètement différents",
            SemanticMutationName::EmptyArray => "Tableau vide",
            SemanticMutationName::NotAnObject => "Pas un objet",
            SemanticMutationName::SingleInvalidValue => "Une valeur invalide",
            SemanticMutationName::MultipleInvalidValues => "Plusieurs valeurs invalides",
            SemanticMutationName::NotAnObjectProp => "Pas un objet",
            SemanticMutationName::AllRequiredMissing => "Toutes les propriétés requises manquantes",
            SemanticMutationName::OneRequiredMissing => "Une propriété requise manquante",
            SemanticMutationName::AdditionalProperties => "Propriétés supplémentaires",
            SemanticMutationName::SingleInvalidProperty => "Une propriété invalide",
            SemanticMutationName::AllInvalidProperties => "Toutes les propriétés invalides",
            SemanticMutationName::InvalidOptionalProperty => "Propriété optionnelle invalide",
            SemanticMutationName::NullForNonNullableProp => "Null pour propriété non-nullable",
            SemanticMutationName::MissingPlusAdditional => "Propriété manquante + supplémentaire",
            SemanticMutationName::EmptyObject => "Objet vide",
            SemanticMutationName::NullObject => "Objet null",
            SemanticMutationName::NotAnObjectDisc => "Pas un objet",
            SemanticMutationName::MissingTag => "Tag manquant",
            SemanticMutationName::InvalidTag => "Tag invalide",
            SemanticMutationName::TagNotString => "Tag non-string",
            SemanticMutationName::InvalidInstance => "Instance invalide",
            SemanticMutationName::InvalidReference => "Référence invalide",
            SemanticMutationName::NonExistentReference => "Référence inexistante",
            SemanticMutationName::NullForEmpty => "Null pour empty",
        }
    }
    
    /// Retourne les mutations applicables pour une forme de schéma
    #[allow(dead_code)]
    pub fn for_schema_form(form: &SchemaForm) -> Vec<Self> {
        match form {
            SchemaForm::Type { .. } => vec![
                SemanticMutationName::WrongType,
                SemanticMutationName::OutOfRange,
                SemanticMutationName::NullForNonNullable,
            ],
            SchemaForm::Enum { .. } => vec![
                SemanticMutationName::NotInEnum,
                SemanticMutationName::SimilarButDifferent,
                SemanticMutationName::EmptyString,
            ],
            SchemaForm::Elements { .. } => vec![
                SemanticMutationName::NotAnArray,
                SemanticMutationName::SingleInvalidElement,
                SemanticMutationName::MixedTypes,
                SemanticMutationName::AllInvalidElements,
                SemanticMutationName::CompletelyDifferentTypes,
                SemanticMutationName::EmptyArray,
            ],
            SchemaForm::Values { .. } => vec![
                SemanticMutationName::NotAnObject,
                SemanticMutationName::SingleInvalidValue,
                SemanticMutationName::MultipleInvalidValues,
            ],
            SchemaForm::Properties { .. } => vec![
                SemanticMutationName::NotAnObjectProp,
                SemanticMutationName::AllRequiredMissing,
                SemanticMutationName::OneRequiredMissing,
                SemanticMutationName::AdditionalProperties,
                SemanticMutationName::SingleInvalidProperty,
                SemanticMutationName::AllInvalidProperties,
                SemanticMutationName::InvalidOptionalProperty,
                SemanticMutationName::NullForNonNullableProp,
                SemanticMutationName::MissingPlusAdditional,
                SemanticMutationName::EmptyObject,
                SemanticMutationName::NullObject,
            ],
            SchemaForm::Discriminator { .. } => vec![
                SemanticMutationName::NotAnObjectDisc,
                SemanticMutationName::MissingTag,
                SemanticMutationName::InvalidTag,
                SemanticMutationName::TagNotString,
                SemanticMutationName::InvalidInstance,
            ],
            SchemaForm::Ref { .. } => vec![
                SemanticMutationName::InvalidReference,
                SemanticMutationName::NonExistentReference,
            ],
            SchemaForm::Empty { .. } => vec![
                SemanticMutationName::NullForEmpty,
            ],
        }
    }
}
