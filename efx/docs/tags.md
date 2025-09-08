## Supported Tags (v0.4+)

> Starting with 0.5 some tags support attributes.
> Unknown attributes result in `compile_error!`.

### `Column`
Vertical container. Generates `ui.vertical(|ui| { ... })`.

**Attributes**
- `align="left|center|right"` — horizontal alignment of children.
- `gap="N"` — vertical spacing between children (f32).
- `padding="N"` — extra top/bottom padding (f32).

```rust
use efx_core::doc_prelude::*;
use efx::*;

efx!(Ui::default(), r#"<Column gap="10" padding="6" align="center">
  <Label>Title</Label>
  <Label size="12">Subtitle</Label>
</Column>"#);
```

### `Row`
Horizontal container. Generates `ui.horizontal(|ui| { ... })`.

**Attributes**

- `align="top|center|bottom"` — vertical alignment of children.
- `gap="N"` — horizontal spacing between children (f32).
- `wrap="true|false"` — wrap children to next line if overflow.
- `padding="N"` — extra left/right padding (f32).

```rust
use efx_core::doc_prelude::*;
use efx::*;

efx!(Ui::default(), r#"<Row gap="8" padding="4" align="center"><Label>A</Label><Label>B</Label></Row>"#);

efx!(Ui::default(), r#"<Row wrap="true"><Label>Item1</Label><Label>Item2</Label><Label>Item3</Label></Row>"#);

```

---

## `<Panel>`

A lightweight frame container to group content with background, padding and stroke. Unlike `Top/Bottom/Side/CentralPanel`, this tag is not a context-root and can be placed anywhere in the UI.

**Syntax**
```xml
<Panel fill="#15151A" padding="8" stroke-width="1" stroke-color="#262A33" id="card-1">
  <Column gap="6">
    <Label bold="true">Card title</Label>
    <Label size="12" color="#AAAAAA">Some description</Label>
  </Column>
</Panel>
```

### Attributes

| Name                                        | Type   | Description                                              |
|---------------------------------------------|--------|----------------------------------------------------------|
| `frame`                                     | bool   | `false` → `Frame::none()`, otherwise `Frame::default()`. |
| `fill`                                      | color  | Background color.                                        |
| `stroke-width`                              | f32    | Border width.                                            |
| `stroke-color`                              | color  | Border color.                                            |
| `padding` / `padding-left/right/top/bottom` | f32    | Inner margin.                                            |
| `margin` / `margin-left/right/top/bottom`   | f32    | Outer margin.                                            |
| `id`                                        | string | Stable `push_id` seed for the panel.                     |

> Returns () (container). Children are rendered inside the frame.

---

### `CentralPanel`

Main content area that fills all remaining space. Wraps children in `egui::CentralPanel` and applies an optional `Frame`.

**Attributes**

- `frame="true|false"` — use default frame (`true`, default) or `none` (`false`).
- `fill="name|#RRGGBB[AA]"` — background fill color.
- `stroke-width="N"` — frame stroke width (f32).
- `stroke-color="name|#RRGGBB[AA]"` — frame stroke color.
- `padding="N"` — inner margin on all sides (f32).
- `padding-left|padding-right|padding-top|padding-bottom="N"` — per-side inner margin.
- `margin="N"` — outer margin on all sides (f32).
- `margin-left|margin-right|margin-top|margin-bottom="N"` — per-side outer margin.

```rust,no_run
use efx_core::doc_prelude::*;
use efx::*;

efx!(Ui::default(), r##"
  <CentralPanel fill="#101014" padding="12" stroke-width="1" stroke-color="#222638">
    <Column gap="8">
      <Label size="18" bold="true">Dashboard</Label>
      <Separator space="6"/>
      <Row gap="12">
        <Label>Welcome!</Label>
        <Hyperlink url="https://efxui.com">Docs</Hyperlink>
      </Row>
    </Column>
  </CentralPanel>
"##);
```

---

### `<SidePanel>`

Docked panel attached to the left or right edge of the window.  
Typically used for navigation, tool palettes, or context inspectors.

**Children:** rendered inside the panel.

**Required attributes**
- `side="left|right"` — which edge to dock to.
- `id="string"` — egui `Id` salt to keep layout state (width, resize state).

**Frame & styling**
- `frame="true|false"` — enable/disable the default frame (default: `true`).
- `fill="#RRGGBB[AA]"` — background color.
- `stroke-width="number"` — border width, in points.
- `stroke-color="#RRGGBB[AA]"` — border color.
- `padding`, `padding-left|right|top|bottom` — inner margin (content padding).
- `margin`, `margin-left|right|top|bottom` — outer margin.

**Sizing & behavior**
- `default-width="number"` — initial width.
- `min-width="number"` — lower width bound.
- `max-width="number"` — upper width bound.
- `resizable="true|false"` — whether the user can drag to resize (default: `true`).

**Example**
```xml
<SidePanel side="left" id="nav" default-width="240" min-width="160" resizable="true" fill="#15151A">
  <Column gap="8" padding="8">
    <Label size="16" bold="true">Navigation</Label>
    <Separator/>
    <Button frame="false">Home</Button>
    <Button frame="false">Projects</Button>
    <Button frame="false">Settings</Button>
  </Column>
</SidePanel>
```

---

### `<TopPanel>`

A docked panel attached to the top edge of the window.  
Useful for app bars, toolbars, status strips, or context headers.

**Children:** rendered inside the panel.

**Required attributes**
- `id="string"` — egui `Id` salt to persist panel state.

**Frame & styling**
- `frame="true|false"` — enable/disable default frame (default: `true`).
- `fill="#RRGGBB[AA]"` — background color.
- `stroke-width="number"` — border width (points).
- `stroke-color="#RRGGBB[AA]"` — border color.
- `padding`, `padding-left|right|top|bottom` — inner margin.
- `margin`, `margin-left|right|top|bottom` — outer margin.

**Sizing & behavior**
- `default-height="number"` — initial height.
- `min-height="number"` — minimum height.
- `max-height="number"` — maximum height.
- `resizable="true|false"` — allow user resize (default: `true`).

**Example**
```xml
<TopPanel id="appbar" default-height="36" fill="#15151A" stroke-width="1" stroke-color="#262A33">
  <Row gap="8" padding="6">
    <Label bold="true">EFx App</Label>
    <Separator/>
    <Button frame="false">File</Button>
    <Button frame="false">Edit</Button>
    <Button frame="false">View</Button>
  </Row>
</TopPanel>
```

---

### `<BottomPanel>`

A docked panel attached to the bottom edge of the window.
Great for logs, consoles, timelines, or status bars.

**Children**: rendered inside the panel.

**Required attributes**
- `id="string"` — egui Id salt.

**Frame & styling**
- `frame="true|false"`, `fill`, `stroke-width`, `stroke-color`, `padding*` / `margin*` — same as `<TopPanel>`.

**Sizing & behavior**
- `default-height`, `min-height`, `max-height`, `resizable` — same as <TopPanel>.

**Example**
```xml
<BottomPanel id="console" default-height="200" resizable="true" fill="#0F1116">
  <ScrollArea axis="vertical" max-height="180" id="console-scroll">
    <Column gap="4" padding="6">
      <Label monospace="true">[12:00:01] Ready.</Label>
      <Label monospace="true">[12:00:02] Build succeeded.</Label>
    </Column>
  </ScrollArea>
</BottomPanel>
```

---

### `ScrollArea`

Scrollable container backed by `egui::ScrollArea`. Wraps its children and provides vertical/horizontal/both scrolling.

**Attributes**
- `axis="vertical|horizontal|both"` — scroll axis (default: vertical).
- `always-show="true|false"` — always show scrollbar even if content fits.
- `max-height="N"` — maximum height of the scroll area (f32).
- `max-width="N"` — maximum width of the scroll area (f32).
- `id="text"` — id source to persist scroll state between frames.
- `bottom="true|false"` — keep view pinned to bottom when new content arrives (useful for logs/chats).
- `right="true|false"` — keep view pinned to right on updates.

```rust,ignore
use efx_core::doc_prelude::*;
use efx::*;

// Vertical log panel with sticky bottom
efx!(Ui::default(), r#"
  <ScrollArea axis="vertical" max_height="200" always_show="true" id="log-pane" stick_to_bottom="true">
    <Column gap="6">
      <Label bold="true">Log:</Label>
      <Label>Line 1</Label>
      <Label>Line 2</Label>
      <Label>Line 3</Label>
    </Column>
  </ScrollArea>
"#);

// Horizontal scroller
efx!(Ui::default(), r#"
  <ScrollArea axis="horizontal" max_width="320" always_show="true">
    <Row gap="12">
      <Label>Item 1</Label>
      <Label>Item 2</Label>
      <Label>Item 3</Label>
      <Label>Item 4</Label>
    </Row>
  </ScrollArea>
"#);

// Both directions (e.g., big grid)
efx!(Ui::default(), r#"
  <ScrollArea axis="both" max_width="400" max_height="220">
    <Column gap="8">
      <Row gap="8"><Label>A1</Label><Label>A2</Label><Label>A3</Label><Label>A4</Label></Row>
      <Row gap="8"><Label>B1</Label><Label>B2</Label><Label>B3</Label><Label>B4</Label></Row>
      <Row gap="8"><Label>C1</Label><Label>C2</Label><Label>C3</Label><Label>C4</Label></Row>
      <Row gap="8"><Label>D1</Label><Label>D2</Label><Label>D3</Label><Label>D4</Label></Row>
    </Column>
  </ScrollArea>
"#);
```

---

### `<Window>`

An independent floating window (overlay) with optional frame and persistent state.

**Children:** rendered inside the window.

**Required attributes**
- `title="string"` — window title.

**Optional**
- `id="string"` — egui `Id` to persist window state (position/size). If omitted, egui derives an id from the title.

**Behavior**
- `open="{expr_bool}"` — binds to a boolean state; user closing the window writes back to the expression.
- `movable="true|false"` — allow dragging.
- `resizable="true|false"` — allow resizing.
- `collapsible="true|false"` — allow collapsing to title bar.
- `title-bar="true|false"` — show/hide title bar.
- `enabled="true|false"` — disable all contents when false.
- `constrain="true|false"` — constrain to viewport.
- `auto-sized="true"` — size to fit content initially.

**Positioning**
- `default-x="number"`, `default-y="number"` — initial position.
- `pos-x="number"`, `pos-y="number"` — force current position each frame.
- `anchor-h="left|center|right"`, `anchor-v="top|center|bottom"`, `anchor-x="number"`, `anchor-y="number"` — anchor to a screen corner/edge with an offset.

**Sizing**
- `default-width`, `default-height` — initial size.
- `min-width`, `min-height` — lower bounds.
- `max-width`, `max-height` — upper bounds.

**Frame & styling**
- `frame="true|false"` — enable/disable default frame (default: `true`).
- `fill="#RRGGBB[AA]"` — background color.
- `stroke-width="number"` — border width.
- `stroke-color="#RRGGBB[AA]"` — border color.
- `padding`, `padding-left|right|top|bottom` — inner margin.
- `margin`, `margin-left|right|top|bottom` — outer margin.

**Example**
```xml
<Window
        id="settings"
        title="Settings"
        open="{self.show_settings}"
        movable="true"
        resizable="true"
        default-width="360"
        default-height="240"
        anchor-h="right"
        anchor-v="top"
        anchor-x="-12"
        anchor-y="12"
        fill="#14161B"
        stroke-width="1"
        stroke-color="#262A33"
>
  <Column gap="8" padding="8">
    <Label bold="true">Preferences</Label>
    <Separator/>
    <Row gap="8">
      <Label>Theme</Label>
      <Button min_width="120">System</Button>
      <Button min_width="120">Dark</Button>
      <Button min_width="120">Light</Button>
    </Row>
  </Column>
</Window>
```

---

### `Label`
Text widget. Only text and interpolations (`{expr}`) in child nodes are allowed.

**Attributes**

- `color="name|#RRGGBB[AA]"` — text color.
- `size="N"` — font size (f32).
- `bold="true|false"`.
- `italic="true|false"`.
- `underline="true|false"`.
- `strike="true|false"`.
- `monospace="true|false"`.
- `wrap="true|false"` — enable line wrapping.

```rust
use efx_core::doc_prelude::*;
use efx::*;

efx!(Ui::default(), r##"<Label color="#66CCFF" size="16" bold="true">Hello user</Label>"##);
```

---

### `Separator`
Self-closing divider. No children allowed (otherwise `compile_error!`).

**Attributes**

- `space="N"` — uniform spacing before & after (f32).
- `space_before="N"` — spacing above.
- `space_after="N"` — spacing below.

```rust
use efx_core::doc_prelude::*;
use efx::*;

efx!(Ui::default(), r#"<Separator space="12"/>"#);
efx!(Ui::default(), r#"<Separator space_before="8" space_after="4"/>"#);
```

```rust,compile_fail
use efx_core::doc_prelude::*;
use efx::*;

/// compile_fail
efx!(Ui::default(), "<Separator>child</Separator>");
```

---

### `Button`
Button is the only tag that returns a response value (`Resp`) at the root of an expression.

**Attributes**

- `fill="color`" — background fill color.
- `rounding="N"` — rounding radius (f32).
- `min_width="N", min_height="N"` — minimum size.
- `frame="true|false"` — draw background/border.
- `enabled="true|false"` — disable/enable button.
- `tooltip="text"` — hover tooltip.

```rust
use efx_core::doc_prelude::*;
use efx::*;

let resp: Resp = efx!(Ui::default(), r#"<Button rounding="8" enabled="false" tooltip="Soon">Run</Button>"#);
assert!(!resp.clicked());
```

---

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

**Texture handle/id (desktop-friendly):**

```rust
// returns egui::Response
let resp = efx!(ui, r#"
  <Image texture="self.logo_tex_id" width="128" height="128" rounding="8" clickable="true" tooltip="Open…"/>
"#);
if resp.clicked() {
    // handle click
}
```

**URI/path source (web / custom loader):**

```rust
let _ = efx!(ui, r#"
  <Image src="assets/logo.png" max-width="200" maintain-aspect="true" id="logo-main"/>
"#);
```

**Tint + background fill:**

```rust
let _ = efx!(ui, r#"
  <Image texture="self.icon_tex" tint="#FFFFFFCC" bg-fill="#00000022" rounding="4"/>
"#);
```

### Notes

* `rounding` is uniform; per-corner radii can be added later if needed.
* `id` helps egui keep the same widget identity across frames when the source is otherwise dynamic.
* On desktop, prefer `texture` with a previously allocated `TextureId`/`TextureHandle` for performance and control. On web, `src` can be convenient alongside your asset loader.

