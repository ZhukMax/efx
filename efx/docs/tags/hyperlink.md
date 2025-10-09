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

```xml
<Column>
    <Hyperlink url="https://efxui.com" color="#66CCFF" tooltip="Project site"/>
    <Hyperlink url="help://about" open_external="false">About</Hyperlink>
</Column>
```

---
