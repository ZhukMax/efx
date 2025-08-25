## Syntax guide

### Structure
- Elements: `<Name ...>children</Name>` and self-closing `<Name .../>`.
- Text nodes and `{expr}` interpolations are allowed inside `Label`/`Button`.
- Multiple elements are allowed on the root - a block with a list of expressions will be generated.

### Interpolations
You can insert arbitrary Rust expressions inside the text:
```rust
use efx_core::doc_prelude::*;
use efx::*;

efx!(Ui::default(), r#"<Label>Hello {1 + 1}</Label>"#);
```

#### Escaping curly braces
The text `{` and `}` can be obtained as `{{` and `}}` respectively.

### Tag attributes (since 0.4)
They are written as in XML: `name="value"`. At the moment, attributes are **parsed** and available in the AST, 
but the renderer **does not use them** - the processing API will be added in future versions.

```xml
<Label color="green" size="lg">Hi</Label>
```

### Compilation errors
- Unknown tag → `compile_error!`.
- Violation of tag restrictions (e.g. children of `<Separator/>`) → `compile_error!`.
- Invalid fragment in interpolation `{ … }` → `compile_error!` with source fragment.

### Debugging
If you want to see what `efx!` generates, compile with `RUSTFLAGS="--emit=mir,llvm-ir"`.
