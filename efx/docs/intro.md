# EFx

efx — declarative UI template engine in Rust
`efx!` is a procedural macro that transforms compact XML-like markup into method calls to your UI (e.g. wrappers over `egui/eframe`).

## Minimal example

```rust
use efx_core::doc_prelude::*;
use efx::*;

efx!(Ui::default(), r#"
    <Column>
        <Label>Hello</Label>
        <Separator/>
        <Row><Label>Row</Label></Row>
    </Column>
"#);
```
**Key Features 0.5**
- Tags: `Column`, `Row`, `Label`, `Separator`, `Button`.
- Insert expressions: `{expr}` within text.
- Escaping: `{{` → `{`, `}}` → `}`.
- Tag attributes are **parsed**.

For more information, see the sections below: **Supported Tags** and **Syntax Guide**.