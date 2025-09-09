## Tags supportés (v0.4+)

> À partir de la version 0.5 certains tags supportent des attributs.
> Les attributs inconnus produisent un `compile_error!`.

### `Column`

Conteneur vertical. Génère `ui.vertical(|ui| { ... })`.

**Attributs**

* `align="left|center|right"` — alignement horizontal des enfants.
* `gap="N"` — espacement vertical entre enfants (f32).
* `padding="N"` — marge supplémentaire haut/bas (f32).

```rust
use efx_core::doc_prelude::*;
use efx::*;

efx!(Ui::default(), r#"<Column gap="10" padding="6" align="center">
  <Label>Titre</Label>
  <Label size="12">Sous-titre</Label>
</Column>"#);
```

### `Row`

Conteneur horizontal. Génère `ui.horizontal(|ui| { ... })`.

**Attributs**

* `align="top|center|bottom"` — alignement vertical des enfants.
* `gap="N"` — espacement horizontal entre enfants (f32).
* `wrap="true|false"` — retour automatique à la ligne si dépassement.
* `padding="N"` — marge supplémentaire gauche/droite (f32).

```rust
use efx_core::doc_prelude::*;
use efx::*;

efx!(Ui::default(), r#"<Row gap="8" padding="4" align="center"><Label>A</Label><Label>B</Label></Row>"#);

efx!(Ui::default(), r#"<Row wrap="true"><Label>Item1</Label><Label>Item2</Label><Label>Item3</Label></Row>"#);
```

### `Label`

Widget texte. Seuls du texte et des interpolations (`{expr}`) sont autorisés comme enfants.

**Attributs**

* `color="name|#RRGGBB[AA]"` — couleur du texte.
* `size="N"` — taille de police (f32).
* `bold="true|false"`.
* `italic="true|false"`.
* `underline="true|false"`.
* `strike="true|false"`.
* `monospace="true|false"`.
* `wrap="true|false"` — activer le retour à la ligne.

```rust
use efx_core::doc_prelude::*;
use efx::*;

efx!(Ui::default(), r##"<Label color="#66CCFF" size="16" bold="true">Bonjour utilisateur</Label>"##);
```

### `Separator`

Séparateur auto-fermable. Aucun enfant autorisé (sinon `compile_error!`).

**Attributs**

* `space="N"` — espacement uniforme avant et après (f32).
* `space_before="N"` — espacement au-dessus.
* `space_after="N"` — espacement en dessous.

```rust
use efx_core::doc_prelude::*;
use efx::*;

efx!(Ui::default(), r#"<Separator space="12"/>"#);
efx!(Ui::default(), r#"<Separator space_before="8" space_after="4"/>"#);
```

```rust,compile_fail
use efx_core::doc_prelude::*;
use efx::*;

/// compile_fail
efx!(Ui::default(), "<Separator>enfant</Separator>");
```

### `Button`

Bouton — c’est le seul tag qui retourne une valeur de type `Resp` à la racine d’une expression.

**Attributs**

* `fill="color"` — couleur de fond.
* `rounding="N"` — rayon d’arrondi (f32).
* `min_width="N", min_height="N"` — taille minimale.
* `frame="true|false"` — dessiner fond/bordure.
* `enabled="true|false"` — activer/désactiver le bouton.
* `tooltip="text"` — info-bulle au survol.

```rust
use efx_core::doc_prelude::*;
use efx::*;

let resp: Resp = efx!(Ui::default(), r#"<Button rounding="8" enabled="false" tooltip="Bientôt">Exécuter</Button>"#);
assert!(!resp.clicked());
```

### `Hyperlink`

Lien cliquable. Génère `ui.hyperlink(url)` ou `ui.hyperlink_to(label, url)`.

**Attributs**

* `url="..."` — adresse de destination (obligatoire).
* `open_external="true|false"` — ouvrir dans le navigateur système (par défaut true).
* `color="name|#RRGGBB[AA]"` — couleur du texte du lien.
* `underline="true|false"` — souligner le texte du lien (par défaut true).
* `tooltip="text"` — info-bulle au survol.

Utilisation multiplateforme :

* **Web :** rendu comme un lien `<a>` standard.
* **Desktop (eframe, bevy\_egui) :** ouvre le navigateur système via `ui.hyperlink(...)`.
* **Overlays jeu/outils :** pratique pour relier docs, dépôts, aide.
* **Apps hors-ligne :** avec schémas d’URL personnalisés (ex. `help://topic`).

```rust
use efx_core::doc_prelude::*;
use efx::*;

efx!(Ui::default(), r##"
    <Column>
        <Hyperlink url="https://efxui.com" color="#66CCFF" tooltip="Site du projet"/>
        <Hyperlink url="help://about" open_external="false">À propos</Hyperlink>
    </Column>
"##);
```

### `TextField`

Champ de saisie (une ou plusieurs lignes). Génère `egui::TextEdit` et l’insère via `ui.add(...)`. Doit être auto-fermable (pas d’enfants).

**Attributs**

* `value="<expr>"` — **obligatoire**. Expression Rust de type `String`, ex. `state.name`. Le générateur prend automatiquement `&mut (<expr>)`.
* `hint="text"` — texte indicatif affiché quand vide.
* `password="true|false"` — masquer les caractères (uniquement en mode ligne simple).
* `width="N"` — largeur souhaitée (f32).
* `multiline="true|false"` — éditeur multi-lignes (`TextEdit::multiline`).

```rust
use efx_core::doc_prelude::*;
use efx::*;

#[derive(Default)]
struct State { name: String }

let mut state = State::default();

// Ligne simple avec placeholder et largeur
efx!(Ui::default(), r#"<TextField value="state.name" hint="Votre nom" width="220"/>"#);

// Champ mot de passe
efx!(Ui::default(), r#"<TextField value="state.name" password="true"/>"#);

// Éditeur multi-lignes
efx!(Ui::default(), r#"<TextField value="state.name" multiline="true" width="320"/>"#);
```

### `CentralPanel`

Zone principale remplissant tout l’espace disponible. Enveloppe ses enfants dans `egui::CentralPanel` et applique éventuellement un `Frame`.

**Attributs**

* `frame="true|false"` — utiliser un frame par défaut (`true`, par défaut) ou aucun (`false`).
* `fill="name|#RRGGBB[AA]"` — couleur de fond.
* `stroke-width="N"` — épaisseur du trait (f32).
* `stroke-color="name|#RRGGBB[AA]"` — couleur du trait.
* `padding="N"` — marge intérieure (f32).
* `padding-left|padding-right|padding-top|padding-bottom="N"`.
* `margin="N"` — marge extérieure (f32).
* `margin-left|margin-right|margin-top|margin-bottom="N"`.

```rust,no_run
use efx_core::doc_prelude::*;
use efx::*;

efx!(Ui::default(), r##"
  <CentralPanel fill="#101014" padding="12" stroke-width="1" stroke-color="#222638">
    <Column gap="8">
      <Label size="18" bold="true">Tableau de bord</Label>
      <Separator space="6"/>
      <Row gap="12">
        <Label>Bienvenue !</Label>
        <Hyperlink url="https://efxui.com">Docs</Hyperlink>
      </Row>
    </Column>
  </CentralPanel>
"##);
```

### `ScrollArea`

Conteneur défilant basé sur `egui::ScrollArea`. Enveloppe ses enfants et fournit un défilement vertical/horizontal/les deux.

**Attributs**

* `axis="vertical|horizontal|both"` — axe de défilement (par défaut: vertical).
* `always-show="true|false"` — toujours afficher la barre de défilement.
* `max-height="N"` — hauteur maximale (f32).
* `max-width="N"` — largeur maximale (f32).
* `id="text"` — identifiant pour persister l’état.
* `bottom="true|false"` — rester collé en bas (logs/chats).
* `right="true|false"` — rester collé à droite.

```rust,ignore
use efx_core::doc_prelude::*;
use efx::*;

// Panneau log vertical collé en bas
efx!(Ui::default(), r#"
  <ScrollArea axis="vertical" max_height="200" always_show="true" id="log-pane" stick_to_bottom="true">
    <Column gap="6">
      <Label bold="true">Log :</Label>
      <Label>Ligne 1</Label>
      <Label>Ligne 2</Label>
      <Label>Ligne 3</Label>
    </Column>
  </ScrollArea>
"#);

// Scroll horizontal
efx!(Ui::default(), r#"
  <ScrollArea axis="horizontal" max_width="320" always_show="true">
    <Row gap="12">
      <Label>Item 1</Label>
      <Label>Item 2</Label>
      <Label>Item 3</Label>
      <Label>Item 4</Label>
    </Row>
  </ScrollArea>
"#);

// Les deux directions (ex. grosse grille)
efx!(Ui::default(), r#"
  <ScrollArea axis="both" max_width="400" max_height="220">
    <Column gap="8">
      <Row gap="8"><Label>A1</Label><Label>A2</Label><Label>A3</Label><Label>A4</Label></Row>
      <Row gap="8"><Label>B1</Label><Label>B2</Label><Label>B3</Label><Label>B4</Label></Row>
      <Row gap="8"><Label>C1</Label><Label>C2</Label><Label>C3</Label><Label>C4</Label></Row>
      <Row gap="8"><Label>D1</Label><Label>D2</Label><Label>D3</Label><Label>D4</Label></Row>
    </Column>
  </ScrollArea>
"#);
```

### `Heading`

Titre de section. Génère `ui.heading(text)` avec des styles optionnels.

**Attributs**

* `level="1..6"` — niveau de titre (entier).
  *Défaut :* `1`. Correspond aux styles prédéfinis d’`egui`.
* `size="N"` — taille de police personnalisée (f32).
* `color="name|#RRGGBB[AA]"` — couleur du texte.
* `tooltip="text"` — info-bulle au survol.

```rust
use efx_core::doc_prelude::*;
use efx::*;

efx!(Ui::default(), r##"
  <Column gap="8">
    <Heading level="1">Titre principal</Heading>
    <Heading level="2" color="#66CCFF">Section</Heading>
    <Heading level="3" size="14" tooltip="Sous-titre">Note</Heading>
  </Column>
"##);
```

L’attribut `level` contrôle le style de base (h1–h6), tandis que `size` et `color` permettent d’affiner l’apparence.
