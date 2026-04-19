# ROADMAP.md

## `patch-ts` Roadmap

This document outlines the planned evolution of `patch-ts` beyond the initial v0.1.0 release. Priorities are informed by the codebase review and alignment with the original specification.

---

### v0.1.0 (Released) — Core Functionality

- [x] Safe patch application with content verification
- [x] Fuzzy line matching (`--fuzz`)
- [x] AST validation using Tree-sitter
- [x] `balance` command (remove extra `}`)
- [x] `explain` command with human/JSON output
- [x] Atomic writes and `.bak` backups
- [x] Comprehensive test suite

---

### v0.2.0 — Diagnostics & Polish

**Goal:** Improve accuracy of JSON diagnostics and round out v0.1 feature edges.

- [ ] **Accurate JSON spans**  
  Extract precise `line` and `column` from `miette::Diagnostic` types (`ContentMismatchError`, `SyntaxErrorDiagnostic`) instead of placeholder values.

- [ ] **Enhanced heredoc parsing**  
  Support trailing whitespace, Windows line endings, and more flexible separator detection.

- [ ] **Improved error messages**  
  Include context lines in `ContentMismatchError` reports, and show diff-like output for mismatches.

- [ ] **Documentation updates**  
  Add a "Limitations & Heuristics" section to README explaining `balance`'s current scope.

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
  Implement Tree-sitter query to restrict `balance` to a specific function body.

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

*Last updated: 2026-04-19*  
*Based on codebase review of commit `8a5edab`.*
