# patch-ts

Tree-sitter-aware patching CLI for AI agents and developers.
Safely apply patches to **Rust, TypeScript, and JavaScript** source files with fuzzy matching, auto‑repair, and marker‑based targeting.

## Features

- **Multi‑language support** – Rust (`.rs`), TypeScript (`.ts`, `.tsx`, `.mts`, `.cts`), and JavaScript (`.js`, `.jsx`, `.mjs`, `.cjs`).
- **AST‑aware patching** – Validates syntax after each patch using tree‑sitter.
- **Fuzzy matching** – Locates target lines/blocks even if line numbers have drifted or whitespace differs.
- **Auto‑repair** – Automatically fixes simple syntax errors (e.g., extra braces) introduced by a patch.
- **Marker‑based targeting** – Replace AST nodes anchored by `// PATCH-ME: <id>` comments.
- **Multiple patch methods**:
  - Exact or fuzzy literal replacement (single‑ or multi‑line)
  - Delete a line after content verification
  - Insert lines after a given line
  - Apply unified diffs (with optional fuzzy context)
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

Language is automatically detected from the file extension. No `--lang` flag needed.

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

Replace a specific line or block of lines.

```bash
patch-ts patch --file <FILE> --line <LINE> --old <EXPECTED> --new <NEW> [OPTIONS]
```

- `--line` : Target line number (1‑indexed).
- `--old`  : Expected current content (single line, or multi‑line with `\n`).
- `--new`  : Replacement content.
- `--fuzz` : Search radius for fuzzy matching (default: 5).

**Heredoc alternative** (for multi‑line content):

```bash
patch-ts patch --file src/lib.rs --line 10 <<'EOF'
<<<
fn old() {
    println!("old");
}
---
fn new() {
    println!("new");
}
EOF
```

### 2. Delete a Line

```bash
patch-ts patch --file <FILE> --delete <LINE> --expect <EXPECTED_CONTENT> [OPTIONS]
```

Verifies that line `<LINE>` contains `<EXPECTED_CONTENT>` (exact match, trimmed) before deleting it.

### 3. Insert After a Line

```bash
patch-ts patch --file <FILE> --after <LINE> --content <NEW_CONTENT> [OPTIONS]
```

Inserts `<NEW_CONTENT>` (can be multi‑line) immediately after line `<LINE>`.

### 4. Apply Unified Diff

```bash
patch-ts patch --file <FILE> --diff [--fuzz <N>] < diff.patch
```

Reads a unified diff from stdin and applies it. If `--fuzz` is provided and the exact context doesn't match, `patch-ts` will attempt to locate the block using fuzzy token matching.

### 5. Marker‑Based Replacement

```bash
patch-ts patch --file <FILE> --marker <ID> --new <NEW_CONTENT> [OPTIONS]
```

Finds a comment `// PATCH-ME: <ID>` (or `/* PATCH-ME: <ID> */`) and replaces the **next AST node** (e.g., function, struct) with `<NEW_CONTENT>`.

### Common Options for `patch`

| Option              | Description |
|---------------------|-------------|
| `--fuzz <N>`        | Search radius for fuzzy line matching (default: 5). |
| `--dry-run`         | Print the resulting file content instead of writing. |
| `--force`           | Skip AST validation (apply even if syntax error). |
| `--no-backup`       | Do not create a `.bak` backup file. |
| `--no-auto-repair`  | Disable automatic repair of simple syntax errors. |
| `--json`            | Output JSON diagnostics instead of human‑readable text. |

## The `balance` Command

Attempts to fix unbalanced delimiters (`{}`, `()`, `[]`) by removing extra or inserting missing ones.

```bash
patch-ts balance --file <FILE> [--apply] [--function <NAME>] [--no-backup] [--json]
```

- `--apply` : Actually modify the file (default is dry‑run, prints what would change).
- `--function <NAME>` : Restrict repairs to the body of a specific function.

## The `explain` Command

Provides a human‑readable (or JSON) explanation of a syntax error at a given line.

```bash
patch-ts explain --file <FILE> --line <LINE> [--json]
```

## JSON Output

When `--json` is used, `patch-ts` prints a JSON object to stdout.
Success response:

```json
{ "success": true, "error": null }
```

Error response:

```json
{
  "success": false,
  "error": {
    "code": "patch_ts::syntax_error",
    "message": "patch introduces syntax error: ...",
    "span": { "file": "src/main.rs", "line": 42, "column": 1 },
    "context": "Extra closing brace detected.",
    "suggestion": "Run `patch-ts balance` to attempt automatic fix",
    "best_score": null,
    "best_match_line": null,
    "candidates": null
  }
}
```

## Examples

### Fuzzy replace across languages

```bash
# TypeScript
patch-ts patch --file src/app.ts --line 42 --old "const x = 1;" --new "const x = 2;" --fuzz 10

# JavaScript
patch-ts patch --file dist/bundle.js --line 15 --old "var a = 1;" --new "let a = 2;"
```

### Balance a TypeScript file

```bash
patch-ts balance --file src/component.tsx --apply
```

### Fix an extra brace in a JavaScript function

```bash
patch-ts balance --file lib/utils.js --function calculateTotal --apply
```

## Exit Codes

- `0` – Success.
- `1` – Error (patch failed, validation error, etc.).

## Limitations

- Missing parenthesis/bracket insertion in TypeScript/JavaScript may not work for all edge cases (improvements planned for v0.5.0).
- `balance` currently handles one error per iteration; multiple errors may require multiple runs.
- Fuzzy diff apply is a stub; full hunk parsing is not yet implemented.

## License

MIT
