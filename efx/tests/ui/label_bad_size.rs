use efx::efx;

#[derive(Default)]
struct Ui;
impl Ui { fn label<S: Into<String>>(&mut self, _s: S) {} }

fn main() {
    let mut ui = Ui::default();
    // size expects a number
    efx!(ui, r#"<Label size="big">X</Label>"#);
}
