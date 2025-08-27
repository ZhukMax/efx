## Supported Tags (v0.4+)

> Starting with 0.5 some tags support attributes.
> Unknown attributes result in `compile_error!`.

### `Column`
Vertical container. Generates `ui.vertical(|ui| { ... })`.

**Attributes**
- `align="left|center|right"` — horizontal alignment of children.
- `gap="N"` — vertical spacing between children (f32).
- `padding="N"` — extra top/bottom padding (f32).

```rust
use efx_core::doc_prelude::*;
use efx::*;

efx!(Ui::default(), r#"<Column gap="10" padding="6" align="center">
  <Label>Title</Label>
  <Label size="12">Subtitle</Label>
</Column>"#);
```

### `Row`
Horizontal container. Generates `ui.horizontal(|ui| { ... })`.

**Attributes**

- `align="top|center|bottom"` — vertical alignment of children.
- `gap="N"` — horizontal spacing between children (f32).
- `wrap="true|false"` — wrap children to next line if overflow.
- `padding="N"` — extra left/right padding (f32).

```rust
use efx_core::doc_prelude::*;
use efx::*;

efx!(Ui::default(), r#"<Row gap="8" padding="4" align="center"><Label>A</Label><Label>B</Label></Row>"#);

efx!(Ui::default(), r#"<Row wrap="true"><Label>Item1</Label><Label>Item2</Label><Label>Item3</Label></Row>"#);

```

### `Label`
Text widget. Only text and interpolations (`{expr}`) in child nodes are allowed.

**Attributes**

- `color="name|#RRGGBB[AA]"` — text color.
- `size="N"` — font size (f32).
- `bold="true|false"`.
- `italic="true|false"`.
- `underline="true|false"`.
- `strike="true|false"`.
- `monospace="true|false"`.
- `wrap="true|false"` — enable line wrapping.

```rust
use efx_core::doc_prelude::*;
use efx::*;

efx!(Ui::default(), r##"<Label color="#66CCFF" size="16" bold="true">Hello user</Label>"##);
```

### `Separator`
Self-closing divider. No children allowed (otherwise `compile_error!`).

**Attributes**

- `space="N"` — uniform spacing before & after (f32).
- `space_before="N"` — spacing above.
- `space_after="N"` — spacing below.

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
efx!(Ui::default(), "<Separator>child</Separator>");
```

### `Button`
Button is the only tag that returns a response value (`Resp`) at the root of an expression.

**Attributes**

- `fill="color`" — background fill color.
- `rounding="N"` — rounding radius (f32).
- `min_width="N", min_height="N"` — minimum size.
- `frame="true|false"` — draw background/border.
- `enabled="true|false"` — disable/enable button.
- `tooltip="text"` — hover tooltip.

```rust
use efx_core::doc_prelude::*;
use efx::*;

let resp: Resp = efx!(Ui::default(), r#"<Button rounding="8" enabled="false" tooltip="Soon">Run</Button>"#);
assert!(!resp.clicked());
```

### `Hyperlink`
Clickable link widget. Generates `ui.hyperlink(url)` or `ui.hyperlink_to(label, url)`.

**Attributes**

- `url="..."` — destination address (string, required).
- `open_external="true|false"` — open link in system browser (default true).
- `color="name|#RRGGBB[AA]"` — link text color.
- `underline="true|false"` — underline link text (default true).
- `tooltip="text"` — hover tooltip.

Cross-platform usage

- **Web:** renders as standard `<a>` link.
- **Desktop (eframe, bevy_egui):** opens system browser via `ui.hyperlink(...)`.
- **Game/tool overlays:** convenient way to link to docs, repos, or help.
- **Offline apps:** with custom URL schemes (e.g. `help://topic`) may open in-app help instead of browser.

```rust
use efx_core::doc_prelude::*;
use efx::*;

efx!(Ui::default(), r##"
    <Column>
        <Hyperlink url="https://efxui.com" color="#66CCFF" tooltip="Project site"/>
        <Hyperlink url="help://about" open_external="false">About</Hyperlink>
    </Column>
"##);
```

### `TextField`
Single-line or multi-line text input. Generates `egui::TextEdit` and inserts it via `ui.add(...)`. Must be self-closing (no children).

**Attributes**

- `value="<expr>"` — **required**. Rust lvalue expression of type `String`, e.g. `state.name`. The generator takes `&mut (<expr>)` automatically.
- `hint="text"` — placeholder text shown when empty.
- `password="true|false"` — mask characters (applies to single-line; ignored with `multiline="true"`).
- `width="N"` — desired width in points (f32).
- `multiline="true|false"` — multi-line editor (`TextEdit::multiline`).

```rust
use efx_core::doc_prelude::*;
use efx::*;

#[derive(Default)]
struct State { name: String }

let mut state = State::default();

// Single-line with placeholder and width
efx!(Ui::default(), r#"<TextField value="state.name" hint="Your name" width="220"/>"#);

// Password field (single-line)
efx!(Ui::default(), r#"<TextField value="state.name" password="true"/>"#);

// Multiline editor
efx!(Ui::default(), r#"<TextField value="state.name" multiline="true" width="320"/>"#);
```

### `CentralPanel`

Main content area that fills all remaining space. Wraps children in `egui::CentralPanel` and applies an optional `Frame`.

**Attributes**

- `frame="true|false"` — use default frame (`true`, default) or `none` (`false`).
- `fill="name|#RRGGBB[AA]"` — background fill color.
- `stroke_width="N"` — frame stroke width (f32).
- `stroke_color="name|#RRGGBB[AA]"` — frame stroke color.
- `padding="N"` — inner margin on all sides (f32).
- `padding_l|padding_r|padding_t|padding_b="N"` — per-side inner margin.
- `margin="N"` — outer margin on all sides (f32).
- `margin_l|margin_r|margin_t|margin_b="N"` — per-side outer margin.

```rust,no_run
use efx_core::doc_prelude::*;
use efx::*;

efx!(Ui::default(), r##"
  <CentralPanel fill="#101014" padding="12" stroke_width="1" stroke_color="#222638">
    <Column gap="8">
      <Label size="18" bold="true">Dashboard</Label>
      <Separator space="6"/>
      <Row gap="12">
        <Label>Welcome!</Label>
        <Hyperlink url="https://efxui.com">Docs</Hyperlink>
      </Row>
    </Column>
  </CentralPanel>
"##);
```

### `ScrollArea`

Scrollable container backed by `egui::ScrollArea`. Wraps its children and provides vertical/horizontal/both scrolling.

**Attributes**
- `axis="vertical|horizontal|both"` — scroll axis (default: vertical).
- `always_show="true|false"` — always show scrollbar even if content fits.
- `max_height="N"` — maximum height of the scroll area (f32).
- `max_width="N"` — maximum width of the scroll area (f32).
- `id="text"` — id source to persist scroll state between frames.
- `stick_to_bottom="true|false"` — keep view pinned to bottom when new content arrives (useful for logs/chats).
- `stick_to_right="true|false"` — keep view pinned to right on updates.

```rust,ignore
use efx_core::doc_prelude::*;
use efx::*;

// Vertical log panel with sticky bottom
efx!(Ui::default(), r#"
  <ScrollArea axis="vertical" max_height="200" always_show="true" id="log-pane" stick_to_bottom="true">
    <Column gap="6">
      <Label bold="true">Log:</Label>
      <Label>Line 1</Label>
      <Label>Line 2</Label>
      <Label>Line 3</Label>
    </Column>
  </ScrollArea>
"#);

// Horizontal scroller
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

// Both directions (e.g., big grid)
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
