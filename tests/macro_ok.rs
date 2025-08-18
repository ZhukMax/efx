use std::fmt::Write;
use efx::efx;

#[derive(Default, Debug)]
struct RecUi {
    pub ops: Vec<String>,
}

#[derive(Clone, Copy, Debug, Default)]
struct DummyResponse {
    clicked: bool,
}
impl DummyResponse {
    fn clicked(&self) -> bool { self.clicked }
}

impl RecUi {
    fn label<S: Into<String>>(&mut self, s: S) {
        self.ops.push(format!("label:{}", s.into()));
    }
    fn button<S: Into<String>>(&mut self, s: S) -> DummyResponse {
        self.ops.push(format!("button:{}", s.into()));
        DummyResponse { clicked: false }
    }
    fn separator(&mut self) {
        self.ops.push("separator".into());
    }
    fn horizontal<F: FnOnce(&mut RecUi)>(&mut self, f: F) {
        self.ops.push("row_begin".into());
        let mut inner = RecUi::default();
        f(&mut inner);
        self.ops.push(format!("row_children={}", inner.ops.len()));
        self.ops.extend(inner.ops);
        self.ops.push("row_end".into());
    }
    fn vertical<F: FnOnce(&mut RecUi)>(&mut self, f: F) {
        self.ops.push("col_begin".into());
        let mut inner = RecUi::default();
        f(&mut inner);
        self.ops.push(format!("col_children={}", inner.ops.len()));
        self.ops.extend(inner.ops);
        self.ops.push("col_end".into());
    }
}

#[test]
fn label_and_layouts_render() {
    let mut ui = RecUi::default();
    efx!(ui, "<Column><Label>Hello {1+1}</Label><Separator/><Row><Label>Row</Label></Row></Column>");

    assert!(ui.ops.contains(&"label:Hello 2".to_string()));
    assert!(ui.ops.contains(&"separator".to_string()));
    assert!(ui.ops.contains(&"label:Row".to_string()));

    assert!(ui.ops.iter().any(|s| s.starts_with("col_begin")));
    assert!(ui.ops.iter().any(|s| s.starts_with("row_begin")));
}

#[test]
fn button_is_expression_root() {
    let mut ui = RecUi::default();
    let resp = efx!(ui, "<Button>Click {40+2}</Button>");
    assert!(ui.ops.first().unwrap().starts_with("button:Click 42"));
    assert!(!resp.clicked());
}

#[test]
fn multiple_roots_block() {
    let mut ui = RecUi::default();
    // Multiple roots should generate a block with statements
    efx!(ui, "<Label>A</Label><Separator/><Label>{\"B\"}</Label>");
    assert_eq!(ui.ops, vec!["label:A", "separator", "label:B"]);
}
