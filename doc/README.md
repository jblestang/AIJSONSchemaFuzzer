# JTD Validator Tests - Outil de test pour la RFC 8927

Cet outil permet de tester qu'une implémentation de la RFC 8927 (JSON Type Definition, JTD) est correcte vis-à-vis de la spécification.

## Fonctionnalités

- **Parser de schéma JTD** : Parse et valide la syntaxe des schémas JTD
- **Validateur** : Valide des instances JSON contre des schémas JTD
- **Suite de tests complète** : Couvre tous les aspects de la RFC 8927
- **Analyse de la RFC** : Affiche tous les tests requis par la spécification
- **Fuzzer** : Génère des JSON invalides (syntaxiquement ou sémantiquement) à partir d'un schéma
- **Générateur de JSON valides** : Génère des instances JSON valides selon un schéma

## Installation

```bash
cargo build --release
```

## Utilisation

### Afficher l'analyse de la RFC

```bash
cargo run -- analyze-rfc
```

Cette commande affiche une analyse complète de la RFC 8927 et liste tous les tests à implémenter pour vérifier la conformité.

### Exécuter tous les tests

```bash
cargo run -- run-tests
```

Exécute tous les cas de test de conformité et affiche les résultats.

### Valider une instance contre un schéma

```bash
cargo run -- validate schema.json instance.json
```

Valide une instance JSON contre un schéma JTD.

### Générer un JSON valide

```bash
cargo run -- generate schema.json [output.json]
```

Génère un JSON valide selon le schéma. Si `output.json` n'est pas spécifié, affiche le JSON sur la sortie standard.

### Fuzzer : Générer des JSON invalides

```bash
# Générer des JSON syntaxiquement invalides (mal formés)
cargo run -- fuzz schema.json --syntax --count 10

# Générer des JSON sémantiquement invalides (violations du schéma)
cargo run -- fuzz schema.json --semantic --count 10

# Générer avec une mutation spécifique
cargo run -- fuzz schema.json --semantic --mutation one-required-missing --count 1

# Lister toutes les mutations disponibles
cargo run -- fuzz schema.json --list-mutations

# Sauvegarder dans un répertoire
cargo run -- fuzz schema.json --semantic --count 10 --output ./fuzz_output
```

Le fuzzer génère des JSON invalides pour tester la robustesse des validateurs :
- **Altérations syntaxiques** : JSON mal formé (accolades manquantes, caractères invalides, etc.)
- **Altérations sémantiques** : JSON valide mais qui viole le schéma (types incorrects, propriétés manquantes, valeurs hors plage, etc.)

**Options disponibles :**
- `--syntax` : Génère des JSON syntaxiquement invalides
- `--semantic` : Génère des JSON sémantiquement invalides (défaut)
- `--mutation NAME` : Spécifie une mutation par nom (ex: `one-required-missing`, `wrong-type`, `mixed-types`)
- `--count N` : Nombre de cas à générer (défaut: 5)
- `--output DIR` : Répertoire de sortie pour les fichiers générés
- `--list-mutations` : Liste toutes les mutations disponibles avec leurs descriptions

Voir `MUTATIONS.md` pour la liste complète des mutations disponibles.

## Documentation

Toute la documentation est disponible dans le dossier `doc/` :

- **[doc/TEST_PLAN.md](doc/TEST_PLAN.md)** : Plan de test logiciel détaillé
- **[doc/TEST_RESULTS.md](doc/TEST_RESULTS.md)** : Résultats des tests exécutés
- **[doc/RFC_COVERAGE.md](doc/RFC_COVERAGE.md)** : Vérification de la couverture complète de la RFC 8927
- **[doc/MUTATIONS.md](doc/MUTATIONS.md)** : Documentation complète des mutations du fuzzer
- **[doc/REGEXP_EXTENSION.md](doc/REGEXP_EXTENSION.md)** : Proposition d'extension RegExp via metadata

## Structure du projet

```
src/
  schema/          # Parser et AST pour les schémas JTD
    ast.rs         # Structures de données pour les schémas
    parser.rs      # Parsing et validation syntaxique
    syntax_checks.rs # Vérifications de syntaxe
  validator/        # Moteur de validation
    validate.rs    # Logique de validation
    cli.rs         # Interface en ligne de commande
  tests/           # Suite de tests
    test_cases.rs  # Cas de test individuels
    runner.rs      # Exécuteur de tests
    rfc_analysis.rs # Analyse de la RFC
  fuzzer/          # Fuzzer pour générer des JSON invalides
    generator.rs   # Générateur de JSON valides
    mutations.rs   # Altérations syntaxiques et sémantiques
    cli.rs         # Interface en ligne de commande
  error.rs         # Types d'erreur
  main.rs          # Point d'entrée
```

## Aspects testés

L'outil teste les aspects suivants de la RFC 8927 :

1. **Syntaxe du schéma** : Formes mutuellement exclusives, contraintes syntaxiques
2. **Forme "empty"** : Accepte toutes les instances
3. **Forme "type"** : Tous les types primitifs (boolean, string, timestamp, float32, float64, int8, uint8, int16, uint16, int32, uint32)
4. **Forme "enum"** : Énumération de chaînes, validation des doublons
5. **Forme "elements"** : Tableaux avec validation des éléments
6. **Forme "values"** : Objets avec valeurs uniformes
7. **Forme "properties"** : Objets avec propriétés nommées, optionalProperties, additionalProperties
8. **Forme "discriminator"** : Objets avec tag discriminant
9. **Forme "ref"** : Références à des définitions
10. **Nullable** : Support de null pour toutes les formes
11. **Format des erreurs** : instancePath et schemaPath selon la RFC
12. **Metadata** : Ignorée lors de la validation
13. **Sécurité** : Détection de références circulaires

## Exemples de schémas

### Schéma simple avec type

```json
{
  "type": "string"
}
```

### Schéma avec propriétés

```json
{
  "properties": {
    "name": {"type": "string"},
    "age": {"type": "uint8"}
  },
  "optionalProperties": {
    "email": {"type": "string"}
  }
}
```

### Schéma avec discriminator

```json
{
  "discriminator": "type",
  "mapping": {
    "user": {
      "properties": {
        "name": {"type": "string"}
      }
    },
    "admin": {
      "properties": {
        "name": {"type": "string"},
        "level": {"type": "uint8"}
      }
    }
  }
}
```

## Dépendances

- `serde` / `serde_json` : Sérialisation JSON
- `thiserror` : Gestion d'erreurs
- `chrono` : Validation des timestamps RFC3339
- `regex` : Utilitaires de validation
- `rand` : Génération aléatoire pour le fuzzer

## Licence

Ce projet est fourni à des fins de test et de validation de conformité à la RFC 8927.
