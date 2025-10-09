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

---
