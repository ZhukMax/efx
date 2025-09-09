# EFx

efx — moteur de templates UI déclaratif en Rust
`efx!` est une macro procédurale qui transforme un balisage compact de type XML en appels de méthodes vers votre UI (par ex. des wrappers autour de `egui/eframe`).

## Exemple minimal

```rust
use efx_core::doc_prelude::*;
use efx::*;

efx!(Ui::default(), r#"
    <Column>
        <Label>Bonjour</Label>
        <Separator/>
        <Row><Label>Ligne</Label></Row>
    </Column>
"#);
```

**Fonctionnalités principales 0.5**

* Tags : `Column`, `Row`, `Label`, `Separator`, `Button`.
* Insertion d’expressions : `{expr}` dans le texte.
* Échappement : `{{` → `{`, `}}` → `}`.
* Les attributs de tags sont **analysés**.

---

### EFx Sandbox (aire de jeu locale)

`efx-sandbox` est un crate binaire utilitaire inclus dans ce dépôt. Il est utilisé pour tester manuellement les tags et comme exemple “vivant” de l’utilisation de la macro de templating dans une application `egui` réelle.

**Pourquoi l’utiliser**

* Vérifier rapidement le comportement des tags dans une fenêtre native (`eframe/egui`).
* Conserver des exemples riches et des “scènes” en dehors des doctests (pas de limitations de test harness).
* Démontrer comment `efx!` s’intègre avec l’état de l’application.

**Où il se trouve**

`/efx-sandbox`

Ce crate fait partie du workspace et n’est **pas publié**.

**Comment l’exécuter**

```bash
cargo run -p efx-sandbox
```

> Assurez-vous que les versions de `eframe/egui` correspondent à celles utilisées par EFx (nous fixons `eframe = "0.32"` pour `egui 0.32.x`).

**Exemple minimal de `main.rs`**

```rust,ignore
use eframe::{egui, NativeOptions};
use efx::*;                    // la macro efx!
use efx_core::doc_prelude::*;  // prélude pratique pour egui

fn main() -> eframe::Result<()> {
    eframe::run_native(
        "EFx Sandbox",
        NativeOptions::default(),
        Box::new(|_cc| Box::new(App::default())),
    )
}

#[derive(Default)]
struct App {
    counter: i32,
    input: String,
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // En-tête
            let _ = efx!(ui, r#"
                <Column gap="8">
                  <Label size="20" bold="true">EFx sandbox</Label>
                  <Separator/>
                </Column>
            "#);

            // Boutons renvoyant Response
            ui.horizontal(|ui| {
                let inc = efx!(ui, r#"<Button tooltip="Incrémenter">+1</Button>"#);
                if inc.clicked() { self.counter += 1; }

                let dec = efx!(ui, r#"<Button tooltip="Décrémenter">-1</Button>"#);
                if dec.clicked() { self.counter -= 1; }
            });

            // Texte dynamique
            let _ = efx!(ui, r#"<Label>Compteur : {self.counter}</Label>"#);

            // Champ de texte
            let _ = efx!(ui, r#"<TextField value="self.input" hint="tapez ici…"/>"#);

            // Scroll + liens + boutons stylés
            let _ = efx!(ui, r#"
                <ScrollArea axis="vertical" max_height="160" always_show="true" id="demo-log">
                  <Column gap="6">
                    <Label monospace="true">Vous avez tapé : {self.input.clone()}</Label>
                    <Row gap="8">
                      <Hyperlink url="https://efxui.com" tooltip="Site du projet"/>
                      <Hyperlink url="help:about" open_external="false">À propos</Hyperlink>
                    </Row>
                    <Separator/>
                    <Row gap="10" wrap="true">
                      <Button fill="#333333AA" rounding="8">A</Button>
                      <Button frame="false">B</Button>
                      <Button min_width="100" tooltip="Large">Large</Button>
                    </Row>
                  </Column>
                </ScrollArea>
            "#);
        });
    }
}
```

**Conseils**

* Gardez plusieurs “scènes” d’exemple comme `&'static str` et changez-les via un `ComboBox` pour tester différents ensembles de tags.
* Préférez les attributs en **snake\_case** (`max_height`, `always_show`, `stroke_width`, …). Si un tag supporte aussi les alias en kebab-case, sa section le mentionnera.
* Les couleurs sont au format `#RRGGBB` ou `#RRGGBBAA` (les formats courts `#RGB/#RGBA` ne sont pas encore supportés).

**Pourquoi un sandbox au lieu des doctests**

Les doctests sont excellents pour la syntaxe et les messages d’erreur, mais `egui` nécessite une vraie boucle de rendu (`Context::run()`), que les doctests ne fournissent pas. Le sandbox exécute une vraie application, tandis que les exemples de cette documentation sont marqués `rust,ignore` pour éviter leur exécution.

---

Pour plus d’informations, voir les sections suivantes : **Supported Tags** et **Syntax Guide**.
