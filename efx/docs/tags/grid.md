## `<Grid>` and `<GridBreak/>`

General-purpose layout grid built on top of `egui::Grid`. Children are placed row-major (left-to-right, top-to-bottom). Use `<GridBreak/>` to start a new row explicitly.

**Syntax**
```xml
<Grid columns="3" spacing-x="8" spacing-y="4" cell-align="left" cell-padding="4" striped="true" id="cards">
  <Panel fill="#15151A" padding="8"><Label>Card A</Label></Panel>
  <Panel fill="#15151A" padding="8"><Label>Card B</Label></Panel>
  <Panel fill="#15151A" padding="8"><Label>Card C</Label></Panel>
  <GridBreak/>
  <Panel fill="#15151A" padding="8"><Label>Card D</Label></Panel>
</Grid>
```

**Attributes – `<Grid>`**

| Name           | Type   | Default  | Description                                               |
|----------------|--------|----------|-----------------------------------------------------------|
| `columns`      | int    | required | Number of columns (≥ 1).                                  |
| `striped`      | bool   | `false`  | Alternate row background.                                 |
| `spacing-x`    | f32    | `8`      | Horizontal spacing between columns (px).                  |
| `spacing-y`    | f32    | `4`      | Vertical spacing between rows (px).                       |
| `cell-padding` | f32    | `0`      | Inner padding in each cell (px, applied on both sides).   |
| `cell-align`   | enum   | `left`   | Horizontal alignment in cells: `left`, `center`, `right`. |
| `id`           | string | —        | Stable id for the grid instance.                          |

**Rules**
- Direct children must be elements (widgets/containers). Text nodes are not allowed at top level.
- `<GridBreak/>` starts a new row. Without it, rows wrap automatically when `columns` is reached.
- Returns `()` (container).

---
