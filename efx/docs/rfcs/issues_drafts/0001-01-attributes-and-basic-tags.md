# RFC-0001 / Issue 1 — Attribute rendering & basic tags

**Milestone:** 0.5  
**Related RFC:** EFX-0001  
**Goal:** Implement typed attribute rendering and add core tags (`Heading`, `Hyperlink`, `TextField`, `Image`, `Grid`).

## Tasks
- [ ] Codegen: support `attr="literal"`, `attr={expr}`, booleans.
- [ ] Map container attrs (`spacing`, etc.) to `egui` calls.
- [ ] Implement tags: Heading, Hyperlink, TextField, Image, Grid.
- [ ] Diagnostics: compile-fail tests for wrong types/unknown attrs.
- [ ] Snapshot tests for representative snippets.
- [ ] Cookbook/docs updates.

## Acceptance criteria
- Attributes compile and behave as expected in examples.
- New tags render; invalid usage produces helpful compile errors.
