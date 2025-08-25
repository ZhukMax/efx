use quote::quote;

#[doc(hidden)]
pub(crate) fn prelude_maker() -> proc_macro2::TokenStream {
    quote! {
        #[allow(unused, dead_code)]
        #[derive(Default)]
        struct Ui {
            spacing_state: egui::Spacing,
        }

        impl Default for Ui {
            fn default() -> Self {
                Ui {
                    spacing_state: egui::Spacing { item_spacing: egui::ItemSpacing { x: 0.0, y: 0.0 } },
                }
            }
        }

        // Accept any text-like type, including String & egui::RichText.
        #[allow(unused, dead_code)]
        impl Ui {
            // basic text/widgets
            fn label<T>(&mut self, _t: T) {}
            fn button<T>(&mut self, _t: T) -> Resp { Resp }
            fn separator(&mut self) {}

            // widget adders (used by Button impl with attrs)
            fn add<T>(&mut self, _w: T) -> Resp { Resp }
            fn add_enabled<T>(&mut self, _b: bool, _w: T) -> Resp { Resp }

            // spacing API
            fn spacing(&self) -> &egui::Spacing { &self.spacing_state }
            fn spacing_mut(&mut self) -> &mut egui::Spacing { &mut self.spacing_state }
            fn add_space(&mut self, _v: f32) {}

            // layouts
            fn horizontal<F: FnOnce(&mut Ui)>(&mut self, f: F) { let mut u = Ui::default(); f(&mut u); }
            fn vertical<F: FnOnce(&mut Ui)>(&mut self, f: F) { let mut u = Ui::default(); f(&mut u); }
            fn horizontal_wrapped<F: FnOnce(&mut Ui)>(&mut self, f: F) { let mut u = Ui::default(); f(&mut u); }
            fn with_layout<F: FnOnce(&mut Ui)>(&mut self, _layout: egui::Layout, f: F) { let mut u = Ui::default(); f(&mut u); }
        }

        #[allow(unused, dead_code)]
        #[derive(Clone, Copy, Default)]
        struct Resp;

        #[allow(unused, dead_code)]
        impl Resp {
            fn clicked(&self) -> bool { false }
            fn on_hover_text(self, _t: &str) -> Self { self }
        }

        // Optional helpers if examples refer to them:
        #[allow(dead_code)]
        mod egui {
            pub struct RichText;
            impl RichText {
                pub fn new<T>(_t: T) -> Self { RichText }
                pub fn color(self, _c: Color32) -> Self { self }
                pub fn size(self, _s: f32) -> Self { self }
                pub fn strong(self) -> Self { self }
                pub fn italics(self) -> Self { self }
                pub fn underline(self) -> Self { self }
                pub fn strikethrough(self) -> Self { self }
                pub fn monospace(self) -> Self { self }
            }
            pub mod widgets {
                use super::RichText;
                pub struct Label;
                impl Label {
                    pub fn new(_r: RichText) -> Self { Label }
                    pub fn wrap(self, _b: bool) -> Self { self }
                }
            }

            pub struct Button;
            impl Button {
                pub fn new(_t: RichText) -> Self { Button }
                pub fn fill(self, _c: Color32) -> Self { self }
                pub fn rounding(self, _r: Rounding) -> Self { self }
                pub fn min_size(self, _v: Vec2) -> Self { self }
                pub fn frame(self, _b: bool) -> Self { self }
            }

            pub struct Rounding(f32);
            impl Rounding {
                pub fn same(r: f32) -> Self { Rounding(r) }
            }

            #[derive(Clone, Copy)]
            pub struct Vec2(pub f32, pub f32);
            pub fn vec2(x: f32, y: f32) -> Vec2 { Vec2(x, y) }

            #[derive(Clone, Copy)]
            pub struct Color32;
            impl Color32 {
                pub const RED: Color32 = Color32;
                pub const GREEN: Color32 = Color32;
                pub const BLUE: Color32 = Color32;
                pub const WHITE: Color32 = Color32;
                pub const BLACK: Color32 = Color32;
                pub const GRAY: Color32 = Color32;
                pub const DARK_GRAY: Color32 = Color32;
                pub const LIGHT_GRAY: Color32 = Color32;
                pub const YELLOW: Color32 = Color32;
                pub const TRANSPARENT: Color32 = Color32;

                pub fn from_rgba_unmultiplied(_: u8, _: u8, _: u8, _: u8) -> Color32 { Color32 }
            }

            #[derive(Clone, Copy)]
            pub enum Align { Min, Center, Max }

            pub struct Layout;
            impl Layout {
                pub fn left_to_right(_y: Align) -> Layout { Layout }
                pub fn top_down(_x: Align) -> Layout { Layout }
            }

            pub struct ItemSpacing { pub x: f32, pub y: f32 }
            pub struct Spacing { pub item_spacing: ItemSpacing }
        }

        let mut ui = Ui::default();
    }
}
