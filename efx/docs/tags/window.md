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
