#![cfg(feature = "expose-parser")]

use efx::{Parser, Node, Text};

#[test]
fn text_only() {
    let mut parser = Parser::new("Hello {{world}}!");
    let ast = parser.parse().unwrap();
    assert!(matches!(
        &ast[0],
        Node::Text(Text { value, .. }) if value == "Hello {world}!"
    ));
}

#[test]
fn self_closed() {
    let mut parser = Parser::new("<Separator/>");
    let ast = parser.parse().unwrap();
    assert_eq!(ast.len(), 1);
}

#[test]
fn element_nested_interp() {
    let mut parser = Parser::new("<Label>{42}</Label>");
    let ast = parser.parse().unwrap();
    assert_eq!(ast.len(), 1);
}

#[test]
fn mismatched_tag() {
    let mut parser = Parser::new("<Row></Column>");
    let err = parser.parse().unwrap_err();
    assert!(err.msg.contains("mismatched closing tag"));
}
