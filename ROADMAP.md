# ROADMAP.md

## `patch-ts` Roadmap

This document outlines the evolution of `patch-ts`. Priorities are informed by the codebase review and alignment with the original specification.

---

### v0.2.0 (Current) — Fuzzy Matching & Auto‑Repair

**Status:** Implemented

- [x] **Fuzzy single‑line matching** – `fuzzy_match_line` with whitespace normalization and similarity threshold.
- [x] **Fuzzy multi‑line block matching** – `find_best_block_match` using tree‑sitter tokenization and Jaccard similarity.
- [x] **Auto‑repair on syntax error** – `quick_balance` attempts to fix extra delimiters when a patch introduces a syntax error.
- [x] **Marker‑based targeting** – `--marker` flag replaces the AST node following a `// PATCH-ME: <id>` comment.
- [x] **Enhanced JSON diagnostics** – `JsonError` now includes `suggestion`, `best_score`, `best_match_line`, and `candidates` fields.
- [x] **Fuzzy flag for unified diffs** – `--fuzz` is allowed with `--diff` (stub for future fuzzy hunk application).

---

### v0.3.0 — Repair Engine Expansion

**Goal:** Make `balance` more comprehensive and useful.

- [ ] **Support all delimiter types**  
  Detect and fix extra/missing `(`, `[`, and `{`.

- [ ] **Insert missing delimiters**  
  Currently only removal is implemented; add insertion at the correct location.

- [ ] **Multi‑error repair**  
  Iteratively fix multiple unbalanced delimiters in a single run.

- [ ] **`--function` scoping**  
  Implement Tree‑sitter query to restrict `balance` to a specific function body.

---

### v0.4.0 — Multi‑language Support

**Goal:** Prove the `Language` trait extensibility with a second language.

- [ ] **Add TypeScript/JavaScript support**  
  Integrate `tree-sitter-typescript` grammar and implement `TypeScriptLanguage`.

- [ ] **Language detection**  
  Auto‑select language based on file extension (`.rs` → Rust, `.ts`/`.js` → TypeScript).

- [ ] **Language‑specific repair heuristics**  
  Ensure `balance` works for TypeScript delimiters.

---

### v1.0.0 — Stability & Ecosystem

**Goal:** Solidify the tool as a reliable part of the AI‑assisted development workflow.

- [ ] **Multi‑file patches**  
  Support applying a patch that spans multiple files (e.g., from a single diff).

- [ ] **Configuration file**  
  Read `patch-ts.toml` for project‑wide defaults (fuzz radius, backup location, language settings).

- [ ] **Integration examples**  
  Provide copy‑pasteable prompts for popular LLMs (ChatGPT, Claude) that generate correct `patch-ts` commands.

- [ ] **CI‑friendly exit codes**  
  Document and stabilize exit codes for scripting.

- [ ] **Publish on crates.io**  
  Official release with semantic versioning.

---

### Future Ideas (Post‑v1.0)

- **IDE plugin** (VSCode extension) to apply patches directly from compiler errors.
- **`patch-ts watch`** mode to automatically apply patches from a queue.
- **Structural diff** that understands AST moves, not just line changes.
- **Integration with `cargo fix`** as a custom tool.

---

*Last updated: 2026‑04‑22*
