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
                <Column>
                    <Label>EFx demo</Label>
                    <Row>
                        <Label>Welcome,</Label>
                        <Label>efxui.com</Label>
                    </Row>
                    <Separator/>
                </Column>
            "#);

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

// ---- Web (wasm32) entrypoint ----
#[cfg(target_arch = "wasm32")]
use eframe::wasm_bindgen::{self, prelude::*};


#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn start() {
    use eframe::{WebOptions, WebRunner};
    use wasm_bindgen_futures::spawn_local;
    use eframe::wasm_bindgen::JsCast;
    use web_sys::{window, HtmlCanvasElement};

    spawn_local(async {
        let doc = window().unwrap().document().unwrap();
        let canvas: HtmlCanvasElement = doc
            .get_element_by_id("the_canvas_id")
            .expect("Missing <canvas id=\"the_canvas_id\"> in HTML")
            .dyn_into()
            .unwrap();

        let options = WebOptions::default();
        WebRunner::new()
            .start(canvas, options, Box::new(|_cc| Ok(Box::<App>::default())))
            .await
            .expect("failed to start eframe WebRunner");
    });
}
