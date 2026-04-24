# patch-ts v1.8.0 — The Knowledge‑Graph Platform Specification Suite

Below are the five core specification documents for **patch‑ts v1.8.0**, which transforms the tool into a project‑aware, multi‑agent orchestration platform with verifiable provenance, a MCP Apps dashboard, and a project knowledge graph.

---

## 1. Vision & Strategic Alignment

```markdown
# Product Vision & Strategic Alignment — patch‑ts v1.8.0

| Field | Value |
|-------|-------|
| Project | patch‑ts |
| Document | Vision & Strategic Alignment |
| Version | 1.0 |
| Date | 2026‑04‑24 |
| Author | patch‑ts team, assisted by spec‑writer |
| Status | Draft |

## 1. Vision Statement

> **patch‑ts becomes the central nervous system for AI‑assisted code quality — a project‑aware, multi‑agent orchestration platform that not only applies patches, but also understands the entire codebase, verifies every change against compliance standards, and presents interactive dashboards directly inside the developer’s conversation.**

## 2. Elevator Pitch (Moore Template)

> For **development teams and AI agent frameworks who need to trust, understand, and verify every code change at scale**, patch‑ts is a **tree‑sitter‑backed governance platform that builds a project knowledge graph, orchestrates multi‑agent validation, emits tamper‑evident provenance records, and provides interactive MCP dashboards**. Unlike other patching tools that only apply diffs, our product **understands the whole codebase, verifies every patch against multiple agents, proves who (or what AI) made each change, and lets you review it all in a rich visual interface** — making it the definitive trust layer for AI‑augmented development.

## 3. Problem Statement & Business Context

**Problem:** v1.7.0 added semantic diffing, provenance tracking, quality gates, and reliability scoring, but the tool still operates file‑by‑file. AI agents waste tokens and time re‑reading the entire codebase for context, and there is no way to cryptographically verify the provenance records. The gate pipeline runs stages sequentially and doesn’t leverage parallel multi‑agent validation. The MCP server exposes tools but lacks interactive dashboards, and the provenance records, while structured, are not aligned with the emerging IETF SCITT standards that enterprises need for supply‑chain compliance.

**Why now:**
- The 2026 research landscape is unanimous: multi‑agent architectures dominate automated program repair (PatchIsland, TraceCoder, Agent‑CoEvo, all ICSE/arXiv 2026).  
- Project knowledge graphs built on tree‑sitter (Codebase‑Memory, gortex, mdkb) are proving that giving agents structured codebase context cuts token usage by 90%+.  
- The IETF is formalizing an “AI Agent Execution Profile of SCITT” for tamper‑evident records, and NIST released AI Agent Standards in February 2026. Enterprises need tools that produce compliant records.  
- MCP Apps (SEP‑1865, Jan 2026) allows servers to return interactive HTML dashboards rendered in the conversation — and Claude, VS Code, and other clients already support it.  
- ChaCo (ICSE 2026) demonstrated that test‑coverage awareness in patches improves PR coverage by 30% at negligible cost.

**Business drivers:**
- Become the **first CLI tool to emit IETF SCITT‑compatible provenance records**, meeting enterprise compliance requirements (EU AI Act, ISO 42001, NIST AI Standards).  
- Position patch‑ts as the **orchestration hub** for multi‑agent validation — the tool that dispatches syntax, compile, coverage, security, and style agents and aggregates their findings.  
- Reduce AI agent token consumption by 90%+ by providing a structured project knowledge graph via MCP resources.  
- Differentiate with an interactive MCP Apps dashboard that no other patching tool offers.

## 4. Target Users / Customers

| Segment | Description |
|---------|-------------|
| **Enterprise compliance teams** | Need tamper‑evident, SCITT‑compatible provenance records for every AI‑assisted change. |
| **AI agent frameworks** | Need structured, semantic codebase context (knowledge graph) to reduce token costs and improve patch quality. |
| **Platform engineering teams** | Need a multi‑agent validation pipeline that gates every change before it reaches production. |
| **Individual developers** | Want a rich, interactive review dashboard inside their AI conversation — not just terminal output. |
| **Open‑source maintainers** | Need cryptographic proof of authorship for every contribution, human or AI. |

**Explicitly NOT targeting (v1.8.0):**
- Full ML‑based patch generation (beyond compiler‑error‑driven fix from v1.6.0).
- Cloud‑based orchestration services.
- Real‑time collaborative editing across multiple human users.
- Blockchain‑based provenance (SCITT uses traditional PKI and transparency logs, not blockchain).

## 5. User Needs & Value Proposition

| Need | patch‑ts v1.8.0 Value |
|------|-----------------------|
| “I need my AI agent to understand the whole project, not just one file.” | The project knowledge graph gives agents a semantic index of functions, types, and callers across all files, cutting token usage by 90%+. |
| “I need proof that my AI‑generated changes are compliant with EU AI Act and NIST standards.” | Agent Trace 2.0 emits tamper‑evident, SCITT‑compatible provenance records that can be verified cryptographically. |
| “I need to see patch diffs, scores, and provenance in a visual dashboard, not text output.” | The MCP Apps dashboard renders interactive HTML with diff comparisons, reliability scores, and provenance timelines inside Claude Desktop / VS Code. |
| “I want multiple validation agents to run in parallel — not one at a time.” | Multi‑agent gate pipeline dispatches syntax, compile, coverage, security, and style agents concurrently and aggregates results. |
| “I need to know if a patch is covered by tests before it’s applied.” | The `coverage` gate stage identifies untested changed lines and optionally suggests test templates. |
| “I need to know if a patch introduces security vulnerabilities.” | Adversarial patch detection scans proposed patches for known malicious patterns (injection, path traversal, data exfiltration). |
| “I need to trust that the score is based on real project structure, not just heuristics.” | Namespace‑aware reliability scoring checks whether referenced identifiers actually exist in the project knowledge graph. |

**Differentiator:** No other tool provides a project knowledge graph, SCITT‑compliant provenance, MCP Apps dashboard, multi‑agent validation pipeline, and namespace‑aware scoring — all in a single, offline binary.

## 6. Desired Outcomes & Success Metrics

### Business Outcomes (v1.8.0)

| ID | Outcome | Key Result / Target |
|----|---------|---------------------|
| G‑1 | Achieve enterprise compliance readiness | At least 2 Fortune 500 companies pilot patch‑ts for SCITT‑compliant AI change auditing within 6 months. |
| G‑2 | Reduce AI agent token costs | Knowledge graph integration reduces token usage by ≥80% for context‑retrieval compared to raw file reads, measured in an A/B test with a partner AI framework. |
| G‑3 | Become the standard multi‑agent validation tool | At least 3 major AI frameworks integrate the multi‑agent gate pipeline. |

### Product Outcomes (v1.8.0)

| ID | Outcome | Metric |
|----|---------|--------|
| P‑1 | Project knowledge graph works for all 16 languages | 100% of a test corpus of 50 multi‑file projects produce correct symbol indices, call graphs, and import maps. |
| P‑2 | Agent Trace 2.0 records are SCITT‑compliant | All emitted records pass validation against the IETF SCITT AI Agent Execution Profile schema. |
| P‑3 | MCP Apps dashboard renders in Claude Desktop | A user can open a patch review dashboard by invoking a single MCP tool, and interact with it (view diff, score, provenance, approve/reject). |
| P‑4 | Multi‑agent gate pipeline is faster than sequential | Parallel execution reduces gate time by ≥40% on a 4‑core machine compared to sequential for the same stages. |

## 7. Strategic Constraints

| Constraint | Description |
|------------|-------------|
| **Backward compatibility** | All v1.7.0 CLI flags, JSON schemas, MCP interface, and WASM plugin interfaces remain unchanged. |
| **Performance** | Knowledge graph index must build in ≤3s for a 1,000‑file project; gate pipeline ≤8s for 5 stages. |
| **Cross‑platform** | All new features must work on Linux, macOS, and Windows. |
| **Dependencies** | No new heavy dependencies; knowledge graph uses existing tree‑sitter grammars; SCITT compliance uses existing JSON and SHA‑256 (no blockchain); MCP Apps dashboard uses embedded HTML/JS served via the existing stdio transport. |
| **Privacy** | All data remains local; no telemetry or cloud submission. |

## 8. Goals and Non‑Goals (v1.8.0)

### Goals

- Implement **Project Knowledge Graph** — a tree‑sitter‑based project‑wide symbol index with call‑graph traversal, import maps, and impact analysis. Expose via MCP resources.  
- Implement **Agent Trace 2.0 / SCITT Compliance** — tamper‑evident provenance records that pass the IETF SCITT AI Agent Execution Profile schema. Add `patch‑ts trace verify` for cryptographic verification.  
- Implement **MCP Apps Dashboard** — interactive HTML dashboard for patch review, scoring, and provenance visualization, served via MCP Apps.  
- Implement **Multi‑Agent Validation Pipeline** — parallel dispatch of syntax, compile, coverage, security, and style agents in `patch‑ts gate`.  
- Implement **Test‑Coverage Aware Gate Stage** — tree‑sitter‑based detection of changed lines, test coverage report, and optional test suggestion.  
- Implement **Namespace‑Aware Reliability Scoring** — enhanced scoring that checks the knowledge graph for identifier existence and naming conflicts.  
- Implement **Adversarial Patch Detection** — `--audit‑security` mode that scans for injection, path traversal, and data exfiltration patterns.  
- Update documentation, README, and compliance guides.

### Non‑Goals (explicitly excluded)

- Full ML‑based patch generation.  
- Cloud‑based orchestration services.  
- Real‑time collaborative editing across multiple human users.  
- Blockchain‑based provenance (SCITT uses traditional PKI, not blockchain).

## 9. Operational Concept & High‑Level Scenarios

### Concept of Operations

A developer or AI agent invokes patch‑ts. The tool builds or loads a cached project knowledge graph, indexing all symbols, call relationships, and imports. When a patch is applied, the multi‑agent gate pipeline dispatches validation agents in parallel, and the reliability score is computed using the knowledge graph for namespace awareness. Provenance records are emitted in SCITT‑compliant format, cryptographically signed. The developer can open an interactive MCP Apps dashboard to review the patch, score, provenance, and agent findings — all inside their AI conversation.

### High‑Level Scenarios (v1.8.0)

1. **AI agent uses knowledge graph for context**  
   An AI coding agent needs to understand how `process_order` is used across the project. Instead of reading 50 files, it queries `patch‑ts://knowledge‑graph/callers/process_order` via MCP and receives a structured list of callers, saving 94% token usage.

2. **Enterprise compliance audit**  
   A compliance officer runs `patch‑ts trace verify --since "2026‑01‑01"` and receives cryptographically verified proof that all AI‑assisted changes in Q1 were properly validated and attributed, with SCITT‑compliant receipts.

3. **MCP Apps dashboard review**  
   A developer invokes `patch‑ts dashboard --patch-id abc123` from Claude Desktop. An interactive HTML dashboard opens showing the diff, reliability score (with dimensional breakdown), provenance timeline, and multi‑agent findings. The developer clicks "Approve" — the patch is applied and the dashboard updates.

4. **Multi‑agent gate catches a security issue**  
   An AI agent proposes a patch that introduces a SQL injection pattern. The security agent in the gate pipeline detects it, the gate fails, and the structured response includes the exact line, pattern, and a suggestion to use parameterized queries.

5. **Test coverage gap identified**  
   The coverage agent in the gate pipeline reports that a changed function has 0% test coverage. The developer receives a suggested test template generated from the function signature and docstring.

## 10. Stakeholders, Sponsorship & Governance

| Role | Name / Org | Responsibility |
|------|------------|----------------|
| **Executive Sponsor** | Project maintainer | Approves strategic direction. |
| **Product Owner** | Project maintainer | Prioritizes features, manages scope. |
| **Engineering Lead** | Core contributor(s) | Oversees technical implementation. |
| **Community** | Open‑source contributors | Review PRs, test pre‑releases. |

## 11. Risks, Assumptions & Open Questions

### Assumptions

- Tree‑sitter grammars provide sufficient node‑type information for call‑graph construction across all 16 languages.  
- The IETF SCITT AI Agent Execution Profile stabilizes and is published as an RFC by the time v1.8.0 ships; if not, we ship the current draft format with a migration plan.  
- Claude Desktop and VS Code continue to support MCP Apps (SEP‑1865); the standard remains stable.

### Risks

| Risk | Likelihood | Impact | Mitigation |
|------|------------|--------|------------|
| Knowledge graph may be inaccurate for dynamic languages (Python, Ruby). | Medium | Medium | Start with static languages; use best‑effort heuristics; clearly document limitations. |
| SCITT spec may change significantly. | Low | Medium | Version the provenance format; provide migration tools; follow IETF mailing list closely. |
| Parallel gate pipeline may introduce race conditions. | Low | High | Each agent runs in a sandboxed process with its own file copy; no shared state. |

### Open Questions

- Should the knowledge graph be built eagerly or lazily? (Eagerly on first invocation, cached per project.)  
- Should the MCP Apps dashboard require an internet connection? (No, all assets are bundled.)  
- How to handle conflicting findings from parallel agents? (All findings are reported; the gate fails if any critical agent fails.)

---

*This vision document anchors the knowledge‑graph platform strategy for patch‑ts v1.8.0.*
```

---

## 2. Business & Stakeholder Requirements Specification (BRS)

```markdown
# Business & Stakeholder Requirements Specification — patch‑ts v1.8.0

| Field | Value |
|-------|-------|
| Project | patch‑ts |
| Document | Business & Stakeholder Requirements Specification |
| Version | 1.0 |
| Date | 2026‑04‑24 |
| Author | patch‑ts team, assisted by spec‑writer |
| Status | Draft |
| References | Vision v1.8.0 |

## 1. Business Context

### 1.1 Purpose

This BRS defines the business‑level requirements for patch‑ts v1.8.0, which adds a project knowledge graph, SCITT‑compliant provenance, an MCP Apps dashboard, a multi‑agent validation pipeline, test‑coverage awareness, namespace‑aware scoring, and adversarial patch detection.

### 1.2 Business Problem / Opportunity

v1.7.0 added governance features but still operates file‑by‑file, cannot cryptographically verify provenance records, lacks interactive dashboards, and runs validation sequentially. The business opportunity is to become the first CLI tool that is **enterprise‑compliance‑ready** (SCITT‑compliant), **project‑aware** (knowledge graph), and **interactive** (MCP Apps dashboard) — three capabilities that no other tool combines.

### 1.3 Scope Boundaries

**In Scope:**
- Project Knowledge Graph (tree‑sitter‑based index, call graph, import maps).
- Agent Trace 2.0 with SCITT‑compliant tamper‑evident records.
- MCP Apps interactive dashboard.
- Multi‑agent parallel gate pipeline.
- Test‑coverage aware gate stage.
- Namespace‑aware reliability scoring.
- Adversarial patch detection.
- Documentation and compliance guides.

**Out of Scope:**
- Full ML‑based patch generation.
- Cloud‑based orchestration.
- Real‑time collaborative editing.
- Blockchain‑based provenance.

## 2. Business Goals, Objectives & Success Metrics

| ID | Goal | Fit Criterion |
|----|------|---------------|
| BR‑001 | Achieve enterprise compliance readiness | At least 2 Fortune 500 companies pilot patch‑ts for SCITT auditing within 6 months. |
| BR‑002 | Reduce AI agent token costs | Knowledge graph integration reduces token usage by ≥80% for context retrieval in an A/B test. |
| BR‑003 | Become the standard multi‑agent validation tool | At least 3 AI frameworks integrate the multi‑agent gate. |

*(Traceability: BR‑001…003 ← Vision G‑1…G‑3)*

## 3–12. Additional Sections

*(Follow the same pattern as previous BRS documents: Glossary, Conceptual Domain Model, Stakeholder Needs, System‑in‑Context, Constraints, Risks, Traceability.)*

Key Stakeholder Needs:
- SN‑001: As an AI agent, I need structured project‑wide context (functions, callers, imports) to reduce token usage and improve patch accuracy.
- SN‑002: As a compliance officer, I need tamper‑evident, cryptographically verifiable provenance records for every AI‑assisted change.
- SN‑003: As a developer, I want to review patches in an interactive dashboard inside my AI conversation.
- SN‑004: As a platform engineer, I need parallel, multi‑agent validation that catches syntax, compile, coverage, security, and style issues.
- SN‑005: As a security auditor, I need to detect adversarial patches that introduce vulnerabilities.

---

*This BRS establishes the business foundation for v1.8.0.*
```

---

## 3. Software Requirements Specification (SRS)

```markdown
# Software Requirements Specification — patch‑ts v1.8.0

| Field | Value |
|-------|-------|
| Project | patch‑ts |
| Document | Software Requirements Specification |
| Version | 1.0 |
| Date | 2026‑04‑24 |
| Author | patch‑ts team, assisted by spec‑writer |
| Status | Draft |
| References | BRS v1.8.0, Vision v1.8.0 |

## 1. Introduction & Scope

This SRS defines the functional and non‑functional requirements for patch‑ts v1.8.0, which adds a project knowledge graph, SCITT‑compliant provenance, an MCP Apps dashboard, a multi‑agent validation pipeline, test‑coverage awareness, namespace‑aware scoring, and adversarial patch detection.

### 1.1 Scope

- Project Knowledge Graph (`src/knowledge.rs`).
- Agent Trace 2.0 / SCITT compliance (`src/scitt.rs`).
- MCP Apps dashboard (`src/dashboard.rs`).
- Multi‑agent gate pipeline upgrade (`src/gate.rs`).
- Test‑coverage aware gate stage (`src/coverage.rs`).
- Namespace‑aware reliability scoring upgrade (`src/score.rs`).
- Adversarial patch detection (`src/audit.rs`).
- Integration tests, documentation, compliance guides.

### 1.2 Out of Scope

- Full ML‑based patch generation.
- Cloud‑based orchestration.
- Real‑time collaborative editing.
- Blockchain‑based provenance.

## 2. System Context & Overview

Same C1 context as before. New internal modules: `knowledge.rs`, `scitt.rs`, `dashboard.rs`, `coverage.rs`, `audit.rs`. Upgraded modules: `gate.rs`, `score.rs`, `mcp.rs`.

## 3. Functional Capabilities & Behavior

### Feature: Project Knowledge Graph

| ID | Requirement | Priority | Acceptance Criteria |
|----|-------------|----------|---------------------|
| FR‑KG‑001 | `patch‑ts index` shall build a project‑wide symbol index for all 16 languages, using tree‑sitter. | Must | Index includes functions, structs, classes, methods, imports, and their file locations. |
| FR‑KG‑002 | The index shall support call‑graph query: `patch‑ts index callers <symbol>` returns all call sites. | Must | Correct callers identified in a multi‑file project. |
| FR‑KG‑003 | The index shall be cached in `.patch‑ts/cache/knowledge‑graph.json` and invalidated when files change. | Should | Cache rebuilt only for changed files. |
| FR‑KG‑004 | The index shall be exposed as an MCP resource: `patch‑ts://knowledge‑graph`. | Should | Available via MCP `resources/read`. |

### Feature: Agent Trace 2.0 / SCITT Compliance

| ID | Requirement | Priority | Acceptance Criteria |
|----|-------------|----------|---------------------|
| FR‑SCITT‑001 | Every provenance record shall include a SHA‑256 hash of the patch content, a timestamp, and the tool/agent identity. | Must | Records include `content_hash`, `timestamp`, `tool`, `agent`. |
| FR‑SCITT‑002 | Records shall be signed with an Ed25519 key (if `‑‑signing‑key` is provided) to enable tamper‑evident verification. | Should | Signature verifiable with `patch‑ts trace verify`. |
| FR‑SCITT‑003 | `patch‑ts trace verify --since <DATE>` shall verify the integrity of all provenance records in the log. | Must | Tampered records are flagged; intact records pass. |
| FR‑SCITT‑004 | The provenance log format shall conform to the IETF SCITT AI Agent Execution Profile draft specification. | Should | Output passes validation against the SCITT schema. |

### Feature: MCP Apps Dashboard

| ID | Requirement | Priority | Acceptance Criteria |
|----|-------------|----------|---------------------|
| FR‑DASH‑001 | `patch‑ts dashboard` shall start an MCP Apps server that returns an HTML dashboard for the most recent patch. | Must | Dashboard accessible via MCP `sampling/createMessage` as an iframe resource. |
| FR‑DASH‑002 | The dashboard shall display: side‑by‑side diff, reliability score with dimensional breakdown, provenance timeline, and multi‑agent findings. | Must | All sections populated with real data. |
| FR‑DASH‑003 | The dashboard shall support “Approve” / “Reject” buttons that call back to patch‑ts via MCP. | Should | Buttons trigger patch application or rejection. |

### Feature: Multi‑Agent Validation Pipeline

| ID | Requirement | Priority | Acceptance Criteria |
|----|-------------|----------|---------------------|
| FR‑GATE‑002v2 | `patch‑ts gate --stages "syntax,compile,coverage,security,style"` shall run stages in parallel and aggregate results. | Must | All stages run concurrently; aggregate result returned. |
| FR‑GATE‑005 | Each stage shall report its findings independently; the gate fails if any `critical` stage fails. | Must | Aggregate result includes per‑stage pass/fail and details. |
| FR‑GATE‑006 | Stage parallelism shall be configurable via `‑‑parallel` (default true) and `‑‑serial` to revert to sequential. | Should | Flags honored. |

### Feature: Test‑Coverage Aware Gate

| ID | Requirement | Priority | Acceptance Criteria |
|----|-------------|----------|---------------------|
| FR‑COV‑001 | `patch‑ts gate coverage` shall identify changed lines (using tree‑sitter diff of old/new content). | Must | Changed lines listed with file and line numbers. |
| FR‑COV‑002 | The stage shall run the configured test suite with coverage (`cargo tarpaulin` or equivalent). | Should | Coverage data collected. |
| FR‑COV‑003 | The stage shall report which changed lines are uncovered, and optionally suggest a test template. | Could | Uncovered lines flagged; test template based on function signature. |

### Feature: Namespace‑Aware Scoring

| ID | Requirement | Priority | Acceptance Criteria |
|----|-------------|----------|---------------------|
| FR‑SCORE‑005 | The reliability score shall check whether all identifiers in the patched content exist in the project knowledge graph. | Should | Unknown identifiers reduce the score and generate warnings. |
| FR‑SCORE‑006 | The score shall check for naming conflicts (e.g., introducing a function with the same name as an existing one in another file). | Could | Conflicts reduce the score and generate warnings. |

### Feature: Adversarial Patch Detection

| ID | Requirement | Priority | Acceptance Criteria |
|----|-------------|----------|---------------------|
| FR‑AUDIT‑001 | `patch‑ts audit --file <PATCH>` shall scan for known malicious patterns: SQL injection, command injection, path traversal, unauthorized data access. | Should | Known patterns detected and reported. |
| FR‑AUDIT‑002 | `‑‑audit‑security` flag on `patch`, `balance`, `gate` shall run the audit before applying the patch. | Should | Malicious patches are blocked; warning with pattern description. |

## 4. Quality & Non‑Functional Requirements

| ID | Category | Requirement | Fit Criterion |
|----|----------|-------------|---------------|
| NFR‑PERF‑001 | Performance | Knowledge graph index must build in ≤3s for a 1,000‑file project. | Benchmarked. |
| NFR‑PERF‑002 | Performance | Multi‑agent gate pipeline must complete in ≤8s for 5 stages on a 4‑core machine. | Benchmarked. |
| NFR‑SEC‑001 | Security | SCITT signing keys must be stored in a local key file with restricted permissions (600). | Audit. |
| NFR‑COMPAT‑001 | Compatibility | All v1.7.0 tests pass without modification. | CI regression suite. |
| NFR‑DOC‑001 | Documentation | Compliance guide, knowledge graph API docs, and MCP Apps integration guide are complete. | Review checklist. |

## 5. External Interfaces & Data Contracts

### CLI New/Modified Flags

- `index` subcommand: `--callers <SYMBOL>`, `--rebuild`.
- `trace verify` subcommand: `--since`, `--key <PATH>`.
- `dashboard` subcommand: starts MCP Apps server.
- `gate` upgraded: `--parallel` (default), `--serial`.
- `--audit‑security` flag on `patch`, `balance`, `gate`.
- `audit` subcommand: `--file <PATCH>`.

### JSON Output Schema (Additions)

```json
{
  "knowledge_graph": {
    "symbols": { "process_order": { "kind": "function", "file": "src/orders.rs", "line": 42, "callers": ["main.rs:25", "api.rs:10"] } },
    "imports": { "orders": { "file": "src/lib.rs", "line": 5 } }
  },
  "scitt_provenance": {
    "records": [
      {
        "content_hash": "sha256:abc123...",
        "timestamp": "2026-04-24T10:30:00Z",
        "signature": "ed25519:def456...",
        "agent": "Claude Code",
        "model": "sonnet-4"
      }
    ]
  },
  "gate_aggregate": {
    "passed": false,
    "stages": {
      "syntax": { "passed": true, "details": "AST valid" },
      "compile": { "passed": true, "details": "Compilation OK" },
      "coverage": { "passed": false, "details": "2 uncovered lines in src/orders.rs" },
      "security": { "passed": false, "details": "SQL injection pattern detected at line 42" },
      "style": { "passed": true, "details": "No style violations" }
    }
  }
}
```

## 6–8. Constraints, Assumptions, TBD

*(Summarized: must maintain backward compatibility, tree‑sitter grammars sufficient for call‑graph, SCITT spec stable.)*

---

*This SRS defines the complete behavioral contract for v1.8.0.*
```

---

## 4. Architecture & Design Specification

```markdown
# Architecture & Design Specification — patch‑ts v1.8.0

| Field | Value |
|-------|-------|
| Project | patch‑ts |
| Document | Architecture & Design Specification |
| Version | 1.0 |
| Date | 2026‑04‑24 |
| Author | patch‑ts team, assisted by spec‑writer |
| Status | Draft |
| References | SRS v1.8.0, BRS v1.8.0 |

## 1. Context & Scope

This document describes the architectural design for the knowledge‑graph platform features in v1.8.0: project knowledge graph, SCITT‑compliant provenance, MCP Apps dashboard, multi‑agent validation pipeline, test‑coverage awareness, namespace‑aware scoring, and adversarial patch detection.

## 2. Goals & Non‑Goals

**Goals:**
- Introduce `knowledge.rs` for tree‑sitter‑based project‑wide symbol index and call‑graph.
- Introduce `scitt.rs` for tamper‑evident, signed provenance records.
- Introduce `dashboard.rs` for MCP Apps HTML interface.
- Upgrade `gate.rs` for parallel multi‑agent dispatch.
- Introduce `coverage.rs` for test‑coverage aware gate stage.
- Upgrade `score.rs` for namespace‑aware heuristics.
- Introduce `audit.rs` for adversarial pattern detection.
- Keep all changes backward‑compatible.

**Non‑Goals:**
- Refactor the entire codebase.
- Introduce external services or heavy new dependencies.
- Replace existing patching engine.

## 3. Architecturally Significant Requirements (ASRs)

| ASR ID | Description | Source |
|--------|-------------|--------|
| ASR‑001 | Knowledge graph must be language‑agnostic, using tree‑sitter grammars. | FR‑KG‑001 |
| ASR‑002 | Provenance records must be tamper‑evident (hash chain) and optionally signed. | FR‑SCITT‑002 |
| ASR‑003 | Multi‑agent gate must run stages in parallel without shared state. | FR‑GATE‑002v2 |
| ASR‑004 | Namespace‑aware scoring must query the knowledge graph. | FR‑SCORE‑005 |
| ASR‑005 | MCP Apps dashboard must be self‑contained (bundled HTML/JS). | FR‑DASH‑001 |

## 4. The Design

### 4.1 System Overview (C4 Level 2)

```
[User/Agent] → [CLI] → [knowledge.rs] [scitt.rs] [dashboard.rs] [gate.rs] [coverage.rs] [score.rs] [audit.rs]
                  ↓         ↓              ↓             ↓          ↓            ↓          ↓
              [tree-sitter] [SHA‑256+Ed25519] [bundled HTML] [parallel::spawn] [tarpaulin] [knowledge graph] [regex patterns]
```

### 4.2 Key Design Changes

**Knowledge Graph (`knowledge.rs`)**
- `build_project_index(root: &Path) -> KnowledgeGraph` walks all source files, extracts symbols, call edges, and imports.
- `callers_of(symbol: &str, kg: &KnowledgeGraph) -> Vec<CallSite>`.
- Cached as JSON in `.patch‑ts/cache/knowledge‑graph.json` with per‑file hashes for invalidation.
- Exposed as MCP resource via `mcp.rs`.

**SCITT Provenance (`scitt.rs`)**
- `ScittRecord { content_hash, timestamp, tool, agent, model, signature }`.
- `emit_scitt_record(record)` appends to `.patch‑ts/provenance.jsonl` and maintains a SHA‑256 chain.
- `verify_scitt_log(since, key)` recomputes hashes, verifies signatures, and reports tampered entries.
- Optional Ed25519 signing via `‑‑signing‑key`; key stored at `~/.config/patch‑ts/signing‑key`.

**MCP Apps Dashboard (`dashboard.rs`)**
- Bundled HTML template with embedded CSS/JS (no external CDN).
- Served as an MCP App resource: `patch‑ts://dashboard/{patch‑id}`.
- Interactive: “Approve” / “Reject” buttons call MCP `tools/call` to apply or reject.
- Data populated via JSON embedded in the HTML page.

**Multi‑Agent Gate (`gate.rs`)**
- `run_gate_parallel(stages, context) -> GateResult` uses `rayon` or `std::thread::spawn` to run stages concurrently.
- Each stage receives a clone of the context (immutable) and returns `StageResult`.
- Aggregate result collects all stage results; overall `passed` is true only if all critical stages pass.

**Coverage Stage (`coverage.rs`)**
- Identifies changed lines using tree‑sitter diff (`semdiff.rs`).
- Runs `cargo tarpaulin` (or configured coverage tool) and parses output.
- Cross‑references changed lines with coverage data.
- Optionally generates a test template using the function signature and docstring.

**Namespace‑Aware Scoring (`score.rs`)**
- Queries the knowledge graph for each identifier in the patched content.
- Scores lower for unknown identifiers or naming conflicts.

**Adversarial Detection (`audit.rs`)**
- Uses regex patterns for SQL injection (`SELECT.*WHERE.*=.*\+`), command injection (`os.system`, `subprocess.call`), path traversal (`../`, `os.path.join` with user input), and data exfiltration (`requests.post` with file content).
- Returns a list of findings with severity.

### 4.3 Data Model

- `KnowledgeGraph { symbols: HashMap<String, SymbolInfo>, call_edges: Vec<CallEdge>, imports: HashMap<String, ImportInfo> }`
- `ScittRecord { content_hash: String, timestamp: String, tool: String, agent: Option<String>, model: Option<String>, signature: Option<String>, previous_hash: String }`
- `DashboardState { patch_id: String, diff: String, score: ReliabilityScore, provenance: Vec<ScittRecord>, gate_result: GateResult }`

## 5. Architecture Decision Records (ADRs)

### ADR‑041: Use tree‑sitter grammars for knowledge graph construction

**Context:** Need to index project‑wide symbols, call edges, and imports.
**Decision:** Use existing tree‑sitter grammars for all 16 languages; define per‑language queries for function definitions, calls, and imports.
**Alternatives:** LSP (too heavy), ctags (not available on all platforms).
**Consequences:** Accurate for statically‑typed languages; best‑effort for dynamic languages; consistent with existing codebase.

### ADR‑042: SHA‑256 hash chain for SCITT provenance

**Context:** Need tamper‑evident provenance records without blockchain.
**Decision:** Each record includes `previous_hash` (SHA‑256 of the previous record). Verification recomputes the chain and detects breakage.
**Alternatives:** Blockchain (overkill), digital signatures only (no chain).
**Consequences:** Simple, verifiable; requires sequential log (no random access without recomputation).

### ADR‑043: Bundled HTML for MCP Apps dashboard

**Context:** Need an interactive dashboard without network dependencies.
**Decision:** Embed HTML, CSS, and JS directly in the binary using `include_str!` or a build‑time bundling step.
**Alternatives:** Serve from CDN (requires internet), generate on‑the‑fly (lower quality).
**Consequences:** Slightly larger binary (<500KB increase); fully offline; consistent experience.

### ADR‑044: Parallel gate stages using `rayon`

**Context:** Multi‑agent validation should run in parallel for speed.
**Decision:** Use `rayon::scope` to spawn each stage as a parallel task.
**Alternatives:** `std::thread::spawn` (more boilerplate), `tokio::spawn` (async not needed).
**Consequences:** Leverages existing `rayon` dependency; each stage must be `Send`; context cloned per stage.

## 6. API & Interface Contracts

CLI extended with new subcommands. MCP server extended with knowledge graph resources and dashboard app. No changes to WASM plugin interface.

## 7. Cross‑cutting Concerns

- **Error Handling**: Parallel gate failures are aggregated; any critical failure fails the gate.
- **Testing**: Integration tests with multi‑file projects; mock coverage tool for coverage stage.
- **Performance**: Knowledge graph cached; gate stages parallelized; dashboard assets bundled.

## 8. Alternatives Considered

| Alternative | Why Rejected |
|-------------|--------------|
| Use LSP for knowledge graph | Too heavy; tree‑sitter is sufficient and already integrated. |
| Use blockchain for provenance | Over‑engineered; SHA‑256 chain provides tamper evidence. |
| Serve dashboard from external URL | Requires internet; violates offline‑first principle. |

## 9. Traceability

| ASR | ADR | Component |
|-----|-----|-----------|
| ASR‑001 | ADR‑041 | knowledge.rs |
| ASR‑002 | ADR‑042 | scitt.rs |
| ASR‑003 | ADR‑044 | gate.rs |
| ASR‑004 | ADR‑041 | score.rs |
| ASR‑005 | ADR‑043 | dashboard.rs |

---

*This architecture specification provides the blueprint for implementing v1.8.0.*
```

---

## 5. Behavioral Specification & Test Verification Plan

```markdown
# Behavioral Specification & Test Verification Plan — patch‑ts v1.8.0

| Field | Value |
|-------|-------|
| Project | patch‑ts |
| Document | Behavioral Specification & Test Verification Plan |
| Version | 1.0 |
| Date | 2026‑04‑24 |
| Author | patch‑ts team, assisted by spec‑writer |
| Status | Draft |
| References | SRS v1.8.0, Architecture v1.8.0 |

## 1. Behavioral Specifications (Specification by Example)

### Feature: Project Knowledge Graph

```gherkin
Feature: Project knowledge graph

  Scenario: Build project index and query callers
    Given a Rust project with "lib.rs" defining `process_order` and "main.rs" calling it
    When I run `patch-ts index`
    Then the knowledge graph is built and cached
    And I can run `patch-ts index callers process_order`
    And the output shows "main.rs:25"
```

### Feature: SCITT‑Compliant Provenance

```gherkin
Feature: Tamper‑evident provenance

  Scenario: Verify an intact provenance log
    Given a provenance log with valid hash chain and signatures
    When I run `patch-ts trace verify`
    Then all records pass verification
    And the output shows "All records verified."

  Scenario: Detect tampered record
    Given a provenance log where one record has been modified
    When I run `patch-ts trace verify`
    Then the tampered record is flagged
    And the output shows "Tampered record detected at timestamp 2026-04-24T10:30:00Z"
```

### Feature: MCP Apps Dashboard

```gherkin
Feature: Interactive dashboard

  Scenario: Open dashboard from Claude Desktop
    Given a recent patch has been applied
    When an MCP client invokes `patch‑ts dashboard`
    Then an HTML dashboard is returned
    And the dashboard shows the diff, reliability score, provenance timeline, and gate results
```

### Feature: Multi‑Agent Gate Pipeline

```gherkin
Feature: Parallel multi‑agent gate

  Scenario: All agents pass
    Given a valid patch with no security or coverage issues
    When I run `patch-ts gate --stages "syntax,compile,coverage,security" --file main.rs`
    Then all stages pass in parallel
    And the aggregate result shows "passed: true"

  Scenario: Security agent catches SQL injection
    Given a patch that introduces a SQL injection pattern
    When I run `patch-ts gate --stages "security" --file main.rs`
    Then the security stage fails
    And the output includes "SQL injection pattern detected"
```

### Feature: Test‑Coverage Gate Stage

```gherkin
Feature: Coverage‑aware gate

  Scenario: Changed lines are not covered by tests
    Given a patch that changes a function with no test coverage
    When I run `patch-ts gate --stages "coverage" --file lib.rs`
    Then the coverage stage reports uncovered lines
    And the stage fails
```

### Feature: Adversarial Patch Detection

```gherkin
Feature: Adversarial patch audit

  Scenario: Detect command injection pattern
    Given a patch that adds `os.system(user_input)`
    When I run `patch-ts audit --file malicious.patch`
    Then the audit reports a command injection pattern
    And the severity is "critical"
```

## 2. Test Strategy & Plan

### 2.1 Test Pyramid

| Level | Scope | Tools |
|-------|-------|-------|
| Unit | Knowledge graph, SCITT verification, audit patterns, coverage diff | Rust `#[test]` |
| Integration | Full CLI commands for `index`, `trace verify`, `dashboard`, `gate`, `audit` | `assert_cmd`, `tempfile` |
| Property | Knowledge graph consistency across file changes; SCITT chain integrity | `proptest` |
| Manual | Dashboard rendering in Claude Desktop / VS Code; multi‑agent integration | Charters |

### 2.2 Risk‑Based Prioritization

| Risk | Test Focus |
|------|------------|
| Knowledge graph false negatives for dynamic languages | Test with Python/Ruby projects. |
| SCITT chain breakage on concurrent writes | Test with parallel patch applications. |
| Dashboard JavaScript errors | Manual testing in multiple MCP clients. |

## 3. Test Case Specifications (Excerpt)

| TC‑ID | Requirement | Steps | Expected |
|-------|-------------|-------|----------|
| TC‑KG‑001 | FR‑KG‑001 | Run `index` on a 10‑file project | Index built, callers queryable |
| TC‑SCITT‑001 | FR‑SCITT‑001 | Apply a patch, inspect provenance record | Record has `content_hash` and `timestamp` |
| TC‑DASH‑001 | FR‑DASH‑001 | Invoke `dashboard` via MCP | HTML response with diff, score, provenance |
| TC‑GATE‑001 | FR‑GATE‑002v2 | Run gate with 5 stages in parallel | All stages complete, aggregate correct |
| TC‑COV‑001 | FR‑COV‑001 | Apply patch, run coverage gate | Uncovered lines reported |
| TC‑AUDIT‑001 | FR‑AUDIT‑001 | Audit a patch with SQL injection | Pattern detected |

## 4. NFR Verification

| NFR | Verification Method |
|-----|---------------------|
| NFR‑PERF‑001 | Benchmark knowledge graph build on 1,000‑file project. |
| NFR‑PERF‑002 | Benchmark gate pipeline on 5‑stage config. |
| NFR‑SEC‑001 | Audit signing key permissions; verify key is not logged. |
| NFR‑COMPAT‑001 | Run v1.7.0 test suite unchanged. |
| NFR‑DOC‑001 | Review compliance guide and knowledge graph API docs. |

## 5. Requirements Traceability Matrix (RTM)

*(Table mapping Vision/BRS objectives → SRS requirements → Test cases.)*

---

*This verification plan ensures complete coverage of all v1.8.0 knowledge‑graph platform features.*
