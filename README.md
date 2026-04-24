# patch-ts

**Tree‑sitter‑aware patching CLI for AI agents and developers.**
Safely apply patches to **16+ languages** with semantic diffing, provenance tracking, quality gates, reliability scoring, and MCP 2.0.

---

## What's New in v1.7.0

- **`patch‑ts sem‑diff`** – See entity‑level changes: ⊕ functions added, ∆ modified, ⊖ removed.
- **`patch‑ts provenance`** – Audit trail of every AI‑assisted change (Agent Trace emission).
- **`patch‑ts gate`** – Configurable quality gate pipeline (syntax → compile → cross‑file → test).
- **`patch‑ts score`** – Multi‑dimensional reliability score (0‑100) for every patch.
- **MCP Server 2.0** – Full JSON Schemas, resources, and sampling for AI agent integration.
- **TUI enhancements** – Entity‑level navigation (`[`/`]`) and inline commenting (`c`).

---

## Installation

```bash
cargo install patch-ts
```

## Supported Languages

Rust | TypeScript | JavaScript | Python | Go | Ruby | PHP | HTML | XML | C | C++ | Java | C# | Swift | Scala | Zig

---

## New Commands in v1.7.0

| Command | Description |
|---------|-------------|
| `sem‑diff` | Semantic entity‑level diff between two file versions |
| `provenance` | Query AI provenance audit trail |
| `gate` | Run quality gate pipeline on a patch |
| `score` | Calculate patch reliability score |

---

## Quick Start – Semantic Diff

```bash
patch‑ts sem‑diff --old lib_v1.rs --new lib_v2.rs
```

## Quick Start – Quality Gate

```bash
patch‑ts gate --file main.rs --stages "syntax,compile,cross‑file"
```

---

## License

MIT
