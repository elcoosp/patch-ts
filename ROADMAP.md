# ROADMAP.md

## `patch-ts` Roadmap

This document outlines the evolution of `patch-ts`. Priorities are informed by the codebase review and alignment with the original specification.

---

### v0.2.0 — Fuzzy Matching & Auto‑Repair

**Status:** ✅ Implemented

- [x] **Fuzzy single‑line matching** – `fuzzy_match_line` with whitespace normalization and similarity threshold.
- [x] **Fuzzy multi‑line block matching** – `find_best_block_match` using tree‑sitter tokenization and Jaccard similarity.
- [x] **Auto‑repair on syntax error** – `quick_balance` attempts to fix extra delimiters when a patch introduces a syntax error.
- [x] **Marker‑based targeting** – `--marker` flag replaces the AST node following a `// PATCH-ME: <id>` comment.
- [x] **Enhanced JSON diagnostics** – `JsonError` now includes `suggestion`, `best_score`, `best_match_line`, and `candidates` fields.
- [x] **Fuzzy flag for unified diffs** – `--fuzz` is allowed with `--diff`.

---

### v0.3.0 — Repair Engine Expansion

**Status:** ✅ Implemented

- [x] **Support all delimiter types** – Detect and fix extra/missing `(`, `[`, and `{`.
- [x] **Insert missing delimiters** – Insertion logic implemented alongside removal.
- [x] **Multi‑error repair** – Iterative repair loop fixes multiple errors in one run.
- [x] **`--function` scoping** – Restrict `balance` repairs to a named function body.
- [x] **JSON output for balance** – Structured `BalanceResult` with action details.

---

### v0.4.0 — Multi‑language Support

**Status:** ✅ Implemented

- [x] **Add TypeScript/JavaScript support** – Integrated `tree-sitter-typescript` and `tree-sitter-javascript`.
- [x] **Language detection** – Auto‑select language based on file extension (`.rs` → Rust, `.ts`/`.tsx`/`.mts`/`.cts` → TypeScript, `.js`/`.jsx`/`.mjs`/`.cjs` → JavaScript).
- [x] **Cross‑language commands** – `patch`, `balance`, and `explain` work identically across all supported languages.
- [x] **MISSING node detection** – Use tree‑sitter queries to find missing delimiters for robust insertion in TS/JS.
- [x] **Integration tests** – Added test suites for TypeScript and JavaScript.

---

### v0.5.0 — Enhanced Insertion & Diagnostics

**Goal:** Improve missing delimiter insertion reliability and diagnostic precision.

- [ ] **Smarter insertion point detection** – Use surrounding node context for better placement of missing delimiters in TS/JS.
- [ ] **Multi‑error batch repair** – Fix multiple delimiter errors in a single pass without re‑parsing.
- [ ] **Enhanced `explain` for TS/JS** – Provide language‑specific error messages and fix suggestions.
- [ ] **JSON Schema validation** – Add `jsonschema` dev‑dependency to validate JSON output structure.

---

### v1.0.0 — Stability & Ecosystem

**Goal:** Solidify the tool as a reliable part of the AI‑assisted development workflow.

- [ ] **Multi‑file patches** – Support applying a patch that spans multiple files (e.g., from a single diff).
- [ ] **Configuration file** – Read `patch-ts.toml` for project‑wide defaults (fuzz radius, backup location, language settings).
- [ ] **Integration examples** – Provide copy‑pasteable prompts for popular LLMs (ChatGPT, Claude) that generate correct `patch-ts` commands.
- [ ] **CI‑friendly exit codes** – Document and stabilize exit codes for scripting.
- [ ] **Publish on crates.io** – Official release with semantic versioning.

---

### Future Ideas (Post‑v1.0)

- **IDE plugin** (VSCode extension) to apply patches directly from compiler errors.
- **`patch-ts watch`** mode to automatically apply patches from a queue.
- **Structural diff** that understands AST moves, not just line changes.
- **Integration with `cargo fix`** as a custom tool.

---

*Last updated: 2026‑04‑22*
