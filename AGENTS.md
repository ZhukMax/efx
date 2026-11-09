# AGENTS.md — EFx

Guide for AI agents (and humans) who develop and promote **EFx**, an XML template engine for `egui`.
Read this file first. Everything here was checked against the code on 2026-11-09; when something
here disagrees with the code, the code wins — fix this file in the same change.

---

## 1. What EFx is

`efx!(ui, r#"<Column><Label>Hi {name}</Label></Column>"#)` is a **proc-macro** that parses an
XML-like string **at compile time** and expands it into ordinary `egui` calls
(`ui.vertical(...)`, `ui.label(...)`, `egui::Button::new(...)`). There is no runtime, no
interpreter, no `eval`: if the template is wrong, the crate does not compile.

- Works with anything that hands out `&mut egui::Ui`: `eframe` (native + wasm), `bevy_egui`,
  raw `winit + wgpu`; Tier-2: `egui-miniquad`, SDL2, `egui_glow`.
- It is an **immediate-mode DSL**, not a retained UI framework (non-goal, see RFC EFX-0001 §2).
- Second macro: `efx_ctx!(ctx, "...")` takes an `egui::Context` and accepts exactly one
  context-root element (`Window`, `CentralPanel`, `TopPanel`, `BottomPanel`, `SidePanel`).
- Author / maintainer: Max Zhuk (`mail@zhukmax.com`). Repo: `github.com/ZhukMax/efx`.
  Site: `efxui.com` (also hosts `llms.txt`).

## 2. Repository map

| Path | What it is | Published |
|---|---|---|
| `efx/` | The `efx` proc-macro crate (v0.6.1 on `main`). Tags, codegen, docs, examples, tests. | crates.io `efx` |
| `efx-core/` | Parser + AST (`parse_str`, `Node`, `Element`, `Attr`), attribute value parsers (`attr.rs`), `doc_prelude` (a fake `Ui` for doctests). v1.1.1. | crates.io `efx-core` |
| `efx-attrnames/` | `#[derive(AttrNames)]` → `ATTR_NAMES` const (+ `ATTR_INFO` with feature `attrnames-info`). v1.0.0. | crates.io `efx-attrnames` |
| `efx-sandbox/` | Native `eframe` app for manual testing of tags (`make sandbox`). | no |
| `efx-syntax/` | VS Code / JetBrains TextMate grammar + snippets for `*.efx` files (npm package layout, not on the Marketplace). | no |
| `efx/docs/` | `intro.md`, `tags.md`, `guide.md` (+ `fr/` translations), RFCs. **Included into docs.rs** via `include_str!` in `efx/src/lib.rs`. | via docs.rs |
| `efx/examples/` | `eframe_demo`, `bevy_overlay`, `winit_wgpu_min` — built in CI, never run. | — |
| `.github/workflows/examples-build.yml` | The only CI on `main`: builds the examples (native, wasm32) and the sandbox. | — |

The workspace patches `efx-core` / `efx-attrnames` to local paths (`[patch.crates-io]` in the
root `Cargo.toml`), so local changes to core are picked up without publishing. `Cargo.lock` is
git-ignored.

## 3. Branches and where work goes

- `main` — last release (0.6.1). Do not develop features here directly.
- `v0.7` — **next release, ~12 commits ahead of `main`, unreleased.** Contains: egui 0.33 /
  bevy 0.18 / bevy_egui 0.39 bump, templates loaded from `.efx` files (`efx-core/src/component.rs`,
  lookup in `$EFX_UI_PATH` and `src/ui/`), unknown tags resolved as file components, docs split
  into `efx/docs/tags/*.md`, `.cursorrules`, CI workflows `test.yml` / `clippy.yml` /
  `rustfmt.yml`, new README.
- `playground` — draft `efx-playground` crate (wasm playground idea; uses a not-yet-existing
  `onClick=` attribute, does not compile against current tags).
- `attrnames`, `grammar`, `rfc`, `sandbox`, `v0.5`, `v0.6` — historical, merged.

PRs target the **release branch of the milestone** (`v0.7` now), not `main` (CONTRIBUTING.md §4).
Branch names: `feat/…`, `fix/…`, `docs/…`, `chore/…`. Commit/PR titles: Conventional Commits
(`feat(button): add rounding attribute`). One issue per branch.

**Before starting any code task, check which branch it belongs to.** Most new work should be
based on `origin/v0.7`, because `main` lacks the 0.7 refactors and the egui bump.

## 4. Commands

```bash
make fmt          # cargo fmt --all -- --check
make format       # cargo fmt --all
make lint         # cargo clippy --workspace --all-targets -- -D warnings
make test         # cargo test -p efx-core && cargo test -p efx
make test-all     # cargo test --workspace  (includes trybuild compile-fail tests)
make check        # fmt + lint + test-all — the pre-release gate
make sandbox      # cargo run -p efx-sandbox  (opens a window)

cargo check -p efx --features extras                     # DataTable lives behind `extras`
cargo build -p efx --example eframe_demo                 # same as CI
cargo build -p efx --example eframe_demo --target wasm32-unknown-unknown
cargo build -p efx --example bevy_overlay                # heavy; needs X11/udev/alsa libs on Linux
cargo doc -p efx --all-features --no-deps                # what docs.rs builds
```

State of the gate on 2026-11-09 (local rustc/clippy 1.99.1):

- `main`: `cargo test --workspace` **passes** (parser 6, attr 9, macro_ok 3, trybuild 9 cases,
  14 doctests). `make lint` **fails** — 32 clippy errors from newer lints (`question_mark`,
  `clone_on_copy`, `manual_contains`, …); `make fmt` fails on one file. So `make check`
  (and therefore `make patch`) is red on `main` today. `v0.7` fixes the style issues.
- `v0.7` as pushed: `efx-sandbox` **does not compile** (`efx_ctx!` → "expected `,`", see §7)
  and trybuild case `tests/ui/unknown_tag.rs` mismatches. Both fixed on branch
  `fix/efx-ctx-input` (PR #43, not merged yet): with it, tests, clippy (`-D warnings`, with and
  without `--all-features`), fmt, examples and sandbox are all green.
- CI on `main` only builds examples + sandbox; tests/clippy/fmt workflows exist only on `v0.7`.
- trybuild `.stderr` output depends on the rustc version — a toolchain bump can break these
  tests without any code change.

Releases (`make patch` / `make minor` / `make minor-core`) run `cargo release` **and**
`cargo publish`. They are irreversible — **never run them, push tags, or publish anything without
Max's explicit go-ahead.** Release order when core changes: `efx-attrnames` → `efx-core` → `efx`.

## 5. Architecture (how a template becomes code)

```
efx!(ui, "<Row>…</Row>")
  └─ efx/src/input.rs        parse `ui_expr , "literal"`
  └─ efx_core::parse_str     Lexer (ast/lexer.rs) → Parser (ast/parser.rs) → Vec<Node>
                             Node = Element{name, attrs, children} | Text | I11n{expr_src}
  └─ efx/src/render.rs       render_element_stmt: match on tag name → render_tag::<T>()
  └─ efx/src/tags/<tag>.rs   T::from_element (validate attrs/children) → T::render → TokenStream
```

Key facts:

- **Two traits** in `efx/src/interfaces/`: `Tag` (leaf/expression tags like `Button`, `Label`)
  and `Block` (containers with `content` + `prolog_epilogue`; blanket `impl<T: Block> Tag for T`).
- **Attributes**: each tag has a private `struct Attributes` with `#[derive(AttrNames)]`;
  `utils::attr::attr_map(el, Attributes::ATTR_NAMES, "<Tag>")` rejects unknown and duplicate
  names; typed helpers `bool_opt`, `f32_opt`, `u8_opt`, `color_tokens_opt`, `margin_tokens`,
  `stroke_tokens`. Value parsing lives in `efx-core/src/attr.rs` (shared with future tooling).
- **Errors are compile errors**: every validation failure returns
  `quote!{ compile_error!("efx: …") }`. Messages start with `efx:` and name the tag.
- **Return value**: a root `<Button>` (alone in the macro) evaluates to `egui::Response`; that is
  how users handle clicks today. Every other root expands to a `()` block.
- **Interpolation** `{expr}` is only meaningful in text content; it becomes
  `format!("{}", expr)`. `{{` / `}}` escape braces.
- **Generated code** refers to `egui::…` by bare path — the user's crate must have `egui` in
  scope (for Bevy: `use bevy_egui::egui;`). Internal temporaries are prefixed `__efx_`.
- **Windows/panels** get the context via `#ui.ctx()`. `efx_ctx!` wraps a `Context` in a tiny
  shim struct that only provides `.ctx()`.
- `efx` depends on `egui` itself (for `extras`/types); the version pin in `efx/Cargo.toml` is a
  public compatibility promise — bumping egui is a **minor** release.

### Syntax as actually implemented (not as the RFC hopes)

- Attribute values **must be quoted**: `gap="8"`, not `gap=8`. Single or double quotes.
- **No boolean shorthand** (`<Button disabled/>` fails) — write `enabled="false"`.
- **No `attr={expr}`** yet. Expression attributes — `TextField value`, `Image texture`,
  `Tabs active`, `Window open` (helpers `expr_req` / `expr_opt` in `utils/expr.rs`) — take Rust
  code inside the quoted string (`value="&mut state.name"`) and are parsed with `syn`.
- Colors: named (`red`, `light_gray`, `transparent`, …) or `#RRGGBB[AA]`. No `#RGB` (issue #21).
- Tag names are case-sensitive.
- Attribute naming is **inconsistent**: `Button`/`Label` use snake_case (`min_width`), panels,
  `ScrollArea`, `Image`, `Grid`, `DataTable` use kebab-case (`default-width`, `max-height`).
  The v0.7 `.cursorrules` wrongly says "always snake_case". Do not rename existing attributes
  (breaking); a unified scheme with aliases is an open maintainer decision (issue #20).

## 6. Adding or changing a tag — checklist

1. `efx/src/tags/<tag>.rs`: struct + `Attributes` with `#[derive(AttrNames)]`; implement `Tag`
   or `Block`. Copy the closest existing tag (`separator.rs` is the smallest `Block`,
   `button.rs` the canonical `Tag`). Pass the **correct tag name** to `attr_map`.
2. Register it in `efx/src/tags/mod.rs` and in the `match` in `efx/src/render.rs`
   (child-only tags like `Tab`, `Tr`, `GridBreak` get a `compile_error!` arm there).
3. If it needs `egui_extras`, gate it with `#[cfg(feature = "extras")]` and add the
   `not(feature)` arm with a helpful error, like `DataTable`.
4. Tests (all required):
   - positive case in `efx/tests/macro_ok.rs` or a doctest using `efx_core::doc_prelude::*`
     (extend the fake `Ui` in `efx-core/src/doc_prelude.rs` if your codegen calls new methods);
   - compile-fail cases in `efx/tests/ui/<tag>_<case>.rs` + `.stderr` (unknown attr, bad value,
     forbidden children). Generate `.stderr` with `TRYBUILD=overwrite cargo test -p efx --test
     compile_tests`, then **read** the diff — never hand-edit expected output.
   - value parsers → `efx-core/tests/attr_tests.rs`; syntax → `efx-core/tests/parser_tests.rs`.
5. Docs: section in `efx/docs/tags.md` (on `v0.7`: `efx/docs/tags/<tag>.md` + an
   `include_str!` line in `lib.rs`). Rust code blocks there are **doctests** — they must compile
   with `doc_prelude`, or be marked `ignore`/`text`. Add the tag to `efx-syntax/snippets/efx.json`
   and to the tag reference in `.cursorrules` / `llms.txt`.
6. Add a usage in `efx-sandbox` if it is visual, and a line in `efx/Changelog.md` under the
   upcoming version's heading (CONTRIBUTING.md says `CHANGELOG.md` / "Unreleased"; the real file
   is `efx/Changelog.md` and uses a `#### 0.7 (conceivably)` heading).
7. Run `make check` and `cargo check -p efx --features extras` before calling it done.

Open tag requests with acceptance criteria already written: #24 ComboBox, #25 SelectableLabel,
#26 Radio, #27 Checkbox, #28 Menu, #29 Slider, #30 ProgressBar, #32 DragValue.
Interactive tags need a binding story (`value="&mut state.x"` like `TextField`) — keep it
consistent with `TextField`.

## 7. Known issues and debt (verified 2026-11-09)

Fix these when touching the area; each is a good small PR.

- `efx/src/tags/scroll_area.rs` passes `"Button"` to `attr_map` → errors on `<ScrollArea>` say
  `<Button> unknown attribute`.
- Stale docs on `main`: `efx/src/lib.rs` doc-comment and `guide.md` say attributes are
  "parsed but ignored" (rendered since 0.5); README says invalid templates show a runtime label
  — they are compile errors; `guide.md` suggests `--emit=mir` instead of `cargo expand`.
- `efx_ctx!` is not documented in README or `docs/` (only the RFC, which says "decision: no").
- RFC EFX-0001 promises `attr={expr}`, boolean shorthand, events, components, themes —
  none implemented. Changelog lists events/components as "0.7 (conceivably)". Keep public
  messaging to what exists.
- MSRV 1.75 is declared but not tested in CI.
- On `v0.7`: `EfxCtxInput::parse` in `efx/src/input.rs` read the comma twice, so
  **`efx_ctx!` was broken** — fixed on `fix/efx-ctx-input`, with regression tests
  `efx/tests/ctx_ok.rs` (renders against a real `egui::Context`) and
  `tests/ui/ctx_non_root.rs`. File templates read with
  `std::fs` are not tracked by cargo, so editing a `.efx` file does not trigger a rebuild
  (needs an `include_str!`/`include_bytes!` trick); recursive file components have no cycle
  detection; README links point to `google.com/search?q=…` and GIFs are placeholders.
- **Generated code must be lint-clean in the user's crate.** Clippy/rustc lints fire on macro
  output in the caller's code (e.g. a trailing `()` → `unused_unit`, deprecated egui APIs,
  unused `#[must_use]` `Response`). After codegen changes, run clippy on the sandbox/examples,
  not only on `efx` itself.
- `<Hyperlink>` returns `Response` only in "advanced" mode (styles/tooltip); the simple mode
  evaluates to `()`. Root-level return type is therefore attribute-dependent.
- Untracked `wip/` and `*.dump.txt` (`make dump`) are git-ignored scratch — don't commit them.

## 8. Roadmap (GitHub milestones)

- **v0.7** (open, 12 issues): theming `efx_theme!` (#17), design tokens (#18), scoped overrides
  (#19), layout attribute unification (#20), events `onClick/onHover/onChange` (#10), classes
  (#11), perf of text/interpolation (#22), docs: theming guide + cookbook + migration (#23, #13).
- **v1.0** (open, 2 issues).
- Unscheduled: template validation CLI / cargo hook (#39), error-message audit (#38),
  inline comments in tags (#15), Hyperlink example (#14), compile-fail tests for events (#12).

New features of size go through an RFC in `efx/docs/rfcs/` (Draft → Proposed → Accepted →
In Progress → Completed). Breaking changes require an issue/RFC first (SemVer, pre-1.0 rules:
breaking = minor bump).

---

## 9. Promotion

### Baseline (2026-11-09) — measure against this

- GitHub: 45 stars, 4 forks, Discussions **off**. External contributors so far: French docs
  (Rob-Parker), CI (aravindan888).
- crates.io `efx`: 1,718 downloads total, **16 in the last 90 days**; `efx-core`: 1,258 / 47.
- Last release 0.6.1 on 2025-10-08; ~13 months without a release. `efxui.com` was **down** during this
  check (HTTPS timed out, HTTP returned 503) — the GitHub homepage field, the docs/examples and
  `llms.txt` all point there, so restoring it comes before any outreach.

The first lever is shipping: a 0.7 release with the egui 0.33 bump is the precondition for any
launch post (people try the crate the day they read about it; an egui version mismatch kills it).

### Positioning

- One-liner: *"Write egui UIs like HTML — compiled to plain egui calls, zero runtime."*
- Audience, in priority order: Bevy devs building debug panels/HUDs; authors of internal tools
  and dashboards on `eframe`; egui users who find nested closures noisy; AI-assisted ("vibe")
  coders — a compact, declarative DSL is easier for an LLM to generate correctly.
- Defensible claims: compile-time only, no runtime cost, compile errors instead of runtime
  surprises, mixes freely with hand-written egui, works across egui runtimes, wasm.
- Claims that need evidence before use: "Fast as C" (say "zero runtime overhead" instead),
  "2× more UI per LLM context" (#40 — measure tokens on a real side-by-side example first),
  anything about components/events/themes until they ship.
- Be honest about the trade-off: templates are strings, so IDE support inside them is limited
  (that is what `efx-syntax`, `.cursorrules` and #39 address).

### Assets — what exists vs. what's missing

| Exists | Missing / broken |
|---|---|
| Cover image `efx/docs/efx_cover.png`, README, docs.rs, FR docs | Real GIFs / screenshots (v0.7 README has placeholders, #35) |
| `efxui.com/llms.txt` (#36), `.cursorrules` on `v0.7` (#37) | Context7 listing (#42) |
| Examples for 3 runtimes, sandbox | Hosted wasm demo / online playground (`playground` branch is a stub) |
| `efx-syntax` grammar | Published VS Code Marketplace / Open VSX extension |
| — | Side-by-side "egui vs EFx" snippet + token count (#40); "no-context coding" video (#41) |

### Channels (candidates — verify each one's current rules before posting)

Reddit r/rust and r/bevy (launch post on a release, with a GIF); This Week in Rust (submit the
release/blog post); users.rust-lang.org announcements; egui and Bevy community
Discords/Discussions; lib.rs (indexes crates.io automatically — keep keywords/categories sharp);
GitHub topics (already set); a short blog post / dev.to article per release; awesome-lists for
egui / Rust GUI. Turn on GitHub Discussions so newcomers have a place to ask.

### Rules for promotion work

- **Nothing is posted, submitted, published or sent without Max's explicit approval** of the
  exact text and the exact destination. Drafts go into the repo (or a doc) for review.
- Every code snippet in marketing material must compile against the released version.
- Every number (stars, downloads, benchmarks, token counts) is dated and sourced.
- Marketing tasks are GitHub issues with the `marketing` label; link the draft in the issue.

---

## 10. General rules for agents in this repo

- Code, docs, comments, commit messages: **English**. FR docs: update `efx/docs/fr/` when you
  change the English original, or add a note in the PR that the translation is behind.
- `cargo fmt` + clippy with `-D warnings` must pass; no new `unwrap()` in codegen paths —
  return a `compile_error!` with an `efx:` message instead.
- Don't bump egui/bevy versions, MSRV, or published crate versions as a side effect of another
  task.
- Don't commit, push, open PRs, release or publish unless asked. Never commit `Cargo.lock`.
- Keep this file current: if you learn something that would have saved you time, add it here.
