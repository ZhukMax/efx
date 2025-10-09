### `Heading`

Text heading. Generates `ui.heading(text)` with optional style overrides.

**Attributes**

- `level="1..6"` — heading level (integer).  
  *Default:* `1`. Maps to predefined `egui` text styles.
- `size="N"` — overrides the font size (f32).
- `color="name|#RRGGBB[AA]"` — text color.
- `tooltip="text"` — hover tooltip.

```xml
  <Column gap="8">
    <Heading level="1">Main title</Heading>
    <Heading level="2" color="#66CCFF">Section</Heading>
    <Heading level="3" size="14" tooltip="Subheading">Small note</Heading>
  </Column>
```
The level attribute controls the base style (h1–h6), while size and color can further adjust the appearance.

---
