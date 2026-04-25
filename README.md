```markdown
# 🔧 patch‑ts

**Tree‑sitter‑backed universal patching CLI for AI agents**

![Version](https://img.shields.io/badge/version-1.17.0-blue) ![Rust](https://img.shields.io/badge/rust-2021%20edition-orange) ![License](https://img.shields.io/badge/license-MIT-green)

**patch‑ts** is a blazing‑fast, multi‑language code patching tool powered by [tree‑sitter](https://tree-sitter.github.io/tree-sitter/).  
It applies AI‑generated changes to 16+ programming languages, validates syntax, runs security audits, and tracks every operation cryptographically – all from a single binary.  
Whether you’re an AI agent (MCP, LSP) or a human developer, patch‑ts gives you safe, explainable, and visually rich code transformations.

---

## ✨ Features

- 🧠 **AI‑first design** – Built for LLM agents: smart matching, structured action spaces, token‑efficient recall, MCP integration.
- 🌳 **Tree‑sitter powered** – AST‑aware patching ensures syntax correctness for Rust, TypeScript, Python, Go, Java, and more.
- 🔍 **Smart matching** – Cascade of exact, anchor, anchor‑pair, ellipsis, Jaccard similarity, and normalized Levenshtein strategies.
- 🛠️ **Balance & heal** – Fix unbalanced delimiters; intelligent heuristic repair with history‑guided strategies.
- 🛡️ **Validation gate** – Multi‑stage pipeline: syntax → compile → cross‑file → semdiff → security audit → OWASP → adversarial → LSP.
- 📜 **SCITT provenance** – Ed25519‑signed, hash‑chained audit trail; EU Cyber Resilience Act (CRA) attestations.
- 🎨 **Syntax highlighting** – Colour‑coded output with selectable themes (dark, light, deuteranopia, highcontrast).
- 📝 **Word‑level diff** – See exactly which tokens changed; gutter indicators ( `+` / `-` / `~` ) in CLI and TUI.
- 🖥️ **Interactive TUI** – Side‑by‑side original vs patched view, syntax highlighting, comment collection.
- 📊 **Scorecards** – Multi‑dimensional reliability score with colour‑gradient bars.
- 📄 **Markdown reports** – Gate and review results as structured Markdown for PR comments.
- 🔌 **MCP server** – Full Model Context Protocol support (stdio + HTTP) with 9 tools and 13 resources.
- 🧬 **Entity patching** – Target functions/classes by name (`--symbol` or `entity replace`).
- 🔑 **Key management** – Generate and rotate Ed25519 keys for SCITT signing.
- 🌐 **Cross‑file impact** – Accurate call‑graph analysis via tree‑sitter queries.
- 🔁 **Recall mode** – Token‑efficient retry context generation (entropy, minimal, session, token budget).
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

# Apply a unified diff from stdin
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

```bash
patch-ts patch --file <FILE> --line <N> --old "<EXPECTED>" --new "<REPLACEMENT>"
patch-ts patch --file <FILE> --diff < diff.patch
patch-ts patch --files "src/**/*.rs" --line 10 --old "foo" --new "bar"
patch-ts patch --file <FILE> --symbol my_function --new "fn my_function() { ... }"
patch-ts patch --file <FILE> --delete 5 --expect "old line"
patch-ts patch --file <FILE> --after 5 --content "new line\nnew line 2"
```

**Key options**
- `--fuzz <N>` – search radius (default 5)
- `--confidence <0.0‑1.0>` – match confidence threshold (default 0.9)
- `--fix‑indent` – auto‑apply original indentation style
- `--cross‑file` – check for callers after rename
- `--dry‑run` – print patched result without writing
- `--force` – skip AST validation
- `--no‑backup` – skip `.bak` creation
- `--json` – machine‑readable output
- `--theme <THEME>` – syntax highlighting theme (dark, light, deuteranopia, highcontrast)
- `--validate‑first` – reject patch if it would introduce syntax errors

---

### `balance`

Fix unbalanced delimiters.

```bash
patch-ts balance --file <FILE> --apply
patch-ts balance --files "*.rs" --apply
patch-ts balance --file <FILE> --function my_func  # scoped to function (Rust only)
patch-ts balance --file <FILE> --plugin plugin.wasm
```

**Options**  
`--max‑cost <N>` – maximum repair cost (default 10)  
`--json` – output list of insert/delete actions

---

### `heal` (v1.15.0)

Intelligent delimiter repair with heuristic‑guided search.

```bash
patch-ts heal --file broken.rs --heuristic language-aware --apply
patch-ts heal --file broken.rs --heuristic history-guided --max-cost 15 --json
```

**Heuristics:**  
`language-aware` (default) – tree‑sitter error node analysis  
`cost-weighted` – penalty‑based cost function  
`history-guided` – reuse successful past repairs  
`balanced` – fallback BFS (same as `balance`)

---

### `explain`

Diagnose a syntax error at a given line.

```bash
patch-ts explain --file <FILE> --line <N> [--json] [--theme dark]
```

---

### `fix`

Auto‑suggest a fix from compiler error output.

```bash
cargo check 2>&1 | patch-ts fix --apply
patch-ts fix --error-file errors.txt --file src/main.rs --apply
```

---

### `git`

Apply a git commit’s diff or diff against a target.

```bash
patch-ts git apply abc123 --file src/main.rs
patch-ts git diff HEAD~1 --apply
```

---

### `semdiff`

Semantic (AST‑level) diff between two files.

```bash
patch-ts semdiff --old old.rs --new new.rs --json
```

---

### `provenance`

Query SCITT provenance records.

```bash
patch-ts provenance --since 2025-01-01 --file src/main.rs --json
```

---

### `gate`

Run a multi‑stage validation gate.

```bash
patch-ts gate --file src/main.rs --stages syntax,compile,cross‑file,semdiff,vuln‑check --json
patch-ts gate --file src/main.rs --markdown
patch-ts gate --file src/main.rs --stages lsp --lsp-command /usr/local/bin/rust-analyzer
patch-ts gate --file src/main.rs --stages adversarial,owasp-tool-poisoning,owasp-prompt-injection
```

**Available stages**  
`syntax`, `compile`, `cross‑file`, `semdiff`, `test`, `swe‑bench`, `vuln‑check`, `regression`, `style`, `lsp`, `adversarial`, `owasp‑tool‑poisoning`, `owasp‑prompt‑injection`, `owasp‑supply‑chain`

**LSP stage** detects type errors and semantic warnings using real language servers.  
**Shadow Editor** (built into `--validate‑first` and LSP gate) diffs diagnostics before/after patch – if new errors appear, the patch is rejected.

---

### `score`

Compute a reliability score for a patch.

```bash
patch-ts score --file src/main.rs --old "fn old()" --new "fn new()" --confidence 0.95
```

Displays a colour‑gradient scorecard unless `--json` is given.

---

### `index`

Build and query a project symbol index.

```bash
patch-ts index --rebuild
patch-ts index --callers my_function --json
```

---

### `evolve`

Run evolutionary search for optimal patch parameters.

```bash
patch-ts evolve --file src/main.rs --old "old" --new "new" --population-size 20 --apply
```

---

### `review`

Run a multi‑agent review across syntax, compile, coverage, security, and semantic diff.

```bash
patch-ts review --file src/main.rs --old "old content" --new "new content" --json
```

---

### `attest`

Generate a CRA‑ready attestation report.

```bash
patch-ts attest --since 2025-01-01 --json
patch-ts attest --since 2025-01-01 --cra‑report --output cra.json
```

---

### `verify`

Check invariants expressed in comments (`//@ invariant`) between original and patched code.

```bash
patch-ts verify --file src/main.rs --json
```

---

### `impact`

Show callers of a symbol using the accurate tree‑sitter call graph.

```bash
patch-ts impact --symbol main --recursive
```

---

### `entity`

Structured action space: operate on named code entities.

```bash
patch-ts entity list --file src/main.rs
patch-ts entity show --symbol foo --file src/main.rs
patch-ts entity replace --symbol foo --file src/main.rs --new "fn foo() { ... }"
patch-ts entity body --symbol foo --file src/main.rs --new "{ /* new body */ }"
```

---

### `key`

SCITT key management.

```bash
patch-ts key generate --output keypair.json
```

---

### `recall` (v1.13+)

Generate a token‑efficient retry context when a patch fails.

```bash
patch-ts recall --file src/main.rs --line 42 --old "let port = 3000;" --new "let port = 8080;" --error-code E002 --json
patch-ts recall --file src/main.rs --line 42 --old "let port = 3000;" --new "let port = 8080;" --error-code E002 --prompt
patch-ts recall --file src/main.rs --line 42 --old "let port = 3000;" --new "let port = 8080;" --error-code E002 --entropy --pre-fetch
patch-ts recall --file src/main.rs --line 42 --old "let port = 3000;" --new "let port = 8080;" --error-code E002 --minimal --max-tokens 500
```

**Options**
- `--error-message <MSG>` – full error message from the failed patch
- `--context-lines <N>` – number of surrounding lines (default 5)
- `--json` / `--prompt` – output format
- `--entropy` – select only high information‑density lines
- `--entropy-threshold <FLOAT>` – minimum entropy score (default 2.5)
- `--pre-fetch` – include callers/callees and containing function body
- `--minimal` – ultra‑compact output (error, line, best strategy)
- `--session <ID>` – track retries across calls; strategies re‑ranked by past success
- `--max-tokens <N>` – enforce hard token budget with intelligent truncation

Token savings compared to full file re‑read: **~86–90%** for typical retries.

---

### `watch`

Watch a file and apply hooks on modification.

```bash
patch-ts watch --path src/ --hooks "echo %file% changed"
```

---

### `lsp`

Start a Language Server for editor integration.

```bash
patch-ts lsp
```

---

### `mcp` / `mcp‑http` / `mcp‑gateway`

Start the MCP server.

```bash
patch-ts mcp                          # stdio
patch-ts mcp-http --port 9090        # HTTP
patch-ts mcp-gateway --port 9090 --policy policy.toml  # policy‑enforced
```

**MCP Gateway** (v1.16.0) enforces a policy file that can limit allowed tools, file paths, patch sizes, and run OWASP security checks.

---

## 🔌 MCP Integration

patch‑ts exposes a comprehensive [Model Context Protocol](https://modelcontextprotocol.io/) server.

**Tools (9):**  
`patch`, `balance`, `explain`, `impact`, `semdiff`, `entity_list`, `entity_replace`, `gate`, `recall`

**Resources (13):**  
`symbols/main.rs`, `history`, `provenance`, `dashboard`, `review`,  
`entities/{file}`, `entity/{file}/{symbol}`, `impact/{symbol}`, `semdiff/{file}`,  
`coverage/{file}`, `gate/{file}`, `score/{file}`, `provenance/{file}`

---

## 🎨 Visual Features

- **Syntax highlighting** – via `syntect`, 4 themes, honouring `NO_COLOR`.
- **Word‑level diff** – only changed tokens are highlighted; gutter column shown.
- **TUI side‑by‑side** – press `s` in the TUI to toggle stacked/side‑by‑side view.
- **Scorecard** – `patch-ts score` renders colour‑coded bars.
- **Markdown gate reports** – `patch-ts gate --markdown` produces a table suitable for GitHub PRs.

---

## 🔐 SCITT & Supply Chain

- Ed25519 key generation and rotation.
- Hash‑chained provenance log (`.patch‑ts/provenance.jsonl`).
- CRA‑ready attestation with SBOM.
- OWASP security gates: tool poisoning, prompt injection, supply chain.

---

## 🧪 Testing & CI

- Full test suite covering all commands, languages, and edge cases.
- Property‑based testing (`proptest`) for repair and validation.
- Fuzz testing of the CLI.
- CI via GitHub Actions (macOS, Linux, Windows).

---

## 🚧 Upcoming (v1.17+)

- **Persistent project index** for instant symbol queries (beyond current `index`).
- **Predictive context engine** – `patch‑ts context` for minimal, high‑value agent context.
- **WASM productionisation** – npm package for browser‑based usage.
- **LSP client improvements** – support for more language servers, Shadow Editor in MCP tools.

---

## 🤝 Contributing

We welcome contributions! See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.  
Open an issue to discuss new features or report bugs.

---

## 📄 License

MIT © [elcoosp](https://github.com/elcoosp)

---

*Built with Rust, tree‑sitter, and a lot of ☕*
