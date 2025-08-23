//! Minimal EFx + bevy_egui overlay example.
//! Run: `cargo run --example bevy_overlay`
use bevy::prelude::*;
use bevy_egui::{EguiPlugin, EguiContexts, EguiPrimaryContextPass};
use efx::efx;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin)
        .insert_resource(UiState::default())
        .add_systems(EguiPrimaryContextPass, ui_system)
        .run();
}

#[derive(Resource, Default)]
struct UiState {
    clicks: usize,
}

fn ui_system(mut contexts: EguiContexts, mut state: ResMut<UiState>) -> Result {
    bevy_egui::egui::Window::new("EFx + Bevy").show(contexts.ctx_mut()?, |ui| {
        if efx::efx!(ui, r#"<Button>Click</Button>"#).clicked() {
            state.clicks += 1;
        }
        ui.label(format!("Clicks: {}", state.clicks));
    });

    Ok(())
}
