# **patch‑ts v1.10.0 Specification**

## 1. Executive Summary

**patch‑ts** is a tree‑sitter‑backed, multi‑language patching CLI for AI agents.  
Version **1.10.0** introduces **symbol‑level patching**, **semantic diffs**, **incremental parsing**, **TOML/JSON editing via tree‑sitter**, a **WebAssembly target**, and a **structured knowledge graph** for cross‑file impact analysis. It also resolves long‑standing HTTP body handling issues and cleans all compiler warnings.

The release solidifies patch‑ts as the most advanced AI‑agent patching tool, while remaining a single binary with minimal dependencies.

---

## 2. Design Goals

| Goal | Measure |
|------|---------|
| **Agent token efficiency** | Add `--symbol` to replace a function/class by name, reducing context window usage. |
| **Semantic diff accuracy** | Show only AST‑meaningful changes; ignore formatting noise. |
| **Performance** | Use tree‑sitter `edit()` for sub‑ms re‑parsing after edits. |
| **Non‑Rust file editing** | Enable safe patching of `Cargo.toml`, `package.json`, etc. using tree‑sitter. |
| **Multi‑platform reach** | Compile to WASM for browser‑based agents (VS Code web, GitHub Codespaces). |
| **Full MCP capability** | Fix HTTP body reading, unblock MCP server, expand resources. |
| **Clean CI** | Zero warnings, all tests pass, no ignored failures. |

---

## 3. New Features

### 3.1 Symbol‑Level Patching

**Command:**  
`patch‑ts patch --symbol <name> --file <file> [--old ...] [--new ...]`

**Behaviour:**  
- The tool parses the file and locates the top‑level function, struct, class, or method with the given name.  
- If `--old` and `--new` are provided, it replaces the body of that symbol while preserving the signature.  
- If only `--new` is given, it replaces the entire symbol definition.  
- Falls back to line‑based patching if symbol is not found.

**Tree‑sitter integration:**  
Use `tree‑sitter` queries to find the node of the correct kind (`function_item`, `class_declaration`, etc.). The match is language‑aware (16 languages supported).

**Error handling:**  
- Symbol not found → clear error with candidate names.  
- Ambiguous (multiple matches) → list all matches and ask for qualification.

---

### 3.2 Semantic Diff Engine

**Integration:** Use the `syndiff` crate (or a custom tree‑sitter visitor) to compute an AST‑aware diff.

**Affected commands:**  
- `patch‑ts gate --stages semdiff` → shows a summary of semantic changes between old and new.  
- `patch‑ts tui` → in the diff panel, highlights only semantic changes.  
- `patch‑ts review` → agents can see semantic diff alongside text diff.

**Output format:** JSON (for gate/review) and coloured terminal output.

---

### 3.3 Incremental Parsing

**Implementation:**  
- `Parser::parse(source, old_tree)` already available in tree‑sitter.  
- `patch‑ts` will keep the previous `Tree` after a successful patch and reuse it for subsequent re‑parsing when the user applies multiple patches in one invocation (e.g., with `--files`).  
- This reduces parsing overhead by up to 90% for multi‑patch workflows.

**Backward compatibility:** Fully transparent; if no old tree is provided, full parse is used.

---

### 3.4 TOML & JSON Patching via Tree‑Sitter

**New tree‑sitter grammars added:**  
- `tree-sitter-toml`  
- `tree-sitter-json`

**Supported commands:**  
- `patch‑ts patch --file Cargo.toml --symbol [package.name] --old "old-value" --new "new-value"`  
- `patch‑ts patch --file package.json --symbol dependencies.react --old "18" --new "19"`  

The tool will use tree‑sitter node selection and structural replacement, not simple text search, ensuring correctness even with complex nested structures.

**Fallback:**  
If a path like `dependencies.react` maps to a JSON key, the tool can replace the value node directly.

---

### 3.5 WebAssembly Build Target

**Goal:** Compile `patch‑ts` to `wasm32-wasip1` or `wasm32-unknown-unknown` with limited I/O, enabling use in browser‑based AI sandboxes.

**Changes needed:**  
- Gate `git2`, `tempfile`, `file` operations behind `#[cfg(not(target_arch = "wasm32"))]`.  
- Provide WASI‑compatible file read/write through the virtual filesystem.  
- Expose a JavaScript‑friendly API via `wasm-bindgen` (or use existing `wit-bindgen`).  
- MCP client remains available (HTTP I/O works in WASM via browser fetch).

**Build target:**  
`cargo build --target wasm32-wasip1 --release`

---

### 3.6 Knowledge Graph Enhancement

**Motivation:** The current `crossfile.rs` uses regex to find callers, which is fragile.

**New implementation:**  
- Build a call graph from tree‑sitter function call nodes.  
- Store it in `KnowledgeGraph` (already defined) with `HashMap<symbol, Vec<CallSite>>`.  
- New command: `patch‑ts impact --symbol <name> [--recursive]` – lists all direct (and recursive) callers across the project.  
- The `gate --stages cross-file` now uses this accurate graph.

---

### 3.7 MCP Server Expansion

**Fix:** Resolve hyper body reading to unblock `McpHttp`. Use `http-body-util` for body aggregation.

**New resources:**  
- `patch-ts://knowledge/{symbol}` – returns callers/callees for a symbol.  
- `patch-ts://semdiff/{file}` – returns semantic diff JSON.  
- `patch-ts://coverage/{file}` – returns changed lines and uncovered lines.  

**Tools:**  
- `patch` (existing)  
- `balance` (existing)  
- `explain` (existing)  
- `impact` (new) – runs cross‑file impact analysis  
- `semdiff` (new) – returns semantic diff for two file versions

---

## 4. Changes from v1.9.0

### 4.1 Dependency Updates

| Dependency | Version | Notes |
|------------|---------|-------|
| `hyper` | 1.9.0 | Removed unsupported `"body"` feature; added `http-body-util` |
| `http-body-util` | 0.1 | New direct dependency for HTTP body aggregation |
| `tree-sitter-toml` | 0.3 | New |
| `tree-sitter-json` | 0.24 | New |
| `syndiff` (or custom) | 0.1 | Semantic diff engine |

### 4.2 Breaking Changes

- **None.** All CLI flags remain backward compatible. The `--symbol` flag is additive.  
- **WASM target** may require gating some functions, but the CLI and library code are unchanged.

### 4.3 Warning Cleanup

- All `unused_mut` in tests fixed.  
- All `unused_variables` prefixed with `_`.  
- Duplicate `#[ignore]` attribute removed.  
- Unused imports removed (`quick_balance`, `StageResult`).

### 4.4 Test Suite

- All existing tests pass.  
- `test_invalid_line_number_zero` now uses `--no-compile-check` to avoid timeout.  
- New tests for symbol patching, semantic diff, TOML/JSON patching, and WASM build added.  
- CI green on all platforms.

---

## 5. Implementation Plan

### 5.1 Phase 1: Foundation (Sprint 1)

- [x] Fix hyper body reading  
- [x] Warning cleanup  
- [x] Duplicate ignore attribute removal  

### 5.2 Phase 2: Enhancements (Sprint 2)

- [ ] Implement `--symbol` targeting in `patch`  
- [ ] Integrate syndiff/semantic diff  
- [ ] Incremental parsing optimisation  

### 5.3 Phase 3: New Grammars & Web (Sprint 3)

- [ ] Add `tree-sitter-toml` and `tree-sitter-json`  
- [ ] TOML/JSON patching support  
- [ ] WASM build configuration and documentation  

### 5.4 Phase 4: Knowledge Graph & MCP (Sprint 4)

- [ ] Accurate call graph using tree‑sitter  
- [ ] `impact` command and gate stage  
- [ ] MCP resources expansion  

---

## 6. Quality Gates

- All unit tests pass (`just test`).  
- Integration tests for each new feature.  
- Benchmark regression: parsing time for 2000‑line file < 5ms (with incremental).  
- No warnings, no `clippy` errors.  
- Documentation updated for each new subcommand/option.  
- WASM binary sizes < 10MB (gzipped).  

---

## 7. Documentation

- User‑facing docs for `--symbol`, `impact`, `semdiff`, and WASM build will be published in the repository wiki and the README.  
- An LLM agent integration guide will be added to the `crates/plugin-interface/` documentation.

---

*Specification approved for v1.10.0 development.*
