use efx::efx;

#[derive(Default)]
struct Ui;
impl Ui {
    fn button<S: Into<String>>(&mut self, _s: S) -> Resp { Resp }
    fn add<T>(&mut self, _w: T) -> Resp { Resp }
    fn add_enabled<T>(&mut self, _b: bool, _w: T) -> Resp { Resp }
}
struct Resp; impl Resp { fn clicked(&self) -> bool { false } }

fn main() {
    let mut ui = Ui::default();
    // enabled must be true/false
    let _ = efx!(ui, r#"<Button enabled="maybe">Run</Button>"#);
}
