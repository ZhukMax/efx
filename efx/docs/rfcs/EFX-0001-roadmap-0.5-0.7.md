# RFC: EFx 0.5–0.7 — attributes, new tags, components, panels, and Bevy

**Status:** Proposed (Draft 1)  
**Author:** Max (with contributions from ChatGPT)  
**Target versions:** 0.5 → 0.7  
**Compatibility:** Backward compatible; new capabilities are opt‑in.

---

## 1) Summary
EFx is a minimalist XML DSL on top of `egui` (via `eframe`, `bevy_egui`, or any other integrator) that speeds up layout of simple panels/menus/debug UIs. This RFC proposes a three‑step roadmap:

- **0.5 (attributes & core tags):** turn on attribute rendering (type‑safe), add semantic tags (`Heading`, `Hyperlink`, `Image`, `TextField`, `Grid`, basic windows/panels), improve diagnostics.
- **0.6 (components & events):** reusable components/slots and syntactic sugar for events (`onClick`, etc.), classes & styles, official `bevy_egui` examples.
- **0.7 (themes & layouts):** style sheets/themes, extended containers (`Table`, `Tabs`), final polish and perf.

EFx remains an immediate‑mode DSL for `egui` — not a retained framework.

---

## 2) Motivation & Goals
**Goals:**
1. Reduce boilerplate for common `egui` widgets/containers.
2. Provide an expressive, type‑checked attribute model.
3. Enable reuse (components + slots) without heavy abstractions.
4. Ship Tier‑1 support for `eframe` and `bevy_egui` (examples + CI builds).
5. Add lightweight theming/styling without turning EFx into a CSS engine.

**Non‑goals:**
- Do not turn EFx into a fully retained UI framework.
- Do not chase platform‑native look & feel beyond what `egui` offers.

---

## 3) User stories
- **Debug/tools:** quickly build debug panels and simple forms.
- **Windows/panels:** declaratively open `Window`/`CentralPanel`/`SidePanel`.
- **Reusability:** small building blocks with parameters and slots.
- **Styling:** centrally adjust fonts/spacing/colors.

---

## 4) Design overview
EFx still compiles into regular `egui` calls. New bits:
- **Attributes:** literals (`size="14"`), numbers (`spacing=8`), booleans (presence = `true`), and `attr={expr}` (including references like `&mut` for bidirectional scenarios).
- **Events:** sugar like `onClick={|| ...}` compiled into `Response` checks.
- **New tags:** see mapping table below. Windows/panels use `ui.ctx()`.
- **Components & slots (0.6):** `#[efx_component]` over plain functions.
- **Styles & classes:** minimal class system + Rust style structs with push/pop scope.

---

## 5) Attribute syntax
### 5.1. General rules
- `attr="literal"` → parsed into string/number/enum.
- `attr={expr}` → arbitrary Rust expression; types checked by the compiler.
- Boolean flags: `<Button disabled/>` → `disabled=true`.
- Defaults follow `egui` defaults.

### 5.2. Event attributes (0.6)
```xml
<Button onClick={|| do_action()}>
  Save
</Button>
```
Equivalent to:
```rust
let resp = ui.button("Save");
if resp.clicked() { do_action(); }
```
A root `<Button>` still returns `egui::Response`.

### 5.3. Text input
```xml
<TextField value={&mut state.name} hint="Your name"/>
```
Generates `egui::TextEdit::singleline(&mut state.name).hint_text("...")` when present.

---

## 6) Tag map to `egui`
| Tag | Purpose | Generated (approx.) |
|---|---|---|
| `Label` | Text | `ui.label(RichText::new(text).<style>)` |
| `Heading` | Header | `ui.heading(text)` (+ styles) |
| `Hyperlink` | Link | `ui.hyperlink(url)` / `ui.hyperlink_to(label, url)` |
| `Button` | Button | `ui.button(label)` → `Response` |
| `Separator` | Separator | `ui.separator()` |
| `Row` | Horizontal | `ui.horizontal(|ui| { ... })` (+ `spacing`) |
| `Column` | Vertical | `ui.vertical(|ui| { ... })` (+ `spacing`) |
| `Grid` | Grid | `egui::Grid::new(id).show(ui, |ui| { ... })` |
| `TextField` | Single-line input | `TextEdit::singleline(&mut value)` + attrs |
| `Image` | Image | `ui.image(texture_id, size)` |
| `Tabs`* | Tabs | via `egui_extras` or minimal switcher |
| `Table`* | Table | `egui_extras::TableBuilder` |
| `Panel` | Generic container | see below |
| `Window` | Window | `Window::new(title).show(ui.ctx(), |ui| {...})` |
| `CentralPanel` | Central area | `CentralPanel::default().show(ui.ctx(), |ui| {...})` |
| `SidePanel` | Side area | `SidePanel::left(id)/right(id).show(ui.ctx(), |ui| {...})` |

(*) behind the `extras` feature.

### 6.1. Windows & panels
We rely on `ui.ctx()` to access `egui::Context` inside the codegen, keeping a single macro `efx!(ui, ...)`.

---

## 7) Styles, classes, and style sheets
Minimalistic — no CSS.  
- `class="h1 muted"` applies predefined presets.  
- `style={EfxStyle { font_size: 14.0, ..Default::default() }}` for inline tweaks.  
- Push/pop style scope (restore on `Drop`) to avoid global side effects.

Style sheets (0.7):
```rust
static THEME: EfxTheme = efx_theme! {
  .h1 { font: Heading, size: 22.0 }
  .muted { color: Gray }
};
```

---

## 8) Components & slots (0.6)
Attribute macro over a plain function:
```rust
#[efx_component]
fn Toolbar(ui: &mut egui::Ui, title: &str, #[efx_slot] actions: impl FnOnce(&mut egui::Ui)) {
    efx!(ui, r#"
        <Row spacing=8>
            <Heading>{title}</Heading>
            { actions(ui) }
        </Row>
    "#);
}
```
Use:
```rust
Toolbar(ui, "Files", |ui| efx!(ui, "<Button>New</Button><Button>Open</Button>"));
```

---

## 9) Supported runtimes (Support tiers)
- **Tier‑1 (officially supported in 0.5: examples + CI build):**
  1. `eframe` (native + wasm).
  2. `bevy_egui` (native).
  3. raw `winit+wgpu` (via `egui-winit` + `egui-wgpu`) — demonstrates EFx is framework‑agnostic.
- **Tier‑2 (already compatible, examples later / community support):**
  - `egui-miniquad` (for `macroquad/miniquad` overlays),
  - `egui_sdl2_*` (SDL2 backends),
  - `egui_glow` / `tao` (lower‑level backends).

---

## 10) Changes in `efx-core`
- **AST:** attributes already exist; add typed codegen (literal → int/float/bool/str/ident), support `{expr}` as token stream.
- **Codegen:**
  - Map attributes to `egui` calls (e.g., `Row spacing=..` → tweak `ui.spacing_mut().item_spacing.x` locally).
  - Style scope push/pop.
  - Event sugar via conditional checks on `Response`.
- **Diagnostics:** friendly `compile_error!` with spans and suggestions.

---

## 11) Compatibility & versions
- 0.5: **no breaking changes**. New tags/attributes are optional.
- 0.6: components/slots — additive.
- 0.7: theme/layout utilities — additive (may introduce warnings/lints only).

Cargo features:
```toml
[features]
extras = ["egui_extras"]
bevy   = []
```

---

## 12) Plan
### 0.5 — Attributes & core tags (2–3 weeks)
1. Attribute rendering: literals + `{expr}`; numbers/bools/strings.
2. `Heading`, `Hyperlink`, `TextField`, `Image`, `Grid`.
3. `Window`/`CentralPanel`/`SidePanel` via `ui.ctx()`.
4. Diagnostics/tests (compile‑fail + snapshot codegen tests).
5. Documentation (README: Supported runtimes, quickstarts for eframe/bevy/winit+wgpu).
6. Examples: `examples/eframe_demo.rs`, `examples/bevy_overlay.rs`, `examples/winit_wgpu_min.rs`.
7. CI: GitHub Actions matrix builds — eframe (native+wasm), bevy (native), winit+wgpu (native).

### 0.6 — Components/events/classes (3–4 weeks)
1. `#[efx_component]` + `#[efx_slot]`.
2. Events: `onClick`, `onHover` sugar.
3. Classes (`class="..."`) + preset pack.
4. `Table/Tabs` behind `extras`.
5. Bevy examples (menus, toolbars, tabs).

### 0.7 — Themes, layouts, polish (3–4 weeks)
1. `efx_theme!` and style sheets.
2. Layout utilities (margins, alignment) as attributes on containers.
3. Perf pass (allocations/clones).
4. Errors/hints, docs polish.

---

## 13) Performance
- Avoid extra `String` allocations.
- Cache common `RichText`/`FontId` in presets.
- Use scoped style changes to prevent global churn.

---

## 14) Test plan
- **compile‑fail:** nested elements inside `Label/Button`, unknown tags/attrs, wrong attr types.
- **snapshot codegen:** expected Rust for typical snippets.
- **integration:** build examples on CI; no GUI runtime required on CI.

---

## 15) Documentation
- README: “Works with any `egui` runtime: `eframe`, `bevy_egui`, raw `winit+wgpu`”.
- Cookbook: forms (`TextField`), windows/panels, grids, links, images.
- Bevy guide: imports, version pitfalls (re‑exported `egui`), best practices.

---

## 16) Open questions
- Keep `Tabs/Table` in core or behind `extras`?
- Event sugar beyond `onClick` (`onChange` for `TextField`) — how far to go?
- Separate macro for context (`efx_ctx!`) — current decision: **no**, keep `ui.ctx()`.

---

## 17) Examples
### 17.1. eframe/bevy window
```rust
efx!(ui, r#"
<Window title="Demo">
  <Column spacing=8>
    <Heading>EFx 0.5</Heading>
    <Row spacing=6>
      <Label>Welcome,</Label>
      <Hyperlink url="https://efxui.com">efxui.com</Hyperlink>
    </Row>
    <Separator/>
    <TextField value={&mut state.name} hint="Your name"/>
    { if efx!(ui, "<Button>Save</Button>").clicked() { save(); } }
  </Column>
</Window>
"#);
```

### 17.2. Event sugar (0.6)
```rust
efx!(ui, r#"<Button onClick={|| save()}>Save</Button>"#);
```

### 17.3. Component with a slot (0.6)
```rust
Toolbar(ui, "Files", |ui| efx!(ui, "<Button>New</Button><Button>Open</Button>"));
```

---

## 18) Risks
- More syntax → more complex macro/errors — mitigate with diagnostics/tests.
- Window/panel via `ui.ctx()` — careful with style scope boundaries.
- `extras` deps behind a feature to keep dependency tree slim.

---

## 19) Acceptance criteria (0.5)
- Attributes: literals/booleans/`{expr}` compile and behave as expected.
- New tags render correctly; compile‑fail tests cover invalid nesting/types.
- CI builds examples: eframe (native+wasm), bevy (native), winit+wgpu (native).

---

## 20) Call for contributors
Looking for help with:
- Bevy example(s) and raw `winit+wgpu` example;
- Attribute codegen & compile‑fail tests;
- `TextField` implementation details and docs.

---

**Disclaimer:** This plan may change based on feedback. Priorities can shift.  
**License:** This RFC is part of EFx documentation and follows the project license.
