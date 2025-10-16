
struct App {
    clicks: i32,
}

impl App {
    fn new(_: &eframe::CreationContext<'_>) -> Self {
        Self { clicks: 0 }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let clicks = self.clicks;
            efx!(ui, r#"
              <Column gap="8" align="center">
                <Heading>EFx Playground</Heading>
                <Label color="green">Clicks: {clicks}</Label>
                <Button onClick=increment>Click me</Button>
              </Column>
            "#);
        });
    }
}

impl App {
    fn increment(&mut self) { self.clicks += 1; }
}

#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result<()> {
    let opts = eframe::NativeOptions::default();
    eframe::run_native("EFx Playground", opts, Box::new(|cc| Box::new(App::new(cc))))
}

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub async fn start() -> Result<(), wasm_bindgen::JsValue> {
    eframe::WebLogger::init(log::LevelFilter::Info).ok();
    let web_options = eframe::WebOptions::default();
    eframe::WebRunner::new()
        .start("efx-canvas", web_options, Box::new(|cc| Box::new(App::new(cc))))
        .await
}
