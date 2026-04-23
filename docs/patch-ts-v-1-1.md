I'll guide you through creating a complete specification suite for implementing the Minimum-Cost Repair algorithm in patch-ts v1.1. This is an internal feature upgrade for an existing tool, so the documents will be scoped accordingly—focusing on the new algorithmic subsystem while leveraging existing context.

Given the depth of the change, I'll produce all five specification documents in sequence, using the spec-writer methodology. The documents will be tailored for a small open-source tool with a single maintainer (or small team), balancing rigor with pragmatism.

---

## patch-ts-v1.1-vision.md

```markdown
# Product Vision & Strategic Alignment — patch-ts v1.1 Minimum-Cost Repair

| Field | Value |
|-------|-------|
| Project | patch-ts |
| Document | Vision & Strategic Alignment |
| Version | 0.1 (Draft) |
| Date | 2026-04-23 |
| Author | elcoosp, assisted by AI |
| Status | Draft — Pending Review |

## 1. Vision Statement

**For AI agents and developers who rely on patch-ts for automated code modification**, the **Minimum-Cost Repair Engine** will replace the current heuristic delimiter balancing with a **search-based, evidence-driven repair algorithm**. **Unlike the legacy `balance` command** that frequently fails due to offset drift and naive stack matching, our new engine will **guarantee the smallest possible set of edits to produce syntactically valid code, with 100% reproducibility and zero silent corruption.**

## 2. Elevator Pitch

> For AI code-generation agents that need to repair malformed code snippets before applying patches, patch-ts v1.1 provides a mathematical minimum-cost delimiter repair engine. Unlike current ad‑hoc balancing heuristics that often introduce new errors or corrupt ASTs, our solution guarantees the smallest valid edit by treating syntax repair as a search problem over possible insertions and deletions of delimiters, validated by Tree-sitter parsing.

## 3. Problem Statement & Business Context

The current `patch-ts balance` command, as evidenced by disabled tests across 15+ language test suites and explicit notes in `repair_tests.rs`, is unreliable. It suffers from:
- **Fragile offset tracking**: Manual `offset_shift` calculations cause repair actions to target incorrect byte ranges after the first edit.
- **Naive stack-based detection**: The `traverse_for_delimiters` function cannot distinguish between delimiters inside strings/comments and those in code structure, leading to false positives.
- **No validation of repair quality**: The rollback mechanism only checks error count, not AST validity, often discarding correct repairs or keeping incorrect ones.

The consequence is that AI agents (the primary target audience) cannot reliably use the `balance` command to fix LLM-generated code with minor syntax errors, forcing them to implement custom, language-specific fixes or discard otherwise usable code.

**Why now?** The project is at v0.9.0 with a clear v1.0 roadmap. The `balance` feature is the last major component with known, test-disabled failures. Fixing it with a robust, algorithmic approach is the critical blocker for a stable v1.0 release and the subsequent v1.1 polish.

## 4. Target Users & Customers

| User class | Description | Key concern |
|------------|-------------|-------------|
| AI Coding Agents (primary) | LLM-based tools that generate and apply patches to user codebases. | Need a predictable, reliable way to fix minor syntax errors (missing/extra braces, parens) before applying a semantic patch. |
| Developer CLI Users | Individual developers using `patch-ts` to automate refactoring or apply pre‑written patches. | Want a `balance` command that "just works" without corrupting their files. |
| Plugin Developers | Authors of WASM plugins that provide custom repair logic. | Require a stable, well‑defined `DelimiterError` model and repair API. |

**Non‑target users:** The repair engine is *not* intended for general-purpose code formatting (use `rustfmt` or Prettier) or for fixing deep semantic errors (e.g., type mismatches). It is strictly for balancing paired delimiters.

## 5. User Needs & Value Proposition

| Need | How this release addresses it |
|------|------------------------------|
| **Reliable delimiter balancing** | Minimum‑cost repair guarantees the smallest valid edit set; searching over multiple candidate fixes eliminates heuristic guesswork. |
| **No file corruption** | Atomic writes and comprehensive test suite (re‑enabled and expanded) will ensure no silent data loss. |
| **Language‑agnostic core** | The algorithm operates on Tree‑sitter ASTs and the existing `DelimiterError` abstraction; no per‑language special‑casing. |
| **Predictable behavior** | Deterministic minimum‑cost output; same input always yields same repair. |
| **Extensibility** | Clean separation of detection (`find_delimiter_errors`) and repair search allows future enhancements (e.g., cost‑weighted by error type). |

**Differentiator:** Unlike all existing tools (which use heuristic pattern matching or full parser error recovery), patch-ts v1.1 will offer *verifiable minimal edit distance* repair, a property that gives AI agents high confidence in the output.

## 6. Desired Outcomes & Success Metrics

### Business/Project Outcomes

| ID | Outcome | Key Result / Metric |
|----|---------|---------------------|
| G-1 | **Stabilize `balance` command** | 100% of previously disabled `balance` and `repair` tests (currently ~15 suites with disabled tests) pass across all 16 supported languages. |
| G-2 | **Enable v1.0 release** | Zero open issues labeled `bug/balance` or `bug/repair` at v1.0 release time. |
| G-3 | **Improve user trust** | No new GitHub issues reporting "balance corrupted my file" within 3 months of release. |

### Product Outcomes (Behavioral)

| ID | Metric | Target |
|----|--------|--------|
| P-1 | Repair success rate on synthetic broken code (property‑based tests) | ≥ 99.5% of generated test cases produce a syntactically valid output. |
| P-2 | Median repair time for a 500‑line source file | ≤ 200 ms (to remain interactive for CLI users). |
| P-3 | Incorrect offset tracking bugs | Zero occurrences detected by new integration test suite across 10,000 random edits. |

## 7. Strategic Constraints

| Constraint | Description |
|------------|-------------|
| **Language scope** | Must support all 16 languages currently in `ast.rs` (Rust, TS/JS, Python, Go, Ruby, PHP, HTML/XML, C/C++, Java, C#, Swift, Scala, Zig). |
| **Tree‑sitter dependency** | Parser and AST generation remain unchanged; the new repair engine operates on the existing `ParseResult` and `DelimiterError` types. |
| **Plugin compatibility** | The WASM plugin interface (`repair.wit`) must remain backward‑compatible; plugins will still receive a list of `DelimiterError` and return a repaired string. |
| **Performance** | Repair must not be slower than current `balance` in the common case (a few errors). |
| **No new heavy dependencies** | The implementation should avoid pulling in crates that significantly increase compile time or binary size. `strsim` is already a dependency; it may be used for similarity scoring. |

## 8. Goals and Non‑Goals

### Goals

- G‑01: Implement a search‑based minimum‑cost repair algorithm that finds the smallest set of delimiter insertions/deletions yielding a syntactically valid AST.
- G‑02: Replace the manual `offset_shift` tracking with a stateless, re‑parse‑after‑every‑candidate approach, eliminating offset drift bugs.
- G‑03: Re‑enable all disabled `balance` and `repair` tests and achieve 100% pass rate.
- G‑04: Add property‑based tests (using `proptest`) to generate invalid code and assert that repair output is always valid.
- G‑05: Provide a new `--max-cost` CLI flag to bound search depth and prevent exponential blowup on severely broken files.

### Non‑goals

- **Full syntax error recovery**: The algorithm only repairs unbalanced delimiters (`( )`, `[ ]`, `{ }`). It will not insert missing keywords, fix operator precedence, or rename identifiers.
- **Semantic validation**: The output is guaranteed syntactically valid; no guarantee that code compiles or runs correctly.
- **Multi‑file repair context**: Repair is per‑file only; no cross‑file analysis.
- **Interactive mode**: No TUI for selecting among multiple valid repairs (always output the minimum‑cost unique solution; if tied, choose lexicographically first edit).
- **Performance under 100 ms for large files**: The primary goal is correctness; performance optimizations are secondary.

## 9. Operational Concept & High‑Level Scenarios

### Scenario 1: AI agent repairs LLM‑generated code

1. An AI coding agent generates a Rust function with a missing closing brace: `fn foo() { let x = 1;`
2. The agent invokes `patch-ts balance --file src/lib.rs --apply`.
3. The new minimum‑cost repair engine:
   - Detects a `Missing` error for `}` at the end of the file (cost = 1 insertion).
   - Generates candidate edits: insert `}` at the end.
   - Re‑parses; AST becomes valid. Accepts the repair.
4. File is atomically written with the repaired content.

### Scenario 2: Local developer with multiple errors

1. A developer accidentally introduced an extra `)` and missing `}` in a TypeScript file.
2. They run `patch-ts balance --dry-run` to preview changes.
3. The engine finds the minimum‑cost sequence (delete extra `)`, insert missing `}`) and outputs the diff.
4. Developer reviews and accepts by running without `--dry-run`.

### Scenario 3: Exceeded search cost bound

1. A user runs `balance` on a file with 100+ random delimiter insertions (severely broken).
2. The search space exceeds the default `max_cost` limit (e.g., 20 edits).
3. The tool exits with a clear error: "Unable to find repair within max cost of 20. Consider manual repair."
4. No file modification occurs.

## 10. Stakeholders & Governance

| Role | Person / Group | Responsibility |
|------|---------------|----------------|
| Executive Sponsor | elcoosp (maintainer) | Approves feature scope and v1.1 release. |
| Product Owner | elcoosp | Defines requirements, prioritizes tasks. |
| Engineering | elcoosp (+ community contributors) | Implements algorithm, re‑enables tests. |
| Users | AI agent developers, CLI users | Provide feedback via GitHub issues. |

**Decision model:** The maintainer has final say on algorithm design and release readiness. All changes will be proposed via GitHub PR and must pass full CI (including newly re‑enabled tests).

## 11. Traceability & Alignment Notes

The outcomes defined in Section 6 (G‑1, G‑2, G‑3) will be traced through requirements in the BRS and SRS. Each functional and non‑functional requirement in the SRS will reference one or more outcomes.

The Impact Map below shows the high‑level chain from goals to deliverables:

```
Goal G-1 (Stabilize balance) → Actors (AI agents, devs)
   → Impact: Confidence in repair → Deliverable: Minimum‑cost repair engine
   → Impact: Fewer support issues  → Deliverable: Re‑enabled test suite

Goal G-2 (Enable v1.0) → Actor (Maintainer)
   → Impact: Unblock release → Deliverable: Completed v1.1 feature set

Goal G-3 (Improve trust) → Actor (Users)
   → Impact: Positive word‑of‑mouth → Deliverable: Zero corruption bug reports
```

## 12. Risks, Assumptions, and Open Questions

### Assumptions

| ID | Assumption |
|----|------------|
| A-1 | Tree‑sitter's incremental parsing can be avoided by re‑parsing from scratch after each candidate edit; performance will remain within acceptable limits. |
| A-2 | The existing `DelimiterError` detection logic (`find_delimiter_errors_via_ast`) is sufficiently accurate for all 16 languages; false negatives/positives will not break the minimum‑cost search. |
| A-3 | `strsim` crate provides adequate string similarity for tie‑breaking when multiple repairs have equal cost. |

### Risks

| ID | Risk | Likelihood | Impact | Mitigation |
|----|------|------------|--------|------------|
| R-1 | Search space explosion for files with many errors. | Medium | High | Implement `--max-cost` flag; default to 10. |
| R-2 | Some languages (e.g., Ruby, Scala) have delimiter‑like keywords that confuse the AST traversal. | Medium | Medium | Focus initial validation on Rust/TS/JS; expand language coverage incrementally with integration tests. |
| R-3 | Re‑enabling all 15+ language balance tests reveals unforeseen edge cases. | High | Low | Accept that some tests may initially fail; triage and fix incrementally before v1.0. |

### Open Questions

| ID | Question | Owner | Due |
|----|----------|-------|-----|
| Q-1 | Is `strsim::normalized_levenshtein` the best similarity metric for tie‑breaking? Or should we use token‑based Jaccard similarity (already in `matching.rs`)? | elcoosp | Before implementation start. |
| Q-2 | Should the minimum‑cost search consider both insertions and deletions concurrently (BFS over edit space) or use a greedy approach with backtracking? | elcoosp | To be decided in ADR. |
| Q-3 | Does the WASM plugin interface need to be extended to expose the minimum‑cost search? | elcoosp | No; plugin receives the final repair string as before. |
```

---

## patch-ts-v1.1-brs.md

```markdown
# Business & Stakeholder Requirements Specification — patch-ts v1.1 Minimum-Cost Repair

| Field | Value |
|-------|-------|
| Project | patch-ts |
| Document | Business & Stakeholder Requirements Specification (BRS) |
| Version | 0.1 (Draft) |
| Date | 2026-04-23 |
| Author | elcoosp, assisted by AI |
| Status | Draft — Pending Review |

## 1. Business Context

### 1.1 Purpose

This document defines the business and stakeholder requirements for implementing a **Minimum‑Cost Repair Engine** in patch-ts, replacing the current heuristic `balance` command. The primary business objective is to achieve **release readiness for v1.0** by eliminating the last known source of unreliability in the tool.

### 1.2 Business Problem / Opportunity

**Problem**: The existing delimiter balancing feature (`patch-ts balance`) is fragile and fails in non‑trivial cases. This is evident from:
- 15+ language test suites with disabled `balance` tests.
- User reports (anecdotally) of file corruption.
- High maintenance burden for the maintainer to troubleshoot offset‑tracking bugs.

**Opportunity**: Implementing a rigorous, algorithmically sound repair method will:
- Restore confidence in the tool among AI agent developers.
- Differentiate patch-ts from other syntax‑aware patching tools.
- Unblock the v1.0 release and establish a foundation for future enhancements.

### 1.3 Scope Boundaries

**In Scope:**
- New repair algorithm that finds minimal delimiters edits (insertion/deletion) to produce valid Tree‑sitter AST.
- Replacement of `offset_shift` tracking with stateless re‑parsing.
- Comprehensive test suite covering all supported languages.
- CLI flag `--max-cost` to limit repair search depth.

**Out of Scope:**
- Repairing semantic errors (type errors, undefined variables).
- Language‑specific custom repair heuristics.
- Multi‑file balancing or cross‑file context awareness.
- Integration with language servers (LSP).

## 2. Business Goals, Objectives & Success Metrics

| ID | Goal / Objective | Fit Criterion |
|----|------------------|---------------|
| BG-1 | **Achieve v1.0 release readiness** | All existing `balance` and `repair` tests pass; no open `bug/balance` issues at release. |
| BG-2 | **Eliminate file corruption reports** | Zero user reports of corruption caused by `balance` command within 3 months post‑release. |
| BG-3 | **Reduce maintenance overhead** | Number of bug fixes related to `repair.rs` decreases by 80% YoY after v1.1 release. |
| BG-4 | **Maintain backwards compatibility** | 100% of existing CLI invocations and plugin interface calls behave identically for valid/unchanged files. |

## 3. Business Model & Processes

patch-ts is an open‑source tool, not a commercial product. The "business model" is sustainability and adoption.

**Value Proposition:** Provide AI agents with a reliable, language‑agnostic patching CLI that understands code structure, not just text.

**Core Process (Current):** Users or AI agents invoke `patch-ts patch` or `patch-ts balance` on one or more files. The tool uses Tree‑sitter to parse, fuzzy‑match expected content, and apply edits. For `balance`, it attempts to fix unmatched delimiters before applying a patch.

**Core Process (Future v1.1):** Same flow, but the `balance` step uses a **search‑based minimum‑cost** repair instead of heuristic stack matching. This increases reliability without changing the user experience.

## 4. Business Rules & Policies

| ID | Rule | Source |
|----|------|--------|
| BR-1 | Repaired code must be syntactically valid according to the target language's Tree‑sitter grammar. | Project quality standard |
| BR-2 | Repairs must be **minimal**: the algorithm must not insert/delete more delimiters than strictly necessary to achieve validity. | Feature definition |
| BR-3 | If multiple minimal repairs exist, the tool must choose **deterministically** (e.g., lexicographically first edit location). | Predictability requirement |
| BR-4 | The tool must never modify a file that is already syntactically valid. | User expectation |
| BR-5 | If a repair cannot be found within a user‑configurable cost bound, the command must exit with a non‑zero exit code and not modify the file. | Safety policy |

## 5. Stakeholders & User Classes

| Stakeholder / User Class | Description | Primary Needs |
|--------------------------|-------------|---------------|
| **AI Coding Agent (Primary User)** | LLM‑powered tool that generates code patches. | Predictable, deterministic repair of minor syntax errors; minimal false positives. |
| **Developer CLI User** | Human developer using patch-ts for automation. | No corruption; clear error messages when repair impossible. |
| **Maintainer (elcoosp)** | Responsible for project health and releases. | Codebase maintainability; reduced bug reports; clear algorithmic design. |
| **Plugin Developer** | Author of WASM repair plugins. | Stable API; clear contract for `DelimiterError` input. |

## 6. Glossary / Ubiquitous Language

| Term | Definition |
|------|------------|
| **Delimiter** | Parentheses `()`, square brackets `[]`, or curly braces `{}`. Only these three types are considered for balancing. |
| **DelimiterError** | A data structure (`ast::DelimiterError`) representing either an extra delimiter or a missing delimiter at a specific byte span. |
| **Minimum‑Cost Repair** | The sequence of delimiter insertions and deletions with the smallest total number of edits that yields a syntactically valid Tree‑sitter AST. |
| **Fit Criterion** | A measurable condition that verifies a requirement is satisfied (Volere terminology). |
| **Repair Candidate** | A proposed set of edits (list of `DelimiterError`‑like actions) that, when applied, produce a modified source string. |
| **ParseResult** | The output of `Language::parse()`: a Tree‑sitter `Tree`, source string, and line index. |
| **Offset Drift** | The classic bug where applying one edit changes byte offsets for subsequent edits, making later repairs target wrong locations. |

## 7. Conceptual Domain Model

The domain model for the repair system is intentionally small, as it only deals with delimiters and parsing.

```
┌─────────────────┐       applies       ┌─────────────────┐
│ DelimiterError  │────────────────────▶│   RepairAction  │
│ - kind          │                     │ - insert(char)  │
│ - span          │                     │ - delete        │
└─────────────────┘                     └─────────────────┘
         │                                      │
         │ detected by                          │ composed into
         ▼                                      ▼
┌─────────────────┐       validates       ┌─────────────────┐
│  ParseResult    │◀──────────────────────│  RepairCandidate│
│ - tree          │                        │ - edits[]       │
│ - source        │                        │ - cost          │
└─────────────────┘                        └─────────────────┘
```

- **ParseResult** is the source of truth for syntax validity.
- **DelimiterError** list is the input to the repair search.
- **RepairCandidate** is evaluated by applying edits to a copy of the source and re‑parsing.
- The goal is to find the **minimum‑cost** candidate that yields a valid AST.

## 8. Stakeholder Needs & User Requirements

| ID | Stakeholder Need | User Requirement |
|----|------------------|------------------|
| SN-1 | As an AI agent, I need to fix a file with a missing closing brace so that I can apply a semantic patch without syntax errors. | The system shall accept a file path and produce a repaired version with the missing brace inserted at the correct location. |
| SN-2 | As a developer, I need to preview the repairs before applying them to ensure they are sensible. | The system shall support a `--dry-run` flag that outputs the diff without modifying the file. |
| SN-3 | As a maintainer, I need the repair algorithm to be verifiably correct to reduce debugging time. | The system shall be accompanied by a property‑based test suite that asserts output validity for randomly broken inputs. |
| SN-4 | As a plugin developer, I need the `DelimiterError` list passed to my plugin to be accurate so that I can apply custom heuristics. | The system shall populate `DelimiterError` with accurate spans before invoking plugin repair. |

## 9. System‑in‑Context & Operational Concept

The repair engine is a subsystem within `patch-ts`, invoked via the `balance` CLI command (or internally during `patch` if `--no-auto-repair` is not set).

**Operational Flow:**
1. User/AI invokes `patch-ts balance --file <path>`.
2. CLI parses arguments and expands glob patterns if `--files` is used.
3. For each file:
   a. Detect language and parse file with Tree‑sitter.
   b. Call `find_delimiter_errors` to get list of errors.
   c. If errors exist, invoke **Minimum‑Cost Repair Search**.
   d. If repair found and not `--dry-run`, write repaired content atomically (with backup).
   e. Output JSON diagnostics if `--json` flag present.
4. Exit with success (0) if all files repaired or already valid; non‑zero if any file exceeds max cost.

## 10. Stakeholder‑Level Constraints & Quality Expectations

| Constraint / Quality | Stakeholder Expectation |
|----------------------|-------------------------|
| **Correctness** | Repairs must not introduce new syntax errors. |
| **Performance** | Repair time should be acceptable for interactive CLI use (<1s for typical files). |
| **Determinism** | Same input must always produce same output. |
| **Safety** | No file modification on failure. |
| **Observability** | JSON output must include repair actions taken (line/column, delimiter, type). |
| **Compatibility** | All existing CLI flags (`--fuzz`, `--function`, `--plugin`, etc.) must continue to work. |

## 11. Risks, Assumptions & Open Issues

Same as documented in the Vision document (Section 12); reiterated here for traceability.

## 12. Traceability Mapping to Vision

| Vision Outcome ID | Stakeholder Need ID | Feature (Epic) |
|-------------------|---------------------|----------------|
| G-1 (Stabilize balance) | SN-1, SN-3 | Minimum‑cost repair engine |
| G-2 (Enable v1.0) | SN-2, SN-4 | Re‑enabled test suites, updated docs |
| G-3 (Improve trust) | SN-1, SN-2 | Zero corruption guarantee |

Each system requirement in the SRS will reference one or more SN‑IDs.
```

---

## patch-ts-v1.1-srs.md

```markdown
# Software Requirements Specification — patch-ts v1.1 Minimum-Cost Repair

| Field | Value |
|-------|-------|
| Project | patch-ts |
| Document | Software Requirements Specification (SRS) |
| Version | 0.1 (Draft) |
| Date | 2026-04-23 |
| Author | elcoosp, assisted by AI |
| Status | Draft — Pending Review |

## 1. Introduction and Scope

### 1.1 Purpose

This SRS specifies the functional and non‑functional requirements for the **Minimum‑Cost Repair Engine** feature in patch-ts v1.1. It builds on the existing `balance` command infrastructure but replaces the core repair logic with a search‑based algorithm.

### 1.2 Scope

**In Scope:**
- `repair::balance_file` and `repair::quick_balance` rewritten to use minimum‑cost search.
- New internal modules: `repair::search` containing BFS/A* search over edit space.
- New CLI flag `--max-cost <N>` (default 10).
- Re‑enabled and expanded test suites (`repair_tests.rs`, language‑specific balance tests).

**Out of Scope:**
- Changes to `patch` command logic (except indirectly via `quick_balance`).
- New plugin capabilities.
- Integration with external tools.

### 1.3 References

- Vision v0.1 (`patch-ts-v1.1-vision.md`)
- BRS v0.1 (`patch-ts-v1.1-brs.md`)
- patch-ts source code (v0.9.0)

## 2. System Context & Overview

The repair engine sits within the `patch_ts::repair` module. Its primary entry points are:

- `balance_file(file_path, function_name, dry_run, language, plugin_path) -> Result<BalanceResult>`
- `quick_balance(content, language) -> Option<String>`

The new module `repair::search` will handle the search logic, leaving `repair.rs` to orchestrate file I/O, error detection, and output.

## 3. Functional Capabilities and Behavior

### Capability: Minimum‑Cost Delimiter Repair Search

**Goal:** Given a source string and a list of `DelimiterError` objects, find the smallest set of insertions and deletions of delimiters that yields a syntactically valid AST.

#### 3.1 Search Algorithm Behavior

- **REQ-FUNC‑001**: The system shall implement a search algorithm (BFS, Dijkstra, or A*) over the space of possible edit sequences.
  - **Priority:** Must have.
  - **Acceptance Criteria:** Algorithm correctly identifies minimal edit distance for all test fixtures.
  - **Trace:** SN‑1, SN‑3.

- **REQ-FUNC‑002**: Each state in the search space consists of a modified source string and the set of remaining errors (or a cost accumulator).
  - **Priority:** Must have.
  - **Acceptance Criteria:** States are correctly memoized to avoid redundant re‑parsing.

- **REQ-FUNC‑003**: The cost of an edit shall be defined as:
  - Insertion of a delimiter: cost 1.
  - Deletion of a delimiter: cost 1.
  - (Future versions may weight by delimiter type; v1.1 uses uniform cost 1.)
  - **Priority:** Must have.
  - **Trace:** BR‑2 (Minimal repair).

- **REQ-FUNC‑004**: The system shall treat a candidate repair as **valid** if and only if `Language::is_valid(&parse_result)` returns `true`.
  - **Priority:** Must have.
  - **Trace:** BR‑1 (Validity).

- **REQ-FUNC‑005**: If multiple repair sequences have equal minimum cost, the system shall deterministically select one. The tie‑breaker shall be:
  1. Prefer deletions over insertions when spans are identical.
  2. Otherwise, compare lexicographically by (edit type, byte offset).
  - **Priority:** Should have.
  - **Trace:** BR‑3 (Determinism).

- **REQ-FUNC‑006**: The search shall be bounded by a maximum cost `--max-cost` (default 10). If no valid repair is found within the bound, the system shall return a `RepairExceededMaxCost` error.
  - **Priority:** Must have.
  - **Trace:** BR‑5 (Cost bound).

**Edge Cases and Unwanted Behavior (EARS‑style):**

- **REQ-FUNC‑007**: If the initial source string is already syntactically valid (`Language::is_valid` true), the system shall return the original content unchanged and report zero actions taken.
- **REQ-FUNC‑008**: If `find_delimiter_errors` returns an empty list but the AST is invalid, the system shall return an error indicating that no delimiter errors were detected (possible grammar error beyond delimiter imbalance).
- **REQ-FUNC‑009**: If a candidate edit introduces new syntax errors, that state shall be pruned from the search (cost increases without reaching validity).
- **REQ-FUNC‑010**: When applying edits during search, the system shall re‑parse the entire source string from scratch to avoid offset drift complexity. Performance shall be monitored but correctness takes precedence.

### Capability: Integration with Existing `balance` Command

- **REQ-FUNC‑011**: The `balance_file` function shall replace its current repair loop with a call to the minimum‑cost search module.
  - **Priority:** Must have.
  - **Acceptance Criteria:** All existing `balance` CLI tests pass when re‑enabled.

- **REQ-FUNC‑012**: The `BalanceResult` struct shall contain the list of `BalanceAction` objects representing the edits applied by the minimum‑cost search.
  - **Priority:** Must have.
  - **Action types:** `"insert"` or `"remove"`, along with delimiter and location.

- **REQ-FUNC‑013**: The `--dry-run` flag shall output the proposed edits without modifying the file, as before.
  - **Priority:** Must have.

- **REQ-FUNC‑014**: The `--plugin` flag shall continue to work: if a plugin is provided, the plugin's `repair` function is called *before* the minimum‑cost search (allowing plugins to perform custom transformations first). The minimum‑cost search then operates on the plugin's output.
  - **Priority:** Should have.
  - **Rationale:** Preserves existing behavior where plugin takes precedence.

- **REQ-FUNC‑015**: The `quick_balance` function (used by `patch`) shall be updated to use a fast‑path: attempt a single‑edit repair (extra or missing) before falling back to the full minimum‑cost search with a low max‑cost (e.g., 3).
  - **Priority:** Should have.
  - **Rationale:** Maintain performance for the common case of simple auto‑repair during patching.

### Capability: Rollback and Safety

- **REQ-FUNC‑016**: The `balance_file` function shall not modify the original file if the minimum‑cost search fails to find a valid repair (including exceeding max cost).
  - **Priority:** Must have.
  - **Trace:** BR‑5.

- **REQ-FUNC‑017**: If `--no-backup` is false (default), the system shall create a backup copy before writing the repaired file.
  - **Priority:** Must have.

### Capability: Progress Indication and Observability

- **REQ-FUNC‑018**: When `--json` flag is present, the output shall include a `repair_cost` field indicating the total cost of the applied repair.
  - **Priority:** Could have.
  - **Rationale:** Helps AI agents understand how much repair was needed.

- **REQ-FUNC‑019**: The `BalanceAction` entries in JSON output shall include `byte_offset` and `line/column` information for each edit.
  - **Priority:** Must have.

## 4. Quality and Non‑Functional Requirements

| ID | Category | Requirement | Fit Criterion |
|----|----------|-------------|---------------|
| NFR‑PERF‑001 | Performance | The minimum‑cost search for a file with ≤3 delimiter errors shall complete in under 500 ms on typical hardware. | Ninety‑fifth percentile time measured across a corpus of 100 open‑source Rust files with artificially introduced errors. |
| NFR‑PERF‑002 | Performance | The search shall not consume more than 1 GB of memory. | Peak RSS measured during benchmark execution. |
| NFR‑REL‑001 | Reliability | The algorithm shall never produce invalid AST from valid input. | Property‑based test with 10,000 random edits asserts that output is valid Tree‑sitter AST. |
| NFR‑REL‑002 | Reliability | The algorithm shall be deterministic. | Two runs on same input produce identical output bytes. |
| NFR‑MAINT‑001 | Maintainability | The search module shall be implemented in a separate file (`repair/search.rs`) with clear interface, documented with comments. | Code review checklist: all public functions have doc comments; algorithmic steps explained in module docs. |
| NFR‑COMPAT‑001 | Compatibility | All existing CLI commands (`patch`, `explain`) shall continue to function unchanged. | Full test suite passes. |

## 5. External Interfaces and Data Contracts

The external interface for the repair engine is the `repair::balance_file` function signature and the CLI. No new public APIs are introduced.

The internal interface between `repair.rs` and `search.rs`:

```rust
/// Given a source string and a list of delimiter errors, attempt to find
/// a minimal set of edits (insertions/deletions) that yields a valid AST.
///
/// Returns `Some(patched_string, actions_taken, total_cost)` if repair succeeds.
/// Returns `None` if no repair within `max_cost`.
pub fn minimum_cost_repair(
    source: &str,
    errors: &[DelimiterError],
    language: &mut dyn Language,
    max_cost: usize,
) -> Option<(String, Vec<RepairAction>, usize)>;

#[derive(Debug, Clone)]
pub enum RepairAction {
    Insert { ch: char, pos: usize }, // byte offset
    Delete { span: Span },
}
```

The `quick_balance` function signature remains unchanged; it will internally call `minimum_cost_repair` with a low `max_cost`.

## 6. Constraints, Assumptions, and Dependencies

### Constraints
- **CON‑001**: The implementation must use only safe Rust (no `unsafe` except where required by Tree‑sitter bindings, which are already wrapped).
- **CON‑002**: The dependency `strsim` may be used for tie‑breaking similarity scoring; other new dependencies require maintainer approval.
- **CON‑003**: The repair algorithm must not rely on language‑specific heuristics; it must work uniformly for all 16 languages.

### Assumptions
- **ASM‑001**: Tree‑sitter parsing is fast enough for repeated re‑parsing during search (cached via memoization of source strings).
- **ASM‑002**: The `find_delimiter_errors` function correctly identifies all unbalanced delimiters with accurate spans for all supported languages.

### Dependencies
- Existing crates: `tree-sitter`, `line-index`, `strsim`.
- No new runtime dependencies.

## 7. TBD Log

| ID | Item | Owner | Due |
|----|------|-------|-----|
| TBD‑001 | Final decision on search algorithm type (BFS vs Dijkstra vs A*) and tie‑breaking metric. | elcoosp | Before implementation start |
| TBD‑002 | Exact value for default `--max-cost`. | elcoosp | Before beta testing |
| TBD‑003 | Performance benchmarks to establish baseline. | elcoosp | During implementation |

## 8. Requirements Attributes and Traceability Model

**ID Scheme:**
- Functional: `REQ-FUNC‑###`
- Non‑functional: `NFR‑CAT‑###`

**Priority:** Must / Should / Could (MoSCoW)

**Traceability links:** Each requirement shall reference one or more Stakeholder Need IDs from the BRS (SN‑###) and/or Business Rules (BR‑###).

The Requirements Traceability Matrix will be maintained in the test specification (`patch-ts-v1.1-test.md`).

## 9. Verification Method per Requirement (Summary)

| Requirement | Primary Verification Method |
|-------------|----------------------------|
| REQ-FUNC‑001–006 | Test (automated integration and property‑based tests) |
| REQ-FUNC‑007–010 | Test (unit tests for search edge cases) |
| REQ-FUNC‑011–015 | Test (re‑enabled CLI tests) |
| REQ-FUNC‑016–017 | Test (simulated failure paths) |
| NFR‑PERF‑001–002 | Test (benchmark suite) |
| NFR‑REL‑001–002 | Test (property‑based) |
| NFR‑MAINT‑001 | Inspection (code review) |
| NFR‑COMPAT‑001 | Test (full test suite) |
```

---

## patch-ts-v1.1-architecture.md

```markdown
# Architecture & Design Specification — patch-ts v1.1 Minimum-Cost Repair

| Field | Value |
|-------|-------|
| Project | patch-ts |
| Document | Architecture & Design Specification |
| Version | 0.1 (Draft) |
| Date | 2026-04-23 |
| Author | elcoosp, assisted by AI |
| Status | Draft — Pending Review |

## 1. Context and Scope

This document describes the architecture and key design decisions for the Minimum‑Cost Repair Engine, a new subsystem within patch-ts targeted for v1.1. It focuses on the `repair` module and its interaction with the existing parser and CLI layers.

## 2. Goals and Non‑Goals

### Goals (Design‑level)
- **G‑ARCH‑01**: Replace manual offset‑tracking repair loop with stateless search.
- **G‑ARCH‑02**: Introduce a clean separation between error detection (`ast.rs`) and repair search (`repair/search.rs`).
- **G‑ARCH‑03**: Maintain backward compatibility with existing `balance` CLI and WASM plugin interface.
- **G‑ARCH‑04**: Ensure the search algorithm is unit‑testable in isolation.

### Non‑Goals
- **No changes to `ast.rs`**: The delimiter detection remains unchanged in v1.1 (though future improvements may follow).
- **No overlapping with `matching.rs` fuzzy‑matching**: Those heuristics are separate and unchanged.
- **No new persistent state**: No caching across invocations.

## 3. Architecturally Significant Requirements (ASRs)

From the SRS, the following requirements shape the architecture:

| ASR ID | Description | Impact |
|--------|-------------|--------|
| ASR‑PERF‑001 | Search must complete in < 500 ms for ≤3 errors. | Requires memoization of parsed results to avoid redundant work. |
| ASR‑REL‑001 | Must never produce invalid AST. | Demands that every candidate edit is validated by re‑parsing; no heuristic acceptance. |
| ASR‑REL‑002 | Deterministic output. | Tie‑breaking rules must be algorithmic, not random. |
| ASR‑MAINT‑001 | Clean separation of concerns. | Search logic must reside in its own module. |

## 4. The Design

### 4.1 System Overview

The repair engine operates as a **search over edit sequences**. At each step, the algorithm generates candidate edits (insert a delimiter at an error span, delete an extra delimiter), applies them to a copy of the source, re‑parses, and checks validity.

We use **Breadth‑First Search (BFS)** over the cost of edit sequences. BFS naturally finds shortest‑path solutions. State memoization by source string hash prevents redundant exploration.

```
┌──────────────┐     find errors     ┌─────────────────┐
│ ParseResult  │────────────────────▶│ Vec<DelimError> │
└──────────────┘                     └─────────────────┘
                                              │
                                              ▼
┌──────────────┐     search loop     ┌─────────────────┐
│ MinCostSearch│◀───────────────────▶│   RepairState   │
│ - queue      │                      │ - source        │
│ - visited    │                      │ - cost          │
│ - max_cost   │                      │ - history       │
└──────────────┘                      └─────────────────┘
         │
         │ success
         ▼
┌──────────────┐
│ BalanceResult│
└──────────────┘
```

### 4.2 C4 Model Descriptions

#### C1 – System Context (patch-ts overall)
*(unchanged from existing documentation; repair engine is internal)*

#### C2 – Container View
The repair engine is a logical component within the `patch-ts` binary. It consumes `Language` trait objects and produces `String` and `BalanceResult`.

#### C3 – Component View (Repair Subsystem)

- **`repair::search::MinimumCostRepair`**: Entry point for the search.
  - **Responsibility**: Orchestrate BFS; manage visited set; return optimal repair.
  - **Collaborators**: `DelimiterError`, `Language`, `RepairAction`.
- **`repair::RepairAction`**: Enum representing an edit (Insert or Delete).
- **`repair::RepairState`**: Internal struct holding current source, cost, and sequence of actions.

### 4.3 Key Data Flows

**Search Algorithm (BFS):**

1. Initialize queue with state (original source, cost=0, empty actions).
2. While queue not empty:
   a. Pop state with lowest cost.
   b. Parse source with `Language`.
   c. If AST valid → return actions and cost.
   d. If cost == `max_cost` → continue (do not expand further).
   e. Compute `errors = language.find_delimiter_errors(&parse_result)`.
   f. For each error, generate child states:
      - For `Extra` error: delete the extra delimiter (cost+1).
      - For `Missing` error: insert the missing delimiter at the end of the span (cost+1).
   g. For each child:
      - Compute source hash; if not visited, add to queue and visited set.
3. If queue exhausted → return `None` (no repair within cost bound).

**Memoization:** We compute a hash of the source string (e.g., using `std::collections::hash_map::DefaultHasher`) and store it in a `HashSet`. This prevents re‑exploring states that have already been tried.

### 4.4 Data Model and Storage Approach

No persistent storage. The search uses in‑memory data structures only.

### 4.5 Security Architecture

No new security concerns; existing atomic file writes and backup mechanisms remain in place.

## 5. Architecture Decision Records (ADRs)

### ADR‑001: Use BFS for Minimum‑Cost Search

**Context:** We need to find the sequence of delimiter edits with minimal total cost. All edit costs are uniform (1). BFS on cost is equivalent to Dijkstra and simpler to implement.

**Decision Drivers:** ASR‑PERF‑001 (performance), ASR‑REL‑002 (determinism).

**Considered Options:**
- **A* search with heuristic**: Heuristic would estimate remaining edit distance; but estimating remaining delimiter errors from an AST is complex and error‑prone. Rejected due to risk of over‑engineering.
- **Depth‑First Search with backtracking**: Could find a solution quickly but may not find the minimal one without exhaustive search. Rejected because minimality is a core requirement.
- **Bounded BFS**: Simple, guaranteed to find minimal solution, and with memoization of visited states performs well for expected small error counts (≤10). **Chosen**.

**Consequences:**
- Positive: Simple to implement, test, and reason about.
- Negative: May explore many states if max cost is high; `--max-cost` flag mitigates this.

### ADR‑002: Re‑parse From Scratch Instead of Incremental Parsing

**Context:** Applying an edit changes byte offsets for all subsequent edits. Tree‑sitter supports incremental parsing via `tree.edit()` but requires careful management of `InputEdit` structs and the old tree.

**Decision Drivers:** ASR‑REL‑001 (correctness over performance), ASR‑MAINT‑001 (simplicity).

**Considered Options:**
- **Incremental parsing with `tree.edit()`**: Would be faster but complex and error‑prone; would require threading the previous `Tree` through the search state, which is large and difficult to memoize.
- **Re‑parse from scratch for each candidate**: Slower but dramatically simpler and guarantees correctness. Memoization by source hash reduces redundant parsing. **Chosen**.

**Consequences:**
- Positive: Eliminates entire class of offset‑drift bugs; code is trivial to understand.
- Negative: Potentially slower; but benchmarks (to be run) expected to show acceptable performance for typical files (few errors, moderate file size).

### ADR‑003: Use Lexicographic Tie‑Breaking on Edit Descriptions

**Context:** When multiple edit sequences have equal cost, we must pick deterministically.

**Decision Drivers:** ASR‑REL‑002.

**Considered Options:**
- **String similarity to original**: Use `strsim` to pick the repair most similar to original. This is intuitive but computationally expensive and may not be deterministic if similarities tie.
- **Prefer deletions first**: Deletions remove noise; deletions before insertions is a sensible heuristic.
- **Lexicographic order of (type, byte‑offset)**: Unambiguous, fast, and deterministic. **Chosen**.

**Consequences:**
- Positive: Very fast to compute during BFS queue ordering.
- Negative: The chosen repair might not be the "most natural" to a human, but predictability outweighs human preference for an AI tool.

## 6. API & Interface Contracts

The public interface is the existing `repair::balance_file` function. Internally, a new private module `repair::search` with:

```rust
pub fn minimum_cost_repair(
    source: &str,
    errors: &[DelimiterError],
    language: &mut dyn Language,
    max_cost: usize,
) -> Option<(String, Vec<RepairAction>, usize)>;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RepairAction {
    Insert { ch: char, pos: usize },
    Delete { start: usize, end: usize },
}
```

## 7. Cross‑Cutting Concerns

### Observability
- When repair is successful, log (at `debug` level) the number of states explored and total cost.
- Include repair actions in JSON output (`BalanceResult.actions`).

### Error Handling
- If search fails, return a structured error (`RepairError::MaxCostExceeded`).
- All panics must be avoided; use `Result` throughout.

### Testing Strategy
- Unit tests for BFS on small artificial examples.
- Integration tests using files from language‑specific test suites.
- Property‑based tests generating random delimiter insertions/deletions and asserting output validity.

## 8. Alternatives Considered

| Alternative | Why Rejected |
|-------------|--------------|
| Keep current heuristic and iterate | Would not meet reliability goals; offset drift too complex to fix incrementally. |
| Use Tree‑sitter error recovery nodes | Does not yield minimal edits; often marks large regions as `ERROR`, providing no actionable fix. |
| Machine‑learning based repair | Overkill for the problem; adds huge dependency and maintenance burden. |

## 9. Traceability (ASRs → ADRs → Components)

| ASR ID | Addressed by ADR(s) | Addressed by Component(s) |
|--------|---------------------|---------------------------|
| ASR‑PERF‑001 | ADR‑002 (memoization) | `MinimumCostRepair::visited` set |
| ASR‑REL‑001 | ADR‑002 (re‑parse) | Full parse in BFS loop |
| ASR‑REL‑002 | ADR‑003 | Tie‑breaking in BFS queue ordering |
| ASR‑MAINT‑001 | ADR‑001 (separate module) | `repair/search.rs` |
```

---

## patch-ts-v1.1-test.md

```markdown
# Behavioral Specification & Test Verification Plan — patch-ts v1.1 Minimum-Cost Repair

| Field | Value |
|-------|-------|
| Project | patch-ts |
| Document | Test Verification Plan |
| Version | 0.1 (Draft) |
| Date | 2026-04-23 |
| Author | elcoosp, assisted by AI |
| Status | Draft — Pending Review |

## 1. Behavioral Specifications (Specification by Example)

### Feature: Minimum‑Cost Repair of Delimiter Errors

#### Scenario 1: Single missing brace — minimal insertion

```gherkin
Given a Rust source file with content:
"""
fn main() {
    let x = 42;
"""
And the file is syntactically invalid
When I run `patch-ts balance --file <file> --apply`
Then the repaired content shall be:
"""
fn main() {
    let x = 42;
}
"""
And the exit code shall be 0.
```

#### Scenario 2: Single extra brace — minimal deletion

```gherkin
Given a Rust source file with content:
"""
fn main() {
    println!("hi");
}
}
"""
When I run `patch-ts balance --file <file> --apply`
Then the repaired content shall be:
"""
fn main() {
    println!("hi");
}
"""
```

#### Scenario 3: Two errors (one extra, one missing) — cost 2 repair

```gherkin
Given a Rust source file with content:
"""
fn main() {
    let x = (1 + 2));
}
"""
# This has an extra ')' and a missing '}'
When I run `patch-ts balance --file <file> --apply`
Then the repaired content shall be:
"""
fn main() {
    let x = (1 + 2);
}
"""
```

#### Scenario 4: Already valid file — no changes

```gherkin
Given a Rust source file with content:
"""
fn main() {}
"""
When I run `patch-ts balance --file <file> --apply`
Then the file content shall remain unchanged.
And the JSON output shall indicate success with zero actions.
```

#### Scenario 5: Exceeds max cost — no modification

```gherkin
Given a severely broken Rust file with 20 randomly inserted extra braces.
When I run `patch-ts balance --file <file> --max-cost 5`
Then the command shall exit with non‑zero code.
And the file shall remain unchanged.
And stderr shall contain "Unable to find repair within max cost 5".
```

### Feature: Integration with `patch` command auto‑repair

```gherkin
Scenario: Patching introduces syntax error; auto‑repair fixes it
  Given a valid Rust file with a line "let x = 1;"
  When I run `patch-ts patch --file <file> --line 1 --old "let x = 1;" --new "let x = (1 + 2"`
  Then the patch shall be applied.
  And because the new line has a missing ')', the auto‑repair shall insert it.
  And the final file shall contain "let x = (1 + 2);"
```

### Decision Table: Tie‑Breaking Determinism

| Condition | Expectation |
|-----------|-------------|
| Two valid repairs of equal cost | The output must be identical across runs. |
| Repair A: delete ')' then insert '}' | Cost = 2 |
| Repair B: insert '}' then delete ')' | Cost = 2 |
| Tie‑breaker rule (lexicographic by edit type/offset) | Determines unique order. |

## 2. Test Strategy & Plan

### 2.1 Overall Approach

- **Unit tests**: Validate `minimum_cost_repair` function in isolation with mock `Language`.
- **Integration tests**: Run actual `patch-ts` binary against fixture files for all 16 languages.
- **Property‑based tests**: Use `proptest` to generate random invalid code and assert repair validity.
- **Performance benchmarks**: Use `criterion` to measure search time on representative files.

### 2.2 Test Pyramid

| Level | Tools | Scope |
|-------|-------|-------|
| Unit | `cargo test` | Search algorithm, action generation, memoization. |
| Integration | `assert_cmd` | CLI behavior, file I/O, JSON output. |
| Property | `proptest` | Random generation of delimiter errors. |
| Benchmarks | `criterion` | Performance of BFS search. |

### 2.3 Environments

- Rust toolchain stable.
- No external services; runs locally.

## 3. Test Case Specifications (Extract)

### TC‑FUNC‑001: BFS Finds Minimum Cost for Simple Missing Brace

| Field | Value |
|-------|-------|
| ID | TC‑FUNC‑001 |
| Related REQ | REQ‑FUNC‑001, REQ‑FUNC‑004 |
| Preconditions | Mock `Language` returning invalid for "fn main() {" and valid for "fn main() {}". |
| Steps | 1. Call `minimum_cost_repair` with source "fn main() {", errors=[Missing('}', span_end)], max_cost=1. |
| Expected | Returns `Some(patched)` with cost 1 and actions=[Insert('}')]. |

### TC‑REL‑001: Determinism Across Runs

| Field | Value |
|-------|-------|
| ID | TC‑REL‑001 |
| Related REQ | REQ‑FUNC‑005, NFR‑REL‑002 |
| Preconditions | Source with multiple possible repairs of equal cost (e.g., "fn foo() { let x = (1 + 2)); }"). |
| Steps | 1. Run `balance` twice on same file. 2. Compare output files byte‑for‑byte. |
| Expected | Outputs are identical. |

### TC‑PERF‑001: 500‑line File with 3 Errors < 500ms

| Field | Value |
|-------|-------|
| ID | TC‑PERF‑001 |
| Related REQ | NFR‑PERF‑001 |
| Steps | 1. Generate a 500‑line Rust file (commented lines with fn main). Insert 3 extra delimiters. 2. Time the call to `minimum_cost_repair`. |
| Expected | Elapsed time < 500 ms (median of 10 runs). |

## 4. NFR Verification Plans

### Performance

- **Load profile**: 100 open‑source Rust files of varying sizes, each injected with 1–5 errors.
- **Tool**: `criterion` benchmark groups.
- **Thresholds**: p95 < 500 ms, max memory < 500 MB.

### Reliability

- **Property‑based test**: Generate random valid Rust snippets using a subset grammar. Insert <N> random extra delimiters and delete <M> random delimiters. Run repair and assert `is_valid` true.
- **Fuzzing**: Use `cargo fuzz` on the repair entry point.

### Security

- Not applicable; no new network I/O or privilege escalation.

### Accessibility

- Not applicable; CLI tool.

## 5. Requirements Traceability Matrix (RTM)

| Business Goal | Stakeholder Need | SRS Requirement | BDD Scenario / Test Case | Status |
|---------------|------------------|-----------------|--------------------------|--------|
| G‑1 | SN‑1 | REQ‑FUNC‑001,004 | Scenarios 1,2,3 | (to be executed) |
| G‑1 | SN‑3 | REQ‑FUNC‑011 | Integration tests (re‑enabled) | |
| G‑2 | SN‑2 | REQ‑FUNC‑013 | Scenario 4 | |
| G‑3 | SN‑1,2 | REQ‑FUNC‑006 | Scenario 5 | |
| N/A | N/A | NFR‑REL‑001 | Property‑based test | |
| N/A | N/A | NFR‑PERF‑001 | TC‑PERF‑001 | |

## 6. Living Documentation Strategy

- Feature files (`.feature`) will be stored in `tests/features/balance/`.
- `cucumber` crate will execute them as part of CI.
- Test results will be visible in CI logs; no separate living‑doc portal is planned for v1.1 (low overhead).

## 7. Test Deliverables

- Updated test files in `tests/*_tests.rs` with previously disabled tests re‑enabled.
- New file `tests/minimum_cost_search_tests.rs` for unit tests.
- New `proptest` in `tests/proptest_repair.rs`.
- Benchmark group expanded in `benches/patch_benchmark.rs`.
```
