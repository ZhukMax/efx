# EFx

efx — moteur de modèles d'interface utilisateur déclaratif en Rust
`efx!` est une macro procédurale qui transforme le balisage compact de type XML en appels de méthode vers votre interface utilisateur (par exemple, des wrappers sur `egui/eframe`).

## Exemple minimal

```rust
use efx_core::doc_prelude::*;
use efx::*;

efx!(Ui::default(), r#"
    <Column>
        <Label>Bonjour</Label>
        <Separator/>
        <Row><Label>Rangée</Label></Row>
    </Column>
"#);
```
**Caractéristiques principales 0.5**
- Mots clés: `Column`, `Row`, `Label`, `Separator`, `Button`.
- Insérer des expressions : `{expr}` dans le texte.
- S'échapper: `{{` → `{`, `}}` → `}`.
- Les attributs de balise sont **analysés**.

---

### EFx Sandbox (aire de jeux locale)

`efx-sandbox` est un crate binaire d'aide conservé dans ce dépôt. Il est utilisé pour les tests manuels des balises et comme exemple concret d'utilisation de la macro de création de modèles dans une application `egui` réelle.
