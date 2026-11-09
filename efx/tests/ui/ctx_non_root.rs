use efx::efx_ctx;

fn main() {
    let ctx = egui::Context::default();
    // Only Window / CentralPanel / TopPanel / BottomPanel / SidePanel may be the root.
    efx_ctx!(ctx, r#"<Label>not a context root</Label>"#);
}
