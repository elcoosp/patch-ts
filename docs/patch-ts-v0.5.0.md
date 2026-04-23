I'll create the full specification suite for **patch-ts v0.5.0** using the layered documentation structure. The documents build on the completed v0.4.0 baseline and focus on enhanced insertion diagnostics, batch repair, and improved TypeScript/JavaScript support.

---

# patch-ts v0.5.0 — Specification Documents

## Document Set Overview

| Document | Purpose |
|----------|---------|
| [Vision & Strategic Alignment](#patch-ts-vision--strategic-alignment) | Long‑term direction and success criteria for enhanced diagnostics and repair. |
| [Business & Stakeholder Requirements (BRS)](#patch-ts-business--stakeholder-requirements-specification-brs) | Business goals, stakeholders, and high‑level needs for v0.5.0. |
| [Software Requirements Specification (SRS)](#patch-ts-software-requirements-specification-srs) | Functional and non‑functional requirements for v0.5.0. |
| [Architecture & Design Specification](#patch-ts-architecture--design-specification) | Architectural decisions, algorithm improvements, and ADRs. |
| [Behavioral Specification & Test Verification Plan](#patch-ts-behavioral-specification--test-verification-plan) | Acceptance criteria, test strategy, and traceability for v0.5.0 features. |

---

# patch-ts Vision & Strategic Alignment

| Field | Value |
|-------|-------|
| Project | patch-ts |
| Document | Vision & Strategic Alignment |
| Version | 3.0 (v0.5.0) |
| Date | 2026-04-22 |
| Author | patch-ts team, assisted by spec-writer |
| Status | Draft |

## 1. Vision Statement

> *patch-ts becomes the definitive, intelligent patching tool for polyglot developers and AI agents—one that not only finds and fixes delimiter errors but understands the structural context to apply repairs with surgical precision, across all supported languages.*

## 2. Elevator Pitch (Moore Template)

> For **AI coding agents and developers working in Rust, TypeScript, and JavaScript** who need patching tools that "just work" even when the patch introduces subtle syntax errors, **patch-ts** is a **tree‑sitter‑backed CLI** that provides **context‑aware delimiter insertion, batch error repair, and language‑specific diagnostics**. Unlike basic patching tools, our product **analyzes the AST to find the optimal insertion point** and **fixes multiple errors in a single pass**, dramatically reducing manual intervention.

## 3. Problem Statement & Business Context

**Problem:** v0.4.0 successfully added TypeScript/JavaScript support, but missing delimiter insertion remains unreliable for certain edge cases (e.g., missing closing brace at end of block, missing parenthesis in complex expressions). Additionally, when multiple errors exist, the tool requires multiple invocations. Diagnostic messages for TS/JS are generic, reducing their utility for AI agents and developers.

**Why now:**  
- Users expect seamless repair regardless of error location.  
- AI agents need precise, actionable feedback to iterate effectively.  
- Fixing these gaps solidifies patch-ts as a trustworthy, production‑ready tool.

**Business drivers:**  
- Reduce user frustration and support burden.  
- Increase patch success rate from ~85% to >95% in benchmark suite.  
- Strengthen position as the go‑to patching utility for AI‑assisted development.

## 4. Target Users / Customers

| Segment | Description |
|---------|-------------|
| **TypeScript/JavaScript developers** | Primary beneficiaries of improved insertion and diagnostics. |
| **AI coding agents** | Require reliable, precise patching and structured feedback. |
| **CI/CD pipelines** | Benefit from batch repair reducing command invocations. |
| **Tool builders** | Value consistent, predictable behavior across languages. |

**Explicitly NOT targeting (v0.5.0):**  
- Additional languages (Python, Go).  
- Semantic repairs beyond delimiters (e.g., missing semicolons).  
- Configuration file support.

## 5. User Needs & Value Proposition

| Need | patch-ts v0.5.0 Value |
|------|------------------------|
| "I need missing braces to be inserted at the correct location, every time." | Context‑aware insertion uses parent node information to place delimiters accurately. |
| "I don't want to run `balance` multiple times to fix all errors." | Batch repair processes all detected errors in one pass. |
| "I need clear, language‑specific error messages for TypeScript." | Enhanced `explain` provides actionable suggestions based on grammar. |
| "I need to trust that JSON output follows a consistent schema." | JSON Schema validation ensures output structure is predictable and documented. |

**Differentiator:** patch-ts is the only CLI patching tool that combines multi‑language support, context‑aware insertion, batch repair, and structured diagnostics in a single binary.

## 6. Desired Outcomes & Success Metrics

### Business Outcomes (v0.5.0)

| ID | Outcome | Key Result / Target |
|----|---------|---------------------|
| G‑1 | Improve patch success rate | >95% of patches in benchmark suite apply without manual intervention. |
| G‑2 | Reduce user friction | Batch repair eliminates need for multiple `balance` invocations in 90% of multi‑error cases. |
| G‑3 | Enhance agent usability | JSON diagnostics include `suggestion` field with actionable fix description for all TS/JS errors. |

### Product Outcomes (v0.5.0)

| ID | Outcome | Metric |
|----|---------|--------|
| P‑1 | Missing delimiters inserted at correct location | Insertion point accuracy >95% on test corpus. |
| P‑2 | Multiple errors fixed in one run | `balance` command resolves all delimiter errors in a single invocation for files with up to 5 errors. |
| P‑3 | Language‑specific diagnostics | `explain` output references TypeScript/JavaScript grammar constructs (e.g., "missing closing brace for function body"). |

## 7. Strategic Constraints

| Constraint | Description |
|------------|-------------|
| **Backward compatibility** | v0.5.0 CLI must accept all v0.4.0 flags and produce equivalent behavior for Rust. |
| **Performance** | Batch repair on 10k‑line file ≤ 200 ms (p95). |
| **Dependencies** | No new heavy dependencies; use existing tree‑sitter and `jsonschema` (dev‑only). |

## 8. Goals and Non‑Goals (v0.5.0)

### Goals

- Implement context‑aware insertion point detection using parent node information.
- Support batch repair in `balance_file` by processing all detected errors before re‑parsing.
- Enhance `explain_error` for TypeScript/JavaScript with grammar‑specific messages.
- Add JSON Schema validation for `BalanceResult` and `JsonError` in tests.

### Non‑Goals (explicitly excluded from v0.5.0)

- Semantic repairs (e.g., adding missing type annotations, fixing borrow checker errors).
- Support for additional languages.
- Multi‑file batch repair.
- Configuration file.

## 9. Operational Concept & High‑Level Scenarios

### Concept of Operations

`patch-ts balance` now analyzes the AST to determine the optimal insertion point for missing delimiters. When multiple errors exist, all are collected and applied in a single pass, reducing the need for iterative runs. `patch-ts explain` provides richer, language‑aware diagnostics that help users and agents understand and fix issues faster.

### High‑Level Scenarios (v0.5.0)

1. **Accurate missing brace insertion**  
   File: `function main() { console.log("hi");`  
   `patch-ts balance --apply` inserts `}` at the end of the function body, not at EOF.

2. **Batch repair of multiple errors**  
   File: `const x = (1 + 2;\nfunction foo() {`  
   `patch-ts balance --apply` inserts `)` after `2` and `}` at end of `foo` body in one run.

3. **Enhanced TypeScript diagnostic**  
   `patch-ts explain --file app.ts --line 10 --json` returns:
   ```json
   {
     "error": {
       "code": "patch_ts::missing_delimiter",
       "message": "Missing closing brace for function 'calculate'",
       "suggestion": "Add '}' at line 15"
     }
   }
   ```

## 10. Stakeholders, Sponsorship & Governance

| Role | Name / Org | Responsibility |
|------|------------|----------------|
| **Executive Sponsor** | (Project maintainer) | Approves strategic direction. |
| **Product Owner** | (Project maintainer) | Prioritizes features, manages roadmap. |
| **Engineering Lead** | (Core contributor) | Oversees technical implementation. |

## 11. Risks, Assumptions & Open Questions

### Assumptions

- Tree‑sitter provides sufficient parent/child relationships to determine correct insertion points.
- The current scanner and MISSING query can be combined to produce a complete error list for batch repair.

### Risks

| Risk | Likelihood | Impact | Mitigation |
|------|------------|--------|------------|
| Context‑aware insertion misplaces delimiter in edge cases | Medium | High | Extensive test corpus; fallback to previous heuristic. |
| Batch repair introduces performance regression | Low | Medium | Benchmark and optimize; keep iteration limit. |

### Open Questions

- Should batch repair apply all fixes even if one fails? (Yes, apply best‑effort and report failures.)

---

# patch-ts Business & Stakeholder Requirements Specification (BRS)

| Field | Value |
|-------|-------|
| Project | patch-ts |
| Document | Business & Stakeholder Requirements Specification |
| Version | 3.0 (v0.5.0) |
| Date | 2026-04-22 |
| Author | patch-ts team, assisted by spec-writer |
| Status | Draft |
| References | Vision v3.0 |

## 1. Business Context

### 1.1 Purpose

This BRS defines the business‑level requirements for patch-ts v0.5.0, focusing on improving missing delimiter insertion accuracy, enabling batch repair, and enhancing diagnostics for TypeScript/JavaScript.

### 1.2 Business Problem / Opportunity

v0.4.0 successfully added TS/JS support, but user feedback indicates that missing delimiter insertion sometimes places delimiters incorrectly, and multiple errors require multiple tool invocations. Diagnostics are too generic for effective AI agent consumption. Addressing these gaps will significantly improve user satisfaction and agent success rates.

### 1.3 Scope Boundaries

**In scope:**  
- Context‑aware insertion point detection for missing delimiters.  
- Batch repair of multiple delimiter errors in one `balance` run.  
- Language‑specific error messages for TS/JS in `explain`.  
- JSON Schema validation for output structures.

**Out of scope:**  
- Semantic repairs.  
- Additional languages.  
- Multi‑file operations.

## 2. Business Goals, Objectives & Success Metrics

| ID | Business Goal | Success Metric (Fit Criterion) |
|----|---------------|-------------------------------|
| BR‑001 | Increase patch success rate | Benchmark suite success rate improves from 85% to ≥95%. |
| BR‑002 | Reduce user friction | 90% of multi‑error files are fully repaired in a single `balance` invocation. |
| BR‑003 | Improve agent usability | JSON diagnostics include actionable `suggestion` field for all TS/JS syntax errors. |

## 3. Business Model & Processes

patch-ts remains an open‑source CLI tool. Continued quality improvements drive adoption and community contributions.

## 4. Business Rules & Policies

| ID | Rule | Source |
|----|------|--------|
| BR‑R1 | All repairs must be atomic and create a backup unless `--no-backup` is specified. | Project policy |
| BR‑R2 | Batch repair must not introduce new syntax errors beyond those being fixed. | Quality requirement |

## 5. Stakeholders & User Classes

| Stakeholder / User Class | Description | Primary Goals |
|--------------------------|-------------|---------------|
| **TypeScript/JavaScript Developer** | Primary user. | Accurate repairs; fewer command invocations. |
| **AI Agent** | Consumes JSON output. | Structured, actionable diagnostics. |
| **CI/CD System** | Automated pipelines. | Reliable, predictable behavior. |

## 6. Glossary / Ubiquitous Language

| Term | Definition |
|------|------------|
| **Context‑aware insertion** | Determining the insertion point for a missing delimiter by examining its parent AST node. |
| **Batch repair** | Applying multiple delimiter fixes in a single pass without intermediate re‑parsing. |
| **MISSING node** | A tree‑sitter node indicating a missing token, accessible via query. |

## 7. Conceptual Domain Model

**Core entities (extended):**  
- `DelimiterError` now includes optional `parent_node_kind` field to aid insertion logic.  
- `RepairAction` may include multiple actions for batch application.

## 8. Stakeholder Needs & User Requirements

| ID | Stakeholder Need | User Class |
|----|------------------|------------|
| SN‑001 | As a TypeScript developer, I want missing braces inserted at the correct location, not at EOF. | Developer |
| SN‑002 | As a JavaScript developer, I want all delimiter errors fixed in one command. | Developer |
| SN‑003 | As an AI agent, I want language‑specific error messages with fix suggestions. | AI Agent |

## 9. System‑in‑Context & Operational Concept

`patch-ts balance` will now:  
1. Parse the file.  
2. Collect all delimiter errors (extra and missing) using scanner + MISSING queries.  
3. Sort errors by position (from end to start to avoid offset shifts).  
4. Apply all repairs in a single content mutation.  
5. Validate final result.

`patch-ts explain` will use tree‑sitter node types to generate contextual messages (e.g., "missing '}' for function body").

## 10. Stakeholder‑Level Constraints & Quality Expectations

| ID | Constraint / Quality Expectation |
|----|----------------------------------|
| C‑001 | Insertion accuracy must exceed 95% on curated test corpus. |
| C‑002 | Batch repair must not degrade performance beyond 2× single‑repair latency. |

## 11. Risks, Assumptions & Open Issues

### Assumptions
- Tree‑sitter grammars expose parent node information reliably.
- Sorting repairs from end to start avoids offset corruption.

### Risks
| Risk | Mitigation |
|------|------------|
| Incorrect insertion due to ambiguous parent context | Fallback to EOF insertion with warning. |
| Batch repair introduces new errors | Validate after each repair; rollback if invalid. |

### Open Issues
- How to handle overlapping repairs? (Process from end to start to avoid conflicts.)

## 12. Traceability Mapping to Vision

| Vision Goal | BRS Goal | Stakeholder Need |
|-------------|----------|------------------|
| G‑1 | BR‑001 | SN‑001, SN‑002 |
| G‑2 | BR‑002 | SN‑002 |
| G‑3 | BR‑003 | SN‑003 |
| P‑1 | BR‑001 | SN‑001 |
| P‑2 | BR‑002 | SN‑002 |
| P‑3 | BR‑003 | SN‑003 |

---

# patch-ts Software Requirements Specification (SRS)

| Field | Value |
|-------|-------|
| Project | patch-ts |
| Document | Software Requirements Specification |
| Version | 3.0 (v0.5.0) |
| Date | 2026-04-22 |
| Author | patch-ts team, assisted by spec-writer |
| Status | Draft |
| References | BRS v3.0, Vision v3.0 |

## 1. Introduction & Scope

This SRS defines functional and non‑functional requirements for patch-ts v0.5.0, enhancing insertion accuracy, batch repair, and diagnostics.

### 1.1 Scope

- Context‑aware insertion for missing delimiters.
- Batch repair of multiple delimiter errors.
- Language‑specific `explain` messages for TS/JS.
- JSON Schema validation in tests.

### 1.2 Out of Scope

- Semantic repairs.
- New languages.
- Configuration file.

## 2. System Context & Overview

**Context Diagram (C1):** Unchanged. The `Language` trait implementors gain enhanced capabilities.

## 3. Functional Capabilities & Behavior

### Feature: Context‑Aware Insertion

| ID | Requirement (EARS pattern) | Priority | Acceptance Criteria |
|----|----------------------------|----------|---------------------|
| FR‑INS‑001 | **When** a missing delimiter is detected via MISSING query, **the system shall** determine the insertion point by examining the parent node of the MISSING node. | Must | Insertion point matches expected location in test corpus. |
| FR‑INS‑002 | **If** the parent node is a block (e.g., `function_body`, `block`), **then** the system shall insert the missing delimiter at the end of that block. | Must | Missing `}` inserted at block end, not EOF. |
| FR‑INS‑003 | **If** the parent node is an expression requiring a closing parenthesis, **the system shall** insert the missing `)` immediately after the last token of the expression. | Must | Missing `)` inserted after operand. |
| FR‑INS‑004 | **If** the parent context is ambiguous, **the system shall** fall back to the previous heuristic (insert at MISSING node span) and log a warning. | Should | Fallback occurs; warning printed in verbose mode. |

### Feature: Batch Repair

| ID | Requirement | Priority | Acceptance Criteria |
|----|-------------|----------|---------------------|
| FR‑BATCH‑001 | **The system shall** collect all delimiter errors (extra and missing) in a single traversal before applying any repairs. | Must | All errors identified in one pass. |
| FR‑BATCH‑002 | **The system shall** sort errors by descending byte position (end to start) before applying repairs to avoid offset shifts. | Must | Repairs applied without corrupting subsequent positions. |
| FR‑BATCH‑003 | **The system shall** apply all repairs in a single content mutation and validate the result. | Must | File becomes valid after single `balance` run for multi‑error files. |
| FR‑BATCH‑004 | **If** the final content is still invalid, **the system shall** report which errors could not be fixed. | Should | Error message lists unresolved issues. |

### Feature: Enhanced Diagnostics for TS/JS

| ID | Requirement | Priority | Acceptance Criteria |
|----|-------------|----------|---------------------|
| FR‑DIAG‑001 | **The system shall** provide language‑specific error messages for TypeScript and JavaScript in `explain` output. | Must | Messages reference TS/JS grammar constructs (e.g., "function body", "arrow function"). |
| FR‑DIAG‑002 | **When** JSON output is requested, **the system shall** include a `suggestion` field with a human‑readable fix description. | Must | `suggestion` field present and actionable. |
| FR‑DIAG‑003 | **The system shall** map common MISSING node types to user‑friendly messages (e.g., `(MISSING "}")` → "Missing closing brace"). | Must | All MISSING delimiters have descriptive messages. |

### Feature: JSON Schema Validation

| ID | Requirement | Priority | Acceptance Criteria |
|----|-------------|----------|---------------------|
| FR‑JSON‑001 | **The system shall** provide a JSON Schema definition for `BalanceResult` and `JsonError` structures. | Should | Schema files exist in `schemas/` directory. |
| FR‑JSON‑002 | **The system's test suite shall** validate all JSON outputs against the schema. | Should | CI fails if output doesn't match schema. |

## 4. Quality & Non‑Functional Requirements

| ID | Category | Requirement | Fit Criterion |
|----|----------|-------------|---------------|
| NFR‑PERF‑001 | Performance efficiency | Batch repair on 10k‑line TS file ≤ 200 ms (p95). | Benchmark suite. |
| NFR‑ACC‑001 | Accuracy | Insertion point correct for ≥95% of test corpus. | Automated test pass rate. |
| NFR‑COMPAT‑001 | Compatibility | All v0.4.0 tests pass without modification. | CI regression suite. |
| NFR‑MAINT‑001 | Maintainability | Context‑aware logic isolated in helper functions with unit tests. | Code coverage ≥80%. |

## 5. External Interfaces & Data Contracts

**CLI Interface:** Unchanged.

**JSON Output Schema (enhanced):**

```json
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "type": "object",
  "properties": {
    "success": { "type": "boolean" },
    "actions": { "type": "array", "items": { "$ref": "#/definitions/BalanceAction" } },
    "error": { "$ref": "#/definitions/JsonError" }
  },
  "definitions": {
    "BalanceAction": {
      "type": "object",
      "properties": {
        "type": { "enum": ["remove", "insert"] },
        "delimiter": { "type": "string", "maxLength": 1 },
        "line": { "type": "integer" },
        "column": { "type": "integer" },
        "message": { "type": "string" },
        "suggestion": { "type": "string" }
      },
      "required": ["type", "delimiter", "line", "column", "message"]
    }
  }
}
```

## 6. Constraints, Assumptions & Dependencies

| Type | Description |
|------|-------------|
| Constraint | Must maintain backward compatibility with v0.4.0 CLI. |
| Assumption | Tree‑sitter grammars provide consistent parent node information. |
| Dev‑dependency | `jsonschema` crate for test validation. |

## 7. TBD Log

| ID | Item | Owner | Due |
|----|------|-------|-----|
| TBD‑001 | Define exact parent node kinds for each insertion heuristic. | Engineering | Before implementation. |

## 8. Requirements Attributes & Traceability Model

**ID Scheme:**  
- Functional: `FR‑INS‑###`, `FR‑BATCH‑###`, `FR‑DIAG‑###`, `FR‑JSON‑###`  
- NFR: `NFR‑CAT‑###`

**Traceability Matrix (excerpt):**

| SRS ID | BRS ID | Verification Method |
|--------|--------|---------------------|
| FR‑INS‑001 | SN‑001 | Test |
| FR‑BATCH‑001 | SN‑002 | Test |
| FR‑DIAG‑001 | SN‑003 | Test |
| NFR‑PERF‑001 | C‑002 | Analysis + Bench |
| NFR‑ACC‑001 | BR‑001 | Test |

---

# patch-ts Architecture & Design Specification

| Field | Value |
|-------|-------|
| Project | patch-ts |
| Document | Architecture & Design Specification |
| Version | 3.0 (v0.5.0) |
| Date | 2026-04-22 |
| Author | patch-ts team, assisted by spec-writer |
| Status | Draft |
| References | SRS v3.0, BRS v3.0 |

## 1. Context & Scope

This document describes architectural changes for v0.5.0: context‑aware insertion, batch repair, and enhanced diagnostics.

## 2. Goals & Non‑Goals

### Goals

- Extend `DelimiterError::Missing` with optional `parent_kind` field.
- Implement `compute_insertion_span` using parent context.
- Modify `balance_file` to collect all errors before applying repairs.
- Enhance `explain_error` with language‑specific message generation.
- Add JSON Schema files and test validation.

### Non‑Goals

- Refactor entire repair module.
- Change public API signatures (except internal enhancements).

## 3. Architecturally Significant Requirements (ASRs)

| ASR ID | Description | Source |
|--------|-------------|--------|
| ASR‑001 | Batch repair must process all errors in one pass without performance regression. | NFR‑PERF‑001 |
| ASR‑002 | Insertion logic must be language‑agnostic where possible, with language‑specific overrides. | FR‑INS‑001 |
| ASR‑003 | Backward compatibility with v0.4.0 must be maintained. | NFR‑COMPAT‑001 |

## 4. The Design

### 4.1 System Overview (C4 Level 2)

Unchanged. Enhancements localized to `ast.rs` and `repair.rs`.

### 4.2 Key Design Changes

**4.2.1 Context‑Aware Insertion**

Add `parent_kind: Option<String>` to `DelimiterError::Missing`. When a MISSING node is found, capture its parent's kind. In `apply_repair`, use this to compute insertion span:

```rust
fn compute_insertion_span(&self, error: &DelimiterError, content: &str, index: &LineIndex) -> Span {
    match error {
        DelimiterError::Missing { expected, insert_at, parent_kind } => {
            if let Some(kind) = parent_kind {
                if kind == "function_body" || kind == "block" {
                    // Insert at end of block (before closing brace if exists, else EOF)
                    return block_end_span(insert_at);
                }
                // ... other heuristics
            }
            // Fallback
            insert_at.clone()
        }
        _ => unreachable!()
    }
}
```

**4.2.2 Batch Repair**

In `balance_file`, replace the iterative loop with:

1. Parse once.
2. Collect all errors via `language.find_delimiter_errors(&parse_result)`.
3. If errors empty, bail.
4. Sort errors by descending byte position.
5. Apply all repairs sequentially to a mutable string.
6. Parse final content; if invalid, report unresolved errors.

**4.2.3 Enhanced Diagnostics**

Add `language_specific_message(&self, error: &DelimiterError) -> String` to `Language` trait. Implement for `TypeScriptLanguage` and `JavaScriptLanguage` using parent kind mapping.

**4.2.4 JSON Schema Validation**

Add `schemas/balance_result.schema.json` and `schemas/json_error.schema.json`. In tests, validate outputs using `jsonschema` crate.

### 4.3 Data Model

Extend `DelimiterError::Missing`:

```rust
Missing {
    expected: char,
    insert_at: Span,
    parent_kind: Option<String>,
}
```

### 4.4 Security Architecture

No changes.

## 5. Architecture Decision Records (ADRs)

### ADR‑007: Batch repair using sorted errors

**Context:** Multiple errors require multiple `balance` runs in v0.4.0.

**Decision:** Collect all errors, sort by descending position, apply in one pass.

**Alternatives:** Iterative re‑parse (current). Rejected as inefficient.

**Consequences:** Faster, single‑pass repair; must handle overlapping errors carefully.

### ADR‑008: Parent context for insertion

**Context:** MISSING node span alone insufficient for accurate insertion.

**Decision:** Capture parent node kind during error detection; use heuristics based on kind.

**Alternatives:** Full AST rewrite. Rejected as overkill.

**Consequences:** Improved accuracy with minimal complexity; fallback available.

## 6. API & Interface Contracts

No public API changes. Internal `DelimiterError` gains a field (backward compatible for serialization if marked `#[serde(skip)]`).

## 7. Cross‑cutting Concerns

| Concern | Approach |
|---------|----------|
| **Testing** | New unit tests for insertion heuristics; integration tests for batch repair. |
| **Performance** | Benchmark batch repair vs iterative; optimize sorting. |

## 8. Alternatives Considered

| Alternative | Why Rejected |
|-------------|--------------|
| Use tree‑sitter edits API | Adds complexity; manual string mutation is sufficient. |
| Implement full AST rewrite for repair | Too heavy; delimiter repair is a narrow problem. |

## 9. Traceability

| ASR | ADR | Component |
|-----|-----|-----------|
| ASR‑001 | ADR‑007 | repair.rs |
| ASR‑002 | ADR‑008 | ast.rs |
| ASR‑003 | – | All |

---

# patch-ts Behavioral Specification & Test Verification Plan

| Field | Value |
|-------|-------|
| Project | patch-ts |
| Document | Behavioral Specification & Test Verification Plan |
| Version | 3.0 (v0.5.0) |
| Date | 2026-04-22 |
| Author | patch-ts team, assisted by spec-writer |
| Status | Draft |
| References | SRS v3.0, Architecture v3.0 |

## 1. Behavioral Specifications (Specification by Example)

### Feature: Context‑Aware Insertion

#### Scenario: Missing brace at end of function body

```gherkin
Feature: Balance command with context-aware insertion
  Scenario: Insert missing closing brace for function body
    Given a file "app.ts" with content:
      """
      function main() {
        console.log("hi");
      """
    When I run `patch-ts balance --file app.ts --apply`
    Then the file content becomes:
      """
      function main() {
        console.log("hi");
      }
      """
```

#### Scenario: Missing parenthesis in expression

```gherkin
  Scenario: Insert missing closing parenthesis
    Given a file "calc.ts" with content:
      """
      const result = (1 + 2;
      """
    When I run `patch-ts balance --file calc.ts --apply`
    Then the file content becomes:
      """
      const result = (1 + 2);
      """
```

### Feature: Batch Repair

#### Scenario: Multiple errors fixed in one run

```gherkin
  Scenario: Fix extra brace and missing parenthesis together
    Given a file "multi.ts" with content:
      """
      const x = (1 + 2;
      function foo() {
      }
      }
      """
    When I run `patch-ts balance --file multi.ts --apply`
    Then the file content becomes:
      """
      const x = (1 + 2);
      function foo() {
      }
      """
```

### Feature: Enhanced Diagnostics

#### Scenario: Language‑specific error message

```gherkin
  Scenario: Explain missing brace in TypeScript function
    Given a file "app.ts" with missing closing brace
    When I run `patch-ts explain --file app.ts --line 1 --json`
    Then the JSON output contains:
      | field                     | value                                    |
      | error.code                | "patch_ts::missing_delimiter"            |
      | error.message             | "Missing closing brace for function body"|
      | error.suggestion          | "Add '}' at line 4"                      |
```

### Decision Table: Insertion Heuristics

| Parent Kind | Expected Insertion |
|-------------|-------------------|
| `function_body` | End of block |
| `block` | End of block |
| `arguments` | After last argument |
| `parenthesized_expression` | After expression |
| Other | Fallback to MISSING node span |

## 2. Test Strategy & Plan

### 2.1 Test Pyramid

| Level | Scope | Tools |
|-------|-------|-------|
| Unit | Insertion heuristics, error sorting | Rust `#[test]` |
| Integration | Full `balance` and `explain` commands | `assert_cmd` |
| Contract | JSON Schema validation | `jsonschema` |
| Regression | v0.4.0 test suite | Cargo test |

### 2.2 Risk‑Based Prioritization

| Risk | Test Focus |
|------|------------|
| Incorrect insertion breaks valid code | Extensive corpus of valid/invalid TS/JS snippets. |
| Batch repair corrupts offsets | Test with overlapping error positions. |
| Performance regression | Benchmark before/after. |

## 3. Test Case Specifications

### TC‑INS‑001: Function body brace insertion

- **Requirement:** FR‑INS‑002
- **Preconditions:** File with missing `}` at end of function.
- **Steps:** Run `balance --apply`.
- **Expected:** `}` inserted at correct line.
- **Automated:** Yes.

### TC‑BATCH‑001: Multi‑error repair

- **Requirement:** FR‑BATCH‑001, FR‑BATCH‑003
- **Preconditions:** File with two delimiter errors.
- **Steps:** Run `balance --apply`.
- **Expected:** Both errors fixed in one run.
- **Automated:** Yes.

### TC‑DIAG‑001: TypeScript missing brace message

- **Requirement:** FR‑DIAG‑001
- **Preconditions:** `.ts` file with missing `}`.
- **Steps:** Run `explain --json`.
- **Expected:** Message references function body.
- **Automated:** Yes.

### TC‑JSON‑001: Schema validation

- **Requirement:** FR‑JSON‑002
- **Preconditions:** JSON output from any command.
- **Steps:** Validate against schema using `jsonschema`.
- **Expected:** Validation passes.
- **Automated:** Yes (in test suite).

## 4. NFR Verification Plans

### NFR‑PERF‑001 (Batch repair performance)

- **Method:** Criterion benchmark on 10k‑line TS file with 5 errors.
- **Threshold:** p95 ≤ 200 ms.

### NFR‑ACC‑001 (Insertion accuracy)

- **Method:** Run on curated corpus of 100 TS/JS files with known insertion points.
- **Threshold:** ≥95% correct.

### NFR‑COMPAT‑001 (Backward compatibility)

- **Method:** Run v0.4.0 test suite.
- **Threshold:** 100% pass.

## 5. Requirements Traceability Matrix (RTM)

| SRS ID | Test Case(s) | Verification Method |
|--------|--------------|---------------------|
| FR‑INS‑001 | TC‑INS‑001 | Test |
| FR‑BATCH‑001 | TC‑BATCH‑001 | Test |
| FR‑DIAG‑001 | TC‑DIAG‑001 | Test |
| FR‑JSON‑002 | TC‑JSON‑001 | Test |
| NFR‑PERF‑001 | bench | Analysis |
| NFR‑ACC‑001 | corpus test | Test |

## 6. Living Documentation Strategy

- JSON Schema files in `schemas/` serve as executable contracts.
- Feature files for BDD scenarios stored in `tests/features/`.
- CI publishes test reports and schema validation results.

---

This completes the specification suite for patch-ts v0.5.0.
