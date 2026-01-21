# Upgrade vers JSON Schema 2020-12

## Vue d'ensemble

Ce document décrit la mise à niveau du projet pour supporter JSON Schema 2020-12 (draft-2020-12) en plus de JTD (RFC 8927).

Référence : https://json-schema.org/draft/2020-12/json-schema-core

## Différences principales entre JTD et JSON Schema 2020-12

### JTD (RFC 8927)
- Format minimaliste avec 8 formes mutuellement exclusives
- `elements` : valide tous les éléments d'un tableau avec le même schéma
- `definitions` : définitions au niveau racine
- `ref` : références simples
- Pas de `prefixItems`, `unevaluatedItems`, etc.

### JSON Schema 2020-12
- Format riche avec de nombreux mots-clés
- `prefixItems` : valide les premiers éléments d'un tableau (tuples)
- `items` : valide les éléments restants après `prefixItems`
- `$defs` : définitions (remplace `definitions`)
- `$ref` / `$dynamicRef` : références statiques et dynamiques
- `unevaluatedItems` / `unevaluatedProperties` : contraintes sur éléments non évalués
- `patternProperties` : propriétés avec patterns regex
- Et bien plus...

## Fonctionnalités à implémenter

### 1. Détection du format
- ✅ Détecter automatiquement si un schéma est JTD ou JSON Schema 2020-12
- ✅ Parser selon le format détecté

### 2. AST pour JSON Schema 2020-12
- ✅ Structure complète créée (`JsonSchema2020`, `JsonSchemaObject`)
- ✅ Support de tous les mots-clés principaux

### 3. Validation JSON Schema 2020-12
- ✅ `prefixItems` : validation des tuples
- ✅ `items` : validation des éléments restants
- ✅ `patternProperties` : propriétés avec patterns regex
- ✅ `$ref` : résolution des références vers `$defs`
- ✅ `allOf` / `anyOf` / `oneOf` / `not` : logique combinatoire
- ✅ `if` / `then` / `else` : validation conditionnelle
- ✅ `minItems` / `maxItems` / `uniqueItems` / `contains` : contraintes sur tableaux
- ✅ `minProperties` / `maxProperties` : contraintes sur objets
- ✅ `minLength` / `maxLength` / `pattern` : contraintes sur chaînes
- ✅ `minimum` / `maximum` / `exclusiveMinimum` / `exclusiveMaximum` / `multipleOf` : contraintes sur nombres
- ✅ `required` : propriétés requises
- ✅ `const` : valeurs constantes
- ⏳ `unevaluatedItems` / `unevaluatedProperties` : à implémenter
- ⏳ `$dynamicRef` / `$dynamicAnchor` : références dynamiques à implémenter

### 4. Tests
- ✅ Tests pour `prefixItems` (3 tests)
- ✅ Tests pour `items` après `prefixItems` (2 tests)
- ✅ Tests pour `patternProperties` (2 tests)
- ✅ Tests pour `allOf` / `anyOf` / `oneOf` (6 tests)
- ✅ Tests pour `$ref` (1 test)
- ✅ Tests pour contraintes sur tableaux (8 tests)
- ✅ Tests pour contraintes sur objets (6 tests)
- ✅ Tests pour contraintes sur chaînes (6 tests)
- ✅ Tests pour contraintes sur nombres (8 tests)
- ✅ Tests pour `const`, `not`, `if/then/else` (8 tests)
- ✅ **Total : 51 tests JSON Schema 2020-12, tous passent**

### 5. Documentation
- ✅ Mise à jour du README principal
- ✅ Mise à jour du document d'upgrade
- ⏳ Exemples JSON Schema 2020-12 dans la documentation
- ⏳ Guide de migration JTD → JSON Schema 2020-12

## Statut actuel

- ✅ Structure AST complète créée
- ✅ Parser JSON Schema 2020-12 avec détection automatique
- ✅ Validateur JSON Schema 2020-12 (prefixItems, items, patternProperties, allOf/anyOf/oneOf, $ref, etc.)
- ✅ Tests (51 tests JSON Schema 2020-12, tous passent)
- ✅ Documentation mise à jour
- ✅ CLI mis à jour pour détection automatique

## Résultats des tests

- **JTD (RFC 8927)** : 52 tests, tous passent ✅
- **JSON Schema 2020-12** : 51 tests, tous passent ✅
- **Total** : 103 tests, tous passent ✅

## Mutations du Fuzzer

- **JTD** : 29 mutations sémantiques + 9 mutations syntaxiques = 38 mutations
- **JSON Schema 2020-12** : 23 mutations sémantiques
- **Total** : 61 mutations

## Prochaines étapes

1. ⏳ Implémenter `unevaluatedItems` / `unevaluatedProperties` : contraintes sur éléments non évalués
2. ⏳ Implémenter `$dynamicRef` / `$dynamicAnchor` : références dynamiques
3. ⏳ Ajouter des tests pour `unevaluatedItems`, `unevaluatedProperties`, `$dynamicRef`, `$dynamicAnchor`
4. ⏳ Compléter le guide de migration JTD → JSON Schema 2020-12

## Mutations du Fuzzer

Le fuzzer supporte maintenant **23 mutations sémantiques** pour JSON Schema 2020-12 :

- **prefixItems** : 6 mutations (wrong-type, extra, too-few, invalid-items, min-items-violation, max-items-violation)
- **patternProperties** : 1 mutation
- **allOf/anyOf/oneOf/not** : 4 mutations
- **if/then/else** : 2 mutations
- **const** : 1 mutation
- **required** : 1 mutation
- **Contraintes** : 8 mutations (minItems, maxItems, minLength, maxLength, pattern, minimum, maximum, multipleOf)
