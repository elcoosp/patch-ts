We'll create the complete specification suite and implementation plan for **patch-ts v0.8.0**. Based on the roadmap, v0.8.0 adds support for C, C++, Java, and C#, along with improvements to overlapping error resolution and performance.

---

# patch-ts v0.8.0 — Specification Documents

## Document Set Overview

| Document | Purpose |
|----------|---------|
| [Vision & Strategic Alignment](#patch-ts-vision--strategic-alignment) | Long‑term direction and success criteria for expanded language support. |
| [Business & Stakeholder Requirements (BRS)](#patch-ts-business--stakeholder-requirements-specification-brs) | Business goals, stakeholders, and high‑level needs for v0.8.0. |
| [Software Requirements Specification (SRS)](#patch-ts-software-requirements-specification-srs) | Functional and non‑functional requirements for v0.8.0. |
| [Architecture & Design Specification](#patch-ts-architecture--design-specification) | Architectural decisions, language trait extensions, and ADRs. |
| [Behavioral Specification & Test Verification Plan](#patch-ts-behavioral-specification--test-verification-plan) | Acceptance criteria, test strategy, and traceability for v0.8.0 features. |

---

# patch-ts Vision & Strategic Alignment

| Field | Value |
|-------|-------|
| Project | patch-ts |
| Document | Vision & Strategic Alignment |
| Version | 6.0 (v0.8.0) |
| Date | 2026-04-22 |
| Author | patch-ts team, assisted by spec-writer |
| Status | Draft |

## 1. Vision Statement

> *patch-ts becomes the definitive universal patching tool, supporting every major programming language with intelligent, AST‑aware repairs that just work—from Rust to C++, Java to C#.*

## 2. Elevator Pitch (Moore Template)

> For **AI coding agents and developers working across Rust, TypeScript, JavaScript, Python, Go, Ruby, PHP, HTML, XML, C, C++, Java, and C#** who need a single, reliable patching tool, **patch-ts** is a **tree‑sitter‑backed CLI** that provides **fuzzy matching, AST‑based delimiter repair, batch error resolution, and marker‑based targeting across twelve languages**. Unlike fragmented toolchains, our product **auto‑detects language** and **applies language‑specific AST heuristics**, making it the one patching utility for every codebase.

## 3. Problem Statement & Business Context

**Problem:** v0.7.0 supports eight languages, but C, C++, Java, and C# developers are still excluded. These languages represent a massive portion of enterprise and systems programming. AI agents need a unified patching solution across all common languages.

**Why now:**  
- C/C++ are foundational for systems programming and embedded development.  
- Java and C# dominate enterprise backend development.  
- Supporting these four languages completes the "top languages" coverage and positions patch-ts as truly universal.

**Business drivers:**  
- Expand user base to enterprise and systems developers.  
- Achieve >99% patch success rate across all twelve languages.  
- Solidify patch-ts as the definitive universal patching utility.

## 4. Target Users / Customers

| Segment | Description |
|---------|-------------|
| **C/C++ developers** | Systems, embedded, game, and performance‑critical developers. |
| **Java developers** | Enterprise backend, Android, and Spring developers. |
| **C# developers** | .NET, Unity, and Windows developers. |
| **AI coding agents** | Require consistent patching across all major languages. |
| **CI/CD pipelines** | Benefit from universal patching in automated workflows. |

**Explicitly NOT targeting (v0.8.0):**  
- Additional languages (Swift, Kotlin, Scala, Rust already covered).  
- Semantic repairs beyond delimiters and basic syntax.  
- Multi‑file operations.

## 5. User Needs & Value Proposition

| Need | patch-ts v0.8.0 Value |
|------|------------------------|
| "I need to patch C files with the same reliability as Rust." | C support via `tree-sitter-c`. |
| "I need to fix unbalanced braces in C++." | C++ support via `tree-sitter-cpp`. |
| "I need to patch Java files." | Java support via `tree-sitter-java`. |
| "I need to fix delimiters in C#." | C# support via `tree-sitter-c-sharp`. |
| "I need overlapping errors to never corrupt my file." | Enhanced offset tracking with rollback validation. |

**Differentiator:** patch-ts is the only CLI patching tool supporting twelve major languages with AST‑aware repair and batch resolution.

## 6. Desired Outcomes & Success Metrics

### Business Outcomes (v0.8.0)

| ID | Outcome | Key Result / Target |
|----|---------|---------------------|
| G‑1 | Expand language coverage | C, C++, Java, and C# support fully integrated; all commands work. |
| G‑2 | Improve overlapping error reliability | >99% of multi‑error test cases repaired correctly in one pass. |
| G‑3 | Maintain performance | Balance on 10k‑line files ≤ 200 ms (p95) across all twelve languages. |

### Product Outcomes (v0.8.0)

| ID | Outcome | Metric |
|----|---------|--------|
| P‑1 | Users can patch C files | CLI accepts `.c` and `.h` files with auto‑detection. |
| P‑2 | Users can patch C++ files | CLI accepts `.cpp`, `.cc`, `.cxx`, `.hpp` files. |
| P‑3 | Users can patch Java files | CLI accepts `.java` files. |
| P‑4 | Users can patch C# files | CLI accepts `.cs` files. |

## 7. Strategic Constraints

| Constraint | Description |
|------------|-------------|
| **Backward compatibility** | v0.8.0 CLI must accept all v0.7.0 flags and produce equivalent behavior. |
| **Dependency footprint** | Add `tree-sitter-c`, `tree-sitter-cpp`, `tree-sitter-java`, `tree-sitter-c-sharp`. |
| **Performance** | Balance on 10k‑line files ≤ 200 ms (p95) across all languages. |

## 8. Goals and Non‑Goals (v0.8.0)

### Goals

- Implement `CLanguage`, `CppLanguage`, `JavaLanguage`, `CSharpLanguage` structs.
- Auto‑detect `.c`, `.h`, `.cpp`, `.cc`, `.cxx`, `.hpp`, `.java`, `.cs` extensions.
- Enhance overlapping error resolution with rollback validation.
- Add integration tests for all new languages.
- Optimize AST traversal performance.

### Non‑Goals (explicitly excluded from v0.8.0)

- Semantic repairs (e.g., missing semicolons, type errors).  
- Additional languages beyond C, C++, Java, C#.  
- Multi‑file operations.  
- Configuration file.

## 9. Operational Concept & High‑Level Scenarios

### Concept of Operations

`patch-ts` now supports twelve languages. The AST‑based traversal engine handles all delimiter types uniformly. Overlapping repairs are applied with cumulative offset tracking and post‑repair validation to prevent corruption.

### High‑Level Scenarios (v0.8.0)

1. **Patch a C file**  
   `patch-ts patch --file main.c --line 10 --old "int x = 1;" --new "int x = 42;"`

2. **Balance a C++ file**  
   `patch-ts balance --file app.cpp --apply` fixes extra `}` or missing `)`.

3. **Patch a Java file**  
   `patch-ts patch --file Main.java --line 5 --old "System.out.println(\"hi\");" --new "System.out.println(\"hello\");"`

4. **Balance a C# file**  
   `patch-ts balance --file Program.cs --apply` fixes unbalanced braces.

5. **Overlapping errors in C**  
   File with missing `)` and extra `}` is repaired correctly in one pass with rollback protection.

## 10. Stakeholders, Sponsorship & Governance

| Role | Name / Org | Responsibility |
|------|------------|----------------|
| **Executive Sponsor** | (Project maintainer) | Approves strategic direction. |
| **Product Owner** | (Project maintainer) | Prioritizes features, manages roadmap. |
| **Engineering Lead** | (Core contributor) | Oversees technical implementation. |

## 11. Risks, Assumptions & Open Questions

### Assumptions

- `tree-sitter-c`, `tree-sitter-cpp`, `tree-sitter-java`, `tree-sitter-c-sharp` grammars are stable.
- AST traversal approach works for all C‑family languages.

### Risks

| Risk | Likelihood | Impact | Mitigation |
|------|------------|--------|------------|
| C++ grammar complexity causes performance issues | Medium | Medium | Benchmark and optimize skip conditions. |
| Java/C# grammars have different node kinds | Low | Low | Map node kinds in traversal. |

### Open Questions

- Should we support Objective‑C? (Deferred to v0.9.0.)

---

# patch-ts Business & Stakeholder Requirements Specification (BRS)

| Field | Value |
|-------|-------|
| Project | patch-ts |
| Document | Business & Stakeholder Requirements Specification |
| Version | 6.0 (v0.8.0) |
| Date | 2026-04-22 |
| Author | patch-ts team, assisted by spec-writer |
| Status | Draft |
| References | Vision v6.0 |

## 1. Business Context

### 1.1 Purpose

This BRS defines business‑level requirements for patch-ts v0.8.0, adding C, C++, Java, and C# support, and enhancing overlapping error resolution.

### 1.2 Business Problem / Opportunity

v0.7.0 supports eight languages but misses the widely used C‑family and enterprise languages. Adding C, C++, Java, and C# completes the "top languages" coverage and makes patch-ts truly universal.

### 1.3 Scope Boundaries

**In scope:**  
- C, C++, Java, C# language support.  
- Enhanced overlapping error resolution with rollback.  
- Performance optimizations.  
- Integration tests for new languages.

**Out of scope:**  
- Other languages.  
- Semantic repairs.  
- Multi‑file operations.

## 2. Business Goals, Objectives & Success Metrics

| ID | Business Goal | Success Metric (Fit Criterion) |
|----|---------------|-------------------------------|
| BR‑001 | Add C support | 100% of CLI commands work on `.c`/`.h` files in test suite. |
| BR‑002 | Add C++ support | 100% of CLI commands work on `.cpp`/`.hpp` files in test suite. |
| BR‑003 | Add Java support | 100% of CLI commands work on `.java` files in test suite. |
| BR‑004 | Add C# support | 100% of CLI commands work on `.cs` files in test suite. |
| BR‑005 | Improve overlapping repair | Multi‑error test suite pass rate ≥99%. |

## 3. Business Model & Processes

patch-ts remains an open‑source CLI tool. Expanded language coverage drives adoption.

## 4. Business Rules & Policies

| ID | Rule | Source |
|----|------|--------|
| BR‑R1 | Language detection based on file extension: `.c`/`.h` → C, `.cpp`/`.cc`/`.cxx`/`.hpp` → C++, `.java` → Java, `.cs` → C#. | Design simplicity. |
| BR‑R2 | Overlapping repairs must not corrupt file content. | Quality requirement. |

## 5. Stakeholders & User Classes

| Stakeholder / User Class | Description | Primary Goals |
|--------------------------|-------------|---------------|
| **C/C++ Developer** | Systems, embedded, performance. | Apply patches safely; fix delimiter errors. |
| **Java Developer** | Enterprise, Android. | Same as above. |
| **C# Developer** | .NET, Unity. | Same as above. |
| **AI Agent** | Generates patches across languages. | Consistent, reliable patching. |

## 6. Glossary / Ubiquitous Language

| Term | Definition |
|------|------------|
| **Rollback validation** | Re‑parsing after each repair to ensure no new errors were introduced; if so, revert the repair. |

## 7. Conceptual Domain Model

**Core entities (extended):**  
- `CLanguage`, `CppLanguage`, `JavaLanguage`, `CSharpLanguage` implement `Language`.

## 8. Stakeholder Needs & User Requirements

| ID | Stakeholder Need | User Class |
|----|------------------|------------|
| SN‑001 | As a C developer, I want to use all patch-ts commands on `.c` files. | C Developer |
| SN‑002 | As a C++ developer, I want the same fuzzy patching and auto‑repair. | C++ Developer |
| SN‑003 | As a Java developer, I want to patch `.java` files. | Java Developer |
| SN‑004 | As a C# developer, I want to patch `.cs` files. | C# Developer |

## 9. System‑in‑Context & Operational Concept

`patch-ts` now supports `.c`, `.h`, `.cpp`, `.cc`, `.cxx`, `.hpp`, `.java`, `.cs`. AST traversal handles all delimiter types. Overlapping repairs use offset tracking and rollback.

## 10. Stakeholder‑Level Constraints & Quality Expectations

| ID | Constraint / Quality Expectation |
|----|----------------------------------|
| C‑001 | New language support must not degrade existing language performance. |
| C‑002 | Rollback must prevent any corruption from overlapping repairs. |

## 11. Risks, Assumptions & Open Issues

### Assumptions
- Grammars are compatible with tree-sitter 0.26.

### Risks
| Risk | Mitigation |
|------|------------|
| C++ grammar complexity | Benchmark; optimize skip conditions. |

### Open Issues
- Objective‑C support? (Deferred.)

## 12. Traceability Mapping to Vision

| Vision Goal | BRS Goal | Stakeholder Need |
|-------------|----------|------------------|
| G‑1 | BR‑001..004 | SN‑001..004 |
| G‑2 | BR‑005 | All |
| P‑1..4 | BR‑001..004 | SN‑001..004 |

---

# patch-ts Software Requirements Specification (SRS)

| Field | Value |
|-------|-------|
| Project | patch-ts |
| Document | Software Requirements Specification |
| Version | 6.0 (v0.8.0) |
| Date | 2026-04-22 |
| Author | patch-ts team, assisted by spec-writer |
| Status | Draft |
| References | BRS v6.0, Vision v6.0 |

## 1. Introduction & Scope

This SRS defines functional and non‑functional requirements for patch-ts v0.8.0, adding C, C++, Java, C# support and enhancing overlapping error resolution.

### 1.1 Scope

- Implement `CLanguage`, `CppLanguage`, `JavaLanguage`, `CSharpLanguage`.  
- Auto‑detect new extensions.  
- Rollback validation for overlapping repairs.  
- Performance optimizations.  
- Integration tests.

### 1.2 Out of Scope

- Other languages.  
- Semantic repairs.  
- Multi‑file.

## 2. System Context & Overview

**Context Diagram (C1):** Unchanged. `Language` trait now has twelve implementors.

## 3. Functional Capabilities & Behavior

### Feature: C Language Support

| ID | Requirement | Priority | Acceptance Criteria |
|----|-------------|----------|---------------------|
| FR‑C‑001 | Provide `CLanguage` implementing `Language`. | Must | Trait methods compile and pass tests. |
| FR‑C‑002 | Use `tree-sitter-c` grammar. | Must | Correct grammar loaded. |
| FR‑C‑003 | Support `.c` and `.h` extensions. | Must | Files use C language. |

### Feature: C++ Language Support

| ID | Requirement | Priority | Acceptance Criteria |
|----|-------------|----------|---------------------|
| FR‑CPP‑001 | Provide `CppLanguage`. | Must | Compiles and passes tests. |
| FR‑CPP‑002 | Use `tree-sitter-cpp`. | Must | Correct grammar. |
| FR‑CPP‑003 | Support `.cpp`, `.cc`, `.cxx`, `.hpp`. | Must | Auto‑detection works. |

### Feature: Java Language Support

| ID | Requirement | Priority | Acceptance Criteria |
|----|-------------|----------|---------------------|
| FR‑JAVA‑001 | Provide `JavaLanguage`. | Must | Compiles and passes tests. |
| FR‑JAVA‑002 | Use `tree-sitter-java`. | Must | Correct grammar. |
| FR‑JAVA‑003 | Support `.java`. | Must | Auto‑detection works. |

### Feature: C# Language Support

| ID | Requirement | Priority | Acceptance Criteria |
|----|-------------|----------|---------------------|
| FR‑CS‑001 | Provide `CSharpLanguage`. | Must | Compiles and passes tests. |
| FR‑CS‑002 | Use `tree-sitter-c-sharp`. | Must | Correct grammar. |
| FR‑CS‑003 | Support `.cs`. | Must | Auto‑detection works. |

### Feature: Enhanced Overlapping Error Resolution

| ID | Requirement | Priority | Acceptance Criteria |
|----|-------------|----------|---------------------|
| FR‑OVER‑005 | After each repair, re‑parse and check error count; rollback if increased. | Must | No silent corruption. |
| FR‑OVER‑006 | Report which errors were fixed and which were rolled back. | Should | JSON includes `rolled_back` list. |

### Feature: Language Detection Update

| ID | Requirement | Priority | Acceptance Criteria |
|----|-------------|----------|---------------------|
| FR‑LANG‑009 | Detect C from `.c`/`.h`. | Must | `detect_language` returns `CLanguage`. |
| FR‑LANG‑010 | Detect C++ from `.cpp`/`.cc`/`.cxx`/`.hpp`. | Must | Returns `CppLanguage`. |
| FR‑LANG‑011 | Detect Java from `.java`. | Must | Returns `JavaLanguage`. |
| FR‑LANG‑012 | Detect C# from `.cs`. | Must | Returns `CSharpLanguage`. |

## 4. Quality & Non‑Functional Requirements

| ID | Category | Requirement | Fit Criterion |
|----|----------|-------------|---------------|
| NFR‑PERF‑001 | Performance | Balance on 10k‑line file ≤ 200 ms (p95) across all languages. | Benchmark suite. |
| NFR‑ACC‑001 | Accuracy | Overlapping error test suite pass rate ≥99%. | Automated test pass rate. |
| NFR‑COMPAT‑001 | Compatibility | All v0.7.0 tests pass without modification. | CI regression suite. |

## 5. External Interfaces & Data Contracts

**CLI Interface:** Unchanged.

**JSON Output Schema:** Add `rolled_back` array to `BalanceResult`.

## 6. Constraints, Assumptions & Dependencies

| Type | Description |
|------|-------------|
| Constraint | Add `tree-sitter-c`, `tree-sitter-cpp`, `tree-sitter-java`, `tree-sitter-c-sharp`. |
| Assumption | Grammars expose standard delimiter nodes. |
| Dependency | `tree-sitter` 0.26. |

## 7. TBD Log

| ID | Item | Owner | Due |
|----|------|-------|-----|
| TBD‑001 | Verify C++ grammar delimiter node kinds. | Engineering | Before implementation. |

## 8. Requirements Attributes & Traceability Model

**ID Scheme:**  
- Functional: `FR‑C‑###`, `FR‑CPP‑###`, `FR‑JAVA‑###`, `FR‑CS‑###`, `FR‑OVER‑###`, `FR‑LANG‑###`  
- NFR: `NFR‑CAT‑###`

**Traceability Matrix (excerpt):**

| SRS ID | BRS ID | Verification Method |
|--------|--------|---------------------|
| FR‑C‑001 | BR‑001 | Test |
| FR‑CPP‑001 | BR‑002 | Test |
| FR‑JAVA‑001 | BR‑003 | Test |
| FR‑CS‑001 | BR‑004 | Test |
| FR‑OVER‑005 | BR‑005 | Test |

---

# patch-ts Architecture & Design Specification

| Field | Value |
|-------|-------|
| Project | patch-ts |
| Document | Architecture & Design Specification |
| Version | 6.0 (v0.8.0) |
| Date | 2026-04-22 |
| Author | patch-ts team, assisted by spec-writer |
| Status | Draft |
| References | SRS v6.0, BRS v6.0 |

## 1. Context & Scope

This document describes architectural changes for v0.8.0: adding C, C++, Java, C# support, and enhancing overlapping error resolution with rollback.

## 2. Goals & Non‑Goals

### Goals

- Add `CLanguage`, `CppLanguage`, `JavaLanguage`, `CSharpLanguage`.  
- Implement rollback validation in `balance_file`.  
- Optimize AST traversal.

### Non‑Goals

- Refactor `Language` trait.  
- Add language‑specific semantic repairs.

## 3. Architecturally Significant Requirements (ASRs)

| ASR ID | Description | Source |
|--------|-------------|--------|
| ASR‑001 | Rollback must prevent corruption from overlapping repairs. | FR‑OVER‑005 |
| ASR‑002 | New languages must not regress performance. | NFR‑PERF‑001 |
| ASR‑003 | Backward compatibility with v0.7.0 must be maintained. | NFR‑COMPAT‑001 |

## 4. The Design

### 4.1 System Overview (C4 Level 2)

```
[User/Agent] → (CLI) → [patch-ts Binary]
                           ├── cli.rs (language detection)
                           ├── ast.rs (Language trait + 12 implementors)
                           ├── repair.rs (AST‑based repair + rollback)
                           └── file.rs (atomic write)
                                     ↓
              ┌──────────────────┼──────────────────┬──────────────────┬──────────────────┐
              ↓                  ↓                  ↓                  ↓                  ↓
     [tree-sitter-*]   [tree-sitter-c] [tree-sitter-cpp] [tree-sitter-java] [tree-sitter-c-sharp]
```

### 4.2 Key Design Changes

**4.2.1 Language Implementations**

Use the existing macro to generate the four new languages:

```rust
impl_language!(CLanguage, tree_sitter_c::LANGUAGE);
impl_language!(CppLanguage, tree_sitter_cpp::LANGUAGE);
impl_language!(JavaLanguage, tree_sitter_java::LANGUAGE);
impl_language!(CSharpLanguage, tree_sitter_c_sharp::LANGUAGE);
```

**4.2.2 Rollback Validation**

In `balance_file`, before applying a repair, store the current content. After applying, re‑parse. If `has_error_node` count increases or new error kinds appear, revert to the stored content and skip that repair.

```rust
let before = current_content.clone();
current_content = apply_repair(&current_content, &adjusted_error, index);
let after_parse = language.parse(&current_content);
if error_count_increased(&before_parse, &after_parse) {
    current_content = before; // rollback
    rolled_back.push(error.clone());
    continue;
}
```

**4.2.3 Performance Optimizations**

- Cache `LineIndex` for the original content.  
- Use `node.kind()` string comparisons only when necessary.

### 4.3 Data Model

Extend `BalanceResult` with `rolled_back: Vec<DelimiterError>`.

### 4.4 Security Architecture

No changes.

## 5. Architecture Decision Records (ADRs)

### ADR‑013: Rollback on repair corruption

**Context:** Offset tracking can still fail; we need a safety net.

**Decision:** Validate after each repair; rollback if new errors appear.

**Alternatives:** None (must prevent corruption).

**Consequences:** Guarantees file integrity; may leave some errors unfixed (reported).

### ADR‑014: Use macro for all Language impls

**Context:** Adding four more languages would duplicate boilerplate.

**Decision:** Continue using the `impl_language!` macro for all new languages.

**Alternatives:** Manual impl for each. Rejected as verbose.

**Consequences:** Clean, maintainable code; all languages behave consistently.

## 6. API & Interface Contracts

JSON output extended with `rolled_back` field.

## 7. Cross‑cutting Concerns

| Concern | Approach |
|---------|----------|
| **Testing** | New integration tests for C, C++, Java, C#. |
| **Performance** | Benchmark on new languages; optimize skip conditions. |

## 8. Alternatives Considered

| Alternative | Why Rejected |
|-------------|--------------|
| Separate repair logic per language | Code duplication. |

## 9. Traceability

| ASR | ADR | Component |
|-----|-----|-----------|
| ASR‑001 | ADR‑013 | repair.rs |
| ASR‑002 | ADR‑014 | ast.rs |
| ASR‑003 | – | All |

---

# patch-ts Behavioral Specification & Test Verification Plan

| Field | Value |
|-------|-------|
| Project | patch-ts |
| Document | Behavioral Specification & Test Verification Plan |
| Version | 6.0 (v0.8.0) |
| Date | 2026-04-22 |
| Author | patch-ts team, assisted by spec-writer |
| Status | Draft |
| References | SRS v6.0, Architecture v6.0 |

## 1. Behavioral Specifications (Specification by Example)

### Feature: C Support

```gherkin
Feature: C language support
  Scenario: Patch a C file exactly
    Given a file "main.c" with content:
      """
      #include <stdio.h>
      int main() {
          printf("hello");
          return 0;
      }
      """
    When I run `patch-ts patch --file main.c --line 3 --old "    printf(\"hello\");" --new "    printf(\"world\");"`
    Then the file content contains "printf(\"world\");"
```

### Feature: C++ Support

```gherkin
Feature: C++ language support
  Scenario: Balance a C++ file with extra brace
    Given a file "app.cpp" with content:
      """
      #include <iostream>
      int main() {
          std::cout << "hi";
      }
      }
      """
    When I run `patch-ts balance --file app.cpp --apply`
    Then the extra '}' is removed.
```

### Feature: Java Support

```gherkin
Feature: Java language support
  Scenario: Patch a Java file
    Given a file "Main.java" with content:
      """
      public class Main {
          public static void main(String[] args) {
              System.out.println("hello");
          }
      }
      """
    When I run `patch-ts patch --file Main.java --line 3 --old "        System.out.println(\"hello\");" --new "        System.out.println(\"world\");"`
    Then the file contains "world".
```

### Feature: C# Support

```gherkin
Feature: C# language support
  Scenario: Balance a C# file with missing brace
    Given a file "Program.cs" with content:
      """
      class Program {
          static void Main() {
              Console.WriteLine("hi");
      """
    When I run `patch-ts balance --file Program.cs --apply`
    Then the missing '}' is inserted.
```

### Feature: Rollback Validation

```gherkin
Feature: Rollback on corruption
  Scenario: Overlapping repair that would corrupt is rolled back
    Given a file with complex overlapping errors
    When I run `patch-ts balance --apply --json`
    Then the JSON output includes a `rolled_back` array with the skipped error.
    And the file is not corrupted.
```

## 2. Test Strategy & Plan

### 2.1 Test Pyramid

| Level | Scope | Tools |
|-------|-------|-------|
| Unit | Rollback logic, language detection | Rust `#[test]` |
| Integration | Full commands on new languages | `assert_cmd` |
| Regression | v0.7.0 test suite | Cargo test |
| Performance | Benchmark all languages | Criterion |

### 2.2 Risk‑Based Prioritization

| Risk | Test Focus |
|------|------------|
| Rollback fails to prevent corruption | Extensive overlapping error corpus. |
| C++ grammar causes performance issues | Benchmark on large C++ files. |

## 3. Test Case Specifications

### TC‑C‑001: C patch exact

- **Requirement:** FR‑C‑001
- **Preconditions:** `.c` file.
- **Steps:** Run `patch --line X --old ... --new ...`.
- **Expected:** Patch applied.
- **Automated:** Yes.

### TC‑CPP‑001: C++ balance

- **Requirement:** FR‑CPP‑001
- **Preconditions:** `.cpp` file with extra `}`.
- **Steps:** Run `balance --apply`.
- **Expected:** Extra `}` removed.
- **Automated:** Yes.

### TC‑JAVA‑001: Java patch

- **Requirement:** FR‑JAVA‑001
- **Preconditions:** `.java` file.
- **Steps:** Run `patch`.
- **Expected:** Patch applied.
- **Automated:** Yes.

### TC‑CS‑001: C# balance

- **Requirement:** FR‑CS‑001
- **Preconditions:** `.cs` file missing `}`.
- **Steps:** Run `balance --apply`.
- **Expected:** `}` inserted.
- **Automated:** Yes.

### TC‑OVER‑001: Rollback validation

- **Requirement:** FR‑OVER‑005
- **Preconditions:** File with overlapping errors.
- **Steps:** Run `balance --apply --json`.
- **Expected:** `rolled_back` present if corruption avoided.
- **Automated:** Yes.

## 4. NFR Verification Plans

### NFR‑PERF‑001 (Performance)

- **Method:** Criterion benchmark on 10k‑line C/C++/Java/C# files.
- **Threshold:** p95 ≤ 200 ms.

### NFR‑ACC‑001 (Accuracy)

- **Method:** Overlapping error test suite.
- **Threshold:** ≥99% pass.

## 5. Requirements Traceability Matrix (RTM)

| SRS ID | Test Case(s) | Verification Method |
|--------|--------------|---------------------|
| FR‑C‑001 | TC‑C‑001 | Test |
| FR‑CPP‑001 | TC‑CPP‑001 | Test |
| FR‑JAVA‑001 | TC‑JAVA‑001 | Test |
| FR‑CS‑001 | TC‑CS‑001 | Test |
| FR‑OVER‑005 | TC‑OVER‑001 | Test |

---

This completes the specification suite for patch-ts v0.8.0.

Now I'll create the detailed implementation plan with chunked tasks and safe script workflow.

---

# patch-ts v0.8.0 Implementation Plan

> **For agentic workers:** REQUIRED: Use superpowers:subagent-driven-development (if subagents available) or superpowers:executing-plans to implement this plan. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add C, C++, Java, and C# language support, implement rollback validation for overlapping repairs, and optimize performance.

**Architecture:** Extend language detection to new extensions, implement four new languages via macro, add rollback logic to `balance_file`, and add integration tests.

**Tech Stack:** Rust, tree-sitter 0.26, tree-sitter-c, tree-sitter-cpp, tree-sitter-java, tree-sitter-c-sharp, anyhow, clap.
