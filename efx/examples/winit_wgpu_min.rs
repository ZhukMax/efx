//! Minimal raw winit+wgpu integration sketch for EFx/egui.
//! Build: `cargo run --example winit_wgpu_min`
//! NOTE: This is a skeleton; actual rendering loop is elided for brevity.
#![allow(unused)]
use egui::{Context, Ui};
use efx::efx;

fn main() {
    // Placeholder: In a real example, set up winit window, egui_winit & egui_wgpu,
    // then in the frame callback obtain `&egui::Context` and build a CentralPanel.
    let mut dummy = DummyUi;
    efx!(&mut dummy, r#"<Column><Label>EFx works with any egui backend</Label></Column>"#);
}

// A minimal stub Ui to demonstrate that EFx expands into `ui.*` calls.
// Replace with real `egui::Ui` in actual example.
struct DummyUi;
impl DummyUi {
    fn label(&mut self, _text: impl Into<String>) {}
    fn vertical<R>(&mut self, add_contents: impl FnOnce(&mut DummyUi) -> R) -> R { add_contents(self) }
    fn horizontal<R>(&mut self, add_contents: impl FnOnce(&mut DummyUi) -> R) -> R { add_contents(self) }
    fn separator(&mut self) {}
    fn button(&mut self, _label: impl Into<String>) -> DummyResponse { DummyResponse }
}
struct DummyResponse;
impl DummyResponse { fn clicked(&self) -> bool { false } }
