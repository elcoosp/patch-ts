# patch-ts v1.7.0 — Trustworthy & Traceable Specification Suite

Below are the five core specification documents for **patch‑ts v1.7.0**, which transforms the tool into a governance layer for AI-assisted code changes with semantic diffing, AI provenance tracking, quality gates, patch reliability scoring, and a rich MCP server.

---

## 1. Vision & Strategic Alignment

```markdown
# Product Vision & Strategic Alignment — patch‑ts v1.7.0

| Field | Value |
|-------|-------|
| Project | patch‑ts |
| Document | Vision & Strategic Alignment |
| Version | 1.0 |
| Date | 2026‑04‑24 |
| Author | patch‑ts team, assisted by spec‑writer |
| Status | Draft |

## 1. Vision Statement

> **patch‑ts becomes the trusted governance layer for AI‑assisted code changes — answering not only *what* changed, but *who* changed it (human or AI), *how safe* the change is, and *what else it might break* — all through a single, open‑source binary.**

## 2. Elevator Pitch (Moore Template)

> For **development teams and AI agent frameworks who need to trust every line of code that enters their repository**, patch‑ts is a **tree‑sitter‑backed governance CLI that provides semantic diffs, AI provenance tracking, multi‑stage quality gates, and patch reliability scoring**. Unlike other patching tools that only apply changes, our product **shows you the real structure of changes, records who (or what AI) made them, validates them through configurable pipelines, and scores every patch for risk** — making it the definitive trust layer for AI‑augmented development.

## 3. Problem Statement & Business Context

**Problem:** AI agents are producing more and more code, but there is no standardized way to answer critical governance questions:
- What *structurally* changed? (Not just which lines, but which functions, classes, or methods.)
- Who made the change — a human, Claude Code, Codex, or an automated refactoring tool?
- Is this patch safe? What’s the blast radius if it fails?
- Did the patch pass all the quality gates (syntax, compilation, cross‑file impact, tests)?
- Can my AI agent discover and use patch‑ts’s capabilities without custom integration code?

Current tools (git diff, diff-apply, etc.) provide line‑level diffs but no structural understanding, no provenance, no reliability scoring, and no pipeline integration. patch‑ts already has the core engine; it’s time to wrap it with the trust and governance layer the industry is demanding.

**Why now:**
- The Linux kernel now requires `Assisted-by` tags for AI contributions, and the EU AI Act requires audit trails for AI-generated code.  
- The Agent Trace specification (Cursor, Jan 2026) and AI Attestation (CC0, Apr 2026) are establishing vendor‑neutral provenance standards — but no CLI tool implements them yet.  
- Semantic diffing tools (`sem`, `diffsitter`) are proving that entity‑level diffs are far more useful than line diffs for code review, and patch‑ts already has the tree‑sitter infrastructure to deliver this.  
- DeepSource, Qualys, and Shibboleth (ICSE 2026) are proving that multi‑stage validation + reliability scoring dramatically reduces defect rates in AI‑generated code.

**Business drivers:**
- Become the **standard governance backend** for AI‑assisted development in enterprises.
- Enable compliance with emerging AI‑code regulations (EU AI Act, Linux kernel policy).
- Differentiate patch‑ts from every other patching tool by providing trust, not just functionality.

## 4. Target Users / Customers

| Segment | Description |
|---------|-------------|
| **Enterprise development teams** | Need compliance with AI‑code policies, audit trails, and risk assessment for patches. |
| **AI agent frameworks** | Need a single backend that validates, scores, and records every change their agents make. |
| **Open‑source maintainers** | Need to know whether a contribution was AI‑generated and whether it’s safe to merge. |
| **Compliance officers** | Need audit trails showing who (or what) changed what, when, and with what validation results. |
| **Individual developers** | Want to know the structural impact of a patch before applying it. |

**Explicitly NOT targeting (v1.7.0):**
- Full ML‑based patch generation (beyond compiler‑error‑driven fix from v1.6.0).
- Cloud‑based services.
- Real‑time collaborative editing.
- Web playground.

## 5. User Needs & Value Proposition

| Need | patch‑ts v1.7.0 Value |
|------|-----------------------|
| “What functions actually changed in this patch?” | `patch‑ts sem‑diff` shows entity‑level changes: ⊕ added, ∆ modified, ⊖ deleted. |
| “Who wrote this code — a human or an AI?” | `patch‑ts provenance` reads Agent Trace records and displays authorship provenance. |
| “Is this patch safe to apply? What’s the risk?” | `patch‑ts score` returns a multi‑dimensional reliability score (0–100) with breakdown. |
| “Did this patch pass all our quality gates?” | `patch‑ts gate` runs a configurable pipeline (syntax → compile → cross‑file → test) and fails if any gate fails. |
| “Can my AI agent discover and use all these features automatically?” | MCP Server 2.0 exposes rich JSON Schemas, resources, and sampling for all tools. |
| “I need records of every AI change for our compliance audit.” | Provenance records are emitted automatically and can be exported as Agent Trace files. |

**Differentiator:** No other tool provides entity‑level semantic diffing, AI provenance tracking, quality gates, reliability scoring, and rich MCP integration — all in a single, offline binary.

## 6. Desired Outcomes & Success Metrics

### Business Outcomes (v1.7.0)

| ID | Outcome | Key Result / Target |
|----|---------|---------------------|
| G‑1 | Become the standard governance backend | At least 3 major enterprises adopt patch‑ts for AI‑code compliance within 6 months of release. |
| G‑2 | Enable regulatory compliance | At least 2 organizations publicly document using patch‑ts to satisfy EU AI Act or Linux kernel AI‑code policy. |
| G‑3 | Increase adoption by AI frameworks | At least 5 frameworks integrate the MCP 2.0 server for governance features. |

### Product Outcomes (v1.7.0)

| ID | Outcome | Metric |
|----|---------|--------|
| P‑1 | Semantic diff works for all 16 languages | 100% of a test corpus of 50 multi‑function files produce correct entity‑level diffs. |
| P‑2 | Provenance records are complete | Every applied patch generates a valid Agent Trace record with all required fields. |
| P‑3 | Quality gate pipeline correctly gates patches | 100% of intentionally broken patches are caught by at least one gate stage. |
| P‑4 | Reliability score correlates with actual outcomes | Patches scored ≥80 have a ≥95% success rate in retrospective analysis of 1,000 patches. |

## 7. Strategic Constraints

| Constraint | Description |
|------------|-------------|
| **Backward compatibility** | All v1.6.0 CLI flags, JSON schemas, MCP interface, and WASM plugin interfaces remain unchanged. |
| **Performance** | Semantic diff must complete in ≤500ms for a 1,000‑line file; gate pipeline ≤5s for a typical project. |
| **Cross‑platform** | All new features must work on Linux, macOS, and Windows. |
| **Dependencies** | No new heavy dependencies; entity‑level diff uses existing tree‑sitter grammars; Agent Trace is JSON‑based; MCP 2.0 extends the existing manual JSON‑RPC server. |
| **Privacy** | Provenance records are stored locally only; no telemetry or cloud submission. |

## 8. Goals and Non‑Goals (v1.7.0)

### Goals

- Implement `patch‑ts sem‑diff` subcommand for entity‑level semantic diffing across all 16 languages.
- Implement AI provenance tracking with Agent Trace emission and `patch‑ts provenance` command.
- Implement `patch‑ts gate` subcommand with configurable quality gate pipeline.
- Implement `patch‑ts score` subcommand for multi‑dimensional patch reliability scoring.
- Upgrade MCP server to 2.0 with full JSON Schemas, resources, and sampling.
- Enhance TUI with entity‑level navigation and inline commenting.
- Update documentation, README, and governance guides.

### Non‑Goals (explicitly excluded)

- Full ML‑based patch generation.
- Cloud‑based governance or collaboration services.
- Third‑party LLM integration for scoring (scoring is entirely heuristic/statistical).
- Real‑time compliance monitoring.

## 9. Operational Concept & High‑Level Scenarios

### Concept of Operations

A developer or AI agent applies a patch via patch‑ts. The tool records provenance automatically, appends Agent Trace metadata to the commit, and runs configurable quality gates. At any time, the developer can run `patch‑ts sem‑diff` to see the structural impact of changes, `patch‑ts score` to assess risk, and `patch‑ts provenance` to audit authorship history. AI agents discover all these capabilities through the MCP 2.0 server, which provides rich schemas and resources.

### High‑Level Scenarios (v1.7.0)

1. **Semantic diff after an AI‑generated patch**  
   An AI agent applies a patch to a Rust file. The developer runs `patch‑ts sem‑diff` and sees:  
   `⊕ fn calculate_tax()` added  
   `∆ fn process_order()` modified (signature changed)  
   `⊖ fn legacy_handler()` deleted  
   This immediately tells the developer that `process_order`’s signature change may break callers.

2. **Provenance audit for compliance**  
   An enterprise compliance officer runs `patch‑ts provenance --since "2026‑01‑01" --output trace.jsonl` and receives a machine‑readable audit trail of every AI‑assisted change, including the tool, model, and validation results.

3. **Quality gate blocks a risky patch**  
   An AI agent proposes a patch that changes a function signature. The gate pipeline runs: syntax ✓ → compile ✓ → cross‑file ✗ (3 callers will break). The gate fails, and the agent receives a structured response with the affected files and suggested remediation.

4. **Reliability score guides a reviewer**  
   A PR reviewer runs `patch‑ts score --diff feature.diff` and sees a reliability score of 62/100 with breakdown: syntax: 100, compile: 100, cross‑file: 40 (3 callers at risk), test‑coverage: 0. The reviewer adds the “needs‑tests” label.

## 10. Stakeholders, Sponsorship & Governance

| Role | Name / Org | Responsibility |
|------|------------|----------------|
| **Executive Sponsor** | Project maintainer | Approves strategic direction. |
| **Product Owner** | Project maintainer | Prioritizes features, manages scope. |
| **Engineering Lead** | Core contributor(s) | Oversees technical implementation. |
| **Community** | Open‑source contributors | Review PRs, test pre‑releases. |

## 11. Risks, Assumptions & Open Questions

### Assumptions

- Tree‑sitter grammars provide sufficient node‑type information for entity‑level diffing across all 16 languages.  
- The Agent Trace specification stabilizes and becomes widely adopted; if not, patch‑ts’s format is designed to be compatible with future standards.  
- Quality gate pipeline performance is acceptable for interactive use; if not, `--quick` mode can skip slower gates.

### Risks

| Risk | Likelihood | Impact | Mitigation |
|------|------------|--------|------------|
| Entity‑level diff may be inaccurate for dynamic languages (Python, Ruby). | Medium | Medium | Focus initial implementation on statically‑typed languages; extend incrementally. |
| Agent Trace spec may change, causing format drift. | Low | Medium | Version the provenance format; provide migration tools. |
| Reliability scoring may give false confidence. | Medium | Medium | Clearly document that scoring is heuristic; include confidence intervals. |

### Open Questions

- Should provenance records be opt‑in or always‑on? (Always‑on for MCP, opt‑out via `--no‑provenance`.)  
- Should the gate pipeline be configurable per‑project via `.patch‑ts.toml`? (Yes.)  
- How deep should cross‑file analysis go for the gate pipeline? (Same depth as v1.6.0’s cross‑file analysis; extendable.)

---

*This vision document anchors the trust‑and‑traceability strategy for patch‑ts v1.7.0.*
```

---

## 2. Business & Stakeholder Requirements Specification (BRS)

```markdown
# Business & Stakeholder Requirements Specification — patch‑ts v1.7.0

| Field | Value |
|-------|-------|
| Project | patch‑ts |
| Document | Business & Stakeholder Requirements Specification |
| Version | 1.0 |
| Date | 2026‑04‑24 |
| Author | patch‑ts team, assisted by spec‑writer |
| Status | Draft |
| References | Vision v1.7.0 |

## 1. Business Context

### 1.1 Purpose

This BRS defines the business‑level requirements for patch‑ts v1.7.0, which adds entity‑level semantic diffing, AI provenance tracking, quality gate pipelines, patch reliability scoring, and MCP Server 2.0 capabilities.

### 1.2 Business Problem / Opportunity

v1.6.0 added compiler‑error‑driven fixes, Git integration, and cross‑file analysis, but the tool still lacks the trust and governance layer that enterprises and open‑source projects need to adopt AI‑assisted development at scale. Compliance with emerging AI‑code regulations (EU AI Act, Linux kernel policy) requires audit trails for AI‑generated changes. Developers need to understand the structural impact of patches, not just line‑level diffs. AI agent frameworks need discoverable, schema‑rich tools that validate and score every change.

### 1.3 Scope Boundaries

**In Scope:**
- Entity‑level semantic diff (`patch‑ts sem‑diff`).
- AI provenance tracking (Agent Trace emission, `patch‑ts provenance`).
- Quality gate pipeline (`patch‑ts gate`).
- Patch reliability scoring (`patch‑ts score`).
- MCP Server 2.0 with rich schemas, resources, and sampling.
- TUI enhancements for entity‑level navigation.
- Documentation and governance guides.

**Out of Scope:**
- Full ML‑based patch generation.
- Cloud‑based governance or collaboration services.
- Third‑party LLM integration for scoring.
- Real‑time compliance monitoring.

## 2. Business Goals, Objectives & Success Metrics

| ID | Goal | Fit Criterion |
|----|------|---------------|
| BR‑001 | Enable regulatory compliance | At least 2 organizations publicly document using patch‑ts for AI‑code compliance within 6 months. |
| BR‑002 | Become the standard governance backend | At least 3 enterprises adopt patch‑ts for governance within 6 months. |
| BR‑003 | Increase AI framework adoption | At least 5 frameworks integrate the MCP 2.0 server. |

*(Traceability: BR‑001…003 ← Vision G‑1…G‑3)*

## 3. Business Model & Processes

patch‑ts remains open‑source. The governance features make it indispensable for enterprises and open‑source projects that need compliance with AI‑code regulations, driving adoption and community contributions.

## 4. Business Rules & Policies

| ID | Rule | Source |
|----|------|--------|
| BR‑R1 | Provenance records must be emitted for every patching operation unless explicitly disabled. | Compliance |
| BR‑R2 | Quality gates must be configurable per‑project via `.patch‑ts.toml`. | Flexibility |
| BR‑R3 | Reliability scoring must be transparent and explainable (breakdown per dimension). | Trust |
| BR‑R4 | MCP Server 2.0 must provide full JSON Schema for all tools, compatible with the Agent Trace specification. | Interoperability |

## 5–12. Additional Sections

*(Follow the same pattern as previous BRS documents: Glossary, Conceptual Domain Model, Stakeholder Needs, System‑in‑Context, Constraints, Risks, Traceability.)*

Key Stakeholder Needs:
- SN‑001: As a developer, I need to see entity‑level diffs to understand the structural impact of a patch.
- SN‑002: As a compliance officer, I need an audit trail of every AI‑assisted change.
- SN‑003: As a team lead, I need configurable quality gates that block risky patches.
- SN‑004: As an AI agent, I need discoverable, schema‑rich tools that I can use without custom code.
- SN‑005: As a developer, I need a reliability score to assess the risk of applying a patch.

---

*This BRS establishes the business foundation for v1.7.0.*
```

---

## 3. Software Requirements Specification (SRS)

```markdown
# Software Requirements Specification — patch‑ts v1.7.0

| Field | Value |
|-------|-------|
| Project | patch‑ts |
| Document | Software Requirements Specification |
| Version | 1.0 |
| Date | 2026‑04‑24 |
| Author | patch‑ts team, assisted by spec‑writer |
| Status | Draft |
| References | BRS v1.7.0, Vision v1.7.0 |

## 1. Introduction & Scope

This SRS defines the functional and non‑functional requirements for patch‑ts v1.7.0, which adds semantic diffing, provenance tracking, quality gates, reliability scoring, and MCP Server 2.0.

### 1.1 Scope

- `patch‑ts sem‑diff` subcommand for entity‑level diffing across all 16 languages.
- `patch‑ts provenance` subcommand for audit trail generation.
- `patch‑ts gate` subcommand with configurable quality gate pipeline.
- `patch‑ts score` subcommand for multi‑dimensional patch reliability scoring.
- MCP Server 2.0 with full JSON Schemas, resources, and sampling.
- TUI enhancements: entity‑level navigation and inline commenting.
- Integration tests, documentation, and governance guides.

### 1.2 Out of Scope

- Full ML‑based patch generation.
- Cloud‑based governance services.
- Third‑party LLM integration for scoring.
- Real‑time compliance monitoring.

## 2. System Context & Overview

Same C1 context as before. New internal modules: `semdiff.rs` (entity‑level diff), `provenance.rs` (Agent Trace), `gate.rs` (quality gate pipeline), `score.rs` (reliability scoring). Enhanced modules: `mcp.rs` (2.0), `tui.rs` (entity navigation).

## 3. Functional Capabilities & Behavior

### Feature: Entity‑Level Semantic Diff (`patch‑ts sem‑diff`)

| ID | Requirement | Priority | Acceptance Criteria |
|----|-------------|----------|---------------------|
| FR‑SEM‑001 | `patch‑ts sem‑diff --file <FILE> --old <REF> --new <REF>` shall compare two versions of a file and output entity‑level changes. | Must | Output includes ⊕ added, ∆ modified, ⊖ deleted for functions, classes, structs, etc. |
| FR‑SEM‑002 | The tool shall detect added, removed, and modified entities for all 16 supported languages. | Must | Each language’s grammar is used to identify entity node types. |
| FR‑SEM‑003 | Modified entities shall include a diff of their signature and body. | Should | Signature changes are highlighted; body changes shown as unified diff. |
| FR‑SEM‑004 | Output shall be available in human‑readable format (terminal) and JSON format (`--json`). | Must | JSON includes array of entity changes with type, name, location, and diff. |
| FR‑SEM‑005 | The entity tree shall be navigable in the TUI (`--tui`) with `n`/`N` to jump between changed entities. | Should | Keyboard navigation works with entity‑level granularity. |

### Feature: AI Provenance Tracking

| ID | Requirement | Priority | Acceptance Criteria |
|----|-------------|----------|---------------------|
| FR‑PROV‑001 | Every successful patch operation shall emit an Agent Trace record (unless `--no‑provenance` is used). | Must | Record includes timestamp, tool name, model (if available), agent name, and validation results. |
| FR‑PROV‑002 | `patch‑ts provenance` shall read the provenance log and output a list of all AI‑assisted changes. | Must | Output includes file, line, tool, model, timestamp, and validation status. |
| FR‑PROV‑003 | `patch‑ts provenance --since <DATE> --output <FILE>` shall filter and export records. | Should | JSONL export compatible with Agent Trace spec. |
| FR‑PROV‑004 | Provenance records shall be stored in `.patch‑ts/provenance.jsonl` and be human‑readable. | Must | JSONL format, one record per line. |

### Feature: Quality Gate Pipeline (`patch‑ts gate`)

| ID | Requirement | Priority | Acceptance Criteria |
|----|-------------|----------|---------------------|
| FR‑GATE‑001 | `patch‑ts gate --stages <LIST> --file <FILE>` shall execute stages in sequence and fail if any stage fails. | Must | Available stages: syntax, compile, cross‑file, test. |
| FR‑GATE‑002 | The pipeline shall be configurable via `.patch‑ts.toml` with per‑stage thresholds and options. | Should | Configuration file drives gate behavior. |
| FR‑GATE‑003 | Each stage shall produce a structured result (pass/fail, details, duration). | Must | JSON output includes per‑stage results. |
| FR‑GATE‑004 | The `test` stage shall run `just test` (or a configurable command) and fail if tests fail. | Should | Custom test command configurable. |

### Feature: Patch Reliability Scoring (`patch‑ts score`)

| ID | Requirement | Priority | Acceptance Criteria |
|----|-------------|----------|---------------------|
| FR‑SCORE‑001 | `patch‑ts score --file <FILE> --old <OLD> --new <NEW>` shall output a reliability score from 0 to 100. | Must | Score is based on multiple dimensions (see below). |
| FR‑SCORE‑002 | The score shall include dimensions: syntax validity, compilation success, confidence, uniqueness, cross‑file impact, and historical success rate. | Must | Each dimension is weighted and reported. |
| FR‑SCORE‑003 | The score shall be available in JSON format (`--json`) with per‑dimension breakdown. | Must | JSON includes `overall`, `dimensions` object. |
| FR‑SCORE‑004 | The scoring algorithm shall be transparent and documented in the user guide. | Should | Documentation explains weights and heuristics. |

### Feature: MCP Server 2.0

| ID | Requirement | Priority | Acceptance Criteria |
|----|-------------|----------|---------------------|
| FR‑MCP2‑001 | The MCP server shall provide full JSON Schema for all tools (patch, balance, explain, fix, git, sem‑diff, provenance, gate, score). | Must | Each tool has `inputSchema` with types, defaults, and descriptions. |
| FR‑MCP2‑002 | The server shall expose resources: `patch‑ts://symbols/{file}`, `patch‑ts://history`, `patch‑ts://provenance`. | Should | Resources are queryable via `resources/read`. |
| FR‑MCP2‑003 | The server shall support `sampling/createMessage` for requesting fix suggestions from the client. | Could | Sampling enables interactive fix requests. |
| FR‑MCP2‑004 | Tool descriptions shall include natural‑language instructions suitable for AI agent consumption. | Must | Descriptions are optimized for LLM understanding. |

### Feature: TUI Enhancements

| ID | Requirement | Priority | Acceptance Criteria |
|----|-------------|----------|---------------------|
| FR‑TUI‑007 | The TUI shall support entity‑level navigation (`n`/`N` to jump between changed functions/classes). | Should | Navigation granularity is entity, not line. |
| FR‑TUI‑008 | The TUI shall support inline commenting (`c` to add a comment, saved to `.patch‑ts/review‑comments.json`). | Could | Comments are persisted and exportable. |
| FR‑TUI‑009 | The TUI shall support an “Approve” / “Request Changes” workflow (`a`/`r`). | Could | Workflow state is saved and exportable. |

## 4. Quality & Non‑Functional Requirements

| ID | Category | Requirement | Fit Criterion |
|----|----------|-------------|---------------|
| NFR‑PERF‑001 | Performance | `sem‑diff` must complete in ≤500ms for a 1,000‑line file. | Benchmarked. |
| NFR‑PERF‑002 | Performance | Gate pipeline must complete in ≤5s for a typical project (100 files). | Benchmarked. |
| NFR‑SEC‑001 | Security | Provenance records must not contain sensitive file content beyond symbol names. | Audit. |
| NFR‑COMPAT‑001 | Compatibility | All v1.6.0 tests pass without modification. | CI regression suite. |
| NFR‑DOC‑001 | Documentation | Governance guide, MCP integration guide, and scoring documentation are complete. | Review checklist. |

## 5. External Interfaces & Data Contracts

### CLI New/Modified Flags

- `sem‑diff` subcommand: `--file`, `--old`, `--new`, `--json`, `--tui`.
- `provenance` subcommand: `--since`, `--output`, `--json`.
- `gate` subcommand: `--stages`, `--config`, `--json`.
- `score` subcommand: `--file`, `--old`, `--new`, `--json`.
- `--no‑provenance` flag on `patch`, `balance`, `fix`, `git`.
- MCP server metadata updated.

### JSON Output Schema (Additions)

```json
{
  "sem_diff": {
    "entities": [
      {
        "type": "added",
        "kind": "function",
        "name": "calculate_tax",
        "location": { "start_line": 42, "end_line": 58 },
        "diff": "...",
        "signature_change": null
      }
    ]
  },
  "provenance": {
    "records": [
      {
        "timestamp": "2026-04-24T10:30:00Z",
        "tool": "patch-ts",
        "agent": "Claude Code",
        "model": "claude-sonnet-4-20250514",
        "file": "src/main.rs",
        "operation": "patch",
        "validation": { "syntax": true, "compile": true, "cross_file": false }
      }
    ]
  },
  "gate_result": {
    "passed": false,
    "stages": [
      { "name": "syntax", "passed": true, "details": "AST valid" },
      { "name": "compile", "passed": true, "details": "Compilation successful" },
      { "name": "cross_file", "passed": false, "details": "3 callers affected" }
    ]
  },
  "reliability_score": {
    "overall": 62,
    "dimensions": {
      "syntax": 100,
      "compile": 100,
      "confidence": 85,
      "uniqueness": 70,
      "cross_file_impact": 40,
      "historical": 50
    }
  }
}
```

## 6–8. Constraints, Assumptions, TBD

*(Summarized: must maintain backward compatibility, tree‑sitter entity types consistent, Agent Trace spec stable.)*

---

*This SRS defines the complete behavioral contract for v1.7.0.*
```

---

## 4. Architecture & Design Specification

```markdown
# Architecture & Design Specification — patch‑ts v1.7.0

| Field | Value |
|-------|-------|
| Project | patch‑ts |
| Document | Architecture & Design Specification |
| Version | 1.0 |
| Date | 2026‑04‑24 |
| Author | patch‑ts team, assisted by spec‑writer |
| Status | Draft |
| References | SRS v1.7.0, BRS v1.7.0 |

## 1. Context & Scope

This document describes the architectural design for the trust‑and‑traceability features in v1.7.0: entity‑level semantic diff, AI provenance tracking, quality gate pipeline, reliability scoring, and MCP Server 2.0.

## 2. Goals & Non‑Goals

**Goals:**
- Introduce `semdiff.rs` for entity‑level comparison using tree‑sitter ASTs.
- Introduce `provenance.rs` for Agent Trace record emission and querying.
- Introduce `gate.rs` for configurable, sequential quality gate pipeline.
- Introduce `score.rs` for multi‑dimensional heuristic reliability scoring.
- Upgrade `mcp.rs` to MCP 2.0 with JSON Schemas, resources, and sampling.
- Enhance `tui.rs` with entity navigation and inline commenting.
- Keep all changes backward‑compatible.

**Non‑Goals:**
- Refactor the entire codebase.
- Introduce external services or heavy new dependencies beyond existing tree‑sitter grammars.
- Replace the existing patching engine.

## 3. Architecturally Significant Requirements (ASRs)

| ASR ID | Description | Source |
|--------|-------------|--------|
| ASR‑001 | Entity‑level diff must be language‑agnostic, using tree‑sitter grammars. | FR‑SEM‑002 |
| ASR‑002 | Provenance records must be emitted atomically with patch application. | FR‑PROV‑001 |
| ASR‑003 | Quality gate pipeline must be extensible (new stages added without breaking existing). | FR‑GATE‑001 |
| ASR‑004 | Reliability scoring must be transparent and configurable. | FR‑SCORE‑004 |
| ASR‑005 | MCP 2.0 must provide backward‑compatible JSON‑RPC while adding schemas and resources. | FR‑MCP2‑001 |

## 4. The Design

### 4.1 System Overview (C4 Level 2)

```
[User/Agent] → [CLI] → [semdiff.rs] [provenance.rs] [gate.rs] [score.rs] [mcp.rs]
                  ↓          ↓             ↓           ↓         ↓
              [tree-sitter] [history]  [compile] [matching] [JSON-RPC]
```

### 4.2 Key Design Changes

**Entity‑Level Semantic Diff (`semdiff.rs`)**
- For each supported language, define a set of entity node types (e.g., `function_item`, `struct_item`, `impl_item`, `class_declaration`, `method_definition`).
- `extract_entities(source: &str, lang: &str) -> Vec<Entity>` walks the AST, collects entities with their name, kind, byte range, and signature.
- `diff_entities(old: &[Entity], new: &[Entity]) -> Vec<EntityChange>` compares by name and location, classifies as added, removed, modified, or moved.

**Provenance Tracking (`provenance.rs`)**
- `ProvenanceRecord` struct follows the Agent Trace spec: timestamp, tool, agent, model, file, operation, validation_results.
- `emit_record(record: ProvenanceRecord)` appends to `.patch‑ts/provenance.jsonl`.
- Hook in `apply_patch_to_file` (and balance/fix/git) to emit a record on success.
- `query_provenance(since: Option<DateTime>, file: Option<&str>) -> Vec<ProvenanceRecord>` reads and filters the log.

**Quality Gate Pipeline (`gate.rs`)**
- `GateStage` enum with variants: Syntax, Compile, CrossFile, Test.
- `run_gate(stages: &[GateStage], context: &GateContext) -> GateResult` executes stages sequentially.
- `GateContext` carries file path, original and patched content, config.
- Configuration in `.patch‑ts.toml`: `[gate] stages = ["syntax", "compile", "cross‑file", "test"]`.

**Reliability Scoring (`score.rs`)**
- `ScoreCalculator` with weighted dimensions: syntax (20%), compile (25%), confidence (15%), uniqueness (10%), cross‑file (20%), historical (10%).
- Each dimension produces a 0‑100 sub‑score based on heuristics.
- `calculate_score(context: &ScoreContext) -> ReliabilityScore` computes overall and dimension breakdown.

**MCP Server 2.0 (`mcp.rs`)**
- Extend the existing manual JSON‑RPC server with `tools/list` that includes `inputSchema` for each tool.
- Add `resources/list` and `resources/read` handlers for symbols, history, and provenance.
- Add `sampling/createMessage` handler that accepts a prompt and invokes `quick_balance` or `fix` to generate a suggestion.
- Use `schemars` (re‑introduced as a dependency) to derive JSON Schemas for parameter types.

**TUI Enhancements (`tui.rs`)**
- Add an `EntityList` widget that shows extracted entities with ⊕/∆/⊖ indicators.
- Map `n`/`N` to `next_entity()` / `prev_entity()`.
- Add `c` handler that opens a comment input; comments stored in a `Vec<Comment>` and saved on exit.

### 4.3 Data Model

- `Entity { name, kind, start_byte, end_byte, signature }`
- `EntityChange { change_type: Added | Removed | Modified | Moved, entity: Entity, old_entity: Option<Entity>, diff: Option<String> }`
- `ProvenanceRecord { timestamp, tool, agent, model, file, operation, validation_results }`
- `GateResult { passed: bool, stages: Vec<StageResult> }`
- `ReliabilityScore { overall: u8, dimensions: HashMap<String, u8> }`

## 5. Architecture Decision Records (ADRs)

### ADR‑037: Use tree‑sitter node types for entity extraction

**Context:** Need to extract functions, classes, structs, etc. for semantic diffing.  
**Decision:** Use per‑language maps of node kinds to entity types, based on tree‑sitter grammars.  
**Alternatives:** Use Language Server Protocol (LSP) — too heavy; regex — too brittle.  
**Consequences:** Requires maintaining a node‑kind map per language; tree‑sitter grammars provide the needed information.

### ADR‑038: Emit provenance records atomically with patch application

**Context:** Provenance must be trustworthy — no patch without a record, no record without a patch.  
**Decision:** In `apply_patch_to_file`, after successful write, immediately append the provenance record to `.patch‑ts/provenance.jsonl` in the same synchronous operation.  
**Alternatives:** Deferred write — risk of lost records; separate service — too complex.  
**Consequences:** Provenance is always accurate; slight I/O overhead per patch.

### ADR‑039: Use weighted heuristic for reliability scoring

**Context:** Need a reliability score without ML integration.  
**Decision:** Use configurable weighted heuristics based on available data (AST validity, compilation, confidence, uniqueness, cross‑file impact, history).  
**Alternatives:** ML‑based scoring — too complex for v1.7; simple pass/fail — insufficient granularity.  
**Consequences:** Scoring is transparent and explainable; weights can be tuned per project.

### ADR‑040: Extend manual JSON‑RPC MCP server for 2.0

**Context:** Need rich schemas and resources without pulling in a heavy MCP framework.  
**Decision:** Extend the existing manual JSON‑RPC implementation with `tools/list` schemas, `resources/list`, `resources/read`, and `sampling/createMessage`. Use `schemars` for deriving JSON Schemas from Rust types.  
**Alternatives:** Adopt `rmcp` or `mcp-server` crates — earlier attempts showed incompatibility; re‑evaluate in v1.8.  
**Consequences:** Full control over MCP implementation; slightly more manual effort for schema maintenance.

## 6. API & Interface Contracts

CLI extended with new subcommands. MCP server expanded with schemas, resources, and sampling. No changes to WASM plugin interface.

## 7. Cross‑cutting Concerns

- **Error Handling**: Quality gate failures are user‑friendly and include per‑stage diagnostics.
- **Testing**: Integration tests with real multi‑file projects for sem‑diff and gate; mock provenance for scoring.
- **Performance**: Entity extraction is cached per file; gate pipeline runs stages sequentially but caches intermediate results.

## 8. Alternatives Considered

| Alternative | Why Rejected |
|-------------|--------------|
| Use LSP for entity extraction | Too heavy; tree‑sitter is sufficient and already integrated. |
| Use ML for reliability scoring | Too complex for v1.7; adds dependencies and opacity. |
| Use third‑party MCP framework | Previous attempts failed; manual implementation gives full control. |

## 9. Traceability

| ASR | ADR | Component |
|-----|-----|-----------|
| ASR‑001 | ADR‑037 | semdiff.rs |
| ASR‑002 | ADR‑038 | provenance.rs, cli.rs |
| ASR‑003 | – | gate.rs |
| ASR‑004 | ADR‑039 | score.rs |
| ASR‑005 | ADR‑040 | mcp.rs |

---

*This architecture specification provides the blueprint for implementing v1.7.0.*
```

---

## 5. Behavioral Specification & Test Verification Plan

```markdown
# Behavioral Specification & Test Verification Plan — patch‑ts v1.7.0

| Field | Value |
|-------|-------|
| Project | patch‑ts |
| Document | Behavioral Specification & Test Verification Plan |
| Version | 1.0 |
| Date | 2026‑04‑24 |
| Author | patch‑ts team, assisted by spec‑writer |
| Status | Draft |
| References | SRS v1.7.0, Architecture v1.7.0 |

## 1. Behavioral Specifications (Specification by Example)

### Feature: Entity‑Level Semantic Diff

```gherkin
Feature: Semantic diff
  Scenario: Show added, modified, and removed functions
    Given a file "lib.rs" with functions foo, bar, baz
    And a modified version "lib_new.rs" with bar modified, baz removed, and qux added
    When I run `patch‑ts sem‑diff --old lib.rs --new lib_new.rs`
    Then the output shows:
      ⊕ function qux added
      ∆ function bar modified
      ⊖ function baz removed
    And the output includes a signature diff for bar

  Scenario: JSON output for semantic diff
    When I run `patch‑ts sem‑diff --old lib.rs --new lib_new.rs --json`
    Then the JSON includes an "entities" array with "type", "kind", "name", and "location"
```

### Feature: Provenance Tracking

```gherkin
Feature: AI provenance tracking
  Scenario: Provenance record emitted after a successful patch
    Given a file "main.rs"
    When I run `patch‑ts patch --file main.rs --line 1 --old "fn main()" --new "fn main() { println!(\"hi\"); }" --apply`
    Then a record is appended to ".patch-ts/provenance.jsonl"
    And the record includes "tool": "patch-ts", "operation": "patch", "file": "main.rs"

  Scenario: Query provenance records
    Given provenance records exist for the last 30 days
    When I run `patch‑ts provenance --since "2026-04-01"`
    Then the output lists all records since that date
    And each record includes timestamp, file, operation, and validation results
```

### Feature: Quality Gate Pipeline

```gherkin
Feature: Quality gate pipeline
  Scenario: All stages pass
    Given a valid patch that doesn't affect other files
    When I run `patch‑ts gate --stages "syntax,compile,cross-file" --file main.rs`
    Then the gate passes
    And the output shows each stage with "passed": true

  Scenario: Cross‑file stage fails
    Given a patch that changes a function signature with callers in other files
    When I run `patch‑ts gate --stages "syntax,compile,cross-file" --file lib.rs`
    Then the gate fails at the cross‑file stage
    And the output includes the affected caller files and line numbers
```

### Feature: Reliability Scoring

```gherkin
Feature: Patch reliability scoring
  Scenario: Score a low‑confidence patch with high cross‑file impact
    Given a patch with confidence 0.72 and 3 affected callers
    When I run `patch‑ts score --file lib.rs --old "fn foo()" --new "fn foo(x: i32)"`
    Then the overall score is below 70
    And the cross‑file impact dimension is below 50

  Scenario: Score a high‑confidence, safe patch
    Given a patch with confidence 1.0, no cross‑file impact, and successful compilation
    When I run `patch‑ts score --file lib.rs --old "let x = 1;" --new "let x = 2;"`
    Then the overall score is above 90
```

### Feature: MCP Server 2.0

```gherkin
Feature: MCP Server 2.0
  Scenario: Tools list includes schemas
    When an MCP client sends `tools/list`
    Then the response includes `inputSchema` for each tool
    And each schema includes `type`, `properties`, and `required` fields

  Scenario: Resources are readable
    When an MCP client sends `resources/read` for `patch‑ts://symbols/main.rs`
    Then the response includes the symbol index for that file
```

## 2. Test Strategy & Plan

### 2.1 Test Pyramid

| Level | Scope | Tools |
|-------|-------|-------|
| Unit | Entity extraction, provenance record, gate stage, score calculation | Rust `#[test]` |
| Integration | Full CLI commands for `sem‑diff`, `provenance`, `gate`, `score`, MCP resources | `assert_cmd`, `tempfile`, `mockito` |
| Property | Scoring consistency, gate idempotency | `proptest` |
| Manual | TUI entity navigation, MCP integration with Claude Code / Codex | Charters |

### 2.2 Risk‑Based Prioritization

| Risk | Test Focus |
|------|------------|
| Entity extraction misses or misclassifies entities | Test with known multi‑function files for all 16 languages. |
| Provenance records lost on crash | Test with simulated file system errors. |
| Gate pipeline order affects results | Test with intentionally failing stages at each position. |

## 3. Test Case Specifications (Excerpt)

| TC‑ID | Requirement | Steps | Expected |
|-------|-------------|-------|----------|
| TC‑SEM‑001 | FR‑SEM‑001 | Create two Rust files with known entity changes, run `sem‑diff` | Output shows ⊕, ∆, ⊖ correctly |
| TC‑PROV‑001 | FR‑PROV‑001 | Apply a patch, check `.patch‑ts/provenance.jsonl` | Record appended |
| TC‑GATE‑001 | FR‑GATE‑001 | Run gate with all stages passing | All stages pass, overall passed |
| TC‑SCORE‑001 | FR‑SCORE‑002 | Run score on a known risky patch | Score below 70, cross‑file dimension low |

## 4. NFR Verification

| NFR | Verification Method |
|-----|---------------------|
| NFR‑PERF‑001 | Benchmark `sem‑diff` on 1,000‑line files across languages. |
| NFR‑PERF‑002 | Benchmark gate pipeline on 100‑file project. |
| NFR‑SEC‑001 | Audit provenance records don't contain sensitive content. |
| NFR‑COMPAT‑001 | Run v1.6.0 test suite unchanged. |
| NFR‑DOC‑001 | Review governance guide and MCP integration documentation. |

## 5. Requirements Traceability Matrix (RTM)

*(Table mapping Vision/BRS objectives → SRS requirements → Test cases.)*

---

*This verification plan ensures complete coverage of all v1.7.0 trust‑and‑traceability features.*
