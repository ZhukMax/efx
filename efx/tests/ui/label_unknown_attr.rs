use efx::efx;

#[derive(Default)]
struct Ui;
impl Ui { fn label<S: Into<String>>(&mut self, _s: S) {} }

fn main() {
    let mut ui = Ui::default();
    // unknown attribute
    efx!(ui, r#"<Label foo="bar">X</Label>"#);
}
