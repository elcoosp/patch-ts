# patch-ts

**Tree‑sitter‑aware patching CLI for AI agents and developers.**
Safely apply patches to **16+ languages** with fuzzy matching, AST validation, auto‑repair, MCP server, compilation validation, and self‑healing feedback.

---

## What's New in v1.4.0

- **MCP Server** – Expose patch‑ts as a Model Context Protocol server. Any MCP‑compatible AI agent can invoke patch, balance, and explain as tools over stdio.
- **Compilation Validation** – Patches are now checked against the actual compiler (cargo check, tsc, node --check, etc.) before being applied. Compilation failures trigger rollback with detailed diagnostics.
- **Self‑Healing Feedback** – Failed patches return structured `retry_prompt` fields in JSON, enabling AI agents to self‑correct.
- **Semantic Symbol Index** – Function‑scoped patching now works across Rust, TypeScript, JavaScript, Python, and Go using tree‑sitter queries.
- **Spec‑Driven Validation** – Cross‑reference patches against project specification documents to catch semantic mismatches.
- **Pipeline Mode** – Chain multiple operations (`patch‑ts pipeline --stages "patch,validate,test,commit"`) with gating at each stage.
- **Adaptive Thresholds** – `patch‑ts suggest-threshold` analyzes your patch history and recommends an optimal confidence threshold.

---

## Installation

```bash
cargo install patch-ts
```

## Supported Languages

Rust | TypeScript | JavaScript | Python | Go | Ruby | PHP | HTML | XML | C | C++ | Java | C# | Swift | Scala | Zig

---

## Usage

```bash
patch-ts <COMMAND> [OPTIONS]
```

### New Commands in v1.4.0

| Command | Description |
|---------|-------------|
| `mcp` | Start an MCP server on stdio |
| `pipeline` | Run chained operations |
| `validate‑spec` | Validate code against a specification |
| `suggest‑threshold` | Get a recommended confidence threshold |

### New Options

| Option | Description |
|--------|-------------|
| `--no‑compile‑check` | Skip post‑patch compilation validation |
| `--compile‑timeout <N>` | Timeout for compilation check (default 30s) |
| `--adaptive‑confidence` | Use history‑based confidence thresholds |

---

## JSON Output (Enhanced)

```json
{
  "success": false,
  "confidence": 0.72,
  "strategy": "fuzzy",
  "compilation_errors": [
    {
      "file": "src/main.rs",
      "line": 42,
      "message": "cannot find value `timeout`"
    }
  ],
  "retry_prompt": "At line 42 of src/main.rs, replace `timeout` with `Duration::from_secs(timeout)`.",
  "symbol_warnings": ["Identifier 'x' not found in original file"],
  "spec_warnings": ["Function signature changed: expected `i32` but got `u64`"]
}
```

---

## License

MIT
