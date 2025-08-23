# RFC-0001 / Issue 3 — Examples & CI matrix

**Milestone:** 0.5  
**Related RFC:** EFX-0001  
**Goal:** Provide runnable examples and a CI matrix to build them.

## Tasks
- [ ] Add `examples/eframe_demo.rs` (native + wasm build).
- [ ] Add `examples/bevy_overlay.rs` (native build).
- [ ] Add `examples/winit_wgpu_min.rs` (native build).
- [ ] GitHub Actions: matrix job builds the three examples.
- [ ] README: Supported runtimes section with links to examples.

## Acceptance criteria
- CI builds the examples on push/PR (no GUI run).
- README documents Tier‑1/Tier‑2 runtimes with links.
