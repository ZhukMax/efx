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

---
