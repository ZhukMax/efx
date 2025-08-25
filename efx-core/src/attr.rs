#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rgba {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

pub fn parse_bool(name: &str, s: &str) -> Result<bool, String> {
    match s.trim().to_ascii_lowercase().as_str() {
        "true" => Ok(true),
        "false" => Ok(false),
        other => Err(format!(
            "efx: attribute `{}` expects boolean (true/false), got `{}`",
            name, other
        )),
    }
}

pub fn parse_f32(name: &str, s: &str) -> Result<f32, String> {
    s.trim().parse::<f32>().map_err(|_| {
        format!(
            "efx: attribute `{}` expects number (f32), got `{}`",
            name, s
        )
    })
}

/// Returns the index of the matched option from allowed
pub fn parse_enum(name: &str, s: &str, allowed: &[&str]) -> Result<usize, String> {
    let val = s.trim().to_ascii_lowercase();
    if let Some((idx, _)) = allowed
        .iter()
        .enumerate()
        .find(|(_, v)| v.eq_ignore_ascii_case(&val))
    {
        Ok(idx)
    } else {
        Err(format!(
            "efx: attribute `{}` invalid value `{}` (allowed: {})",
            name,
            s,
            allowed.join("|")
        ))
    }
}

/// Named colors + #RRGGBB[AA] → RGBA
pub fn parse_color_rgba(name: &str, s: &str) -> Result<Rgba, String> {
    let v = s.trim();
    if let Some(rgba) = named_color_rgba(v) {
        return Ok(rgba);
    }
    if let Some(rgba) = hex_color_rgba(v) {
        return Ok(rgba);
    }
    Err(format!(
        "efx: attribute `{}` expects color name (e.g. red, light_gray) or hex #RRGGBB[AA], got `{}`",
        name, s
    ))
}

fn named_color_rgba(s: &str) -> Option<Rgba> {
    let (r, g, b, a) = match s.to_ascii_lowercase().as_str() {
        "red" => (255, 0, 0, 255),
        "green" => (0, 255, 0, 255),
        "blue" => (0, 0, 255, 255),
        "white" => (255, 255, 255, 255),
        "black" => (0, 0, 0, 255),
        "gray" | "grey" => (128, 128, 128, 255),
        "dark_gray" | "darkgrey" | "dark_grey" => (64, 64, 64, 255),
        "light_gray" | "lightgrey" | "light_grey" => (192, 192, 192, 255),
        "yellow" => (255, 255, 0, 255),
        "transparent" => (0, 0, 0, 0),
        _ => return None,
    };
    Some(Rgba { r, g, b, a })
}

fn hex_color_rgba(s: &str) -> Option<Rgba> {
    let hs = s.strip_prefix('#')?;
    if hs.len() != 6 && hs.len() != 8 {
        return None;
    }

    let r = u8::from_str_radix(&hs[0..2], 16).ok()?;
    let g = u8::from_str_radix(&hs[2..4], 16).ok()?;
    let b = u8::from_str_radix(&hs[4..6], 16).ok()?;
    let a = if hs.len() == 8 {
        u8::from_str_radix(&hs[6..8], 16).ok()?
    } else {
        0xFF
    };

    Some(Rgba { r, g, b, a })
}
