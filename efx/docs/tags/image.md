## `<Image>`

Display a bitmap/texture in the UI. Works both with a preloaded texture handle/id (recommended for desktop) and with a URI-like source (useful on web or when you have your own loader).

**Syntax**

```xml
<Image
  texture="self.logo_tex_id"
  width="128"
  height="128"
  rounding="6"
  clickable="true"
  tooltip="Click to open"
/>
```

or

```xml
<Image
  src="assets/logo.png"
  max-width="256"
  maintain-aspect="true"
  id="logo-1"
/>
```

**Attributes**

| Name              | Type                                                                         | Default | Description                                                           |
|-------------------|------------------------------------------------------------------------------|---------|-----------------------------------------------------------------------|
| `texture`         | **expr** (`egui::TextureId`, `&egui::TextureHandle`, or `egui::ImageSource`) | —       | Source texture/handle. Mutually exclusive with `src`.                 |
| `src`             | string (URI/path)                                                            | —       | Image URI/path. Mutually exclusive with `texture`.                    |
| `width`           | f32                                                                          | —       | Target width. If both `width` and `height` are set, uses exact size.  |
| `height`          | f32                                                                          | —       | Target height. If both `width` and `height` are set, uses exact size. |
| `max-width`       | f32                                                                          | `∞`     | Max width (used if exact size isn’t specified).                       |
| `max-height`      | f32                                                                          | `∞`     | Max height (used if exact size isn’t specified).                      |
| `maintain-aspect` | bool                                                                         | `false` | Keep original aspect ratio when fitting.                              |
| `rounding`        | u8                                                                           | —       | Uniform corner radius.                                                |
| `tint`            | color                                                                        | —       | Multiplies image color (e.g. `#FFFFFF80` for 50% fade).               |
| `bg-fill`         | color                                                                        | —       | Background fill behind the image rect.                                |
| `id`              | string                                                                       | —       | Stable id seed (`id_source`) for consistent layout/caching.           |
| `clickable`       | bool                                                                         | `false` | If `true`, image responds to clicks (`Sense::click`).                 |
| `tooltip`         | string                                                                       | —       | Hover text shown on the image.                                        |

> Either `texture` **or** `src` must be provided (not both). `<Image>` does not accept children.

**Behavior & sizing rules**

* **Exact size**: if both `width` and `height` are set → the image is fit to that exact `vec2(width, height)`.
* **Max size**: otherwise, a max box is computed from `max-width`/`max-height` (falling back to `width`/`height` if only one side is provided).
* **Aspect**: `maintain-aspect="true"` keeps the original ratio when fitting.
* **Interactivity**: with `clickable="true"` the tag returns a normal `Response` you can query (`.clicked()`, etc.). Tooltips are applied via `on_hover_text`.

### Examples

**URI/path source (web / custom loader):**

```xml
<Image src="assets/logo.png" max-width="200" maintain-aspect="true" id="logo-main"/>
```

**Tint + background fill:**

```xml
<Image texture="self.icon_tex" tint="#FFFFFFCC" bg-fill="#00000022" rounding="4"/>
```

### Notes

* `rounding` is uniform; per-corner radii can be added later if needed.
* `id` helps egui keep the same widget identity across frames when the source is otherwise dynamic.
* On desktop, prefer `texture` with a previously allocated `TextureId`/`TextureHandle` for performance and control. On web, `src` can be convenient alongside your asset loader.

---
