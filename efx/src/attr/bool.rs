/// Parsing bool: "true"/"false" (case-insensitive).
pub(crate) fn parse_bool(name: &str, s: &str) -> Result<bool, String> {
    match s.trim().to_ascii_lowercase().as_str() {
        "true" => Ok(true),
        "false" => Ok(false),
        other => Err(format!("efx: attribute `{}` expects boolean (true/false), got `{}`", name, other)),
    }
}
