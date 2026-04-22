I'll create the full specification suite for **patch-ts v0.3.0** following the layered documentation structure defined in the `spec-writer` skill. The documents reflect the current v0.2.0 baseline and the planned enhancements for v0.3.0 as outlined in the roadmap.

---

# patch-ts v0.3.0 — Specification Documents

## Document Set Overview

| Document | Purpose |
|----------|---------|
| [Vision & Strategic Alignment](#patch-ts-vision--strategic-alignment) | Why patch-ts exists, long‑term direction, and success criteria. |
| [Business & Stakeholder Requirements (BRS)](#patch-ts-business--stakeholder-requirements-specification-brs) | Business goals, stakeholders, and high‑level needs in business language. |
| [Software Requirements Specification (SRS)](#patch-ts-software-requirements-specification-srs) | Functional and non‑functional requirements for the v0.3.0 release. |
| [Architecture & Design Specification](#patch-ts-architecture--design-specification) | Architectural decisions, C4 views, ADRs, and design trade‑offs. |
| [Behavioral Specification & Test Verification Plan](#patch-ts-behavioral-specification--test-verification-plan) | Acceptance criteria, test strategy, and traceability for v0.3.0 features. |

---

# patch-ts Vision & Strategic Alignment

| Field | Value |
|-------|-------|
| Project | patch-ts |
| Document | Vision & Strategic Alignment |
| Version | 1.0 |
| Date | 2026-04-22 |
| Author | patch-ts team, assisted by spec-writer |
| Status | Approved |

## 1. Vision Statement

> *patch-ts exists to make automated code patching **safe, reliable, and resilient** for AI agents and developers working with Rust codebases. We aim to eliminate brittle line‑number dependencies and syntax‑error risks so that patches succeed the first time, every time.*

## 2. Elevator Pitch (Moore Template)

> For **AI coding agents and Rust developers** who are frustrated by patches that fail due to line drift or introduce syntax errors, **patch-ts** is a **tree‑sitter‑aware CLI tool** that provides **fuzzy matching, auto‑repair, and marker‑based targeting**. Unlike simple `sed` or line‑based patching tools, our product **validates AST integrity and automatically corrects simple mistakes**, dramatically increasing patch success rates.

## 3. Problem Statement & Business Context

**Problem:** AI‑generated code patches (from LLMs like Claude, GPT, or local agents) frequently fail when applied to evolving codebases because:
- Line numbers drift as files change.
- Patches may introduce unbalanced braces or invalid syntax.
- Agents lack a reliable way to target specific code structures without line numbers.

**Why now:** AI‑assisted development is accelerating, and tools that bridge the gap between generated patches and real‑world codebases are essential to realize productivity gains.

**Business drivers:**
- Reduce manual intervention when applying AI‑generated fixes.
- Increase trust in automated refactoring workflows.
- Enable scalable, headless patching in CI/CD and agentic pipelines.

## 4. Target Users / Customers

| Segment | Description |
|---------|-------------|
| **AI coding agents** | LLM‑powered tools that generate code changes. Need a reliable, machine‑readable patching interface. |
| **Rust developers** | Individual developers applying patches from PR reviews, refactoring scripts, or AI assistants. |
| **CI/CD pipelines** | Automated systems applying bulk changes (e.g., dependency upgrades, lint fixes). |
| **Tool builders** | Developers creating Rust tooling that needs programmatic AST‑aware patching. |

**Explicitly NOT targeting (v0.3.0):**
- Languages other than Rust (deferred to v0.4.0).
- Interactive UI or IDE plugin (future consideration).

## 5. User Needs & Value Proposition

| Need | patch-ts Value |
|------|----------------|
| "I need a patch to apply even if line numbers shifted." | Fuzzy matching finds the right location within a configurable radius. |
| "I don't want a patch to break my code's syntax." | AST validation + auto‑repair fixes simple errors before writing. |
| "I need to target a specific function, not a line number." | Marker comments (`// PATCH-ME: id`) anchor patches to AST nodes. |
| "I need to fix unbalanced braces automatically." | The `balance` command removes or inserts delimiters to restore validity. |
| "I need machine‑readable output for my agent." | JSON diagnostics include suggestions, scores, and line candidates. |

**Differentiator:** Unlike generic patching tools, patch-ts understands Rust syntax and can *repair* the damage a patch might cause.

## 6. Desired Outcomes & Success Metrics

### Business Outcomes (v0.3.0)

| ID | Outcome | Key Result / Target |
|----|---------|---------------------|
| G‑1 | Increase patch success rate | 90% of fuzzy‑matched patches apply without manual intervention in benchmark suite. |
| G‑2 | Expand auto‑repair coverage | Auto‑repair succeeds for 80% of single‑delimiter mismatch cases (including `(`, `[`, `{`). |
| G‑3 | Maintain performance | Patch application on 10k‑line files remains under 100 ms (benchmarked). |

### Product Outcomes (v0.3.0)

| ID | Outcome | Metric |
|----|---------|--------|
| P‑1 | Users can fix unbalanced delimiters of any type | `balance` command handles `(`, `[`, `{` with both removal and insertion. |
| P‑2 | Users can scope balance to a function | `--function` flag restricts repairs to a named function body. |
| P‑3 | Multi‑error repairs succeed in one run | `balance` iteratively fixes multiple issues until file is valid or no progress. |

## 7. Strategic Constraints

| Constraint | Description |
|------------|-------------|
| **Language** | Rust only (tree‑sitter‑rust). |
| **Backward compatibility** | v0.3.0 CLI must remain compatible with v0.2.0 flags and behaviors. |
| **Performance** | Repair operations must be sub‑100ms for typical files. |
| **Dependencies** | tree‑sitter‑rust, clap, miette, line‑index; no new heavy dependencies. |

## 8. Goals and Non‑Goals (v0.3.0)

### Goals

- Extend `balance` to detect and fix extra/missing `(`, `[`, `{`.
- Implement insertion of missing delimiters (currently only removal).
- Support multiple error fixes in a single `balance` run.
- Add `--function` flag to scope `balance` to a specific function body.

### Non‑Goals (explicitly excluded from v0.3.0)

- Full multi‑language support (TypeScript, Python, etc.) — deferred to v0.4.0.
- Semantic error repair (e.g., type mismatches, borrow checker fixes).
- Interactive mode or UI.
- Automatic detection of unbalanced delimiters without explicit `balance` command.
- Fixing malformed code that requires more than delimiter insertion/removal (e.g., missing semicolons).

## 9. Operational Concept & High‑Level Scenarios

### Concept of Operations

Developers or AI agents invoke `patch-ts` via CLI. The tool reads the target file, applies the requested operation (patch, balance, explain), validates the result using tree‑sitter, and writes back atomically with a backup. JSON output is available for machine consumption.

### High‑Level Scenarios (v0.3.0)

1. **Balance extra parenthesis**  
   User runs `patch-ts balance --file src/lib.rs --apply`. The tool finds an extra `)` and removes it, making the file valid.

2. **Balance missing brace**  
   File is missing a closing `}`. `patch-ts balance` inserts it at the correct location.

3. **Balance scoped to function**  
   `patch-ts balance --file src/lib.rs --function process_data --apply` repairs delimiters only within the `process_data` function body.

4. **Multi‑error repair**  
   File has both an extra `}` and a missing `)`. The tool removes the extra brace, then inserts the missing parenthesis in one run.

5. **Fuzzy patch with auto‑repair**  
   Agent applies a patch that removes a closing brace. patch-ts auto‑repairs by re‑inserting the brace.

## 10. Stakeholders, Sponsorship & Governance

| Role | Name / Org | Responsibility |
|------|------------|----------------|
| **Executive Sponsor** | (Project maintainer) | Approves strategic direction and resource allocation. |
| **Product Owner** | (Project maintainer) | Prioritizes features, manages roadmap. |
| **Engineering Lead** | (Core contributor) | Oversees technical implementation and architecture. |
| **Contributors** | Open‑source community | Submit PRs, report issues. |

**Governance:** The vision document is reviewed quarterly. Changes to goals or non‑goals require maintainer consensus.

## 11. Risks, Assumptions & Open Questions

### Assumptions

- tree‑sitter‑rust provides sufficient AST information to locate and fix delimiters accurately.
- The `balance` heuristics (extra/missing delimiter detection) remain reliable for common Rust patterns.
- Users understand that `balance` is a best‑effort tool and may not fix all syntax errors.

### Risks

| Risk | Likelihood | Impact | Mitigation |
|------|------------|--------|------------|
| Insertion heuristics place delimiter incorrectly | Medium | High | Use tree‑sitter error recovery to locate the most probable insertion point; provide `--dry-run` preview. |
| Multi‑error repair loops indefinitely | Low | Medium | Limit iterations to a reasonable maximum (e.g., 10). |
| Performance degrades with large files | Low | Medium | Maintain benchmarks; optimize AST traversal. |

### Open Questions

- Should `--function` accept a regex or exact name? (Decision: exact name initially, can extend later.)
- How to handle nested delimiters when inserting? (Use parser's error node position.)

## 12. Traceability & Alignment Notes

| Vision ID | Maps to BRS Section | Maps to SRS Requirements |
|-----------|---------------------|-------------------------|
| G‑1, G‑2, G‑3 | Business Goals (BR‑001..003) | NFR‑PERF‑001, FR‑BAL‑001..006 |
| P‑1, P‑2, P‑3 | Stakeholder Needs (SN‑001..003) | FR‑BAL‑001..006 |

---

# patch-ts Business & Stakeholder Requirements Specification (BRS)

| Field | Value |
|-------|-------|
| Project | patch-ts |
| Document | Business & Stakeholder Requirements Specification |
| Version | 1.0 |
| Date | 2026-04-22 |
| Author | patch-ts team, assisted by spec-writer |
| Status | Approved |
| References | Vision v1.0 |

## 1. Business Context

### 1.1 Purpose

This BRS defines the business‑level requirements for patch-ts v0.3.0, focusing on expanding the repair engine (`balance` command) to handle all delimiter types, insertion of missing delimiters, multi‑error repair, and function‑scoped balancing.

### 1.2 Business Problem / Opportunity

AI agents and developers frequently encounter unbalanced delimiters (`(`, `[`, `{`) after applying patches or during manual editing. The current `balance` command only removes extra `}` and cannot insert missing delimiters or handle parentheses/brackets. This limits the tool's utility for automated repair.

### 1.3 Scope Boundaries

**In scope:**
- Extend delimiter detection to `(`, `[`, `{`.
- Implement insertion logic for missing delimiters.
- Support iterative repair of multiple delimiter errors.
- Add `--function` flag to limit balancing to a named function.

**Out of scope:**
- Repair of semantic errors (type mismatches, borrow checker).
- Languages other than Rust.
- Automatic invocation of `balance` during `patch` (already exists as auto‑repair; not expanded).

## 2. Business Goals, Objectives & Success Metrics

| ID | Business Goal | Success Metric (Fit Criterion) |
|----|---------------|-------------------------------|
| BR‑001 | Improve patch‑and‑repair success rate | In benchmark suite of 100 realistic patches, auto‑repair success rate increases from 60% to ≥85%. |
| BR‑002 | Expand `balance` command utility | `balance` can fix ≥90% of single‑delimiter mismatch cases in a curated corpus of common Rust errors. |
| BR‑003 | Maintain performance | End‑to‑end `balance` execution on 10k‑line files remains under 150 ms (p95). |

## 3. Business Model & Processes

patch-ts is an open‑source CLI tool. Its "business model" is community adoption and integration into AI agent workflows. Key processes:
- **Patch application:** Agent provides expected/old content, patch-ts applies with fuzzy matching and validation.
- **Repair:** User invokes `balance` to fix delimiter errors; in v0.3.0, this becomes a more comprehensive repair tool.

## 4. Business Rules & Policies

| ID | Rule | Source |
|----|------|--------|
| BR‑R1 | All modifications must be atomic (temp file + rename) and create a `.bak` backup unless `--no-backup` is specified. | Project policy |
| BR‑R2 | AST validation must be performed after every patch unless `--force` is used. | Safety requirement |
| BR‑R3 | The tool must exit with non‑zero code on any failure (patch mismatch, syntax error, etc.). | Scripting requirement |

## 5. Stakeholders & User Classes

| Stakeholder / User Class | Description | Primary Goals |
|--------------------------|-------------|---------------|
| AI Agent (primary) | LLM‑based coding assistant | Apply patches reliably; receive structured feedback. |
| Rust Developer (primary) | Individual programmer | Fix syntax errors quickly; apply refactoring patches. |
| CI/CD System (secondary) | Automated pipeline | Bulk‑apply fixes without manual oversight. |
| Tool Builder (secondary) | Developer integrating patch-ts | Programmatic, stable interface. |

**Key Persona (Developer):**  
Alex, a Rust backend engineer, frequently applies patches from PR reviews. They use `patch-ts balance` to quickly fix accidental unbalanced braces or parentheses before committing.

## 6. Glossary / Ubiquitous Language

| Term | Definition |
|------|------------|
| **Delimiter** | `(`, `)`, `[`, `]`, `{`, `}`. |
| **Balance** | The act of making delimiters properly paired and nested. |
| **Extra delimiter** | A closing delimiter with no matching open delimiter in the correct scope. |
| **Missing delimiter** | An expected closing delimiter that is absent. |
| **Fuzzy matching** | Locating a target line/block using similarity metrics instead of exact line numbers. |
| **Auto‑repair** | Automatic invocation of `quick_balance` when a patch introduces a syntax error. |
| **Marker** | `// PATCH-ME: <id>` comment used to anchor patches to AST nodes. |

## 7. Conceptual Domain Model

**Core entities:**
- `SourceFile`: a Rust source file with text content and line index.
- `ParseTree`: tree‑sitter AST of the file.
- `DelimiterError`: a detected imbalance (extra or missing delimiter).
- `RepairAction`: removal or insertion of a delimiter at a specific span.

**Relationships:**
- A `SourceFile` has one `ParseTree`.
- A `ParseTree` may contain zero or more `DelimiterError`s.
- Each `DelimiterError` suggests one or more `RepairAction`s.

## 8. Stakeholder Needs & User Requirements

| ID | Stakeholder Need | User Class |
|----|------------------|------------|
| SN‑001 | As a developer, I want to fix unbalanced parentheses and brackets, not just braces. | Developer |
| SN‑002 | As a developer, I want `balance` to add missing delimiters, not just remove extras. | Developer |
| SN‑003 | As a developer, I want `balance` to fix multiple errors in one run. | Developer |
| SN‑004 | As a developer, I want to limit balancing to a specific function to avoid unintended changes elsewhere. | Developer |
| SN‑005 | As an AI agent, I want JSON output from `balance` indicating what was fixed. | AI Agent |

## 9. System‑in‑Context & Operational Concept

patch-ts operates as a CLI invoked on a single file. The `balance` command:
1. Parses the file.
2. Identifies delimiter errors (extra/missing).
3. Applies repair actions (removal/insertion) iteratively until the file is valid or no progress.
4. Writes the repaired file (or prints diff in dry‑run).
5. Outputs human‑readable summary or JSON.

**Operational scenario (scoped balance):**
```
$ patch-ts balance --file src/lib.rs --function handle_request --apply
Fixed missing ')' in function `handle_request`.
File updated; backup at src/lib.rs.bak.
```

## 10. Stakeholder‑Level Constraints & Quality Expectations

| ID | Constraint / Quality Expectation |
|----|----------------------------------|
| C‑001 | The tool must not alter semantically correct code (false positive repairs). |
| C‑002 | Repair must be deterministic and idempotent. |
| C‑003 | Performance: balance on 10k‑line file <150ms. |
| C‑004 | JSON output must include file path, line, column, and action taken. |

## 11. Risks, Assumptions & Open Issues

### Assumptions
- Tree‑sitter's error recovery provides reliable information about missing/extra delimiters.
- Function name scoping can be implemented via a simple tree‑sitter query.

### Risks
| Risk | Mitigation |
|------|------------|
| Insertion logic places delimiter incorrectly | Use error node span as primary hint; allow user preview with `--dry-run`. |
| Scoped balance misses errors outside function | Document limitation; users can run full balance separately. |

### Open Issues
- Should `--function` support nested functions? (Yes, if unique; otherwise error.)

## 12. Traceability Mapping to Vision

| Vision Goal | BRS Goal | Stakeholder Need |
|-------------|----------|------------------|
| G‑1 | BR‑001 | SN‑001..004 |
| G‑2 | BR‑002 | SN‑001, SN‑002, SN‑003 |
| G‑3 | BR‑003 | C‑003 |
| P‑1 | BR‑002 | SN‑001 |
| P‑2 | – | SN‑004 |
| P‑3 | – | SN‑003 |

---

# patch-ts Software Requirements Specification (SRS)

| Field | Value |
|-------|-------|
| Project | patch-ts |
| Document | Software Requirements Specification |
| Version | 1.0 (v0.3.0) |
| Date | 2026-04-22 |
| Author | patch-ts team, assisted by spec-writer |
| Status | Approved |
| References | BRS v1.0, Vision v1.0 |

## 1. Introduction & Scope

This SRS defines the functional and non‑functional requirements for patch-ts v0.3.0, focusing on the enhanced `balance` command. The document serves as the authoritative specification for implementation and testing.

### 1.1 Scope

- Extend `balance` to detect and repair extra/missing `(`, `[`, `{` delimiters.
- Implement insertion of missing delimiters.
- Support iterative repair of multiple delimiter errors.
- Add `--function <NAME>` flag to scope repairs to a named function body.
- Maintain backward compatibility with v0.2.0 CLI.

### 1.2 Out of Scope

- Repair of other syntax errors (e.g., missing semicolons, type errors).
- Multi‑language support.
- Automatic scoping without explicit flag.

## 2. System Context & Overview

**Context Diagram (C1):**
- **User (Developer/Agent)** invokes `patch-ts balance` via CLI.
- **File System** provides source file and receives modified file + backup.
- **tree‑sitter‑rust** provides parsing and AST.

**High‑level capabilities (v0.3.0):**
- Delimiter‑agnostic error detection.
- Bidirectional repair (remove extra, insert missing).
- Iterative repair loop.
- Function‑scoped repair.

## 3. Functional Capabilities & Behavior

Requirements are organized by feature. Each requirement includes a unique ID, priority (MoSCoW), and acceptance criteria.

### Feature: Delimiter‑Agnostic Detection

| ID | Requirement (EARS pattern) | Priority | Acceptance Criteria |
|----|----------------------------|----------|---------------------|
| FR‑BAL‑001 | **Event‑driven:** When `balance` is invoked on a file with an extra `)`, `]`, or `}`, the system shall identify the extra delimiter and its span. | Must | Given a file with an extra closing delimiter, the tool outputs the correct line and column of the extra delimiter. |
| FR‑BAL‑002 | **Event‑driven:** When `balance` is invoked on a file with a missing `)`, `]`, or `}`, the system shall identify the missing delimiter and the suggested insertion point. | Must | Given a file missing a closing delimiter, the tool outputs the suggested insertion location. |

### Feature: Insertion of Missing Delimiters

| ID | Requirement | Priority | Acceptance Criteria |
|----|-------------|----------|---------------------|
| FR‑BAL‑003 | **If** a missing delimiter is detected, **then** the system shall insert the appropriate delimiter at the calculated position when `--apply` is used. | Must | File is modified with the missing delimiter inserted; backup created. |
| FR‑BAL‑004 | **While** in dry‑run mode (`--apply` not specified), the system shall describe the insertion that would be made without modifying the file. | Should | Output includes "Would insert X at line Y". |

### Feature: Multi‑Error Repair

| ID | Requirement | Priority | Acceptance Criteria |
|----|-------------|----------|---------------------|
| FR‑BAL‑005 | **While** the file contains syntax errors and progress is being made, the system shall iteratively apply repairs (removals/insertions) until the file is valid or no further repairs can be identified. | Must | A file with two delimiter errors is repaired in a single run. |
| FR‑BAL‑006 | **If** the repair loop exceeds a maximum iteration limit (e.g., 10), **then** the system shall abort and report failure. | Must | Infinite loop avoided; error message indicates limit reached. |

### Feature: Function Scoping

| ID | Requirement | Priority | Acceptance Criteria |
|----|-------------|----------|---------------------|
| FR‑BAL‑007 | **Where** the `--function <NAME>` flag is provided, the system shall restrict delimiter detection and repair to the body of the named function. | Must | Errors outside the function are ignored; only function body is repaired. |
| FR‑BAL‑008 | **If** the function name is not unique (multiple functions with same name in file), **then** the system shall report an ambiguity error and exit. | Should | Error message lists locations of ambiguous functions. |
| FR‑BAL‑009 | **If** the function name is not found, **then** the system shall report that the function does not exist and exit. | Must | Clear error message. |

### Feature: JSON Output for Balance

| ID | Requirement | Priority | Acceptance Criteria |
|----|-------------|----------|---------------------|
| FR‑BAL‑010 | **When** `--json` is used with `balance`, the system shall output a JSON object containing `success`, `actions` (list of repairs), and `error` (if any). | Should | JSON includes file, line, column, action type (remove/insert), and delimiter character. |

## 4. Quality & Non‑Functional Requirements

Organized per ISO 25010:2023.

| ID | Category | Requirement | Fit Criterion |
|----|----------|-------------|---------------|
| NFR‑PERF‑001 | Performance efficiency | `balance` on a 10k‑line Rust file shall complete within 150 ms (p95). | Benchmark suite measures execution time. |
| NFR‑REL‑001 | Reliability | Repairs shall be deterministic: same input yields same output. | Test with fixed corpus; output must match exactly. |
| NFR‑SEC‑001 | Security | No new security vulnerabilities introduced; tool only reads/writes specified file. | Pass `cargo audit` and manual review. |
| NFR‑MAINT‑001 | Maintainability | New delimiter logic shall be implemented via extensions to the existing `Language` trait and `repair` module without breaking existing tests. | All existing tests pass; new code has ≥80% coverage. |
| NFR‑COMPAT‑001 | Compatibility | v0.3.0 CLI must accept all v0.2.0 flags and produce equivalent behavior (except enhancements). | Regression test suite passes. |

## 5. External Interfaces & Data Contracts

### 5.1 CLI Interface

**Command:** `patch-ts balance --file <PATH> [--function <NAME>] [--apply] [--no-backup] [--json]`

| Argument | Description |
|----------|-------------|
| `--file` | Path to Rust source file (required). |
| `--function` | Limit repairs to named function (optional). |
| `--apply` | Actually modify file; default is dry‑run. |
| `--no-backup` | Skip creating `.bak` file. |
| `--json` | Output JSON instead of human‑readable text. |

**Exit codes:** 0 success, 1 error.

### 5.2 JSON Output Schema (Balance)

```json
{
  "success": true,
  "actions": [
    {
      "type": "remove" | "insert",
      "delimiter": "}" | ")" | "]",
      "line": 42,
      "column": 1,
      "message": "Removed extra '}'"
    }
  ],
  "error": null
}
```

## 6. Constraints, Assumptions & Dependencies

| Type | Description |
|------|-------------|
| Constraint | Must use tree‑sitter‑rust 0.24 (compatible with tree‑sitter 0.25). |
| Assumption | tree‑sitter error recovery reliably identifies missing/extra delimiters. |
| Dependency | `line-index` crate for byte ↔ line/col mapping. |

## 7. TBD Log

| ID | Item | Owner | Due |
|----|------|-------|-----|
| TBD‑001 | Determine exact iteration limit for multi‑error repair. | Engineering | Before implementation. |
| TBD‑002 | Define ambiguity resolution for function name (e.g., line number hint). | Product | Before beta. |

## 8. Requirements Attributes & Traceability Model

**ID Scheme:**
- Functional: `FR‑BAL‑###`
- NFR: `NFR‑CAT‑###`
- Trace to BRS: `SN‑###`, `C‑###`

**Traceability Matrix (excerpt):**

| SRS ID | BRS ID | Verification Method |
|--------|--------|---------------------|
| FR‑BAL‑001 | SN‑001 | Test (automated) |
| FR‑BAL‑002 | SN‑002 | Test |
| FR‑BAL‑003 | SN‑002 | Test + Demo |
| FR‑BAL‑005 | SN‑003 | Test |
| FR‑BAL‑007 | SN‑004 | Test + Inspection |
| NFR‑PERF‑001 | BR‑003 | Analysis + Test (bench) |

---

# patch-ts Architecture & Design Specification

| Field | Value |
|-------|-------|
| Project | patch-ts |
| Document | Architecture & Design Specification |
| Version | 1.0 (v0.3.0) |
| Date | 2026-04-22 |
| Author | patch-ts team, assisted by spec-writer |
| Status | Draft |
| References | SRS v1.0, BRS v1.0 |

## 1. Context & Scope

This document describes the architectural design for the v0.3.0 enhancements to patch-ts, focusing on the expanded `balance` command. The design must integrate with the existing codebase while adding new capabilities for delimiter‑agnostic repair, insertion, multi‑error handling, and function scoping.

## 2. Goals & Non‑Goals

### Goals
- Extend `Language` trait with methods to find *all* delimiter errors (extra/missing).
- Implement insertion logic in `balance_file`.
- Add iterative repair loop.
- Implement function scoping via tree‑sitter query.

### Non‑Goals
- Refactor the entire codebase; changes should be localized to `repair.rs` and `ast.rs`.
- Implement semantic analysis beyond delimiter matching.

## 3. Architecturally Significant Requirements (ASRs)

Extracted from SRS NFRs and key functional requirements:

| ASR ID | Description | Source |
|--------|-------------|--------|
| ASR‑001 | Repair must be deterministic and idempotent. | NFR‑REL‑001 |
| ASR‑002 | Performance: balance on 10k‑line file <150ms. | NFR‑PERF‑001 |
| ASR‑003 | Backward compatibility with v0.2.0 CLI. | NFR‑COMPAT‑001 |
| ASR‑004 | Insertion logic must place missing delimiter correctly. | FR‑BAL‑002, FR‑BAL‑003 |
| ASR‑005 | Scoping must restrict repairs to a named function body. | FR‑BAL‑007 |

## 4. The Design

### 4.1 System Overview (C4 Level 2)

```
[User/Agent] → (CLI) → [patch-ts Binary]
                           ├── cli.rs (argument parsing)
                           ├── repair.rs (balance logic)
                           ├── ast.rs (Language trait, RustLanguage)
                           └── file.rs (atomic write)
                                     ↓
                              [tree-sitter-rust] → [AST]
```

### 4.2 Key Design Changes

**4.2.1 Delimiter Detection Enhancements**

- Extend `Language` trait:
  ```rust
  fn find_delimiter_errors(&self, result: &ParseResult) -> Vec<DelimiterError>;
  ```
- `DelimiterError` enum:
  ```rust
  enum DelimiterError {
      Extra { span: Span, delimiter: char },
      Missing { expected: char, insert_at: Span },
  }
  ```
- Implement for `RustLanguage` using tree‑sitter's error nodes and a stack‑based delimiter matcher.

**4.2.2 Insertion Logic**

- When a missing delimiter is detected, the insertion point is derived from the parser's error node or the span where the imbalance is detected.
- The `balance_file` function will handle both removal and insertion actions.

**4.2.3 Iterative Repair Loop**

```rust
let mut content = original_content;
let mut max_iterations = 10;
while max_iterations > 0 {
    let parse_result = language.parse(&content);
    if language.is_valid(&parse_result) { break; }
    let errors = language.find_delimiter_errors(&parse_result);
    if errors.is_empty() { break; }
    apply_first_repair(&mut content, &errors[0])?;
    max_iterations -= 1;
}
```

**4.2.4 Function Scoping**

- Use tree‑sitter query to find the function node with the given name.
- Restrict `find_delimiter_errors` to the byte range of that function's body.

### 4.3 Data Model

No new persistent data; all operations in memory.

### 4.4 Security Architecture

No changes; tool remains a local file manipulation utility.

## 5. Architecture Decision Records (ADRs)

### ADR‑001: Use stack‑based delimiter matching for detection

**Context:** Need to detect extra/missing delimiters reliably.

**Decision:** Implement a stack that tracks open delimiters while traversing the AST. Compare against expected closings; mismatches indicate errors.

**Alternatives:** Rely solely on tree‑sitter error nodes. Rejected because error nodes alone don't always pinpoint the exact missing delimiter location.

**Consequences:** Slightly more complex implementation but higher accuracy.

### ADR‑002: Iterative repair with fixed limit

**Context:** Multiple delimiter errors may exist; repairing one may fix others.

**Decision:** Loop applying one repair at a time, re‑parsing after each, up to 10 iterations.

**Alternatives:** Attempt to compute all repairs in one pass. Rejected because repairs can shift spans.

**Consequences:** Simple, safe, and sufficient for realistic cases.

### ADR‑003: Function scoping via tree‑sitter query

**Context:** Need to restrict repairs to a specific function.

**Decision:** Use `tree-sitter` query `(function_item name: (identifier) @name)` to locate function node; then use its byte range.

**Alternatives:** Regex‑based scoping. Rejected as brittle.

**Consequences:** Accurate, leverages existing parser.

## 6. API & Interface Contracts

No new external APIs. CLI interface updated per SRS section 5.1.

## 7. Cross‑cutting Concerns

| Concern | Approach |
|---------|----------|
| **Observability** | Log repairs at `info` level; JSON output for machine consumption. |
| **Error Handling** | Use `anyhow::Result` with contextual messages; exit non‑zero on failure. |
| **Testing** | Unit tests for detection logic; integration tests for full `balance` command. |

## 8. Alternatives Considered

| Alternative | Why Rejected |
|-------------|--------------|
| Full syntax tree rewrite | Overkill; simple delimiter repair is sufficient. |
| Use `rust-analyzer` for repair | Heavy dependency; tree‑sitter is lighter and sufficient. |
| Auto‑detect function scope | Ambiguous; explicit flag is clearer. |

## 9. Traceability

| ASR | ADR | C4 Component |
|-----|-----|--------------|
| ASR‑001 | ADR‑002 | repair.rs |
| ASR‑002 | – | repair.rs, ast.rs |
| ASR‑004 | ADR‑001 | ast.rs, repair.rs |
| ASR‑005 | ADR‑003 | repair.rs |

---

# patch-ts Behavioral Specification & Test Verification Plan

| Field | Value |
|-------|-------|
| Project | patch-ts |
| Document | Behavioral Specification & Test Verification Plan |
| Version | 1.0 (v0.3.0) |
| Date | 2026-04-22 |
| Author | patch-ts team, assisted by spec-writer |
| Status | Draft |
| References | SRS v1.0, Architecture v1.0 |

## 1. Behavioral Specifications (Specification by Example)

### Feature: Balance command enhancements

#### Scenario: Remove extra parenthesis

```gherkin
Feature: Balance command
  As a developer
  I want to fix unbalanced delimiters
  So that my Rust code compiles

  Scenario: Remove extra closing parenthesis
    Given a file "src/lib.rs" with content:
      """
      fn main() {
          let x = (1 + 2));
      }
      """
    When I run `patch-ts balance --file src/lib.rs --apply`
    Then the file content becomes:
      """
      fn main() {
          let x = (1 + 2);
      }
      """
    And a backup file "src/lib.rs.bak" is created.
```

#### Scenario: Insert missing closing brace

```gherkin
  Scenario: Insert missing closing brace
    Given a file with content:
      """
      fn main() {
          println!("Hello");
      """
    When I run `patch-ts balance --file src/lib.rs --apply`
    Then the file content becomes:
      """
      fn main() {
          println!("Hello");
      }
      """
```

#### Scenario: Multi‑error repair (extra brace and missing parenthesis)

```gherkin
  Scenario: Fix multiple delimiter errors in one run
    Given a file with content:
      """
      fn main() {
          let x = (1 + 2;
      }
      }
      """
    When I run `patch-ts balance --file src/lib.rs --apply`
    Then the file content becomes:
      """
      fn main() {
          let x = (1 + 2);
      }
      """
```

#### Scenario: Function‑scoped repair

```gherkin
  Scenario: Balance only within a specific function
    Given a file with content:
      """
      fn foo() {
          let x = (1 + 2;
      }
      fn bar() {
          let y = (3 + 4));
      }
      """
    When I run `patch-ts balance --file src/lib.rs --function foo --apply`
    Then only `foo` is repaired:
      """
      fn foo() {
          let x = (1 + 2);
      }
      fn bar() {
          let y = (3 + 4));
      }
      """
```

#### Scenario: Dry‑run preview

```gherkin
  Scenario: Preview changes without applying
    Given a file with an extra '}'
    When I run `patch-ts balance --file src/lib.rs` (no --apply)
    Then the output contains "Would remove extra '}' at line X"
    And the file is unchanged.
```

#### Scenario: JSON output

```gherkin
  Scenario: Machine‑readable output
    Given a file with an extra '}'
    When I run `patch-ts balance --file src/lib.rs --apply --json`
    Then stdout is valid JSON with:
      | field         | value                     |
      | success       | true                      |
      | actions[0].type | "remove"                |
      | actions[0].delimiter | "}"                   |
```

### Decision Table: Delimiter Detection

| Condition | Extra `}` | Missing `)` | Both | Valid |
|-----------|-----------|-------------|------|-------|
| Action: remove extra | X |   | X |   |
| Action: insert missing |   | X | X |   |
| Result: file valid | X | X | X | X |

### State Transition: Repair Loop

```
[Start] → Parse → {Valid?} → yes → [Done]
                     ↓ no
                Find errors
                     ↓
            {Errors found?} → no → [Fail: cannot fix]
                     ↓ yes
               Apply one repair
                     ↓
            [Loop with iteration limit]
```

## 2. Test Strategy & Plan

### 2.1 Test Pyramid

| Level | Scope | Tools | Ownership |
|-------|-------|-------|-----------|
| Unit | Delimiter detection logic, insertion point calculation | Rust `#[test]` | Developers |
| Integration | Full `balance` command end‑to‑end | `assert_cmd`, tempfile | Developers |
| Performance | Benchmark large file repairs | Criterion | Developers |
| Exploratory | Edge cases with complex nested delimiters | Manual charters | QA / Contributors |

### 2.2 Test Environments

- Local development (macOS/Linux).
- CI (GitHub Actions) with Ubuntu latest.

### 2.3 Risk‑Based Prioritization

| Risk | Test Focus |
|------|------------|
| Incorrect insertion breaks valid code | Extensive unit tests on insertion heuristics. |
| Multi‑error repair infinite loop | Test with limit and known complex cases. |
| Scoping misses nested functions | Test with nested and multiple same‑named functions. |

## 3. Test Case Specifications

### TC‑BAL‑001: Remove extra parenthesis

- **Requirement:** FR‑BAL‑001, FR‑BAL‑003
- **Preconditions:** File with extra `)`.
- **Steps:** Run `balance --apply`.
- **Expected:** Extra `)` removed; file valid.
- **Automated:** Yes (`tests/repair_tests.rs`).

### TC‑BAL‑002: Insert missing brace

- **Requirement:** FR‑BAL‑002, FR‑BAL‑003
- **Preconditions:** File missing `}`.
- **Steps:** Run `balance --apply`.
- **Expected:** `}` inserted at correct location.
- **Automated:** Yes.

### TC‑BAL‑003: Multi‑error repair

- **Requirement:** FR‑BAL‑005
- **Preconditions:** File with two delimiter errors.
- **Steps:** Run `balance --apply`.
- **Expected:** Both errors fixed.
- **Automated:** Yes.

### TC‑BAL‑004: Function scoping (unique)

- **Requirement:** FR‑BAL‑007
- **Preconditions:** File with errors in two functions.
- **Steps:** Run `balance --function foo --apply`.
- **Expected:** Only `foo` repaired.
- **Automated:** Yes.

### TC‑BAL‑005: Function scoping (ambiguous)

- **Requirement:** FR‑BAL‑008
- **Preconditions:** File with two functions named `foo`.
- **Steps:** Run `balance --function foo`.
- **Expected:** Error message about ambiguity; exit code 1.
- **Automated:** Yes.

### TC‑BAL‑006: Dry‑run output

- **Requirement:** FR‑BAL‑004
- **Preconditions:** File with error.
- **Steps:** Run `balance` without `--apply`.
- **Expected:** Descriptive message; file unchanged.
- **Automated:** Yes.

### TC‑BAL‑007: JSON output

- **Requirement:** FR‑BAL‑010
- **Preconditions:** File with error.
- **Steps:** Run `balance --apply --json`.
- **Expected:** Valid JSON with actions array.
- **Automated:** Yes.

## 4. NFR Verification Plans

### NFR‑PERF‑001 (Performance)

- **Method:** Benchmark using Criterion.
- **Test:** `benches/repair_benchmark.rs` measuring `balance` on 10k‑line file.
- **Threshold:** p95 < 150 ms.

### NFR‑REL‑001 (Determinism)

- **Method:** Repeated runs on same input corpus; compare outputs.
- **Test:** Integration test that runs `balance` twice and asserts identical output.

### NFR‑COMPAT‑001 (Backward Compatibility)

- **Method:** Run existing v0.2.0 test suite against v0.3.0 binary.
- **Test:** All existing tests must pass.

## 5. Requirements Traceability Matrix (RTM)

| SRS ID | Test Case(s) | Verification Method | Status |
|--------|--------------|---------------------|--------|
| FR‑BAL‑001 | TC‑BAL‑001, TC‑BAL‑003 | Test | Planned |
| FR‑BAL‑002 | TC‑BAL‑002, TC‑BAL‑003 | Test | Planned |
| FR‑BAL‑003 | TC‑BAL‑001, TC‑BAL‑002 | Test | Planned |
| FR‑BAL‑004 | TC‑BAL‑006 | Test | Planned |
| FR‑BAL‑005 | TC‑BAL‑003 | Test | Planned |
| FR‑BAL‑006 | (unit test) | Test | Planned |
| FR‑BAL‑007 | TC‑BAL‑004 | Test | Planned |
| FR‑BAL‑008 | TC‑BAL‑005 | Test | Planned |
| FR‑BAL‑009 | (unit test) | Test | Planned |
| FR‑BAL‑010 | TC‑BAL‑007 | Test | Planned |
| NFR‑PERF‑001 | bench | Analysis + Test | Planned |
| NFR‑REL‑001 | determinism test | Test | Planned |
| NFR‑COMPAT‑001 | regression suite | Test | Planned |

## 6. Living Documentation Strategy

- Feature files (Gherkin) stored in `tests/features/` and executed via `cucumber‑rust`.
- Test results published in CI as HTML reports.
- ADRs and design docs kept in `docs/` alongside code.

---

This completes the specification suite for patch-ts v0.3.0. The documents provide a clear, traceable path from vision through implementation and verification.
