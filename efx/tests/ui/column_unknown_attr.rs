use efx::efx;
#[derive(Default)] struct Ui;
impl Ui { fn vertical<F: FnOnce(&mut Ui)>(&mut self, f: F){ let mut u=Ui::default(); f(&mut u)} }
fn main() {
    let mut ui = Ui::default();
    efx!(ui, r#"<Column foo="bar"><Label>A</Label></Column>"#);
}
