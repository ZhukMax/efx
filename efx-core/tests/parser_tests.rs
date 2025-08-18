use efx_core::{parse_str, Node, Text};

#[test]
fn text_only() {
    let ast = parse_str("Hello {{world}}!").unwrap();
    assert!(matches!(
        &ast[0],
        Node::Text(Text { value, .. }) if value == "Hello {world}!"
    ));
}

#[test]
fn element_nested_interp() {
    let src = "<Row><Label>Hello {name}</Label><Separator/></Row>";
    let ast = parse_str(src).unwrap();
    match &ast[0] {
        Node::Element(el) => {
            assert_eq!(el.name, "Row");
            assert_eq!(el.children.len(), 2);
        }
        _ => panic!("expected root element"),
    }
}

#[test]
fn mismatched_tag() {
    let err = parse_str("<A>oops</B>").unwrap_err();
    assert!(err.msg.contains("unmatched closing tag"));
}

#[test]
fn self_closed() {
    let ast = parse_str("<Separator/><Separator/>").unwrap();
    assert_eq!(ast.len(), 2);
}
