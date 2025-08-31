//! Doc-test prelude used by EFx examples.
//! Provides a tiny `Ui` stub compatible with codegen, and re-exports `egui`.

pub use egui;

#[derive(Clone, Copy, Default)]
pub struct ItemSpacing {
    pub x: f32,
    pub y: f32,
}

#[derive(Clone, Copy, Default)]
pub struct Spacing {
    pub item_spacing: ItemSpacing,
}

pub struct Resp;
impl Resp {
    #[inline]
    pub fn clicked(&self) -> bool {
        false
    }
    #[inline]
    pub fn on_hover_text(self, _t: &str) -> Self {
        self
    }
}

pub struct Ui {
    spacing_state: Spacing,
}

impl Default for Ui {
    fn default() -> Self {
        Ui {
            spacing_state: Spacing {
                item_spacing: ItemSpacing { x: 0.0, y: 0.0 },
            },
        }
    }
}

impl Ui {
    #[inline]
    pub fn label<T>(&mut self, _text: T) {}
    #[inline]
    pub fn heading<T>(&mut self, _text: T) {}
    #[inline]
    pub fn button<T>(&mut self, _text: T) -> Resp {
        Resp
    }
    #[inline]
    pub fn separator(&mut self) {}
    #[inline]
    pub fn hyperlink<T: Into<String>>(&mut self, _url: T) {}
    #[inline]
    pub fn hyperlink_to<T1: Into<String>, T2: Into<String>>(&mut self, _label: T1, _url: T2) {}
    #[inline]
    pub fn ctx(&self) -> egui::Context {
        egui::Context::default()
    }

    // --- add/add_enabled for Button w/attr ---
    #[inline]
    pub fn add<T>(&mut self, _w: T) -> Resp {
        Resp
    }
    #[inline]
    pub fn add_enabled<T>(&mut self, _enabled: bool, _w: T) -> Resp {
        Resp
    }

    // --- spacing/padding API ---
    #[inline]
    pub fn spacing(&self) -> &Spacing {
        &self.spacing_state
    }
    #[inline]
    pub fn spacing_mut(&mut self) -> &mut Spacing {
        &mut self.spacing_state
    }
    #[inline]
    pub fn add_space(&mut self, _v: f32) {}

    // --- layout API ---
    #[inline]
    pub fn horizontal<F: FnOnce(&mut Ui)>(&mut self, f: F) {
        let mut u = Ui::default();
        f(&mut u);
    }
    #[inline]
    pub fn vertical<F: FnOnce(&mut Ui)>(&mut self, f: F) {
        let mut u = Ui::default();
        f(&mut u);
    }
    #[inline]
    pub fn horizontal_wrapped<F: FnOnce(&mut Ui)>(&mut self, f: F) {
        let mut u = Ui::default();
        f(&mut u);
    }
    #[inline]
    pub fn with_layout<F: FnOnce(&mut Ui)>(&mut self, _layout: egui::Layout, f: F) {
        let mut u = Ui::default();
        f(&mut u);
    }
}
