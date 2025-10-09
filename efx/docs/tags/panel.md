## Panels

### `<Panel>`

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

**Attributes**

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

```xml
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
