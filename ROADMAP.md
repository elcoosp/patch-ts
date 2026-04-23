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

### v1.4.0 — Platform‑Grade Patching (Current)
**Goal:** Transform patch‑ts into a verifiable, self‑healing patching platform.

- [x] **MCP Server** – Exposes patch, balance, explain as JSON‑RPC 2.0 tools via stdio.
- [x] **Compilation Validation** – Post‑patch compiler checks for Rust, TS, JS, Python, Go with graceful fallback.
- [x] **Self‑Healing Feedback** – Structured retry prompts in JSON output for failed patches.
- [x] **Semantic Symbol Index** – Tree‑sitter‑based function index for 5 languages.
- [x] **Spec‑Driven Validation** – Cross‑references patches against specification documents.
- [x] **Pipeline Mode** – Chained operations (patch → validate → test → commit).
- [x] **Adaptive Thresholds** – Suggests optimal confidence thresholds from history.

### Future Ideas

- **Machine learning‑based repair** – Train a small model to predict correct fixes.
- **Git integration** – Apply patches directly to staged changes or commits.
- **WebAssembly playground** – Try patch‑ts in the browser.
- **More language support** – Expand to cover Kotlin, Lua, Dart.

*Last updated: 2026‑04‑23*
