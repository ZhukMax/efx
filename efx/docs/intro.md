# EFx

efx — declarative UI template engine in Rust
`efx!` is a procedural macro that transforms compact XML-like markup into method calls to your UI (e.g. wrappers over `egui/eframe`).

## Minimal example

```rust
use efx::*;
# efx_doc_prelude!();

efx!(ui, r#"
    <Column>
        <Label>Hello</Label>
        <Separator/>
        <Row><Label>Row</Label></Row>
    </Column>
"#);
```
**Key Features 0.4**
- Tags: `Column`, `Row`, `Label`, `Separator`, `Button`.
- Insert expressions: `{expr}` within text.
- Escaping: `{{` → `{`, `}}` → `}`.
- Tag attributes are **parsed**, API for use in the renderer will be expanded.

For more information, see the sections below: **Supported Tags** and **Syntax Guide**.