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
