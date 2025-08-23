//! Minimal EFx + eframe example.
//! Build native: `cargo run --example eframe_demo`
//! Build wasm:   `cargo build --example eframe_demo --target wasm32-unknown-unknown`
use eframe::egui;
use efx::efx;

#[derive(Default)]
struct State {
    name: String,
    clicks: usize,
}

pub fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "EFx + eframe demo",
        native_options,
        Box::new(|_cc| Box::<App>::default()),
    )
}

struct App {
    state: State,
}
impl Default for App {
    fn default() -> Self { Self { state: State::default() } }
}
impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            efx!(ui, r#"
                <Column spacing=8>
                    <Heading>EFx 0.5 (preview)</Heading>
                    <Row spacing=6>
                        <Label>Welcome,</Label>
                        <Hyperlink url="https://efxui.com">efxui.com</Hyperlink>
                    </Row>
                    <Separator/>
                "#);
            // Simple text field (will require 0.5 TextField implementation; placeholder below)
            ui.horizontal(|ui| {
                ui.label("Your name:");
                ui.text_edit_singleline(&mut self.state.name);
            });
            if efx!(ui, r#"<Button>Click me</Button>"#).clicked() {
                self.state.clicks += 1;
            }
            ui.label(format!("Clicks: {}", self.state.clicks));
            efx!(ui, r#"</Column>"#);
        });
    }
}
