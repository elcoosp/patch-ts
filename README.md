# patch-ts

**Tree‑sitter‑aware patching CLI for AI agents and developers.**
Safely apply patches to **16+ languages** with compiler‑error‑driven fixes, Git‑native commands, cross‑file analysis, and CI integration.

---

## What's New in v1.6.0

- **`patch‑ts fix`** – Parse compiler errors from `rustc`, `tsc`, `node`, or `python` and auto‑suggest a fix.
- **`patch‑ts git apply` / `git diff`** – Apply patches from specific commits or diff against branches.
- **Cross‑file analysis** – When a patch changes a function signature, patch‑ts warns about callers in other files.
- **GitHub Action & GitLab CI** – Pre‑built templates to validate every PR for delimiter issues.
- **Adaptive threshold tuning** – `patch‑ts adapt‑threshold` reads your history and suggests the best confidence threshold.

---

## Installation

```bash
cargo install patch-ts
```

## Supported Languages

Rust | TypeScript | JavaScript | Python | Go | Ruby | PHP | HTML | XML | C | C++ | Java | C# | Swift | Scala | Zig

---

## New Commands in v1.6.0

| Command | Description |
|---------|-------------|
| `fix` | Parse compiler errors and propose/suggest a fix |
| `git apply` | Apply the diff from a specific commit |
| `git diff` | Diff current branch against another branch |
| `adapt‑threshold` | Suggest an optimal confidence threshold from patch history |
| `adapt‑strategy` | List strategies ordered by success rate |

## New Options

| Option | Description |
|--------|-------------|
| `--cross‑file` | Enable cross‑file semantic analysis (warns about callers of changed functions) |

---

## Quick Start – Fix a Compiler Error

```bash
cargo build 2>&1 | patch‑ts fix --apply
```

---

## License

MIT
