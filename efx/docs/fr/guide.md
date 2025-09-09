## Guide de syntaxe

### Structure

* Éléments : `<Name ...>enfants</Name>` et auto-fermants `<Name .../>`.
* Les nœuds de texte et les interpolations `{expr}` sont autorisés à l’intérieur de `Label`/`Button`.
* Plusieurs éléments sont autorisés à la racine — un bloc avec une liste d’expressions sera généré.

### Interpolations

Vous pouvez insérer des expressions Rust arbitraires dans le texte :

```rust
use efx_core::doc_prelude::*;
use efx::*;

efx!(Ui::default(), r#"<Label>Bonjour {1 + 1}</Label>"#);
```

---

### Sécurité des interpolations `{expr}`

Certains développeurs, habitués aux moteurs de templates PHP ou JavaScript, peuvent craindre que les expressions dans les templates soient dangereuses ou mélangent logique et balisage.

EFx fonctionne différemment :

* **Uniquement à la compilation** : `{expr}` est développé par le compilateur Rust. Il n’y a pas de `eval`, pas d’exécution de chaîne dynamique au runtime.
* **Typé et sûr** : le code inséré est du Rust normal, entièrement vérifié par le compilateur.
  Si l’expression ne compile pas, le template échoue à la compilation.
* **Portée limitée** : les interpolations ne sont autorisées que dans des tags textuels comme `<Label>` ou `<Button>`, où elles sont développées en appels comme :

  ```rust
  use efx_core::doc_prelude::*;
  use efx::efx;

  let user_name = "Max";

  efx!(Ui::default(), "<Label>Bonjour {user_name}</Label>");
  // s’étend en :
  Ui::default().label(format!("Bonjour {}", user_name));
  ```
* **Aucun risque d’injection** : contrairement aux templates PHP, il est impossible pour des données non fiables d’introduire du nouveau code. Toutes les valeurs passent par `format!` / `Display`.

En résumé, EFx garde un style déclaratif tout en préservant les garanties de compilation de Rust.
Cela rend l’interpolation sûre et prévisible, loin des pratiques dynamiques et risquées des templates PHP classiques.

### Écrire directement du code UI en Rust n’est-il pas déjà sûr ?

Oui — écrire du Rust natif avec `egui` est déjà mémoire-sûr.
EFx n’ajoute pas de sécurité supplémentaire ici. Son but est différent :

* **Réduire le boilerplate** : au lieu de closures imbriquées, vous pouvez exprimer des layouts en balisage XML compact.
* **Préserver les garanties Rust** : les interpolations `{expr}` sont du Rust, vérifiées par le compilateur.
* **Rester compatible** : EFx se compile en appels `ui.*` normaux, vous pouvez mélanger librement snippets EFx et code `egui` écrit à la main.

En bref : Rust vous donne la sûreté mémoire. EFx vous donne *l’ergonomie développeur* par-dessus, sans sacrifier la sécurité ni le contrôle.

#### Échapper les accolades

Le texte `{` et `}` s’obtient via `{{` et `}}` respectivement.

### Attributs de tag (depuis 0.4)

Ils s’écrivent comme en XML : `name="value"`.
Pour l’instant, les attributs sont **analysés** et disponibles dans l’AST,
mais le renderer **ne les utilise pas** — l’API de traitement sera ajoutée dans de futures versions.

```xml
<Label color="green" size="lg">Salut</Label>
```

### Erreurs de compilation

* Tag inconnu → `compile_error!`.
* Violation des restrictions d’un tag (ex. enfants de `<Separator/>`) → `compile_error!`.
* Fragment invalide dans une interpolation `{ … }` → `compile_error!` avec extrait de code source.

### Débogage

Si vous voulez voir ce que génère `efx!`, compilez avec :

```bash
RUSTFLAGS="--emit=mir,llvm-ir"
```
