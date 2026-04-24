# patch‑ts v1.14.0 — Specification  
**Deeper Token Efficiency & WASM Everywhere**

## 1. Executive Summary  

Building on v1.13.0’s recall mode, **v1.14.0** turns patch‑ts into a **token‑saving, browser‑ready** patching tool.  
Two pillars:  

- **Smarter Recall**: entropy‑based context selection, predictive pre‑fetch, session memory, and hard token budgets **halve** the tokens needed for retries.  
- **WASM Build Target**: compile `patch‑ts‑core` to `wasm32‑wasi`, enabling browser‑based AI agents to apply tree‑sitter‑backed patches without a server.  

This release expands the addressable market (browser sandboxes, VS Code web, ChatGPT canvas) while directly reducing API costs for every agent that uses patch‑ts.

---

## 2. Design Goals  

| ID | Goal | Measurement |
|----|------|-------------|
| G1 | WASM build | `cargo build --target wasm32‑wasi` succeeds; basic patching works in Node.js/WASI runtime. |
| G2 | Entropy‑based recall | Recall context includes only lines with high information density; token savings ≥40% over v1.13 recall. |
| G3 | Predictive pre‑fetch | Recall context optionally includes callers/callees and containing symbol body. |
| G4 | Compressed recall format | `recall --minimal` output <200 tokens for simple failures. |
| G5 | Session memory | `recall --session <ID>` tracks attempts across retries and improves strategy suggestions. |
| G6 | Token budget mode | `recall --max‑tokens <N>` guarantees output stays within budget. |
| G7 | Backward compatibility | All existing commands unchanged; new flags are additive. |

---

## 3. New Features  

### 3.1 WASM Build Target  

**Goal:** Compile the core patching logic to WebAssembly.  

**Scoping:**  
- `patch‑ts-core` (AST, matching, repair, semdiff, recall) and `patch‑ts‑gate` compile to `wasm32‑wasi`.  
- CLI, TUI, LSP, and MCP are **gated** behind `#[cfg(not(target_arch = "wasm32"))]`.  
- A JavaScript shim (`wasm/patch-ts.js`) wraps the core functions and provides a `patch‑ts‑wasm` npm package.  
- File I/O uses WASI’s `wasi‑filesystem`; when unavailable (browser without WASI), provide a `VirtualFS` trait with an in‑memory implementation.  

**Build command:**  
```bash
cargo build --target wasm32-wasi --release -p patch-ts-core -p patch-ts-gate
```

**JavaScript API (sketch):**  
```javascript
import { applyPatch } from 'patch-ts-wasm';
const result = await applyPatch({
  filePath: '/src/main.rs',
  line: 42,
  old: 'let port = 3000;',
  new: 'let port = 8080;',
  fuzz: 3,
});
```

### 3.2 Entropy‑Based Recall Context  

**Problem:** The current recall context includes fixed‑size surrounding lines, regardless of their usefulness.  

**Solution:** Score each line using **Shannon entropy** (character‑level or token‑level). Lines with the highest entropy (most unique information) are included first, up to a configurable count.  

**New flags:**  
- `recall --entropy` – enable entropy‑based selection (default: on).  
- `recall --entropy‑threshold <FLOAT>` – minimum entropy score to include a line (default: 2.5).  

**Algorithm:**  
1. For each line in the surrounding window, compute character‑level entropy or use a pre‑trained token rarity model (initially simple entropy).  
2. Sort lines by descending entropy.  
3. Take top N lines (where N = `context‑lines` or `max‑tokens` limit).  
4. Preserve line order in output.  

### 3.3 Predictive Pre‑fetch  

**Goal:** When a recall is triggered, automatically include relevant **callers, callees, and the containing function body** if they might help the agent understand the failure.  

**New flag:**  
- `recall --pre‑fetch` – enable predictive pre‑fetch (default: off).  

**Sources:**  
- Call graph (already built by `impact` command).  
- `find_containing_symbol` (enhanced from v1.13 to return function body).  

**Output:**  
- Under `context.related_symbols`, list files/byte offsets of callers/callees.  
- Under `context.containing_body`, include the full function body (truncated if >500 chars).  

### 3.4 Compressed Recall Format  

**Goal:** Provide an ultra‑minimal output for when every token counts.  

**New flag:**  
- `recall --minimal` – output a compact JSON with only:  
  - `error_code`  
  - `error_message`  
  - `target_line_content` (the actual line in the file)  
  - `single_best_strategy` (name + description)  

**Example:**  
```json
{
  "error": "E002",
  "message": "expected 'let port = 3000;' but found 'let port = 3000'",
  "line": "    let port = 3000;",
  "best_strategy": "Increase --fuzz radius to 3"
}
```

### 3.5 Session Memory  

**Goal:** Track multiple retry attempts for the same file/change, learning which strategies succeed.  

**New flag:**  
- `recall --session <ID>` – associate this recall with a session.  

**Storage:**  
- `.patch‑ts/sessions/<ID>.jsonl` – each entry is a `RecallRecord` with `attempt_number`, `strategy_used`, `success`, `timestamp`.  

**Effect:**  
- `history.similar_failures` counts within the session.  
- `history.suggested_approach` prioritises strategies that previously succeeded in the same session.  

### 3.6 Token Budget Mode  

**Goal:** Guarantee the recall output fits within a strict token limit (e.g., 500 tokens).  

**New flag:**  
- `recall --max‑tokens <N>` – maximum tokens allowed for the entire recall JSON (or prompt).  

**Truncation rules:**  
1. If with default context the output exceeds N, switch to `--minimal` mode.  
2. If still exceeds N, truncate `error_message` to first sentence.  
3. If still exceeds N, drop `strategies` and keep only `best_strategy`.  
4. Last resort: return only `error_code` and `target_line_content`.  

---

## 4. Technical Design  

### 4.1 WASM Build  

- **Workspace changes:** Add `crates/patch-ts-wasm/` containing the `wasm-bindgen` wrapper and JS shim.  
- **Conditional compilation:** Use `#[cfg(target_arch = "wasm32")]` to exclude `git2`, `tempfile`, `notify`, `crossterm`, `ratatui`, `tower-lsp`.  
- **File I/O abstraction:** Introduce a `FileSystem` trait with implementations for `std::fs` and `wasi-filesystem`.  

### 4.2 Entropy Calculation  

- Add a dependency on `entropy` crate or implement a simple character‑frequency entropy:  
  `H = -sum(p(x) * log2(p(x)))` for each character in the line.  
- Cache entropy scores per file to avoid recomputation on multiple recalls.  

### 4.3 Session Storage  

- Use a simple JSONL file per session.  
- Append‑only; no locking required (parallel calls to different sessions are safe).  
- Prune old sessions after 7 days via a background task on `recall` invocation (optional).  

### 4.4 Token Counting  

- Use `tiktoken-rs` or a simple heuristic (1 word ≈ 1.3 tokens) for budget estimation.  
- If exact token count is needed, integrate with the agent’s tokeniser via environment variable (`PATCH_TS_TOKENIZER=tiktoken`).  

---

## 5. Changes from v1.13.0  

| Aspect | Change |
|--------|--------|
| Dependencies | Added: `entropy`, `tiktoken-rs` (optional), `wasi-filesystem` |
| Workspace | Added `crates/patch-ts-wasm/` |
| Conditional comp. | Extensive `#[cfg(not(target_arch = "wasm32"))]` for CLI/TUI/LSP |
| Recall command | New flags: `--entropy`, `--entropy‑threshold`, `--pre‑fetch`, `--minimal`, `--session`, `--max‑tokens` |
| MCP recall tool | Updated `inputSchema` to include new parameters |

---

## 6. Implementation Plan (4 Sprints)  

### Sprint 1 – WASM Build Target  
- Set up `crates/patch-ts-wasm`  
- Implement `FileSystem` trait  
- Gate non‑WASM dependencies  
- Achieve first successful `cargo build --target wasm32-wasi`  
- JavaScript shim and basic demo  

### Sprint 2 – Entropy & Pre‑fetch  
- Implement entropy scoring in `recall.rs`  
- Add `--entropy`, `--entropy‑threshold`, `--pre‑fetch` flags  
- Integrate call graph into recall context  
- Unit tests for entropy selection  

### Sprint 3 – Compressed Format & Session Memory  
- Implement `--minimal` output  
- Create session storage and retrieval  
- Add `--session` flag  
- Update history suggestions with session data  
- Token budget mode (`--max‑tokens`)  

### Sprint 4 – Documentation & Polish  
- Update README, CHANGELOG  
- Create `docs/wasm.md`  
- Cross‑platform testing (WASI runtime)  
- Bump version to 1.14.0, tag, release  

---

## 7. Quality Gates  

- All existing tests pass (no regression).  
- `cargo build --target wasm32-wasi` succeeds for core and gate crates.  
- `recall --minimal` output <200 tokens for a typical E002 error.  
- `recall --entropy` reduces context size by ≥40% vs v1.13 recall on a 50‑line window.  
- Session memory persists and correctly updates strategy suggestions.  
- Token budget mode never exceeds specified limit.  
- WASM demo applies a patch to a small file in under 2 seconds.  

---

## 8. Risks & Mitigation  

| Risk | Likelihood | Mitigation |
|------|------------|------------|
| WASI filesystem not available in all browsers | Medium | Provide a virtual in‑memory FS implementation; clearly document limitations. |
| Entropy computation increases recall latency | Low | Cache scores per file; compute asynchronously if needed. |
| `tiktoken-rs` adds large dependency | Low | Make token counting optional; use word‑count heuristic by default. |
| Session files grow unboundedly | Low | Implement TTL‑based pruning on each session access. |

---

## 9. Documentation Checklist  

- [ ] `README.md` updated with WASM build instructions and new recall flags.  
- [ ] `CHANGELOG.md` entry for v1.14.0.  
- [ ] `docs/wasm.md` – how to use patch‑ts in browser/Node.js.  
- [ ] `docs/recall‑mode.md` updated with new entropy/minimal/session features.  
- [ ] `wasm/patch-ts.js` API documentation comments.  

---

*Specification approved for v1.14.0 development.*
