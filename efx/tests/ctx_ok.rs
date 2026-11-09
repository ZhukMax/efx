//! `efx_ctx!` must accept `ctx, "template"` and render context-root tags
//! against a real `egui::Context` (regression: the input parser used to
//! expect a second comma after the template).

use efx::efx_ctx;

#[test]
fn efx_ctx_renders_window_and_panels() {
    let ctx = egui::Context::default();

    let _ = ctx.run(Default::default(), |ctx| {
        efx_ctx!(
            ctx,
            r#"<Window title="EFx" id="efx-ctx-test"><Label>Hello</Label></Window>"#
        );
        efx_ctx!(
            ctx,
            r#"<TopPanel id="efx-ctx-top"><Label>Top</Label></TopPanel>"#
        );
        efx_ctx!(ctx, r#"<CentralPanel><Label>Body</Label></CentralPanel>"#);
    });
}

#[test]
fn efx_ctx_accepts_expression_ctx() {
    struct App {
        ctx: egui::Context,
    }
    let app = App {
        ctx: egui::Context::default(),
    };

    let _ = app.ctx.run(Default::default(), |_| {
        efx_ctx!(
            &app.ctx,
            r#"<CentralPanel><Label>Ok</Label></CentralPanel>"#
        );
    });
}

#[test]
fn efx_ctx_evaluates_to_unit_for_every_context_root() {
    let ctx = egui::Context::default();

    let _ = ctx.run(Default::default(), |ctx| {
        let () = efx_ctx!(ctx, r#"<Window title="W"><Label>w</Label></Window>"#);
        let () = efx_ctx!(ctx, r#"<TopPanel id="t"><Label>t</Label></TopPanel>"#);
        let () = efx_ctx!(ctx, r#"<BottomPanel id="b"><Label>b</Label></BottomPanel>"#);
        let () = efx_ctx!(
            ctx,
            r#"<SidePanel side="left" id="s"><Label>s</Label></SidePanel>"#
        );
        let () = efx_ctx!(ctx, r#"<CentralPanel><Label>c</Label></CentralPanel>"#);
    });
}
