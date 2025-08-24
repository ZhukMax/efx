## Supported Tags (v0.4+)

> Attributes are parsed, but are currently ignored during rendering.

### `Column`
Vertical container. Generates `ui.vertical(|ui| { ... })`.

```rust
use efx::*;
# efx_doc_prelude!();

efx!(ui, "<Column><Label>A</Label></Column>");
```

### `Row`
Horizontal container. Generates `ui.horizontal(|ui| { ... })`.

```rust
use efx::*;
# efx_doc_prelude!();

efx!(ui, "<Row><Label>B</Label></Row>");
```

### `Label`
Text widget. Only text and interpolations (`{expr}`) in child nodes are allowed.

```rust
use efx::*;
# efx_doc_prelude!();

efx!(ui, "<Label>Hello {1+1}</Label>");
```

### `Separator`
Self-closing divider. No children allowed (otherwise `compile_error!`).

```rust
use efx::*;
# efx_doc_prelude!();

efx!(ui, "<Separator/>");
```

```rust,compile_fail
use efx::*;
# efx_doc_prelude!();

/// compile_fail
efx!(ui, "<Separator>child</Separator>");
```

### `Button`
Button is the only tag that returns a response value (`Resp`) at the root of an expression.

```rust
use efx::*;
# efx_doc_prelude!();

let resp = efx!(ui, "<Button>Click {40+2}</Button>");
assert!(!resp.clicked());
```
