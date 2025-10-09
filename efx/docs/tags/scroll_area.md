## `ScrollArea`

Scrollable container backed by `egui::ScrollArea`. Wraps its children and provides vertical/horizontal/both scrolling.

**Attributes**
- `axis="vertical|horizontal|both"` — scroll axis (default: vertical).
- `always-show="true|false"` — always show scrollbar even if content fits.
- `max-height="N"` — maximum height of the scroll area (f32).
- `max-width="N"` — maximum width of the scroll area (f32).
- `id="text"` — id source to persist scroll state between frames.
- `bottom="true|false"` — keep view pinned to bottom when new content arrives (useful for logs/chats).
- `right="true|false"` — keep view pinned to right on updates.

Vertical log panel with sticky bottom
```xml
  <ScrollArea axis="vertical" max_height="200" always_show="true" id="log-pane" stick_to_bottom="true">
    <Column gap="6">
      <Label bold="true">Log:</Label>
      <Label>Line 1</Label>
      <Label>Line 2</Label>
      <Label>Line 3</Label>
    </Column>
  </ScrollArea>
```

Horizontal scroller
```xml
  <ScrollArea axis="horizontal" max_width="320" always_show="true">
    <Row gap="12">
      <Label>Item 1</Label>
      <Label>Item 2</Label>
      <Label>Item 3</Label>
      <Label>Item 4</Label>
    </Row>
  </ScrollArea>
```

Both directions (e.g., big grid)
```xml
  <ScrollArea axis="both" max_width="400" max_height="220">
    <Column gap="8">
      <Row gap="8"><Label>A1</Label><Label>A2</Label><Label>A3</Label><Label>A4</Label></Row>
      <Row gap="8"><Label>B1</Label><Label>B2</Label><Label>B3</Label><Label>B4</Label></Row>
      <Row gap="8"><Label>C1</Label><Label>C2</Label><Label>C3</Label><Label>C4</Label></Row>
      <Row gap="8"><Label>D1</Label><Label>D2</Label><Label>D3</Label><Label>D4</Label></Row>
    </Column>
  </ScrollArea>
```

---
