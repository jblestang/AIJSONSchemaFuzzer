# Mutations - Documentation Complète

Ce document décrit **toutes les mutations** implémentées dans le fuzzer pour générer des JSON invalides (syntaxiquement ou sémantiquement).

## Vue d'ensemble

Le fuzzer génère des JSON invalides de deux types :
- **Mutations syntaxiques** : JSON mal formé (9 types) - Support JTD uniquement
- **Mutations sémantiques** : JSON valide syntaxiquement mais qui viole le schéma
  - **JTD (RFC 8927)** : 29 types de mutations selon le schéma
  - **JSON Schema 2020-12** : 20+ types de mutations selon le schéma

## Utilisation

```bash
# Générer 10 JSON sémantiquement invalides (mutations aléatoires)
cargo run -- fuzz schema.json --semantic --count 10 --output ./fuzz_output

# Générer un JSON avec une mutation spécifique
cargo run -- fuzz schema.json --semantic --mutation one-required-missing --count 1

# Lister toutes les mutations disponibles
cargo run -- fuzz schema.json --list-mutations

# Générer des JSON syntaxiquement invalides avec une mutation spécifique
cargo run -- fuzz schema.json --syntax --mutation missing-closing-brace --count 5
```

## Mutations syntaxiques

**9 types de mutations** pour générer des JSON mal formés :

- **missing-closing-brace** : Accolade fermante manquante (`}` ou `]`)
- **missing-opening-brace** : Accolade ouvrant manquante (`{` ou `[`)
- **invalid-character** : Caractère invalide ajouté à la fin
- **comma-to-semicolon** : Virgule remplacée par point-virgule (`;` au lieu de `,`)
- **remove-quotes** : Guillemets supprimés des clés (JSON invalide)
- **trailing-comma** : Virgule trailing (virgule après le dernier élément)
- **colon-to-equals** : `:` remplacé par `=` (ex: `{"key"= "value"}`)
- **truncated-json** : JSON tronqué (moitié du JSON supprimée)
- **mixed-indentation** : Mélange tabulations et espaces dans l'indentation (peut rendre le JSON invalide selon certains parsers stricts)

## Mutations sémantiques par type de schéma

### 1. Forme "Type"

**3 types de mutations** pour les types primitifs (boolean, string, timestamp, float32, float64, int8, uint8, int16, uint16, int32, uint32) :

- **wrong-type** : Type incorrect - Génère un type différent (ex: string au lieu de number, bool au lieu de string)
- **out-of-range** : Valeur hors plage - Génère une valeur en dehors des limites autorisées
  - int8: > 127 ou < -128
  - uint8: < 0 ou > 255
  - int16: hors plage -32768..32767
  - uint16: < 0 ou > 65535
  - int32: hors plage -2147483648..2147483647
  - uint32: < 0 ou > 4294967295
  - float32: valeur très grande (1e50)
- **null-for-non-nullable** : Null si nullable=false - Génère null pour un type non-nullable

### 2. Forme "Enum"

**3 types de mutations** :

- **not-in-enum** : Valeur non dans l'enum - Génère une chaîne aléatoire qui n'est pas dans la liste
- **similar-but-different** : Chaîne similaire mais différente - Génère une variation d'une valeur de l'enum (ex: ajout de suffixe)
- **empty-string** : Chaîne vide - Génère une chaîne vide si l'enum ne contient pas de chaîne vide

### 3. Forme "Elements" (tableaux)

**6 types de mutations différentes** :

- **not-an-array** : Type incorrect (pas un tableau) - génère un objet, une chaîne ou autre type
- **single-invalid-element** : Un seul élément invalide - génère un tableau avec un élément qui viole le schéma
- **mixed-types** : **Types mixtes** - mélange d'éléments valides et invalides dans le même tableau
  - Exemple: `["valid", invalid_value, "valid", invalid_value, ...]`
- **all-invalid-elements** : Tous les éléments invalides - génère un tableau où tous les éléments violent le schéma
- **completely-different-types** : **Types complètement différents** - génère un tableau avec un mélange de types JSON (string, number, bool, object, array, null)
  - Exemple: `["string", 42, true, {}, [], null]` pour un schéma qui attend uniquement des strings
- **empty-array** : Tableau vide (peut être valide selon le schéma, mais testé)

#### Exemples de mutations pour types mixtes

```json
// Schéma: {"elements": {"type": "string"}}
// Mutation: mixed-types - Mélange valide/invalide
["valid_string", 42, "another_valid", true, "valid"]

// Schéma: {"elements": {"type": "uint8"}}
// Mutation: completely-different-types - Types complètement différents
["string", 42, true, {}, [], null]

// Schéma: {"elements": {"properties": {"name": {"type": "string"}}}}
// Mutation: mixed-types - Mélange d'objets valides et invalides
[{"name": "valid"}, "not_an_object", {"name": "valid2"}, 42]
```

### 4. Forme "Values" (objets avec valeurs uniformes)

**3 types de mutations** :

- **not-an-object** : Type incorrect (pas un objet) - Génère un tableau, null ou autre type au lieu d'un objet
- **single-invalid-value** : Une clé avec valeur invalide - Génère un objet avec une seule clé ayant une valeur invalide
- **multiple-invalid-values** : Plusieurs clés avec valeurs invalides - Génère un objet avec 2-4 clés ayant des valeurs invalides

### 5. Forme "Properties" (objets avec propriétés nommées)

**11 types de mutations différentes** :

#### Mutations de structure
- **not-an-object-prop** : Type incorrect (pas un objet) - génère un tableau ou autre type
- **empty-object** : Objet vide - toutes les propriétés requises manquantes
- **null-object** : Objet null (si nullable=false) ou objet avec propriétés partielles

#### Mutations de propriétés manquantes
- **all-required-missing** : Toutes les propriétés requises manquantes (seulement propriétés optionnelles)
- **one-required-missing** : Une propriété requise manquante (choisie aléatoirement)
- **missing-plus-additional** : Propriété requise manquante + propriété supplémentaire

#### Mutations de propriétés supplémentaires
- **additional-properties** : Propriétés supplémentaires non autorisées (si `additionalProperties=false`)
  - Génère 1-3 propriétés supplémentaires avec des noms comme `extra_property_0`, `extra_property_1`, etc.

#### Mutations de valeurs invalides
- **single-invalid-property** : Valeur invalide pour une seule propriété requise
- **all-invalid-properties** : Toutes les valeurs de propriétés requises invalides
- **invalid-optional-property** : Valeur invalide pour une propriété optionnelle
- **null-for-non-nullable-prop** : Propriété requise avec valeur null (si `nullable=false`)

### 6. Forme "Discriminator"

**5 types de mutations** :

- **not-an-object-disc** : Type incorrect (pas un objet) - Génère un tableau ou autre type
- **missing-tag** : Tag manquant - Génère un objet sans la clé discriminator (mais avec les propriétés du mapping)
- **invalid-tag** : Tag invalide - Génère un tag qui n'existe pas dans le mapping (ex: "invalid_tag")
- **tag-not-string** : Tag non-string - Génère un tag avec un type incorrect (ex: number au lieu de string)
- **invalid-instance** : Instance invalide - Génère une instance qui viole le schéma du mapping correspondant

### 7. Forme "Ref" (références)

- **invalid-reference** : Référence invalide - Génère une valeur qui viole le schéma référencé
- **non-existent-reference** : Référence inexistante - Si la référence n'existe pas dans definitions

### 8. Forme "Empty"

- **null-for-empty** : Null si nullable=false - Génère null si nullable est explicitement false

## Mutations pour JSON Schema 2020-12

Le fuzzer supporte également les mutations sémantiques pour JSON Schema 2020-12. Les mutations sont automatiquement détectées selon le format du schéma.

### prefixItems (tuples)

**6 types de mutations** :

- **prefix-items-wrong-type** : Mauvais type à une position spécifique dans le tuple
- **prefix-items-extra** : Trop d'éléments quand `items: false` (éléments supplémentaires interdits)
- **prefix-items-too-few** : Pas assez d'éléments (moins que le nombre requis par prefixItems)
- **prefix-items-invalid-items** : Élément supplémentaire invalide selon le schéma `items` (après prefixItems)
- **prefix-items-min-items-violation** : Tableau avec prefixItems valides mais total < `minItems` (combinaison prefixItems + minItems)
- **prefix-items-max-items-violation** : Tableau avec prefixItems + items qui dépasse `maxItems` (combinaison prefixItems + maxItems)

### patternProperties

**1 type de mutation** :

- **pattern-properties-invalid-value** : Clé qui match le pattern regex mais valeur invalide selon le schéma du pattern

### allOf/anyOf/oneOf/not

**4 types de mutations** :

- **all-of-invalid** : Valeur qui viole un des sous-schémas de `allOf`
- **any-of-all-invalid** : Valeur qui viole tous les sous-schémas de `anyOf`
- **one-of-multiple-valid** : Valeur qui satisfait plusieurs sous-schémas de `oneOf` (devrait être invalide)
- **not-satisfied** : Valeur qui satisfait le schéma `not` (devrait être invalide)

### if/then/else

**2 types de mutations** :

- **if-then-invalid** : Condition `if` vraie mais `then` invalide
- **if-else-invalid** : Condition `if` fausse mais `else` invalide

### const

**1 type de mutation** :

- **const-different** : Valeur différente de la constante requise

### required

**1 type de mutation** :

- **missing-required** : Objet sans une propriété requise

### Contraintes sur tableaux

**2 types de mutations** :

- **min-items-violation** : Tableau trop court (moins que `minItems`) - génère des valeurs valides selon `items` si présent
- **max-items-violation** : Tableau trop long (plus que `maxItems`) - génère des valeurs valides selon `items` si présent

**Note** : Ces mutations fonctionnent avec ou sans `prefixItems`. Si `prefixItems` est présent, utilisez les mutations spécifiques `prefix-items-min-items-violation` et `prefix-items-max-items-violation`.

### Contraintes sur chaînes

**3 types de mutations** :

- **min-length-violation** : Chaîne trop courte (moins que `minLength`)
- **max-length-violation** : Chaîne trop longue (plus que `maxLength`)
- **pattern-violation** : Chaîne qui ne match pas le pattern regex

### Contraintes sur nombres

**3 types de mutations** :

- **minimum-violation** : Nombre trop petit (moins que `minimum`)
- **maximum-violation** : Nombre trop grand (plus que `maximum`)
- **multiple-of-violation** : Nombre qui n'est pas un multiple de `multipleOf`
## Exemples de mutations

### Suppression de clés requises

```json
// Schéma: {"properties": {"name": {"type": "string"}, "age": {"type": "uint8"}}}
// Mutation: one-required-missing
{
  "name": "Alice"
  // "age" manquant
}
```

### Ajout de clés inattendues

```json
// Schéma: {"properties": {"name": {"type": "string"}}, "additionalProperties": false}
// Mutation: additional-properties
{
  "name": "Alice",
  "unexpected_key": "invalid",
  "extra_property_0": "invalid"
}
```

### Valeurs invalides

```json
// Schéma: {"properties": {"age": {"type": "uint8"}}}
// Mutation: wrong-type (via single-invalid-property)
{
  "age": "not_an_integer"  // string au lieu de number
}

// Mutation: out-of-range
{
  "age": 300  // > 255 pour uint8
}

// Mutation: null-for-non-nullable-prop
{
  "age": null  // si nullable n'est pas true
}
```

## Récapitulatif des mutations

| Type de schéma | Mutations syntaxiques | Mutations sémantiques | Total |
|----------------|----------------------|---------------------|-------|
| **Syntaxiques** | 9 | - | 9 |
| **Type** | - | 3 | 3 |
| **Enum** | - | 3 | 3 |
| **Elements** | - | 6 | 6 |
| **Values** | - | 3 | 3 |
| **Properties** | - | 11 | 11 |
| **Discriminator** | - | 5 | 5 |
| **Ref** | - | 2 | 2 |
| **Empty** | - | 1 | 1 |
| **TOTAL JTD** | **9** | **29** | **38** |
| **JSON Schema 2020-12** | - | **23** | **23** |
|   - prefixItems | - | 6 | 6 |
|   - patternProperties | - | 1 | 1 |
|   - allOf/anyOf/oneOf/not | - | 4 | 4 |
|   - if/then/else | - | 2 | 2 |
|   - const | - | 1 | 1 |
|   - required | - | 1 | 1 |
|   - Contraintes tableaux | - | 2 | 2 |
|   - Contraintes chaînes | - | 3 | 3 |
|   - Contraintes nombres | - | 3 | 3 |
| **TOTAL GLOBAL** | **9** | **52** | **61** |

## Couverture des tests

Ces mutations permettent de tester :
- ✅ Détection de propriétés manquantes
- ✅ Détection de propriétés supplémentaires
- ✅ Validation des types
- ✅ Validation des plages de valeurs
- ✅ Validation des enums
- ✅ Validation des structures (tableaux, objets)
- ✅ Validation des discriminators
- ✅ Gestion de nullable
- ✅ Validation des références
- ✅ Types mixtes dans les tableaux
- ✅ Validation syntaxique du JSON
- ✅ Valeurs similaires mais invalides (enum)
- ✅ Plusieurs violations simultanées
- ✅ Validation des tuples (prefixItems)
- ✅ Validation des patterns regex (patternProperties, pattern)
- ✅ Validation de la logique combinatoire (allOf/anyOf/oneOf/not)
- ✅ Validation conditionnelle (if/then/else)
- ✅ Validation des constantes (const)
- ✅ Validation des contraintes (minItems, maxItems, minLength, maxLength, minimum, maximum, multipleOf)