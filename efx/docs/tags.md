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

efx!(Ui::default(), "<Separator space_before=\"12\"/>");
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
