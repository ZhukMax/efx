use quote::quote;

#[doc(hidden)]
pub(crate) fn prelude_maker() -> proc_macro2::TokenStream {
    quote! {
        #[allow(unused, dead_code)]
        #[derive(Default)]
        struct Ui;

        #[allow(unused, dead_code)]
        #[derive(Clone, Copy, Default)]
        struct Resp;

        #[allow(unused, dead_code)]
        impl Resp { fn clicked(&self) -> bool { false } }

        #[allow(unused, dead_code)]
        impl Ui {
            fn label<S: Into<String>>(&mut self, _s: S) {}
            fn button<S: Into<String>>(&mut self, _s: S) -> Resp { Resp }
            fn separator(&mut self) {}
            fn horizontal<F: FnOnce(&mut Ui)>(&mut self, f: F) {
                let mut inner = Ui::default();
                f(&mut inner);
            }
            fn vertical<F: FnOnce(&mut Ui)>(&mut self, f: F) {
                let mut inner = Ui::default();
                f(&mut inner);
            }
        }

        let mut ui = Ui::default();
    }
}
