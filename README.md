# patch-ts

**Tree‑sitter‑aware patching CLI for AI agents and developers.**
Safely apply patches to **16+ languages** with fuzzy matching, AST validation, auto‑repair, MCP server, compilation validation, and LLM‑output sanitization.

---

## What's New in v1.5.0

- **LLM Output Sanitizer** – Automatically strips `<think>` blocks, repairs malformed JSON (trailing commas, single quotes), and extracts diffs from prose‑heavy LLM output.
- **Ellipsis Pattern Support** – Recognizes `...` wildcards in heredoc search/replace blocks.
- **Uniqueness‑Adjusted Confidence** – Common lines (like `}`) now require higher confidence before being matched.
- **Multi‑Line Anchor Detection** – When single‑line matching fails, falls back to pairs of unique lines.
- **Whitespace‑Flexible Diff Matching** – Ignores leading whitespace in diff contexts by default.
- **Enhanced Self‑Healing Feedback** – Failed patches now return error codes (`E001`–`E006`), surrounding context lines, and confidence breakdowns.

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

### New Commands in v1.5.0

| Command | Description |
|---------|-------------|
| `mcp` | Start an MCP server on stdio |
| `pipeline` | Run chained operations |
| `validate‑spec` | Validate code against a specification |
| `suggest‑threshold` | Get a recommended confidence threshold |

### New Options in v1.5.0

| Option | Description |
|--------|-------------|
| `--no‑sanitize` | Disable LLM output sanitizer |
| `--no‑ellipsis` | Disable ellipsis pattern support |
| `--uniqueness‑weight <N>` | Adjust uniqueness influence on confidence (default 0.2) |
| `--strict‑whitespace` | Disable whitespace‑flexible diff matching |
| `--no‑compile‑check` | Skip post‑patch compilation validation |
| `--compile‑timeout <N>` | Timeout for compilation check (default 30s) |
| `--fix‑indent` | Automatically fix indentation of new content |

---

## JSON Output (Enhanced)

```json
{
  "success": false,
  "confidence": 0.72,
  "strategy": "fuzzy",
  "uniqueness_score": 0.45,
  "error_code": "E002",
  "retry_prompt": "Error code: E002\nFile: src/main.rs, Line: 42\nConfidence: 0.72 (threshold: 0.90)\n...",
  "context_lines": [
    "  40: fn main() {",
    "  41:     let y = 2;",
    "  42:     println!(\"{}\", y);"
  ],
  "compilation_errors": [
    {
      "file": "src/main.rs",
      "line": 42,
      "message": "cannot find value `timeout`"
    }
  ]
}
```

---

## License

MIT
