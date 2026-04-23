#!/usr/bin/env bash
set -euo pipefail

cd "$(git rev-parse --show-toplevel)"

cat > ROADMAP.md << 'EOF'
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

### v0.7.0 — Ruby, PHP, HTML, XML & AST Reliability

**Status:** ✅ Implemented

- [x] Ruby language support (`.rb`)
- [x] PHP language support (`.php`)
- [x] HTML/XML language support (`.html`, `.xml`)
- [x] AST‑based delimiter detection for all languages
- [x] Python balance tests re‑enabled and passing
- [x] Integration tests for all new languages

---

### v0.8.0 — C, C++, Java, C# & Rollback Validation

**Status:** ✅ Implemented

- [x] C language support (`.c`, `.h`)
- [x] C++ language support (`.cpp`, `.cc`, `.cxx`, `.hpp`)
- [x] Java language support (`.java`)
- [x] C# language support (`.cs`)
- [x] Rollback validation for overlapping repairs
- [x] Integration tests for all new languages

---

### v0.9.0 — Swift, Scala, Zig & Advanced Features

**Status:** ✅ Implemented

- [x] Swift language support (`.swift`)
- [x] Scala language support (`.scala`)
- [x] Zig language support (`.zig`)
- [x] Configuration file support (`patch-ts.toml`)
- [x] Watch mode foundation (`notify` watcher)
- [x] TUI foundation (`ratatui` diff viewer)
- [x] Multi‑file balance support (serial)

---

### v1.0.0 — Stability, Performance & Ecosystem

**Status:** ✅ Implemented

- [x] **Parallel processing** – `rayon`‑powered multi‑file operations with thread‑safe language instances.
- [x] **Multi‑file patches** – Apply a single patch across multiple files via glob patterns.
- [x] **Plugin system** – WASM‑based plugin interface for custom repair strategies.
- [x] **Comprehensive documentation** – User guide, AI agent guide, plugin development guide.
- [x] **Publish on crates.io** – Official package released.
- [x] **Homebrew and Scoop formulas** – Easy installation for macOS and Windows.

---

### v1.1.0 — Minimum‑Cost Repair Engine

**Status:** ✅ Implemented

- [x] **BFS‑based minimum‑cost repair** – Replaces heuristic balancing with a search that guarantees the smallest set of delimiter edits.
- [x] **Memoization** – Avoids redundant re‑parsing during search.
- [x] **`--max-cost` flag** – User‑configurable search depth (default 10) to prevent explosion on severely broken files.
- [x] **Re‑enabled & new tests** – Property‑based tests for `quick_balance`; re‑enabled test suites.
- [x] **Documentation update** – README reflects new repair approach.

---

### v1.2.0 — Interactive & Collaborative Features (Current)

**Goal:** Elevate the user experience with richer interfaces and workflow‑enhancing capabilities.

- [ ] **TUI improvements** – Syntax‑highlighted diffs, side‑by‑side view, inline editing.
- [ ] **Watch mode enhancements** – Debouncing, ignore patterns, event hooks.
- [ ] **Remote patch sources** – Fetch patches from URLs or git repositories.
- [ ] **Patch history and undo** – Track applied patches and revert if needed.
- [ ] **LSP integration** – Provide real‑time diagnostics and code actions (previously planned for v1.1.0, now targeted for v1.2.0).

---

### Future Ideas

- **Machine learning‑based repair** – Train a small model to predict correct fixes.
- **Git integration** – Apply patches directly to staged changes or commits.
- **WebAssembly playground** – Try patch‑ts in the browser.
- **More language support** – Expand to cover other popular languages (e.g., Kotlin, Lua, Dart).

---

*Last updated: 2026‑04‑23*
EOF

git add ROADMAP.md
git commit -m "docs: update roadmap – v1.0.0 and v1.1.0 completed, v1.2.0 defined"
