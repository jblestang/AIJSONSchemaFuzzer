# JTD Validator Tests - Outil de test pour la RFC 8927 et JSON Schema 2020-12

Cet outil permet de tester qu'une implémentation de la RFC 8927 (JSON Type Definition, JTD) et de JSON Schema 2020-12 (draft-2020-12) est correcte vis-à-vis de leurs spécifications.

## Fonctionnalités

- **Parser de schéma JTD** : Parse et valide la syntaxe des schémas JTD (RFC 8927)
- **Parser de schéma JSON Schema 2020-12** : Parse et valide la syntaxe des schémas JSON Schema 2020-12 (draft-2020-12)
- **Détection automatique** : Détecte automatiquement le format du schéma (JTD ou JSON Schema 2020-12)
- **Validateur** : Valide des instances JSON contre des schémas JTD ou JSON Schema 2020-12
- **Suite de tests complète** : Couvre tous les aspects de la RFC 8927 et JSON Schema 2020-12
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

Exécute tous les tests de conformité à la RFC 8927 et affiche les résultats.

### Valider une instance contre un schéma

```bash
cargo run -- validate <schema.json> <instance.json>
```

Valide une instance JSON contre un schéma JTD et affiche les erreurs de validation si présentes.

### Générer un JSON valide

```bash
cargo run -- generate <schema.json> [--output <file.json>]
```

Génère une instance JSON valide selon le schéma fourni.

### Fuzzer

```bash
cargo run -- fuzz <schema.json> [--type <mutation_name>] [--count N] [--output DIR] [--list-mutations]
```

Génère des JSON invalides à partir d'un schéma pour tester la robustesse d'un validateur.

Options :
- `--type <mutation_name>` : Spécifie une mutation particulière (voir `--list-mutations`)
- `--count N` : Nombre de cas à générer (défaut: 5)
- `--output DIR` : Répertoire de sortie (défaut: `fuzz_output/`)
- `--list-mutations` : Affiche la liste de toutes les mutations disponibles

Exemples :

```bash
# Générer 10 cas avec mutations aléatoires
cargo run -- fuzz test_schema.json --count 10

# Générer des cas avec une mutation spécifique
cargo run -- fuzz test_schema.json --type wrong-type --count 5

# Lister toutes les mutations disponibles
cargo run -- fuzz test_schema.json --list-mutations
```

## Documentation

Toute la documentation est disponible dans le dossier `doc/` :

- **[doc/USER_MANUAL.md](doc/USER_MANUAL.md)** : **Manuel utilisateur complet** - Guide détaillé de toutes les fonctionnalités
- **[doc/TEST_PLAN.md](doc/TEST_PLAN.md)** : Plan de test logiciel détaillé (JTD et JSON Schema 2020-12)
- **[doc/TEST_RESULTS.md](doc/TEST_RESULTS.md)** : Résultats des tests exécutés (103 tests, 100% de réussite)
- **[doc/RFC_COVERAGE.md](doc/RFC_COVERAGE.md)** : Vérification de la couverture complète de la RFC 8927
- **[doc/MUTATIONS.md](doc/MUTATIONS.md)** : Documentation complète des mutations du fuzzer (61 mutations)
- **[doc/JSON_SCHEMA_2020_12_UPGRADE.md](doc/JSON_SCHEMA_2020_12_UPGRADE.md)** : Documentation de l'upgrade vers JSON Schema 2020-12

## Dépendances

- `serde` / `serde_json` : Sérialisation JSON
- `thiserror` : Gestion d'erreurs
- `chrono` : Validation des timestamps RFC3339
- `regex` : Utilitaires de validation
- `rand` : Génération aléatoire pour le fuzzer

## Licence

Ce projet est fourni à des fins de test et de validation de conformité à la RFC 8927.
