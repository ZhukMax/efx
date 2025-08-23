//! Minimal EFx + bevy_egui overlay example.
//! Run: `cargo run --example bevy_overlay`
use bevy::prelude::*;
use bevy_egui::{EguiPlugin, EguiContexts};
use efx::efx;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin)
        .insert_resource(UiState::default())
        .add_systems(Update, ui_system)
        .run();
}

#[derive(Resource, Default)]
struct UiState {
    clicks: usize,
}

fn ui_system(mut egui_ctx: EguiContexts, mut state: ResMut<UiState>) {
    bevy_egui::egui::Window::new("EFx + Bevy overlay").show(egui_ctx.ctx_mut(), |ui| {
        efx!(ui, r#"
            <Column spacing=8>
                <Heading>Overlay</Heading>
                <Separator/>
            "#);
        if efx!(ui, r#"<Button>Click</Button>"#).clicked() {
            state.clicks += 1;
        }
        ui.label(format!("Clicks: {}", state.clicks));
        efx!(ui, r#"</Column>"#);
    });
}
