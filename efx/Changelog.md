## Changelog

#### 0.7 (conceivably)
- CI: GitHub Actions matrix builds — eframe (native+wasm), bevy (native), winit+wgpu (native)
- Events: `onClick`, `onHover` sugar
- `#[efx_component]` + `#[efx_slot]`
- Fix: `efx_ctx!(ctx, "...")` failed to parse its arguments ("expected `,`")
- Fix: generated code no longer triggers warnings in user crates — clippy `unused_unit`
  (panels/windows/`efx_ctx!`), deprecated `egui::Rounding` (now `CornerRadius` for
  `Button`/`Image` `rounding`), unused `Response` of non-root `<Button>`/`<Hyperlink>`

#### 0.6
- New Tags: Heading, Image, Grid, Table, Tabs, DataTable
- Added Panel Tags: Window, SidePanel, TopPanel, BottomPanel, Panel
- Sandbox
- More examples & tests
- Updated and added translation (🇫🇷) docs

#### 0.5
- Attribute rendering (efx-core)
- Label, Button, Row, Column, Separator attributes
- Updated documentation (attributes, quickstarts for eframe/bevy/winit+wgpu)
- New Tags: ~~Heading~~, Hyperlink, TextField, ~~Image~~, ~~Grid~~
- Added Panel Tags: ~~Window~~, CentralPanel, ~~SidePanel~~, ScrollArea
- Added examples & tests

#### 0.4
- Basic Nodes Parser (efx-core) refactoring
- Added tag attributes

#### 0.3
- Divided the library into two crates
- Created Basic Nodes Parser
- Rewrote the macro to use the parser
- Tags: Label/Button/Row/Column/Separator
- Added tests

#### 0.2
- Refactoring expressions
- Added Button tag

#### 0.1
- Init engine
- Added rsx macros
- Added Label tag
