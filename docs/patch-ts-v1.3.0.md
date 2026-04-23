# patch-ts v1.3.0 — AI‑Resilience Specification Suite

Below are the five core specification documents for making `patch‑ts` robust against incorrect AI inputs. They follow the layered model: Vision, BRS, SRS, Architecture, and Behavioral Spec with Test Verification.

---

## 1. Vision & Strategic Alignment

```markdown
# Product Vision & Strategic Alignment — patch‑ts v1.3.0

| Field | Value |
|-------|-------|
| Project | patch‑ts |
| Document | Vision & Strategic Alignment |
| Version | 1.0 |
| Date | 2026‑04‑23 |
| Author | patch‑ts team, assisted by spec‑writer |
| Status | Draft |

## 1. Vision Statement

> **patch‑ts becomes the world’s most AI‑resilient patching tool – one that not only applies patches perfectly, but actively defends against LLM hallucinations, communicates confidence to agents, and gracefully self‑corrects when given ambiguous or malformed input.**

## 2. Elevator Pitch (Moore Template)

> For **AI coding agents and developers who need absolute reliability when applying LLM‑generated patches**, patch‑ts is a **tree‑sitter‑backed CLI with built‑in input hardening, confidence scoring, and multi‑strategy fuzzy matching**. Unlike other tools that blindly apply whatever the LLM outputs, our product **validates every input, scores every match, and provides structured retry hints** – making it the safest bridge between human intent and AI execution.

## 3. Problem Statement & Business Context

**Problem:** v1.2.0 is fully functional, but relies on the AI agent to produce correct command syntax, accurate line numbers, and proper content. LLMs frequently:
- Hallucinate file paths, flags, or content that doesn’t exist.
- Inject markdown fences or invisible control characters.
- Assume wrong indentation or context, causing near‑matches to fail silently.
- Produce patches that are syntactically valid but semantically wrong (e.g., referencing non‑existent variables).

When these failures occur, the agent receives only a binary success/failure, lacking the **confidence data** needed to self‑correct. This leads to frustrating, slow feedback loops.

**Why now:**  
- AI coding agents (Claude Code, Codex CLI, Aider, etc.) are increasingly mainstream, and their primary failure mode is **tool invocation error**.  
- The core patching engine is now rock‑solid; the next frontier is **defensive robustness**.
- Research across the industry (Sentry CLI input hardening, mpatch fuzzy resilience, arXiv hallucination detection) shows proven patterns we can adopt.

**Business drivers:**  
- Reduce support burden and community complaints about “silent corruption” from AI‑generated patches.
- Position patch‑ts as the **trusted intermediary** in any AI‑assisted coding workflow.
- Increase adoption by AI agent frameworks that need a robust, self‑diagnosing patching backend.

## 4. Target Users / Customers

| Segment | Description |
|---------|-------------|
| **AI agent frameworks** | Claude Code, Codex, Aider, Continue.dev, etc. – need reliable, structured feedback and safe defaults. |
| **Individual developers using AI** | Use LLMs for daily refactoring; need protection against bad LLM output. |
| **DevOps / CI pipelines** | Cannot afford silent corruption in automated patches. |
| **Plugin authors** | Need the same robustness guarantees in their custom repair plugins. |

**Explicitly NOT targeting (v1.3.0):**  
- Semantic repair beyond delimiter balancing.  
- Cloud‑based validation services.  
- Real‑time LLM interaction within the tool.

## 5. User Needs & Value Proposition

| Need | patch‑ts v1.3.0 Value |
|------|-----------------------|
| “I need patch‑ts to reject clearly malicious or malformed input.” | Input validation layer blocks control characters, path traversal, and encoding attacks. |
| “I need to know *how confident* patch‑ts is about a match.” | JSON output now includes a `confidence` score (0.0‑1.0) and details of which strategy succeeded. |
| “The LLM gave me a patch wrapped in ``` ``` – I want it to just work.” | Heredoc parser now strips Markdown fences automatically. |
| “The LLM used 4‑space indentation, but my file uses tabs – the patch failed.” | Smart indentation correction normalizes added lines to match the target file. |
| “The LLM invented a variable name – I need to know.” | Identifier cross‑validation flags when patched code references symbols not present in the file. |
| “I want to test my AI agent’s resilience before deploying it.” | Adversarial input test suite and fuzzing harness included in CI. |

**Differentiator:** No other patching tool provides a **confidence‑scored, multi‑strategy cascade with hallucination detection** – all without requiring network access or an LLM runtime.

## 6. Desired Outcomes & Success Metrics

### Business Outcomes (v1.3.0)

| ID | Outcome | Key Result / Target |
|----|---------|---------------------|
| G‑1 | Reduce AI‑caused corruption reports | Zero reports of file corruption due to hallucinated input within 6 months of release. |
| G‑2 | Improve AI agent self‑correction rate | At least 60% of agent retries succeed after receiving structured confidence feedback (measured via telemetry opt‑in). |
| G‑3 | Increase adoption by AI frameworks | At least 3 major AI coding frameworks document patch‑ts as their recommended patching backend. |

### Product Outcomes (v1.3.0)

| ID | Outcome | Metric |
|----|---------|--------|
| P‑1 | All inputs are validated | 100% of known hallucination patterns (control chars, path traversal, encoding attacks) are rejected with clear errors. |
| P‑2 | Confidence scoring is accurate | Confidence scores correlate with actual match success: when confidence ≥0.95, actual success rate is ≥99%. |
| P‑3 | Multi‑strategy cascade improves success rate | Patches that fail exact match succeed after anchor‑based or similarity‑based fallback in ≥20% of test cases. |
| P‑4 | LLM output fences are handled | 100% of heredoc inputs wrapped in single‑level Markdown fences are correctly parsed. |
| P‑5 | Adversarial inputs don’t cause panics | Fuzzed CLI inputs produce only graceful errors, no panics, and no file modifications. |

## 7. Strategic Constraints

| Constraint | Description |
|------------|-------------|
| **Backward compatibility** | All v1.2.0 CLI flags, JSON schemas, and WASM plugin interfaces remain unchanged. |
| **Performance** | Input validation and confidence scoring must add ≤5ms overhead to each command. |
| **Cross‑platform** | All new features must work on Linux, macOS, and Windows. |
| **Dependencies** | No new heavy dependencies; reuse existing crates (tree‑sitter, strsim, serde_json). |

## 8. Goals and Non‑Goals (v1.3.0)

### Goals

- Implement input validation layer (control chars, path traversal, encoding).
- Add confidence scoring to JSON output, with multi‑strategy cascade (exact → anchor → similarity → fuzzy).
- Markdown‑fence detection and stripping in heredoc parser.
- Smart indentation correction during patch application.
- Identifier cross‑validation (when applicable) via tree‑sitter queries.
- Adversarial input test suite and CLI fuzzing harness.
- Update documentation and JSON schema for new fields.

### Non‑Goals (explicitly excluded)

- Full semantic validation (type checking, borrow checking).
- Integration with specific LLM providers.
- Automatic patch generation.
- Real‑time agent interaction loop.

## 9. Operational Concept & High‑Level Scenarios

### Concept of Operations

An AI agent invokes `patch‑ts` as before. At the entry point, every argument is validated: control characters rejected, path traversal blocked, encoding attacks flagged. When a patch is applied, the fuzzy matcher now cascades through multiple strategies, recording which one succeeded and with what confidence. The JSON response includes this score, enabling the agent to decide whether to accept or retry.

### High‑Level Scenarios (v1.3.0)

1. **Hallucinated file path blocked**  
   Agent: `patch‑ts patch --file ../../etc/passwd` → Error: “Path traversal detected: ../../etc/passwd”. File untouched.

2. **Confidence‑scored fuzzy match**  
   Agent sends a patch with drifted line numbers. Exact match fails. Anchor‑based search finds candidate at 0.82 confidence. Similarity search finds same line at 0.96. Tool applies patch and returns `{ "confidence": 0.96, "strategy": "similarity" }`. Agent logs confidence for future tuning.

3. **Markdown fence stripped automatically**  
   Agent sends heredoc with ` ``` ``` ` wrapping the patch. `patch‑ts` detects the fence, strips it, applies the content. No error.

4. **Smart indentation correction**  
   Target file uses tabs; LLM patch uses spaces. During application, `patch‑ts` detects the indentation style of surrounding lines and normalizes the patch to match. Patch applies successfully where it would have failed before.

5. **Identifier hallucination flagged**  
   Agent changes a function call to `do_thing(x)`, but `x` doesn’t exist in scope. `patch‑ts` applies the patch (since syntax is valid) but returns a warning: `{ "warnings": ["Identifier 'x' not found in file"] }`.

## 10. Stakeholders, Sponsorship & Governance

| Role | Name / Org | Responsibility |
|------|------------|----------------|
| **Executive Sponsor** | Project maintainer | Approves strategic direction. |
| **Product Owner** | Project maintainer | Prioritizes features, manages scope. |
| **Engineering Lead** | Core contributor(s) | Oversees technical implementation. |
| **Community** | Open‑source contributors | Review PRs, test pre‑releases. |

## 11. Risks, Assumptions & Open Questions

### Assumptions

- Tree‑sitter queries can provide sufficient information for basic identifier existence checks without a full symbol table.
- `strsim` and existing tokenization can support anchor‑based and similarity‑based strategies with acceptable performance.
- The community will accept additional JSON fields without breaking existing integrations.

### Risks

| Risk | Likelihood | Impact | Mitigation |
|------|------------|--------|------------|
| Input validation may reject legitimate but unusual file paths. | Low | Medium | Provide `--allow‑all‑paths` flag for power users. |
| Smart indentation may alter patch semantics in edge cases. | Medium | High | Make smart indentation opt‑in (`--fix‑indent`), disabled by default. |
| Confidence scoring may give false sense of security. | Medium | Medium | Document that confidence only reflects match quality, not semantic correctness. |

### Open Questions

- Should identifier cross‑validation be opt‑in or always‑on? (Always‑on as a warning, not a blocking error.)
- How to handle multi‑file patches with confidence scoring? (Score per file, aggregate minimum.)

---

*This vision document anchors the AI‑resilience strategy for patch‑ts v1.3.0.*
```

---

## 2. Business & Stakeholder Requirements Specification (BRS)

```markdown
# Business & Stakeholder Requirements Specification — patch‑ts v1.3.0

| Field | Value |
|-------|-------|
| Project | patch‑ts |
| Document | Business & Stakeholder Requirements Specification |
| Version | 1.0 |
| Date | 2026‑04‑23 |
| Author | patch‑ts team, assisted by spec‑writer |
| Status | Draft |
| References | Vision v1.3.0 |

## 1. Business Context

### 1.1 Purpose

This BRS defines the business‑level requirements for patch‑ts v1.3.0, which focuses on hardening the tool against incorrect AI inputs, providing actionable confidence feedback, and improving the success rate of patches generated by LLMs.

### 1.2 Business Problem / Opportunity

AI coding agents frequently produce malformed, incorrectly‑structured, or semantically impossible patches. Without built‑in resilience, patch‑ts can silently corrupt files, fail with cryptic errors, or waste developer time on debugging. By adding input validation, confidence scoring, and format flexibility, patch‑ts becomes a reliable intermediary that both protects the codebase and helps AI agents self‑correct.

### 1.3 Scope Boundaries

**In Scope:**
- Input validation layer (control characters, path traversal, encoding attacks).
- Multi‑strategy fuzzy matching cascade with confidence scores.
- Markdown‑fence detection and stripping.
- Smart indentation correction (opt‑in).
- Identifier existence warnings via tree‑sitter queries.
- Adversarial input test suite.

**Out of Scope:**
- Full semantic validation.
- Integration with external LLM APIs.
- Automatic patch generation.

## 2. Business Goals, Objectives & Success Metrics

| ID | Goal | Fit Criterion |
|----|------|---------------|
| BR‑001 | Eliminate AI‑caused corruption | Zero user reports of file corruption caused by hallucinated inputs within 6 months. |
| BR‑002 | Increase AI agent self‑correction rate | At least 60% of agent retries succeed after receiving structured confidence feedback. |
| BR‑003 | Improve patch success rate on realistic AI output | ≥95% of patches in a curated corpus of real LLM‑generated patches succeed after multi‑strategy cascade. |
| BR‑004 | Adoption by AI frameworks | At least 3 major frameworks document patch‑ts integration. |

*(Traceability: BR‑001…004 ← Vision G‑1…G‑3)*

## 3. Business Model & Processes

patch‑ts remains open‑source. AI‑resilience features attract framework integrations and increase community trust, driving adoption and contributions.

## 4. Business Rules & Policies

| ID | Rule | Source |
|----|------|--------|
| BR‑R1 | All user‑supplied arguments must be validated before any file operation. | Security |
| BR‑R2 | Confidence scores must be included in JSON output for every patch, balance, and explain command. | Usability |
| BR‑R3 | Markdown‑fence detection must be on by default; can be disabled with `--no‑strip‑fence`. | Flexibility |
| BR‑R4 | Smart indentation must be opt‑in via `--fix‑indent` to avoid unintended changes. | Safety |

## 5–12. Additional Sections

*(Follow the same pattern as previous BRS documents: Glossary, Conceptual Domain Model, Stakeholder Needs, System‑in‑Context, Constraints, Risks, Traceability – summarized for brevity.)*

Key Stakeholder Needs:
- SN‑001: As an AI agent, I need structured confidence feedback so I can decide whether to retry.
- SN‑002: As a developer, I need protection against hallucinated file paths and malicious inputs.
- SN‑003: As a developer, I want patches wrapped in Markdown fences to just work.
- SN‑004: As a CI pipeline, I need deterministic, safe behavior even with adversarial inputs.

---

*This BRS establishes the business foundation for v1.3.0.*
```

---

## 3. Software Requirements Specification (SRS)

```markdown
# Software Requirements Specification — patch‑ts v1.3.0

| Field | Value |
|-------|-------|
| Project | patch‑ts |
| Document | Software Requirements Specification |
| Version | 1.0 |
| Date | 2026‑04‑23 |
| Author | patch‑ts team, assisted by spec‑writer |
| Status | Draft |
| References | BRS v1.3.0, Vision v1.3.0 |

## 1. Introduction & Scope

This SRS defines the functional and non‑functional requirements for patch‑ts v1.3.0, which adds AI‑input resilience, confidence scoring, multi‑strategy matching, Markdown fence handling, smart indentation, identifier validation, and adversarial testing.

### 1.1 Scope

- Input validation module.
- Multi‑strategy cascade with confidence scores.
- Markdown fence stripping.
- Smart indentation correction.
- Identifier existence warnings.
- Adversarial input test suite.
- Updated JSON schema and documentation.

### 1.2 Out of Scope

- Full semantic validation.
- External LLM integration.
- Automatic patch generation.

## 2. System Context & Overview

Same C1 context as before. New internal modules: `validate.rs` (input hardening), enhancements to `matching.rs` (cascade), `patch.rs` (indentation), `ast.rs` (identifier queries).

## 3. Functional Capabilities & Behavior

### Feature: Input Validation Layer

| ID | Requirement | Priority | Acceptance Criteria |
|----|-------------|----------|---------------------|
| FR‑VAL‑001 | All string arguments shall be scanned for control characters (ASCII 0x00‑0x1F except \n, \t). If found, reject with error. | Must | Test with embedded NULL, ESC, BEL chars. |
| FR‑VAL‑002 | File path arguments shall be validated against path traversal patterns (`../`, `..\\`, absolute paths with /etc, C:\). Reject with error. | Must | `../../etc/passwd` blocked; `--allow‑all‑paths` bypasses. |
| FR‑VAL‑003 | File path arguments shall be checked for double‑encoding (e.g., `%2F`, `%3A`). Reject with error. | Should | Encoded slashes rejected. |
| FR‑VAL‑004 | Arguments exceeding configurable length limits (default 10KB for content) shall be rejected. | Should | Huge inputs blocked. |

### Feature: Confidence Scoring & Multi‑Strategy Cascade

| ID | Requirement | Priority | Acceptance Criteria |
|----|-------------|----------|---------------------|
| FR‑CONF‑001 | The fuzzy matching pipeline shall implement a cascade: exact match → anchor‑based (key unique lines) → similarity (token‑based Jaccard) → fuzzy (Levenshtein). | Must | Each strategy tried in order; first with confidence ≥ threshold wins. |
| FR‑CONF‑002 | Each strategy shall produce a confidence score (0.0‑1.0) representing match quality. | Must | Exact = 1.0; anchor = token overlap; similarity = Jaccard; fuzzy = normalized Levenshtein. |
| FR‑CONF‑003 | JSON output shall include a `confidence` field (number) and a `strategy` field (string). | Must | `{"confidence": 0.92, "strategy": "anchor"}` |
| FR‑CONF‑004 | The default confidence threshold shall be 0.90 (configurable via `--confidence <N>`). | Must | Below threshold: reject with structured error. |
| FR‑CONF‑005 | The cascade shall be parallelizable for multi‑file operations (per‑file). | Should | Parallelism doesn't affect per‑file confidence scores. |

### Feature: Markdown Fence Handling

| ID | Requirement | Priority | Acceptance Criteria |
|----|-------------|----------|---------------------|
| FR‑FENCE‑001 | The heredoc parser shall detect leading/trailing Markdown code fences (``` ) and strip them before processing. | Must | ` ```\n<<< ...\n---\n ...\n``` ` works. |
| FR‑FENCE‑002 | Fence stripping may be disabled with `--no‑strip‑fence`. | Should | Flag honored. |

### Feature: Smart Indentation Correction

| ID | Requirement | Priority | Acceptance Criteria |
|----|-------------|----------|---------------------|
| FR‑INDENT‑001 | When `--fix‑indent` is used, the tool shall detect the indentation style (tabs/spaces, count) of the target line and normalize the new content to match. | Must | LLM patch with spaces adapts to tab‑indented file. |
| FR‑INDENT‑002 | Indentation correction shall only apply to the leading whitespace of each new line. | Must | Content after indent is unchanged. |
| FR‑INDENT‑003 | If indentation style is ambiguous, no correction shall be applied (warn). | Should | Mixed tabs/spaces disables correction. |

### Feature: Identifier Cross‑Validation

| ID | Requirement | Priority | Acceptance Criteria |
|----|-------------|----------|---------------------|
| FR‑IDENT‑001 | After a successful patch, the tool shall check if any identifiers in the new content exist in the file’s scope (via tree‑sitter query for definitions/references). | Should | Warnings emitted for unknown identifiers. |
| FR‑IDENT‑002 | Identifier warnings shall be included in JSON output as a `warnings` array. | Should | Non‑blocking; patch still applied. |
| FR‑IDENT‑003 | Identifier validation shall be language‑aware (Rust, TypeScript, JavaScript, Python, Go). | Could | Initial support for these five; others follow. |

### Feature: Adversarial Input Test Suite

| ID | Requirement | Priority | Acceptance Criteria |
|----|-------------|----------|---------------------|
| FR‑ADV‑001 | The test suite shall include a corpus of malformed inputs: control chars, path traversal, double‑encoding, extreme values, missing flags. | Must | CI runs these tests; no panics. |
| FR‑ADV‑002 | A CLI fuzzer (based on `proptest` or custom harness) shall generate random argument combinations and verify graceful error handling. | Should | No crashes in 10,000 fuzz iterations. |

## 4. Quality & Non‑Functional Requirements

| ID | Category | Requirement | Fit Criterion |
|----|----------|-------------|---------------|
| NFR‑PERF‑001 | Performance | Input validation must add ≤1ms overhead per command. | Benchmarked. |
| NFR‑PERF‑002 | Performance | Multi‑strategy cascade must complete within existing performance budgets (patch <200ms). | Benchmark suite. |
| NFR‑SEC‑001 | Security | No new vulnerabilities introduced; input validation blocks known attack patterns. | Pass security audit. |
| NFR‑COMPAT‑001 | Compatibility | All v1.2.0 tests pass without modification. | CI regression suite. |
| NFR‑DOC‑001 | Documentation | New CLI flags, JSON fields, and behaviors are documented in README and inline help. | Review checklist. |

## 5. External Interfaces & Data Contracts

### CLI New/Modified Flags

- `--confidence <N>` : set minimum confidence threshold (default 0.90).
- `--no‑strip‑fence` : disable Markdown fence stripping.
- `--fix‑indent` : enable smart indentation correction.
- `--allow‑all‑paths` : disable path traversal protection.

### JSON Output Schema (Additions)

```json
{
  "success": true,
  "confidence": 0.96,
  "strategy": "similarity",
  "warnings": ["Identifier 'x' not found in file"],
  "actions": [...]
}
```

### Error Response (Enhanced)

```json
{
  "success": false,
  "error": {
    "code": "patch_ts::input_validation",
    "message": "Control character 0x00 found in argument --old",
    "confidence": null,
    "retry_hint": "Remove null bytes from input"
  }
}
```

## 6–8. Constraints, Assumptions, TBD

*(Summarized: must maintain backward compatibility, tree‑sitter queries sufficient for identifier checks, new dependencies minimal.)*

---

*This SRS defines the complete behavioral contract for v1.3.0.*
```

---

## 4. Architecture & Design Specification

```markdown
# Architecture & Design Specification — patch‑ts v1.3.0

| Field | Value |
|-------|-------|
| Project | patch‑ts |
| Document | Architecture & Design Specification |
| Version | 1.0 |
| Date | 2026‑04‑23 |
| Author | patch‑ts team, assisted by spec‑writer |
| Status | Draft |
| References | SRS v1.3.0, BRS v1.3.0 |

## 1. Context & Scope

This document describes the architectural design for the AI‑resilience features in v1.3.0: input validation, confidence‑scored matching cascade, Markdown fence handling, smart indentation, identifier validation, and adversarial testing infrastructure.

## 2. Goals & Non‑Goals

**Goals:**
- Introduce `validate.rs` for argument hardening.
- Refactor `matching.rs` to implement a cascade of strategies producing confidence scores.
- Extend `cli.rs` heredoc parsing to handle Markdown fences.
- Add `indent.rs` (or extend `patch.rs`) for smart indentation.
- Use tree‑sitter queries in `ast.rs` for identifier existence checks.
- Build an adversarial test suite and CLI fuzzer.

**Non‑Goals:**
- Refactor the entire codebase.
- Introduce external services or heavy new dependencies.

## 3. Architecturally Significant Requirements (ASRs)

| ASR ID | Description | Source |
|--------|-------------|--------|
| ASR‑001 | Input validation must run before any file I/O. | FR‑VAL‑001 |
| ASR‑002 | Confidence scores must be consistent and reproducible. | FR‑CONF‑002 |
| ASR‑003 | Multi‑strategy cascade must not regress performance. | NFR‑PERF‑002 |
| ASR‑004 | New JSON fields must be backward‑compatible. | NFR‑COMPAT‑001 |

## 4. The Design

### 4.1 System Overview (C4 Level 2)

```
[User/Agent] → (CLI) → [Input Validation] → [Command Handler]
                                                   ├── matching.rs (cascade)
                                                   ├── patch.rs (indent)
                                                   ├── repair.rs
                                                   └── lsp, watch, remote...
```

### 4.2 Key Design Changes

**Input Validation (`validate.rs`)**
- A set of pure functions: `validate_string`, `validate_path`.
- Called from `cli.rs` immediately after argument parsing.
- Rejects with structured error using `miette`.

**Confidence‑Scored Cascade (`matching.rs`)**
- Replace the single `fuzzy_match_line` with a `MatchCascade` struct.
- Strategies: `exact`, `anchor` (find lines that are unique and identical), `similarity` (Jaccard on tokenized lines), `fuzzy` (Levenshtein).
- Each returns `Option<MatchResult>` with a confidence score.
- Cascade stops at first match exceeding `confidence_threshold`.

**Markdown Fence Handling (`cli.rs`)**
- In `parse_heredoc`, after reading stdin, check for leading `` ``` `` and trailing `` ``` ``. Strip if present.
- Controlled by `--no‑strip‑fence` flag.

**Smart Indentation (`patch.rs`)**
- New function `normalize_indentation(new_content, target_line_indent)`.
- Called when `--fix‑indent` is set.
- Detects indent style from the line being replaced.

**Identifier Validation (`ast.rs`)**
- New method on `Language` trait: `check_identifiers(&self, source: &str, new_text: &str) -> Vec<String>`.
- For Rust: query for `(identifier)` nodes in new_text, check if they appear in source (simple existence check; not full scope analysis).
- Returns list of unknown identifiers as warnings.

**Adversarial Testing**
- New file `tests/adversarial_input_tests.rs` with hardcoded malicious inputs.
- New binary target `fuzz_cli` that uses `proptest` to generate random args and assert no panics.

### 4.3 Data Model

- `MatchResult` extended with `confidence: f64` and `strategy: String`.
- `JsonDiagnostic` extended with `confidence: Option<f64>`, `strategy: Option<String>`, `warnings: Option<Vec<String>>`.
- `PatchOptions` extended with `confidence_threshold: f64`, `fix_indent: bool`, `strip_fence: bool`.

### 4.4 Security Architecture

Input validation runs before any file operation. Path traversal protection is defense‑in‑depth; the OS would likely block writing, but we explicitly reject before attempting.

## 5. Architecture Decision Records (ADRs)

### ADR‑025: Cascade with confidence scores over single fuzzy strategy

**Context:** Single fuzzy strategy provides no insight into match quality.
**Decision:** Implement a cascade with confidence scores.
**Alternatives:** Keep current system. Rejected for lack of feedback.
**Consequences:** Better feedback for AI agents; slight complexity increase.

### ADR‑026: Input validation layer as separate module

**Context:** Need to harden against hallucinated inputs.
**Decision:** Create `validate.rs` with reusable validators.
**Alternatives:** Inline checks in CLI. Rejected for reusability.
**Consequences:** Centralized security; easy to extend.

### ADR‑027: Smart indentation opt‑in

**Context:** Changing indentation can alter meaning in some languages (Python).
**Decision:** Make it opt‑in via `--fix‑indent`.
**Alternatives:** Always on. Rejected for safety.
**Consequences:** User controls when it’s applied.

## 6. API & Interface Contracts

CLI extended with new flags. JSON schema updated with new optional fields.

## 7. Cross‑cutting Concerns

- **Error Handling**: Input validation errors are user‑friendly and include remediation hints.
- **Testing**: Adversarial test suite; fuzzer for robustness.
- **Performance**: Cascade strategies ordered by speed; early exit prevents unnecessary work.

## 8. Alternatives Considered

| Alternative | Why Rejected |
|-------------|--------------|
| Integrate with LLM for confidence | Requires network; against offline‑first principle. |
| Full symbol table analysis | Too complex for v1.3.0; simple existence check suffices. |

## 9. Traceability

| ASR | ADR | Component |
|-----|-----|-----------|
| ASR‑001 | ADR‑026 | validate.rs |
| ASR‑002 | ADR‑025 | matching.rs |
| ASR‑003 | ADR‑025 | matching.rs, cli.rs |
| ASR‑004 | – | diagnostics.rs |

---

*This architecture specification provides the blueprint for implementing v1.3.0.*
```

---

## 5. Behavioral Specification & Test Verification Plan

```markdown
# Behavioral Specification & Test Verification Plan — patch‑ts v1.3.0

| Field | Value |
|-------|-------|
| Project | patch‑ts |
| Document | Behavioral Specification & Test Verification Plan |
| Version | 1.0 |
| Date | 2026‑04‑23 |
| Author | patch‑ts team, assisted by spec‑writer |
| Status | Draft |
| References | SRS v1.3.0, Architecture v1.3.0 |

## 1. Behavioral Specifications (Specification by Example)

### Feature: Input Validation

```gherkin
Feature: Input validation
  Scenario: Reject control character in --old argument
    When I run `patch‑ts patch --file main.rs --line 1 --old "hello\x00" --new "world"`
    Then the command fails with message containing "Control character 0x00"
    And the file is unchanged

  Scenario: Reject path traversal in --file
    When I run `patch‑ts patch --file "../../etc/passwd" --line 1 --old "a" --new "b"`
    Then the command fails with message "Path traversal"
    And the file is untouched
```

### Feature: Confidence‑Scored Cascade

```gherkin
Feature: Confidence‑scored matching cascade
  Scenario: Exact match returns confidence 1.0
    Given a file with "const x = 1;" at line 3
    When I run `patch‑ts patch --file main.rs --line 3 --old "const x = 1;" --new "const x = 2;" --json`
    Then the JSON response contains `"confidence": 1.0`
    And `"strategy": "exact"`

  Scenario: Fuzzy match returns lower confidence
    Given a file with "const y = 1;" at line 4
    When I run `patch‑ts patch --file main.rs --line 4 --old "const x = 1;" --new "const x = 2;" --json --fuzz 5`
    Then the JSON response contains `"confidence"` less than 1.0
    And `"strategy"` is not "exact"
```

### Feature: Markdown Fence Handling

```gherkin
Feature: Markdown fence stripping
  Scenario: Heredoc wrapped in Markdown fence
    When I run `patch‑ts patch --file main.rs --line 1` with heredoc:
      """
      BT
      <<<
      old line
      ---
      new line
      BT
      """
    Then the patch is applied successfully
    And the "old line" is replaced with "new line"
```

### Feature: Smart Indentation

```gherkin
Feature: Smart indentation correction
  Scenario: LLM uses spaces, target file uses tabs
    Given a file with "\t\tlet x = 1;" at line 2
    When I run `patch‑ts patch --file main.rs --line 2 --old "let x = 1;" --new "let x = 2;" --fix‑indent`
    Then the new line is indented with tabs, matching the file style
```

### Feature: Identifier Warnings

```gherkin
Feature: Identifier cross‑validation
  Scenario: Patch references non‑existent variable
    Given a file with "fn main() { let y = 1; }"
    When I run `patch‑ts patch --file main.rs --line 1 --old "let y = 1;" --new "let x = 2;" --json`
    Then the patch succeeds (valid syntax)
    And the JSON contains a warning: "Identifier 'x' not found in file"
```

### Feature: Adversarial Robustness

```gherkin
Feature: Adversarial input handling
  Scenario: Fuzzed arguments do not cause panic
    When I run the fuzz harness with 10,000 random argument combinations
    Then no process panic occurs
    And all failures produce a clear error message
```

## 2. Test Strategy & Plan

### 2.1 Test Pyramid

| Level | Scope | Tools |
|-------|-------|-------|
| Unit | Validators, cascade strategies, indent logic | Rust `#[test]` |
| Integration | Full CLI with adversarial inputs | `assert_cmd`, tempfile |
| Property | Fuzzing CLI with random args | `proptest` |
| Manual | Reviewing confidence scores on real LLM outputs | Ad‑hoc charters |

### 2.2 Risk‑Based Prioritization

| Risk | Test Focus |
|------|------------|
| Input validation false positives | Test with unusual but valid file paths (Unicode, spaces) |
| Confidence scores misleading | Validate against known‑correct corpus |
| Markdown fence false positives | Test with code that contains ``` in string literals |

## 3. Test Case Specifications (Excerpt)

| TC‑ID | Requirement | Steps | Expected |
|-------|-------------|-------|----------|
| TC‑VAL‑001 | FR‑VAL‑001 | Pass `--old` with embedded `\x00` | Error message, exit non‑zero |
| TC‑CONF‑001 | FR‑CONF‑001 | Apply patch with shifted line | Confidence < 1.0, strategy ≠ exact |
| TC‑FENCE‑001 | FR‑FENCE‑001 | Heredoc with `` ``` `` wrapping | Patch applied |
| TC‑INDENT‑001 | FR‑INDENT‑001 | File with tabs, patch with spaces | Indent style matches file |
| TC‑IDENT‑001 | FR‑IDENT‑001 | Patch changes `y` to `x` where `x` not present | Warning in JSON |

## 4. NFR Verification

| NFR | Verification Method |
|-----|---------------------|
| NFR‑PERF‑001 | Micro‑benchmark input validation functions |
| NFR‑PERF‑002 | Extend existing criterion benchmarks with cascade |
| NFR‑SEC‑001 | Code audit; fuzz testing for known patterns |
| NFR‑COMPAT‑001 | Run v1.2.0 test suite unchanged |
| NFR‑DOC‑001 | Review updated README, inline help |

## 5. Requirements Traceability Matrix (RTM)

*(Table mapping Vision/BRS objectives → SRS requirements → Test cases.)*
