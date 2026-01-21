# Manuel Utilisateur - JTD Validator Tests

## Table des matières

1. [Introduction](#introduction)
2. [Installation](#installation)
3. [Vue d'ensemble des fonctionnalités](#vue-densemble-des-fonctionnalités)
4. [Commandes principales](#commandes-principales)
   - [Analyse de la RFC](#analyse-de-la-rfc)
   - [Exécution des tests](#exécution-des-tests)
   - [Validation](#validation)
   - [Génération de JSON valides](#génération-de-json-valides)
   - [Fuzzer](#fuzzer)
5. [Formats de schémas supportés](#formats-de-schémas-supportés)
6. [Exemples d'utilisation](#exemples-dutilisation)
7. [Guide de référence rapide](#guide-de-référence-rapide)

---

## Introduction

**JTD Validator Tests** est un outil complet en Rust pour tester et valider des implémentations de la **RFC 8927 (JSON Type Definition, JTD)** et de **JSON Schema 2020-12 (draft-2020-12)**. Il permet de :

- ✅ Valider des instances JSON contre des schémas JTD ou JSON Schema 2020-12
- ✅ Générer des JSON valides selon un schéma
- ✅ Générer des JSON invalides (syntaxiquement ou sémantiquement) pour tester la robustesse
- ✅ Exécuter une suite complète de tests de conformité
- ✅ Analyser la RFC 8927 pour identifier tous les tests requis

---

## Installation

### Prérequis

- **Rust** : Version 1.70 ou supérieure
- **Cargo** : Gestionnaire de paquets Rust (inclus avec Rust)

### Installation depuis les sources

```bash
# Cloner le dépôt
git clone https://github.com/jblestang/AIJSONSchemaFuzzer.git
cd AIJSONSchemaFuzzer

# Compiler le projet
cargo build --release

# L'exécutable sera disponible dans target/release/jtd-validator-tests
```

### Installation rapide (développement)

```bash
# Compiler en mode debug (plus rapide, moins optimisé)
cargo build

# L'exécutable sera disponible dans target/debug/jtd-validator-tests
```

---

## Vue d'ensemble des fonctionnalités

### 1. Validation de schémas et instances

L'outil peut valider des instances JSON contre des schémas JTD ou JSON Schema 2020-12 avec **détection automatique du format**. Il supporte :

- **JTD (RFC 8927)** : 8 formes de schéma (empty, type, enum, elements, values, properties, discriminator, ref)
- **JSON Schema 2020-12** : Mots-clés principaux (prefixItems, items, patternProperties, allOf/anyOf/oneOf/not, if/then/else, const, required, contraintes)

### 2. Génération de JSON

- **Génération de JSON valides** : Crée des instances JSON conformes à un schéma
- **Fuzzer sémantique** : Génère des JSON valides syntaxiquement mais invalides sémantiquement (61 mutations disponibles)
- **Fuzzer syntaxique** : Génère des JSON mal formés (9 types de mutations)

### 3. Suite de tests

- **103 tests de conformité** : 52 tests JTD + 51 tests JSON Schema 2020-12
- **Taux de réussite** : 100% (tous les tests passent)

### 4. Analyse de la RFC

- Affiche tous les tests requis par la RFC 8927
- Identifie les aspects à tester pour une implémentation complète

---

## Commandes principales

### Analyse de la RFC

Affiche une analyse complète de la RFC 8927 et liste tous les tests à implémenter.

```bash
cargo run -- analyze-rfc
```

**Sortie** : Liste détaillée de tous les aspects de la RFC 8927 à tester, organisés par forme de schéma.

**Exemple de sortie** :
```
=== Analyse de la RFC 8927 ===

Formes de schéma à tester :
- empty
- type (11 types primitifs)
- enum
- elements
- values
- properties
- discriminator
- ref

...
```

---

### Exécution des tests

Exécute tous les tests de conformité (JTD et JSON Schema 2020-12).

```bash
cargo run -- run-tests
```

**Sortie** : Résultats détaillés de tous les tests avec statistiques.

**Exemple de sortie** :
```
=== Exécution des tests de conformité RFC 8927 (JTD) ===
✓ empty schema accepts any value
✓ empty schema with nullable accepts null
...

=== Exécution des tests de conformité JSON Schema 2020-12 ===
✓ prefixItems validates tuple correctly
✓ prefixItems rejects wrong type at position
...

=== Résultats ===
JTD - Passés: 52, Échoués: 0, Total: 52
JSON Schema 2020-12 - Passés: 51, Échoués: 0, Total: 51
Total global - Passés: 103, Échoués: 0, Total: 103
```

---

### Validation

Valide une instance JSON contre un schéma (JTD ou JSON Schema 2020-12).

#### Syntaxe

```bash
cargo run -- validate <schema.json> <instance.json>
```

#### Description

- **Détection automatique** : Le format du schéma (JTD ou JSON Schema 2020-12) est détecté automatiquement
- **Validation complète** : Vérifie tous les aspects du schéma (types, contraintes, propriétés, etc.)
- **Rapport d'erreurs** : Affiche toutes les erreurs de validation avec les chemins JSON Pointer (`instancePath` et `schemaPath`)

#### Exemples

**Exemple 1 : Validation JTD réussie**

```bash
# Schéma JTD (schema.json)
{
  "properties": {
    "name": {"type": "string"},
    "age": {"type": "uint8"}
  }
}

# Instance valide (instance.json)
{
  "name": "Alice",
  "age": 30
}

# Commande
cargo run -- validate schema.json instance.json

# Sortie : Aucune erreur (validation réussie)
```

**Exemple 2 : Validation avec erreurs**

```bash
# Instance invalide (instance.json)
{
  "name": "Alice",
  "age": 300  # Erreur : 300 > 255 pour uint8
}

# Commande
cargo run -- validate schema.json instance.json

# Sortie :
# Erreur de validation :
#   - instancePath: /age, schemaPath: /properties/age/type
```

**Exemple 3 : Validation JSON Schema 2020-12**

```bash
# Schéma JSON Schema 2020-12 (schema.json)
{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "type": "object",
  "properties": {
    "name": {"type": "string", "minLength": 3},
    "age": {"type": "integer", "minimum": 0, "maximum": 150}
  },
  "required": ["name", "age"]
}

# Instance valide
{
  "name": "Alice",
  "age": 30
}

# Commande
cargo run -- validate schema.json instance.json

# Sortie : Validation réussie
```

---

### Génération de JSON valides

Génère une instance JSON valide selon un schéma JTD.

#### Syntaxe

```bash
cargo run -- generate <schema.json> [--output <file.json>]
```

#### Options

- `schema.json` : Chemin vers le fichier schéma (JTD ou JSON Schema 2020-12)
- `--output <file.json>` : (Optionnel) Fichier de sortie. Si omis, affiche sur stdout

#### Description

- Génère des valeurs aléatoires conformes au schéma
- Supporte toutes les formes JTD (type, enum, elements, values, properties, discriminator, ref)
- Pour JSON Schema 2020-12, génère des valeurs basiques selon le type

#### Exemples

**Exemple 1 : Génération basique**

```bash
# Schéma (schema.json)
{
  "properties": {
    "name": {"type": "string"},
    "age": {"type": "uint8"}
  }
}

# Commande
cargo run -- generate schema.json

# Sortie (exemple) :
{
  "name": "valid",
  "age": 42
}
```

**Exemple 2 : Sauvegarde dans un fichier**

```bash
cargo run -- generate schema.json --output output.json

# Le JSON généré est sauvegardé dans output.json
```

**Exemple 3 : Génération pour un tableau**

```bash
# Schéma (schema.json)
{
  "elements": {
    "type": "string"
  }
}

# Commande
cargo run -- generate schema.json

# Sortie (exemple) :
["valid", "valid", "valid"]
```

---

### Fuzzer

Génère des JSON invalides (syntaxiquement ou sémantiquement) pour tester la robustesse d'un validateur.

#### Syntaxe

```bash
cargo run -- fuzz <schema.json> [--syntax|--semantic] [--mutation <mutation_name>] [--count N] [--output DIR] [--list-mutations]
```

#### Options

- `schema.json` : Chemin vers le fichier schéma (JTD ou JSON Schema 2020-12)
- `--syntax` : Génère des JSON syntaxiquement invalides (mal formés) - **JTD uniquement**
- `--semantic` : Génère des JSON sémantiquement invalides (valides syntaxiquement mais violant le schéma) - **Défaut**
- `--mutation <mutation_name>` : Spécifie une mutation particulière (voir `--list-mutations`)
- `--count N` : Nombre de cas à générer (défaut: 5)
- `--output DIR` : Répertoire de sortie (défaut: `fuzz_output/`)
- `--list-mutations` : Affiche la liste de toutes les mutations disponibles

#### Description

Le fuzzer génère des JSON invalides pour tester la robustesse d'un validateur. Il supporte :

- **9 mutations syntaxiques** (JTD uniquement) : JSON mal formés
- **29 mutations sémantiques JTD** : Violations de schéma JTD
- **23 mutations sémantiques JSON Schema 2020-12** : Violations de schéma JSON Schema 2020-12

**Total : 61 mutations**

#### Lister les mutations disponibles

```bash
cargo run -- fuzz schema.json --list-mutations
```

**Sortie** :
```
=== Mutations syntaxiques (JTD uniquement) ===
  missing-closing-brace - Accolade fermante manquante
  missing-opening-brace - Accolade ouvrant manquante
  invalid-character - Caractère invalide ajouté
  ...

=== Mutations sémantiques (JTD) ===
  wrong-type - Type incorrect
  out-of-range - Valeur hors plage
  not-in-enum - Valeur non dans l'enum
  ...

=== Mutations sémantiques (JSON Schema 2020-12) ===
  prefix-items-wrong-type - Mauvais type à une position dans prefixItems
  prefix-items-extra - Trop d'éléments quand items: false
  min-items-violation - Tableau trop court
  ...
```

#### Exemples

**Exemple 1 : Génération aléatoire (défaut)**

```bash
# Génère 5 JSON sémantiquement invalides avec mutations aléatoires
cargo run -- fuzz schema.json --semantic --count 5
```

**Exemple 2 : Mutation spécifique**

```bash
# Génère un JSON avec une mutation spécifique
cargo run -- fuzz schema.json --semantic --mutation wrong-type --count 1
```

**Exemple 3 : Mutations syntaxiques**

```bash
# Génère 10 JSON syntaxiquement invalides
cargo run -- fuzz schema.json --syntax --count 10
```

**Exemple 4 : Sauvegarde dans un répertoire**

```bash
# Génère 20 cas et les sauvegarde dans ./test_cases/
cargo run -- fuzz schema.json --semantic --count 20 --output ./test_cases/
```

**Exemple 5 : Mutation JSON Schema 2020-12**

```bash
# Schéma JSON Schema 2020-12
{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "prefixItems": [{"type": "string"}, {"type": "number"}],
  "items": false
}

# Génère une mutation spécifique pour prefixItems
cargo run -- fuzz schema.json --semantic --mutation prefix-items-extra --count 1
```

---

## Formats de schémas supportés

### JTD (RFC 8927)

Le format JTD est détecté automatiquement si le schéma ne contient pas de mots-clés JSON Schema 2020-12.

**Formes supportées** :
- `empty` : Schéma vide acceptant toutes les valeurs
- `type` : Types primitifs (boolean, string, timestamp, float32, float64, int8, uint8, int16, uint16, int32, uint32)
- `enum` : Énumération de chaînes
- `elements` : Tableaux avec éléments uniformes
- `values` : Objets avec valeurs uniformes
- `properties` : Objets avec propriétés nommées
- `discriminator` : Objets avec tag discriminant
- `ref` : Références vers definitions

**Exemple de schéma JTD** :
```json
{
  "properties": {
    "name": {"type": "string"},
    "age": {"type": "uint8"}
  },
  "optionalProperties": {
    "email": {"type": "string"}
  },
  "additionalProperties": false
}
```

### JSON Schema 2020-12

Le format JSON Schema 2020-12 est détecté automatiquement si :
- Le schéma contient `"$schema": "https://json-schema.org/draft/2020-12/schema"`
- Le schéma contient des mots-clés spécifiques JSON Schema 2020-12 (`prefixItems`, `$ref`, `allOf`, etc.)

**Mots-clés supportés** :
- `prefixItems` : Validation de tuples
- `items` : Validation des éléments restants après prefixItems
- `patternProperties` : Propriétés avec patterns regex
- `$ref` : Références vers `$defs`
- `allOf`, `anyOf`, `oneOf`, `not` : Logique combinatoire
- `if`, `then`, `else` : Validation conditionnelle
- `const` : Valeurs constantes
- `required` : Propriétés requises
- Contraintes sur tableaux : `minItems`, `maxItems`, `uniqueItems`, `contains`
- Contraintes sur objets : `minProperties`, `maxProperties`
- Contraintes sur chaînes : `minLength`, `maxLength`, `pattern`
- Contraintes sur nombres : `minimum`, `maximum`, `exclusiveMinimum`, `exclusiveMaximum`, `multipleOf`

**Exemple de schéma JSON Schema 2020-12** :
```json
{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "type": "object",
  "properties": {
    "name": {"type": "string", "minLength": 3},
    "age": {"type": "integer", "minimum": 0, "maximum": 150}
  },
  "required": ["name", "age"]
}
```

---

## Exemples d'utilisation

### Scénario 1 : Tester un validateur JTD

```bash
# 1. Créer un schéma de test
cat > test_schema.json << EOF
{
  "properties": {
    "name": {"type": "string"},
    "age": {"type": "uint8"}
  }
}
EOF

# 2. Générer des cas de test valides
cargo run -- generate test_schema.json --output valid_cases/

# 3. Générer des cas de test invalides
cargo run -- fuzz test_schema.json --semantic --count 20 --output invalid_cases/

# 4. Valider une instance
cargo run -- validate test_schema.json instance.json
```

### Scénario 2 : Tester un validateur JSON Schema 2020-12

```bash
# 1. Créer un schéma JSON Schema 2020-12
cat > test_schema.json << EOF
{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "prefixItems": [{"type": "string"}, {"type": "number"}],
  "items": false,
  "minItems": 2,
  "maxItems": 2
}
EOF

# 2. Générer des mutations spécifiques
cargo run -- fuzz test_schema.json --semantic --mutation prefix-items-wrong-type --count 5
cargo run -- fuzz test_schema.json --semantic --mutation prefix-items-extra --count 5
cargo run -- fuzz test_schema.json --semantic --mutation min-items-violation --count 5
```

### Scénario 3 : Suite de tests complète

```bash
# Exécuter tous les tests de conformité
cargo run -- run-tests

# Analyser la RFC pour identifier les tests requis
cargo run -- analyze-rfc
```

### Scénario 4 : Validation en pipeline

```bash
# Valider plusieurs instances
for instance in instances/*.json; do
  cargo run -- validate schema.json "$instance"
done
```

---

## Guide de référence rapide

### Commandes principales

| Commande | Description |
|----------|-------------|
| `cargo run -- analyze-rfc` | Affiche l'analyse de la RFC 8927 |
| `cargo run -- run-tests` | Exécute tous les tests de conformité |
| `cargo run -- validate <schema> <instance>` | Valide une instance contre un schéma |
| `cargo run -- generate <schema> [--output <file>]` | Génère un JSON valide |
| `cargo run -- fuzz <schema> [options]` | Génère des JSON invalides |

### Options du fuzzer

| Option | Description |
|--------|-------------|
| `--syntax` | Mutations syntaxiques (JSON mal formés) |
| `--semantic` | Mutations sémantiques (violations de schéma) - **Défaut** |
| `--mutation <name>` | Mutation spécifique |
| `--count N` | Nombre de cas à générer (défaut: 5) |
| `--output DIR` | Répertoire de sortie (défaut: `fuzz_output/`) |
| `--list-mutations` | Liste toutes les mutations disponibles |

### Mutations populaires

#### Mutations JTD

- `wrong-type` : Type incorrect
- `out-of-range` : Valeur hors plage
- `not-in-enum` : Valeur non dans l'enum
- `one-required-missing` : Propriété requise manquante
- `additional-properties` : Propriétés supplémentaires non autorisées

#### Mutations JSON Schema 2020-12

- `prefix-items-wrong-type` : Mauvais type dans prefixItems
- `prefix-items-extra` : Éléments supplémentaires interdits
- `min-items-violation` : Tableau trop court
- `max-items-violation` : Tableau trop long
- `pattern-violation` : Chaîne ne correspondant pas au pattern

### Formats de sortie

- **Validation** : Erreurs affichées sur stderr avec chemins JSON Pointer
- **Génération** : JSON affiché sur stdout (ou fichier si `--output` spécifié)
- **Fuzzer** : JSON affichés sur stdout (ou fichiers dans le répertoire de sortie)

### Codes de sortie

- **0** : Succès
- **1** : Erreur (validation échouée, fichier introuvable, etc.)

---

## Dépannage

### Problème : "Schema format could not be detected"

**Solution** : Vérifiez que le schéma est valide JSON et contient soit :
- Des mots-clés JTD (type, enum, elements, etc.)
- Des mots-clés JSON Schema 2020-12 (`$schema`, `prefixItems`, etc.)

### Problème : "File not found"

**Solution** : Vérifiez les chemins des fichiers. Utilisez des chemins absolus ou relatifs corrects.

### Problème : "Syntax mutations not yet implemented for JSON Schema 2020-12"

**Solution** : Les mutations syntaxiques sont disponibles uniquement pour JTD. Utilisez `--semantic` pour JSON Schema 2020-12.

### Problème : "Mutation not found"

**Solution** : Utilisez `--list-mutations` pour voir toutes les mutations disponibles. Vérifiez que vous utilisez le bon format de schéma (JTD vs JSON Schema 2020-12).

---

## Documentation complémentaire

Pour plus de détails, consultez :

- **[doc/TEST_PLAN.md](TEST_PLAN.md)** : Plan de test logiciel détaillé
- **[doc/TEST_RESULTS.md](TEST_RESULTS.md)** : Résultats des tests exécutés
- **[doc/MUTATIONS.md](MUTATIONS.md)** : Documentation complète des mutations (61 mutations)
- **[doc/RFC_COVERAGE.md](RFC_COVERAGE.md)** : Couverture complète de la RFC 8927
- **[doc/JSON_SCHEMA_2020_12_UPGRADE.md](JSON_SCHEMA_2020_12_UPGRADE.md)** : Documentation de l'upgrade vers JSON Schema 2020-12

---

## Support

Pour signaler un bug ou proposer une fonctionnalité, ouvrez une issue sur le dépôt GitHub.

---

**Version** : 0.1.0  
**Dernière mise à jour** : 2024
