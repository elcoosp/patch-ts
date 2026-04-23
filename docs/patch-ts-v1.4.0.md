# patch-ts v1.4.0 — Platform‑Grade Patching Specification Suite

Below are the five core specification documents for **patch‑ts v1.4.0**, which transforms the tool into a platform‑grade, self‑healing, and verifiable patching backend for the AI‑agent ecosystem. The documents follow the layered model: Vision, BRS, SRS, Architecture, and Behavioral Spec with Test Verification.

---

## 1. Vision & Strategic Alignment

```markdown
# Product Vision & Strategic Alignment — patch‑ts v1.4.0

| Field | Value |
|-------|-------|
| Project | patch‑ts |
| Document | Vision & Strategic Alignment |
| Version | 1.0 |
| Date | 2026‑04‑23 |
| Author | patch‑ts team, assisted by spec‑writer |
| Status | Draft |

## 1. Vision Statement

> **patch‑ts becomes the definitive, platform‑grade patching backend for the AI‑agent ecosystem – a tool that not only applies patches safely, but also validates them at every level (syntax → semantics → compilation), self‑heals failures, and integrates seamlessly with any agent via the Model Context Protocol (MCP).**

## 2. Elevator Pitch (Moore Template)

> For **AI agent frameworks and developers who need a verifiable, self‑correcting patching platform**, patch‑ts is a **tree‑sitter‑backed CLI with an MCP server, post‑patch compilation checking, self‑healing feedback, and semantic symbol indexing**. Unlike other tools that simply apply diffs, our product **validates patches through compilation, refuses low‑confidence changes, and teaches agents how to fix them** – making it the universal, trustworthy patching layer for every AI‑assisted workflow.

## 3. Problem Statement & Business Context

**Problem:** v1.3.0 hardened patch‑ts against hallucinated inputs and added confidence scoring, but patches can still pass syntax validation yet fail compilation or introduce semantic errors. AI agents lack a standardised way to invoke patching tools, and when patches fail, the feedback is not rich enough to enable genuine self‑correction. The tool is still a standalone CLI, not a platform component.

**Why now:**  
- The **Model Context Protocol (MCP)** is rapidly becoming the standard for AI‑tool interoperability. Being an MCP server makes patch‑ts instantly available to Claude Code, Codex CLI, OpenDev, and any future MCP‑compatible agent.  
- The industry is moving from syntax‑only validation to **full compilation‑gate checking** – the Google “Abstain and Validate” paper shows that refusing low‑confidence patches can improve success rates by up to 39 percentage points.  
- Compiler‑informed repair (e.g., SynthFix) shows that symbolic feedback dramatically improves patch quality – patch‑ts already has the tree‑sitter foundation to deliver this.  
- Patch‑ts has the opportunity to become the “last mile” validation service that every agentic coding workflow depends on.

**Business drivers:**  
- Become the standard patching backend for all major AI coding frameworks.  
- Eliminate the “looks valid but doesn’t compile” failure mode that erodes agent trust.  
- Enable a self‑healing loop where agents can retry with precise, actionable diagnostics.

## 4. Target Users / Customers

| Segment | Description |
|---------|-------------|
| **AI agent frameworks** | Claude Code, Codex CLI, Aider, OpenDev, Continue.dev, etc. – need a standardised, verifiable patching backend. |
| **Individual developers using AI** | Use LLMs for daily refactoring; want assurance that applied patches actually compile and work. |
| **DevOps / CI pipelines** | Need automated, gated patching with compilation‑level validation. |
| **Plugin authors** | Need the same robustness guarantees and semantic indexing in their custom repair plugins. |

**Explicitly NOT targeting (v1.4.0):**  
- Full program synthesis or automatic patch generation.  
- Cloud‑based collaboration platforms.  
- Replacement of language servers (LSP is complementary).

## 5. User Needs & Value Proposition

| Need | patch‑ts v1.4.0 Value |
|------|-----------------------|
| “I need AI agents to discover and use patch‑ts without custom integration.” | MCP server exposes all commands as standard tools, with structured schemas. |
| “The patch looked valid but my code no longer compiles.” | Post‑patch compilation validation runs language‑appropriate compilers and blocks/reports failures. |
| “When a patch fails, I need the agent to know exactly how to fix it.” | Self‑healing feedback generates structured retry prompts with precise diagnostics. |
| “I need to target patches by function name, not line number.” | Semantic symbol index enables function‑scoped patching for all supported languages. |
| “I want low‑confidence patches to be rejected automatically.” | Patch gating (abstention) refuses patches below a configurable confidence threshold. |
| “I need to validate patches against my project’s specification.” | Spec‑driven validation cross‑references tree‑sitter queries against specification documents. |
| “I need to chain multiple patch operations safely.” | Pipeline mode sequences multiple operations with gate policies at each stage. |

**Differentiator:** No other patching tool provides an MCP server, compilation‑level validation, self‑healing feedback, and semantic symbol indexing in a single, offline binary.

## 6. Desired Outcomes & Success Metrics

### Business Outcomes (v1.4.0)

| ID | Outcome | Key Result / Target |
|----|---------|---------------------|
| G‑1 | Become the standard patching backend | At least 5 major AI frameworks integrate patch‑ts via MCP within 6 months of release. |
| G‑2 | Eliminate “valid but doesn’t compile” failures | Reduce compilation‑failure rate of applied patches by ≥80% compared to v1.3.0 in benchmark suite. |
| G‑3 | Enable agent self‑correction | At least 70% of agent retries succeed after receiving self‑healing feedback (measured via telemetry opt‑in). |
| G‑4 | Increase user trust | Zero reports of silent corruption or undetected semantic errors within 6 months of release. |

### Product Outcomes (v1.4.0)

| ID | Outcome | Metric |
|----|---------|--------|
| P‑1 | MCP server is functional | All patch‑ts commands are callable via MCP with correct schema. |
| P‑2 | Compilation validation catches real errors | ≥95% of compilation‑breaking patches are detected and blocked. |
| P‑3 | Self‑healing feedback improves retry success | Structured retry prompts lead to ≥20% improvement in patch acceptance on retry. |
| P‑4 | Symbol index enables function‑scoped patching | Function‑scoped patching works for all 16 supported languages. |
| P‑5 | Patch gating reduces noise | Low‑confidence patches are rejected before modifying files, with clean error messages. |

## 7. Strategic Constraints

| Constraint | Description |
|------------|-------------|
| **Backward compatibility** | All v1.3.0 CLI flags, JSON schemas, and WASM plugin interfaces remain unchanged. |
| **Performance** | Compilation validation must complete in under 5 seconds for typical files; symbol indexing under 100ms. |
| **Cross‑platform** | MCP server must work on Linux, macOS, and Windows. |
| **Dependencies** | MCP implementation should use existing Rust MCP crates (e.g., `rmcp` or custom); compilation validation uses subprocess calls; no new heavy dependencies. |

## 8. Goals and Non‑Goals (v1.4.0)

### Goals

- Implement MCP server exposing all patch‑ts commands as standard tools.
- Add post‑patch compilation validation (cargo check, tsc --noEmit, etc.).
- Implement self‑healing feedback loop with structured retry prompts.
- Build semantic symbol index for all 16 languages.
- Implement patch gating (abstention) based on confidence threshold.
- Add spec‑driven validation (cross‑reference patches against specification documents).
- Implement pipeline mode for chained operations.
- Add learning from history (adaptive thresholds, fuzz radii).
- Update documentation, README, and MCP integration guide.

### Non‑Goals (explicitly excluded)

- Full program synthesis or automatic patch generation.
- Cloud‑based services; everything remains local.
- Replacement of existing language servers or compilers.
- Real‑time collaboration between multiple human users.

## 9. Operational Concept & High‑Level Scenarios

### Concept of Operations

An AI agent discovers patch‑ts as an MCP server. It invokes a patch operation with all necessary parameters. patch‑ts validates input, applies the patch using the confidence‑scored cascade, then runs post‑patch compilation validation. If compilation fails, patch‑ts diagnoses the errors, generates a structured retry prompt, and rejects the patch (or applies a partial fix if possible). If the patch is low‑confidence, it may be gated entirely. The agent receives rich JSON feedback including compilation status, symbol index results, and retry instructions.

### High‑Level Scenarios (v1.4.0)

1. **MCP server discovery**  
   Agent: `mcp.discover("patch‑ts")` → Returns list of available tools: `patch`, `balance`, `explain`, `validate`.

2. **Compilation‑gated patch**  
   Agent applies a patch that looks syntactically valid. `patch‑ts` runs `cargo check` and finds a type error. Patch is rejected with a diagnostic including the compiler error and a retry prompt: “Replace `timeout` with `Duration::from_secs(timeout)` in function `connect`.”

3. **Self‑healing loop**  
   Agent retries with the retry prompt. Second patch passes syntax, compilation, and semantic validation. Applied successfully.

4. **Spec‑driven validation**  
   Agent applies a patch that modifies API behavior. `patch‑ts` cross‑references the project specification (from `spec‑writer`) and warns: “Function signature changed: `get_user(id: i32)` → `get_user(id: u64)`. Specification expects `i32`.”

5. **Pipeline mode**  
   Agent runs `patch‑ts pipeline --stages "patch,validate,test,commit"`. Each stage gates the next; failure at any stage halts the pipeline with diagnostic.

## 10. Stakeholders, Sponsorship & Governance

| Role | Name / Org | Responsibility |
|------|------------|----------------|
| **Executive Sponsor** | Project maintainer | Approves strategic direction. |
| **Product Owner** | Project maintainer | Prioritizes features, manages scope. |
| **Engineering Lead** | Core contributor(s) | Oversees technical implementation. |
| **Community** | Open‑source contributors | Review PRs, test pre‑releases. |

## 11. Risks, Assumptions & Open Questions

### Assumptions

- The Rust MCP ecosystem will provide a stable, well‑maintained crate for server implementation. If not, a thin custom implementation is feasible.
- Compilation validation subprocess calls are fast enough for interactive use on typical codebases.
- Symbol index across 16 languages can be built using tree‑sitter queries without excessive memory or time.

### Risks

| Risk | Likelihood | Impact | Mitigation |
|------|------------|--------|------------|
| MCP crate may be immature or incompatible. | Medium | High | Evaluate options (`rmcp`, custom stdio‑based); fallback to basic JSON‑RPC. |
| Compilation validation may be too slow on large projects. | Medium | Medium | Allow opt‑out; cache incremental results; support `--no‑compile‑check` flag. |
| Symbol index may produce false positives/negatives. | Medium | Low | Start with best‑effort warnings; make it non‑blocking. |
| Spec‑driven validation requires spec format standardisation. | High | Medium | Support only the spec‑writer output format initially; expand later. |

### Open Questions

- Which MCP transport should patch‑ts support? (stdio first, HTTP later.)
- Should compilation validation be enabled by default? (Yes, with opt‑out.)
- How to handle multi‑language projects in a single pipeline? (Per‑file language detection.)

---

*This vision document anchors the platform‑grade patching strategy for patch‑ts v1.4.0.*
```

---

## 2. Business & Stakeholder Requirements Specification (BRS)

```markdown
# Business & Stakeholder Requirements Specification — patch‑ts v1.4.0

| Field | Value |
|-------|-------|
| Project | patch‑ts |
| Document | Business & Stakeholder Requirements Specification |
| Version | 1.0 |
| Date | 2026‑04‑23 |
| Author | patch‑ts team, assisted by spec‑writer |
| Status | Draft |
| References | Vision v1.4.0 |

## 1. Business Context

### 1.1 Purpose

This BRS defines the business‑level requirements for patch‑ts v1.4.0, which adds MCP server integration, post‑patch compilation validation, self‑healing feedback, semantic symbol indexing, patch gating, spec‑driven validation, and pipeline mode.

### 1.2 Business Problem / Opportunity

v1.3.0 hardened patch‑ts against malformed inputs and added confidence scoring, but patches that pass syntax validation may still fail when compiled. AI agents lack a standard way to discover and invoke patching tools, and when patches fail, the feedback is insufficient for agents to self‑correct. patch‑ts has the opportunity to become the universal, verifiable patching backend for the AI‑assisted development ecosystem.

### 1.3 Scope Boundaries

**In Scope:**
- MCP server exposing all patch‑ts commands as tools.
- Post‑patch compilation validation (Rust, TypeScript, JavaScript, Python, Go).
- Self‑healing feedback including structured retry prompts.
- Semantic symbol index for all 16 languages.
- Patch gating (abstention) based on confidence threshold.
- Spec‑driven validation against structured specification documents.
- Pipeline mode for chained operations.
- Learning from history (adaptive thresholds).

**Out of Scope:**
- Full program synthesis or automatic patch generation.
- Cloud‑based services.
- Replacement of language servers or compilers.
- Multi‑user collaboration.

## 2. Business Goals, Objectives & Success Metrics

| ID | Goal | Fit Criterion |
|----|------|---------------|
| BR‑001 | Become the standard patching backend | At least 5 major AI frameworks document MCP integration within 6 months. |
| BR‑002 | Eliminate “valid but doesn’t compile” failures | ≥80% reduction in compilation‑breaking patches in benchmark suite. |
| BR‑003 | Enable agent self‑correction | ≥70% of agent retries succeed after receiving self‑healing feedback. |
| BR‑004 | Increase user trust | Zero reports of silent corruption or undetected semantic errors within 6 months. |

*(Traceability: BR‑001…004 ← Vision G‑1…G‑4)*

## 3. Business Model & Processes

patch‑ts remains open‑source. MCP integration makes it a natural backend for AI agent frameworks, driving adoption and community contributions. Compilation validation and self‑healing feedback make it indispensable for production AI‑assisted workflows.

## 4. Business Rules & Policies

| ID | Rule | Source |
|----|------|--------|
| BR‑R1 | MCP server must be discoverable and provide schema for all tools. | Interoperability |
| BR‑R2 | Compilation validation must be enabled by default; opt‑out via `--no‑compile‑check`. | Safety |
| BR‑R3 | Low‑confidence patches (below `--confidence` threshold) must be rejected with structured feedback. | Noise reduction |
| BR‑R4 | Self‑healing feedback must include a `retry_prompt` field in JSON output. | Agent usability |

## 5–12. Additional Sections

*(Follow the same pattern as previous BRS documents: Glossary, Conceptual Domain Model, Stakeholder Needs, System‑in‑Context, Constraints, Risks, Traceability – summarized for brevity.)*

Key Stakeholder Needs:
- SN‑001: As an AI framework, I need to discover and invoke patch‑ts via standard MCP protocol.
- SN‑002: As a developer, I need patches to be validated at compilation level before being applied.
- SN‑003: As an AI agent, I need structured retry instructions when a patch fails.
- SN‑004: As a developer, I want low‑confidence patches to be refused rather than applied.
- SN‑005: As a project lead, I want patches to be cross‑referenced against the project specification.

---

*This BRS establishes the business foundation for v1.4.0.*
```

---

## 3. Software Requirements Specification (SRS)

```markdown
# Software Requirements Specification — patch‑ts v1.4.0

| Field | Value |
|-------|-------|
| Project | patch‑ts |
| Document | Software Requirements Specification |
| Version | 1.0 |
| Date | 2026‑04‑23 |
| Author | patch‑ts team, assisted by spec‑writer |
| Status | Draft |
| References | BRS v1.4.0, Vision v1.4.0 |

## 1. Introduction & Scope

This SRS defines the functional and non‑functional requirements for patch‑ts v1.4.0, which adds MCP server, compilation validation, self‑healing feedback, semantic symbol index, patch gating, spec‑driven validation, pipeline mode, and adaptive thresholds.

### 1.1 Scope

- MCP server module exposing all commands as tools.
- Post‑patch compilation validation for Rust, TypeScript, JavaScript, Python, Go.
- Self‑healing feedback with structured retry prompts.
- Semantic symbol index via tree‑sitter queries.
- Patch gating based on configurable confidence threshold.
- Spec‑driven validation using tree‑sitter queries against spec documents.
- Pipeline mode for chained operations.
- Adaptive thresholds based on history.
- Integration tests and documentation.

### 1.2 Out of Scope

- Full program synthesis or automatic patch generation.
- Cloud‑based services.
- Replacement of language servers or compilers.
- Multi‑user collaboration.

## 2. System Context & Overview

Same C1 context as before. New internal modules: `mcp.rs` (MCP server), `compile.rs` (compilation validation), `heal.rs` (self‑healing feedback), `symbols.rs` (symbol index), `pipeline.rs` (pipeline mode), `adaptive.rs` (adaptive thresholds).

## 3. Functional Capabilities & Behavior

### Feature: MCP Server

| ID | Requirement | Priority | Acceptance Criteria |
|----|-------------|----------|---------------------|
| FR‑MCP‑001 | `patch‑ts mcp` shall start an MCP server on stdio. | Must | Server responds to `tools/list` and `tools/call` requests. |
| FR‑MCP‑002 | The server shall expose `patch`, `balance`, `explain` as MCP tools with full JSON schemas. | Must | Each tool has input schema and returns structured output. |
| FR‑MCP‑003 | MCP server shall support `textDocument/didOpen`, `textDocument/didChange`, `textDocument/didSave` for integration with MCP‑aware editors. | Should | Real‑time diagnostics via MCP. |
| FR‑MCP‑004 | The server shall be compatible with the `rmcp` crate or equivalent. | Should | Builds without manual protocol implementation. |

### Feature: Post‑Patch Compilation Validation

| ID | Requirement | Priority | Acceptance Criteria |
|----|-------------|----------|---------------------|
| FR‑COMP‑001 | After a successful syntax‑valid patch, the tool shall run language‑appropriate compilation check. | Must | `cargo check` for Rust; `tsc --noEmit` for TypeScript; `node --check` for JavaScript; `python -m py_compile` for Python; `go build -o /dev/null` for Go. |
| FR‑COMP‑002 | If compilation fails, the patch shall be rolled back (file restored to original). | Must | File is not modified on compilation failure. |
| FR‑COMP‑003 | Compilation errors shall be parsed and included in JSON output. | Must | JSON includes `compilation_errors` array. |
| FR‑COMP‑004 | Compilation validation may be disabled with `--no‑compile‑check`. | Should | Flag honored. |
| FR‑COMP‑005 | Compilation validation shall time out after 30 seconds (configurable). | Should | Hanging builds don’t block tool. |

### Feature: Self‑Healing Feedback

| ID | Requirement | Priority | Acceptance Criteria |
|----|-------------|----------|---------------------|
| FR‑HEAL‑001 | When a patch fails, the JSON output shall include a `retry_prompt` field. | Must | Retry prompt includes file, line, error type, suggestion. |
| FR‑HEAL‑002 | Retry prompt shall be structured as a natural‑language instruction an LLM can consume. | Must | e.g., “At line 42 of src/main.rs, replace `timeout` with `Duration::from_secs(timeout)`.” |
| FR‑HEAL‑003 | Self‑healing shall categorise failures: content_mismatch, confidence_below_threshold, syntax_error, compilation_error, semantic_warning. | Must | Error codes match categories. |

### Feature: Semantic Symbol Index

| ID | Requirement | Priority | Acceptance Criteria |
|----|-------------|----------|---------------------|
| FR‑SYM‑001 | The tool shall build a symbol index for each file using tree‑sitter queries. | Must | Index includes function names, struct names, variable declarations. |
| FR‑SYM‑002 | Symbol index shall be used to support `--function <NAME>` scoping for all 16 languages. | Must | Function‑scoped balance and patch works for all languages. |
| FR‑SYM‑003 | Symbol index shall be cached in memory for the duration of a command. | Should | No duplicate parsing. |
| FR‑SYM‑004 | Symbol index shall power improved identifier cross‑validation warnings. | Should | Warnings list functions/variables not in symbol index. |

### Feature: Patch Gating (Abstention)

| ID | Requirement | Priority | Acceptance Criteria |
|----|-------------|----------|---------------------|
| FR‑GATE‑001 | If the confidence score of a patch is below the `--confidence` threshold (default 0.9), the patch shall be rejected entirely. | Must | File unchanged; JSON response indicates rejection with confidence score. |
| FR‑GATE‑002 | The rejection response shall include the threshold and actual confidence. | Must | e.g., `{"rejected": true, "confidence": 0.72, "threshold": 0.90}`. |
| FR‑GATE‑003 | Gating may be bypassed with `--force`. | Should | Force flag overrides gating. |

### Feature: Spec‑Driven Validation

| ID | Requirement | Priority | Acceptance Criteria |
|----|-------------|----------|---------------------|
| FR‑SPEC‑001 | `patch‑ts validate‑spec --file <spec.md> --code <file.rs>` shall check that code conforms to specification. | Could | Basic conformance checks. |
| FR‑SPEC‑002 | The tool shall parse spec‑writer output (Markdown with IDs) and cross‑reference with tree‑sitter queries. | Could | Warnings emitted for missing expected functions/types. |
| FR‑SPEC‑003 | Spec‑driven validation shall be non‑blocking (warnings only). | Could | Patches still apply; warnings included in JSON. |

### Feature: Pipeline Mode

| ID | Requirement | Priority | Acceptance Criteria |
|----|-------------|----------|---------------------|
| FR‑PIPE‑001 | `patch‑ts pipeline --stages "patch,validate,test,commit"` shall execute stages in sequence. | Should | Each stage gates the next. |
| FR‑PIPE‑002 | Pipeline stages shall be configurable: `patch`, `validate` (compilation), `test` (run test suite), `commit`. | Should | Custom stages via configuration. |
| FR‑PIPE‑003 | Pipeline shall support dry‑run (`--dry-run`). | Should | Shows which stages would pass/fail. |

### Feature: Adaptive Thresholds

| ID | Requirement | Priority | Acceptance Criteria |
|----|-------------|----------|---------------------|
| FR‑ADAPT‑001 | The tool shall record confidence scores in history and compute per‑project thresholds. | Could | After 10 patches, suggest a confidence threshold. |
| FR‑ADAPT‑002 | `patch‑ts suggest‑threshold` shall output a recommended `--confidence` value. | Could | Based on recent failure rates. |
| FR‑ADAPT‑003 | Adaptive thresholds shall be opt‑in via `--adaptive‑confidence`. | Could | Default is fixed threshold. |

## 4. Quality & Non‑Functional Requirements

| ID | Category | Requirement | Fit Criterion |
|----|----------|-------------|---------------|
| NFR‑PERF‑001 | Performance | Compilation validation must complete in ≤5s for typical files; timeout at 30s. | Benchmarked. |
| NFR‑PERF‑002 | Performance | Symbol index building must complete in ≤100ms per file. | Benchmarked. |
| NFR‑SEC‑001 | Security | MCP server must only accept local connections; no network exposure by default. | Security audit. |
| NFR‑COMPAT‑001 | Compatibility | All v1.3.0 tests pass without modification. | CI regression suite. |
| NFR‑DOC‑001 | Documentation | MCP integration guide, pipeline usage guide, and self‑healing feedback reference. | Review checklist. |

## 5. External Interfaces & Data Contracts

### CLI New/Modified Flags

- `mcp` subcommand
- `--no‑compile‑check`
- `--compile‑timeout <N>`
- `pipeline` subcommand with `--stages`
- `suggest‑threshold`
- `--adaptive‑confidence`

### JSON Output Schema (Additions)

```json
{
  "success": false,
  "rejected": true,
  "confidence": 0.72,
  "threshold": 0.90,
  "strategy": "fuzzy",
  "compilation_errors": [
    {
      "file": "src/main.rs",
      "line": 42,
      "column": 5,
      "message": "cannot find value `timeout`",
      "retry_prompt": "At line 42 of src/main.rs, replace `timeout` with `Duration::from_secs(timeout)`."
    }
  ],
  "symbol_index": {
    "functions": ["main", "connect"],
    "structs": ["Config"],
    "imports": ["std::time::Duration"]
  },
  "spec_warnings": [
    "Function signature changed: get_user(id: i32) -> get_user(id: u64). Spec expects i32."
  ]
}
```

## 6–8. Constraints, Assumptions, TBD

*(Summarized: must maintain backward compatibility, MCP crate availability, compilation tools present on user system.)*

---

*This SRS defines the complete behavioral contract for v1.4.0.*
```

---

## 4. Architecture & Design Specification

```markdown
# Architecture & Design Specification — patch‑ts v1.4.0

| Field | Value |
|-------|-------|
| Project | patch‑ts |
| Document | Architecture & Design Specification |
| Version | 1.0 |
| Date | 2026‑04‑23 |
| Author | patch‑ts team, assisted by spec‑writer |
| Status | Draft |
| References | SRS v1.4.0, BRS v1.4.0 |

## 1. Context & Scope

This document describes the architectural design for the platform‑grade features in v1.4.0: MCP server, compilation validation, self‑healing feedback, semantic symbol index, patch gating, spec‑driven validation, pipeline mode, and adaptive thresholds.

## 2. Goals & Non‑Goals

**Goals:**
- Introduce `mcp.rs` for Model Context Protocol server.
- Introduce `compile.rs` for post‑patch compilation validation.
- Introduce `heal.rs` for self‑healing feedback generation.
- Introduce `symbols.rs` for semantic symbol indexing.
- Introduce `pipeline.rs` for pipeline mode.
- Introduce `adaptive.rs` for adaptive thresholds.
- Extend `cli.rs` with new subcommands and flags.
- Keep all changes backward‑compatible.

**Non‑Goals:**
- Refactor the entire codebase.
- Introduce external services or heavy new dependencies beyond MCP crate.
- Replace existing validation pipeline; extend it.

## 3. Architecturally Significant Requirements (ASRs)

| ASR ID | Description | Source |
|--------|-------------|--------|
| ASR‑001 | MCP server must integrate with existing CLI commands without duplication. | FR‑MCP‑001 |
| ASR‑002 | Compilation validation must rollback on failure. | FR‑COMP‑002 |
| ASR‑003 | Self‑healing feedback must categorise failures. | FR‑HEAL‑003 |
| ASR‑004 | Symbol index must be built using tree‑sitter queries. | FR‑SYM‑001 |
| ASR‑005 | Pipeline mode must sequence operations with gating. | FR‑PIPE‑001 |

## 4. The Design

### 4.1 System Overview (C4 Level 2)

```
[AI Agent / MCP Client] → (MCP Server) → [patch‑ts Core]
                                               ├── compile.rs (validation)
                                               ├── heal.rs (feedback)
                                               ├── symbols.rs (index)
                                               ├── pipeline.rs (sequence)
                                               ├── adaptive.rs (learn)
                                               └── repair, patch, matching...
```

### 4.2 Key Design Changes

**MCP Server (`mcp.rs`)**
- Implement using the `rmcp` crate (or fallback to manual JSON‑RPC).
- Expose `tools/list` and `tools/call` handlers that delegate to existing command handlers.
- Define JSON schemas for all tools inline.

**Compilation Validation (`compile.rs`)**
- After a successful syntax‑only patch, spawn a subprocess for the language’s compiler.
- Parse output for error locations and messages.
- If errors exist, rollback the file and return diagnostics.
- Configurable timeout.

**Self‑Healing Feedback (`heal.rs`)**
- Given a failure category and error details, generate a `retry_prompt` string.
- Use a simple templating system: “At line {line} of {file}, replace `{old}` with `{new}`.” for content mismatch; “Compilation error: {message}. Consider fixing this before retrying.” for compilation errors.

**Semantic Symbol Index (`symbols.rs`)**
- For each supported language, define a tree‑sitter query that captures function definitions, struct declarations, variable declarations, and imports.
- Build a `SymbolIndex` struct per file and cache it.
- Expose `lookup_function(file, name) -> Option<Span>` to enable function‑scoped operations.

**Pipeline Mode (`pipeline.rs`)**
- Accept a list of stage names. Each stage is a function that returns `Result<()>`.
- Execute stages sequentially; if any fails, halt with diagnostic.
- Stages: `patch`, `validate` (compilation), `test` (run `just test`), `commit`.

**Adaptive Thresholds (`adaptive.rs`)**
- Read `.patch‑ts/history.jsonl` and compute success/failure rates per confidence level.
- Suggest a confidence threshold that would have rejected the fewest successes and accepted the fewest failures.
- Store suggested threshold in `.patch‑ts/adaptive.toml`.

### 4.3 Data Model

- New structs: `McpServer`, `CompileResult`, `HealFeedback`, `SymbolIndex`, `PipelineConfig`, `AdaptiveState`.
- Extend `BalanceResult` and `JsonDiagnostic` with new fields.

## 5. Architecture Decision Records (ADRs)

### ADR‑028: Use `rmcp` crate for MCP server

**Context:** Need to implement MCP protocol quickly and reliably.
**Decision:** Use the `rmcp` crate (Rust MCP) for server implementation.
**Alternatives:** Manual JSON‑RPC (too much work), `tower‑lsp`‑style (not designed for MCP).
**Consequences:** Adds dependency; if `rmcp` is immature, fallback to basic stdio JSON‑RPC.

### ADR‑029: Compilation validation as subprocess

**Context:** Need to check if patched code compiles.
**Decision:** Spawn language‑specific compiler subprocess and parse output.
**Alternatives:** Integrate compiler as library (complex, not available for all languages).
**Consequences:** Requires compiler toolchain installed; configurable timeout prevents hanging.

### ADR‑030: Symbol index using tree‑sitter queries

**Context:** Need function‑scoped operations for all languages.
**Decision:** Use per‑language tree‑sitter queries to extract symbols.
**Alternatives:** Language‑specific parsers (too complex), full LSP integration (overkill).
**Consequences:** Accurate for well‑formed code; may miss some dynamic constructs; best‑effort.

## 6. API & Interface Contracts

MCP tools schema defined in JSON. CLI extended with new subcommands. No changes to WASM plugin interface.

## 7. Cross‑cutting Concerns

- **Error Handling**: Compilation validation failures are user‑friendly and include retry prompts.
- **Testing**: Integration tests for MCP server, compilation validation with mock compilers, pipeline mode.
- **Performance**: Symbol index cached; compilation validation has timeout.

## 8. Alternatives Considered

| Alternative | Why Rejected |
|-------------|--------------|
| Integrate compilers as libraries | Not feasible across 5+ languages; subprocess is simpler and more universal. |
| Use LSP for symbol index | Too heavy; tree‑sitter queries are fast and sufficient for function‑scoped operations. |
| Cloud‑based compilation validation | Violates offline‑first principle; adds latency and complexity. |

## 9. Traceability

| ASR | ADR | Component |
|-----|-----|-----------|
| ASR‑001 | ADR‑028 | mcp.rs |
| ASR‑002 | ADR‑029 | compile.rs, patch.rs |
| ASR‑003 | ADR‑029, ADR‑030 | heal.rs |
| ASR‑004 | ADR‑030 | symbols.rs |
| ASR‑005 | – | pipeline.rs |

---

*This architecture specification provides the blueprint for implementing v1.4.0.*
```

---

## 5. Behavioral Specification & Test Verification Plan

```markdown
# Behavioral Specification & Test Verification Plan — patch‑ts v1.4.0

| Field | Value |
|-------|-------|
| Project | patch‑ts |
| Document | Behavioral Specification & Test Verification Plan |
| Version | 1.0 |
| Date | 2026‑04‑23 |
| Author | patch‑ts team, assisted by spec‑writer |
| Status | Draft |
| References | SRS v1.4.0, Architecture v1.4.0 |

## 1. Behavioral Specifications (Specification by Example)

### Feature: MCP Server

```gherkin
Feature: MCP server
  Scenario: List available tools
    Given patch‑ts is running in MCP mode
    When I send a `tools/list` request
    Then the response includes `patch`, `balance`, `explain` with schemas

  Scenario: Call patch tool via MCP
    Given a file "main.rs" with content needing a patch
    When I send a `tools/call` for `patch` with correct arguments
    Then the file is patched and the response includes confidence and strategy
```

### Feature: Compilation Validation

```gherkin
Feature: Compilation validation
  Scenario: Block patch that breaks compilation
    Given a valid Rust file
    When I apply a patch that introduces a type error (syntax valid, semantic invalid)
    Then the patch is blocked
    And the file is unchanged
    And the response includes `compilation_errors` with the type error

  Scenario: Allow patch that compiles
    Given a valid Rust file
    When I apply a patch that is semantically correct
    Then the patch is applied
    And `compilation_errors` is empty
```

### Feature: Self‑Healing Feedback

```gherkin
Feature: Self‑healing feedback
  Scenario: Retry prompt on content mismatch
    Given a file with "let x = 1;" at line 3
    When I patch with `--old "let y = 1;"` (mismatch)
    Then the response includes a `retry_prompt` field
    And the retry prompt mentions line 3 and the actual content

  Scenario: Retry prompt on compilation failure
    Given a patch that introduces a compilation error
    Then the response includes a `retry_prompt` field
    And the retry prompt includes the compiler error message
```

### Feature: Patch Gating

```gherkin
Feature: Patch gating
  Scenario: Low‑confidence patch rejected
    Given a patch with confidence 0.7
    When the confidence threshold is 0.9
    Then the patch is rejected
    And the file is unchanged
    And the response includes `rejected: true` and the confidence score

  Scenario: High‑confidence patch accepted
    Given a patch with confidence 0.95
    When the confidence threshold is 0.9
    Then the patch is applied
```

## 2. Test Strategy & Plan

### 2.1 Test Pyramid

| Level | Scope | Tools |
|-------|-------|-------|
| Unit | MCP handlers, compilation parser, feedback generator, symbol queries | Rust `#[test]` |
| Integration | Full MCP server, compilation validation with real compilers, pipeline mode | `assert_cmd`, `rmcp` test client |
| Property | Fuzzing MCP requests, adaptive threshold computation | `proptest` |
| Manual | MCP integration with real AI agents, spec‑driven validation | Charters |

### 2.2 Risk‑Based Prioritization

| Risk | Test Focus |
|------|------------|
| MCP server fails to start or respond | Integration tests with MCP client. |
| Compilation validation timeout or false positives | Mock slow compilers; test with known‑bad and known‑good patches. |
| Self‑healing feedback generation is misleading | Review by AI agent developers; iterate on templates. |

## 3. Test Case Specifications (Excerpt)

| TC‑ID | Requirement | Steps | Expected |
|-------|-------------|-------|----------|
| TC‑MCP‑001 | FR‑MCP‑001 | Start MCP server, send `initialize`, then `tools/list` | Response contains `tools` array |
| TC‑COMP‑001 | FR‑COMP‑001 | Apply syntactically valid but semantically broken patch to Rust file | Patch blocked; file unchanged |
| TC‑HEAL‑001 | FR‑HEAL‑001 | Trigger content mismatch; inspect JSON | `retry_prompt` field present and meaningful |
| TC‑GATE‑001 | FR‑GATE‑001 | Patch below confidence threshold; verify rejection | File unchanged; JSON indicates rejection |

## 4. NFR Verification

| NFR | Verification Method |
|-----|---------------------|
| NFR‑PERF‑001 | Benchmark compilation validation on 100‑line to 5000‑line files. |
| NFR‑PERF‑002 | Benchmark symbol index build on 16‑language corpus. |
| NFR‑SEC‑001 | Verify MCP server binds only to local stdio; no open ports. |
| NFR‑COMPAT‑001 | Run v1.3.0 test suite unchanged. |
| NFR‑DOC‑001 | Review MCP integration guide with external AI framework developer. |

## 5. Requirements Traceability Matrix (RTM)

*(Table mapping Vision/BRS objectives → SRS requirements → Test cases.)*
