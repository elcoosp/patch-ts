# patch‑ts v1.15.0 — Specification  
**Intelligent Delimiter Repair & Patch Hardening**

## 1. Executive Summary  

Building on v1.14.0’s deeper token efficiency and WASM scaffold, **v1.15.0** tackles the most common failure mode for AI‑generated patches: **unbalanced delimiters**.  
The release introduces:

- **`heal` – intelligent delimiter repair** that combines tree‑sitter error recovery with heuristic‑guided search, outperforming current `balance` command.
- **Adversarial gate** – a new validation stage that stress‑tests patches with edge‑case inputs before they reach the test suite.
- **Repair strategy learning** – leveraging recall and session history to accelerate future repairs.
- **Pre‑patch validation** – automatic syntax check of proposed patch content, avoiding wasted attempts.

The result: **fewer failed patches, faster repairs, and more robust code**.

---

## 2. Problem Statement & Research Basis  

Unbalanced delimiters account for a significant portion of AI patch failures (PAGENT study, 2025).  
Current `patch‑ts balance` uses BFS minimum‑cost search, which:

- Works for 1‑3 errors but **exponential search space** defeats it for complex cases.
- Has **no language‑aware heuristics** – it treats all insertions/deletions equally.
- Cannot learn from past repairs.

Tree‑sitter’s error recovery provides valuable signals (`ERROR` nodes, mismatched delimiters) but is not optimised for producing **minimal repair suggestions**.  
Research from Agent Factory (Mozilla, 2025), InfCode, and CodeMender shows that combining AST feedback with heuristic pruning dramatically outperforms pure BFS or LLM‑based re‑generation.

patch‑ts v1.15.0 closes this gap.

---

## 3. Design Goals  

| ID | Goal | Measurement |
|----|------|-------------|
| G1 | Faster repairs | `heal` resolves common delimiter errors (1‑5 errors) in <50ms; BFS fallback for >5 errors. |
| G2 | Language‑aware heuristics | For Rust, Python, JavaScript, TypeScript: heuristic prioritises closing functions, blocks, and expressions correctly. |
| G3 | Adversarial robustness | `gate --stages adversarial` catches >=80% of edge‑case failures before they reach the test suite. |
| G4 | Repair learning | `heal --heuristic history‑guided` uses past successes to reduce search steps by >=40%. |
| G5 | Pre‑patch safety | `patch --validate‑first` prevents application of syntax‑breaking patches and provides recall context. |
| G6 | Backward compatibility | `balance` command unchanged; `heal` is additive. |

---

## 4. New Features  

### 4.1 `patch‑ts heal` — Intelligent Delimiter Repair  

**Command:**
```bash
patch-ts heal --file <FILE> [--apply] [--max‑cost <N>] [--heuristic <NAME>] [--json]
```

**Behaviour:**
1. Parse the file, extract all `ERROR` nodes from tree‑sitter.
2. For each error node, determine its context (inside `function_item`, `match_arm`, `string_literal`, etc.).
3. Apply the chosen heuristic to propose a repair sequence.
4. If `--apply` is set, write the repaired file; otherwise, print the sequence.

**Heuristics:**

| Heuristic | Description |
|-----------|-------------|
| `language‑aware` (default) | Uses tree‑sitter `ERROR` node parent/child relationships to prioritise repairs (e.g., missing `}` at end of function). |
| `cost‑weighted` | Uses context‑based cost function: inserting `}` to close a function costs 1, inserting `}` inside a string costs 100. Search is pruned by cost threshold. |
| `history‑guided` | Queries `.patch‑ts/repair‑history.jsonl` for similar error patterns and uses the most frequent successful repair as the first candidate. Falls back to `language‑aware` if no history. |
| `balanced` | Falls back to the existing BFS minimum‑cost search (same as `balance`). |

**Incremental parsing:**
During repair search, after each candidate edit, only the affected subtree is re‑parsed (using tree‑sitter `edit()`). This reduces validation time from O(n) to O(log n) per candidate.

**Output (JSON):**
```json
{
  "success": true,
  "actions": [
    {"type": "insert", "char": "}", "line": 42, "column": 1, "reason": "close function 'main'"},
    {"type": "delete", "char": ")", "line": 20, "column": 15, "reason": "extra parenthesis in expression"}
  ],
  "total_cost": 2,
  "heuristic_used": "language-aware"
}
```

### 4.2 `gate --stages adversarial` — Adversarial Validation  

**Command:**
```bash
patch-ts gate --file <FILE> --stages adversarial [--adversarial‑iterations <N>]
```

**Behaviour:**
- Generates adversarial inputs based on function signatures (empty strings, boundary values, type mismatches, Unicode edge cases).
- For each input, parses the patched file with tree‑sitter to detect syntax errors.
- Reports any failures as a `StageResult` with details.

**Adversarial input generation:**
- For each function in the file, extract parameter types.
- Generate: `null`/`None`, empty string, maximum integer, minimum integer, special characters, extremely long strings.
- If the file has no functions, apply syntax‑fuzzing rules (random deletion of a delimiter, insertion of an extra delimiter).

**Example output:**
```
❌ adversarial – 2 failures: function 'divide' with input (0, 0) caused panic; function 'parse' with empty string returned syntax error
```

### 4.3 Repair Strategy Learning  

**Storage:** `.patch‑ts/repair‑history.jsonl`

**Schema:**
```json
{
  "error_pattern": "missing '}' in function_item",
  "language": "rs",
  "repair_sequence": [
    {"action": "insert", "char": "}", "byte_offset": 452}
  ],
  "success": true,
  "timestamp": "2026-04-25T10:00:00Z",
  "agent": "Claude Code",
  "model": "sonnet"
}
```

**Integration:**
- On successful `heal`, record the error pattern and repair sequence.
- On subsequent `heal --heuristic history‑guided`, query the history for the same error pattern and use the most frequent successful repair as the initial candidate.
- If the candidate fails, fall back to `language‑aware` heuristic.

**Pruning:**  
Repair history automatically prunes entries older than 30 days on each write (maintains max 10,000 entries).

### 4.4 Pre‑Patch Validation (`--validate‑first`)  

**New flag for `patch`:**
```bash
patch-ts patch --file <FILE> --line <N> --old "<OLD>" --new "<NEW>" --validate‑first
```

**Behaviour:**
- Before applying the patch, construct the proposed new file content in memory.
- Parse it with tree‑sitter.
- If syntax errors are detected, **do not modify the file**. Instead:
  - Print the syntax error location and details.
  - Generate a `recall` context automatically (as if `patch‑ts recall` had been called).
  - Return exit code 3 (syntax error introduced).
- If syntax is clean, proceed with the patch normally.

**Token savings:** Prevents the agent from burning tokens on a patch that will definitely fail, without requiring an additional round‑trip.

---

## 5. Technical Design  

### 5.1 New/Modified Modules  

| Module | Change |
|--------|--------|
| `src/heal.rs` | New – core healing logic, heuristic engine, history query |
| `src/gate.rs` | Modified – add `gate_adversarial` stage; register in `run_stage` |
| `src/patch.rs` | Modified – add `--validate‑first` logic in `apply_patch_to_file` |
| `src/recall.rs` | Modified – record repair history on successful `heal` |
| `src/cli/heal.rs` | New – CLI handler for `heal` command |
| `src/cli/types.rs` | Modified – add `HealArgs`, `Heal(HealArgs)` variant, `--validate‑first` flag |

### 5.2 Dependency Changes  

| Crate | Version | Purpose |
|-------|---------|---------|
| (none new) | – | All new features use existing dependencies (tree‑sitter, serde, etc.) |

### 5.3 Heuristic Engine Design  

```
trait Heuristic {
    fn propose_repair(
        &self,
        content: &str,
        errors: &[DelimiterError],
        language: &dyn Language,
        parse_cache: &mut ParseCache,
        max_cost: usize,
    ) -> Option<(String, Vec<RepairAction>, usize)>;
}

struct LanguageAwareHeuristic;
struct CostWeightedHeuristic;
struct HistoryGuidedHeuristic;
struct BalancedHeuristic; // wraps existing BFS
```

### 5.4 Repair History Querying  

```rust
fn query_repair_history(
    error_pattern: &str,
    language: &str,
    limit: usize,
) -> Vec<RepairSequence>;
```

---

## 6. Implementation Plan (4 Sprints)  

### Sprint 1 – `heal` Core Engine  
- Implement `LanguageAwareHeuristic`  
- Integrate with tree‑sitter `ERROR` nodes  
- Implement incremental parsing for candidate validation  
- Basic CLI handler  

### Sprint 2 – Heuristic Variants & History  
- Implement `CostWeightedHeuristic` and `HistoryGuidedHeuristic`  
- Create repair history storage and querying  
- Add `--heuristic` flag support  

### Sprint 3 – Adversarial Gate & Pre‑Patch Validation  
- Implement `gate_adversarial` stage  
- Add `--validate‑first` to `patch` command  
- Register in gate pipeline and CLI  

### Sprint 4 – Documentation, Polish & Release  
- Update README, CHANGELOG  
- Create `docs/heal‑mode.md`  
- Cross‑platform testing  
- Bump version, tag, release  

---

## 7. Quality Gates  

- All existing `balance` tests pass unchanged.  
- `heal --heuristic language‑aware` resolves >=80% of single‑error delimiter issues in <50ms.  
- `heal --heuristic history‑guided` reduces search steps by >=40% after training on 10+ similar errors.  
- `gate --stages adversarial` catches >=80% of edge‑case failures on a curated set of 50 patches.  
- `patch --validate‑first` correctly rejects syntax‑breaking patches and provides recall context.  
- No regression in existing patch/balance/recall behavior.  

---

## 8. Risks & Mitigation  

| Risk | Likelihood | Impact | Mitigation |
|------|------------|--------|------------|
| Heuristic incorrectly prioritises wrong repair | Medium | Low | Fallback to `balanced` (BFS) if heuristic fails; unit test with diverse error patterns. |
| Incremental parsing introduces subtle errors | Low | Medium | Validate final result with full parse; compare incremental vs full parse in debug CI. |
| Adversarial input generation too slow for large files | Medium | Low | Limit adversarial iterations per file (default 5); generate inputs only for exported functions. |
| Repair history grows large | Low | Low | Implement size cap (10,000 entries) and TTL pruning (30 days). |

---

## 9. Documentation Checklist  

- [ ] `README.md` updated with `heal` command, `--validate‑first`, and adversarial gate.  
- [ ] `CHANGELOG.md` entry for v1.15.0.  
- [ ] `docs/heal‑mode.md` – detailed guide to `heal` heuristics and usage.  
- [ ] `docs/adversarial‑gate.md` – how adversarial validation works and how to interpret results.  

---

*Specification approved for v1.15.0 development.*
