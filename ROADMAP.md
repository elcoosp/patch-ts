# ROADMAP.md

## `patch-ts` Roadmap

### v0.2.0 — Fuzzy Matching & Auto‑Repair
**Status:** ✅ Implemented

### v0.3.0 — Repair Engine Expansion
**Status:** ✅ Implemented

### v0.4.0 — Multi‑language Support
**Status:** ✅ Implemented

### v0.5.0 — Enhanced Insertion & Diagnostics
**Status:** ✅ Implemented

### v0.6.0 — Python & Go Support, Overlapping Error Resolution
**Status:** ✅ Implemented

### v0.7.0 — Ruby, PHP, HTML, XML & AST Reliability
**Status:** ✅ Implemented

### v0.8.0 — C, C++, Java, C# & Rollback Validation
**Status:** ✅ Implemented

### v0.9.0 — Swift, Scala, Zig & Advanced Features
**Status:** ✅ Implemented

### v1.0.0 — Stability, Performance & Ecosystem
**Status:** ✅ Implemented

### v1.1.0 — Minimum‑Cost Repair Engine
**Status:** ✅ Implemented

### v1.2.0 — Interactive & Collaborative Features
**Status:** ✅ Implemented

### v1.3.0 — AI‑Resilience Features
**Status:** ✅ Implemented

### v1.4.0 — Platform‑Grade Patching
**Status:** ✅ Implemented

### v1.5.0 — LLM‑Input Resilience
**Status:** ✅ Implemented

### v1.6.0 — Intelligent & Integrated Patching (Current)
**Goal:** Close the loop between compiler errors, Git, CI, and cross‑file analysis.

- [x] **Compiler‑error‑driven fix** – `patch‑ts fix` parses compiler output and suggests patches.
- [x] **Git‑native commands** – `patch‑ts git apply` and `git diff` for commit‑based patching.
- [x] **Cross‑file semantic analysis** – Warns when a patch changes symbols used in other files.
- [x] **CI/CD integration** – Pre‑built GitHub Action and GitLab CI template.
- [x] **Adaptive threshold tuning** – `patch‑ts adapt‑threshold` suggests optimal confidence from history.

### Future Ideas

- **Machine learning‑based repair** – Train a small model to predict correct fixes.
- **WebAssembly playground** – Try patch‑ts in the browser.
- **More language support** – Expand to cover Kotlin, Lua, Dart.

*Last updated: 2026‑04‑24*
