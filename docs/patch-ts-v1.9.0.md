# patch‑ts v1.9.0 — Self‑Improving Governance Specification Suite

Below are the five core specification documents for **patch‑ts v1.9.0**, which adds evolutionary repair, streamable HTTP MCP transport, an agentic code‑review dashboard, CRA‑ready compliance evidence, curiosity‑driven test generation, and invariant‑based verification.

---

## 1. Vision & Strategic Alignment

```markdown
# Product Vision & Strategic Alignment — patch‑ts v1.9.0

| Field | Value |
|-------|-------|
| Project | patch‑ts |
| Document | Vision & Strategic Alignment |
| Version | 1.0 |
| Date | 2026‑04‑24 |
| Author | patch‑ts team, assisted by spec‑writer |
| Status | Draft |

## 1. Vision Statement

> **patch‑ts becomes a self‑improving governance platform that not only applies and validates patches, but learns from every change, evolves better fixes, autonomously generates tests, proves compliance with regulatory standards, and serves as a remote, scalable service for AI‑assisted development at any scale.**

## 2. Elevator Pitch (Moore Template)

> For **development teams and AI agent frameworks who need a patching platform that gets smarter over time, proves regulatory compliance automatically, and scales across thousands of repositories**, patch‑ts is a **tree‑sitter‑backed governance platform with evolutionary repair, streamable MCP transport, agentic code review, CRA‑ready compliance evidence, and autonomous test generation**. Unlike other patching tools that apply fixes statically, our product **learns from every patch to propose better fixes, runs a team of AI review agents per change, generates machine‑readable compliance reports, and creates tests for uncovered code** — making it the definitive self‑improving governance layer for AI‑augmented development.

## 3. Problem Statement & Business Context

**Problem:** v1.8.0 provided a knowledge graph, SCITT provenance, and parallel gates, but patches are still applied as one‑off operations with no learning from history. The MCP server only supports stdio, preventing remote deployment. Code review is still mostly manual, and compliance evidence (for the EU Cyber Resilience Act, CMMC 2.0, ISO 42001) requires manual assembly. There is no automatic test generation for patched code, and malicious patterns may slip through without agent‑facing security skills. Finally, there is no formal verification of patches — a growing need for safety‑critical systems.

**Why now:**
- **EvolRepair and TraceRepair** (ICSE 2026) prove that population‑based evolutionary repair and multi‑agent trace‑based validation significantly outperform single‑shot patching.  
- The **MCP 2026 Roadmap** prioritizes Streamable HTTP transport and task‑based workflows for production deployments.  
- The **EU Cyber Resilience Act** takes effect September 2026, requiring machine‑readable compliance evidence and SBOM generation.  
- **CovQValue and DryRUN** (arXiv Apr 2026) show that curiosity‑driven and LLM‑simulated test generation can achieve 51–77% higher branch coverage.  
- **COOP 2026** demonstrates that LLM‑generated loop invariants reach over 79% logical equivalence with expert specifications, making lightweight invariant verification feasible.  
- **Apiiro’s agent‑facing security CLI** (Apr 2026) proves that structured security skills for AI agents reduce vulnerability introduction rates.

**Business drivers:**
- Become the **first patching platform that learns and improves over time** — a competitive moat.  
- Enable **remote, scalable governance as a service** via Streamable HTTP MCP.  
- Achieve **regulatory compliance readiness** (CRA, CMMC 2.0, ISO 42001) out of the box.  
- Reduce the **test‑writing burden** for developers by generating coverage for patched code.  
- Provide **formal verification for safety‑critical patches** — a differentiator in aerospace, automotive, and medical software.

## 4. Target Users / Customers

| Segment | Description |
|---------|-------------|
| **Enterprise compliance teams** | Need automated CRA‑ready evidence and SBOM generation for every AI‑assisted change. |
| **AI agent frameworks** | Need a remote, scalable patching governance service with streamable MCP and async workflows. |
| **Safety‑critical software teams** | Need invariant‑based verification for patches in aerospace, automotive, medical. |
| **Platform engineering teams** | Need agentic code‑review dashboards and autonomous test generation to reduce manual work. |
| **Security teams** | Need agent‑facing security skills that block malicious patches before they reach production. |
| **Individual developers** | Benefit from evolutionary repair that gets smarter with every patch. |

**Explicitly NOT targeting (v1.9.0):**
- Full ML model training for repair (the evolutionary algorithm uses existing LLM‑generated candidates, not model fine‑tuning).
- Cloud‑based orchestration beyond Streamable HTTP MCP.
- Blockchain‑based provenance (SCITT uses traditional PKI, already implemented in v1.8).

## 5. User Needs & Value Proposition

| Need | patch‑ts v1.9.0 Value |
|------|-----------------------|
| “I want patch‑ts to get smarter with every patch, not start from scratch.” | Evolutionary repair maintains a population of repair strategies and synthesizes better fixes over time. |
| “I need to deploy patch‑ts as a remote service, not just a local CLI.” | Streamable HTTP MCP transport enables remote, scalable governance with async task‑based workflows. |
| “I want a team of AI agents to review every patch — not just a gate check.” | Agentic code‑review dashboard dispatches syntax, compile, coverage, security, and style agents per change. |
| “I need automated compliance evidence for the EU Cyber Resilience Act.” | `patch‑ts attest` generates machine‑readable compliance reports and SBOMs for every patched file. |
| “I don’t have time to write tests for patched code — can patch‑ts do it?” | Curiosity‑driven test generation explores branches and creates coverage for changed code. |
| “I need to know if a patch breaks any invariants in my safety‑critical system.” | Invariant‑based verification checks loop invariants, pre/post-conditions, and type constraints. |
| “I want AI agents to automatically block malicious patches before they’re applied.” | Agent‑facing security skill scans every proposed patch for injection, traversal, and exfiltration patterns. |

**Differentiator:** No other tool provides evolutionary repair, streamable MCP governance, CRA‑ready compliance, autonomous test generation, invariant verification, and agent‑facing security — all in a single, learning platform.

## 6. Desired Outcomes & Success Metrics

### Business Outcomes (v1.9.0)

| ID | Outcome | Key Result / Target |
|----|---------|---------------------|
| G‑1 | Achieve regulatory compliance readiness | `patch‑ts attest` produces a valid CRA‑compliant evidence report for 100% of patched files in a benchmark of 100 changes. |
| G‑2 | Enable remote, scalable governance | At least 3 enterprises deploy patch‑ts via Streamable HTTP MCP within 6 months. |
| G‑3 | Improve patch quality over time | Evolutionary repair achieves ≥15% higher test‑passing rate than single‑shot patching in a Defects4J‑style benchmark. |

### Product Outcomes (v1.9.0)

| ID | Outcome | Metric |
|----|---------|--------|
| P‑1 | Evolutionary repair works | On a benchmark of 100 historical patches, `patch‑ts evolve` proposes a correct fix in ≥85% of cases (vs. ≤70% for single‑shot). |
| P‑2 | Streamable HTTP MCP transport | A remote MCP client can submit a patch, poll for results, and receive a completed gate report within 5 seconds for a typical file. |
| P‑3 | Agentic code‑review dashboard | A user can see findings from 5 review agents (syntax, compile, coverage, security, style) in a single TUI or HTML dashboard. |
| P‑4 | CRA‑ready compliance evidence | An exported attestation report passes validation against a CRA schema checklist. |
| P‑5 | Curiosity‑driven test generation | Generated tests achieve ≥50% branch coverage on patched code in a benchmark of 50 Rust functions. |

## 7. Strategic Constraints

| Constraint | Description |
|------------|-------------|
| **Backward compatibility** | All v1.8.0 CLI flags, JSON schemas, MCP interface, and WASM plugin interfaces remain unchanged. |
| **Performance** | Evolutionary repair must complete in ≤10s for a population of 10 candidates on a typical file; Streamable HTTP MCP must handle ≥50 concurrent connections. |
| **Cross‑platform** | All new features must work on Linux, macOS, and Windows. |
| **Dependencies** | No new heavy dependencies; evolutionary repair uses existing matching engine and gate pipeline; Streamable HTTP uses `hyper` or `actix‑web` (already in dependency tree via `reqwest`); compliance reports are JSON‑based; test generation uses existing tree‑sitter and coverage modules. |
| **Privacy** | All data remains local; compliance reports are generated locally and never transmitted. |

## 8. Goals and Non‑Goals (v1.9.0)

### Goals

- Implement **Evolutionary Repair Mode** — `patch‑ts evolve` with population‑based semantic evolution.
- Implement **Streamable HTTP MCP Transport** — add HTTP transport alongside stdio for the MCP server.
- Implement **Agentic Code‑Review Dashboard** — multi‑agent findings aggregation in TUI and MCP App.
- Implement **CRA‑Ready Compliance Evidence** — `patch‑ts attest` generating SBOM, provenance, and validation reports.
- Implement **Curiosity‑Driven Test Generation** — `patch‑ts generate‑tests` with Bayesian coverage exploration.
- Implement **Agent‑Facing Security Skill** — structured command definitions and automated remediation loops.
- Implement **Invariant‑Based Patch Verification** — lightweight model checking for loop/function invariants.
- Update documentation, README, compliance guide, and MCP integration guide.

### Non‑Goals (explicitly excluded)

- Full ML model training for repair.
- Cloud‑based orchestration services beyond Streamable HTTP MCP.
- Blockchain‑based provenance.
- Real‑time collaborative editing across multiple human users.

## 9. Operational Concept & High‑Level Scenarios

### Concept of Operations

A developer or AI agent proposes a patch. patch‑ts runs evolutionary repair — generating multiple candidate fixes, evaluating them against the knowledge graph, test suite, and gate pipeline, and synthesizing the best combination. The patch is then reviewed by a team of AI agents (syntax, compile, coverage, security, style), and the results are presented in an interactive dashboard. Compliance evidence is automatically generated, and coverage gaps are filled by curiosity‑driven test generation. For safety‑critical patches, invariant verification checks that loop boundaries and pre/post-conditions are preserved. All operations are accessible via Streamable HTTP MCP for remote, scalable deployment.

### High‑Level Scenarios (v1.9.0)

1. **Evolutionary repair improves a patch over time**  
   An AI agent proposes a patch that fails the gate pipeline. `patch‑ts evolve` generates five alternative candidates, evaluates them, and selects one that passes all gates. Over time, the system learns which strategies work best for this codebase.

2. **Remote governance via Streamable HTTP MCP**  
   An enterprise deploys patch‑ts as a governance service. CI/CD pipelines submit patches via HTTP, poll for results, and receive structured gate reports with compliance evidence — all without installing patch‑ts locally.

3. **Agentic code review on a pull request**  
   A PR is opened. patch‑ts dispatches five review agents (syntax, compile, coverage, security, style). The agentic dashboard shows inline annotations, severity rankings, and an “Approve”/“Request Changes” button.

4. **Automatic compliance evidence for CRA audit**  
   A compliance officer runs `patch‑ts attest --since "2026‑01‑01"` and receives a machine‑readable JSON‑LD report containing SBOM, provenance chain, validation results, and CRA‑required metadata — ready for auditor submission.

5. **Test generation for uncovered patched code**  
   A patch changes a function that has 0% test coverage. `patch‑ts generate‑tests` uses Bayesian exploration to create three test cases that cover the changed lines, achieving 80% branch coverage.

6. **Invariant verification catches a broken loop**  
   A safety‑critical patch changes a loop condition. `patch‑ts verify` checks the loop invariant, detects a violation, and blocks the patch with a detailed report.

## 10. Stakeholders, Sponsorship & Governance

| Role | Name / Org | Responsibility |
|------|------------|----------------|
| **Executive Sponsor** | Project maintainer | Approves strategic direction. |
| **Product Owner** | Project maintainer | Prioritizes features, manages scope. |
| **Engineering Lead** | Core contributor(s) | Oversees technical implementation. |
| **Community** | Open‑source contributors | Review PRs, test pre‑releases. |

## 11. Risks, Assumptions & Open Questions

### Assumptions

- The evolutionary algorithm can use the existing confidence‑scored cascade and gate pipeline without modification.  
- `hyper` or `actix‑web` (already in the dependency tree via `reqwest`) can serve as the Streamable HTTP MCP transport without significant additional binary size.  
- The CRA compliance schema stabilizes sufficiently to produce valid reports; if not, we target the draft spec with a migration path.  
- Curiosity‑driven test generation can use the existing tree‑sitter module and knowledge graph to identify relevant branches.

### Risks

| Risk | Likelihood | Impact | Mitigation |
|------|------------|--------|------------|
| Evolutionary algorithm may be too slow for interactive use. | Medium | Medium | Limit population size and iteration count; provide `‑‑evolve‑timeout` flag. |
| Streamable HTTP MCP may introduce security vulnerabilities. | Low | High | Bind to localhost by default; require TLS for remote connections; add authentication token support. |
| Compliance schema may change before CRA enforcement. | Low | Medium | Version the attestation format; provide migration tools. |
| Curiosity‑driven test generation may produce invalid or flaky tests. | Medium | Medium | Run generated tests through the gate pipeline; reject failing tests. |

### Open Questions

- Should evolutionary repair be opt‑in or always‑on? (Opt‑in via `‑‑evolve` flag, with a `‑‑population‑size` parameter.)  
- Should Streamable HTTP MCP replace stdio or be additive? (Additive; both transports available.)  
- How deep should invariant verification go? (Start with loop invariants and function pre/post-conditions; extend to type constraints in v1.10.)

---

*This vision document anchors the self‑improving governance strategy for patch‑ts v1.9.0.*
```

---

## 2. Business & Stakeholder Requirements Specification (BRS)

```markdown
# Business & Stakeholder Requirements Specification — patch‑ts v1.9.0

| Field | Value |
|-------|-------|
| Project | patch‑ts |
| Document | Business & Stakeholder Requirements Specification |
| Version | 1.0 |
| Date | 2026‑04‑24 |
| Author | patch‑ts team, assisted by spec‑writer |
| Status | Draft |
| References | Vision v1.9.0 |

## 1. Business Context

### 1.1 Purpose

This BRS defines the business‑level requirements for patch‑ts v1.9.0, which adds evolutionary repair, streamable HTTP MCP transport, agentic code review, CRA‑ready compliance evidence, curiosity‑driven test generation, agent‑facing security skills, and invariant‑based verification.

### 1.2 Business Problem / Opportunity

v1.8.0 provided a knowledge graph and SCITT provenance, but patches are still one‑off operations with no learning, the MCP server only supports local stdio, compliance evidence is manual, test coverage is the developer’s responsibility, and security is reactive. The business opportunity is to become the first self‑improving, remotely‑deployable governance platform that learns from every patch, autonomously generates tests, proves compliance automatically, and blocks malicious changes proactively.

### 1.3 Scope Boundaries

**In Scope:**
- Evolutionary repair mode (`patch‑ts evolve`).
- Streamable HTTP MCP transport.
- Agentic code‑review dashboard.
- CRA‑ready compliance evidence (`patch‑ts attest`).
- Curiosity‑driven test generation (`patch‑ts generate‑tests`).
- Agent‑facing security skill.
- Invariant‑based patch verification (`patch‑ts verify`).
- Documentation and compliance guides.

**Out of Scope:**
- Full ML model training for repair.
- Cloud‑based orchestration beyond Streamable HTTP MCP.
- Blockchain‑based provenance.
- Real‑time collaborative editing.

## 2. Business Goals, Objectives & Success Metrics

| ID | Goal | Fit Criterion |
|----|------|---------------|
| BR‑001 | Achieve regulatory compliance readiness | `patch‑ts attest` produces valid CRA evidence for 100% of patched files in benchmark. |
| BR‑002 | Enable remote, scalable governance | At least 3 enterprises deploy Streamable HTTP MCP within 6 months. |
| BR‑003 | Improve patch quality over time | Evolutionary repair achieves ≥15% higher test‑passing rate vs. single‑shot in benchmark. |

*(Traceability: BR‑001…003 ← Vision G‑1…G‑3)*

## 3. Business Model & Processes

patch‑ts remains open‑source. Self‑improving governance features make it indispensable for enterprises needing compliance and safety‑critical teams needing verification, driving adoption and community contributions.

## 4. Business Rules & Policies

| ID | Rule | Source |
|----|------|--------|
| BR‑R1 | Evolutionary repair must never apply a patch without explicit `‑‑apply` (dry‑run by default). | Safety |
| BR‑R2 | Streamable HTTP MCP must bind to localhost by default; remote binding requires explicit configuration and TLS. | Security |
| BR‑R3 | Compliance evidence must be generated locally and never transmitted without user action. | Privacy |
| BR‑R4 | Generated tests must pass the existing test suite before being saved. | Quality |

## 5–12. Additional Sections

*(Follow the same pattern as previous BRS documents: Glossary, Conceptual Domain Model, Stakeholder Needs, System‑in‑Context, Constraints, Risks, Traceability.)*

Key Stakeholder Needs:
- SN‑001: As a developer, I want patch‑ts to learn from every patch and propose better fixes over time.
- SN‑002: As a platform engineer, I want to deploy patch‑ts as a remote governance service accessible via HTTP.
- SN‑003: As a team lead, I want a dashboard showing multi‑agent review findings for every change.
- SN‑004: As a compliance officer, I want automated CRA‑ready compliance evidence for every AI‑assisted change.
- SN‑005: As a developer, I want patch‑ts to generate tests for the code I changed.
- SN‑006: As a security engineer, I want AI agents to automatically detect and block malicious patches.
- SN‑007: As a safety‑critical systems engineer, I want to verify that patches preserve loop invariants and function contracts.

---

*This BRS establishes the business foundation for v1.9.0.*
```

---

## 3. Software Requirements Specification (SRS)

```markdown
# Software Requirements Specification — patch‑ts v1.9.0

| Field | Value |
|-------|-------|
| Project | patch‑ts |
| Document | Software Requirements Specification |
| Version | 1.0 |
| Date | 2026‑04‑24 |
| Author | patch‑ts team, assisted by spec‑writer |
| Status | Draft |
| References | BRS v1.9.0, Vision v1.9.0 |

## 1. Introduction & Scope

This SRS defines the functional and non‑functional requirements for patch‑ts v1.9.0, which adds evolutionary repair, Streamable HTTP MCP, agentic code review, CRA compliance, curiosity‑driven test generation, agent‑facing security, and invariant verification.

### 1.1 Scope

- `src/evolve.rs` — evolutionary repair engine.
- `src/mcp_http.rs` — Streamable HTTP MCP transport.
- `src/review.rs` — agentic code‑review dashboard engine.
- `src/attest.rs` — CRA‑ready compliance evidence generator.
- `src/curiosity.rs` — curiosity‑driven test generation.
- `src/invariant.rs` — invariant‑based patch verification.
- Upgrades to `src/audit.rs` — agent‑facing security skill.
- CLI subcommands: `evolve`, `attest`, `generate‑tests`, `verify`.
- Integration tests, documentation, compliance guides.

### 1.2 Out of Scope

- Full ML model training.
- Cloud‑based orchestration beyond Streamable HTTP MCP.
- Blockchain‑based provenance.
- Real‑time collaborative editing.

## 2. System Context & Overview

Same C1 context as before. New internal modules: `evolve.rs`, `mcp_http.rs`, `review.rs`, `attest.rs`, `curiosity.rs`, `invariant.rs`. Upgraded modules: `audit.rs`, `mcp.rs`.

## 3. Functional Capabilities & Behavior

### Feature: Evolutionary Repair Mode

| ID | Requirement | Priority | Acceptance Criteria |
|----|-------------|----------|---------------------|
| FR‑EVO‑001 | `patch‑ts evolve --file <FILE> --old <OLD> --new <NEW>` shall generate a population of N candidate patches (default N=10). | Must | Candidates are generated using the existing cascade strategies, with parameter variations. |
| FR‑EVO‑002 | The population shall be evaluated against the gate pipeline (syntax, compile, cross‑file, coverage, security). | Must | Each candidate receives a fitness score based on gate pass/fail. |
| FR‑EVO‑003 | The best candidate (highest fitness) shall be presented as the recommended patch, with a ranked list of alternatives. | Must | JSON output includes `recommended` and `alternatives` arrays. |
| FR‑EVO‑004 | Over successive invocations, the system shall record which strategies succeeded and bias future populations toward successful strategies. | Should | After 50 patches, the initial population reflects learned strategy weights. |
| FR‑EVO‑005 | `‑‑population‑size <N>` shall control the number of candidates; `‑‑evolve‑timeout <S>` shall limit total evolution time. | Should | Flags honored. |

### Feature: Streamable HTTP MCP Transport

| ID | Requirement | Priority | Acceptance Criteria |
|----|-------------|----------|---------------------|
| FR‑MCP‑HTTP‑001 | `patch‑ts mcp‑http --port <PORT>` shall start an HTTP MCP server on the specified port. | Must | Server responds to POST requests with JSON‑RPC bodies. |
| FR‑MCP‑HTTP‑002 | The server shall support `tools/list`, `tools/call`, `resources/list`, `resources/read`, and `sampling/createMessage` — identical semantics to stdio MCP. | Must | All stdio MCP capabilities work over HTTP. |
| FR‑MCP‑HTTP‑003 | The server shall bind to `127.0.0.1` by default; the `‑‑bind <ADDR>` flag shall allow custom binding. | Must | Security: no external access by default. |
| FR‑MCP‑HTTP‑004 | The server shall support an optional `‑‑auth‑token <TOKEN>` for bearer authentication. | Should | Unauthorized requests return 401. |

### Feature: Agentic Code‑Review Dashboard

| ID | Requirement | Priority | Acceptance Criteria |
|----|-------------|----------|---------------------|
| FR‑REVIEW‑001 | `patch‑ts review --file <FILE> --old <OLD> --new <NEW>` shall run syntax, compile, coverage, security, and style agents on the patch and aggregate findings. | Must | Output includes per‑agent findings with severity and inline annotations. |
| FR‑REVIEW‑002 | The aggregated result shall be displayed in the TUI (`‑‑tui`) and MCP App (`‑‑dashboard`). | Should | TUI and HTML dashboard show multi‑agent findings. |
| FR‑REVIEW‑003 | The review shall support “Approve” / “Request Changes” workflow, with structured feedback returned to the caller. | Should | JSON output includes `action` field. |

### Feature: CRA‑Ready Compliance Evidence

| ID | Requirement | Priority | Acceptance Criteria |
|----|-------------|----------|---------------------|
| FR‑ATTEST‑001 | `patch‑ts attest --since <DATE> --output <FILE>` shall generate a JSON‑LD compliance evidence report. | Must | Report includes SBOM, provenance chain, validation results, and CRA metadata. |
| FR‑ATTEST‑002 | The report shall include a SHA‑256 hash chain for all patched files, SCITT provenance records, and gate validation results. | Must | Report passes validation against CRA schema checklist. |
| FR‑ATTEST‑003 | The report shall be exportable in JSON‑LD and PDF (via `‑‑format` flag). | Should | Multiple formats supported. |

### Feature: Curiosity‑Driven Test Generation

| ID | Requirement | Priority | Acceptance Criteria |
|----|-------------|----------|---------------------|
| FR‑CURI‑001 | `patch‑ts generate‑tests --file <FILE> --old <OLD> --new <NEW>` shall generate test cases for changed code. | Must | At least one test case per changed function is generated. |
| FR‑CURI‑002 | The generator shall use Bayesian exploration: query the knowledge graph for branch structure, generate test inputs that maximize branch coverage, and iterate. | Should | Generated tests achieve ≥50% branch coverage on patched code in benchmark. |
| FR‑CURI‑003 | Generated tests shall be saved to a test file and run through the existing test suite to verify they pass. | Must | Tests that fail are discarded and reported. |

### Feature: Agent‑Facing Security Skill

| ID | Requirement | Priority | Acceptance Criteria |
|----|-------------|----------|---------------------|
| FR‑SEC‑SKILL‑001 | `patch‑ts audit` shall expose a structured security skill definition (JSON) that AI agents can introspect. | Must | Skill includes input schema, parameter descriptions, and output format. |
| FR‑SEC‑SKILL‑002 | The `‑‑audit‑security` flag on `patch`, `balance`, `gate`, and `evolve` shall run the security agent and block patches with critical findings. | Must | Critical findings prevent patch application. |
| FR‑SEC‑SKILL‑003 | The security agent shall support automated remediation suggestions for common vulnerability patterns. | Should | JSON output includes `remediation` field. |

### Feature: Invariant‑Based Patch Verification

| ID | Requirement | Priority | Acceptance Criteria |
|----|-------------|----------|---------------------|
| FR‑INV‑001 | `patch‑ts verify --file <FILE>` shall extract loop invariants and function pre/post-conditions from source code using tree‑sitter annotations. | Must | Invariants extracted from specially‑formatted comments (`//@ invariant`, `//@ requires`, `//@ ensures`). |
| FR‑INV‑002 | After a patch is applied, the tool shall check that all extracted invariants still hold by symbolic evaluation. | Should | Violated invariants are reported with the violating line and expression. |
| FR‑INV‑003 | Invariant verification shall be integrated as a gate stage (`invariant`) in the gate pipeline. | Should | Gate stage fails if any invariant is violated. |

## 4. Quality & Non‑Functional Requirements

| ID | Category | Requirement | Fit Criterion |
|----|----------|-------------|---------------|
| NFR‑PERF‑001 | Performance | Evolutionary repair must complete in ≤10s for a population of 10 candidates on a 500‑line file. | Benchmarked. |
| NFR‑PERF‑002 | Performance | Streamable HTTP MCP must handle ≥50 concurrent connections with <100ms latency overhead. | Load tested. |
| NFR‑SEC‑001 | Security | HTTP MCP server must bind to localhost by default; remote binding must require TLS and authentication. | Security audit. |
| NFR‑COMPAT‑001 | Compatibility | All v1.8.0 tests pass without modification. | CI regression suite. |
| NFR‑DOC‑001 | Documentation | Compliance guide, evolutionary repair guide, and MCP HTTP deployment guide. | Review checklist. |

## 5. External Interfaces & Data Contracts

### CLI New/Modified Flags

- `evolve` subcommand: `‑‑file`, `‑‑old`, `‑‑new`, `‑‑population‑size <N>`, `‑‑evolve‑timeout <S>`, `‑‑apply`.
- `mcp‑http` subcommand: `‑‑port <PORT>`, `‑‑bind <ADDR>`, `‑‑auth‑token <TOKEN>`.
- `review` subcommand: `‑‑file`, `‑‑old`, `‑‑new`, `‑‑tui`, `‑‑dashboard`.
- `attest` subcommand: `‑‑since <DATE>`, `‑‑output <FILE>`, `‑‑format <jsonld|pdf>`.
- `generate‑tests` subcommand: `‑‑file`, `‑‑old`, `‑‑new`, `‑‑output <DIR>`.
- `verify` subcommand: `‑‑file`.
- `‑‑audit‑security` flag on `patch`, `balance`, `gate`, `evolve`.
- `invariant` gate stage.

### JSON Output Schema (Additions)

```json
{
  "evolve": {
    "recommended": { "old": "...", "new": "...", "fitness": 0.92 },
    "alternatives": [ ... ],
    "population_size": 10,
    "generations": 3
  },
  "review": {
    "findings": [
      { "agent": "syntax", "severity": "pass", "details": "AST valid" },
      { "agent": "security", "severity": "critical", "details": "SQL injection pattern at line 42", "remediation": "Use parameterized queries" }
    ],
    "action": "request_changes"
  },
  "attest": {
    "report_id": "attest‑2026‑04‑24‑001",
    "cra_compliant": true,
    "sbom": { "files": [ ... ], "hashes": [ ... ] },
    "provenance_chain": [ ... ],
    "validation_results": [ ... ]
  },
  "generate_tests": {
    "tests": [ "fn test_foo() { ... }" ],
    "branch_coverage_before": 0.0,
    "branch_coverage_after": 0.85
  },
  "verify": {
    "invariants_checked": 3,
    "violations": [
      { "invariant": "x > 0", "line": 42, "expression": "x = -1" }
    ]
  }
}
```

## 6–8. Constraints, Assumptions, TBD

*(Summarized: must maintain backward compatibility, CRA schema stable, `hyper`/`actix‑web` already in dependency tree, evolutionary algorithm uses existing components.)*

---

*This SRS defines the complete behavioral contract for v1.9.0.*
```

---

## 4. Architecture & Design Specification

```markdown
# Architecture & Design Specification — patch‑ts v1.9.0

| Field | Value |
|-------|-------|
| Project | patch‑ts |
| Document | Architecture & Design Specification |
| Version | 1.0 |
| Date | 2026‑04‑24 |
| Author | patch‑ts team, assisted by spec‑writer |
| Status | Draft |
| References | SRS v1.9.0, BRS v1.9.0 |

## 1. Context & Scope

This document describes the architectural design for the self‑improving governance features in v1.9.0: evolutionary repair, Streamable HTTP MCP, agentic code review, CRA compliance, curiosity‑driven test generation, and invariant verification.

## 2. Goals & Non‑Goals

**Goals:**
- Introduce `evolve.rs` for population‑based repair using existing gate pipeline.
- Introduce `mcp_http.rs` for Streamable HTTP transport using `hyper`.
- Introduce `review.rs` for multi‑agent review aggregation.
- Introduce `attest.rs` for CRA‑compliant JSON‑LD report generation.
- Introduce `curiosity.rs` for Bayesian test generation.
- Introduce `invariant.rs` for tree‑sitter‑based invariant extraction and checking.
- Upgrade `audit.rs` with structured skill definition.
- Keep all changes backward‑compatible.

**Non‑Goals:**
- Full ML model training.
- Cloud‑based orchestration beyond Streamable HTTP MCP.
- Blockchain‑based provenance.

## 3. Architecturally Significant Requirements (ASRs)

| ASR ID | Description | Source |
|--------|-------------|--------|
| ASR‑001 | Evolutionary repair must use the existing gate pipeline for fitness evaluation. | FR‑EVO‑002 |
| ASR‑002 | HTTP MCP must reuse the existing MCP handler logic with a new transport layer. | FR‑MCP‑HTTP‑002 |
| ASR‑003 | Agentic review must aggregate findings from independently running agents. | FR‑REVIEW‑001 |
| ASR‑004 | Compliance reports must be self‑contained JSON‑LD with no external dependencies. | FR‑ATTEST‑001 |
| ASR‑005 | Test generation must use the knowledge graph for branch discovery. | FR‑CURI‑002 |

## 4. The Design

### 4.1 System Overview (C4 Level 2)

```
[User/Agent] → [CLI] → [evolve.rs] [review.rs] [attest.rs] [curiosity.rs] [invariant.rs] [audit.rs]
                  ↓         ↓            ↓          ↓           ↓              ↓          ↓
              [gate.rs]   [gate agents]  [provenance] [knowledge] [tree‑sitter] [regex patterns]
```

### 4.2 Key Design Changes

**Evolutionary Repair (`evolve.rs`)**
- `run_evolution(file, old, new, population_size, timeout) -> EvolveResult`
- Generates a population by varying cascade parameters (fuzz, confidence, uniqueness_weight).
- Each candidate is evaluated by `gate::run_gate_parallel`.
- Fitness = weighted sum of (syntax_pass * 1.0 + compile_pass * 2.0 + coverage_pass * 1.5 + security_pass * 3.0).
- Best candidate returned; strategy weights updated in `.patch‑ts/evolve‑weights.json`.

**Streamable HTTP MCP (`mcp_http.rs`)**
- Uses `hyper::Server` to listen for POST requests at `/mcp`.
- Each request body is parsed as JSON‑RPC, dispatched to the same handler functions as stdio MCP.
- Response returned as JSON.
- Authentication via `Authorization: Bearer <token>` header.

**Agentic Code Review (`review.rs`)**
- `run_review(file, old, new) -> ReviewResult`
- Spawns syntax, compile, coverage, security, and style agents as parallel tasks (reusing gate stage functions).
- Aggregates findings into a `ReviewReport` with per‑agent severity and inline annotations.

**CRA Compliance (`attest.rs`)**
- `generate_attestation(since, output) -> AttestationReport`
- Collects all provenance records since date, generates SBOM (list of files + SHA‑256 hashes), includes validation results, formats as JSON‑LD.

**Curiosity‑Driven Test Generation (`curiosity.rs`)**
- `generate_tests(file, old, new) -> GeneratedTests`
- Uses knowledge graph to identify changed functions and their branch structure.
- Generates test inputs using boundary value analysis and random exploration.
- Selects tests that maximize branch coverage (Bayesian optimization).

**Invariant Verification (`invariant.rs`)**
- `extract_invariants(source) -> Vec<Invariant>`
- Parses specially‑formatted comments (`//@ invariant`, `//@ requires`, `//@ ensures`).
- `verify_invariants(old, new, invariants) -> Vec<Violation>`
- Uses simple symbolic evaluation: substitutes new values into invariant expressions and checks satisfiability.

**Security Skill (`audit.rs` upgrade)**
- Add `get_skill_definition() -> Value` that returns a JSON Schema for the audit tool.
- Add `remediate_finding(finding) -> Option<String>` that returns a suggested fix.

### 4.3 Data Model

- `EvolveResult { recommended: PatchCandidate, alternatives: Vec<PatchCandidate>, generations: usize }`
- `ReviewReport { findings: Vec<AgentFinding>, action: String }`
- `AttestationReport { report_id, cra_compliant, sbom, provenance_chain, validation_results }`
- `GeneratedTests { tests: Vec<String>, branch_coverage: f64 }`
- `Invariant { kind, expression, line }`
- `Violation { invariant, line, expression }`

## 5. Architecture Decision Records (ADRs)

### ADR‑045: Use existing gate pipeline for evolutionary fitness

**Context:** Need to evaluate candidate patches in the evolutionary algorithm.  
**Decision:** Reuse `gate::run_gate_parallel` with weighted scoring for fitness.  
**Alternatives:** Build a separate evaluation framework — redundant.  
**Consequences:** Leverages existing code; fitness scores are transparent and explainable.

### ADR‑046: Use `hyper` for Streamable HTTP MCP

**Context:** Need an HTTP transport for MCP.  
**Decision:** Use `hyper` (already in dependency tree via `reqwest`) with a simple JSON‑RPC handler.  
**Alternatives:** `actix‑web` (heavier), `tiny_http` (less maintained).  
**Consequences:** Minimal additional binary size; production‑grade performance.

### ADR‑047: Use Bayesian exploration for test generation

**Context:** Need to generate tests that maximize branch coverage.  
**Decision:** Implement a simple Bayesian optimizer (Gaussian Process or Upper Confidence Bound) that selects test inputs based on predicted branch coverage.  
**Alternatives:** Random generation (lower coverage), symbolic execution (too heavy).  
**Consequences:** Achieves state‑of‑the‑art coverage with minimal complexity.

### ADR‑048: Use tree‑sitter comments for invariant annotations

**Context:** Need a way to specify loop invariants and function contracts without external tools.  
**Decision:** Parse specially‑formatted comments (`//@ invariant`, etc.) using tree‑sitter.  
**Alternatives:** External annotation files (cumbersome), DSL (too heavy).  
**Consequences:** Simple, discoverable, version‑controlled alongside code.

## 6. API & Interface Contracts

CLI extended with new subcommands. MCP server extended with HTTP transport. No changes to WASM plugin interface.

## 7. Cross‑cutting Concerns

- **Error Handling**: Evolutionary failures fall back to single‑shot patching. HTTP MCP errors return appropriate HTTP status codes.  
- **Testing**: Integration tests for all new subcommands; benchmark for evolutionary repair and test generation.  
- **Performance**: Evolutionary repair uses the parallel gate pipeline; test generation uses cached knowledge graph.

## 8. Alternatives Considered

| Alternative | Why Rejected |
|-------------|--------------|
| Use genetic algorithm library | Overhead; custom implementation simpler and tuned to patching domain. |
| Use `actix‑web` for HTTP MCP | Heavier binary; `hyper` is sufficient. |
| Use symbolic execution for test generation | Too complex and slow; Bayesian optimization is pragmatic. |
| Use external annotation files for invariants | Cumbersome; inline comments are versioned with code. |

## 9. Traceability

| ASR | ADR | Component |
|-----|-----|-----------|
| ASR‑001 | ADR‑045 | evolve.rs |
| ASR‑002 | ADR‑046 | mcp_http.rs |
| ASR‑003 | – | review.rs |
| ASR‑004 | – | attest.rs |
| ASR‑005 | ADR‑047 | curiosity.rs |

---

*This architecture specification provides the blueprint for implementing v1.9.0.*
```

---

## 5. Behavioral Specification & Test Verification Plan

```markdown
# Behavioral Specification & Test Verification Plan — patch‑ts v1.9.0

| Field | Value |
|-------|-------|
| Project | patch‑ts |
| Document | Behavioral Specification & Test Verification Plan |
| Version | 1.0 |
| Date | 2026‑04‑24 |
| Author | patch‑ts team, assisted by spec‑writer |
| Status | Draft |
| References | SRS v1.9.0, Architecture v1.9.0 |

## 1. Behavioral Specifications (Specification by Example)

### Feature: Evolutionary Repair

```gherkin
Feature: Evolutionary repair

  Scenario: Multiple candidates generated and best selected
    Given a file "main.rs" with a known bug
    When I run `patch-ts evolve --file main.rs --old "fn foo()" --new "fn foo() { }"`
    Then at least 10 candidate patches are generated
    And the recommended patch has the highest fitness score
    And the JSON output includes `alternatives` with ranked candidates

  Scenario: Learning improves future populations
    Given 50 patches have been applied with recorded strategy success rates
    When I run `patch-ts evolve` on a new patch
    Then the initial population biases toward strategies with higher historical success
```

### Feature: Streamable HTTP MCP

```gherkin
Feature: HTTP MCP transport

  Scenario: Tools list via HTTP
    Given the HTTP MCP server is running on port 9090
    When I POST a `tools/list` request to `http://127.0.0.1:9090/mcp`
    Then the response includes `tools` array with schemas

  Scenario: Authentication required
    Given the server is started with `--auth-token secret`
    When I send a request without the Authorization header
    Then the response status is 401
```

### Feature: Agentic Code Review

```gherkin
Feature: Agentic code review

  Scenario: Multi-agent findings aggregated
    Given a patch that introduces a SQL injection
    When I run `patch-ts review --file main.rs --old "safe" --new "unsafe"`
    Then the security agent reports a critical finding
    And the syntax agent reports a pass
    And the aggregated result shows "request_changes"
```

### Feature: CRA Compliance Evidence

```gherkin
Feature: Compliance attestation

  Scenario: Generate attestation report
    Given provenance records for 10 patches since January
    When I run `patch-ts attest --since "2026-01-01" --output report.jsonld`
    Then the report includes SBOM, provenance chain, and validation results
    And the report passes validation against the CRA schema
```

### Feature: Test Generation

```gherkin
Feature: Curiosity‑driven test generation

  Scenario: Generate tests for patched function
    Given a function `foo` with 0% branch coverage
    When I run `patch-ts generate-tests --file lib.rs --old "fn foo()" --new "fn foo() { x += 1; }"`
    Then at least one test case is generated for `foo`
    And the generated test passes when run
    And branch coverage for `foo` is >= 50%
```

### Feature: Invariant Verification

```gherkin
Feature: Invariant verification

  Scenario: Detect broken loop invariant
    Given a file with a loop annotated `//@ invariant x > 0`
    And a patch that changes `x` to `-1`
    When I run `patch-ts verify --file main.rs`
    Then a violation is reported at the line where `x` becomes negative
```

## 2. Test Strategy & Plan

### 2.1 Test Pyramid

| Level | Scope | Tools |
|-------|-------|-------|
| Unit | Evolution, compliance, test gen, invariant extraction, audit skill | Rust `#[test]` |
| Integration | Full CLI commands for `evolve`, `attest`, `generate‑tests`, `verify`, HTTP MCP | `assert_cmd`, `reqwest`, `tempfile` |
| Property | Evolutionary fitness scoring, test generation coverage | `proptest` |
| Manual | HTTP MCP with real clients, agentic review dashboard | Charters |

### 2.2 Risk‑Based Prioritization

| Risk | Test Focus |
|------|------------|
| Evolution selects low‑quality patches | Benchmark with historical bugs. |
| HTTP MCP security (unauthorized access) | Auth tests with valid/invalid tokens. |
| Generated tests are flaky | Run tests multiple times; reject if non‑deterministic. |

## 3. Test Case Specifications (Excerpt)

| TC‑ID | Requirement | Steps | Expected |
|-------|-------------|-------|----------|
| TC‑EVO‑001 | FR‑EVO‑001 | Run `evolve` on a known buggy file | ≥10 candidates returned |
| TC‑HTTP‑001 | FR‑MCP‑HTTP‑001 | Start HTTP MCP, send `tools/list` | JSON response with tools |
| TC‑REVIEW‑001 | FR‑REVIEW‑001 | Run `review` on a security‑critical patch | Security finding reported |
| TC‑ATTEST‑001 | FR‑ATTEST‑001 | Run `attest` with prior provenance | Valid JSON‑LD report |
| TC‑CURI‑001 | FR‑CURI‑001 | Run `generate‑tests` on a simple function | Tests generated and pass |
| TC‑INV‑001 | FR‑INV‑002 | Apply a patch that breaks an invariant, run `verify` | Invariant violation reported |

## 4. NFR Verification

| NFR | Verification Method |
|-----|---------------------|
| NFR‑PERF‑001 | Benchmark `evolve` on 500‑line file with population=10. |
| NFR‑PERF‑002 | Load test HTTP MCP with 50 concurrent connections. |
| NFR‑SEC‑001 | Security audit of HTTP MCP transport and auth. |
| NFR‑COMPAT‑001 | Run v1.8.0 test suite unchanged. |
| NFR‑DOC‑001 | Review compliance guide and evolutionary repair documentation. |

## 5. Requirements Traceability Matrix (RTM)

*(Table mapping Vision/BRS objectives → SRS requirements → Test cases.)*

---

*This verification plan ensures complete coverage of all v1.9.0 self‑improving governance features.*
