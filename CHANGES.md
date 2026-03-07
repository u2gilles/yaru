# Changelog de la bibliothèque yaru

## v0.2.3 — Fix: Compatibilité Web (WASM)

### 🐛 Correction : Compilation WASM
La bibliothèque échouait à compiler pour la cible `wasm32-unknown-unknown` car sa dépendance `comfy-table` activait par défaut des fonctionnalités de gestion du terminal (`crossterm`) incompatibles avec le Web.

**Changements :**
- **`comfy-table`** : Désactivation des `default-features`. Cela supprime la dépendance transitive à `crossterm` et permet à `yaru` d'être utilisé dans des projets Dioxus Web/WASM.
- L'affichage des tables SQL reste fonctionnel mais sans les couleurs/styles dynamiques du terminal (qui ne sont pas supportés dans un navigateur de toute façon).

---

## v0.2.1 — SQL : fonctions → macros (breaking change)

### ⚠️ Breaking Change : API SQL

Les 3 fonctions d'affichage SQL ont été remplacées par des **macros** `#[macro_export]`.
Le paramètre `name: &str` est supprimé : le nom de la variable est capturé automatiquement
via `stringify!`.

**Avant (v0.1.x) :**
```rust
yaru::print_sql_table("tasks", &tasks);
yaru::print_sql_json_table("result", &result);
yaru::print_sql_json("items", &items);
```

**Après (v0.2.1) :**
```rust
yaru::print_sql_table!(tasks);
yaru::print_sql_json_table!(result);
yaru::print_sql_json!(items);
```

### Détail des changements

1. **`print_sql_table!`** — Affiche uniquement la table UTF-8
2. **`print_sql_json_table!`** — Affiche le Debug (`{:#?}`) puis la table
3. **`print_sql_json!`** — Affiche uniquement le Debug

- `internal_print_sql_table` est maintenant `pub` (avec `#[doc(hidden)]`) pour être
  accessible depuis les macros `#[macro_export]`
- Format des titres : `=> TABLE: tasks (3)` et `=> JSON: tasks (3)`
- Docstrings complètes (crates.io-ready) sur chaque macro
- README, lib.rs et module doc mis à jour

---

## Résumé des modifications (v0.1.x)

### 1. Nouveau module time_log avec fonctions format

Ajout de fonctions `format` pour chaque fonction `print` existante dans `time_log.rs`:

**Fonctions ajoutées:**
- `t_format(msg: &str) -> String` - Retourne un message formaté avec timestamp
- `nt_format(msg: &str) -> String` - Retourne un message formaté avec newline + timestamp
- `ti_format(msg: &str) -> String` - Retourne un message formaté avec thread ID + timestamp
- `nti_format(msg: &str) -> String` - Retourne un message formaté avec newline + thread ID + timestamp

**Macros ajoutées:**
- `t_format!(...)` - Version macro de t_format
- `nt_format!(...)` - Version macro de nt_format
- `ti_format!(...)` - Version macro de ti_format
- `nti_format!(...)` - Version macro de nti_format

**Pattern de refactoring:**
Les fonctions `print` ont été refactorées pour appeler les fonctions `format`, puis faire un `println!`:
```rust
pub fn t_print(msg: &str) {
    println!("{}", t_format(msg));
}
```

Cela évite la duplication de code et garantit la cohérence.

### 2. Documentation améliorée

#### Dans README.md:
- Ajout d'une section "Print vs Format" expliquant les deux versions
- Table mise à jour montrant les variantes `t*`, `nt*`, `ti*`, `nti*`
- Exemples d'utilisation des fonctions format
- **EMPHASE FORTE** sur la règle importante: NE PAS utiliser `&` avec les macros

#### Documentation détaillée sur l'utilisation des macros:
- ✅ Exemples corrects: `print_ptr!(v)`, `print_vec_ptr!(v)`, `print_box_ptr!(b)`
- ❌ Exemples incorrects: `print_ptr!(&v)` - crée `&&v` (double référence temporaire)
- Explication du "pourquoi": les macros ajoutent déjà le `&` automatiquement
- Section "Quick Reference" pour un accès rapide

#### Dans lib.rs:
- Documentation du module mise à jour avec les fonctions format
- Exemples complets montrant print ET format

#### Dans ptr_inspect/mod.rs:
- Documentation renforcée sur l'utilisation correcte des macros
- Exemples étendus avec Box, Rc, Vec
- Explication claire de pourquoi `&` est problématique

### 3. Convention de nommage cohérente

**IMPORTANT:** Tous les noms utilisent des underscores pour être cohérents avec le reste de l'API:
- `t_format` (pas `tformat`)
- `nt_format` (pas `ntformat`)
- `ti_format` (pas `tiformat`)
- `nti_format` (pas `ntiformat`)

Et pareillement pour les macros:
- `t_format!` (pas `tformat!`)
- `nt_format!` (pas `ntformat!`)
- etc.

### 4. Recommandations mises en évidence

Le README et la documentation insistent particulièrement sur ces points clés:

**Pour les macros de ptr_inspect:**
```rust
let v = vec![1, 2, 3];
print_vec_ptr!(v);       // ✅ CORRECT
print_vec_ptr!(&v);      // ❌ MAUVAIS - crée &&v

let b = Box::new(42);
print_box_ptr!(b);       // ✅ CORRECT
print_box_ptr!(&b);      // ❌ MAUVAIS
```

**Règle simple:** Passez la valeur directement à la macro, sans `&`. La macro s'occupe d'emprunter pour vous.

### 5. Fichiers modifiés

1. `libs/yaru/src/time_log.rs` - Ajout des fonctions et macros format
2. `libs/yaru/README.md` - Documentation étendue et amélorée
3. `libs/yaru/src/lib.rs` - Documentation du module mise à jour
4. `libs/yaru/src/ptr_inspect/mod.rs` - Documentation renforcée
5. `libs/yaru/examples/test_format.rs` - Nouveau fichier de test (créé)

## Prêt pour publication sur crates.io

La bibliothèque est maintenant prête pour être publiée sur crates.io avec:
- API complète et cohérente (print ET format pour toutes les fonctions)
- Documentation exhaustive et claire
- Exemples pratiques tout au long
- Emphase forte sur les bonnes pratiques d'utilisation
- Zero dépendances
- Convention de nommage cohérente
