# Supported Tags (v0.4+)

> Starting with 0.5 some tags support attributes.
> Unknown attributes result in `compile_error!`.

### `Hyperlink`
Clickable link widget. Generates `ui.hyperlink(url)` or `ui.hyperlink_to(label, url)`.

**Attributes**

- `url="..."` — destination address (string, required).
- `open_external="true|false"` — open link in system browser (default true).
- `color="name|#RRGGBB[AA]"` — link text color.
- `underline="true|false"` — underline link text (default true).
- `tooltip="text"` — hover tooltip.

Cross-platform usage

- **Web:** renders as standard `<a>` link.
- **Desktop (eframe, bevy_egui):** opens system browser via `ui.hyperlink(...)`.
- **Game/tool overlays:** convenient way to link to docs, repos, or help.
- **Offline apps:** with custom URL schemes (e.g. `help://topic`) may open in-app help instead of browser.

```rust
use efx_core::doc_prelude::*;
use efx::*;

efx!(Ui::default(), r##"
    <Column>
        <Hyperlink url="https://efxui.com" color="#66CCFF" tooltip="Project site"/>
        <Hyperlink url="help://about" open_external="false">About</Hyperlink>
    </Column>
"##);
```

---

### `TextField`
Single-line or multi-line text input. Generates `egui::TextEdit` and inserts it via `ui.add(...)`. Must be self-closing (no children).

**Attributes**

- `value="<expr>"` — **required**. Rust lvalue expression of type `String`, e.g. `state.name`. The generator takes `&mut (<expr>)` automatically.
- `hint="text"` — placeholder text shown when empty.
- `password="true|false"` — mask characters (applies to single-line; ignored with `multiline="true"`).
- `width="N"` — desired width in points (f32).
- `multiline="true|false"` — multi-line editor (`TextEdit::multiline`).

```rust
use efx_core::doc_prelude::*;
use efx::*;

#[derive(Default)]
struct State { name: String }

let mut state = State::default();

// Single-line with placeholder and width
efx!(Ui::default(), r#"<TextField value="state.name" hint="Your name" width="220"/>"#);

// Password field (single-line)
efx!(Ui::default(), r#"<TextField value="state.name" password="true"/>"#);

// Multiline editor
efx!(Ui::default(), r#"<TextField value="state.name" multiline="true" width="320"/>"#);
```

---

### `<Resize>`

A resizable container that lets the user drag a handle to change the size of its content.  
Useful for side views, inspectors, consoles, etc., when a full docked panel is too heavy.

**Children:** rendered inside the resizable area.

**Required attributes**
- `id="string"` — egui `Id` salt to persist the size across frames.

**Behavior**
- `resizable="true|false"` — enable/disable user resizing (default: `true` in egui).

**Sizing**
- `default-width="number"`, `default-height="number"` — initial size.
- `min-width="number"`, `min-height="number"` — lower bounds.
- `max-width="number"`, `max-height="number"` — upper bounds.

> Each dimension is optional. If only one dimension is provided, the other falls back to `0.0` (for min/default) or `∞` (for max).

**Example**
```xml
<CentralPanel fill="#101014">
  <Resize id="console" default-height="200" min-height="120">
    <ScrollArea axis="vertical" max_height="9999" id="console-scroll">
      <Column gap="6" padding="6">
        <Label monospace="true">[12:00:01] Ready.</Label>
        <Label monospace="true">[12:00:02] Build succeeded.</Label>
      </Column>
    </ScrollArea>
  </Resize>
</CentralPanel>
```

---

### `Heading`

Text heading. Generates `ui.heading(text)` with optional style overrides.

**Attributes**

- `level="1..6"` — heading level (integer).  
  *Default:* `1`. Maps to predefined `egui` text styles.
- `size="N"` — overrides the font size (f32).
- `color="name|#RRGGBB[AA]"` — text color.
- `tooltip="text"` — hover tooltip.

```rust
use efx_core::doc_prelude::*;
use efx::*;

efx!(Ui::default(), r##"
  <Column gap="8">
    <Heading level="1">Main title</Heading>
    <Heading level="2" color="#66CCFF">Section</Heading>
    <Heading level="3" size="14" tooltip="Subheading">Small note</Heading>
  </Column>
"##);
```
The level attribute controls the base style (h1–h6), while size and color can further adjust the appearance.

---

## `<Image>`

Display a bitmap/texture in the UI. Works both with a preloaded texture handle/id (recommended for desktop) and with a URI-like source (useful on web or when you have your own loader).

### Syntax

```xml
<Image
  texture="self.logo_tex_id"
  width="128"
  height="128"
  rounding="6"
  clickable="true"
  tooltip="Click to open"
/>
```

or

```xml
<Image
  src="assets/logo.png"
  max-width="256"
  maintain-aspect="true"
  id="logo-1"
/>
```

### Attributes

| Name              | Type                                                                         | Default | Description                                                           |
|-------------------|------------------------------------------------------------------------------|---------|-----------------------------------------------------------------------|
| `texture`         | **expr** (`egui::TextureId`, `&egui::TextureHandle`, or `egui::ImageSource`) | —       | Source texture/handle. Mutually exclusive with `src`.                 |
| `src`             | string (URI/path)                                                            | —       | Image URI/path. Mutually exclusive with `texture`.                    |
| `width`           | f32                                                                          | —       | Target width. If both `width` and `height` are set, uses exact size.  |
| `height`          | f32                                                                          | —       | Target height. If both `width` and `height` are set, uses exact size. |
| `max-width`       | f32                                                                          | `∞`     | Max width (used if exact size isn’t specified).                       |
| `max-height`      | f32                                                                          | `∞`     | Max height (used if exact size isn’t specified).                      |
| `maintain-aspect` | bool                                                                         | `false` | Keep original aspect ratio when fitting.                              |
| `rounding`        | u8                                                                           | —       | Uniform corner radius.                                                |
| `tint`            | color                                                                        | —       | Multiplies image color (e.g. `#FFFFFF80` for 50% fade).               |
| `bg-fill`         | color                                                                        | —       | Background fill behind the image rect.                                |
| `id`              | string                                                                       | —       | Stable id seed (`id_source`) for consistent layout/caching.           |
| `clickable`       | bool                                                                         | `false` | If `true`, image responds to clicks (`Sense::click`).                 |
| `tooltip`         | string                                                                       | —       | Hover text shown on the image.                                        |

> Either `texture` **or** `src` must be provided (not both). `<Image>` does not accept children.

### Behavior & sizing rules

* **Exact size**: if both `width` and `height` are set → the image is fit to that exact `vec2(width, height)`.
* **Max size**: otherwise, a max box is computed from `max-width`/`max-height` (falling back to `width`/`height` if only one side is provided).
* **Aspect**: `maintain-aspect="true"` keeps the original ratio when fitting.
* **Interactivity**: with `clickable="true"` the tag returns a normal `Response` you can query (`.clicked()`, etc.). Tooltips are applied via `on_hover_text`.

### Examples

**URI/path source (web / custom loader):**

```xml
<Image src="assets/logo.png" max-width="200" maintain-aspect="true" id="logo-main"/>
```

**Tint + background fill:**

```xml
<Image texture="self.icon_tex" tint="#FFFFFFCC" bg-fill="#00000022" rounding="4"/>
```

### Notes

* `rounding` is uniform; per-corner radii can be added later if needed.
* `id` helps egui keep the same widget identity across frames when the source is otherwise dynamic.
* On desktop, prefer `texture` with a previously allocated `TextureId`/`TextureHandle` for performance and control. On web, `src` can be convenient alongside your asset loader.

---

## `<Tabs>` and `<Tab>`

Tabbed container. Controlled via a string-like `active` binding that holds the id of the currently selected tab.

**Syntax**
```xml
<Tabs active="self.active_tab" gap="8">
  <Tab id="home"  title="Home">
    <Label>Welcome home!</Label>
  </Tab>
  <Tab id="logs"  title="Logs">
    <ScrollArea axis="vertical" max-height="180">
      <Label monospace="true">[12:00:01] Ready.</Label>
    </ScrollArea>
  </Tab>
  <Tab id="about" title="About" enabled="false">
    <Label>This tab is disabled</Label>
  </Tab>
</Tabs>
```

**Attributes – `<Tabs>`**

| Name     | Type | Default                       | Description                                                                    |
|----------|------|-------------------------------|--------------------------------------------------------------------------------|
| `active` | expr | required                      | String/\&str expression with the id of the active tab (`"home"`, `"logs"`, …). |
| `gap`    | f32  | `ui.spacing().item_spacing.x` | Space between tab headers (px).                                                |

**Attributes – `<Tab>`**

| Name      | Type   | Default | Description                                                      |
|-----------|--------|---------|------------------------------------------------------------------|
| `id`      | string | —       | Unique tab id. Used for matching and as default title.           |
| `title`   | string | `id`    | Header text.                                                     |
| `enabled` | bool   | `true`  | When `false`, the tab header is disabled and cannot be selected. |

**Behavior**

- Clicking a tab header updates active to that tab’s `id`. You can read `active` from your state to switch content.

- `<Tab>` is only allowed as a child of `<Tabs>` and may contain any regular EFx content in its body.

- Returns `()` (container).

---

## `<Table>`, `<Tr>`, `<Td>`

Lightweight tables built on top of `egui::Grid`. Suitable for most static layouts. For resizable/feature-rich tables we plan a `<DataTable>` based on `egui_extras::TableBuilder` (future work).

**Syntax**
```xml
<Table columns="3" striped="true" spacing-x="8" spacing-y="4" cell-align="left" cell-padding="4" id="users">
  <Tr>
    <Td><Label bold="true">Name</Label></Td>
    <Td><Label bold="true">Email</Label></Td>
    <Td><Label bold="true">Role</Label></Td>
  </Tr>

  <Tr>
    <Td><Label>Alice</Label></Td>
    <Td><Label>alice@example.com</Label></Td>
    <Td><Label>Admin</Label></Td>
  </Tr>
</Table>
```

**Attributes – `<Table>`**

| Name           | Type   | Default  | Description                                      |
|----------------|--------|----------|--------------------------------------------------|
| `columns`      | int    | required | Number of columns (must be ≥ 1).                 |
| `striped`      | bool   | `false`  | Alternate row background.                        |
| `spacing-x`    | f32    | `8`      | Horizontal spacing between columns (px).         |
| `spacing-y`    | f32    | `4`      | Vertical spacing between rows (px).              |
| `cell-padding` | f32    | `0`      | Padding inside each cell (px).                   |
| `cell-align`   | enum   | `left`   | Horizontal alignment: `left`, `center`, `right`. |
| `id`           | string | —        | Stable id for grid instance.                     |

**Rules**

- Only `<Tr>` children are allowed inside `<Table>`.
- `<Tr>` may only contain `<Td>` elements.
- `colspan` / `rowspan` are not supported in this version (a compile error is emitted if used).

Returns `()` (container). Content inside `<Td>` can be any EFx widgets.

**Notes**

- This implementation uses `egui::Grid` to keep dependencies minimal and performance high.
- If you need column resizing, multi-row headers, scrolling inside the table, etc., we’ll introduce a `<DataTable>` tag based on `egui_extras::TableBuilder` behind an optional feature flag in a follow-up.

---

## `<DataTable>` (requires `features = ["extras"]`)

Feature-rich table built on top of `egui_extras::TableBuilder`.

**Columns**
Declare columns via `<Columns>` and `<Column>`:
- `mode="auto|initial|exact|remainder"`
- `width="..."` (required for `initial` / `exact`)
- `resizable="true|false"` (defaults to table’s `default-resizable`)
- `clip="true|false"`

**Header**
Single header row specified via `<Header>` with `<Td>` children (one per column).

**Body**
Body consists of `<Tr>` rows with `<Td>` cells (missing cells are filled with blanks). Per-row height can be set via `height` on `<Tr>`.

**Example**
```xml
<DataTable id="users" striped="true" resizable="true"
           default-resizable="true" header-height="24" row-height="22"
           cell-align="left" cell-padding="4">
  <Columns>
    <Column mode="initial" width="160" resizable="true"/>
    <Column mode="auto"/>
    <Column mode="remainder" resizable="true" clip="true"/>
  </Columns>

  <Header>
    <Td><Label bold="true">Name</Label></Td>
    <Td><Label bold="true">Email</Label></Td>
    <Td><Label bold="true">Role</Label></Td>
  </Header>

  <Tr>
    <Td><Label>Alice</Label></Td>
    <Td><Label>alice@example.com</Label></Td>
    <Td><Label>Admin</Label></Td>
  </Tr>

  <Tr>
    <Td><Label>Bob</Label></Td>
    <Td><Label>bob@example.com</Label></Td>
    <Td><Label>User</Label></Td>
  </Tr>
</DataTable>
```

**Attributes – `<DataTable>`**

| Name                | Type   | Default | Description                                                      |
|---------------------|--------|---------|------------------------------------------------------------------|
| `id`                | string | —       | Stable id (`push_id`) wrapping the whole table.                  |
| `striped`           | bool   | `false` | Alternate row background.                                        |
| `resizable`         | bool   | `false` | Enables column resizing globally (can be overridden per column). |
| `default-resizable` | bool   | —       | Default `resizable` for each `<Column>` if not set.              |
| `header-height`     | f32    | `22`    | Header row height (px).                                          |
| `row-height`        | f32    | `22`    | Default body row height (px).                                    |
| `cell-padding`      | f32    | `0`     | Inner padding per cell, in px (applied on both sides).           |
| `cell-align`        | enum   | `left`  | Horizontal alignment in cells: `left`, `center`, `right`.        |

**Children order**
`<Columns>` → optional `<Header>` → `<Tr>*`.

Multiple `<Header>` are not allowed; `<Header>` must have exactly one row, with exactly one `<Td>` per column.

**Notes**
- `colspan` / `rowspan` are not supported in this version (compile error if used).
- Returns `()` (container).

---

