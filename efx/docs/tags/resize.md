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
