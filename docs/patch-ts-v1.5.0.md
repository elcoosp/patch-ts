# patch-ts v1.5.0 — LLM‑Input Resilience Specification Suite

Below are the five core specification documents for **patch‑ts v1.5.0**, which focuses on making the tool dramatically more resilient to malformed AI output through output sanitization, format‑agnostic parsing, enhanced self‑healing feedback, and adaptive matching strategies.

---

## 1. Vision & Strategic Alignment

```markdown
# Product Vision & Strategic Alignment — patch‑ts v1.5.0

| Field | Value |
|-------|-------|
| Project | patch‑ts |
| Document | Vision & Strategic Alignment |
| Version | 1.0 |
| Date | 2026‑04‑23 |
| Author | patch‑ts team, assisted by spec‑writer |
| Status | Draft |

## 1. Vision Statement

> **patch‑ts becomes the most forgiving, self‑healing patching tool in the world – one that accepts the messy, malformed, and often bizarre output that LLMs produce, and transforms it into clean, correct patches with minimal developer intervention.**

## 2. Elevator Pitch (Moore Template)

> For **AI coding agents and developers who struggle with LLMs that produce malformed diffs, hallucinated line numbers, and broken JSON arguments**, patch‑ts is a **tree‑sitter‑backed CLI with a built‑in LLM output sanitizer, format‑agnostic parser, and powerful self‑healing feedback**. Unlike other patching tools that fail on the first syntax error or malformed argument, our product **repairs broken JSON, strips irrelevant prose, handles ellipsis placeholders, and teaches the agent how to fix its mistakes** – making it the ultimate bridge between chaotic LLM output and reliable code changes.

## 3. Problem Statement & Business Context

**Problem:** v1.4.0 hardened patch‑ts against hallucinated inputs and added MCP integration, but LLMs still produce output with predictable structural failures: Markdown‑wrapped code, `<think>` blocks interspersed with content, trailing commas in JSON arguments, single‑quoted keys, ellipsis placeholders (`...`), inconsistent indentation, and explanatory prose around the actual patch. These failures are systematic, not random, and existing tools fail on them because they were designed for human‑crafted input.

**Why now:**
- AI coding agents are now mainstream (Claude Code, Codex CLI, Aider, etc.), and their primary failure mode is tool‑invocation error, not code‑generation error.  
- The `jsonrepair` and `fuzzy-parser` crates provide production‑ready solutions for repairing malformed JSON and fuzzy‑matching field names.  
- Research from Aider, `diff-apply`, and `mpatch` demonstrates proven patterns: output sanitization, multi‑strategy cascades, ellipsis handling, and structured self‑healing feedback.  
- The community consistently reports that "the AI gave me the right fix, but patch‑ts couldn't parse it" – this is a solvable problem.

**Business drivers:**  
- Dramatically increase patch‑ts's success rate on real‑world LLM output (target: ≥95% of realistic malformed inputs successfully parsed).  
- Position patch‑ts as the **indispensable intermediary** between any LLM and any codebase.  
- Reduce the volume of "patch‑ts failed but the fix was correct" support issues to near‑zero.

## 4. Target Users / Customers

| Segment | Description |
|---------|-------------|
| **AI agent frameworks** | Claude Code, Codex CLI, Aider, Continue.dev, etc. – need a patching backend that "just works" regardless of LLM quirks. |
| **Individual developers using AI** | Use LLMs for daily refactoring; frustrated when the tool rejects perfectly good fixes due to formatting. |
| **DevOps / CI pipelines** | Cannot afford manual intervention when automated patches fail due to LLM output issues. |
| **Plugin authors** | Need the same resilience guarantees in their custom repair plugins. |

**Explicitly NOT targeting (v1.5.0):**  
- Full program synthesis or automatic patch generation.  
- Cloud‑based validation services.  
- Real‑time LLM interaction within the tool (beyond self‑healing feedback).

## 5. User Needs & Value Proposition

| Need | patch‑ts v1.5.0 Value |
|------|-----------------------|
| “The LLM gave me a valid diff, but patch‑ts rejected it because of a stray comment or fence.” | Format‑agnostic parser extracts the diff from prose, fences, and mixed content automatically. |
| “The LLM used `...` to mean ‘leave the rest unchanged’, but patch‑ts didn’t understand.” | Ellipsis‑pattern handling recognizes `...` placeholders in search/replace blocks. |
| “The LLM’s JSON arguments had trailing commas and single quotes – patch‑ts failed.” | Built‑in JSON repair silently fixes trailing commas, single‑quoted keys, and unclosed brackets. |
| “The LLM wrapped the fix in a `<think>` block and patch‑ts tried to apply the thinking as code.” | `<think>` block stripping removes reasoning‑model artifacts before parsing. |
| “I need patch‑ts to tell the agent exactly how to fix its malformed input.” | Enhanced self‑healing feedback includes ±3 context lines, confidence breakdown, and error codes. |
| “The same patch keeps failing because the target line isn’t unique enough.” | Uniqueness‑adjusted confidence dynamically raises the threshold for common lines. |

**Differentiator:** No other patching tool provides a built‑in LLM output sanitizer, JSON repair, ellipsis handling, and structured self‑healing feedback in a single offline binary.

## 6. Desired Outcomes & Success Metrics

### Business Outcomes (v1.5.0)

| ID | Outcome | Key Result / Target |
|----|---------|---------------------|
| G‑1 | Dramatically increase parse success rate | ≥95% of realistic malformed LLM outputs successfully parsed and applied. |
| G‑2 | Reduce user frustration | Near‑zero support issues citing "patch‑ts rejected a valid fix due to formatting". |
| G‑3 | Become the default patching backend | At least 5 major AI frameworks document the sanitizer as a key integration feature. |

### Product Outcomes (v1.5.0)

| ID | Outcome | Metric |
|----|---------|--------|
| P‑1 | LLM output sanitizer operational | 100% of known malformation patterns (fences, think blocks, JSON quirks, prose) are normalized. |
| P‑2 | Format‑agnostic diff extraction | Diffs embedded in prose, multi‑level fences, and mixed content are correctly extracted in ≥95% of test cases. |
| P‑3 | JSON repair works | Trailing commas, single‑quoted keys, and unclosed brackets are repaired and parsed correctly in ≥99% of test cases. |
| P‑4 | Ellipsis patterns handled | `...` in search/replace blocks is interpreted as "match zero or more lines" and applied correctly. |
| P‑5 | Enhanced self‑healing feedback | Retry prompts include ±3 context lines, confidence breakdown, and error codes. |

## 7. Strategic Constraints

| Constraint | Description |
|------------|-------------|
| **Backward compatibility** | All v1.4.0 CLI flags, JSON schemas, MCP interface, and WASM plugin interfaces remain unchanged. |
| **Performance** | Sanitization and JSON repair must add ≤10ms overhead per command. |
| **Cross‑platform** | All new features must work on Linux, macOS, and Windows. |
| **Dependencies** | Use existing crates where possible (`jsonrepair`, `fuzzy-parser`); no new heavy dependencies. |

## 8. Goals and Non‑Goals (v1.5.0)

### Goals

- Implement LLM output sanitizer: strip `<think>` blocks, repair malformed JSON, extract diff content from prose.
- Build format‑agnostic diff extraction: auto‑detect and extract unified diffs from mixed‑content LLM output.
- Enhanced self‑healing feedback: multi‑line context, confidence breakdown, error codes.
- Uniqueness‑adjusted confidence: dynamically adjust confidence threshold based on target content uniqueness.
- Ellipsis pattern support: recognize `...` in search/replace blocks.
- Multi‑line anchor detection: use pairs of unique lines when single‑line anchors fail.
- Whitespace‑flexible diff matching: make diff context matching ignore whitespace differences by default.
- Update documentation, README, and integration guides.

### Non‑Goals (explicitly excluded)

- Full program synthesis or automatic patch generation.
- Cloud‑based services.
- Real‑time LLM interaction beyond structured feedback.
- Replacement of existing language servers or compilers.

## 9. Operational Concept & High‑Level Scenarios

### Concept of Operations

An AI agent invokes patch‑ts via CLI or MCP. The raw input passes through a sanitization pipeline: `<think>` blocks are removed, Markdown fences are stripped, JSON arguments are repaired (trailing commas, single quotes), and prose is separated from structural content. The resulting clean diff or heredoc is then processed through the existing confidence‑scored cascade, now enhanced with ellipsis handling, multi‑line anchors, and uniqueness‑adjusted thresholds. On failure, a structured self‑healing response is returned with context, confidence breakdown, and retry instructions.

### High‑Level Scenarios (v1.5.0)

1. **LLM output wrapped in prose and fences**  
   Agent: "Sure! Here's the fix:" ` ```diff ... ``` `  
   patch‑ts strips the prose, extracts the diff, applies it. No manual cleaning needed.

2. **JSON arguments with trailing commas**  
   Agent: `{"file": "main.rs", "line": 42, "old": "x = 1",}`  
   patch‑ts repairs the trailing comma before parsing, then applies the patch.

3. **`<think>` block interference**  
   Agent (from reasoning model): `<think>I should patch line 42</think>{"file": "main.rs", ...}`  
   patch‑ts strips the `<think>` block, extracts the clean JSON, applies the patch.

4. **Ellipsis pattern in search/replace**  
   Agent: `<<<fn main() { ... }---fn main() { println!("hi"); }`  
   patch‑ts recognizes `...` as "match the ellipsis lines unchanged" and applies the replacement correctly.

5. **Enhanced self‑healing feedback**  
   Patch fails due to low confidence. patch‑ts returns:  
   `{"retry_prompt": "At line 42 of src/main.rs, expected `let x = 1;` but found `let y = 2;`. Surrounding context: ... Confidence: 0.72. Threshold: 0.90. Error code: E002."}`

## 10. Stakeholders, Sponsorship & Governance

| Role | Name / Org | Responsibility |
|------|------------|----------------|
| **Executive Sponsor** | Project maintainer | Approves strategic direction. |
| **Product Owner** | Project maintainer | Prioritizes features, manages scope. |
| **Engineering Lead** | Core contributor(s) | Oversees technical implementation. |
| **Community** | Open‑source contributors | Review PRs, test pre‑releases. |

## 11. Risks, Assumptions & Open Questions

### Assumptions

- The `jsonrepair` crate handles the most common JSON malformations (trailing commas, single quotes, unclosed brackets).  
- `<think>` blocks are well‑formed and can be stripped with a simple regex.  
- Ellipsis patterns are used consistently by LLMs (three dots on a line by itself or inline).

### Risks

| Risk | Likelihood | Impact | Mitigation |
|------|------------|--------|------------|
| JSON repair may produce semantically wrong but syntactically valid JSON. | Medium | Medium | Validate repaired arguments; warn on repair. |
| Sanitization may strip legitimate content. | Low | Medium | Provide `--no‑sanitize` flag to bypass. |
| Ellipsis patterns may be ambiguous in edge cases. | Medium | Low | Fall back to standard matching when ambiguous. |

### Open Questions

- Should sanitization be enabled by default? (Yes, with opt‑out.)
- How to handle multiple diffs in a single LLM output? (Extract all, apply in order.)

---

*This vision document anchors the LLM‑input resilience strategy for patch‑ts v1.5.0.*
```

---

## 2. Business & Stakeholder Requirements Specification (BRS)

```markdown
# Business & Stakeholder Requirements Specification — patch‑ts v1.5.0

| Field | Value |
|-------|-------|
| Project | patch‑ts |
| Document | Business & Stakeholder Requirements Specification |
| Version | 1.0 |
| Date | 2026‑04‑23 |
| Author | patch‑ts team, assisted by spec‑writer |
| Status | Draft |
| References | Vision v1.5.0 |

## 1. Business Context

### 1.1 Purpose

This BRS defines the business‑level requirements for patch‑ts v1.5.0, which adds LLM output sanitization, format‑agnostic parsing, enhanced self‑healing feedback, uniqueness‑adjusted confidence, and ellipsis pattern support.

### 1.2 Business Problem / Opportunity

v1.4.0 hardened patch‑ts against hallucinated inputs, but LLMs still produce malformed output that causes parse failures. The business opportunity is to become the **universal, unforgiving‑to‑LLMs‑but‑forgiving‑to‑users** patching backend that agents can rely on regardless of output quality.

### 1.3 Scope Boundaries

**In Scope:**
- LLM output sanitizer (think blocks, JSON repair, prose extraction).
- Format‑agnostic diff extraction.
- Enhanced self‑healing feedback with context and error codes.
- Uniqueness‑adjusted confidence.
- Ellipsis pattern support.
- Multi‑line anchor detection.
- Whitespace‑flexible diff matching.

**Out of Scope:**
- Full program synthesis.
- Cloud‑based services.
- Real‑time LLM interaction beyond structured feedback.

## 2. Business Goals, Objectives & Success Metrics

| ID | Goal | Fit Criterion |
|----|------|---------------|
| BR‑001 | Dramatically increase parse success rate | ≥95% of realistic malformed LLM outputs successfully parsed. |
| BR‑002 | Reduce user frustration | Near‑zero support issues about formatting failures. |
| BR‑003 | Become the default patching backend | At least 5 major AI frameworks document the sanitizer feature. |

*(Traceability: BR‑001…003 ← Vision G‑1…G‑3)*

## 3. Business Model & Processes

patch‑ts remains open‑source. LLM‑input resilience makes it indispensable for AI agent frameworks, driving adoption and community contributions.

## 4. Business Rules & Policies

| ID | Rule | Source |
|----|------|--------|
| BR‑R1 | Sanitization must be enabled by default; opt‑out via `--no‑sanitize`. | Usability |
| BR‑R2 | JSON repair must warn when repairs are applied (`--verbose`). | Transparency |
| BR‑R3 | Self‑healing feedback must include error codes and context. | Agent usability |

## 5–12. Additional Sections

*(Follow the same pattern as previous BRS documents: Glossary, Conceptual Domain Model, Stakeholder Needs, System‑in‑Context, Constraints, Risks, Traceability.)*

Key Stakeholder Needs:
- SN‑001: As an AI agent, I need patch‑ts to accept my output even when it contains prose, fences, or JSON quirks.
- SN‑002: As a developer, I want patch‑ts to understand `...` placeholders in patches.
- SN‑003: As a developer, I want detailed, actionable feedback when a patch fails so the LLM can self‑correct.

---

*This BRS establishes the business foundation for v1.5.0.*
```

---

## 3. Software Requirements Specification (SRS)

```markdown
# Software Requirements Specification — patch‑ts v1.5.0

| Field | Value |
|-------|-------|
| Project | patch‑ts |
| Document | Software Requirements Specification |
| Version | 1.0 |
| Date | 2026‑04‑23 |
| Author | patch‑ts team, assisted by spec‑writer |
| Status | Draft |
| References | BRS v1.5.0, Vision v1.5.0 |

## 1. Introduction & Scope

This SRS defines the functional and non‑functional requirements for patch‑ts v1.5.0, which adds LLM output sanitization, format‑agnostic diff extraction, enhanced self‑healing feedback, uniqueness‑adjusted confidence, and ellipsis pattern support.

### 1.1 Scope

- LLM output sanitizer (think blocks, JSON repair, prose extraction).
- Format‑agnostic diff extraction.
- Enhanced self‑healing feedback with context and error codes.
- Uniqueness‑adjusted confidence.
- Ellipsis pattern support.
- Multi‑line anchor detection.
- Whitespace‑flexible diff matching.
- Integration tests and documentation.

### 1.2 Out of Scope

- Full program synthesis or automatic patch generation.
- Cloud‑based services.
- Real‑time LLM interaction beyond structured feedback.

## 2. System Context & Overview

Same C1 context as before. New internal modules: `sanitize.rs` (LLM output sanitizer), `extract.rs` (format‑agnostic diff extraction). Enhancements to `matching.rs` (ellipsis, multi‑line anchor, uniqueness), `heal.rs` (context, error codes).

## 3. Functional Capabilities & Behavior

### Feature: LLM Output Sanitizer

| ID | Requirement | Priority | Acceptance Criteria |
|----|-------------|----------|---------------------|
| FR‑SAN‑001 | The tool shall strip `<think>` blocks from any input before parsing. | Must | `<think>reasoning</think>` removed; content after block parsed. |
| FR‑SAN‑002 | The tool shall strip Markdown code fences (single and multi‑level) from heredoc and diff input. | Must | ```` ``` ```` stripped even when nested. |
| FR‑SAN‑003 | The tool shall repair malformed JSON arguments (trailing commas, single‑quoted keys, unclosed brackets). | Must | `{"a": 1,}` → `{"a": 1}`; `{'a': 1}` → `{"a": 1}`. |
| FR‑SAN‑004 | The tool shall extract structural content (JSON, diff) from surrounding prose. | Should | "Here's the fix: `{...}`" → `{...}` extracted. |
| FR‑SAN‑005 | Sanitization may be disabled with `--no‑sanitize`. | Should | Flag honored. |

### Feature: Format‑Agnostic Diff Extraction

| ID | Requirement | Priority | Acceptance Criteria |
|----|-------------|----------|---------------------|
| FR‑EXT‑001 | The tool shall detect and extract unified diffs embedded in free‑form text. | Must | Prose + ` ```diff ... ``` ` + prose → diff extracted. |
| FR‑EXT‑002 | The tool shall handle multiple diffs in a single input (apply in order). | Should | Sequential diffs applied. |
| FR‑EXT‑003 | The tool shall auto‑detect non‑standard diff formats (context diff, bare `@@`). | Could | Context diffs converted to unified. |

### Feature: Ellipsis Pattern Support

| ID | Requirement | Priority | Acceptance Criteria |
|----|-------------|----------|---------------------|
| FR‑ELL‑001 | In heredoc `<<<` blocks, `...` on a line by itself shall match any number of lines between the surrounding content. | Must | `fn main() { ... }` matches `fn main() { let x = 1; }`. |
| FR‑ELL‑002 | Ellipsis matching shall be greedy (match the shortest span that aligns with the next explicit line). | Should | Correct span selected. |
| FR‑ELL‑003 | Ellipsis support may be disabled with `--no‑ellipsis`. | Should | Flag honored. |

### Feature: Enhanced Self‑Healing Feedback

| ID | Requirement | Priority | Acceptance Criteria |
|----|-------------|----------|---------------------|
| FR‑HEAL‑004 | The `retry_prompt` field shall include ±3 lines of surrounding context from the target file. | Must | Context shown in feedback. |
| FR‑HEAL‑005 | The `retry_prompt` field shall include the confidence score, threshold, and strategy attempted. | Must | Breakdown included. |
| FR‑HEAL‑006 | Error responses shall include machine‑readable error codes (E001, E002, etc.). | Must | Error codes mapped to categories. |

### Feature: Uniqueness‑Adjusted Confidence

| ID | Requirement | Priority | Acceptance Criteria |
|----|-------------|----------|---------------------|
| FR‑UNIQ‑001 | The tool shall compute a "uniqueness score" for the target content (0 = very common, 1 = unique). | Must | Score computed. |
| FR‑UNIQ‑002 | The effective confidence threshold shall be raised for low‑uniqueness content and lowered for high‑uniqueness content. | Must | Threshold adjusted dynamically. |
| FR‑UNIQ‑003 | The adjustment shall be configurable via `--uniqueness‑weight <N>` (default 0.2). | Should | Configurable. |

### Feature: Multi‑Line Anchor Detection

| ID | Requirement | Priority | Acceptance Criteria |
|----|-------------|----------|---------------------|
| FR‑ANCH‑001 | When single‑line anchors fail, the tool shall attempt pairs of unique lines. | Should | Fallback succeeds. |
| FR‑ANCH‑002 | Anchor pairs shall be selected based on uniqueness (rarest lines in the expected block). | Should | Best pair chosen. |

### Feature: Whitespace‑Flexible Diff Matching

| ID | Requirement | Priority | Acceptance Criteria |
|----|-------------|----------|---------------------|
| FR‑WS‑001 | Diff context matching shall ignore leading whitespace differences by default. | Must | Indentation changes don't break matching. |
| FR‑WS‑002 | Whitespace‑flexible mode may be disabled with `--strict‑whitespace`. | Should | Flag honored. |

## 4. Quality & Non‑Functional Requirements

| ID | Category | Requirement | Fit Criterion |
|----|----------|-------------|---------------|
| NFR‑PERF‑001 | Performance | Sanitization and JSON repair must add ≤10ms overhead per command. | Benchmarked. |
| NFR‑SEC‑001 | Security | Sanitization must not inadvertently execute or evaluate input. | Review. |
| NFR‑COMPAT‑001 | Compatibility | All v1.4.0 tests pass without modification. | CI regression suite. |
| NFR‑DOC‑001 | Documentation | New flags, JSON fields, and behaviors documented in README and inline help. | Review checklist. |

## 5. External Interfaces & Data Contracts

### CLI New/Modified Flags

- `--no‑sanitize` : disable LLM output sanitizer.
- `--no‑ellipsis` : disable ellipsis pattern support.
- `--uniqueness‑weight <N>` : adjust uniqueness influence on confidence (default 0.2).
- `--strict‑whitespace` : disable whitespace‑flexible diff matching.

### JSON Output Schema (Additions)

```json
{
  "success": false,
  "confidence": 0.72,
  "threshold": 0.90,
  "uniqueness_score": 0.45,
  "strategy": "fuzzy",
  "retry_prompt": "At line 42 of src/main.rs, expected `let x = 1;` but found `let y = 2;`. Context: ... Confidence: 0.72. Threshold: 0.90. Error code: E002.",
  "error_code": "E002",
  "context_lines": ["line 41: fn main() {", "line 42: let y = 2;", "line 43: }"],
  "sanitization_applied": ["think_block_removed", "json_repaired"]
}
```

## 6–8. Constraints, Assumptions, TBD

*(Summarized: must maintain backward compatibility, `jsonrepair` and `fuzzy-parser` crates stable, sanitization reversible via `--no‑sanitize`.)*

---

*This SRS defines the complete behavioral contract for v1.5.0.*
```

---

## 4. Architecture & Design Specification

```markdown
# Architecture & Design Specification — patch‑ts v1.5.0

| Field | Value |
|-------|-------|
| Project | patch‑ts |
| Document | Architecture & Design Specification |
| Version | 1.0 |
| Date | 2026‑04‑23 |
| Author | patch‑ts team, assisted by spec‑writer |
| Status | Draft |
| References | SRS v1.5.0, BRS v1.5.0 |

## 1. Context & Scope

This document describes the architectural design for the LLM‑input resilience features in v1.5.0: output sanitizer, format‑agnostic diff extraction, enhanced self‑healing feedback, uniqueness‑adjusted confidence, and ellipsis support.

## 2. Goals & Non‑Goals

**Goals:**
- Introduce `sanitize.rs` for pre‑processing malformed LLM output.
- Introduce `extract.rs` for format‑agnostic diff extraction.
- Enhance `matching.rs` with ellipsis, multi‑line anchors, and uniqueness scoring.
- Enhance `heal.rs` with context and error codes.
- Keep all changes backward‑compatible.

**Non‑Goals:**
- Refactor the entire codebase.
- Introduce external services or heavy new dependencies beyond `jsonrepair` and `fuzzy-parser`.

## 3. Architecturally Significant Requirements (ASRs)

| ASR ID | Description | Source |
|--------|-------------|--------|
| ASR‑001 | Sanitization must run before all other parsing. | FR‑SAN‑001 |
| ASR‑002 | Ellipsis patterns must be handled in the search cascade. | FR‑ELL‑001 |
| ASR‑003 | Uniqueness scoring must influence confidence thresholds. | FR‑UNIQ‑001 |
| ASR‑004 | Self‑healing feedback must include context and error codes. | FR‑HEAL‑004/006 |

## 4. The Design

### 4.1 System Overview (C4 Level 2)

```
[User/Agent] → [Sanitizer] → [Format Extractor] → [CLI Handler] → [Patch/Balance Engine]
```

### 4.2 Key Design Changes

**Sanitizer (`sanitize.rs`)**
- `strip_think_blocks(input: &str) -> String` – regex `<think>.*?</think>`.
- `repair_json(input: &str) -> Result<serde_json::Value>` – uses `jsonrepair` crate.
- `extract_structural_content(input: &str) -> String` – removes known prose patterns, extracts JSON or diff.

**Format‑Agnostic Extractor (`extract.rs`)**
- `extract_diffs(input: &str) -> Vec<String>` – uses regex to find `@@ ... @@` blocks, even within fences or prose.

**Ellipsis Handling (`matching.rs`)**
- In `apply_literal_patch`, when `...` appears on a line by itself in the `expected` block, treat it as a wildcard matching zero or more lines. Use a greedy algorithm to find the smallest span that aligns with the next explicit line.

**Uniqueness Scoring (`matching.rs`)**
- Before the cascade, compute `uniqueness_score` = 1 / (1 + count_of_matching_lines_in_file). Adjust `confidence_threshold += uniqueness_weight * (1 - uniqueness_score)`.

**Enhanced Self‑Healing (`heal.rs`)**
- Add `context_lines: Vec<String>` to `HealContext`.
- Add `error_code: &str` mapping.

### 4.3 Data Model

- New structs: `SanitizeResult`, `ExtractedDiff`.
- Extend `MatchResult` with `uniqueness_score`.
- Extend `HealContext` with `context_lines`, `error_code`.

## 5. Architecture Decision Records (ADRs)

### ADR‑031: Use `jsonrepair` crate for malformed JSON

**Context:** LLMs frequently produce JSON with trailing commas, single‑quoted keys, and unclosed brackets.  
**Decision:** Use the `jsonrepair` crate, which handles all known patterns.  
**Alternatives:** Custom parser (too much work).  
**Consequences:** Adds dependency; well‑tested and maintained.

### ADR‑032: Ellipsis as greedy wildcard

**Context:** LLMs use `...` to mean "leave the rest unchanged" in search/replace blocks.  
**Decision:** Treat `...` on its own line in the expected block as a wildcard matching zero or more lines, greedy to the next explicit match.  
**Alternatives:** Full diff‑style context (more complex).  
**Consequences:** Simple, matches common LLM behavior; may need edge‑case handling.

### ADR‑033: Uniqueness‑adjusted confidence

**Context:** Common lines (e.g., `}`) have low uniqueness and should require higher confidence that we're matching the right one.  
**Decision:** Compute a uniqueness score from the frequency of the line in the file, and adjust the confidence threshold proportionally.  
**Alternatives:** Fixed threshold (current).  
**Consequences:** Fewer false positives on common lines; slightly more complex cascade.

## 6. API & Interface Contracts

CLI extended with new flags. JSON schema updated with new optional fields.

## 7. Cross‑cutting Concerns

- **Error Handling**: Sanitization failures are non‑fatal; original input is used as fallback.
- **Testing**: Integration tests with real LLM output samples.
- **Performance**: Sanitization runs once at input entry; JSON repair uses efficient algorithm.

## 8. Alternatives Considered

| Alternative | Why Rejected |
|-------------|--------------|
| Custom JSON parser for repair | `jsonrepair` already exists and handles all known patterns. |
| Full diff‑context matching for ellipsis | Over‑complex; greedy wildcard is simpler and matches LLM behavior. |

## 9. Traceability

| ASR | ADR | Component |
|-----|-----|-----------|
| ASR‑001 | ADR‑031 | sanitize.rs |
| ASR‑002 | ADR‑032 | matching.rs |
| ASR‑003 | ADR‑033 | matching.rs |
| ASR‑004 | – | heal.rs |

---

*This architecture specification provides the blueprint for implementing v1.5.0.*
```

---

## 5. Behavioral Specification & Test Verification Plan

```markdown
# Behavioral Specification & Test Verification Plan — patch‑ts v1.5.0

| Field | Value |
|-------|-------|
| Project | patch‑ts |
| Document | Behavioral Specification & Test Verification Plan |
| Version | 1.0 |
| Date | 2026‑04‑23 |
| Author | patch‑ts team, assisted by spec‑writer |
| Status | Draft |
| References | SRS v1.5.0, Architecture v1.5.0 |

## 1. Behavioral Specifications (Specification by Example)

### Feature: LLM Output Sanitizer

```gherkin
Feature: LLM output sanitization
  Scenario: Strip think blocks
    Given raw input: "<think>I should patch line 42</think>{\"file\": \"main.rs\", ...}"
    When the sanitizer processes this input
    Then the `<think>` block is removed
    And the JSON content remains

  Scenario: Repair trailing comma in JSON
    Given raw JSON: `{"file": "main.rs", "line": 42,}`
    When the sanitizer processes this JSON
    Then the trailing comma is removed
    And the JSON parses successfully

  Scenario: Repair single-quoted JSON
    Given raw JSON: `{'file': 'main.rs', 'line': 42}`
    When the sanitizer processes this JSON
    Then single quotes are converted to double quotes
    And the JSON parses successfully

  Scenario: Extract JSON from prose
    Given raw input: "Here's the fix: {\"file\": \"main.rs\", \"line\": 42}"
    When the sanitizer processes this input
    Then the JSON is extracted from the surrounding prose
    And the result is `{"file": "main.rs", "line": 42}`
```

### Feature: Ellipsis Pattern Support

```gherkin
Feature: Ellipsis pattern support
  Scenario: Ellipsis in heredoc expected block
    Given a file with content "fn main() {\n    let x = 1;\n}"
    When I run `patch‑ts patch --file main.rs --line 1 --old "fn main() {\n...\n}" --new "fn main() {\n    println!(\"hi\");\n}"`
    Then the ellipsis matches the lines between the braces
    And the patch is applied successfully

  Scenario: Disable ellipsis with flag
    When I run with `--no‑ellipsis` and `...` in expected
    Then `...` is treated as a literal string, not a wildcard
    And the patch fails with content mismatch
```

### Feature: Enhanced Self‑Healing Feedback

```gherkin
Feature: Enhanced self‑healing feedback
  Scenario: Retry prompt with context
    When a patch fails due to content mismatch
    Then the JSON response includes a `retry_prompt` field
    And the retry prompt includes ±3 lines of surrounding context
    And the retry prompt includes the confidence score, threshold, and strategy

  Scenario: Error code in response
    When any error occurs
    Then the JSON response includes an `error_code` field
    And the error code is one of E001‑E010
```

### Feature: Uniqueness‑Adjusted Confidence

```gherkin
Feature: Uniqueness‑adjusted confidence
  Scenario: Common line requires higher confidence
    Given a file with 50 identical `}` lines
    When a patch targets line 30 with content `}`
    Then the effective confidence threshold is raised
    And a fuzzy match with low confidence is rejected

  Scenario: Unique line allows lower confidence
    Given a file with a single unique line
    When a patch targets that line
    Then the effective confidence threshold is lowered
    And a fuzzy match with moderate confidence is accepted
```

## 2. Test Strategy & Plan

### 2.1 Test Pyramid

| Level | Scope | Tools |
|-------|-------|-------|
| Unit | Sanitizer, ellipsis matcher, uniqueness scoring, heal feedback | Rust `#[test]` |
| Integration | Full CLI with malformed inputs (real LLM samples) | `assert_cmd`, tempfile |
| Property | Fuzzing sanitizer with random malformed JSON | `proptest` |
| Manual | Review with real LLM outputs from Claude, GPT, Codex | Charters |

### 2.2 Risk‑Based Prioritization

| Risk | Test Focus |
|------|------------|
| Sanitizer strips legitimate content | Test with code containing `<>`, `{}`, etc. |
| JSON repair changes meaning | Validate repaired JSON matches intent. |
| Ellipsis greedy matching wrong span | Test with nested blocks, ambiguous spans. |

## 3. Test Case Specifications (Excerpt)

| TC‑ID | Requirement | Steps | Expected |
|-------|-------------|-------|----------|
| TC‑SAN‑001 | FR‑SAN‑001 | Provide input with `<think>` block | `<think>` removed |
| TC‑SAN‑002 | FR‑SAN‑003 | Provide JSON with trailing comma | Comma removed, valid JSON |
| TC‑ELL‑001 | FR‑ELL‑001 | Provide heredoc with `...` | Ellipsis matches correct span |
| TC‑HEAL‑001 | FR‑HEAL‑004 | Trigger content mismatch | `retry_prompt` contains context lines |
| TC‑UNIQ‑001 | FR‑UNIQ‑001 | Target common line with low confidence | Rejected due to adjusted threshold |

## 4. NFR Verification

| NFR | Verification Method |
|-----|---------------------|
| NFR‑PERF‑001 | Micro‑benchmark sanitizer and JSON repair functions. |
| NFR‑SEC‑001 | Code audit; ensure sanitizer doesn't execute input. |
| NFR‑COMPAT‑001 | Run v1.4.0 test suite unchanged. |
| NFR‑DOC‑001 | Review README, inline help for new flags. |

## 5. Requirements Traceability Matrix (RTM)

*(Table mapping Vision/BRS objectives → SRS requirements → Test cases.)*

---

*This verification plan ensures complete coverage of all v1.5.0 LLM‑input resilience features.*
