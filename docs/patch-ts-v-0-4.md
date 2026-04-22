I'll create the full specification suite for **patch-ts v0.4.0** using the `spec-writer` framework. The documents build on the existing v0.3.0 baseline and focus on multi‑language support (TypeScript/JavaScript).

---

# patch-ts v0.4.0 — Specification Documents

## Document Set Overview

| Document | Purpose |
|----------|---------|
| [Vision & Strategic Alignment](#patch-ts-vision--strategic-alignment) | Long‑term direction and success criteria for multi‑language expansion. |
| [Business & Stakeholder Requirements (BRS)](#patch-ts-business--stakeholder-requirements-specification-brs) | Business goals, stakeholders, and high‑level needs for TypeScript/JavaScript support. |
| [Software Requirements Specification (SRS)](#patch-ts-software-requirements-specification-srs) | Functional and non‑functional requirements for v0.4.0. |
| [Architecture & Design Specification](#patch-ts-architecture--design-specification) | Architectural decisions, trait extensions, and ADRs for language abstraction. |
| [Behavioral Specification & Test Verification Plan](#patch-ts-behavioral-specification--test-verification-plan) | Acceptance criteria, test strategy, and traceability for v0.4.0 features. |

---

# patch-ts Vision & Strategic Alignment

| Field | Value |
|-------|-------|
| Project | patch-ts |
| Document | Vision & Strategic Alignment |
| Version | 2.0 (v0.4.0) |
| Date | 2026-04-22 |
| Author | patch-ts team, assisted by spec-writer |
| Status | Draft |

## 1. Vision Statement

> *patch-ts becomes the go‑to, language‑agnostic patching tool for AI agents and developers, starting with robust support for Rust, TypeScript, and JavaScript. We enable safe, fuzzy, and self‑repairing code modifications across the most popular languages in modern development.*

## 2. Elevator Pitch (Moore Template)

> For **AI coding agents and developers working with TypeScript/JavaScript** who need the same reliable patching experience currently available for Rust, **patch-ts** is a **tree‑sitter‑backed CLI** that provides **fuzzy matching, auto‑repair, and marker‑based targeting across multiple languages**. Unlike single‑language tools, our product **auto‑detects the language from file extensions** and **applies language‑specific repair heuristics**, making it the universal patching utility for polyglot codebases.

## 3. Problem Statement & Business Context

**Problem:** patch‑ts v0.3.0 is Rust‑only. Many AI agents and developers work in TypeScript/JavaScript ecosystems and cannot benefit from patch‑ts's fuzzy matching and auto‑repair capabilities.

**Why now:**  
- TypeScript/JavaScript are among the most used languages globally.  
- AI‑generated patches for frontend and Node.js code frequently fail due to line drift and delimiter errors.  
- Extending patch‑ts to a second language proves the `Language` trait's extensibility and unlocks future multi‑language growth.

**Business drivers:**  
- Increase patch‑ts's addressable user base by 10×.  
- Validate the architecture for adding more languages (Python, Go, etc.) in future releases.  
- Strengthen patch‑ts's position as the universal patching companion for AI coding assistants.

## 4. Target Users / Customers

| Segment | Description |
|---------|-------------|
| **TypeScript/JavaScript developers** | Frontend, backend (Node.js), and full‑stack developers applying patches. |
| **AI coding agents** | LLM‑based tools generating patches for TS/JS codebases. |
| **CI/CD pipelines** | Automated systems applying bulk fixes (e.g., lint autofix, dependency updates). |
| **Tool builders** | Developers creating cross‑language refactoring tools. |

**Explicitly NOT targeting (v0.4.0):**  
- Other languages (Python, Go, etc.) — deferred to future versions.  
- Framework‑specific repair (React, Angular) — kept language‑generic.  
- JSX/TSX‑specific AST handling — treated as regular syntax.

## 5. User Needs & Value Proposition

| Need | patch‑ts v0.4.0 Value |
|------|------------------------|
| "I need fuzzy patching for my TypeScript codebase." | Language auto‑detection routes `.ts`/`.tsx` files to the TypeScript grammar. |
| "I need to fix unbalanced braces in JavaScript." | The `balance` command now works for `{}`, `()`, `[]` in JS/TS. |
| "I don't want to remember to pass `--lang` every time." | File extension detection automatically selects the correct language. |
| "I need JSON diagnostics for my agent." | Consistent JSON output across both Rust and TypeScript/JavaScript. |

**Differentiator:** patch‑ts is the only CLI patching tool that combines fuzzy matching, auto‑repair, marker targeting, and multi‑language support in a single, lightweight binary.

## 6. Desired Outcomes & Success Metrics

### Business Outcomes (v0.4.0)

| ID | Outcome | Key Result / Target |
|----|---------|---------------------|
| G‑1 | Expand user base | At least 50 GitHub stars from TypeScript/JavaScript community within 3 months of release. |
| G‑2 | Validate multi‑language architecture | Zero breaking changes to Rust functionality; all existing tests pass. |
| G‑3 | Maintain performance | Patching/balance on TS/JS files ≤ 1.5× Rust file latency (benchmarked). |

### Product Outcomes (v0.4.0)

| ID | Outcome | Metric |
|----|---------|--------|
| P‑1 | Users can patch TypeScript files without manual language selection. | CLI accepts `.ts`/`.tsx`/`.js`/`.jsx` files with auto‑detection. |
| P‑2 | Users can balance delimiters in TypeScript/JavaScript. | `balance` command fixes extra/missing `{}`, `()`, `[]` in TS/JS test corpus. |
| P‑3 | Users receive consistent JSON diagnostics across languages. | JSON output schema identical for Rust and TS/JS operations. |

## 7. Strategic Constraints

| Constraint | Description |
|------------|-------------|
| **Backward compatibility** | v0.4.0 CLI must accept all v0.3.0 flags and produce equivalent Rust behavior. |
| **Dependency footprint** | Add `tree-sitter-typescript` and `tree-sitter-javascript`; keep binary size reasonable. |
| **Performance** | Language detection overhead < 1 ms. |
| **Code maintainability** | Avoid duplicating repair logic; leverage the existing `Language` trait. |

## 8. Goals and Non‑Goals (v0.4.0)

### Goals

- Implement `TypeScriptLanguage` and `JavaScriptLanguage` structs that satisfy the `Language` trait.
- Auto‑detect language from file extension (`.rs` → Rust, `.ts`/`.tsx` → TypeScript, `.js`/`.jsx` → JavaScript).
- Ensure `find_delimiter_errors` works for TS/JS (the character‑based scanner is language‑agnostic; only parser setup differs).
- Ensure `balance_file` works for TS/JS (leveraging the scanner).
- Add integration tests for TypeScript/JavaScript files.

### Non‑Goals (explicitly excluded from v0.4.0)

- Support for JSX/TSX‑specific AST queries (treated as regular syntax).
- TypeScript‑specific semantic repairs (e.g., adding missing type annotations).
- Language‑specific `explain_error` enhancements (use generic error messages).
- Multi‑file patch application.
- Configuration file (`patch-ts.toml`).

## 9. Operational Concept & High‑Level Scenarios

### Concept of Operations

Users invoke `patch-ts` exactly as before, but now on `.ts`/`.js` files. The tool inspects the file extension, instantiates the appropriate `Language` implementor, and proceeds with the requested operation. All commands (`patch`, `balance`, `explain`) work identically across languages.

### High‑Level Scenarios (v0.4.0)

1. **Patch a TypeScript file with fuzzy matching**  
   `patch-ts patch --file src/app.ts --line 42 --old "const x = 1;" --new "const x = 2;" --fuzz 3`  
   → The tool uses TypeScript grammar, finds the fuzzy match, and applies the patch.

2. **Balance a JavaScript file**  
   `patch-ts balance --file dist/bundle.js --apply`  
   → Extra/missing braces/parentheses are repaired using the JavaScript parser.

3. **Explain a syntax error in a TypeScript file**  
   `patch-ts explain --file src/component.tsx --line 15 --json`  
   → Returns JSON diagnostic with error location and suggestion.

4. **Auto‑detection fallback**  
   If a file has no extension or an unknown extension, the tool errors with a clear message listing supported languages.

## 10. Stakeholders, Sponsorship & Governance

| Role | Name / Org | Responsibility |
|------|------------|----------------|
| **Executive Sponsor** | (Project maintainer) | Approves strategic direction. |
| **Product Owner** | (Project maintainer) | Prioritizes features, manages roadmap. |
| **Engineering Lead** | (Core contributor) | Oversees technical implementation. |
| **Contributors** | Open‑source community | Submit PRs, report issues. |

## 11. Risks, Assumptions & Open Questions

### Assumptions

- `tree-sitter-typescript` and `tree-sitter-javascript` provide stable grammars.
- The character‑based delimiter scanner works correctly for TS/JS (no language‑specific edge cases that break it).
- Users will not expect TypeScript‑specific semantic repairs in v0.4.0.

### Risks

| Risk | Likelihood | Impact | Mitigation |
|------|------------|--------|------------|
| TS/JS grammars have different node kinds that break existing `find_extra_delimiter` logic | Medium | Medium | Keep `find_extra_delimiter` Rust‑only for now; rely on scanner for `balance`. |
| Performance degradation due to additional dependencies | Low | Low | Benchmark and optimize if needed. |
| Confusion about supported languages | Medium | Low | Clear CLI help and error messages. |

### Open Questions

- Should we support `.mjs`/`.cjs`? (Yes, treat as JavaScript.)
- Should we support `.vue` or `.svelte`? (No, out of scope.)

---

# patch-ts Business & Stakeholder Requirements Specification (BRS)

| Field | Value |
|-------|-------|
| Project | patch-ts |
| Document | Business & Stakeholder Requirements Specification |
| Version | 2.0 (v0.4.0) |
| Date | 2026-04-22 |
| Author | patch-ts team, assisted by spec-writer |
| Status | Draft |
| References | Vision v2.0 |

## 1. Business Context

### 1.1 Purpose

This BRS defines the business‑level requirements for patch-ts v0.4.0, which adds support for TypeScript and JavaScript as first‑class languages alongside Rust.

### 1.2 Business Problem / Opportunity

patch-ts is currently limited to Rust, excluding the vast TypeScript/JavaScript ecosystem. AI agents and developers in those ecosystems lack a reliable, fuzzy‑aware patching tool with auto‑repair. Adding TS/JS support expands the potential user base dramatically and proves the architecture's extensibility.

### 1.3 Scope Boundaries

**In scope:**  
- Language auto‑detection from file extension.  
- Implementation of `TypeScriptLanguage` and `JavaScriptLanguage` satisfying the `Language` trait.  
- Full support for `patch`, `balance`, and `explain` commands on TS/JS files.  
- Integration tests for TS/JS.

**Out of scope:**  
- Other languages (Python, Go, etc.).  
- Framework‑specific features (React hooks, Angular templates).  
- TypeScript‑specific semantic repairs (type checking).  
- Multi‑file patches.

## 2. Business Goals, Objectives & Success Metrics

| ID | Business Goal | Success Metric (Fit Criterion) |
|----|---------------|-------------------------------|
| BR‑001 | Extend patch‑ts to TypeScript/JavaScript | 100% of existing CLI commands work on `.ts`/`.tsx`/`.js`/`.jsx` files in test suite. |
| BR‑002 | Maintain Rust functionality unchanged | All v0.3.0 tests pass without modification. |
| BR‑003 | Achieve adoption in TS/JS community | At least 10 external contributions or issues filed within 3 months. |

## 3. Business Model & Processes

patch-ts remains an open‑source CLI tool. The "business model" is community growth and integration into AI agent workflows. Multi‑language support is a key driver for adoption.

## 4. Business Rules & Policies

| ID | Rule | Source |
|----|------|--------|
| BR‑R1 | Language is determined by file extension; no content‑based detection. | Design simplicity. |
| BR‑R2 | If extension is unknown, the tool must error with a clear message. | User experience. |
| BR‑R3 | All existing flags and behaviors must remain unchanged for Rust files. | Backward compatibility. |

## 5. Stakeholders & User Classes

| Stakeholder / User Class | Description | Primary Goals |
|--------------------------|-------------|---------------|
| **TypeScript Developer** | Writes frontend/backend TS code. | Apply patches safely; fix delimiter errors. |
| **JavaScript Developer** | Writes Node.js or browser JS. | Same as above. |
| **AI Agent** | Generates patches for TS/JS. | Reliable, machine‑readable patching. |
| **CI/CD System** | Runs in pipelines. | Consistent behavior across languages. |

## 6. Glossary / Ubiquitous Language

| Term | Definition |
|------|------------|
| **Language trait** | Rust trait defining parser, validation, and error detection methods. |
| **Auto‑detection** | Selecting the `Language` implementor based on file extension. |
| **TS/JS** | TypeScript / JavaScript. |
| **Grammar** | tree‑sitter language definition. |

## 7. Conceptual Domain Model

**Core entities (unchanged from v0.3.0):**  
- `SourceFile`  
- `ParseTree`  
- `DelimiterError`  
- `RepairAction`

**New relationships:**  
- A `SourceFile` is associated with one `Language` implementor (Rust, TypeScript, or JavaScript) based on its extension.

## 8. Stakeholder Needs & User Requirements

| ID | Stakeholder Need | User Class |
|----|------------------|------------|
| SN‑001 | As a TypeScript developer, I want to use all patch‑ts commands on `.ts` files without extra configuration. | TypeScript Developer |
| SN‑002 | As a JavaScript developer, I want the same fuzzy patching and auto‑repair I get for Rust. | JavaScript Developer |
| SN‑003 | As an AI agent, I want consistent JSON output across languages. | AI Agent |
| SN‑004 | As a CI user, I want predictable behavior regardless of file type. | CI/CD System |

## 9. System‑in‑Context & Operational Concept

patch-ts operates as before, but now inspects the file extension at the start of each command. Based on the extension, it instantiates `RustLanguage`, `TypeScriptLanguage`, or `JavaScriptLanguage` and delegates all operations. The CLI interface remains unchanged.

## 10. Stakeholder‑Level Constraints & Quality Expectations

| ID | Constraint / Quality Expectation |
|----|----------------------------------|
| C‑001 | The tool must not require users to specify language manually. |
| C‑002 | Performance on TS/JS files should be comparable to Rust (within 2×). |
| C‑003 | Error messages for unsupported file types must be actionable. |

## 11. Risks, Assumptions & Open Issues

### Assumptions
- tree‑sitter TS/JS grammars are mature and stable.
- The existing scanner logic is language‑agnostic.

### Risks
| Risk | Mitigation |
|------|------------|
| TS/JS grammars produce different error node structures that break `find_extra_delimiter`. | Keep that method Rust‑only; rely on scanner for `balance`. |
| Users expect TS‑specific fixes (e.g., missing types). | Document limitations clearly. |

### Open Issues
- Should we support `.mjs`/`.cjs`/`.mts`/`.cts`? (Yes, treat as JS/TS respectively.)

## 12. Traceability Mapping to Vision

| Vision Goal | BRS Goal | Stakeholder Need |
|-------------|----------|------------------|
| G‑1 | BR‑001, BR‑003 | SN‑001, SN‑002 |
| G‑2 | BR‑002 | All |
| G‑3 | BR‑001 | C‑002 |
| P‑1 | BR‑001 | SN‑001, SN‑002 |
| P‑2 | BR‑001 | SN‑001, SN‑002 |
| P‑3 | BR‑001 | SN‑003 |

---

# patch-ts Software Requirements Specification (SRS)

| Field | Value |
|-------|-------|
| Project | patch-ts |
| Document | Software Requirements Specification |
| Version | 2.0 (v0.4.0) |
| Date | 2026-04-22 |
| Author | patch-ts team, assisted by spec-writer |
| Status | Draft |
| References | BRS v2.0, Vision v2.0 |

## 1. Introduction & Scope

This SRS defines functional and non‑functional requirements for patch-ts v0.4.0, adding TypeScript and JavaScript support via the `Language` trait.

### 1.1 Scope

- Implement `TypeScriptLanguage` and `JavaScriptLanguage` structs.
- Auto‑detect language by file extension.
- Ensure `patch`, `balance`, and `explain` commands work for TS/JS.
- Add integration tests for TS/JS.

### 1.2 Out of Scope

- Other languages.
- TypeScript‑specific semantic analysis.
- JSX/TSX‑specific handling beyond what the grammar provides.

## 2. System Context & Overview

**Context Diagram (C1):**  
Same as v0.3.0; now the `Language` trait is implemented by three concrete types.

**High‑level capabilities (v0.4.0):**  
- Language detection and dispatch.  
- TS/JS grammar integration.  
- Cross‑language test suite.

## 3. Functional Capabilities & Behavior

### Feature: Language Auto‑Detection

| ID | Requirement (EARS pattern) | Priority | Acceptance Criteria |
|----|----------------------------|----------|---------------------|
| FR‑LANG‑001 | **When** a command is invoked with a file path, **the system shall** determine the language from the file extension: `.rs` → Rust, `.ts`/`.tsx` → TypeScript, `.js`/`.jsx` → JavaScript. | Must | CLI selects correct language based on extension. |
| FR‑LANG‑002 | **If** the file extension is unknown or missing, **then** the system shall error with a message listing supported extensions. | Must | Error message includes `.rs, .ts, .tsx, .js, .jsx`. |
| FR‑LANG‑003 | **The system shall** treat `.mjs` and `.cjs` as JavaScript, and `.mts` and `.cts` as TypeScript. | Should | These extensions work correctly. |

### Feature: TypeScript Language Implementation

| ID | Requirement | Priority | Acceptance Criteria |
|----|-------------|----------|---------------------|
| FR‑TS‑001 | **The system shall** provide a `TypeScriptLanguage` struct that implements the `Language` trait. | Must | Trait methods compile and pass tests. |
| FR‑TS‑002 | **When** parsing TypeScript source, **the system shall** use `tree-sitter-typescript` with the `typescript` grammar. | Must | Correct grammar loaded. |
| FR‑TS‑003 | **The system shall** support `.tsx` files using the `tsx` grammar. | Must | `.tsx` files parse correctly. |

### Feature: JavaScript Language Implementation

| ID | Requirement | Priority | Acceptance Criteria |
|----|-------------|----------|---------------------|
| FR‑JS‑001 | **The system shall** provide a `JavaScriptLanguage` struct that implements the `Language` trait. | Must | Trait methods compile and pass tests. |
| FR‑JS‑002 | **When** parsing JavaScript source, **the system shall** use `tree-sitter-javascript`. | Must | Correct grammar loaded. |

### Feature: Cross‑Language Command Support

| ID | Requirement | Priority | Acceptance Criteria |
|----|-------------|----------|---------------------|
| FR‑CMD‑001 | **The system shall** support `patch`, `balance`, and `explain` commands on TypeScript files with the same semantics as Rust. | Must | Commands succeed on valid TS/JS inputs. |
| FR‑CMD‑002 | **The system shall** produce JSON output for TS/JS operations that matches the Rust schema. | Must | JSON structure identical across languages. |

## 4. Quality & Non‑Functional Requirements

| ID | Category | Requirement | Fit Criterion |
|----|----------|-------------|---------------|
| NFR‑PERF‑001 | Performance efficiency | Language detection overhead < 1 ms. | Benchmarked. |
| NFR‑PERF‑002 | Performance efficiency | `balance` on 10k‑line TS file ≤ 2× Rust file latency. | Benchmark suite. |
| NFR‑COMPAT‑001 | Compatibility | All v0.3.0 tests must pass without modification. | CI regression suite. |
| NFR‑MAINT‑001 | Maintainability | New language implementations must share no code duplication with Rust beyond the trait contract. | Code review. |

## 5. External Interfaces & Data Contracts

**CLI Interface:** Unchanged from v0.3.0. Language detection is transparent to the user.

**JSON Output Schema:** Unchanged. The `BalanceResult` and `JsonError` structures are language‑agnostic.

## 6. Constraints, Assumptions & Dependencies

| Type | Description |
|------|-------------|
| Constraint | Must add `tree-sitter-typescript` and `tree-sitter-javascript` crates. |
| Assumption | TS/JS grammars do not require additional runtime setup. |
| Dependency | `tree-sitter` 0.25. |

## 7. TBD Log

| ID | Item | Owner | Due |
|----|------|-------|-----|
| TBD‑001 | Determine if `find_extra_delimiter` should be implemented for TS/JS or remain Rust‑only. | Engineering | Before implementation. |

## 8. Requirements Attributes & Traceability Model

**ID Scheme:**  
- Functional: `FR‑LANG‑###`, `FR‑TS‑###`, `FR‑JS‑###`, `FR‑CMD‑###`  
- NFR: `NFR‑CAT‑###`

**Traceability Matrix (excerpt):**

| SRS ID | BRS ID | Verification Method |
|--------|--------|---------------------|
| FR‑LANG‑001 | SN‑001, SN‑002 | Test |
| FR‑TS‑001 | SN‑001 | Test |
| FR‑JS‑001 | SN‑002 | Test |
| FR‑CMD‑001 | SN‑001, SN‑002 | Test |
| NFR‑PERF‑001 | C‑002 | Analysis + Bench |

---

# patch-ts Architecture & Design Specification

| Field | Value |
|-------|-------|
| Project | patch-ts |
| Document | Architecture & Design Specification |
| Version | 2.0 (v0.4.0) |
| Date | 2026-04-22 |
| Author | patch-ts team, assisted by spec-writer |
| Status | Draft |
| References | SRS v2.0, BRS v2.0 |

## 1. Context & Scope

This document describes the architectural changes required to add TypeScript and JavaScript support to patch-ts, leveraging the existing `Language` trait abstraction.

## 2. Goals & Non‑Goals

### Goals
- Add `TypeScriptLanguage` and `JavaScriptLanguage` as `Language` implementors.
- Implement language detection in `cli.rs`.
- Ensure existing scanner and repair logic work unchanged for TS/JS.

### Non‑Goals
- Refactor the `Language` trait.
- Add language‑specific repair heuristics beyond delimiter scanning.

## 3. Architecturally Significant Requirements (ASRs)

| ASR ID | Description | Source |
|--------|-------------|--------|
| ASR‑001 | Language detection must be transparent and automatic. | FR‑LANG‑001 |
| ASR‑002 | All existing Rust functionality must remain unchanged. | NFR‑COMPAT‑001 |
| ASR‑003 | TS/JS implementations must reuse the scanner‑based delimiter detection. | NFR‑MAINT‑001 |

## 4. The Design

### 4.1 System Overview (C4 Level 2)

```
[User/Agent] → (CLI) → [patch-ts Binary]
                           ├── cli.rs (language detection)
                           ├── ast.rs (Language trait, RustLanguage, TypeScriptLanguage, JavaScriptLanguage)
                           ├── repair.rs (balance logic, unchanged)
                           └── file.rs (atomic write)
                                     ↓
              ┌──────────────────┼──────────────────┐
              ↓                  ↓                  ↓
     [tree-sitter-rust] [tree-sitter-typescript] [tree-sitter-javascript]
```

### 4.2 Key Design Changes

**4.2.1 Language Detection**

In `cli.rs`, before invoking any command, inspect the file extension:

```rust
fn detect_language(file_path: &Path) -> Result<Box<dyn Language>> {
    match file_path.extension().and_then(|e| e.to_str()) {
        Some("rs") => Ok(Box::new(RustLanguage::new())),
        Some("ts") | Some("tsx") | Some("mts") | Some("cts") => Ok(Box::new(TypeScriptLanguage::new())),
        Some("js") | Some("jsx") | Some("mjs") | Some("cjs") => Ok(Box::new(JavaScriptLanguage::new())),
        _ => anyhow::bail!("Unsupported file extension. Supported: .rs, .ts, .tsx, .js, .jsx, .mts, .cts, .mjs, .cjs"),
    }
}
```

**4.2.2 TypeScriptLanguage and JavaScriptLanguage**

Both structs will be thin wrappers around `tree_sitter::Parser`:

```rust
pub struct TypeScriptLanguage {
    parser: Parser,
}

impl TypeScriptLanguage {
    pub fn new() -> Self {
        let mut parser = Parser::new();
        parser.set_language(&tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into()).unwrap();
        Self { parser }
    }
}

impl Language for TypeScriptLanguage {
    fn parse(&mut self, source: &str) -> ParseResult { /* ... */ }
    fn is_valid(&self, result: &ParseResult) -> bool { /* ... */ }
    fn find_extra_delimiter(&self, result: &ParseResult) -> Option<Span> { None } // not implemented
    fn explain_error(&self, result: &ParseResult, line: usize) -> Option<SyntaxErrorDiagnostic> { /* ... */ }
    fn find_delimiter_errors(&self, result: &ParseResult) -> Vec<DelimiterError> {
        // Reuse the same scanner logic (needs access to LineIndex)
        // We'll move the scanner to a free function in ast.rs.
        scan_delimiter_errors(result.text(), &result.index)
    }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any { self }
}
```

Similarly for `JavaScriptLanguage` using `tree-sitter-javascript`.

**4.2.3 Scanner Refactoring**

Move `scan_delimiter_errors` to a free function in `ast.rs` so it can be called from any `Language` implementor.

### 4.3 Data Model

No changes.

### 4.4 Security Architecture

No changes.

## 5. Architecture Decision Records (ADRs)

### ADR‑004: Use file extension for language detection

**Context:** Need to select the correct `Language` implementor.

**Decision:** Inspect file extension only; no content‑based detection.

**Alternatives:** Parse a few lines and look for Rust/TS/JS keywords. Rejected as slower and error‑prone.

**Consequences:** Simple, fast, predictable. Users must use correct extensions.

### ADR‑005: Reuse scanner for TS/JS delimiter detection

**Context:** `balance` command relies on `find_delimiter_errors`.

**Decision:** Use the same character‑based scanner for all languages, as delimiters are universal.

**Alternatives:** Implement grammar‑specific traversal for each language. Rejected as unnecessary duplication.

**Consequences:** Fast, consistent behavior. May miss language‑specific edge cases (e.g., regex literals in JS), but scanner already handles comments and strings.

## 6. API & Interface Contracts

No new external APIs.

## 7. Cross‑cutting Concerns

| Concern | Approach |
|---------|----------|
| **Observability** | Unchanged. |
| **Error Handling** | Language detection errors are user‑friendly. |
| **Testing** | New integration tests for TS/JS files; reuse existing test patterns. |

## 8. Alternatives Considered

| Alternative | Why Rejected |
|-------------|--------------|
| Content‑based language detection | Slower, ambiguous for small files. |
| Separate scanner per language | Unnecessary code duplication. |

## 9. Traceability

| ASR | ADR | C4 Component |
|-----|-----|--------------|
| ASR‑001 | ADR‑004 | cli.rs |
| ASR‑002 | – | All |
| ASR‑003 | ADR‑005 | ast.rs, repair.rs |

---

# patch-ts Behavioral Specification & Test Verification Plan

| Field | Value |
|-------|-------|
| Project | patch-ts |
| Document | Behavioral Specification & Test Verification Plan |
| Version | 2.0 (v0.4.0) |
| Date | 2026-04-22 |
| Author | patch-ts team, assisted by spec-writer |
| Status | Draft |
| References | SRS v2.0, Architecture v2.0 |

## 1. Behavioral Specifications (Specification by Example)

### Feature: Multi‑language support

#### Scenario: Auto‑detect TypeScript file

```gherkin
Feature: Language auto-detection
  As a TypeScript developer
  I want patch-ts to automatically use the TypeScript grammar
  So that I don't need to specify the language manually

  Scenario: Patch a TypeScript file without --lang flag
    Given a file "src/app.ts" with content:
      """
      const x = 1;
      console.log(x);
      """
    When I run `patch-ts patch --file src/app.ts --line 1 --old "const x = 1;" --new "const x = 2;"`
    Then the command succeeds
    And the file content becomes:
      """
      const x = 2;
      console.log(x);
      """
```

#### Scenario: Balance a JavaScript file

```gherkin
  Scenario: Fix extra brace in JavaScript
    Given a file "script.js" with content:
      """
      function main() {
        console.log("hi");
      }
      }
      """
    When I run `patch-ts balance --file script.js --apply`
    Then the extra '}' is removed
    And the file content becomes:
      """
      function main() {
        console.log("hi");
      }
      """
```

#### Scenario: Unsupported extension errors

```gherkin
  Scenario: Unknown file extension
    Given a file "data.txt"
    When I run `patch-ts patch --file data.txt --line 1 --old "a" --new "b"`
    Then the command fails with error message containing "Unsupported file extension"
    And the error lists supported extensions.
```

### Decision Table: Language Detection

| Extension | Language Selected |
|-----------|------------------|
| `.rs`     | Rust |
| `.ts`     | TypeScript |
| `.tsx`    | TypeScript (tsx) |
| `.js`     | JavaScript |
| `.jsx`    | JavaScript |
| `.mjs`    | JavaScript |
| `.cjs`    | JavaScript |
| `.mts`    | TypeScript |
| `.cts`    | TypeScript |
| other     | Error |

## 2. Test Strategy & Plan

### 2.1 Test Pyramid

| Level | Scope | Tools | Ownership |
|-------|-------|-------|-----------|
| Unit | Language detection, scanner refactor | Rust `#[test]` | Developers |
| Integration | Full commands on TS/JS files | `assert_cmd`, tempfile | Developers |
| Regression | All existing Rust tests | Cargo test | CI |

### 2.2 Test Environments

- Local development (macOS/Linux).
- CI (GitHub Actions) with Ubuntu latest.

### 2.3 Risk‑Based Prioritization

| Risk | Test Focus |
|------|------------|
| Breaking Rust functionality | Run full v0.3.0 test suite on every change. |
| Incorrect language detection | Unit tests for `detect_language`. |
| Scanner fails on JS regex literals | Add specific test cases with regex containing braces. |

## 3. Test Case Specifications

### TC‑LANG‑001: TypeScript auto‑detection

- **Requirement:** FR‑LANG‑001
- **Preconditions:** File with `.ts` extension.
- **Steps:** Run `patch‑ts balance --file test.ts`.
- **Expected:** TypeScript grammar used; no errors.
- **Automated:** Yes.

### TC‑LANG‑002: Unsupported extension

- **Requirement:** FR‑LANG‑002
- **Preconditions:** File with `.txt` extension.
- **Steps:** Run `patch‑ts patch ...`.
- **Expected:** Error with supported extensions.
- **Automated:** Yes.

### TC‑TS‑001: Parse valid TypeScript

- **Requirement:** FR‑TS‑001
- **Preconditions:** Valid `.ts` file.
- **Steps:** Run `patch‑ts explain --file test.ts --line 1`.
- **Expected:** No syntax error reported.
- **Automated:** Yes.

### TC‑BAL‑TS‑001: Balance TypeScript file

- **Requirement:** FR‑CMD‑001
- **Preconditions:** `.ts` file with extra `}`.
- **Steps:** Run `patch‑ts balance --apply`.
- **Expected:** Extra `}` removed.
- **Automated:** Yes.

## 4. NFR Verification Plans

### NFR‑PERF‑001 (Detection overhead)

- **Method:** Benchmark `detect_language` function.
- **Threshold:** < 1 ms.

### NFR‑PERF‑002 (TS/JS performance)

- **Method:** Criterion benchmark comparing `balance` on equivalent Rust and TS files.
- **Threshold:** TS/JS ≤ 2× Rust latency.

### NFR‑COMPAT‑001 (Backward compatibility)

- **Method:** Run full test suite from v0.3.0 against v0.4.0 binary.
- **Threshold:** 100% pass.

## 5. Requirements Traceability Matrix (RTM)

| SRS ID | Test Case(s) | Verification Method | Status |
|--------|--------------|---------------------|--------|
| FR‑LANG‑001 | TC‑LANG‑001 | Test | Planned |
| FR‑LANG‑002 | TC‑LANG‑002 | Test | Planned |
| FR‑TS‑001 | TC‑TS‑001 | Test | Planned |
| FR‑CMD‑001 | TC‑BAL‑TS‑001 | Test | Planned |
| NFR‑PERF‑001 | bench | Analysis | Planned |
| NFR‑PERF‑002 | bench | Analysis | Planned |
| NFR‑COMPAT‑001 | regression suite | Test | Planned |

## 6. Living Documentation Strategy

- New test files: `tests/typescript_tests.rs`, `tests/javascript_tests.rs`.
- Feature files (Gherkin) for cross‑language scenarios.
- CI publishes test reports.

---

This completes the specification suite for patch-ts v0.4.0. The documents provide a clear, traceable path from vision through implementation and verification.
