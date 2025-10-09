### `Separator`
Self-closing divider. No children allowed (otherwise `compile_error!`).

**Attributes**

- `space="N"` — uniform spacing before & after (f32).
- `space_before="N"` — spacing above.
- `space_after="N"` — spacing below.

```xml
<Separator space="12"/>
<Separator space_before="8" space_after="4"/>
```

```rust,compile_fail
use efx_core::doc_prelude::*;
use efx::*;

/// compile_fail
efx!(Ui::default(), "<Separator>child</Separator>");
```

---
