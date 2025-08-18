use efx::efx;

#[derive(Default)]
struct Ui;
impl Ui {
    fn label<S: Into<String>>(&mut self, _s: S) {}
    fn button<S: Into<String>>(&mut self, _s: S) -> Resp { Resp }
    fn separator(&mut self) {}
    fn horizontal<F: FnOnce(&mut Ui)>(&mut self, f: F) { let mut inner = Ui::default(); f(&mut inner); }
    fn vertical<F: FnOnce(&mut Ui)>(&mut self, f: F) { let mut inner = Ui::default(); f(&mut inner); }
}
struct Resp;
impl Resp { fn clicked(&self) -> bool { false } }

fn main() {
    let mut ui = Ui::default();
    // Unknown tag should cause compile_error!
    let _ = efx!(ui, "<WAT>oops</WAT>");
}
