<div align="center">
  <img src="https://raw.githubusercontent.com/ZhukMax/efx/main/efx/docs/efx_cover.png" alt="EFX — Rust templating for egui/eframe">

  # EFx
  
  **🧑‍💻 Easy as HTML, 🚀 Fast as C, 🔐 Safe cause Rust**
  
  *Write `egui` UIs as easily as HTML. EFx is a macro-based XML template engine that turns tiny UI snippets into native egui calls with zero overhead.*

  [![Build](https://github.com/ZhukMax/efx/actions/workflows/examples-build.yml/badge.svg)](https://github.com/ZhukMax/efx/actions)
  [![Tests](https://github.com/ZhukMax/efx/actions/workflows/test.yml/badge.svg)](https://github.com/ZhukMax/efx/actions)
  [![Clippy](https://github.com/ZhukMax/efx/actions/workflows/clippy.yml/badge.svg)](https://github.com/ZhukMax/efx/actions)
  [![Rustfmt](https://github.com/ZhukMax/efx/actions/workflows/rustfmt.yml/badge.svg)](https://github.com/ZhukMax/efx/actions)
  <br>
  [![Crates.io](https://img.shields.io/crates/v/efx.svg?color=green)](https://crates.io/crates/efx)
  ![Crates.io Version](https://img.shields.io/crates/v/efx-core?label=efx-core)
  [![MSRV](https://img.shields.io/badge/rustc-1.75%2B-blue.svg?logo=rust)](#)
  [![Docs.rs](https://docs.rs/efx/badge.svg)](https://docs.rs/efx)
  [![License](https://img.shields.io/crates/l/efx)](https://github.com/ZhukMax/efx/blob/main/LICENSE)
</div>

---

### ⚡ See it in action

<div align="center">
  <img src="https://via.placeholder.com/400x250.png?text=GIF+1:+Writing+efx!+macro+code" alt="Coding UI in EFx" width="45%">
  &nbsp;
  <img src="https://via.placeholder.com/400x250.png?text=GIF+2:+egui+rendering+in+real-time" alt="Resulting egui application" width="45%">
</div>

**EFx** provides a procedural macro `efx!` for writing XML-like UI snippets within [egui](https://github.com/emilk/egui)-based frameworks (`eframe`, `bevy_egui`, `egui-winit`, etc.). 

**Perfect for:** 🖥️ Desktop apps | 🎮 Game HUDs (`bevy_egui`) | 🌐 WebAssembly apps | 🧰 Internal dev tools | 🔬 Scientific dashboards

---

### ⏱️ Quick Start (< 5 mins)

Get a working app running instantly. We'll use `eframe` for this example.

**1. Add dependencies**
```toml
[dependencies]
efx    = "0.7"
eframe = "0.33"

```

**2. Copy & Run**

```rust
use eframe::egui;
use efx::efx;

fn main() -> eframe::Result<()> {
    eframe::run_simple_native("EFx Quickstart", Default::default(), |ctx, _frame| {
        egui::CentralPanel::default().show(ctx, |ui| {
            let name = "Rustacean";
            // Write UI using XML-like syntax. Expressions inside `{}` are evaluated!
            efx!(ui, r#"
                <Column>
                    <Label>Hello, {name}!</Label>
                    <Separator/>
                    <Button>Click me</Button>
                </Column>
            "#);
        });
    })
}

```

---

### 📖 Documentation & Guides

Explore the full capabilities of EFx on [docs.rs](https://docs.rs/efx) or dive into our local guides (available in English and French 🇫🇷):

* [Introduction](https://www.google.com/search?q=efx/docs/intro.md) ([🇫🇷 fr](https://www.google.com/search?q=efx/docs/fr/intro.md))
* [Supported Tags](https://www.google.com/search?q=efx/docs/tags.md) ([🇫🇷 fr](https://www.google.com/search?q=efx/docs/fr/tags.md))
* [Comprehensive Guide](https://www.google.com/search?q=efx/docs/guide.md) ([🇫🇷 fr](https://www.google.com/search?q=efx/docs/fr/guide.md))

---

### 🧩 Key Features

#### Expression Interpolation `{...}`

Inside tag content, you can insert any Rust expression that implements `Display`. Need to print `{` or `}`, just use double braces like in `format!`.

```rust
let a = 2; let b = 3;
efx!(ui, r#"<Label>Sum: {a + b} (Literals: {{ and }})</Label>"#);

```

#### Beautiful Compile-time & Runtime Diagnostics

At compile time, the macro parses your snippet. At runtime, if something is wrong (e.g., *Unknown tag* or *Mismatched open/close tags*), EFx won't crash your app; it safely renders a readable diagnostic message directly via `ui.Label(...)`.

---

### ⚙️ Supported `egui` runtimes

EFx renders into any runtime that provides `&mut egui::Ui`.

* **Tier-1:** `eframe` (native + wasm), `bevy_egui` (native), raw `winit+wgpu` (via `egui-winit` + `egui-wgpu`).
* **Tier-2 (Community support):** `egui-miniquad`, `egui_sdl2_*`, `egui_glow` / `tao`.

*Looking for setup examples for Bevy or Raw winit? Check the `examples/` directory in this repository.*

---

### 🤝 Contributing & Roadmap

EFx is designed as a minimalist XML DSL. We welcome community contributions!

* **RFC index:** [RFC/README.md](https://www.google.com/search?q=efx/docs/rfcs/README.md)
* **Guidelines:** Read our [Contributing Guide](https://www.google.com/search?q=./CONTRIBUTING.md) for CI requirements, PR rules, and coding conventions.
* **Changelog:** [Changelog.md](https://www.google.com/search?q=efx/Changelog.md)
* **Licence:** [MIT License](https://www.google.com/search?q=efx/LICENSE)
