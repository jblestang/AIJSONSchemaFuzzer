# Plan de Test Logiciel - JTD Validator Tests

## 1. Introduction

### 1.1 Objectif
Ce document décrit le plan de test pour l'outil de validation et de test de conformité à la RFC 8927 (JSON Type Definition). L'objectif est de vérifier que l'implémentation est correcte et complète vis-à-vis de la spécification.

### 1.2 Portée
Les tests couvrent :
- La validation de schémas JTD selon la RFC 8927
- La validation de schémas JSON Schema 2020-12 (draft-2020-12)
- La validation d'instances JSON contre des schémas JTD ou JSON Schema 2020-12
- La génération de JSON valides et invalides (fuzzer)
- La conformité à toutes les formes et contraintes de la RFC 8927
- La conformité aux mots-clés principaux de JSON Schema 2020-12

### 1.3 Références
- RFC 8927 : JSON Type Definition (JTD)
- RFC 8259 : The JavaScript Object Notation (JSON) Data Interchange Format
- RFC 3339 : Date and Time on the Internet: Timestamps
- JSON Schema 2020-12 : https://json-schema.org/draft/2020-12/json-schema-core

### 1.4 Code Source des Tests
- **[src/tests/test_cases.rs](../src/tests/test_cases.rs)** : Cas de test JTD (RFC 8927)
- **[src/tests/json_schema_test_cases.rs](../src/tests/json_schema_test_cases.rs)** : Cas de test JSON Schema 2020-12
- **[src/tests/runner.rs](../src/tests/runner.rs)** : Exécuteur des tests
- **[src/tests/mod.rs](../src/tests/mod.rs)** : Module de tests

## 2. Stratégie de Test

### 2.1 Types de Tests

#### 2.1.1 Tests Unitaires
- Tests des fonctions individuelles de validation
- Tests des parsers de schéma
- Tests des générateurs de JSON

#### 2.1.2 Tests d'Intégration
- Tests de validation complète schéma + instance
- Tests de génération et validation en chaîne
- Tests des références entre définitions

#### 2.1.3 Tests de Conformité RFC
- Tests de toutes les formes de schéma JTD (8 formes)
- Tests de tous les types primitifs JTD (11 types)
- Tests des contraintes JTD (nullable, additionalProperties, etc.)
- Tests des mots-clés JSON Schema 2020-12 (prefixItems, items, patternProperties, allOf/anyOf/oneOf/not, if/then/else, const, required, contraintes sur tableaux/objets/chaînes/nombres)
- Tests des cas limites et d'erreur

#### 2.1.4 Tests de Fuzzing
- Génération de JSON syntaxiquement invalides
- Génération de JSON sémantiquement invalides
- Tests de robustesse du validateur

### 2.2 Critères de Réussite
- ✅ 100% des cas de test de conformité RFC passent (JTD et JSON Schema 2020-12)
- ✅ Toutes les formes de schéma JTD sont supportées (8 formes)
- ✅ Tous les types primitifs JTD sont validés correctement (11 types)
- ✅ Les mots-clés principaux JSON Schema 2020-12 sont supportés
- ✅ Les erreurs sont rapportées avec les bons chemins (instancePath, schemaPath)
- ✅ Le fuzzer génère des cas de test variés pour JTD et JSON Schema 2020-12

## 3. Catégories de Tests

### 3.1 Syntaxe du Schéma

**Objectif** : Vérifier que les schémas sont correctement parsés et validés.

**Tests** :
- Schéma vide `{}` est de forme 'empty'
- Schéma avec uniquement `nullable` est de forme 'empty'
- Les 8 formes sont mutuellement exclusives
- `definitions` ne peut apparaître qu'au niveau racine
- Les membres comme `nullable` doivent être booléens

**Critères** :
- ✅ Parsing réussi pour schémas valides
- ✅ Erreurs de syntaxe détectées pour schémas invalides

### 3.2 Forme "empty"

**Objectif** : Vérifier que la forme empty accepte toutes les instances.

**Code source** : [src/tests/test_cases.rs](../src/tests/test_cases.rs) - Fonction `all_test_cases()`

**Tests** :
- `{}` accepte n'importe quelle valeur
- `{"nullable": true}` accepte null
- `{"nullable": false}` rejette null

**Critères** :
- ✅ Toutes les instances sont acceptées (sauf null si nullable=false)

### 3.3 Forme "type"

**Objectif** : Vérifier la validation de tous les types primitifs.

**Code source** : [src/tests/test_cases.rs](../src/tests/test_cases.rs) - Fonction `all_test_cases()`

**Types testés** :
- `boolean` : true/false
- `string` : chaînes de caractères
- `timestamp` : format RFC3339
- `float32`, `float64` : nombres flottants
- `int8`, `uint8`, `int16`, `uint16`, `int32`, `uint32` : entiers signés/non signés

**Tests par type** :
- ✅ Accepte les valeurs valides
- ✅ Rejette les valeurs de type incorrect
- ✅ Rejette les valeurs hors plage (pour les entiers)
- ✅ Supporte `nullable: true`

**Critères** :
- ✅ Validation correcte des types
- ✅ Validation correcte des plages pour les entiers
- ✅ Validation correcte du format RFC3339 pour timestamp

### 3.4 Forme "enum"

**Objectif** : Vérifier la validation des énumérations.

**Code source** : [src/tests/test_cases.rs](../src/tests/test_cases.rs) - Fonction `all_test_cases()`

**Tests** :
- Enum accepte une valeur dans la liste
- Enum rejette une valeur non dans la liste
- Enum rejette les non-chaînes
- Enum avec `nullable: true` accepte null
- Enum doit être non vide (test de syntaxe)
- Enum ne doit pas contenir de doublons (test de syntaxe)

**Critères** :
- ✅ Validation correcte des valeurs enum
- ✅ Rejet des valeurs invalides

### 3.5 Forme "elements"

**Objectif** : Vérifier la validation des tableaux.

**Code source** : [src/tests/test_cases.rs](../src/tests/test_cases.rs) - Fonction `all_test_cases()`

**Tests** :
- Instance doit être un tableau
- Tous les éléments doivent satisfaire le schéma `elements`
- Erreurs multiples sont rapportées pour plusieurs éléments invalides
- `instancePath` pointe vers l'élément invalide (ex: `/1`, `/2`)
- `schemaPath` pointe vers `/elements`

**Critères** :
- ✅ Validation récursive des éléments
- ✅ Collecte de toutes les erreurs
- ✅ Chemins d'erreur corrects

### 3.6 Forme "values"

**Objectif** : Vérifier la validation des objets avec valeurs uniformes.

**Code source** : [src/tests/test_cases.rs](../src/tests/test_cases.rs) - Fonction `all_test_cases()`

**Tests** :
- Instance doit être un objet
- Toutes les valeurs doivent satisfaire le schéma `values`
- Erreurs multiples sont rapportées
- `instancePath` pointe vers la valeur invalide (ex: `/key`)

**Critères** :
- ✅ Validation récursive des valeurs
- ✅ Collecte de toutes les erreurs
- ✅ Chemins d'erreur corrects

### 3.7 Forme "properties"

**Objectif** : Vérifier la validation des objets avec propriétés nommées.

**Code source** : [src/tests/test_cases.rs](../src/tests/test_cases.rs) - Fonction `all_test_cases()`

**Tests** :
- Instance doit être un objet
- Toutes les propriétés requises doivent être présentes
- Propriétés manquantes génèrent des erreurs
- `optionalProperties` peuvent être absentes
- `optionalProperties` présentes doivent être valides
- `additionalProperties: false` (défaut) rejette propriétés supplémentaires
- `additionalProperties: true` accepte propriétés supplémentaires
- `additionalProperties` ne s'applique qu'à la forme properties

**Critères** :
- ✅ Validation des propriétés requises
- ✅ Validation des propriétés optionnelles
- ✅ Gestion correcte de `additionalProperties`
- ✅ Chemins d'erreur corrects

### 3.8 Forme "discriminator"

**Objectif** : Vérifier la validation des objets avec tag discriminant.

**Code source** : [src/tests/test_cases.rs](../src/tests/test_cases.rs) - Fonction `all_test_cases()`

**Tests** :
- Instance doit être un objet
- Instance doit avoir la clé `discriminator`
- La valeur du `discriminator` doit être une chaîne
- Le tag doit correspondre à une clé du `mapping`
- L'instance doit satisfaire le schéma du mapping correspondant
- Le tag `discriminator` est exempt de validation dans le schéma mapping
- Tag manquant génère erreur
- Tag non dans mapping génère erreur

**Critères** :
- ✅ Validation du tag discriminant
- ✅ Validation du schéma mapping correspondant
- ✅ Exemption correcte du tag dans le schéma mapping

### 3.9 Forme "ref" et definitions

**Objectif** : Vérifier la résolution des références.

**Code source** : [src/tests/test_cases.rs](../src/tests/test_cases.rs) - Fonction `all_test_cases()`

**Tests** :
- `ref` doit référencer une définition existante
- `ref` vers définition inexistante génère erreur
- `definitions` ne peut apparaître qu'au niveau racine
- Références récursives valides sont supportées
- Références circulaires doivent être détectées

**Critères** :
- ✅ Résolution correcte des références
- ✅ Détection des références invalides
- ✅ Détection des références circulaires

### 3.10 Nullable

**Objectif** : Vérifier le support de null pour toutes les formes.

**Code source** : [src/tests/test_cases.rs](../src/tests/test_cases.rs) - Fonction `all_test_cases()`

**Tests** :
- `nullable: true` permet null pour toutes les formes
- `nullable: false` ou absent rejette null (sauf forme empty)
- `nullable` fonctionne avec toutes les formes

**Critères** :
- ✅ Support correct de nullable pour toutes les formes

### 3.11 Format des Erreurs

**Objectif** : Vérifier que les erreurs sont rapportées correctement.

**Tests** :
- Erreurs sont un tableau d'objets
- Chaque erreur a `instancePath` (JSON Pointer)
- Chaque erreur a `schemaPath` (JSON Pointer)
- Plusieurs erreurs peuvent être rapportées simultanément
- Les chemins sont corrects pour erreurs imbriquées

**Critères** :
- ✅ Format d'erreur conforme à la RFC 8927
- ✅ Chemins JSON Pointer corrects

### 3.12 Metadata

**Objectif** : Vérifier que metadata est ignorée.

**Tests** :
- `metadata` peut contenir n'importe quel objet
- `metadata` n'affecte pas la validation

**Critères** :
- ✅ Metadata ignorée lors de la validation

### 3.13 Sécurité

**Objectif** : Vérifier la protection contre les attaques.

**Tests** :
- Détection de références circulaires
- Pas de boucle infinie lors de validation

**Critères** :
- ✅ Pas de vulnérabilités de sécurité
- ✅ Gestion correcte des cas limites

## 3.14 Tests JSON Schema 2020-12

### 3.14.1 prefixItems (tuples)

**Objectif** : Vérifier la validation des tuples avec `prefixItems`.

**Code source** : [src/tests/json_schema_test_cases.rs](../src/tests/json_schema_test_cases.rs) - Fonction `all_json_schema_test_cases()`

**Tests** :
- `prefixItems` valide les premiers éléments d'un tableau selon leurs schémas respectifs
- `prefixItems` rejette les mauvais types à chaque position
- `prefixItems` avec `items: false` rejette les éléments supplémentaires
- `prefixItems` avec `items: schema` valide les éléments restants selon le schéma `items`
- Combinaison `prefixItems` + `minItems` / `maxItems`

**Critères** :
- ✅ Validation correcte des tuples
- ✅ Gestion correcte de `items` après `prefixItems`

### 3.14.2 patternProperties

**Objectif** : Vérifier la validation des propriétés avec patterns regex.

**Code source** : [src/tests/json_schema_test_cases.rs](../src/tests/json_schema_test_cases.rs) - Fonction `all_json_schema_test_cases()`

**Tests** :
- Propriétés dont les clés match un pattern sont validées selon le schéma correspondant
- Propriétés avec clés qui ne matchent aucun pattern sont gérées selon `additionalProperties`

**Critères** :
- ✅ Validation correcte des patterns regex
- ✅ Gestion correcte des propriétés non matchées

### 3.14.3 allOf / anyOf / oneOf / not

**Objectif** : Vérifier la logique combinatoire.

**Code source** : [src/tests/json_schema_test_cases.rs](../src/tests/json_schema_test_cases.rs) - Fonction `all_json_schema_test_cases()`

**Tests** :
- `allOf` : instance doit satisfaire tous les sous-schémas
- `anyOf` : instance doit satisfaire au moins un sous-schéma
- `oneOf` : instance doit satisfaire exactement un sous-schéma
- `not` : instance ne doit pas satisfaire le schéma

**Critères** :
- ✅ Logique combinatoire correcte

### 3.14.4 if / then / else

**Objectif** : Vérifier la validation conditionnelle.

**Code source** : [src/tests/json_schema_test_cases.rs](../src/tests/json_schema_test_cases.rs) - Fonction `all_json_schema_test_cases()`

**Tests** :
- Si `if` est satisfait, `then` doit être satisfait
- Si `if` n'est pas satisfait, `else` doit être satisfait (si présent)

**Critères** :
- ✅ Validation conditionnelle correcte

### 3.14.5 const

**Objectif** : Vérifier la validation des valeurs constantes.

**Code source** : [src/tests/json_schema_test_cases.rs](../src/tests/json_schema_test_cases.rs) - Fonction `all_json_schema_test_cases()`

**Tests** :
- Instance doit être exactement égale à la valeur `const`
- Types différents sont rejetés même si valeurs similaires

**Critères** :
- ✅ Validation des constantes correcte

### 3.14.6 required

**Objectif** : Vérifier la validation des propriétés requises.

**Code source** : [src/tests/json_schema_test_cases.rs](../src/tests/json_schema_test_cases.rs) - Fonction `all_json_schema_test_cases()`

**Tests** :
- Propriétés listées dans `required` doivent être présentes
- Propriétés manquantes génèrent des erreurs

**Critères** :
- ✅ Validation des propriétés requises correcte

### 3.14.7 Contraintes sur tableaux

**Objectif** : Vérifier les contraintes sur les tableaux.

**Code source** : [src/tests/json_schema_test_cases.rs](../src/tests/json_schema_test_cases.rs) - Fonction `all_json_schema_test_cases()`

**Tests** :
- `minItems` : tableau doit avoir au moins N éléments
- `maxItems` : tableau doit avoir au plus N éléments
- `uniqueItems` : tous les éléments doivent être uniques (si `true`)
- `contains` : tableau doit contenir au moins un élément satisfaisant le schéma

**Critères** :
- ✅ Validation des contraintes sur tableaux correcte

### 3.14.8 Contraintes sur objets

**Objectif** : Vérifier les contraintes sur les objets.

**Code source** : [src/tests/json_schema_test_cases.rs](../src/tests/json_schema_test_cases.rs) - Fonction `all_json_schema_test_cases()`

**Tests** :
- `minProperties` : objet doit avoir au moins N propriétés
- `maxProperties` : objet doit avoir au plus N propriétés

**Critères** :
- ✅ Validation des contraintes sur objets correcte

### 3.14.9 Contraintes sur chaînes

**Objectif** : Vérifier les contraintes sur les chaînes.

**Code source** : [src/tests/json_schema_test_cases.rs](../src/tests/json_schema_test_cases.rs) - Fonction `all_json_schema_test_cases()`

**Tests** :
- `minLength` : chaîne doit avoir au moins N caractères
- `maxLength` : chaîne doit avoir au plus N caractères
- `pattern` : chaîne doit correspondre à l'expression régulière

**Critères** :
- ✅ Validation des contraintes sur chaînes correcte

### 3.14.10 Contraintes sur nombres

**Objectif** : Vérifier les contraintes sur les nombres.

**Code source** : [src/tests/json_schema_test_cases.rs](../src/tests/json_schema_test_cases.rs) - Fonction `all_json_schema_test_cases()`

**Tests** :
- `minimum` / `exclusiveMinimum` : nombre doit être >= (ou >) la valeur
- `maximum` / `exclusiveMaximum` : nombre doit être <= (ou <) la valeur
- `multipleOf` : nombre doit être un multiple de la valeur

**Critères** :
- ✅ Validation des contraintes sur nombres correcte

### 3.14.11 $ref vers $defs

**Objectif** : Vérifier la résolution des références.

**Code source** : [src/tests/json_schema_test_cases.rs](../src/tests/json_schema_test_cases.rs) - Fonction `all_json_schema_test_cases()`

**Tests** :
- `$ref` vers `#/$defs/Name` résout correctement la définition
- Références inexistantes génèrent des erreurs

**Critères** :
- ✅ Résolution des références correcte

## 4. Tests du Fuzzer

### 4.1 Mutations Syntaxiques

**Objectif** : Générer des JSON syntaxiquement invalides.

**Mutations testées** :
- Accolades manquantes
- Caractères invalides
- Virgules trailing
- Colon remplacé par égal
- JSON tronqué
- Mélange tabulations/espaces

**Critères** :
- ✅ Génération de JSON mal formés
- ✅ Variété des mutations

### 4.2 Mutations Sémantiques

**Objectif** : Générer des JSON valides syntaxiquement mais invalides sémantiquement.

**Mutations testées (JTD)** :
- Types incorrects
- Valeurs hors plage
- Propriétés manquantes
- Propriétés supplémentaires
- Valeurs enum invalides
- Types mixtes dans tableaux

**Mutations testées (JSON Schema 2020-12)** :
- `prefixItems` : mauvais types, éléments supplémentaires, pas assez d'éléments
- `patternProperties` : valeurs invalides pour propriétés matchant le pattern
- `allOf` / `anyOf` / `oneOf` / `not` : violations de logique combinatoire
- `if` / `then` / `else` : violations de validation conditionnelle
- `const` : valeurs différentes de la constante
- `required` : propriétés requises manquantes
- Contraintes : violations de `minItems`, `maxItems`, `minLength`, `maxLength`, `pattern`, `minimum`, `maximum`, `multipleOf`

**Critères** :
- ✅ Génération de JSON sémantiquement invalides
- ✅ Couverture de toutes les formes de schéma JTD
- ✅ Couverture des mots-clés principaux JSON Schema 2020-12

## 5. Environnement de Test

### 5.1 Outils
- **Langage** : Rust
- **Framework de test** : Tests unitaires Rust intégrés
- **Outils CLI** : Commandes `cargo run -- run-tests`
- **Fuzzer** : Outil intégré avec mutations nommées

### 5.2 Exécution
```bash
# Exécuter tous les tests
cargo run -- run-tests
# Code source : [src/tests/runner.rs](../src/tests/runner.rs) - Fonction `run_all_tests()`

# Analyser la RFC
cargo run -- analyze-rfc

# Tester une validation spécifique
cargo run -- validate schema.json instance.json

# Générer des cas de test avec le fuzzer
cargo run -- fuzz schema.json --semantic --count 10
```

## 6. Critères d'Acceptation

### 6.1 Couverture
- ✅ Toutes les 8 formes de schéma JTD testées
- ✅ Tous les 11 types primitifs JTD testés
- ✅ Toutes les contraintes JTD testées (nullable, additionalProperties, etc.)
- ✅ Mots-clés principaux JSON Schema 2020-12 testés (prefixItems, items, patternProperties, allOf/anyOf/oneOf/not, if/then/else, const, required, contraintes)
- ✅ Tous les cas d'erreur testés

### 6.2 Qualité
- ✅ Tous les tests passent (objectif : 100%)
- ✅ Les erreurs sont rapportées correctement
- ✅ Les chemins JSON Pointer sont corrects
- ✅ Pas de régressions

### 6.3 Performance
- ✅ Validation rapide même pour schémas complexes
- ✅ Pas de boucles infinies
- ✅ Gestion mémoire correcte

## 7. Planification

### 7.1 Phases de Test

**Phase 1 : Tests Unitaires**
- Tests des fonctions de validation individuelles
- Tests des parsers

**Phase 2 : Tests d'Intégration**
- Tests de validation complète
- Tests des références

**Phase 3 : Tests de Conformité RFC**
- Tests de toutes les formes
- Tests de tous les types
- Tests des cas limites

**Phase 4 : Tests de Fuzzing**
- Génération de cas de test
- Validation de robustesse

### 7.2 Maintenance
- Les tests doivent être maintenus à jour avec l'évolution du code
- Nouveaux tests ajoutés pour nouvelles fonctionnalités
- Révision périodique de la couverture

## 8. Risques et Limitations

### 8.1 Risques
- **Couverture incomplète** : Certains cas limites peuvent ne pas être testés
- **Tests non maintenus** : Les tests peuvent devenir obsolètes
- **Performance** : Tests lents pour schémas très complexes

### 8.2 Limitations
- Tests manuels pour certains cas complexes
- Pas de tests de performance automatisés
- Pas de tests de compatibilité inter-implémentations

## 9. Métriques

### 9.1 Métriques de Test
- **Nombre total de tests JTD** : 52 cas de test
- **Nombre total de tests JSON Schema 2020-12** : 51 cas de test
- **Nombre total de tests** : 103 cas de test
- **Taux de réussite** : 100% (tous les tests passent)
- **Couverture des formes JTD** : 8/8 (100%)
- **Couverture des types JTD** : 11/11 (100%)
- **Couverture des mots-clés JSON Schema 2020-12** : 20+ mots-clés principaux

### 9.2 Métriques de Code
- **Couverture de code** : À mesurer avec outils Rust
- **Complexité cyclomatique** : À maintenir faible
- **Maintenabilité** : Code bien structuré et documenté

## 10. Conclusion

Ce plan de test assure une couverture complète de la RFC 8927 et garantit la qualité de l'implémentation. Les tests sont organisés par catégorie et couvrent tous les aspects de la spécification.
