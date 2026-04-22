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

- [x] Context‑aware insertion
- [x] Batch repair (single pass)
- [x] Language‑specific diagnostics for TS/JS
- [x] Public `ParseResult.index`

---

### v0.6.0 — Python & Go Support, Overlapping Error Resolution

**Status:** ✅ Implemented

- [x] Python language support (`.py`, `.pyi`)
- [x] Go language support (`.go`)
- [x] Overlapping error resolution with offset tracking
- [x] Integration tests for Python and Go

---

### v0.7.0 — Robust Batch Repair & Additional Languages

**Goal:** Improve batch repair edge cases and add more languages.

- [ ] **Full overlapping error support** – Handle complex overlapping scenarios perfectly.
- [ ] **Ruby support** – Integrate `tree-sitter-ruby`.
- [ ] **PHP support** – Integrate `tree-sitter-php`.
- [ ] **HTML/XML support** – Delimiter repair for tags.

---

### v1.0.0 — Stability & Ecosystem

**Goal:** Solidify the tool as a reliable part of the AI‑assisted development workflow.

- [ ] **Multi‑file patches**
- [ ] **Configuration file** (`patch-ts.toml`)
- [ ] **Integration examples** for LLMs
- [ ] **CI‑friendly exit codes**
- [ ] **Publish on crates.io**

---

*Last updated: 2026‑04‑22*

### v0.7.0 — Ruby, PHP, HTML, XML & AST Reliability

**Status:** ✅ Implemented

- [x] Ruby language support (`.rb`)
- [x] PHP language support (`.php`)
- [x] HTML/XML language support (`.html`, `.xml`)
- [x] AST‑based delimiter detection for all languages
- [x] Python balance tests re‑enabled and passing
- [x] Integration tests for all new languages


### v0.8.0 — C, C++, Java, C# & Rollback Validation

**Status:** ✅ Implemented

- [x] C language support (`.c`, `.h`)
- [x] C++ language support (`.cpp`, `.cc`, `.cxx`, `.hpp`)
- [x] Java language support (`.java`)
- [x] C# language support (`.cs`)
- [x] Rollback validation for overlapping repairs
- [x] Integration tests for all new languages
- [x] Performance optimizations

