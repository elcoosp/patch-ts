# patch-ts v0.6.0 — Specification Documents

## Document Set Overview

| Document | Purpose |
|----------|---------|
| [Vision & Strategic Alignment](#patch-ts-vision--strategic-alignment) | Long‑term direction and success criteria for polyglot expansion. |
| [Business & Stakeholder Requirements (BRS)](#patch-ts-business--stakeholder-requirements-specification-brs) | Business goals, stakeholders, and high‑level needs for v0.6.0. |
| [Software Requirements Specification (SRS)](#patch-ts-software-requirements-specification-srs) | Functional and non‑functional requirements for v0.6.0. |
| [Architecture & Design Specification](#patch-ts-architecture--design-specification) | Architectural decisions, language trait extensions, and ADRs. |
| [Behavioral Specification & Test Verification Plan](#patch-ts-behavioral-specification--test-verification-plan) | Acceptance criteria, test strategy, and traceability for v0.6.0 features. |

---

# patch-ts Vision & Strategic Alignment

| Field | Value |
|-------|-------|
| Project | patch-ts |
| Document | Vision & Strategic Alignment |
| Version | 4.0 (v0.6.0) |
| Date | 2026-04-22 |
| Author | patch-ts team, assisted by spec-writer |
| Status | Draft |

## 1. Vision Statement

> *patch-ts becomes the universal patching companion for polyglot developers and AI agents, supporting all major programming languages with intelligent, context‑aware repairs that just work.*

## 2. Elevator Pitch (Moore Template)

> For **AI coding agents and developers working across Rust, TypeScript, JavaScript, Python, and Go** who need a single, reliable patching tool that handles syntax errors intelligently, **patch-ts** is a **tree‑sitter‑backed CLI** that provides **fuzzy matching, auto‑repair, marker‑based targeting, and now supports five languages with robust batch repair**. Unlike fragmented toolchains, our product **auto‑detects language from file extension** and **applies language‑specific heuristics**, making it the one patching utility for every codebase.

## 3. Problem Statement & Business Context

**Problem:** patch-ts v0.5.0 supports Rust, TypeScript, and JavaScript, but Python and Go developers are left out. Batch repair can fail when multiple errors overlap (e.g., a missing brace that shifts offsets for subsequent fixes). Performance on large files with many errors could be improved.

**Why now:**  
- Python and Go are top‑10 languages; supporting them triples the addressable user base.  
- AI agents frequently generate patches across multiple languages; a unified tool reduces integration complexity.  
- Robust batch repair is a prerequisite for reliable headless operation.

**Business drivers:**  
- Expand user base to Python and Go communities.  
- Increase patch success rate to >98% in benchmark suite.  
- Strengthen position as the definitive patching utility for AI‑assisted development.

## 4. Target Users / Customers

| Segment | Description |
|---------|-------------|
| **Python developers** | Data scientists, backend engineers, AI/ML practitioners. |
| **Go developers** | Cloud infrastructure, CLI tools, and microservices developers. |
| **AI coding agents** | Require consistent patching across all major languages. |
| **CI/CD pipelines** | Benefit from reliable batch repair in automated workflows. |

**Explicitly NOT targeting (v0.6.0):**  
- Additional languages (Ruby, PHP, Java, C/C++) — deferred to v0.7.0+.  
- Semantic repairs beyond delimiters.  
- Configuration file.

## 5. User Needs & Value Proposition

| Need | patch-ts v0.6.0 Value |
|------|------------------------|
| "I need to patch Python files with the same reliability as Rust." | Python support via `tree-sitter-python`. |
| "I need to fix unbalanced delimiters in Go code." | Go support via `tree-sitter-go`. |
| "Batch repair sometimes breaks when errors overlap." | Overlapping error resolution re‑evaluates positions after each fix. |
| "I need faster repair on large files." | Performance optimizations reduce parsing overhead. |

**Differentiator:** patch-ts is the only CLI patching tool supporting five major languages with context‑aware insertion, batch repair, and structured diagnostics.

## 6. Desired Outcomes & Success Metrics

### Business Outcomes (v0.6.0)

| ID | Outcome | Key Result / Target |
|----|---------|---------------------|
| G‑1 | Expand language coverage | Python and Go support fully integrated; all existing commands work. |
| G‑2 | Improve batch repair reliability | >98% of multi‑error test cases repaired correctly in one pass. |
| G‑3 | Maintain performance | Balance on 10k‑line Python/Go files ≤ 1.5× Rust baseline. |

### Product Outcomes (v0.6.0)

| ID | Outcome | Metric |
|----|---------|--------|
| P‑1 | Users can patch Python files | CLI accepts `.py` files with auto‑detection. |
| P‑2 | Users can patch Go files | CLI accepts `.go` files with auto‑detection. |
| P‑3 | Overlapping errors fixed correctly | Test suite with overlapping errors passes. |

## 7. Strategic Constraints

| Constraint | Description |
|------------|-------------|
| **Backward compatibility** | v0.6.0 CLI must accept all v0.5.0 flags and produce equivalent behavior. |
| **Dependency footprint** | Add `tree-sitter-python` and `tree-sitter-go`; keep binary size reasonable. |
| **Performance** | Batch repair on 10k‑line files ≤ 200 ms (p95). |

## 8. Goals and Non‑Goals (v0.6.0)

### Goals

- Implement `PythonLanguage` and `GoLanguage` structs satisfying the `Language` trait.
- Auto‑detect `.py` and `.go` file extensions.
- Implement overlapping error resolution in `balance_file` (re‑compute positions after each repair or use offset mapping).
- Optimize batch repair by reducing re‑parsing.
- Add integration tests for Python and Go.

### Non‑Goals (explicitly excluded from v0.6.0)

- Semantic repairs (e.g., indentation fixes, import sorting).
- Additional languages beyond Python and Go.
- Configuration file.

## 9. Operational Concept & High‑Level Scenarios

### Concept of Operations

`patch-ts balance` on Python/Go files works identically to existing languages. Overlapping errors are handled by applying repairs from end to start, re‑validating offsets to avoid corruption. Performance is improved by minimizing full AST re‑parses.

### High‑Level Scenarios (v0.6.0)

1. **Patch a Python file**  
   `patch-ts patch --file main.py --line 10 --old "print('hello')" --new "print('world')"`

2. **Balance a Go file with overlapping errors**  
   `patch-ts balance --file main.go --apply` correctly fixes both a missing `)` and an extra `}`.

3. **Explain a syntax error in Python**  
   `patch-ts explain --file script.py --line 5 --json` returns language‑specific diagnostic.

## 10. Stakeholders, Sponsorship & Governance

| Role | Name / Org | Responsibility |
|------|------------|----------------|
| **Executive Sponsor** | (Project maintainer) | Approves strategic direction. |
| **Product Owner** | (Project maintainer) | Prioritizes features, manages roadmap. |
| **Engineering Lead** | (Core contributor) | Oversees technical implementation. |

## 11. Risks, Assumptions & Open Questions

### Assumptions

- `tree-sitter-python` and `tree-sitter-go` grammars are stable and expose delimiter nodes consistently.
- The character‑based scanner works correctly for Python and Go.

### Risks

| Risk | Likelihood | Impact | Mitigation |
|------|------------|--------|------------|
| Python/Go grammars have different delimiter node kinds | Medium | Medium | Test extensively; add language‑specific mappings. |
| Overlapping error resolution introduces new bugs | Medium | High | Incremental implementation with comprehensive tests. |

### Open Questions

- Should we support `.pyi` (Python stub) files? (Yes, treat as Python.)
- Should we support Go workspaces? (No, single‑file only.)

---

# patch-ts Business & Stakeholder Requirements Specification (BRS)

| Field | Value |
|-------|-------|
| Project | patch-ts |
| Document | Business & Stakeholder Requirements Specification |
| Version | 4.0 (v0.6.0) |
| Date | 2026-04-22 |
| Author | patch-ts team, assisted by spec-writer |
| Status | Draft |
| References | Vision v4.0 |

## 1. Business Context

### 1.1 Purpose

This BRS defines business‑level requirements for patch-ts v0.6.0, adding Python and Go support, and improving batch repair reliability.

### 1.2 Business Problem / Opportunity

patch-ts currently supports Rust, TypeScript, and JavaScript. Python and Go developers lack a reliable, fuzzy‑aware patching tool. Batch repair can fail on overlapping errors, reducing trust in automated workflows.

### 1.3 Scope Boundaries

**In scope:**  
- Python and Go language support.  
- Overlapping error resolution in batch repair.  
- Performance optimizations.  
- Integration tests for Python and Go.

**Out of scope:**  
- Other languages.  
- Semantic repairs.  
- Multi‑file operations.

## 2. Business Goals, Objectives & Success Metrics

| ID | Business Goal | Success Metric (Fit Criterion) |
|----|---------------|-------------------------------|
| BR‑001 | Add Python support | 100% of CLI commands work on `.py` files in test suite. |
| BR‑002 | Add Go support | 100% of CLI commands work on `.go` files in test suite. |
| BR‑003 | Improve batch repair success rate | Multi‑error test suite pass rate increases from 85% to ≥98%. |

## 3. Business Model & Processes

patch-ts remains an open‑source CLI tool. Expanded language support drives adoption and community contributions.

## 4. Business Rules & Policies

| ID | Rule | Source |
|----|------|--------|
| BR‑R1 | Language detection based on file extension; `.py` → Python, `.go` → Go. | Design simplicity. |
| BR‑R2 | Overlapping repairs must not corrupt file content. | Quality requirement. |

## 5. Stakeholders & User Classes

| Stakeholder / User Class | Description | Primary Goals |
|--------------------------|-------------|---------------|
| **Python Developer** | Data, backend, AI/ML. | Apply patches safely; fix delimiter errors. |
| **Go Developer** | Cloud, CLI, microservices. | Same as above. |
| **AI Agent** | Generates patches across languages. | Consistent, reliable patching. |

## 6. Glossary / Ubiquitous Language

| Term | Definition |
|------|------------|
| **Overlapping errors** | Multiple delimiter errors where fixing one shifts the byte offsets of subsequent errors. |
| **Offset mapping** | Technique to track original positions and adjust repairs dynamically. |

## 7. Conceptual Domain Model

**Core entities (unchanged):**  
- `SourceFile`, `ParseTree`, `DelimiterError`, `RepairAction`

**New relationships:**  
- `PythonLanguage` and `GoLanguage` implement `Language` trait.

## 8. Stakeholder Needs & User Requirements

| ID | Stakeholder Need | User Class |
|----|------------------|------------|
| SN‑001 | As a Python developer, I want to use all patch-ts commands on `.py` files. | Python Developer |
| SN‑002 | As a Go developer, I want the same fuzzy patching and auto‑repair. | Go Developer |
| SN‑003 | As an AI agent, I want batch repair to handle overlapping errors correctly. | AI Agent |

## 9. System‑in‑Context & Operational Concept

`patch-ts` now supports `.py` and `.go` extensions. Batch repair processes errors from end to start, re‑computing positions after each repair using the original content and accumulated offset shifts.

## 10. Stakeholder‑Level Constraints & Quality Expectations

| ID | Constraint / Quality Expectation |
|----|----------------------------------|
| C‑001 | Python/Go support must not degrade Rust/TS/JS performance. |
| C‑002 | Overlapping error repair must be correct for ≥98% of test cases. |

## 11. Risks, Assumptions & Open Issues

### Assumptions
- `tree-sitter-python` and `tree-sitter-go` are compatible with tree-sitter 0.26.

### Risks
| Risk | Mitigation |
|------|------------|
| Python's significant whitespace causes parsing edge cases. | Rely on tree-sitter's robust error recovery. |

### Open Issues
- How to handle Python's indentation‑based blocks? (Scanner is delimiter‑only; indentation ignored.)

## 12. Traceability Mapping to Vision

| Vision Goal | BRS Goal | Stakeholder Need |
|-------------|----------|------------------|
| G‑1 | BR‑001, BR‑002 | SN‑001, SN‑002 |
| G‑2 | BR‑003 | SN‑003 |
| P‑1 | BR‑001 | SN‑001 |
| P‑2 | BR‑002 | SN‑002 |
| P‑3 | BR‑003 | SN‑003 |

---

# patch-ts Software Requirements Specification (SRS)

| Field | Value |
|-------|-------|
| Project | patch-ts |
| Document | Software Requirements Specification |
| Version | 4.0 (v0.6.0) |
| Date | 2026-04-22 |
| Author | patch-ts team, assisted by spec-writer |
| Status | Draft |
| References | BRS v4.0, Vision v4.0 |

## 1. Introduction & Scope

This SRS defines functional and non‑functional requirements for patch-ts v0.6.0, adding Python and Go support and improving batch repair.

### 1.1 Scope

- Implement `PythonLanguage` and `GoLanguage`.
- Auto‑detect `.py` and `.go` extensions.
- Overlapping error resolution in batch repair.
- Performance optimizations.

### 1.2 Out of Scope

- Other languages.
- Semantic repairs.
- Configuration file.

## 2. System Context & Overview

**Context Diagram (C1):** Unchanged. `Language` trait now has five implementors.

## 3. Functional Capabilities & Behavior

### Feature: Python Language Support

| ID | Requirement (EARS pattern) | Priority | Acceptance Criteria |
|----|----------------------------|----------|---------------------|
| FR‑PY‑001 | **The system shall** provide a `PythonLanguage` struct implementing `Language`. | Must | Trait methods compile and pass tests. |
| FR‑PY‑002 | **When** parsing Python source, **the system shall** use `tree-sitter-python`. | Must | Correct grammar loaded. |
| FR‑PY‑003 | **The system shall** support `.py` and `.pyi` extensions. | Should | Files with these extensions use Python language. |

### Feature: Go Language Support

| ID | Requirement | Priority | Acceptance Criteria |
|----|-------------|----------|---------------------|
| FR‑GO‑001 | **The system shall** provide a `GoLanguage` struct implementing `Language`. | Must | Trait methods compile and pass tests. |
| FR‑GO‑002 | **When** parsing Go source, **the system shall** use `tree-sitter-go`. | Must | Correct grammar loaded. |
| FR‑GO‑003 | **The system shall** support `.go` extension. | Must | Files with `.go` use Go language. |

### Feature: Overlapping Error Resolution

| ID | Requirement | Priority | Acceptance Criteria |
|----|-------------|----------|---------------------|
| FR‑OVER‑001 | **When** applying multiple repairs, **the system shall** adjust subsequent repair positions based on accumulated offset shifts. | Must | Overlapping errors fixed without corruption. |
| FR‑OVER‑002 | **The system shall** validate the final content after all repairs; if invalid, report unresolved errors. | Must | Error message lists unresolved issues. |

### Feature: Language Detection Update

| ID | Requirement | Priority | Acceptance Criteria |
|----|-------------|----------|---------------------|
| FR‑LANG‑004 | **The system shall** detect Python from `.py` and `.pyi` extensions. | Must | `detect_language` returns `PythonLanguage`. |
| FR‑LANG‑005 | **The system shall** detect Go from `.go` extension. | Must | `detect_language` returns `GoLanguage`. |

## 4. Quality & Non‑Functional Requirements

| ID | Category | Requirement | Fit Criterion |
|----|----------|-------------|---------------|
| NFR‑PERF‑001 | Performance efficiency | Batch repair on 10k‑line Python file ≤ 200 ms (p95). | Benchmark suite. |
| NFR‑ACC‑001 | Accuracy | Overlapping error repair correct for ≥98% of test corpus. | Automated test pass rate. |
| NFR‑COMPAT‑001 | Compatibility | All v0.5.0 tests pass without modification. | CI regression suite. |

## 5. External Interfaces & Data Contracts

**CLI Interface:** Unchanged.

**JSON Output Schema:** Unchanged.

## 6. Constraints, Assumptions & Dependencies

| Type | Description |
|------|-------------|
| Constraint | Add `tree-sitter-python` and `tree-sitter-go` crates. |
| Assumption | Grammars expose standard delimiter nodes. |
| Dependency | `tree-sitter` 0.26, `tree-sitter-python` 0.23, `tree-sitter-go` 0.23. |

## 7. TBD Log

| ID | Item | Owner | Due |
|----|------|-------|-----|
| TBD‑001 | Verify Python grammar exposes MISSING nodes for delimiters. | Engineering | Before implementation. |

## 8. Requirements Attributes & Traceability Model

**ID Scheme:**  
- Functional: `FR‑PY‑###`, `FR‑GO‑###`, `FR‑OVER‑###`, `FR‑LANG‑###`  
- NFR: `NFR‑CAT‑###`

**Traceability Matrix (excerpt):**

| SRS ID | BRS ID | Verification Method |
|--------|--------|---------------------|
| FR‑PY‑001 | BR‑001 | Test |
| FR‑GO‑001 | BR‑002 | Test |
| FR‑OVER‑001 | BR‑003 | Test |
| NFR‑PERF‑001 | C‑001 | Analysis + Bench |

---

# patch-ts Architecture & Design Specification

| Field | Value |
|-------|-------|
| Project | patch-ts |
| Document | Architecture & Design Specification |
| Version | 4.0 (v0.6.0) |
| Date | 2026-04-22 |
| Author | patch-ts team, assisted by spec-writer |
| Status | Draft |
| References | SRS v4.0, BRS v4.0 |

## 1. Context & Scope

This document describes architectural changes for v0.6.0: adding Python and Go support, and improving overlapping error resolution.

## 2. Goals & Non‑Goals

### Goals

- Add `PythonLanguage` and `GoLanguage` implementors.
- Implement offset‑aware batch repair.
- Optimize performance by caching parse results.

### Non‑Goals

- Refactor `Language` trait.
- Add language‑specific repair heuristics beyond scanner.

## 3. Architecturally Significant Requirements (ASRs)

| ASR ID | Description | Source |
|--------|-------------|--------|
| ASR‑001 | Batch repair must handle overlapping errors without corruption. | FR‑OVER‑001 |
| ASR‑002 | New languages must not regress performance. | NFR‑PERF‑001 |
| ASR‑003 | Backward compatibility with v0.5.0 must be maintained. | NFR‑COMPAT‑001 |

## 4. The Design

### 4.1 System Overview (C4 Level 2)

```
[User/Agent] → (CLI) → [patch-ts Binary]
                           ├── cli.rs (language detection)
                           ├── ast.rs (Language trait + 5 implementors)
                           ├── repair.rs (batch repair with offset tracking)
                           └── file.rs (atomic write)
                                     ↓
              ┌──────────────────┼──────────────────┬──────────────────┬──────────────────┐
              ↓                  ↓                  ↓                  ↓                  ↓
     [tree-sitter-rust] [tree-sitter-typescript] [tree-sitter-javascript] [tree-sitter-python] [tree-sitter-go]
```

### 4.2 Key Design Changes

**4.2.1 PythonLanguage and GoLanguage**

Thin wrappers similar to existing languages:

```rust
pub struct PythonLanguage { parser: Parser }
impl Language for PythonLanguage { ... }
```

Use `scan_extra_delimiter_errors` + `find_missing_delimiters` with appropriate grammar.

**4.2.2 Overlapping Error Resolution**

In `balance_file`, after sorting errors by descending start byte, track a cumulative offset shift:

```rust
let mut offset_shift: isize = 0;
for error in &sorted_errors {
    let adjusted_span = error.span().shift(offset_shift);
    current_content = apply_repair_at(&current_content, &error, adjusted_span);
    offset_shift += error.delta(); // negative for removal, positive for insertion
}
```

**4.2.3 Performance Optimizations**

- Cache the initial parse result; reuse for validation after repairs.
- Only re‑parse after all repairs are applied.

### 4.3 Data Model

Extend `DelimiterError` with methods to get span and delta:

```rust
impl DelimiterError {
    fn span(&self) -> &Span { ... }
    fn delta(&self) -> isize {
        match self {
            Extra { .. } => -(span length as isize),
            Missing { .. } => 1,
        }
    }
}
```

Add `Span::shift(&self, offset: isize) -> Span`.

### 4.4 Security Architecture

No changes.

## 5. Architecture Decision Records (ADRs)

### ADR‑009: Offset tracking for overlapping repairs

**Context:** Applying repairs from end to start works for non‑overlapping errors, but overlapping errors still cause offset corruption.

**Decision:** Track cumulative offset shift and adjust each repair's span before applying.

**Alternatives:** Re‑parse after each repair (slower). Rejected for performance.

**Consequences:** Accurate overlapping repair with minimal overhead.

### ADR‑010: Reuse scanner for Python/Go

**Context:** Need delimiter detection for new languages.

**Decision:** Use existing character‑based scanner + MISSING queries; no language‑specific traversal.

**Alternatives:** Implement grammar‑specific traversal. Rejected as unnecessary.

**Consequences:** Fast, consistent; may miss language‑specific edge cases (acceptable for v0.6.0).

## 6. API & Interface Contracts

No public API changes.

## 7. Cross‑cutting Concerns

| Concern | Approach |
|---------|----------|
| **Testing** | New integration tests for Python/Go; overlapping error test suite. |
| **Performance** | Benchmark batch repair on multi‑error Python/Go files. |

## 8. Alternatives Considered

| Alternative | Why Rejected |
|-------------|--------------|
| Full AST rewrite for repair | Too complex. |
| Language‑specific scanner for each language | Code duplication. |

## 9. Traceability

| ASR | ADR | Component |
|-----|-----|-----------|
| ASR‑001 | ADR‑009 | repair.rs |
| ASR‑002 | ADR‑010 | ast.rs |
| ASR‑003 | – | All |

---

# patch-ts Behavioral Specification & Test Verification Plan

| Field | Value |
|-------|-------|
| Project | patch-ts |
| Document | Behavioral Specification & Test Verification Plan |
| Version | 4.0 (v0.6.0) |
| Date | 2026-04-22 |
| Author | patch-ts team, assisted by spec-writer |
| Status | Draft |
| References | SRS v4.0, Architecture v4.0 |

## 1. Behavioral Specifications (Specification by Example)

### Feature: Python Support

#### Scenario: Balance a Python file

```gherkin
Feature: Python language support
  Scenario: Fix missing parenthesis in Python
    Given a file "script.py" with content:
      """
      print("hello"
      """
    When I run `patch-ts balance --file script.py --apply`
    Then the file content becomes:
      """
      print("hello")
      """
```

#### Scenario: Patch a Python file exactly

```gherkin
  Scenario: Exact patch on Python file
    Given a file "main.py" with content:
      """
      x = 1
      y = 2
      """
    When I run `patch-ts patch --file main.py --line 1 --old "x = 1" --new "x = 42"`
    Then the file content becomes:
      """
      x = 42
      y = 2
      """
```

### Feature: Go Support

#### Scenario: Balance a Go file

```gherkin
Feature: Go language support
  Scenario: Fix extra brace in Go
    Given a file "main.go" with content:
      """
      package main
      func main() {
          println("hi")
      }
      }
      """
    When I run `patch-ts balance --file main.go --apply`
    Then the extra '}' is removed.
```

### Feature: Overlapping Error Resolution

#### Scenario: Fix overlapping errors correctly

```gherkin
Feature: Batch repair with overlapping errors
  Scenario: Missing paren and extra brace overlapping
    Given a file "overlap.py" with content:
      """
      x = (1 + 2
      }
      """
    When I run `patch-ts balance --file overlap.py --apply`
    Then the file content becomes:
      """
      x = (1 + 2)
      """
```

### Decision Table: Language Detection

| Extension | Language Selected |
|-----------|------------------|
| `.py`     | Python |
| `.pyi`    | Python |
| `.go`     | Go |

## 2. Test Strategy & Plan

### 2.1 Test Pyramid

| Level | Scope | Tools |
|-------|-------|-------|
| Unit | Offset tracking, language detection | Rust `#[test]` |
| Integration | Full commands on Python/Go files | `assert_cmd` |
| Regression | v0.5.0 test suite | Cargo test |
| Performance | Benchmark batch repair | Criterion |

### 2.2 Risk‑Based Prioritization

| Risk | Test Focus |
|------|------------|
| Offset tracking corrupts content | Extensive overlapping error corpus. |
| Python/Go grammars missing delimiter nodes | Verify MISSING queries work. |

## 3. Test Case Specifications

### TC‑PY‑001: Python balance

- **Requirement:** FR‑PY‑001
- **Preconditions:** `.py` file with missing `)`.
- **Steps:** Run `balance --apply`.
- **Expected:** `)` inserted.
- **Automated:** Yes.

### TC‑GO‑001: Go balance

- **Requirement:** FR‑GO‑001
- **Preconditions:** `.go` file with extra `}`.
- **Steps:** Run `balance --apply`.
- **Expected:** Extra `}` removed.
- **Automated:** Yes.

### TC‑OVER‑001: Overlapping errors

- **Requirement:** FR‑OVER‑001
- **Preconditions:** File with overlapping errors.
- **Steps:** Run `balance --apply`.
- **Expected:** Both fixed; file valid.
- **Automated:** Yes.

## 4. NFR Verification Plans

### NFR‑PERF‑001 (Performance)

- **Method:** Criterion benchmark on 10k‑line Python file with 5 errors.
- **Threshold:** p95 ≤ 200 ms.

### NFR‑ACC‑001 (Accuracy)

- **Method:** Run overlapping error test suite.
- **Threshold:** ≥98% pass.

## 5. Requirements Traceability Matrix (RTM)

| SRS ID | Test Case(s) | Verification Method |
|--------|--------------|---------------------|
| FR‑PY‑001 | TC‑PY‑001 | Test |
| FR‑GO‑001 | TC‑GO‑001 | Test |
| FR‑OVER‑001 | TC‑OVER‑001 | Test |
| NFR‑PERF‑001 | bench | Analysis |
