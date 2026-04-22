# ROADMAP.md

## `patch-ts` Roadmap

This document outlines the evolution of `patch-ts`.

---

### v0.2.0 — Fuzzy Matching & Auto‑Repair

**Status:** ✅ Implemented

- [x] Fuzzy single‑line matching
- [x] Fuzzy multi‑line block matching
- [x] Auto‑repair on syntax error
- [x] Marker‑based targeting
- [x] Enhanced JSON diagnostics
- [x] Fuzzy flag for unified diffs

---

### v0.3.0 — Repair Engine Expansion

**Status:** ✅ Implemented

- [x] Support all delimiter types (`(`, `[`, `{`)
- [x] Insert missing delimiters
- [x] Multi‑error repair (iterative)
- [x] `--function` scoping (Rust)
- [x] JSON output for balance

---

### v0.4.0 — Multi‑language Support

**Status:** ✅ Implemented

- [x] TypeScript/JavaScript support
- [x] Language detection by file extension
- [x] Cross‑language commands
- [x] MISSING node detection
- [x] Integration tests for TS/JS

---

### v0.5.0 — Enhanced Insertion & Diagnostics

**Status:** ✅ Implemented

- [x] **Context‑aware insertion** – Uses parent node kind to place missing delimiters accurately.
- [x] **Batch repair** – Collects and sorts all errors, applies fixes in one pass.
- [x] **Language‑specific diagnostics** – `diagnostic_message` with context for TS/JS.
- [x] **Public `ParseResult.index`** – Enables span calculations in repair logic.

---

### v0.6.0 — Robust Batch Repair & Additional Languages

**Goal:** Improve batch repair reliability and add Python support.

- [ ] **Overlapping error resolution** – Handle cases where repairs affect subsequent positions.
- [ ] **Python support** – Integrate `tree-sitter-python` and implement `PythonLanguage`.
- [ ] **Go support** – Integrate `tree-sitter-go` (stretch goal).
- [ ] **Performance optimizations** – Reduce parsing overhead in batch mode.

---

### v1.0.0 — Stability & Ecosystem

**Goal:** Solidify the tool as a reliable part of the AI‑assisted development workflow.

- [ ] **Multi‑file patches** – Apply a patch spanning multiple files.
- [ ] **Configuration file** – `patch-ts.toml` for project defaults.
- [ ] **Integration examples** – LLM prompts for generating correct `patch-ts` commands.
- [ ] **CI‑friendly exit codes** – Documented and stable.
- [ ] **Publish on crates.io** – Official release.

---

### Future Ideas (Post‑v1.0)

- **IDE plugin** (VSCode extension)
- **`patch-ts watch`** mode
- **Structural diff** (AST‑aware moves)
- **Integration with `cargo fix`**

---

*Last updated: 2026‑04‑22*
