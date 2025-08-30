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

---

### Safety of `{expr}` interpolations

Sometimes developers familiar with PHP or JavaScript templating engines may worry that
expressions inside templates could be unsafe or mix logic with markup.

EFx works differently:

- **Compile-time only**: `{expr}` is expanded by the Rust compiler. There is no `eval`,
  no dynamic string execution at runtime.
- **Type-safe**: inserted code is just normal Rust, fully checked by the compiler.
  If the expression does not compile, the template fails to compile.
- **Limited scope**: interpolations are only allowed inside textual tags such as
  `<Label>` or `<Button>`, where they expand into calls like:

  ```rust
  use efx_core::doc_prelude::*;
  use efx::efx;
  
  let user_name = "Max";
  
  efx!(Ui::default(), "<Label>Hello {user_name}</Label>");
  // expands to:
  Ui::default().label(format!("Hello {}", user_name));
  ```
- **No injection risk**: unlike PHP templating, there is no way for untrusted data
  to introduce new code. All values are rendered through `format!` / `Display`.

In short, EFx keeps declarative style while preserving Rust’s compile-time guarantees.
This makes interpolation safe and predictable, not the dynamic and unsafe practice
associated with classic PHP templates.

### Isn’t writing UI code directly in Rust already safe?

Yes — writing plain Rust with `egui` is already memory-safe.  
EFx does not add any “extra” safety here. Its purpose is different:

- **Reduce boilerplate**: instead of multiple nested closures you can express layouts in compact XML-like markup.
- **Keep Rust guarantees**: interpolations `{expr}` are just Rust code, checked by the compiler.
- **Stay compatible**: EFx expands into regular `ui.*` calls, so you can freely mix EFx snippets with hand-written `egui` code.

In short: Rust already gives you memory safety. EFx gives you *developer ergonomics* on top of it, without sacrificing safety or control.


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
