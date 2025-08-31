//! Minimal EFx + eframe example.
//! Native: cargo run -p efx --example eframe_demo
//! Wasm (option 1): cargo install wasm-server-runner
//!                  cargo run -p efx --example eframe_demo --target wasm32-unknown-unknown
//! Wasm (option 2): trunk serve --example eframe_demo

use eframe::egui;
use efx::efx;

#[derive(Default)]
struct State {
    name: String,
    clicks: usize,
}

#[derive(Default)]
struct App {
    state: State,
}
impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            efx!(
                ui,
                r#"
                <Column>
                    <Label>EFx demo</Label>
                    <Row>
                        <Label>Welcome,</Label>
                        <Label>efxui.com</Label>
                    </Row>
                    <Separator/>
                </Column>
            "#
            );

            ui.horizontal(|ui| {
                ui.label("Your name:");
                ui.text_edit_singleline(&mut self.state.name);
            });

            if efx!(ui, r#"<Button>Click me</Button>"#).clicked() {
                self.state.clicks += 1;
            }
            ui.label(format!("Clicks: {}", self.state.clicks));
        });
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "EFx + eframe demo",
        native_options,
        Box::new(|_cc| Ok(Box::<App>::default())),
    )
}

#[cfg(target_arch = "wasm32")]
fn main() {
    use wasm_bindgen::JsCast;
    use wasm_bindgen_futures::spawn_local;
    use web_sys::{HtmlCanvasElement, window};

    let canvas: HtmlCanvasElement = window()
        .unwrap()
        .document()
        .unwrap()
        .get_element_by_id("the_canvas_id")
        .expect("Missing <canvas id=\"the_canvas_id\"> in HTML")
        .dyn_into()
        .unwrap();

    let web_options = eframe::WebOptions::default();
    spawn_local(async move {
        eframe::WebRunner::new()
            .start(
                canvas,
                web_options,
                Box::new(|_cc| Ok(Box::<App>::default())),
            )
            .await
            .expect("failed to start eframe WebRunner");
    });
}
