use std::{env, fs};
use std::path::{Path, PathBuf};

pub fn resolve_path(name: &str) -> Option<PathBuf> {
    let explicit_extension = name.ends_with(".efx");
    let mut search_roots = Vec::new();

    if let Ok(p) = env::var("EFX_UI_PATH") {
        search_roots.push(PathBuf::from(p));
    }

    if let Ok(manifest) = env::var("CARGO_MANIFEST_DIR") {
        search_roots.push(Path::new(&manifest).join("src").join("ui"));
    } else {
        search_roots.push(PathBuf::from("src/ui"));
    }

    for root in search_roots {
        if !root.exists() { continue; }

        if explicit_extension {
            let path = root.join(name);
            if path.exists() { return Some(path); }
        } else {
            if let Some(path) = find_recursive(&root, name) {
                return Some(path);
            }
        }
    }

    None
}

fn find_recursive(dir: &Path, name: &str) -> Option<PathBuf> {
    if !dir.is_dir() { return None; }

    let exact = dir.join(format!("{}.efx", name));
    if exact.exists() { return Some(exact); }

    let snake = to_snake_case(name);
    let snake_path = dir.join(format!("{}.efx", snake));
    if snake_path.exists() { return Some(snake_path); }

    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                if !name.contains('/') && !name.contains('\\') {
                    if let Some(found) = find_recursive(&path, name) {
                        return Some(found);
                    }
                }
            }
        }
    }

    None
}

fn to_snake_case(s: &str) -> String {
    let mut result = String::new();
    for (i, c) in s.char_indices() {
        if c.is_uppercase() {
            if i > 0 { result.push('_'); }
            result.push(c.to_ascii_lowercase());
        } else {
            result.push(c);
        }
    }
    result
}
