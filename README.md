# 🔧 patch‑ts

**Tree‑sitter‑backed universal patching CLI for AI agents**

![Version](https://img.shields.io/badge/version-1.18.0-blue) ![Rust](https://img.shields.io/badge/rust-2021%20edition-orange) ![License](https://img.shields.io/badge/license-MIT-green)

**patch‑ts** is a blazing‑fast, multi‑language code patching tool powered by [tree‑sitter](https://tree-sitter.github.io/tree-sitter/).  
It applies AI‑generated changes to 16+ programming languages, validates syntax, runs security audits, and tracks every operation cryptographically – all from a single binary.  
Designed for LLM agents, **patch‑ts** now supports **content‑based replacement** (no line number required), **SEARCH/REPLACE blocks**, **entity body patching**, **hunk‑fuzzy diff application**, **automatic search relaxation**, **AST‑node‑targeted MCP tools**, and **enhanced recall with strategy history**.

---

## ✨ Features

- 🧠 **AI‑first design** – Built for LLM agents: smart matching, content‑based patches, structured entity actions, token‑efficient recall, MCP integration.
- 🌳 **Tree‑sitter powered** – AST‑aware patching ensures syntax correctness for Rust, TypeScript, Python, Go, Java, and more.
- 🔍 **Smart matching** – Cascade of exact, anchor, anchor‑pair, ellipsis, Jaccard similarity, and normalized Levenshtein strategies.
- 📄 **Content‑based replacement** – Patch without line numbers: provide the old block, and patch‑ts locates and replaces it.
- 🔍 **SEARCH/REPLACE blocks** – Use the `<<< SEARCH … --- …` heredoc format for multi‑line replacements (no line numbers).
- 🧬 **Entity patching** – Target functions/classes by name (`--symbol`); new `--entity-body` replaces only the body of an entity.
- 🔄 **Hunk‑fuzzy diff** – Apply unified diffs even when line numbers are wrong; matches context lines against the file content.
- ♻️ **Auto‑relaxation** – When an exact search fails, cascading fallbacks (whitespace normalization, comment stripping, Jaccard similarity) automatically retry.
- 🛠️ **Balance & heal** – Fix unbalanced delimiters; intelligent heuristic repair with history‑guided strategies.
- 🛡️ **Validation gate** – Multi‑stage pipeline: syntax → compile → cross‑file → semdiff → security audit → OWASP → adversarial → LSP.
- 📜 **SCITT provenance** – Ed25519‑signed, hash‑chained audit trail; EU Cyber Resilience Act (CRA) attestations.
- 🎨 **Syntax highlighting** – Colour‑coded output with selectable themes (dark, light, deuteranopia, highcontrast).
- 📝 **Word‑level diff** – See exactly which tokens changed; gutter indicators ( `+` / `-` / `~` ) in CLI and TUI.
- 🖥️ **Interactive TUI** – Side‑by‑side original vs patched view, syntax highlighting, comment collection.
- 📊 **Scorecards** – Multi‑dimensional reliability score with colour‑gradient bars.
- 📄 **Markdown reports** – Gate and review results as structured Markdown for PR comments.
- 🔌 **MCP server** – Full Model Context Protocol support (stdio + HTTP) with 12 tools and 13 resources.
- 🔑 **Key management** – Generate and rotate Ed25519 keys for SCITT signing.
- 🌐 **Cross‑file impact** – Accurate call‑graph analysis via tree‑sitter queries.
- 🔁 **Recall mode** – Token‑efficient retry context generation (entropy, minimal, session, token budget, strategy history).
- 🛂 **MCP Gateway** – Policy‑based tool allow‑listing for agentic security.
- ⚡ **High performance** – Parallel multi‑file processing with Rayon; incremental parsing support.
- 🔄 **Atomic writes** – Patches are applied atomically with optional `.bak` backups.

---

## 📦 Installation

### Homebrew (macOS/Linux)
```bash
brew install elcoosp/tap/patch-ts
```

### Scoop (Windows)
```powershell
scoop bucket add elcoosp https://github.com/elcoosp/scoop-patch-ts
scoop install patch-ts
```

### Cargo (any platform)
```bash
cargo install patch-ts
```

### Pre‑built binaries
Grab the latest from [GitHub Releases](https://github.com/elcoosp/patch-ts/releases).

---

## 🚀 Quick Start

```bash
# Replace a line with fuzzy matching
patch-ts patch --file src/main.rs --line 10 --old "let port = 3000;" --new "let port = 8080;"

# Content‑based replacement (no line number required)
patch-ts patch --file src/main.rs --old "fn old() { do_thing(); }" --new "fn old() { do_other(); }"

# SEARCH/REPLACE block via heredoc
patch-ts patch --file src/main.rs --line 1 << 'EOF'
<<< SEARCH
fn old() {
    do_thing();
}
---
fn old() {
    do_other();
}
EOF

# Replace only the body of a function (entity)
patch-ts patch --file src/main.rs --symbol main --new "    let x = 42;\n    println!(\"{}\", x);\n" --entity-body

# Apply a unified diff (hunk‑fuzzy – line numbers are ignored)
pbpaste | patch-ts patch --file src/lib.rs --diff

# Balance unbalanced delimiters
patch-ts balance --file src/broken.rs --apply

# Explain a syntax error
patch-ts explain --file src/main.rs --line 42

# Run a multi‑stage validation gate
patch-ts gate --file src/main.rs --stages syntax,compile,lsp --json
```

---

## 🌍 Supported Languages

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

Language detection is automatic based on file extension.  
Unsupported files (`.toml`, `.json`, etc.) fall back to full‑file string replacement.

---

## 📖 Commands

### `patch`

Apply a source change using various strategies.  
**New in v1.18:** Content‑based replacement, SEARCH/REPLACE blocks, entity body replacement, hunk‑fuzzy diff, auto‑relaxation.

#### Traditional line‑based patch
```bash
patch-ts patch --file <FILE> --line <N> --old "<EXPECTED>" --new "<REPLACEMENT>"
```

#### Content‑based replacement (no `--line` required)
```bash
patch-ts patch --file <FILE> --old "<SEARCH_BLOCK>" --new "<REPLACEMENT>"
# The tool searches for SEARCH_BLOCK as a substring anywhere in the file.
# If not found, it automatically tries relaxed matching (whitespace, comments, Jaccard).
```

#### SEARCH/REPLACE block (via stdin or heredoc)
```bash
patch-ts patch --file <FILE> --line <N> << 'EOF'
<<< SEARCH
<original code block>
---
<replacement code block>
EOF
# Note: --line is still required for the heredoc path (for compatibility);
# the actual line number is ignored when using SEARCH/REPLACE format.
```

#### Entity (symbol) patching
```bash
patch-ts patch --file <FILE> --symbol <NAME> --new "<ENTITY_BODY>"
# Replaces the entire entity (function, class, etc.) with the new text.
```

#### Entity body‑only replacement (new in v1.18)
```bash
patch-ts patch --file <FILE> --symbol <NAME> --new "<BODY>" --entity-body
# Replaces only the body of the entity, keeping its signature and outer braces intact.
```

#### Unified diff with hunk‑fuzzy matching
```bash
patch-ts patch --file <FILE> --diff < diff.patch
# Line numbers in the diff are ignored; context lines are used to locate the change.
```

#### Other patch actions
```bash
patch-ts patch --file <FILE> --delete <LINE> --expect "<EXACT_LINE>"
patch-ts patch --file <FILE> --after <LINE> --content "<MULTI_LINE_STRING>"
patch-ts patch --files "src/**/*.rs" --marker "PATCH‑ME" --new "<NEW_LINE>"
```

#### Key patch options
| Flag | Description |
|------|-------------|
| `--file <FILE>` | File to patch |
| `--files <GLOB>` | Multi‑file patching |
| `--line <N>` | Line number (optional; not needed for content‑based patches) |
| `--old <TEXT>` | Expected old content |
| `--new <TEXT>` | Replacement content |
| `--symbol <NAME>` | Replace entity by name |
| `--entity-body` | Replace only the body of an entity (requires `--symbol`) |
| `--marker <TEXT>` | Replace line containing a unique comment marker |
| `--diff` | Apply unified diff from stdin |
| `--fuzz <N>` | Search radius (default 5) |
| `--confidence <F>` | Match confidence threshold (default 0.9) |
| `--fix-indent` | Auto‑apply original indentation style |
| `--cross-file` | Check for callers after rename |
| `--dry-run` | Print patched result without writing |
| `--force` | Skip AST validation |
| `--no-backup` | Skip `.bak` creation |
| `--json` | Machine‑readable output |
| `--validate-first` | Reject patch if it introduces syntax errors |
| `--fix-headers` | Automatically correct hunk headers in unified diffs |
| `--no-compile-check` | Skip compilation check after patching |
| `--compile-timeout <SECS>` | Timeout for compilation check (default 30) |
| `--agent <NAME>`, `--model <NAME>` | Record provenance |
| `--no-provenance` | Skip provenance recording |
| `--no-sanitize` | Skip sanitization of input (e.g., LLM fences) |
| `--no-ellipsis` | Disable ellipsis pattern matching |
| `--uniqueness-weight <F>` | Uniqueness weight for matching (default 0.2) |
| `--strict-whitespace` | Do not normalize whitespace in diffs |
| `--serial` | Process multi‑file patches sequentially |

---

### Other commands

(`balance`, `heal`, `explain`, `fix`, `git`, `semdiff`, `provenance`, `gate`, `score`, `index`, `evolve`, `review`, `attest`, `verify`, `impact`, `entity`, `key`, `recall`, `watch`, `mcp`, `mcp‑http`, `mcp‑gateway`) – updated with new flags and capabilities as described in the original README, plus:

#### `recall` (v1.18 enhancements)
- **Strategy history**: `patch‑ts recall` now suggests the strategy (e.g., "increase_fuzz", "use_symbol") with the highest historical success rate for the given error code.
- Additional options: `--error-message`, `--entropy`, `--entropy-threshold`, `--pre-fetch`, `--minimal`, `--session`, `--max-tokens`.

#### MCP tools (new in v1.18)
Three new tools for AST‑node‑targeted edits:
- **`replace_node`** – replace an AST node matching a tree‑sitter query
- **`delete_node`** – delete an AST node matching a tree‑sitter query
- **`insert_before_node`** – insert text before an AST node matching a query

Example usage via MCP:
```json
{
  "method": "tools/call",
  "params": {
    "name": "replace_node",
    "arguments": {
      "file": "src/main.rs",
      "query": "(function_item name: (identifier) @name (#eq? @name \"old\"))",
      "new": "fn new() { /* ... */ }"
    }
  }
}
```

---

## 🔌 MCP Integration (updated)

**Tools (12):**  
`patch`, `balance`, `explain`, `impact`, `semdiff`, `entity_list`, `entity_replace`, `gate`, `recall`,  
**`replace_node`**, **`delete_node`**, **`insert_before_node`**

**Resources (13):** unchanged.

---

## 🎨 Visual Features (unchanged)

---

## 🔐 SCITT & Supply Chain (unchanged)

---

## 🧪 Testing & CI (unchanged)

---

## 🚧 Upcoming (post‑v1.18)

- **Persistent project index** for instant symbol queries.
- **Predictive context engine** – `patch‑ts context` for minimal, high‑value agent context.
- **WASM productionisation** – npm package for browser‑based usage.
- **LSP client improvements** – support for more language servers.
- **More language‑specific entity body ranges** (currently Rust‑focused).
- **Query‑based entity patching in CLI** (currently MCP‑only).

---

## 🤝 Contributing

We welcome contributions! See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.  
Open an issue to discuss new features or report bugs.

---

## 📄 License

MIT © [elcoosp](https://github.com/elcoosp)

---

*Built with Rust, tree‑sitter, and a lot of ☕*
