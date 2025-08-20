
# EFx
[![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg?style=flat-square)](https://makeapullrequest.com)
[![Crates.io](https://img.shields.io/crates/v/efx.svg)](https://crates.io/crates/efx)
[![Docs.rs](https://docs.rs/efx/badge.svg)](https://docs.rs/efx)
[![License](https://img.shields.io/crates/l/efx)](https://github.com/ZhukMax/efx/blob/main/LICENSE)

**EFx** — Rust 🦀 XML Template Engine for [eframe](https://github.com/emilk/egui/tree/master/crates/eframe) / [egui](https://github.com/emilk/egui) framework.
`efx!` is a proc-macro for writing tiny XML-like UI snippets in `eframe/egui`. It converts short tags into `egui` calls.

Current scope: **no attributes**. You can embed arbitrary Rust expressions inside braces (`{...}`).

---

### Install & import

Requires `eframe/egui` (the project currently uses `eframe 0.32`). Add to `Cargo.toml`:
```toml
[dependencies]
efx = "0.4"
eframe = "0.32"
```

Inside this repo just import the macro:
```rust
use efx::efx; // the macro
```

---

### Quick start

#### `<Label>…</Label>`

Renders a text Label. Returns `()` (equivalent to `ui.Label(...)`).

```rust
efx!(ui, r#"<Label>Hello, world</Label>"#);

// With interpolation:
let status = "ready";
efx!(ui, r#"<Label>Status: {status}</Label>"#);
```

#### `<Button>…</Button>`

Renders a button. Returns `egui::Response` (so you can check clicks).

```rust
if efx!(ui, r#"<Button>Run</Button>"#).clicked() {
    // handle click
}
```

> Note: tag names are **case-sensitive**. `Label` is lowercase; `Button` starts with a capital B.

---

### Expression interpolation `{...}`

Inside tag content you can insert any Rust expression that implements `Display`:

```rust
let a = 2;
let b = 3;
efx!(ui, r#"<Label>Sum: {a + b}</Label>"#);
```

#### Escaping braces

To print `{` or `}`, use double braces (same as `format!`):

```rust
efx!(ui, r#"<Label>Literals: {{ and }}</Label>"#);
```

---

### Errors & diagnostics

At compile time the macro parses your snippet; at runtime it shows readable diagnostics via `ui.Label(...)` when input is invalid:

* **Unknown tag**
  Output: `Unknown tag: <TagName>`

* **Mismatched open/close tags**
  Output: `Mismatched tags: <open> and </close>`

* **Interpolation count mismatch**
  Happens if the parser found e.g. two `{expr}` parts but after processing the text there’s a different number of `{}` placeholders. Make the counts match.

---

### Current limitations

* **Best with one-line snippets.** Line breaks inside content may not parse.
* **Case-sensitive tag names.**
* Interpolated expressions must implement `Display`.

---

### Todo ideas

* Icons collections
* Styles and style sheets
* Layouts
* Template syntax highlighting in code editors
* Multi-line code snippets
* Event Handling Attributes
* Fonts and text style
* Documentation on docs.rs

---

## Recipes

### Right-aligned dynamic status

```rust
ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
    let status = arbitrage_ui.arbitrage_status.to_string();
    efx!(ui, r#"<Label>Status: {status}</Label>"#);
});
```

### Button with click handling

```rust
if efx!(ui, r#"<Button>Refresh quotes</Button>"#).clicked() {
    // trigger refresh
}
```

### Braces + expression

```rust
let text = "test text";
efx!(ui, r#"<Label>Print {{ {text} }}</Label>"#);
```

---

### Tag reference

* `Label`

  **Syntax:** `<Label>Text with {expressions}</Label>`

  **Equivalent:** `ui.label(RichText::new(text))`

  **Returns:** `()`


* `Button`

  **Syntax:** `<Button>Text with {expressions}</Button>`

  **Equivalent:** `ui.button(text)`

  **Returns:** `egui::Response`

---

### Changelog
See in file [Changelog.md](Changelog.md)

### Licence
The MIT License. Please see [License File](LICENSE) for more information.
