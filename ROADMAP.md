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

### v1.5.0 — LLM‑Input Resilience (Current)
**Goal:** Make patch‑ts dramatically more resilient to malformed LLM output.

- [x] **LLM Output Sanitizer** – Strips `<think>` blocks, repairs malformed JSON, extracts fenced blocks.
- [x] **Format‑Agnostic Diff Extraction** – Detects and extracts diffs from prose and mixed‑content LLM output.
- [x] **Ellipsis Pattern Support** – Handles `...` wildcards in heredoc search/replace blocks.
- [x] **Uniqueness‑Adjusted Confidence** – Dynamically adjusts confidence thresholds based on target content uniqueness.
- [x] **Multi‑Line Anchor Detection** – Falls back to paired unique lines when single‑line anchors fail.
- [x] **Whitespace‑Flexible Diff Matching** – Ignores leading whitespace differences by default.
- [x] **Enhanced Self‑Healing Feedback** – Error codes, context lines, and confidence breakdown in JSON output.

### Future Ideas

- **Machine learning‑based repair** – Train a small model to predict correct fixes.
- **Git integration** – Apply patches directly to staged changes or commits.
- **WebAssembly playground** – Try patch‑ts in the browser.
- **More language support** – Expand to cover Kotlin, Lua, Dart.

*Last updated: 2026‑04‑24*
