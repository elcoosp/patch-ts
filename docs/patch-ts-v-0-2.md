I'll guide you through creating a comprehensive set of specification documents for the `patch-ts` improvements, using the `spec-writer` skill framework. Since we're working from the detailed analysis you provided, I'll synthesize that information into formal spec documents following the evidence‑backed templates.

I'll proceed with the **/spec** full walkthrough, creating all five documents in sequence. The output files will be saved to `/mnt/user-data/outputs/` with the project name `patch-ts-enhancements`.

Let's begin.

---

## Document 1: Product Vision & Strategic Alignment (`/spec-vision`)

**Reference:** `vision.md` (already loaded in context)

**Gathered context from analysis:**
- Project: Enhancements to existing Rust CLI tool `patch-ts`
- Target users: AI coding agents (primary), developers applying patches (secondary)
- Problem: Current tool fails on slightly misaligned patches, causing rework and manual intervention
- Desired outcome: Higher patch success rate, reduced syntax‑error rejections, smoother AI‑agent workflows

**Vision document generated:**

```markdown
# Product Vision & Strategic Alignment: patch-ts Enhancements

| Field | Value |
|-------|-------|
| Project | patch-ts-enhancements |
| Document | Vision & Strategic Alignment |
| Version | 0.1 (Draft) |
| Date | 2026-04-22 |
| Author | AI‑assisted specification |
| Status | Draft — Pending Review |

## Vision Statement

> **For** AI coding agents and developers applying automated code modifications, **who** struggle with brittle line‑number‑based patches and syntax‑error rejections, **our enhanced patch‑ts tool** is a **tree‑sitter‑powered patching utility** that **intelligently matches intended changes even when context has shifted and automatically repairs minor syntactic flaws**. **Unlike** traditional diff tools or the current patch‑ts, **our tool** tolerates whitespace variations, finds semantic equivalents, and auto‑corrects delimiter mismatches, making patches succeed far more often with less manual effort.

## Elevator Pitch

> For AI‑driven development workflows and engineers applying patches programmatically, who are frustrated by patch failures due to minor context drift or trivial syntax errors, **patch‑ts Enhancements** is a **semantic‑aware patching CLI** that **applies changes with fuzzy, token‑based matching and built‑in auto‑repair**. Unlike strict diff tools or the current version, our enhanced tool understands code structure, tolerates whitespace and formatting changes, and can automatically fix unbalanced braces or missing semicolons—dramatically increasing patch success rates.

## Problem Statement & Business Context

The current `patch-ts` tool provides a solid foundation with tree‑sitter parsing and fuzzy line matching, but it still rejects many patches that are conceptually correct but slightly misaligned with the current file state. This leads to:

- AI agents needing multiple retries or manual corrections, wasting token usage and time.
- Developers abandoning automated patching workflows due to brittleness.
- Missed opportunities for seamless, automated code modifications in CI/CD and AI‑assisted development.

**Why now?** The rise of AI coding assistants (Copilot, Cursor, Aider, etc.) has increased the volume of programmatically generated patches. The cost of patch failures is growing, and a more resilient patching tool directly addresses a critical pain point in the AI‑augmented software development lifecycle.

## Target Users & Customers

| User Class | Description |
|------------|-------------|
| **AI Coding Agents** | LLM‑powered tools that generate code changes and invoke patch‑ts to apply them. They need high success rates and low‑friction error recovery. |
| **Developers** | Engineers who use patch‑ts via CLI or scripts to apply changes from code reviews, refactoring tools, or automated fixes. They value reliability and clear diagnostics. |
| **DevOps / CI Systems** | Automated pipelines that run patch‑ts as part of validation or auto‑remediation steps. They require deterministic behavior and minimal false negatives. |

**Non‑targets (explicitly excluded for this phase):**
- Non‑Rust languages (only Rust supported initially; language extensibility deferred).
- GUI or IDE plugin integration (CLI focus remains).
- Patch generation (we only apply patches; not generate them).

## User Needs & Value Proposition

| User Need | How patch‑ts Enhancements Addresses It |
|-----------|----------------------------------------|
| **Tolerate slight context shifts** | Multi‑line block matching and token‑based similarity find the correct location even if line numbers have changed. |
| **Apply patches with minor syntax errors** | Auto‑repair mode invokes `balance` command internally and applies the fixed result with a warning. |
| **Understand semantic structure** | AST‑aware matching finds the correct node (e.g., a function) even when surrounding text changes. |
| **Clear, actionable failure messages** | JSON diagnostics provide structured error details and suggested fixes, enabling agents to retry intelligently. |

**Key Differentiator:** Unlike `patch` or `sed`, patch‑ts understands Rust syntax. Unlike the current version, it tolerates formatting differences and can automatically fix common syntax mistakes introduced by the patch.

## Desired Outcomes & Success Metrics

### Business Outcomes (Level 0)
| ID | Outcome | Measurement |
|----|---------|-------------|
| G‑1 | Increase patch success rate in AI‑agent workflows | ≥ 30% relative reduction in patch‑application failures across a benchmark of AI‑generated patches. |
| G‑2 | Reduce manual intervention for patching tasks | ≥ 50% reduction in support requests related to patch‑ts failures from internal users. |

### Product Outcomes (Leading Indicators)
| ID | Outcome | Measurement |
|----|---------|-------------|
| P‑1 | Fuzzy matching correctly locates intended lines | ≥ 95% of fuzzy‑matched patches apply successfully in synthetic drift tests. |
| P‑2 | Auto‑repair resolves simple delimiter errors | ≥ 80% of syntax‑error‑causing patches are auto‑repaired and applied successfully. |
| P‑3 | Users perceive higher reliability | Post‑release survey shows ≥ 4.2/5 satisfaction with patch success rate. |

## Strategic Constraints

| Constraint | Description |
|------------|-------------|
| **Language** | Initial enhancements focus on Rust (tree‑sitter‑rust). Architecture must allow future language additions. |
| **Performance** | Patch application time must not increase by more than 20% for typical files (<2000 lines). |
| **Backward Compatibility** | Existing CLI flags and behavior must remain unchanged unless explicitly deprecated. |
| **Dependency** | Must remain compatible with current `tree-sitter` 0.25 and `flickzeug` for diff handling. |

## Goals and Non‑goals (Scope / Anti‑scope)

### Goals
- [ ] **G1:** Implement whitespace‑normalized and token‑based fuzzy matching for single‑line and multi‑line expected content.
- [ ] **G2:** Integrate auto‑repair (balance) as an optional step when a patch introduces a syntax error.
- [ ] **G3:** Add marker‑based targeting (`--marker`) to allow patches anchored to comments.
- [ ] **G4:** Enhance JSON diagnostic output with structured suggestions and possible fixes.
- [ ] **G5:** Support fuzzy context matching for unified diff application (`--diff --fuzz`).
- [ ] **G6:** Improve performance with incremental parsing where possible.

### Non‑goals (explicitly excluded)
- **NG1:** Support for languages other than Rust in this release.
- **NG2:** Full AST‑based refactoring engine (we only apply textual patches with AST validation).
- **NG3:** Interactive patch conflict resolution UI.
- **NG4:** Integration with specific AI agent frameworks (tool remains general‑purpose CLI).
- **NG5:** Automatic patch generation (we apply existing patches, not create them).

## Operational Concept & High‑Level Scenarios

### Concept of Operations
patch‑ts operates as a command‑line tool invoked by scripts, AI agents, or humans. It reads a source file, applies a specified change (literal replacement, deletion, insertion, or unified diff), validates the result with tree‑sitter, and writes the updated file (or previews it). The enhancements add tolerance for minor mismatches and automatic repair capabilities.

### Key Scenarios

1. **AI Agent Applies Fuzzy Patch**  
   An AI coding assistant generates a patch that replaces a function body. The line numbers are slightly off due to prior edits. patch‑ts uses multi‑line token‑based matching to locate the correct block, applies the change, and reports success.

2. **Auto‑Repair of Missing Brace**  
   A patch removes a closing brace, causing a syntax error. Instead of failing, patch‑ts automatically runs the `balance` command, identifies the missing brace, and inserts it. A warning is emitted, but the patched file is saved.

3. **Marker‑Based Update**  
   A developer adds a `// PATCH-ME: update-config` comment in the source. A script invokes patch‑ts with `--marker "update-config"` to replace the associated block with new configuration code, regardless of line number shifts.

4. **Fuzzy Unified Diff**  
   A CI pipeline applies a diff that expects a line `let x = 42;` but the actual file has `let x = 42; // meaning of life`. With `--diff --fuzz=3`, patch‑ts matches the line with high similarity and applies the change.

## Stakeholders, Sponsorship & Governance

| Role | Name/Group | Responsibility |
|------|------------|----------------|
| **Executive Sponsor** | (TBD) | Approves resource allocation and strategic direction. |
| **Product Owner** | (Maintainer) | Owns vision, prioritizes features, accepts deliverables. |
| **Lead Developer** | (Contributor) | Technical design and implementation. |
| **Key Stakeholders** | AI tooling teams, Rust developer community | Provide feedback and use cases. |

**Decision Model:** Changes to goals, non‑goals, or success metrics require review and approval by Product Owner and Executive Sponsor.

## Traceability & Alignment Notes

| Goal ID | Related Initiative | Success Metric |
|---------|-------------------|----------------|
| G‑1 | Fuzzy matching enhancements | P‑1, P‑3 |
| G‑2 | Auto‑repair integration | P‑2, P‑3 |
| G‑3 | Marker‑based targeting | User adoption metric |

This document will feed into the Business Requirements Specification (BRS) where stakeholder needs will be detailed and traced to system requirements.

## Risks, Assumptions, and Open Questions

### Top Assumptions
1. Users will accept a small performance overhead for improved success rates.
2. AI‑generated patches are often semantically correct but syntactically imperfect due to line‑number drift.
3. Auto‑repair of simple delimiter errors is sufficient for most syntax failures.

### Major Risks
| Risk | Likelihood | Impact | Mitigation |
|------|------------|--------|------------|
| Token‑based matching increases false positives | Medium | High | Use conservative similarity threshold; provide `--force` override. |
| Auto‑repair may introduce unintended changes | Low | High | Run only when patch introduces syntax error AND original was valid; emit clear warning. |
| Performance degradation on large files | Medium | Medium | Implement incremental parsing; benchmark and optimize. |

### Open Questions
1. What similarity threshold should be default for token‑based matching? (Initial: 0.85)
2. Should auto‑repair be opt‑in (`--auto-repair`) or default when validation fails?
3. How to handle ambiguous matches (multiple candidates with similar scores)? (Proposed: fail with diagnostic listing candidates)
```

---

## Document 2: Business & Stakeholder Requirements Specification (`/spec-brs`)

**Reference:** `brs.md`

**Pre‑check:** Vision document exists; extracted goals (G‑1, G‑2), user classes, and constraints.

```markdown
# Business & Stakeholder Requirements Specification: patch-ts Enhancements

| Field | Value |
|-------|-------|
| Project | patch-ts-enhancements |
| Document | BRS |
| Version | 0.1 (Draft) |
| Date | 2026-04-22 |
| Author | AI‑assisted specification |
| Status | Draft — Pending Review |

## 1. Introduction

This document captures the business and stakeholder requirements for enhancing the `patch-ts` command‑line tool. It defines *what* business outcomes and stakeholder needs must be satisfied, without prescribing technical solutions. This BRS feeds directly into the Software Requirements Specification (SRS) and subsequent architecture and test documents.

## 2. Business Context

### 2.1 Business Purpose
The purpose of the enhancements is to increase the reliability and success rate of automated patch application, particularly in AI‑assisted development workflows. This reduces friction, saves developer time, and lowers the cost of AI‑generated code modifications.

### 2.2 Business Problem / Opportunity
Currently, `patch-ts` rejects many patches that are conceptually correct but fail due to:
- Line‑number drift caused by preceding edits.
- Minor formatting differences (whitespace, comments).
- Simple syntax errors introduced by the patch (e.g., missing brace).

These failures force manual intervention or costly retries, diminishing the value of automation.

### 2.3 Business Scope
**In‑scope:**
- Enhancing the patch application logic to tolerate context shifts and formatting variations.
- Adding automatic repair of common syntax errors resulting from patches.
- Improving diagnostic output to guide automated retries.

**Out‑of‑scope:**
- Generating patches (the tool only applies given patches).
- Supporting languages other than Rust (in this phase).
- Modifying the unified diff application core (we enhance around `flickzeug`).

## 3. Business Goals, Objectives & Success Metrics

| ID | Objective | Key Result / Fit Criterion |
|----|-----------|----------------------------|
| BG‑1 | Increase overall patch success rate | Achieve ≥30% relative reduction in patch application failures across a representative benchmark of AI‑generated patches within 3 months of release. |
| BG‑2 | Reduce manual troubleshooting effort | Decrease the number of GitHub issues / support requests related to patch‑ts failures by ≥50% within 6 months. |
| BG‑3 | Maintain syntactic correctness guarantees | 0% of successful patches should introduce syntax errors that go undetected (i.e., validation remains rigorous). |

## 4. Business Model and Processes

### 4.1 Value Propositions
- For AI agent developers: Higher patch success rates → lower token costs, faster iterations.
- For individual developers: Less time spent debugging failed patches → increased productivity.
- For CI/CD pipelines: More reliable automated fixes → fewer manual interventions.

### 4.2 Core Business Processes (High‑Level)
1. **Patch Application Workflow:** User/agent provides a source file and a patch specification → tool locates target, applies change, validates → outputs result or diagnostic.
2. **Error Recovery:** If validation fails, tool may attempt auto‑repair and re‑validate → outputs warning but proceeds if repair succeeds.

## 5. Business Rules and Policies

| ID | Rule | Source |
|----|------|--------|
| BR‑001 | Patches must not be applied if the resulting code introduces syntax errors that cannot be auto‑repaired, unless `--force` is used. | Quality policy |
| BR‑002 | When auto‑repair is performed, the user must be clearly warned (via stderr or JSON field) that the applied content differs from the intended patch. | Transparency requirement |
| BR‑003 | All changes must be traceable: a backup file must be created by default (`--no-backup` to disable). | Audit requirement |

## 6. Stakeholders and User Classes

### 6.1 Stakeholder Map
| Stakeholder | Role | Key Concerns |
|-------------|------|--------------|
| AI Tooling Developers | Primary user | High success rate, structured error messages for retry logic. |
| Rust Developers | Secondary user | Ease of use, clear failure reasons, safety (no corruption). |
| Maintainers | Project owners | Maintainability, performance, community adoption. |
| CI/CD Engineers | Tertiary user | Deterministic behavior, exit codes, JSON output for automation. |

### 6.2 User Classes & Personas
**Primary: AI Coding Agent (via API/CLI)**
- Needs to apply hundreds of patches per session.
- Requires high tolerance for small mismatches.
- Expects structured JSON diagnostics with actionable suggestions.

**Secondary: Developer (Human)**
- Uses CLI interactively or in scripts.
- Values clear, human‑readable error messages.
- Appreciates safety features (backups, dry‑run).

### 6.3 Jobs to Be Done (JTBD)
| User Class | Job Statement |
|------------|---------------|
| AI Agent | When applying a patch that may have drifted, I want the tool to find the closest match in the file so that the change is applied successfully without manual correction. |
| Developer | When a patch fails due to a missing brace, I want the tool to offer to fix it automatically so that I don't have to manually edit the file. |

## 7. Glossary / Ubiquitous Language

| Term | Definition | Notes |
|------|------------|-------|
| **Patch** | A specification of a change to a source file: literal replacement, deletion, insertion, or unified diff. | |
| **Fuzzy matching** | Locating the intended target line/block using similarity metrics rather than exact line number. | |
| **Auto‑repair** | Automatic correction of syntax errors introduced by a patch, using the `balance` command internally. | |
| **ASR** | Architecturally Significant Requirement — a requirement (often NFR) that heavily influences design. | |
| **Marker** | A specially formatted comment (e.g., `// PATCH-ME: id`) used as an anchor for patch location. | |

## 8. Stakeholder Needs and User Requirements

### 8.1 Stakeholder Needs (StRS‑level)
| ID | Stakeholder | Need Statement |
|----|-------------|----------------|
| SN‑001 | AI Agent | The tool must locate the target of a patch even when line numbers have shifted by up to ±N lines or when whitespace differs. |
| SN‑002 | AI Agent | When a patch introduces a syntax error that is trivially fixable (e.g., missing brace), the tool should automatically repair it and apply the patched+repaired content. |
| SN‑003 | Developer | The tool must provide clear, actionable diagnostic messages when a patch cannot be applied, including the reason and possible remediation. |
| SN‑004 | Developer | The tool must not silently corrupt the source file; backups and dry‑run must be available. |
| SN‑005 | CI/CD Engineer | The tool must support machine‑readable output (JSON) for integration into automated pipelines. |
| SN‑006 | Maintainer | The enhancements must not significantly degrade performance or increase maintenance burden. |

### 8.2 User Requirements (High‑Level)
| ID | User Class | Requirement | Traced From |
|----|------------|-------------|-------------|
| UR‑001 | AI Agent | The tool shall locate a replacement target using fuzzy matching of the expected content, tolerating whitespace variations and line shifts. | SN‑001 |
| UR‑002 | AI Agent | The tool shall support multi‑line expected content and find the best‑matching block in the file. | SN‑001 |
| UR‑003 | AI Agent | If fuzzy matching yields multiple candidates with similar scores, the tool shall report ambiguity and fail, providing candidate locations. | SN‑001 |
| UR‑004 | AI Agent | The tool shall provide an option to automatically repair syntax errors introduced by a patch (e.g., balancing delimiters). | SN‑002 |
| UR‑005 | Developer | The tool shall emit a warning when auto‑repair is applied, indicating that the final content differs from the intended patch. | SN‑002, BR‑002 |
| UR‑006 | Developer | JSON diagnostic output shall include a structured `suggestion` field with actionable next steps (e.g., "Try increasing --fuzz"). | SN‑003, SN‑005 |
| UR‑007 | Developer | The tool shall support targeting a patch via a marker comment (e.g., `// PATCH-ME: id`) instead of a line number. | SN‑001 |
| UR‑008 | All | The tool shall apply unified diffs with fuzzy context matching when `--diff --fuzz` is specified. | SN‑001 |

## 9. System‑in‑Context and Operational Concept

### 9.1 System Context
patch‑ts operates as a standalone CLI executable. It reads source files from the filesystem, parses them with tree‑sitter, applies changes, and writes updated files (or prints to stdout). External systems: none; all operations are local.

### 9.2 Operational Scenarios
1. **Fuzzy Single‑Line Replacement**  
   User invokes `patch-ts patch --file src/main.rs --line 42 --old "old line" --new "new line" --fuzz 5`. Tool searches within ±5 lines, finds best match (e.g., at line 45), replaces, validates, and writes file.

2. **Multi‑Line Block Replacement with Auto‑Repair**  
   User provides heredoc with a block of lines to replace. Tool finds best‑matching block using token‑based similarity, replaces it, detects a missing closing brace, auto‑balances, and writes file with warning.

3. **Marker‑Based Patch**  
   User adds `// PATCH-ME: update-auth` in source. Invokes `patch-ts patch --file src/auth.rs --marker "update-auth" --new "fn new_auth() { ... }"`. Tool locates marker, replaces the associated statement/block, and applies.

4. **Fuzzy Diff Application**  
   CI pipeline applies a diff that expects a line `println!("Hello");` but the file has `println!("Hello"); // greet`. With `--diff --fuzz=3`, tool matches the line and applies the change.

## 10. Stakeholder‑Level Constraints and Quality Expectations

| ID | Constraint / Quality Expectation | Fit Criterion |
|----|----------------------------------|---------------|
| C‑001 | Patch application time must not increase by more than 20% for files up to 2000 lines. | Measured via benchmark suite; average latency increase ≤20%. |
| C‑002 | The tool must remain fully backward compatible with existing CLI invocations. | All existing tests pass without modification. |
| C‑003 | Auto‑repair must only be applied when the original file was syntactically valid. | Validation: if original file has syntax errors, auto‑repair is skipped and patch fails as before. |
| Q‑001 | Fuzzy matching false positive rate must be low (<5% on benchmark). | Manual review of a sample of 100 fuzzy‑matched patches shows <5 incorrect target selections. |

## 11. Risks, Assumptions, and Open Issues

### 11.1 Assumptions
- AI‑generated patches are generally semantically correct but may have line‑number drift or minor formatting differences.
- Users prefer a successful patch with a warning over a hard failure.
- The `balance` command is sufficient to fix the majority of syntax errors introduced by patches.

### 11.2 Risks
| Risk | Mitigation |
|------|------------|
| Fuzzy matching selects wrong location, causing silent logic errors. | Conservative similarity threshold (0.9 default); provide `--force` to override; fail on ambiguity. |
| Auto‑repair makes incorrect fix, leading to valid but wrong code. | Only run auto‑repair when original was valid and patch introduces error; warn prominently. |
| Performance degrades unacceptably on large files. | Implement incremental parsing; benchmark and optimize; consider caching. |

### 11.3 Open Issues
- What should be the default fuzzy matching threshold? (Proposed: 0.9 for token‑based, 0.85 for string similarity)
- Should auto‑repair be opt‑in (`--auto-repair`) or default when validation fails? (Decision: default on, can be disabled with `--no-auto-repair`)

## 12. Traceability Mapping to Vision

| Business Goal (Vision) | Stakeholder Need (BRS) | User Requirement (BRS) |
|------------------------|------------------------|------------------------|
| G‑1 (increase success rate) | SN‑001, SN‑002 | UR‑001, UR‑002, UR‑007, UR‑008 |
| G‑2 (reduce manual intervention) | SN‑003, SN‑004 | UR‑005, UR‑006 |
| G‑3 (maintain correctness) | SN‑004 | UR‑005 (warning) |
```

---

## Document 3: Software Requirements Specification (`/spec-srs`)

**Reference:** `srs.md`

**Pre‑check:** BRS exists; we will reference stakeholder needs (SN‑xxx) and business rules.

```markdown
# Software Requirements Specification: patch-ts Enhancements

| Field | Value |
|-------|-------|
| Project | patch-ts-enhancements |
| Document | SRS |
| Version | 0.1 (Draft) |
| Date | 2026-04-22 |
| Author | AI‑assisted specification |
| Status | Draft — Pending Review |

## 1. Introduction and Scope

### 1.1 Purpose
This SRS defines the functional and non‑functional requirements for the enhanced `patch-ts` tool. It describes the software behavior required to satisfy the stakeholder needs identified in the BRS (SN‑001 to SN‑006).

### 1.2 Scope
This document covers:
- Enhancements to the patch application logic (fuzzy matching, multi‑line support, marker targeting).
- Auto‑repair integration.
- Diagnostic output improvements.
- Performance and backward compatibility constraints.

Out of scope: support for languages other than Rust, patch generation, GUI.

### 1.3 References
- Business Requirements Specification (BRS) v0.1
- Product Vision v0.1

## 2. System Context and Overview

patch‑ts is a Rust command‑line tool that uses tree‑sitter to parse Rust source files and apply patches. The system consists of:
- **CLI Parser** (`clap`): Handles command‑line arguments.
- **Patch Engine**: Core logic for locating targets and applying changes.
- **Language Module**: Rust‑specific parsing and AST utilities.
- **Repair Module**: Delimiter balancing and syntax error explanation.
- **File Manager**: Atomic writes and backups.

**External Actors:** User (human or AI agent) via terminal/script.

## 3. Functional Capabilities and Behavior

### 3.1 Capability: Enhanced Fuzzy Matching for Literal Patches

**Goal:** Locate the intended replacement target even when line numbers have shifted or content differs slightly.

**Requirements:**

| ID | Requirement (EARS style) | Priority | Acceptance Criteria |
|----|--------------------------|----------|---------------------|
| REQ‑FUNC‑001 | When applying a literal patch with `--line` and `--fuzz`, the system shall search for the expected content within ±`fuzz` lines of the specified line. | Must | A patch with expected content "old line" finds the line "old line" at line 45 when specified line is 42 and fuzz ≥3. |
| REQ‑FUNC‑002 | When comparing expected content to candidate lines, the system shall normalize whitespace (trim leading/trailing spaces, collapse multiple spaces) before computing similarity. | Must | Expected "  foo  " matches actual "foo" with similarity 1.0. |
| REQ‑FUNC‑003 | The system shall compute similarity using `normalized_levenshtein` on normalized strings, with a configurable threshold (default 0.9). | Must | Similarity below threshold causes failure. |
| REQ‑FUNC‑004 | If multiple candidate lines have similarity ≥ threshold, the system shall select the one with highest score. If scores are equal (tie), the system shall fail and report ambiguity with candidate locations. | Must | Ambiguity diagnostic lists line numbers of tied candidates. |
| REQ‑FUNC‑005 | When the expected content contains newlines (multi‑line block), the system shall search for the best‑matching contiguous block of lines using a sliding window and token‑based similarity (ignoring whitespace). | Should | Patch with 3‑line expected block finds correct block offset by ±5 lines. |
| REQ‑FUNC‑006 | Token‑based similarity for multi‑line blocks shall use tree‑sitter to tokenize both expected and candidate blocks and compute Jaccard similarity on token sequences. | Should | Two blocks differing only in comments/whitespace have similarity 1.0. |
| REQ‑FUNC‑007 | If the expected block contains no newlines but fuzzy matching is enabled, the system shall fall back to single‑line fuzzy matching as per REQ‑FUNC‑001–004. | Must | Consistent behavior. |

**Edge Cases:**
- If no line in fuzz radius meets threshold → fail with diagnostic.
- If fuzz radius is 0 → exact match required at specified line.
- Empty expected content → error.

### 3.2 Capability: Marker‑Based Targeting

**Goal:** Allow patches to be anchored to a unique marker comment, eliminating line‑number dependency.

**Requirements:**

| ID | Requirement | Priority | Acceptance Criteria |
|----|-------------|----------|---------------------|
| REQ‑FUNC‑010 | The system shall provide a `--marker <id>` option for the `patch` subcommand, mutually exclusive with `--line`. | Must | `--marker` cannot be used with `--line`. |
| REQ‑FUNC‑011 | When `--marker` is specified, the system shall scan the file for a comment matching `// PATCH-ME: <id>` (or `/* PATCH-ME: <id> */`). | Must | Finds marker `// PATCH-ME: update-auth`. |
| REQ‑FUNC‑012 | If the marker is found, the system shall replace the **immediately following statement or block** (as determined by tree‑sitter) with the new content. | Must | Marker before a function definition replaces that entire function. |
| REQ‑FUNC‑013 | If multiple markers with the same ID exist, the system shall fail with an ambiguity error listing their locations. | Must | Diagnostic lists line numbers. |
| REQ‑FUNC‑014 | If no marker is found, the system shall fail with a clear error message. | Must | Error: "Marker 'update-auth' not found". |

### 3.3 Capability: Auto‑Repair of Syntax Errors

**Goal:** Automatically fix simple syntax errors introduced by a patch (e.g., unbalanced delimiters) to increase success rate.

**Requirements:**

| ID | Requirement | Priority | Acceptance Criteria |
|----|-------------|----------|---------------------|
| REQ‑FUNC‑020 | After applying a patch, if validation fails and `--force` is not used, the system shall optionally attempt auto‑repair using the `balance` algorithm. | Must | Enabled by default; can be disabled with `--no-auto-repair`. |
| REQ‑FUNC‑021 | Auto‑repair shall only be attempted if the original file was syntactically valid. | Must | If original had errors, skip auto‑repair and fail. |
| REQ‑FUNC‑022 | The system shall call `repair::quick_balance` (or equivalent) on the patched content. If the repair produces a valid AST, the system shall write the repaired content and emit a warning. | Must | Warning: "Patch introduced syntax error but was auto‑repaired." |
| REQ‑FUNC‑023 | If auto‑repair fails to produce a valid AST, the system shall fail with the original syntax error diagnostic. | Must | No change applied. |
| REQ‑FUNC‑024 | The warning (when auto‑repair succeeds) shall be included in JSON output under a `warning` field. | Must | JSON `{ "success": true, "warning": "..." }`. |

### 3.4 Capability: Fuzzy Unified Diff Application

**Goal:** Apply unified diffs even when context lines have minor changes.

**Requirements:**

| ID | Requirement | Priority | Acceptance Criteria |
|----|-------------|----------|---------------------|
| REQ‑FUNC‑030 | The `--diff` flag shall accept an optional `--fuzz` parameter to enable fuzzy matching of context lines. | Should | `patch-ts patch --file foo.rs --diff --fuzz 3` applies diff with fuzzy context. |
| REQ‑FUNC‑031 | When applying a hunk, if exact context lines do not match, the system shall search within ±`fuzz` lines for the best match using the same similarity logic as literal patches. | Should | Hunk applies successfully despite line offset. |
| REQ‑FUNC‑032 | The system shall fall back to exact matching if `--fuzz` is 0 or not provided. | Must | Backward compatible. |

### 3.5 Capability: Enhanced JSON Diagnostics

**Goal:** Provide structured, actionable error information for AI agents.

**Requirements:**

| ID | Requirement | Priority | Acceptance Criteria |
|----|-------------|----------|---------------------|
| REQ‑FUNC‑040 | All error responses in JSON mode shall include a `suggestion` field with a human‑readable recommended action. | Must | E.g., `"Try increasing --fuzz radius"`. |
| REQ‑FUNC‑041 | For fuzzy matching failures, the JSON error shall include `best_score` and `best_match_line` fields. | Should | Helps agent decide next steps. |
| REQ‑FUNC‑042 | For ambiguity errors, the JSON error shall include a `candidates` array with line numbers and similarity scores. | Should | `"candidates": [{"line": 45, "score": 0.95}, ...]`. |
| REQ‑FUNC‑043 | For syntax errors, the JSON error shall include the span (line/column) and a `possible_fix` field if the tool can suggest one (e.g., "Missing `}`"). | Should | Derived from `explain_error`. |

## 4. Quality and Non‑functional Requirements

Organized per ISO/IEC 25010:2023.

### 4.1 Performance Efficiency

| ID | Requirement | Fit Criterion |
|----|-------------|---------------|
| NFR‑PERF‑001 | Patch application latency for a 2000‑line file shall not increase by more than 20% compared to baseline (pre‑enhancement). | Benchmark shows average latency increase ≤20%. |
| NFR‑PERF‑002 | Tokenization for multi‑line fuzzy matching shall complete within 100 ms for a 500‑line block. | Measured in CI benchmarks. |

### 4.2 Reliability

| ID | Requirement | Fit Criterion |
|----|-------------|---------------|
| NFR‑REL‑001 | The tool shall never corrupt the source file; atomic writes with backup must be used. | All write operations use `tempfile` and `persist`. |
| NFR‑REL‑002 | Fuzzy matching false positive rate (incorrect target selection) shall be <5% on a benchmark of 100 AI‑generated patches. | Manual review confirms <5 incorrect matches. |

### 4.3 Security

| ID | Requirement | Fit Criterion |
|----|-------------|---------------|
| NFR‑SEC‑001 | The tool shall not execute arbitrary code; all input is treated as data. | Code review confirms no `eval`-like constructs. |
| NFR‑SEC‑002 | Backup files shall be created with restrictive permissions (0o600). | File mode verified. |

### 4.4 Maintainability

| ID | Requirement | Fit Criterion |
|----|-------------|---------------|
| NFR‑MAINT‑001 | New fuzzy matching logic shall be modular and testable in isolation. | Unit tests cover similarity functions. |
| NFR‑MAINT‑002 | The tool shall continue to compile with Rust stable and no new major dependency conflicts. | CI passes. |

### 4.5 Compatibility

| ID | Requirement | Fit Criterion |
|----|-------------|---------------|
| NFR‑COMP‑001 | All existing CLI flags and behaviors must remain unchanged unless explicitly deprecated. | Existing tests pass without modification. |
| NFR‑COMP‑002 | JSON output structure must be backward compatible; new fields added but existing fields retained. | Existing JSON parsers continue to work. |

## 5. External Interfaces and Data Contracts

### 5.1 Command‑Line Interface (CLI)

**New/Modified Flags:**

| Flag | Description |
|------|-------------|
| `--fuzz <N>` | Existing; now also applies to `--diff` and multi‑line patches. |
| `--marker <ID>` | New; anchor patch to marker comment. |
| `--no-auto-repair` | New; disable automatic syntax repair. |
| `--diff --fuzz <N>` | New combination; enable fuzzy diff context. |

### 5.2 JSON Output Schema

**Success Response (existing):**
```json
{ "success": true }
```

**Enhanced Success (with warning):**
```json
{
  "success": true,
  "warning": "Patch introduced syntax error but was auto-repaired."
}
```

**Error Response (enhanced):**
```json
{
  "success": false,
  "error": {
    "code": "patch_ts::content_mismatch",
    "message": "no match found with similarity >= 0.9",
    "span": { "file": "src/main.rs", "line": 42, "column": 1 },
    "context": "expected 'old line' but found 'actual line'",
    "suggestion": "Try increasing --fuzz radius",
    "best_score": 0.82,
    "best_match_line": 45,
    "candidates": [ ... ]  // for ambiguity
  }
}
```

## 6. Constraints, Assumptions, and Dependencies

| ID | Constraint |
|----|------------|
| CON‑001 | The tool must remain compatible with tree‑sitter 0.25 and tree‑sitter‑rust 0.24. |
| CON‑002 | The tool must not introduce new dependencies that are not pure Rust (e.g., no C libraries) to maintain easy cross‑compilation. |

## 7. TBD Log

| ID | Item | Owner | Due |
|----|------|-------|-----|
| TBD‑001 | Finalize default similarity thresholds for token‑based matching. | Product Owner | Before beta |
| TBD‑002 | Decide whether auto‑repair is default‑on or opt‑in. | Product Owner | Before beta |
| TBD‑003 | Determine exact marker comment syntax (e.g., `// PATCH-ME:` vs `// @patch`). | Team | During design |

## 8. Requirements Attributes and Traceability Model

All requirements are assigned unique IDs with prefix:
- `REQ‑FUNC‑XXX` for functional
- `NFR‑XXX‑XXX` for non‑functional
- `CON‑XXX` for constraints

**Traceability Matrix (excerpt):**

| SRS Requirement | Traced to BRS |
|-----------------|---------------|
| REQ‑FUNC‑001 | SN‑001, UR‑001 |
| REQ‑FUNC‑002 | SN‑001 |
| REQ‑FUNC‑005 | SN‑001, UR‑002 |
| REQ‑FUNC‑010–014 | SN‑001, UR‑007 |
| REQ‑FUNC‑020–024 | SN‑002, UR‑004, UR‑005 |
| REQ‑FUNC‑030–032 | SN‑001, UR‑008 |
| REQ‑FUNC‑040–043 | SN‑003, SN‑005, UR‑006 |
| NFR‑PERF‑001 | C‑001 |
| NFR‑REL‑002 | Q‑001 |
| NFR‑COMP‑001 | C‑002 |
```

---

## Document 4: Architecture & Design Specification (`/spec-architecture`)

**Reference:** `architecture.md`

**Pre‑check:** SRS exists; ASRs extracted from NFRs and constraints.

```markdown
# Architecture & Design Specification: patch-ts Enhancements

| Field | Value |
|-------|-------|
| Project | patch-ts-enhancements |
| Document | Architecture & Design |
| Version | 0.1 (Draft) |
| Date | 2026-04-22 |
| Author | AI‑assisted specification |
| Status | Draft — Pending Review |

## 1. Context and Scope

This document describes the architectural changes required to implement the enhancements defined in the SRS. It focuses on the new modules and modifications to existing components needed to support fuzzy matching, marker‑based targeting, auto‑repair integration, and improved diagnostics.

## 2. Goals and Non‑goals

### Goals (Design‑Level)
- Extend the existing `PatchOptions` and patch application flow to incorporate fuzzy matching and auto‑repair without breaking backward compatibility.
- Introduce a modular matching subsystem that can be reused across literal patches and unified diffs.
- Keep performance impact minimal by using incremental parsing where possible and caching parse results.

### Non‑goals
- Refactoring the entire codebase; changes are scoped to `patch.rs`, `repair.rs`, and `cli.rs`.
- Adding support for languages other than Rust in this phase (but design should allow future extension).

## 3. Architecturally Significant Requirements (ASRs)

Extracted from SRS NFRs and constraints:

| ASR ID | Description | Source |
|--------|-------------|--------|
| ASR‑001 | Patch application latency increase ≤20% for typical files. | NFR‑PERF‑001 |
| ASR‑002 | False positive rate for fuzzy matching <5%. | NFR‑REL‑002 |
| ASR‑003 | Backward compatibility with existing CLI and behavior. | NFR‑COMP‑001, CON‑001 |
| ASR‑004 | Auto‑repair only when original file was valid. | REQ‑FUNC‑021 |

## 4. System Overview and High‑Level Structure

The enhanced system retains the same top‑level modules (`cli`, `patch`, `repair`, `ast`, `diagnostics`, `file`). New submodules are introduced:

- `matching`: Contains fuzzy matching logic (string‑based, token‑based, multi‑line block search).
- `marker`: Marker comment detection and AST‑based replacement.

**C4 Container Diagram (Textual Description):**
- **User** invokes `patch-ts` CLI.
- **CLI Parser** (`clap`) routes to appropriate command.
- **Patch Engine** orchestrates:
  - **Matching Subsystem**: Finds target location using fuzzy logic or marker.
  - **Repair Subsystem**: Optionally balances delimiters.
  - **AST Validator**: Uses tree‑sitter to check syntax.
- **File Manager**: Handles atomic writes and backups.

## 5. Detailed Design

### 5.1 Matching Subsystem (`src/matching.rs`)

**Purpose:** Provide a unified interface for locating patch targets with varying degrees of fuzziness.

**Key Components:**

- `LineMatcher`: Fuzzy single‑line matching with whitespace normalization and Levenshtein similarity.
- `BlockMatcher`: Multi‑line block search using sliding window and token‑based similarity (Jaccard on token sequences).
- `TokenSimilarity`: Uses tree‑sitter to tokenize a string and compare token sets.

**Integration with `apply_literal_patch`:**
```rust
// pseudocode
let target = if options.marker.is_some() {
    marker::find_and_replace(...)
} else if expected.contains('\n') {
    matching::find_best_block_match(&lines, expected, options)
} else {
    matching::fuzzy_match_line(&lines, line_num, expected, options)
};
```

### 5.2 Marker Handling (`src/marker.rs`)

**Purpose:** Locate a marker comment and replace the associated AST node.

**Algorithm:**
1. Parse file, iterate over comments (using tree‑sitter query for comments).
2. Find comment containing `// PATCH-ME: <id>`.
3. Locate the next sibling node (statement, expression, or item) using tree‑sitter's `next_sibling`.
4. Replace that node's text range with new content.

### 5.3 Auto‑Repair Integration (`src/patch.rs`)

**Modification to `apply_literal_patch` (and similar functions):**
```rust
if !options.force && was_valid && !is_valid && !options.no_auto_repair {
    if let Ok(fixed) = repair::quick_balance(&new_content, language) {
        eprintln!("Warning: Patch introduced syntax error but was auto-repaired.");
        final_content = fixed;
        // Optionally record warning in JSON output
    } else {
        anyhow::bail!(original_error);
    }
}
```

### 5.4 Fuzzy Diff Application (`src/patch.rs`)

Extend `apply_unified_diff` to accept `fuzz_radius`. For each hunk, if exact context fails, invoke `matching::find_best_block_match` to locate the hunk's context lines, then apply the change relative to that offset.

### 5.5 Enhanced Diagnostics (`src/diagnostics.rs`)

Add fields to `JsonError`:
- `suggestion: Option<String>`
- `best_score: Option<f64>`
- `best_match_line: Option<usize>`
- `candidates: Option<Vec<Candidate>>`

Populate these fields in error constructors.

## 6. Architecture Decision Records (ADRs)

### ADR‑0001: Use Token‑Based Similarity for Multi‑Line Blocks

**Context:** Multi‑line fuzzy matching requires comparing blocks of code where formatting may differ. String similarity is insufficient (e.g., `foo()` vs `foo( )`).

**Decision Drivers:** ASR‑002 (low false positives), ASR‑001 (performance).

**Considered Options:**
- A) String similarity on concatenated lines.
- B) Token‑based Jaccard similarity using tree‑sitter tokenization.
- C) Full AST comparison (tree edit distance).

**Decision Outcome:** Choose Option B. It balances accuracy and performance, leveraging existing tree‑sitter parser. AST comparison (C) is too heavy for this use case.

**Consequences:**
- Positive: More robust to formatting changes.
- Negative: Requires tokenizing both expected and candidate blocks, adding some overhead (mitigated by limiting window size).

### ADR‑0002: Auto‑Repair Default On with Opt‑Out

**Context:** Should auto‑repair be enabled by default or require an explicit flag?

**Decision Drivers:** User experience (SN‑002 wants less manual work), safety (ASR‑004).

**Considered Options:**
- A) Default on, can disable with `--no-auto-repair`.
- B) Opt‑in with `--auto-repair`.

**Decision Outcome:** Choose Option A. The benefit of higher patch success rate outweighs the risk, especially since auto‑repair only triggers when original file was valid and the tool emits a clear warning. Users concerned about safety can use `--no-auto-repair`.

**Consequences:**
- Positive: Higher success rate out of the box.
- Negative: Potential for surprise if auto‑repair makes an unintended change (mitigated by warning and backup).

### ADR‑0003: Marker Syntax `// PATCH-ME: <id>`

**Context:** Need a standard marker comment format.

**Decision Drivers:** Simplicity, unlikelihood of collision with normal comments.

**Considered Options:**
- A) `// PATCH-ME: <id>`
- B) `// @patch <id>`
- C) `/* PATCH <id> */`

**Decision Outcome:** Choose Option A. It's explicit, unlikely to appear in normal code, and easy to parse.

## 7. API and Interface Contracts

No external APIs; CLI interface changes as documented in SRS Section 5.1.

## 8. Cross‑cutting Concerns

- **Observability:** Existing `log` and `env_logger` used; additional debug logs for matching decisions.
- **Error Handling:** All new fallible operations return `anyhow::Result` with context.
- **Testing:** New modules will have unit tests; integration tests will cover end‑to‑end scenarios.

## 9. Alternatives Considered

| Alternative | Reason Rejected |
|-------------|-----------------|
| Full AST diff/patch using tree edit distance | Too complex, performance overhead, and overkill for text‑based patching. |
| Using `similar` crate for diff fuzzy matching | Adds another dependency; can implement simpler fuzzy matching in‑house. |
| Making auto‑repair opt‑in | See ADR‑0002; default‑on better aligns with goal of higher success rate. |

## 10. Traceability

| ASR | Addressed By |
|-----|--------------|
| ASR‑001 | Token‑based matching limited to reasonable window; benchmark monitoring. |
| ASR‑002 | Similarity threshold (0.9 default) and tie‑breaking logic. |
| ASR‑003 | All new flags are additive; existing behavior unchanged. |
| ASR‑004 | Auto‑repair gated by `was_valid` check. |
```

---

## Document 5: Behavioral Specification & Test Verification (`/spec-test`)

**Reference:** `verification.md`

**Pre‑check:** SRS defines features; we'll elaborate BDD scenarios for key functional requirements.

```markdown
# Behavioral Specification & Test Verification Plan: patch-ts Enhancements

| Field | Value |
|-------|-------|
| Project | patch-ts-enhancements |
| Document | Test Verification Plan |
| Version | 0.1 (Draft) |
| Date | 2026-04-22 |
| Author | AI‑assisted specification |
| Status | Draft — Pending Review |

## 1. Behavioral Specifications (Specification by Example)

### Feature: Fuzzy Single‑Line Matching

**Scenario: Exact match within fuzz radius (success)**
```gherkin
Given a file with content:
  """
  line1
  line2
  line3
  line4
  line5
  """
When I apply a literal patch with:
  | --line | 3 |
  | --old  | "line3" |
  | --new  | "new line3" |
  | --fuzz | 2 |
Then the file content becomes:
  """
  line1
  line2
  new line3
  line4
  line5
  """
And the command exits with success.
```

**Scenario: Fuzzy match with whitespace variation**
```gherkin
Given a file with line "  line3  " (extra spaces)
When I apply a patch with expected "line3" (no spaces) and --fuzz 2
Then the patch succeeds and replaces the line with normalized matching.
```

**Scenario: No match above similarity threshold**
```gherkin
Given a file with lines: "apple", "banana", "cherry"
When I apply a patch with expected "orange" and --fuzz 5
Then the command fails with an error containing "no match found with similarity >= 0.9"
And JSON output includes "best_score" and "best_match_line".
```

**Scenario: Ambiguous match (tie)**
```gherkin
Given a file with two identical lines "target" at lines 10 and 20
When I apply a patch with expected "target" at line 15 with --fuzz 10
Then the command fails with an ambiguity error
And JSON output contains a "candidates" array with both line numbers.
```

### Feature: Multi‑Line Block Fuzzy Matching

**Scenario: Block shifted by several lines**
```gherkin
Given a file where a 3‑line function body appears at lines 42‑44 instead of 40‑42
When I apply a multi‑line patch with expected block content (3 lines) at line 40 with --fuzz 5
Then the system locates the correct block at lines 42‑44 and replaces it.
```

**Scenario: Block with formatting differences**
```gherkin
Given expected block:
  """
  fn foo() {
      println!("hi");
  }
  """
And actual block with extra spaces and comments:
  """
  fn foo() { // comment
      println!("hi");
  }
  """
When token‑based similarity is used
Then the match score is 1.0 and the patch succeeds.
```

### Feature: Marker‑Based Targeting

**Scenario: Replace function after marker**
```gherkin
Given a file containing:
  """
  // PATCH-ME: update-auth
  fn old_auth() { ... }
  """
When I run `patch-ts patch --file src/lib.rs --marker "update-auth" --new "fn new_auth() { ... }"`
Then the `old_auth` function is replaced with `new_auth`.
```

**Scenario: Marker not found**
```gherkin
When I run with --marker "nonexistent"
Then the command fails with "Marker 'nonexistent' not found".
```

**Scenario: Multiple markers with same ID**
```gherkin
Given a file with two `// PATCH-ME: dup` markers
When I run with --marker "dup"
Then the command fails with ambiguity error listing both locations.
```

### Feature: Auto‑Repair on Syntax Error

**Scenario: Patch introduces missing brace, auto‑repaired**
```gherkin
Given a valid file with "fn main() {}"
When I apply a patch that changes it to "fn main() {" (missing '}')
And --no-auto-repair is NOT used
Then the system balances delimiters and writes "fn main() {}\n"
And a warning is emitted: "Patch introduced syntax error but was auto-repaired."
And JSON output includes a "warning" field.
```

**Scenario: Auto‑repair fails, original error reported**
```gherkin
Given a valid file
When I apply a patch that introduces an unfixable syntax error (e.g., "fn main() { let x: = 1; }")
Then the command fails with the syntax error diagnostic
And no file is written.
```

**Scenario: Original file invalid, auto‑repair skipped**
```gherkin
Given a file with existing syntax error "fn main() {"
When I apply any patch that would trigger auto‑repair
Then the command fails with syntax error (no auto‑repair attempted).
```

### Feature: Fuzzy Unified Diff

**Scenario: Diff applies with fuzzy context**
```gherkin
Given a file where a line has an extra comment: "line2 // comment"
And a diff that expects exactly "line2"
When I apply the diff with `--diff --fuzz 3`
Then the hunk matches the line and applies successfully.
```

## 2. Test Strategy and Plan

### 2.1 Test Pyramid Stance

- **Unit Tests:** Extensive coverage for matching algorithms (`matching.rs`), marker detection, and auto‑repair logic.
- **Integration Tests:** End‑to‑end CLI tests using `assert_cmd` covering all new scenarios (as Gherkin above).
- **Acceptance/BDD Tests:** The Gherkin scenarios will be automated using `cucumber‑rust` or similar, running against the actual binary.
- **Performance Benchmarks:** Extend existing `criterion` benchmarks to measure latency impact of fuzzy matching and tokenization.

### 2.2 Test Environment

- Rust stable toolchain.
- Temporary directories for file operations (`tempfile`).
- Sample Rust files of varying sizes (fixtures).

### 2.3 Risk‑Based Test Prioritization

| Risk Area | Test Focus |
|-----------|------------|
| False positive fuzzy matches | Negative tests with similar but incorrect targets. |
| Auto‑repair correctness | Tests with various delimiter errors; ensure no unintended changes. |
| Backward compatibility | Full existing test suite must pass. |
| Performance regression | Benchmark comparison against baseline. |

## 3. Test Case Specifications (Scripted)

**Example Test Case: TC‑FUNC‑001‑01**

| Field | Value |
|-------|-------|
| ID | TC‑FUNC‑001‑01 |
| Title | Fuzzy match with whitespace normalization |
| Requirement | REQ‑FUNC‑002 |
| Preconditions | File contains line "  foo  " at line 3. |
| Steps | Invoke `patch-ts patch --file test.rs --line 1 --old "foo" --new "bar" --fuzz 5` |
| Expected Result | File updated with "bar" at line 3; exit code 0. |
| Automation | CLI integration test in `tests/cli_tests.rs`. |

## 4. NFR Verification Plans

### 4.1 Performance (NFR‑PERF‑001)
- **Test:** Benchmark suite comparing patch application time for 2000‑line file with and without fuzzy matching.
- **Tool:** `criterion`.
- **Threshold:** Average time increase ≤20%.

### 4.2 Reliability (NFR‑REL‑002)
- **Test:** Run a corpus of 100 AI‑generated patches with known correct targets. Measure false positive rate (incorrect location chosen).
- **Threshold:** <5% false positives.

### 4.3 Compatibility (NFR‑COMP‑001)
- **Test:** Run entire existing test suite (`cargo test`). All tests must pass.
- **Additional:** Manually verify that old CLI invocations produce identical results (except where enhancements intentionally change behavior, which must be documented).

## 5. Requirements Traceability Matrix (RTM Excerpt)

| Requirement ID | BDD Scenario(s) | Test Case ID(s) |
|----------------|-----------------|-----------------|
| REQ‑FUNC‑001 | Fuzzy single‑line match success | TC‑FUNC‑001‑01, CLI‑fuzzy‑exact |
| REQ‑FUNC‑002 | Fuzzy match with whitespace variation | TC‑FUNC‑001‑02 |
| REQ‑FUNC‑004 | Ambiguous match (tie) | CLI‑fuzzy‑ambiguous |
| REQ‑FUNC‑005 | Multi‑line block shifted | TC‑FUNC‑005‑01 |
| REQ‑FUNC‑010‑014 | Marker replacement, not found, multiple | CLI‑marker‑* |
| REQ‑FUNC‑020‑024 | Auto‑repair success, failure, skip | CLI‑auto‑repair‑* |
| REQ‑FUNC‑030‑032 | Fuzzy diff application | CLI‑diff‑fuzzy |
| NFR‑PERF‑001 | - | bench‑patch‑latency |

## 6. Living Documentation Strategy

- Feature files (Gherkin) stored in `tests/features/` alongside code.
- CI runs `cucumber` and publishes HTML report as artifact.
- JSON test results integrated with existing tooling.
```

---

All five specification documents have been generated and saved to `/mnt/user-data/outputs/patch-ts-enhancements-{document}.md`. These documents form a complete, traceable specification suite for implementing the identified improvements.

Would you like me to elaborate on any particular section, or shall we proceed to planning the implementation sprints?
