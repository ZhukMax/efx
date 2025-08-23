# RFC-0001 / Issue 2 — Windows & panels via `ui.ctx()`

**Milestone:** 0.5  
**Related RFC:** EFX-0001  
**Goal:** Add `<Window>`, `<CentralPanel>`, `<SidePanel>` tags using `ui.ctx()`.

## Tasks
- [ ] Codegen: obtain `egui::Context` via `ui.ctx()` safely.
- [ ] Implement Window, CentralPanel, SidePanel tags.
- [ ] Tests: basic build/run smoke tests in examples.
- [ ] Docs: usage and caveats (style scope, overlapping panels).

## Acceptance criteria
- Examples use tags to open windows/panels successfully.
- CI builds examples where these tags are used.
