We'll create the complete specification suite and implementation plan for **patch-ts v0.7.0**. The documents follow the same layered structure: Vision, BRS, SRS, Architecture, and Test Verification, then the actionable plan with bash scripts.

---

# patch-ts v0.7.0 — Specification Documents

## Document Set Overview

| Document | Purpose |
|----------|---------|
| [Vision & Strategic Alignment](#patch-ts-vision--strategic-alignment) | Long‑term direction and success criteria for robust multi‑language repair. |
| [Business & Stakeholder Requirements (BRS)](#patch-ts-business--stakeholder-requirements-specification-brs) | Business goals, stakeholders, and high‑level needs for v0.7.0. |
| [Software Requirements Specification (SRS)](#patch-ts-software-requirements-specification-srs) | Functional and non‑functional requirements for v0.7.0. |
| [Architecture & Design Specification](#patch-ts-architecture--design-specification) | Architectural decisions, AST‑based repair engine, and ADRs. |
| [Behavioral Specification & Test Verification Plan](#patch-ts-behavioral-specification--test-verification-plan) | Acceptance criteria, test strategy, and traceability for v0.7.0 features. |

---

# patch-ts Vision & Strategic Alignment

| Field | Value |
|-------|-------|
| Project | patch-ts |
| Document | Vision & Strategic Alignment |
| Version | 5.0 (v0.7.0) |
| Date | 2026-04-22 |
| Author | patch-ts team, assisted by spec-writer |
| Status | Draft |

## 1. Vision Statement

> *patch-ts becomes the definitive, intelligent patching tool for all major programming languages—delivering perfect delimiter repair through AST‑aware heuristics, supporting Ruby, PHP, and HTML/XML, and handling overlapping errors flawlessly.*

## 2. Elevator Pitch (Moore Template)

> For **AI coding agents and developers working across Rust, TypeScript, JavaScript, Python, Go, Ruby, PHP, and HTML/XML** who need a single, reliable patching tool that never corrupts overlapping fixes, **patch-ts** is a **tree‑sitter‑backed CLI** that provides **AST‑based delimiter repair, batch error resolution, and marker‑based targeting across eight languages**. Unlike fragmented toolchains, our product **auto‑detects language** and **applies language‑specific AST heuristics**, making it the universal patching utility for every codebase.

## 3. Problem Statement & Business Context

**Problem:** v0.6.0 added Python and Go support, but Python balance commands remain unreliable due to string literal edge cases. Overlapping error resolution works but needs refinement. Ruby and PHP developers are still excluded. HTML/XML tag mismatch is a common pain point for web developers.

**Why now:**  
- Ruby and PHP are top‑15 languages; supporting them expands the addressable user base significantly.  
- AST‑based repair solves Python string issues and provides a foundation for all languages.  
- Tag repair opens patch-ts to frontend and templating workflows.

**Business drivers:**  
- Expand user base to Ruby, PHP, and web developers.  
- Achieve >99% patch success rate across all supported languages.  
- Position patch-ts as the definitive universal patching utility.

## 4. Target Users / Customers

| Segment | Description |
|---------|-------------|
| **Ruby developers** | Rails, Jekyll, and backend developers. |
| **PHP developers** | WordPress, Laravel, and legacy web developers. |
| **Frontend/Web developers** | Working with HTML/XML templates. |
| **AI coding agents** | Require consistent patching across all major languages. |
| **CI/CD pipelines** | Benefit from flawless batch repair in automated workflows. |

**Explicitly NOT targeting (v0.7.0):**  
- Additional languages (C/C++, Java, C#) — deferred to v0.8.0+.  
- Semantic repairs beyond delimiters and tags.

## 5. User Needs & Value Proposition

| Need | patch-ts v0.7.0 Value |
|------|------------------------|
| "I need Python balance to work reliably." | AST‑based traversal correctly skips string literals and comments. |
| "I need to fix unbalanced delimiters in Ruby." | Ruby support via `tree-sitter-ruby`. |
| "I need to fix unbalanced delimiters in PHP." | PHP support via `tree-sitter-php`. |
| "I need to fix mismatched HTML/XML tags." | Tag repair using tree‑sitter‑html and tree‑sitter‑xml grammars. |
| "I need overlapping errors to never corrupt my file." | Offset tracking with re‑validation after each repair. |

**Differentiator:** patch-ts is the only CLI patching tool supporting eight languages with AST‑aware repair, batch resolution, and tag balancing.

## 6. Desired Outcomes & Success Metrics

### Business Outcomes (v0.7.0)

| ID | Outcome | Key Result / Target |
|----|---------|---------------------|
| G‑1 | Expand language coverage | Ruby and PHP support fully integrated; all existing commands work. |
| G‑2 | Achieve perfect Python balance | 100% of Python balance test corpus passes. |
| G‑3 | Add HTML/XML tag repair | `balance` command fixes mismatched tags in test suite. |
| G‑4 | Improve overlapping error reliability | >99% of multi‑error test cases repaired correctly in one pass. |

### Product Outcomes (v0.7.0)

| ID | Outcome | Metric |
|----|---------|--------|
| P‑1 | Users can patch Ruby files | CLI accepts `.rb` files with auto‑detection. |
| P‑2 | Users can patch PHP files | CLI accepts `.php` files with auto‑detection. |
| P‑3 | Users can balance HTML/XML tags | `balance` fixes `<div>` → `</div>` mismatches. |
| P‑4 | Python balance works reliably | All previously disabled Python balance tests pass. |

## 7. Strategic Constraints

| Constraint | Description |
|------------|-------------|
| **Backward compatibility** | v0.7.0 CLI must accept all v0.6.0 flags and produce equivalent behavior. |
| **Dependency footprint** | Add `tree-sitter-ruby`, `tree-sitter-php`, `tree-sitter-html`, `tree-sitter-xml`. |
| **Performance** | Balance on 10k‑line files ≤ 200 ms (p95) across all languages. |

## 8. Goals and Non‑Goals (v0.7.0)

### Goals

- Implement `RubyLanguage` and `PHPLanguage` structs.
- Implement `HtmlLanguage` and `XmlLanguage` with tag‑aware repair.
- Auto‑detect `.rb`, `.php`, `.html`, `.htm`, `.xml` extensions.
- Refine AST traversal to correctly handle all string/comment types across languages.
- Achieve 100% pass rate on Python balance tests.
- Enhance offset tracking with re‑validation to prevent corruption.

### Non‑Goals (explicitly excluded from v0.7.0)

- Semantic repairs (e.g., indentation, type annotations).
- Additional languages beyond Ruby, PHP, HTML, XML.
- Configuration file.

## 9. Operational Concept & High‑Level Scenarios

### Concept of Operations

`patch-ts balance` now uses language‑specific AST traversal to accurately detect delimiter and tag errors. Overlapping repairs are applied with cumulative offset tracking and post‑repair re‑validation. Ruby, PHP, HTML, and XML files are auto‑detected and processed with the same reliability as existing languages.

### High‑Level Scenarios (v0.7.0)

1. **Fix Python missing parenthesis reliably**  
   `patch-ts balance --file script.py --apply` correctly inserts `)` after `print("hello"`.

2. **Balance a Ruby file**  
   `patch-ts balance --file app.rb --apply` fixes extra `end` or missing `}`.

3. **Balance a PHP file**  
   `patch-ts balance --file index.php --apply` fixes unbalanced braces in mixed HTML/PHP.

4. **Fix mismatched HTML tags**  
   `patch-ts balance --file page.html --apply` changes `</div>` to `</span>` or inserts missing closing tag.

5. **Overlapping errors in Python**  
   File with missing `)` and extra `}` is repaired correctly in one pass.

## 10. Stakeholders, Sponsorship & Governance

| Role | Name / Org | Responsibility |
|------|------------|----------------|
| **Executive Sponsor** | (Project maintainer) | Approves strategic direction. |
| **Product Owner** | (Project maintainer) | Prioritizes features, manages roadmap. |
| **Engineering Lead** | (Core contributor) | Oversees technical implementation. |

## 11. Risks, Assumptions & Open Questions

### Assumptions

- `tree-sitter-ruby`, `tree-sitter-php`, `tree-sitter-html`, `tree-sitter-xml` grammars are stable.
- AST traversal approach generalizes to all delimiter and tag nodes.

### Risks

| Risk | Likelihood | Impact | Mitigation |
|------|------------|--------|------------|
| Tag repair heuristics are complex | Medium | High | Start with simple mismatch detection; iterate. |
| Performance degradation with eight grammars | Low | Medium | Benchmark and optimize loading. |

### Open Questions

- Should tag repair support self‑closing tags? (Yes, treat as balanced.)
- How to handle PHP mixed‑mode (HTML + PHP)? (Parse with PHP grammar; tags inside PHP strings ignored.)

---

# patch-ts Business & Stakeholder Requirements Specification (BRS)

| Field | Value |
|-------|-------|
| Project | patch-ts |
| Document | Business & Stakeholder Requirements Specification |
| Version | 5.0 (v0.7.0) |
| Date | 2026-04-22 |
| Author | patch-ts team, assisted by spec-writer |
| Status | Draft |
| References | Vision v5.0 |

## 1. Business Context

### 1.1 Purpose

This BRS defines business‑level requirements for patch-ts v0.7.0, adding Ruby, PHP, HTML, and XML support, refining AST‑based repair for Python, and enhancing overlapping error resolution.

### 1.2 Business Problem / Opportunity

v0.6.0 added Python and Go but left Python balance unreliable. Ruby and PHP developers lack a dedicated patching tool. HTML/XML tag mismatches are a common source of web development errors that AI agents struggle to fix.

### 1.3 Scope Boundaries

**In scope:**  
- Ruby, PHP, HTML, XML language support.  
- AST‑based repair refinements for all languages.  
- Tag balancing for HTML/XML.  
- Enhanced overlapping error resolution.  
- Integration tests for new languages.

**Out of scope:**  
- Other languages.  
- Semantic repairs.  
- Multi‑file operations.

## 2. Business Goals, Objectives & Success Metrics

| ID | Business Goal | Success Metric (Fit Criterion) |
|----|---------------|-------------------------------|
| BR‑001 | Add Ruby support | 100% of CLI commands work on `.rb` files in test suite. |
| BR‑002 | Add PHP support | 100% of CLI commands work on `.php` files in test suite. |
| BR‑003 | Add HTML/XML tag repair | `balance` fixes tag mismatches in test corpus with ≥95% accuracy. |
| BR‑004 | Fix Python balance | 100% of Python balance test corpus passes. |
| BR‑005 | Improve overlapping repair | Multi‑error test suite pass rate ≥99%. |

## 3. Business Model & Processes

patch-ts remains an open‑source CLI tool. Expanded language coverage drives adoption and community contributions.

## 4. Business Rules & Policies

| ID | Rule | Source |
|----|------|--------|
| BR‑R1 | Language detection based on file extension: `.rb` → Ruby, `.php` → PHP, `.html`/`.htm` → HTML, `.xml` → XML. | Design simplicity. |
| BR‑R2 | Tag repair must not alter non‑tag content. | Quality requirement. |

## 5. Stakeholders & User Classes

| Stakeholder / User Class | Description | Primary Goals |
|--------------------------|-------------|---------------|
| **Ruby Developer** | Rails, backend. | Apply patches safely; fix delimiter errors. |
| **PHP Developer** | WordPress, Laravel. | Same as above. |
| **Web Developer** | HTML/XML templates. | Fix tag mismatches automatically. |
| **AI Agent** | Generates patches across languages. | Consistent, reliable patching. |

## 6. Glossary / Ubiquitous Language

| Term | Definition |
|------|------------|
| **Tag balancing** | Ensuring opening and closing HTML/XML tags match in name and nesting. |
| **AST‑based repair** | Using the concrete syntax tree to detect errors while ignoring non‑code regions. |

## 7. Conceptual Domain Model

**Core entities (extended):**  
- `TagError` enum for mismatched/missing tags.  
- `RepairAction` now includes `InsertTag` and `RemoveTag`.

## 8. Stakeholder Needs & User Requirements

| ID | Stakeholder Need | User Class |
|----|------------------|------------|
| SN‑001 | As a Ruby developer, I want to use all patch-ts commands on `.rb` files. | Ruby Developer |
| SN‑002 | As a PHP developer, I want the same fuzzy patching and auto‑repair. | PHP Developer |
| SN‑003 | As a web developer, I want to fix unbalanced HTML tags automatically. | Web Developer |
| SN‑004 | As a Python developer, I want `balance` to work reliably. | Python Developer |

## 9. System‑in‑Context & Operational Concept

`patch-ts` now supports `.rb`, `.php`, `.html`, `.htm`, `.xml`. AST traversal accurately detects delimiter and tag errors. Overlapping repairs are applied with offset tracking and validation.

## 10. Stakeholder‑Level Constraints & Quality Expectations

| ID | Constraint / Quality Expectation |
|----|----------------------------------|
| C‑001 | New language support must not degrade existing language performance. |
| C‑002 | Tag repair must not introduce malformed HTML/XML. |

## 11. Risks, Assumptions & Open Issues

### Assumptions
- `tree-sitter-html` and `tree-sitter-xml` provide reliable tag node detection.

### Risks
| Risk | Mitigation |
|------|------------|
| Tag repair heuristics are complex | Start with simple mismatch detection; iterate. |

### Open Issues
- How to handle self‑closing tags? (Treat as balanced.)

## 12. Traceability Mapping to Vision

| Vision Goal | BRS Goal | Stakeholder Need |
|-------------|----------|------------------|
| G‑1 | BR‑001, BR‑002 | SN‑001, SN‑002 |
| G‑2 | BR‑004 | SN‑004 |
| G‑3 | BR‑003 | SN‑003 |
| G‑4 | BR‑005 | All |

---

# patch-ts Software Requirements Specification (SRS)

| Field | Value |
|-------|-------|
| Project | patch-ts |
| Document | Software Requirements Specification |
| Version | 5.0 (v0.7.0) |
| Date | 2026-04-22 |
| Author | patch-ts team, assisted by spec-writer |
| Status | Draft |
| References | BRS v5.0, Vision v5.0 |

## 1. Introduction & Scope

This SRS defines functional and non‑functional requirements for patch-ts v0.7.0, adding Ruby, PHP, HTML, XML support, refining AST repair, and enhancing overlapping error resolution.

### 1.1 Scope

- Implement `RubyLanguage`, `PHPLanguage`, `HtmlLanguage`, `XmlLanguage`.
- Auto‑detect `.rb`, `.php`, `.html`, `.htm`, `.xml` extensions.
- Implement tag‑aware repair for HTML/XML.
- Refine AST traversal for all languages.
- Enhance overlapping error resolution with re‑validation.
- Integration tests for new languages.

### 1.2 Out of Scope

- Other languages.
- Semantic repairs.
- Configuration file.

## 2. System Context & Overview

**Context Diagram (C1):** Unchanged. `Language` trait now has eight implementors.

## 3. Functional Capabilities & Behavior

### Feature: Ruby Language Support

| ID | Requirement (EARS pattern) | Priority | Acceptance Criteria |
|----|----------------------------|----------|---------------------|
| FR‑RB‑001 | **The system shall** provide a `RubyLanguage` struct implementing `Language`. | Must | Trait methods compile and pass tests. |
| FR‑RB‑002 | **When** parsing Ruby source, **the system shall** use `tree-sitter-ruby`. | Must | Correct grammar loaded. |
| FR‑RB‑003 | **The system shall** support `.rb` extension. | Must | Files with `.rb` use Ruby language. |

### Feature: PHP Language Support

| ID | Requirement | Priority | Acceptance Criteria |
|----|-------------|----------|---------------------|
| FR‑PHP‑001 | **The system shall** provide a `PHPLanguage` struct implementing `Language`. | Must | Trait methods compile and pass tests. |
| FR‑PHP‑002 | **When** parsing PHP source, **the system shall** use `tree-sitter-php`. | Must | Correct grammar loaded. |
| FR‑PHP‑003 | **The system shall** support `.php` extension. | Must | Files with `.php` use PHP language. |

### Feature: HTML/XML Tag Repair

| ID | Requirement | Priority | Acceptance Criteria |
|----|-------------|----------|---------------------|
| FR‑HTML‑001 | **The system shall** provide `HtmlLanguage` and `XmlLanguage` structs. | Must | Trait methods compile. |
| FR‑HTML‑002 | **The system shall** detect mismatched opening/closing tags using AST traversal. | Must | Tag errors identified correctly. |
| FR‑HTML‑003 | **The system shall** repair mismatched tags by renaming the closing tag to match the opening tag. | Must | `<div>...</span>` → `<div>...</div>`. |
| FR‑HTML‑004 | **The system shall** insert missing closing tags at the appropriate location. | Should | Unclosed `<div>` gets `</div>`. |

### Feature: AST Traversal Refinements

| ID | Requirement | Priority | Acceptance Criteria |
|----|-------------|----------|---------------------|
| FR‑AST‑001 | **The system shall** correctly skip all string literal and comment node types for each language. | Must | Python balance tests pass. |
| FR‑AST‑002 | **The system shall** use language‑specific node kind constants for delimiter detection. | Must | Works across all eight languages. |

### Feature: Enhanced Overlapping Error Resolution

| ID | Requirement | Priority | Acceptance Criteria |
|----|-------------|----------|---------------------|
| FR‑OVER‑003 | **The system shall** re‑validate the AST after each repair and rollback if corruption is detected. | Must | No silent corruption. |
| FR‑OVER‑004 | **The system shall** report which errors were successfully fixed and which remain. | Should | JSON output includes `fixed` and `unresolved` lists. |

### Feature: Language Detection Update

| ID | Requirement | Priority | Acceptance Criteria |
|----|-------------|----------|---------------------|
| FR‑LANG‑006 | **The system shall** detect Ruby from `.rb` extension. | Must | `detect_language` returns `RubyLanguage`. |
| FR‑LANG‑007 | **The system shall** detect PHP from `.php` extension. | Must | `detect_language` returns `PHPLanguage`. |
| FR‑LANG‑008 | **The system shall** detect HTML from `.html`/`.htm` and XML from `.xml`. | Must | Correct language selected. |

## 4. Quality & Non‑Functional Requirements

| ID | Category | Requirement | Fit Criterion |
|----|----------|-------------|---------------|
| NFR‑PERF‑001 | Performance efficiency | Balance on 10k‑line file ≤ 200 ms (p95) across all languages. | Benchmark suite. |
| NFR‑ACC‑001 | Accuracy | Python balance test corpus passes 100%. | Automated test pass rate. |
| NFR‑ACC‑002 | Accuracy | Tag repair correct for ≥95% of test corpus. | Automated test pass rate. |
| NFR‑COMPAT‑001 | Compatibility | All v0.6.0 tests pass without modification. | CI regression suite. |

## 5. External Interfaces & Data Contracts

**CLI Interface:** Unchanged.

**JSON Output Schema (enhanced):** Add `fixed` and `unresolved` arrays to `BalanceResult`.

## 6. Constraints, Assumptions & Dependencies

| Type | Description |
|------|-------------|
| Constraint | Add `tree-sitter-ruby`, `tree-sitter-php`, `tree-sitter-html`, `tree-sitter-xml`. |
| Assumption | Grammars expose standard delimiter and tag nodes. |
| Dependency | `tree-sitter` 0.26. |

## 7. TBD Log

| ID | Item | Owner | Due |
|----|------|-------|-----|
| TBD‑001 | Determine exact tag repair heuristics for nested mismatches. | Engineering | Before implementation. |

## 8. Requirements Attributes & Traceability Model

**ID Scheme:**  
- Functional: `FR‑RB‑###`, `FR‑PHP‑###`, `FR‑HTML‑###`, `FR‑AST‑###`, `FR‑OVER‑###`, `FR‑LANG‑###`  
- NFR: `NFR‑CAT‑###`

**Traceability Matrix (excerpt):**

| SRS ID | BRS ID | Verification Method |
|--------|--------|---------------------|
| FR‑RB‑001 | BR‑001 | Test |
| FR‑PHP‑001 | BR‑002 | Test |
| FR‑HTML‑002 | BR‑003 | Test |
| FR‑AST‑001 | BR‑004 | Test |
| FR‑OVER‑003 | BR‑005 | Test |

---

# patch-ts Architecture & Design Specification

| Field | Value |
|-------|-------|
| Project | patch-ts |
| Document | Architecture & Design Specification |
| Version | 5.0 (v0.7.0) |
| Date | 2026-04-22 |
| Author | patch-ts team, assisted by spec-writer |
| Status | Draft |
| References | SRS v5.0, BRS v5.0 |

## 1. Context & Scope

This document describes architectural changes for v0.7.0: adding Ruby, PHP, HTML, XML support, refining AST traversal, and enhancing overlapping error resolution.

## 2. Goals & Non‑Goals

### Goals

- Add `RubyLanguage`, `PHPLanguage`, `HtmlLanguage`, `XmlLanguage` implementors.
- Implement tag‑aware repair for HTML/XML.
- Refine AST traversal to correctly skip all string/comment nodes.
- Add rollback mechanism for corrupted overlapping repairs.

### Non‑Goals

- Refactor `Language` trait.
- Add language‑specific semantic repairs.

## 3. Architecturally Significant Requirements (ASRs)

| ASR ID | Description | Source |
|--------|-------------|--------|
| ASR‑001 | AST traversal must correctly handle all language‑specific string/comment nodes. | FR‑AST‑001 |
| ASR‑002 | Tag repair must not corrupt non‑tag content. | FR‑HTML‑002 |
| ASR‑003 | Overlapping repair must rollback on corruption. | FR‑OVER‑003 |

## 4. The Design

### 4.1 System Overview (C4 Level 2)

```
[User/Agent] → (CLI) → [patch-ts Binary]
                           ├── cli.rs (language detection)
                           ├── ast.rs (Language trait + 8 implementors)
                           ├── repair.rs (AST‑based repair + offset tracking)
                           └── file.rs (atomic write)
                                     ↓
              ┌──────────────────┼──────────────────┬──────────────────┬──────────────────┐
              ↓                  ↓                  ↓                  ↓                  ↓
     [tree-sitter-*]   [tree-sitter-ruby] [tree-sitter-php] [tree-sitter-html] [tree-sitter-xml]
```

### 4.2 Key Design Changes

**4.2.1 AST Traversal Refinements**

Define language‑specific sets of node kinds to skip:

```rust
fn should_skip_node(kind: &str, lang: LanguageId) -> bool {
    match lang {
        LanguageId::Rust => kind.contains("string") || kind.contains("comment"),
        LanguageId::Python => kind == "string" || kind == "comment",
        LanguageId::Ruby => kind == "string" || kind == "comment",
        LanguageId::Php => kind.contains("string") || kind.contains("comment"),
        LanguageId::Html | LanguageId::Xml => kind == "quoted_attribute_value" || kind == "comment",
        _ => kind.contains("string") || kind.contains("comment"),
    }
}
```

**4.2.2 Tag Repair for HTML/XML**

Add `TagError` enum and detection logic:

```rust
enum TagError {
    Mismatched { open: String, close: String, open_span: Span, close_span: Span },
    Missing { tag_name: String, insert_at: Span },
}

fn find_tag_errors(root: Node, index: &LineIndex) -> Vec<TagError> {
    // Traverse, push tag names onto stack, pop on closing tags, detect mismatches.
}
```

**4.2.3 Overlapping Repair Rollback**

In `balance_file`, before applying repairs, store original content. After each repair, re‑parse; if `has_error_node` increases, rollback and skip that repair.

### 4.3 Data Model

Extend `DelimiterError` with `TagError` (or create a parallel enum). `RepairAction` gains `FixTag` variant.

### 4.4 Security Architecture

No changes.

## 5. Architecture Decision Records (ADRs)

### ADR‑011: Use AST traversal for all delimiter detection

**Context:** Character scanner fails on complex string literals.

**Decision:** Replace scanner with AST traversal for all languages, skipping string/comment nodes.

**Alternatives:** Enhance scanner per language. Rejected as unsustainable.

**Consequences:** Accurate detection; slight performance overhead (acceptable).

### ADR‑012: Rollback on overlapping repair corruption

**Context:** Offset tracking can still fail in edge cases.

**Decision:** Validate after each repair; rollback if new errors introduced.

**Alternatives:** None (must prevent corruption).

**Consequences:** Guarantees file integrity; may leave some errors unfixed (reported).

## 6. API & Interface Contracts

JSON output extended with `fixed` and `unresolved` fields.

## 7. Cross‑cutting Concerns

| Concern | Approach |
|---------|----------|
| **Testing** | New integration tests for Ruby, PHP, HTML, XML; Python balance tests uncommented. |
| **Performance** | Benchmark on all languages; optimize skip condition. |

## 8. Alternatives Considered

| Alternative | Why Rejected |
|-------------|--------------|
| Keep character scanner | Fails on Python edge cases. |
| Separate repair logic per language | Code duplication. |

## 9. Traceability

| ASR | ADR | Component |
|-----|-----|-----------|
| ASR‑001 | ADR‑011 | ast.rs |
| ASR‑002 | ADR‑012 | repair.rs |
| ASR‑003 | ADR‑012 | repair.rs |

---

# patch-ts Behavioral Specification & Test Verification Plan

| Field | Value |
|-------|-------|
| Project | patch-ts |
| Document | Behavioral Specification & Test Verification Plan |
| Version | 5.0 (v0.7.0) |
| Date | 2026-04-22 |
| Author | patch-ts team, assisted by spec-writer |
| Status | Draft |
| References | SRS v5.0, Architecture v5.0 |

## 1. Behavioral Specifications (Specification by Example)

### Feature: Python Balance Reliability

#### Scenario: Python balance works with f‑strings and triple quotes

```gherkin
Feature: Python balance reliability
  Scenario: Fix missing parenthesis inside f-string
    Given a file "script.py" with content:
      """
      print(f"hello {name}"
      """
    When I run `patch-ts balance --file script.py --apply`
    Then the file content becomes:
      """
      print(f"hello {name}")
      """
```

### Feature: Ruby Support

```gherkin
Feature: Ruby language support
  Scenario: Fix extra end in Ruby
    Given a file "app.rb" with content:
      """
      def hello
        puts "hi"
      end
      end
      """
    When I run `patch-ts balance --file app.rb --apply`
    Then the extra 'end' is removed.
```

### Feature: PHP Support

```gherkin
Feature: PHP language support
  Scenario: Fix missing brace in PHP
    Given a file "index.php" with content:
      """
      <?php
      function hello() {
        echo "hi";
      """
    When I run `patch-ts balance --file index.php --apply`
    Then the missing '}' is inserted.
```

### Feature: HTML Tag Repair

```gherkin
Feature: HTML/XML tag repair
  Scenario: Fix mismatched closing tag
    Given a file "page.html" with content:
      """
      <div>
        <span>hello</div>
      </span>
      """
    When I run `patch-ts balance --file page.html --apply`
    Then the closing '</div>' is changed to '</span>' or vice versa to match nesting.
```

## 2. Test Strategy & Plan

### 2.1 Test Pyramid

| Level | Scope | Tools |
|-------|-------|-------|
| Unit | AST skip logic, tag detection | Rust `#[test]` |
| Integration | Full commands on new languages | `assert_cmd` |
| Regression | v0.6.0 test suite | Cargo test |
| Performance | Benchmark all languages | Criterion |

### 2.2 Risk‑Based Prioritization

| Risk | Test Focus |
|------|------------|
| AST traversal misses delimiters | Extensive corpus for each language. |
| Tag repair corrupts HTML | Test with nested mismatches and self‑closing tags. |

## 3. Test Case Specifications

### TC‑RB‑001: Ruby balance

- **Requirement:** FR‑RB‑001
- **Preconditions:** `.rb` file with extra `end`.
- **Steps:** Run `balance --apply`.
- **Expected:** Extra `end` removed.
- **Automated:** Yes.

### TC‑PHP‑001: PHP balance

- **Requirement:** FR‑PHP‑001
- **Preconditions:** `.php` file with missing `}`.
- **Steps:** Run `balance --apply`.
- **Expected:** `}` inserted.
- **Automated:** Yes.

### TC‑HTML‑001: HTML tag mismatch

- **Requirement:** FR‑HTML‑003
- **Preconditions:** `.html` file with `<div>...</span>`.
- **Steps:** Run `balance --apply`.
- **Expected:** Tags match.
- **Automated:** Yes.

### TC‑PY‑BAL‑001: Python f‑string balance

- **Requirement:** FR‑AST‑001
- **Preconditions:** Python file with f‑string missing `)`.
- **Steps:** Run `balance --apply`.
- **Expected:** `)` inserted.
- **Automated:** Yes.

## 4. NFR Verification Plans

### NFR‑PERF‑001 (Performance)

- **Method:** Criterion benchmark on 10k‑line files for each new language.
- **Threshold:** p95 ≤ 200 ms.

### NFR‑ACC‑001 (Python balance accuracy)

- **Method:** Run previously disabled Python balance tests.
- **Threshold:** 100% pass.

## 5. Requirements Traceability Matrix (RTM)

| SRS ID | Test Case(s) | Verification Method |
|--------|--------------|---------------------|
| FR‑RB‑001 | TC‑RB‑001 | Test |
| FR‑PHP‑001 | TC‑PHP‑001 | Test |
| FR‑HTML‑003 | TC‑HTML‑001 | Test |
| FR‑AST‑001 | TC‑PY‑BAL‑001 | Test |

---

This completes the specification suite for patch-ts v0.7.0.

Now I'll create the detailed implementation plan with chunked tasks and safe script workflow.

---

# patch-ts v0.7.0 Implementation Plan

> **For agentic workers:** REQUIRED: Use superpowers:subagent-driven-development (if subagents available) or superpowers:executing-plans to implement this plan. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add Ruby, PHP, HTML, XML support; refine AST traversal for Python reliability; implement tag repair; and enhance overlapping error resolution with rollback.

**Architecture:** Extend language detection to new extensions, implement `RubyLanguage`, `PHPLanguage`, `HtmlLanguage`, `XmlLanguage` using AST traversal, add tag‑aware repair for HTML/XML, refine skip conditions for Python strings, and add rollback validation to batch repair.

**Tech Stack:** Rust, tree-sitter 0.26, tree-sitter-ruby, tree-sitter-php, tree-sitter-html, tree-sitter-xml, anyhow, clap.
