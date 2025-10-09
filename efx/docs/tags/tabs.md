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
