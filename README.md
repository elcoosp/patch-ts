# `patch-ts`

**Tree‑sitter‑backed patching CLI for AI agents—safe, structural, and scriptable.**

[![Crates.io](https://img.shields.io/crates/v/patch-ts)](https://crates.io/crates/patch-ts)
[![License](https://img.shields.io/badge/license-MIT-blue)](LICENSE)

`patch-ts` replaces brittle `sed`‑based patching in AI‑assisted coding workflows. It accepts line‑based edit commands from an LLM, verifies expected content before modification, validates the result with a Rust syntax tree, and provides actionable diagnostics when things go wrong. When a patch leaves the code structurally broken, `patch-ts` can **repair** unbalanced delimiters and explain syntax errors with AST‑level context.

> **Why this exists:** LLMs often output patch commands using line numbers from compiler errors. `sed -i '234s/old/new/'` fails silently if the file changed, and it can't detect or fix brace mismatches. `patch-ts` adds a safety net and a feedback loop that helps the AI converge on a correct fix.

---

## Features

- **Line‑based patch commands** with heredoc‑style expected/actual blocks—no regex escaping nightmares.
- **Fuzzy matching** (`--fuzz N`) locates the intended line even when the file has drifted.
- **Tree‑sitter validation** aborts edits that introduce syntax errors (unless `--force`).
- **Structural repair** commands:
  - `patch-ts balance` – automatically fix extra/missing braces, parens, or brackets.
  - `patch-ts explain` – show AST context and likely cause of a syntax error at a given line.
- **Dry‑run** mode (`--dry-run`) outputs a unified diff without modifying the file.
- **Atomic writes** and automatic `.bak` backups.
- **LLM‑friendly diagnostics** with source snippets and suggestions (optional JSON output).

---

## Installation

### From crates.io (requires Rust)

```bash
cargo install patch-ts
```

### Pre‑built binaries

Download the latest release for your platform from the [Releases](https://github.com/your-org/patch-ts/releases) page.

### From source

```bash
git clone https://github.com/your-org/patch-ts
cd patch-ts
cargo build --release
```

---

## Quick Start

### 1. Apply a type annotation fix (safe replace)

```bash
patch-ts --file src/state.rs --line 234 <<'EOF'
<<<
cx.spawn(|cx| async move {
---
cx.spawn(|cx: /* Type */| async move {
EOF
```

- The tool verifies line 234 exactly matches the block after `<<<`.
- If the content matches, it replaces it with the block after `---` and validates the AST.
- If the expected content isn't found, it prints a diff and aborts.

### 2. Delete an extra closing brace (with verification)

```bash
patch-ts --file src/state.rs --delete 260 --expect "        }"
```

- Only deletes line 260 if its trimmed content is exactly `}`.
- AST validation prevents deleting a brace that would unbalance the file.

### 3. Auto‑balance mismatched delimiters

```bash
patch-ts balance --file src/state.rs --dry-run
```

- Detects extra or missing braces, parens, or brackets.
- Outputs the proposed fix. Remove `--dry-run` to apply.

### 4. Explain a confusing compiler error

```bash
patch-ts explain --file src/state.rs --line 566
```

Output:

```
Line 566: `}`

AST context:
- This closing brace is at the top level of the file.
- The preceding function `RouterState::navigate` ends at line 565.
- No opening brace matches this closing brace.

Likely cause:
  An extra `}` was inserted earlier, causing the parser to treat the remainder
  of the file as outside any function. The actual extra brace is likely before
  line 566.

Suggestion:
  Run `patch-ts balance --function navigate` to automatically remove the extra brace.
```

---

## Command Reference

| Command                           | Description |
|-----------------------------------|-------------|
| `patch-ts --file <FILE> --line <N> [--fuzz N] <<'EOF' ...` | Replace a block of lines with verification. |
| `patch-ts --file <FILE> --delete <N> --expect <CONTENT>` | Delete a line after verifying its content. |
| `patch-ts --file <FILE> --after <N> --content <TEXT>` | Insert text after a line. |
| `patch-ts --diff <<'EOF' ...` | Apply a unified diff (like `git apply`). |
| `patch-ts balance --file <FILE> [--function <NAME>] [--dry-run]` | Fix unbalanced delimiters. |
| `patch-ts explain --file <FILE> --line <N> [--json]` | Show AST‑based diagnosis of a syntax error. |
| `patch-ts --help` | Show all options. |

All patch commands support:
- `--dry-run` – preview changes without writing.
- `--no-backup` – skip creating a `.bak` file.
- `--force` – skip AST validation (use with caution).

---

## How It Works

`patch-ts` uses **[tree-sitter](https://tree-sitter.github.io/)** to parse Rust source code into a concrete syntax tree. Before writing any change, it:

1. **Applies the edit in‑memory** to a temporary buffer.
2. **Parses the buffer** and compares the new AST against the original.
3. **Aborts** if new `ERROR` nodes appear (or if the expected content wasn't found).
4. **Writes atomically** via a tempfile + rename, with an optional `.bak` backup.

The `balance` command analyzes `ERROR` nodes to identify delimiter mismatches and proposes minimal deletions/insertions. `explain` walks the AST to provide context about an error location.

This architecture ensures that even when an LLM mis‑predicts line numbers, the tool fails safely with actionable feedback—enabling a collaborative, iterative patching loop.

---

## Why Not `sed`?

| Problem                               | `sed`                           | `patch-ts`                                      |
|---------------------------------------|---------------------------------|-------------------------------------------------|
| Line numbers shift                    | Silent corruption               | Fuzz search or abort with clear mismatch        |
| Accidental syntax break               | Undetected                      | AST validation prevents write (unless `--force`) |
| Unbalanced braces after bad patch     | Manual fix required             | `balance` command auto‑repairs                  |
| LLM needs context to correct          | Only raw error from compiler    | `explain` gives AST‑level guidance              |
| Multi‑line replace with escaping      | Painful                         | Heredoc literal blocks                          |

---

## Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.  
The project is built with Rust, `tree-sitter`, and `clap`.

---

## License

MIT — see [LICENSE](LICENSE) for details.

---

*Built for the AI‑assisted coding era. Let your LLM patch fearlessly.*
