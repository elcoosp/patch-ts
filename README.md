# patch-ts

**Tree‑sitter‑aware patching CLI for AI agents and developers.**
Safely apply patches to **16+ languages** with fuzzy matching, AST validation, auto‑repair, marker‑based targeting, multi‑file bulk operations, parallel processing, an interactive TUI, watch mode, and a WASM plugin system.

---

## Features

- **Multi‑language support** – Rust, TypeScript, JavaScript, Python, Go, Ruby, PHP, HTML, XML, C, C++, Java, C#, Swift, Scala, Zig, and more. Language is detected automatically by file extension.
- **AST‑aware patching** – Validates syntax after each patch using tree‑sitter, preventing silent corruption.
- **Fuzzy matching** – Locates target lines/blocks even when line numbers have drifted (configurable fuzz radius).
- **Auto‑repair** – Automatically fixes simple syntax errors (e.g., unbalanced delimiters) introduced by a patch, using a minimum‑cost search for the smallest valid edit.
- **Context‑aware insertion** – Uses parent node information to place missing delimiters accurately.
- **Batch repair** – Fixes multiple delimiter errors in a single pass with built‑in rollback protection.
- **Marker‑based targeting** – Replace AST nodes anchored by `// PATCH-ME: <id>` comments.
- **Multi‑file patches** – Apply the same change across many files using glob patterns (`--files "**/*.rs"`).
- **Parallel processing** – Blazing‑fast multi‑file operations powered by `rayon` (optional `--serial` flag).
- **Interactive TUI** – Review and approve patches visually with `--tui`.
- **Watch mode** – Automatically apply queued patches when files change (`--watch`).
- **Configuration file** – Set project‑specific defaults in `patch-ts.toml`.
- **WASM plugin system** – Extend repair logic with custom WebAssembly plugins (sandboxed).
- **Rich diagnostics** – Language‑specific error messages with suggestions, available in human‑readable or JSON format.
- **Dry‑run & backup** – Preview changes and keep automatic `.bak` backups.

---

## Installation

### From crates.io (recommended)

```
cargo install patch-ts
```

### From source

```
git clone https://github.com/elcoosp/patch-ts
cd patch-ts
cargo build --release
```

*(Homebrew and Scoop packages are available; see [Distribution](#distribution) below.)*

---

## Supported Languages

| Language | Extensions |
|----------|------------|
| Rust | `.rs` |
| TypeScript | `.ts`, `.tsx`, `.mts`, `.cts` |
| JavaScript | `.js`, `.jsx`, `.mjs`, `.cjs` |
| Python | `.py`, `.pyi` |
| Go | `.go` |
| Ruby | `.rb` |
| PHP | `.php` |
| HTML | `.html`, `.htm` |
| XML | `.xml` |
| C | `.c`, `.h` |
| C++ | `.cpp`, `.cc`, `.cxx`, `.hpp` |
| Java | `.java` |
| C# | `.cs` |
| Swift | `.swift` |
| Scala | `.scala` |
| Zig | `.zig` |

Language is automatically detected – no need for a `--lang` flag.

---

## Usage

```
patch-ts <COMMAND> [OPTIONS]
```

### Commands

| Command   | Description |
|-----------|-------------|
| `patch`   | Apply a patch using literal replacement, deletion, insertion, unified diff, or marker targeting. |
| `balance` | Detect and fix unbalanced delimiters (`{}`, `()`, `[]`). Supports multi‑file and function scoping (Rust). |
| `explain` | Explain a syntax error at a given line with language‑specific context. |
| `help`    | Print help information. |

---

## The `patch` Command

Applies changes to one or more source files. You must specify **one** of the following operation types.

### 1. Literal Replacement (exact or fuzzy)

```
patch-ts patch --file <FILE> --line <LINE> --old <EXPECTED> --new <NEW> [OPTIONS]
# or multi‑file
patch-ts patch --files "<GLOB>" --line <LINE> --old <EXPECTED> --new <NEW> [OPTIONS]
```

- `--line` : Target line number (1‑indexed).
- `--old`  : Expected current content.
- `--new`  : Replacement content.
- `--fuzz` : Search radius for fuzzy matching (default: 5).

### 2. Delete a Line

```
patch-ts patch --file <FILE> --delete <LINE> --expect <EXPECTED_CONTENT>
```

### 3. Insert After a Line

```
patch-ts patch --file <FILE> --after <LINE> --content <NEW_CONTENT>
```

### 4. Apply Unified Diff

```
patch-ts patch --file <FILE> --diff [--fuzz <N>] < diff.patch
```

### 5. Marker‑Based Replacement

```
patch-ts patch --file <FILE> --marker <ID> --new <NEW_CONTENT>
```

### Common Options for `patch`

| Option              | Description |
|---------------------|-------------|
| `--files <GLOB>`    | Apply the patch to all files matching the glob pattern. |
| `--fuzz <N>`        | Search radius for fuzzy line matching (default: 5). |
| `--dry-run`         | Print the resulting file content instead of writing. |
| `--force`           | Skip AST validation. |
| `--no-backup`       | Do not create a `.bak` backup file. |
| `--no-auto-repair`  | Disable automatic repair of simple syntax errors. |
| `--json`            | Output JSON diagnostics. |
| `--serial`          | Process files sequentially instead of in parallel. |
| `--plugin <PATH>`   | Use a custom WASM plugin for repair. |
| `--tui`             | Launch an interactive terminal UI to review the patch. |

---

## The `balance` Command

Attempts to fix unbalanced delimiters (`{}`, `()`, `[]`) using a **minimum‑cost repair engine** that finds the smallest set of edits to restore valid syntax. Automatically rolls back if a repair would introduce new errors.

```
patch-ts balance --file <FILE> [--files <GLOB>] [--apply] [--function <NAME>] [--no-backup] [--json] [--max-cost <N>]
```

- `--apply` : Actually modify the file (default is dry‑run).
- `--function <NAME>` : Restrict repairs to a specific function (Rust only).
- `--files <GLOB>` : Balance multiple files at once.
- `--max-cost <N>`  : Maximum number of edits the repair engine may perform (default: 10). Increase for more complex fixes.
- `--serial` : Disable parallel processing.

---

## The `explain` Command

Provides human‑readable or JSON explanation of a syntax error, with language‑specific suggestions.

```
patch-ts explain --file <FILE> --line <LINE> [--json]
```

---

## Advanced Features

### Configuration File (`patch-ts.toml`)

Place a `patch-ts.toml` file in your project root (or any parent directory) to set default options:

```
fuzz = 5                  # default fuzz radius
backup = true             # create .bak files by default
auto_repair = true        # enable auto‑repair
similarity_threshold = 0.9
```

CLI flags override config file settings.

### Watch Mode (`--watch`)

Monitor files for changes and apply queued patches automatically:

```
patch-ts watch --path src/ --queue patches.json
```

### Interactive TUI (`--tui`)

Review patches in a terminal UI with syntax‑highlighted diffs before applying:

```
patch-ts patch --file src/main.rs --tui ...
```

### WASM Plugins

Extend `patch-ts` with custom repair logic written in any language that compiles to WebAssembly. Place `.wasm` plugins in `~/.config/patch-ts/plugins/` and activate them with `--plugin <name>`.

See the [Plugin Development Guide](docs/plugin-guide.md) for details.

---

## JSON Output

When `--json` is used, `patch-ts` prints a machine‑readable JSON object.

**Success:**
```
{ "success": true, "error": null }
```

**Error:**
```
{
  "success": false,
  "error": {
    "code": "patch_ts::content_mismatch",
    "message": "expected line 42 to contain 'x' but found 'y'",
    "span": { "file": "src/main.rs", "line": 42, "column": 1 },
    "suggestion": "Try increasing --fuzz radius"
  }
}
```

---

## Distribution

- **crates.io**: `cargo install patch-ts`
- **Homebrew**: `brew install elcoosp/tap/patch-ts`
- **Scoop**: `scoop install patch-ts`

Pre‑built binaries are also available on the [GitHub Releases](https://github.com/elcoosp/patch-ts/releases) page.

---

## Examples

### Fuzzy replace across multiple TypeScript files

```
patch-ts patch --files "src/**/*.ts" --line 42 --old "const x = 1;" --new "const x = 2;" --fuzz 10
```

### Balance all Python files in a project (parallel)

```
patch-ts balance --files "**/*.py" --apply
```

### Use a custom plugin to repair company‑specific DSL

```
patch-ts balance --file main.custom --plugin my_dsl --apply
```

### Review a patch interactively before applying

```
patch-ts patch --file src/lib.rs --line 10 --old "fn old()" --new "fn new()" --tui
```

---

## Exit Codes

- `0` – Success.
- `1` – General error.
- `2` – Content mismatch / patch not applied.
- `3` – Syntax error introduced.

---

## Documentation

- [Vision & Roadmap](docs/patch-ts-vision.md)
- [Plugin Guide](docs/plugin-guide.md)
- [Architecture Overview](docs/patch-ts-archi.md)
- [Full Command Reference](docs/patch-ts-srs.md)

---

## License

MIT

---

**patch-ts** – Safe, intelligent patching for every language.
