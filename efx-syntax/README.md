
# EFx Syntax (TextMate / VS Code / JetBrains)

Syntax highlighting for EFx templates:

- Files: `*.efx`
- Basic constructions: tags `<Window>`, `<Label>`, attributes and strings with embedded expressions (`self.*`, `ctx.*`, bool, numbers, hex colors).
- Snippets for basic tags.

## Installation

### VS Code
1. Open the command palette → `Developer: Install from VSIX...` (See below for how to build a VSIX).
2. Open `.efx` - highlighting will be activated.

Build VSIX:
```bash
# Inside the package folder
vsce package   # or: npx @vscode/vsce package
```
Will create a file like `efx-syntax-0.5.0.vsix`, which can be installed via the palette.

### JetBrains / RustRover
1. Preferences → Editor → **TextMate Bundles** → `+` → select the root folder of the package (where `syntaxes/efx.tmLanguage.json` is located).
2. If necessary: Preferences → Editor → **File Types** → add `*.efx` as text type or bind automatically.
3. Open `.efx` - highlighting will be activated.

## Further

- EFx injection into Rust string literals (for `efx!(..., r#"..."#)`) — by a separate grammar-injector.
- Tree-sitter grammar for more precise highlighting and structuring.
- LSP server for diagnostics, completion and refactoring.

License: [MIT](LICENSE)