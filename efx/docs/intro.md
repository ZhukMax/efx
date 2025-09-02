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

---

### EFx Sandbox (local playground)

`efx-sandbox` is a helper binary crate kept in this repository. It’s used for manual testing of tags and as a “live” example of how to use the templating macro in a real `egui` app.

**Why use it**

* Quickly verify tag behavior in a native window (`eframe/egui`).
* Keep rich examples and “scenes” outside doctests (no test harness limitations).
* Demonstrate how `efx!` integrates with application state.

**Where it lives**

`/efx-sandbox`

This crate is part of the workspace and is **not published**.

**How to run**

```bash
cargo run -p efx-sandbox
```

> Make sure `eframe/egui` versions match those used by EFx (we pin `eframe = "0.32"` for `egui 0.32.x`).

**Minimal `main.rs` example**

```rust,ignore
use eframe::{egui, NativeOptions};
use efx::*;                    // the efx! macro
use efx_core::doc_prelude::*;  // convenient egui prelude

fn main() -> eframe::Result<()> {
    eframe::run_native(
        "EFx Sandbox",
        NativeOptions::default(),
        Box::new(|_cc| Box::new(App::default())),
    )
}

#[derive(Default)]
struct App {
    counter: i32,
    input: String,
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // Header
            let _ = efx!(ui, r#"
                <Column gap="8">
                  <Label size="20" bold="true">EFx sandbox</Label>
                  <Separator/>
                </Column>
            "#);

            // Buttons returning Response
            ui.horizontal(|ui| {
                let inc = efx!(ui, r#"<Button tooltip="Increment">+1</Button>"#);
                if inc.clicked() { self.counter += 1; }

                let dec = efx!(ui, r#"<Button tooltip="Decrement">-1</Button>"#);
                if dec.clicked() { self.counter -= 1; }
            });

            // Dynamic text
            let _ = efx!(ui, r#"<Label>Counter: {self.counter}</Label>"#);

            // Text input
            let _ = efx!(ui, r#"<TextField value="self.input" hint="type here…"/>"#);

            // Scroll + links + styled buttons
            let _ = efx!(ui, r#"
                <ScrollArea axis="vertical" max_height="160" always_show="true" id="demo-log">
                  <Column gap="6">
                    <Label monospace="true">You typed: {self.input.clone()}</Label>
                    <Row gap="8">
                      <Hyperlink url="https://efxui.com" tooltip="Project site"/>
                      <Hyperlink url="help:about" open_external="false">About</Hyperlink>
                    </Row>
                    <Separator/>
                    <Row gap="10" wrap="true">
                      <Button fill="#333333AA" rounding="8">A</Button>
                      <Button frame="false">B</Button>
                      <Button min_width="100" tooltip="Wide">Wide</Button>
                    </Row>
                  </Column>
                </ScrollArea>
            "#);
        });
    }
}
```

**Tips**

* Keep several example “scenes” as `&'static str` and switch them via a `ComboBox` to test different tag sets.
* Prefer **snake\_case** attributes (`max_height`, `always_show`, `stroke_width`, …). If a tag supports kebab-case aliases, the tag’s section will mention it.
* Colors are `#RRGGBB` or `#RRGGBBAA` (short `#RGB/#RGBA` is not supported yet).

**Why sandbox instead of doctests**

Doctests are great for syntax and error messages, but `egui` requires a proper render loop (`Context::run()`), which doctests don’t provide. The sandbox runs a real app, while examples in this documentation are marked `rust,ignore` to avoid execution.

---

For more information, see the sections below: **Supported Tags** and **Syntax Guide**.