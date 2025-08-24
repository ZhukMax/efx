# EFx

efx — declarative UI template engine in Rust
`efx!` is a procedural macro that transforms compact XML-like markup into method calls to your UI (e.g. wrappers over `egui/eframe`).

## Minimal example

```rust
use efx::efx;
# // --- doctest prelude (hidden) ---
# #[derive(Default)] struct Ui;
# struct Resp; impl Resp { fn clicked(&self)->bool { false } }
# impl Ui {
    #   fn label<S: Into<String>>(&mut self, _s: S) {}
    #   fn button<S: Into<String>>(&mut self, _s: S) -> Resp { Resp }
    #   fn separator(&mut self) {}
    #   fn horizontal<F: FnOnce(&mut Ui)>(&mut self, f: F) { let mut inner = Ui::default(); f(&mut inner); }
    #   fn vertical<F: FnOnce(&mut Ui)>(&mut self, f: F) { let mut inner = Ui::default(); f(&mut inner); }
    # }
# let mut ui = Ui::default();
# // --- end prelude ---

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