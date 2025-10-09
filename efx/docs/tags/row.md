### `Row`
Horizontal container. Generates `ui.horizontal(|ui| { ... })`.

**Attributes**

- `align="top|center|bottom"` — vertical alignment of children.
- `gap="N"` — horizontal spacing between children (f32).
- `wrap="true|false"` — wrap children to next line if overflow.
- `padding="N"` — extra left/right padding (f32).

```xml
<Row gap="8" padding="4" align="center">
    <Label>A</Label><Label>B</Label>
</Row>

<Row wrap="true">
    <Label>Item1</Label><Label>Item2</Label><Label>Item3</Label>
</Row>
```

---
