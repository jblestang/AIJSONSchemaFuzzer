# Extension RegExp pour la validation des chaînes

## Contexte

La **RFC 8927 (JSON Type Definition)** ne définit **pas** de support natif pour les expressions régulières (RegExp) pour valider les chaînes. C'est une spécification minimaliste qui se limite à :
- `"type": "string"` : Vérifie uniquement que la valeur est une chaîne
- `"enum"` : Liste de valeurs autorisées

## Proposition : Extension via Metadata

Pour rester conforme à la RFC 8927 tout en ajoutant cette fonctionnalité utile, nous proposons d'utiliser le champ **`metadata`** qui est défini dans la RFC mais ignoré lors de la validation standard.

### Avantages

1. ✅ **Conformité RFC 8927** : Le champ `metadata` est officiellement supporté et ignoré par les validateurs standards
2. ✅ **Rétrocompatibilité** : Les schémas sans `metadata.pattern` fonctionnent comme avant
3. ✅ **Interopérabilité** : Les outils qui ne comprennent pas cette extension ignorent simplement `metadata`
4. ✅ **Utilité pratique** : Permet de valider des formats courants (email, téléphone, UUID, etc.)

### Format proposé

```json
{
  "type": "string",
  "metadata": {
    "pattern": "^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}$",
    "patternFlags": "i"  // Optionnel : flags (i, m, s, etc.)
  }
}
```

### Exemples d'utilisation

#### Email
```json
{
  "type": "string",
  "metadata": {
    "pattern": "^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}$"
  }
}
```

#### UUID
```json
{
  "type": "string",
  "metadata": {
    "pattern": "^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$",
    "patternFlags": "i"
  }
}
```

#### Téléphone (format français)
```json
{
  "type": "string",
  "metadata": {
    "pattern": "^0[1-9]([ .-]?[0-9]{2}){4}$"
  }
}
```

## Implémentation proposée

### 1. Parser : Extraire le pattern depuis metadata

```rust
// Dans src/schema/ast.rs ou nouveau fichier
pub fn extract_pattern_from_metadata(schema: &JtdSchema) -> Option<Regex> {
    if let Some(metadata) = &schema.metadata {
        if let Some(pattern) = metadata.get("pattern").and_then(|v| v.as_str()) {
            let flags = metadata.get("patternFlags")
                .and_then(|v| v.as_str())
                .unwrap_or("");
            // Compiler le regex avec les flags
            // ...
        }
    }
    None
}
```

### 2. Validateur : Valider le pattern si présent

```rust
// Dans src/validator/validate.rs
fn validate_type(...) {
    match type_name {
        TypeName::String => {
            if !instance.is_string() {
                return Err(...);
            }
            
            // Extension : Valider le pattern si présent dans metadata
            if let Some(pattern) = extract_pattern_from_metadata(schema) {
                if let Some(s) = instance.as_str() {
                    if !pattern.is_match(s) {
                        return Err(vec![ValidationError {
                            instance_path: instance_path.to_string(),
                            schema_path: format!("{}/metadata/pattern", schema_path),
                        }]);
                    }
                }
            }
            
            Ok(())
        }
        // ...
    }
}
```

### 3. Fuzzer : Générer des chaînes qui violent le pattern

```rust
// Dans src/fuzzer/mutations.rs
// Nouvelle mutation : "pattern-violation"
// Génère une chaîne qui ne correspond pas au pattern
```

## Standard recommandé : I-Regexp (RFC 9485)

Pour maximiser la portabilité et la sécurité, nous recommandons d'utiliser **I-Regexp (RFC 9485)** qui définit un sous-ensemble portable et sûr des expressions régulières.

### Avantages d'I-Regexp
- ✅ Sous-ensemble portable entre langages
- ✅ Évite les problèmes de sécurité (ReDoS)
- ✅ Standardisé par l'IETF

## Alternatives

### Alternative 1 : Extension non-standard directe
Ajouter un champ `pattern` directement dans le schéma (non conforme RFC 8927) :

```json
{
  "type": "string",
  "pattern": "^[a-z]+$"  // Extension non-standard
}
```

**Inconvénient** : Violation de la RFC 8927, pas d'interopérabilité.

### Alternative 2 : Utiliser enum pour valeurs limitées
Pour des cas simples, utiliser `enum` :

```json
{
  "enum": ["option1", "option2", "option3"]
}
```

**Limitation** : Ne fonctionne que pour des listes finies.

## Recommandation

✅ **Implémenter via metadata** : C'est la meilleure approche car :
- Reste conforme à la RFC 8927
- Permet l'interopérabilité
- Ajoute une fonctionnalité utile sans casser l'existant
- Peut être documenté comme extension optionnelle

## Prochaines étapes

1. ✅ Ajouter le support de `metadata.pattern` dans le validateur
2. ✅ Ajouter des tests pour la validation de pattern
3. ✅ Ajouter des mutations dans le fuzzer pour générer des chaînes invalides
4. ✅ Documenter dans le README et MUTATIONS.md
5. ⚠️ Optionnel : Implémenter I-Regexp pour plus de sécurité

## Exemple de schéma complet

```json
{
  "properties": {
    "email": {
      "type": "string",
      "metadata": {
        "pattern": "^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}$"
      }
    },
    "phone": {
      "type": "string",
      "metadata": {
        "pattern": "^0[1-9]([ .-]?[0-9]{2}){4}$"
      }
    },
    "name": {
      "type": "string"
    }
  }
}
```
