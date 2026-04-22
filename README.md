# patch-ts

Tree-sitter-aware patching CLI for AI agents and developers.
Safely apply patches to **Rust, TypeScript, and JavaScript** source files with fuzzy matching, auto‑repair, marker‑based targeting, and context‑aware delimiter insertion.

## Features

- **Multi‑language support** – Rust (`.rs`), TypeScript (`.ts`, `.tsx`, `.mts`, `.cts`), and JavaScript (`.js`, `.jsx`, `.mjs`, `.cjs`).
- **AST‑aware patching** – Validates syntax after each patch using tree‑sitter.
- **Fuzzy matching** – Locates target lines/blocks even if line numbers have drifted.
- **Auto‑repair** – Automatically fixes simple syntax errors (e.g., extra braces) introduced by a patch.
- **Context‑aware insertion** – Uses parent node information to place missing delimiters accurately (v0.5.0).
- **Batch repair** – Fixes multiple delimiter errors in a single pass (v0.5.0).
- **Enhanced diagnostics** – Language‑specific error messages with actionable suggestions for TS/JS (v0.5.0).
- **Marker‑based targeting** – Replace AST nodes anchored by `// PATCH-ME: <id>` comments.
- **JSON output** for easy integration with tools and agents.
- **Dry‑run mode** to preview changes without modifying files.
- **Automatic backups** (can be disabled).

## Installation

```bash
cargo install --path .
```

Or build from source:

```bash
git clone https://github.com/elcoosp/patch-ts
cd patch-ts
cargo build --release
```

## Supported Languages

| Language | Extensions |
|----------|------------|
| Rust | `.rs` |
| TypeScript | `.ts`, `.tsx`, `.mts`, `.cts` |
| JavaScript | `.js`, `.jsx`, `.mjs`, `.cjs` |

Language is automatically detected from the file extension.

## Usage

```bash
patch-ts <COMMAND> [OPTIONS]
```

### Commands

| Command   | Description |
|-----------|-------------|
| `patch`   | Apply a patch to a file using one of several methods. |
| `balance` | Detect and fix unbalanced delimiters (`{}`, `()`, `[]`). |
| `explain` | Explain a syntax error at a given line. |
| `help`    | Print help information. |

## The `patch` Command

Applies changes to a source file. You must specify **one** of the following operation types:

### 1. Literal Replacement (exact or fuzzy)

```bash
patch-ts patch --file <FILE> --line <LINE> --old <EXPECTED> --new <NEW> [OPTIONS]
```

- `--line` : Target line number (1‑indexed).
- `--old`  : Expected current content.
- `--new`  : Replacement content.
- `--fuzz` : Search radius for fuzzy matching (default: 5).

### 2. Delete a Line

```bash
patch-ts patch --file <FILE> --delete <LINE> --expect <EXPECTED_CONTENT> [OPTIONS]
```

### 3. Insert After a Line

```bash
patch-ts patch --file <FILE> --after <LINE> --content <NEW_CONTENT> [OPTIONS]
```

### 4. Apply Unified Diff

```bash
patch-ts patch --file <FILE> --diff [--fuzz <N>] < diff.patch
```

### 5. Marker‑Based Replacement

```bash
patch-ts patch --file <FILE> --marker <ID> --new <NEW_CONTENT> [OPTIONS]
```

### Common Options for `patch`

| Option              | Description |
|---------------------|-------------|
| `--fuzz <N>`        | Search radius for fuzzy line matching (default: 5). |
| `--dry-run`         | Print the resulting file content instead of writing. |
| `--force`           | Skip AST validation. |
| `--no-backup`       | Do not create a `.bak` backup file. |
| `--no-auto-repair`  | Disable automatic repair of simple syntax errors. |
| `--json`            | Output JSON diagnostics. |

## The `balance` Command

Attempts to fix unbalanced delimiters (`{}`, `()`, `[]`) using context‑aware insertion and batch repair.

```bash
patch-ts balance --file <FILE> [--apply] [--function <NAME>] [--no-backup] [--json]
```

- `--apply` : Actually modify the file (default is dry‑run).
- `--function <NAME>` : Restrict repairs to a specific function (Rust only).

## The `explain` Command

Provides human‑readable or JSON explanation of a syntax error, with language‑specific suggestions.

```bash
patch-ts explain --file <FILE> --line <LINE> [--json]
```

## JSON Output

When `--json` is used, `patch-ts` prints a JSON object.
Success response:

```json
{ "success": true, "error": null }
```

Error response includes `code`, `message`, `span`, `context`, and `suggestion`.

## Examples

### Fuzzy replace across languages

```bash
# TypeScript
patch-ts patch --file src/app.ts --line 42 --old "const x = 1;" --new "const x = 2;" --fuzz 10

# JavaScript
patch-ts patch --file dist/bundle.js --line 15 --old "var a = 1;" --new "let a = 2;"
```

### Balance a TypeScript file with context‑aware insertion

```bash
patch-ts balance --file src/component.tsx --apply
```

### Get language‑specific diagnostic

```bash
patch-ts explain --file lib/utils.js --line 10 --json
```

## Exit Codes

- `0` – Success.
- `1` – Error.

## Limitations

- Batch repair may not handle all overlapping error cases perfectly (improvements planned for v0.6.0).
- Function scoping (`--function`) currently only supported for Rust.

## License

MIT
