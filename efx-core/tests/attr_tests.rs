use efx_core::attr::{parse_bool, parse_f32, parse_enum, parse_color_rgba, Rgba};

fn rgba(r: u8, g: u8, b: u8, a: u8) -> Rgba { Rgba { r, g, b, a } }

#[test]
fn parse_bool_ok() {
    assert_eq!(parse_bool("b", "true").unwrap(), true);
    assert_eq!(parse_bool("b", "false").unwrap(), false);
    // case-insensitive and with spaces
    assert_eq!(parse_bool("b", "  TrUe ").unwrap(), true);
}

#[test]
fn parse_bool_err() {
    let e = parse_bool("b", "yes").unwrap_err();
    assert!(e.contains("expects boolean"));
}

#[test]
fn parse_f32_ok() {
    assert!((parse_f32("n", "14").unwrap() - 14.0).abs() < f32::EPSILON);
    assert!((parse_f32("n", "3.5").unwrap() - 3.5).abs() < f32::EPSILON);
    assert!((parse_f32("n", "  0.125 ").unwrap() - 0.125).abs() < f32::EPSILON);
}

#[test]
fn parse_f32_err() {
    let e = parse_f32("n", "abc").unwrap_err();
    assert!(e.contains("expects number"));
}

#[test]
fn parse_enum_ok() {
    let opts = ["left", "center", "right"];
    assert_eq!(parse_enum("align", "left", &opts).unwrap(), 0);
    assert_eq!(parse_enum("align", "CENTER", &opts).unwrap(), 1);
    assert_eq!(parse_enum("align", "Right", &opts).unwrap(), 2);
}

#[test]
fn parse_enum_err() {
    let opts = ["left", "center", "right"];
    let e = parse_enum("align", "middle", &opts).unwrap_err();
    assert!(e.contains("invalid value"));
    assert!(e.contains("left|center|right"));
}

#[test]
fn parse_color_named_ok() {
    assert_eq!(parse_color_rgba("color", "red").unwrap(), rgba(255, 0, 0, 255));
    assert_eq!(parse_color_rgba("color", "light_grey").unwrap(), rgba(192, 192, 192, 255)); // алиасы
    assert_eq!(parse_color_rgba("color", "TRANSPARENT").unwrap(), rgba(0, 0, 0, 0));
}

#[test]
fn parse_color_hex_ok() {
    assert_eq!(parse_color_rgba("color", "#112233").unwrap(), rgba(0x11, 0x22, 0x33, 0xFF));
    assert_eq!(parse_color_rgba("color", "#AABBCCDD").unwrap(), rgba(0xAA, 0xBB, 0xCC, 0xDD));
}

#[test]
fn parse_color_hex_err() {
    // Invalid length
    let e = parse_color_rgba("color", "#12345").unwrap_err();
    assert!(e.contains("expects color name"));

    // invalid characters
    let e = parse_color_rgba("color", "#GG0011").unwrap_err();
    assert!(e.contains("expects color name"));

    // without # - also an error
    let e = parse_color_rgba("color", "112233").unwrap_err();
    assert!(e.contains("expects color name"));
}
