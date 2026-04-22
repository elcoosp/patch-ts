The search results provide useful context: the `glob` crate handles multi‑file patterns, `rayon` is the standard for parallel iterators, tree‑sitter parsers are thread‑safe when each thread owns its own instance, `wasmtime` is the leading sandboxed WASM runtime, crates.io now supports trusted publishing via OIDC, `formulaic` can generate Homebrew formulas for Rust projects, and `sprinkles-rs` provides a Rust API for Scoop manifest generation. These confirm the technical feasibility of the v1.0.0 features.

The user has correctly identified that LSP is out of scope for v1.0.0. Removing it simplifies the release and allows us to focus on core stability and distribution.

---

# patch‑ts v1.0.0 — Revised Specification Documents

*LSP server implementation removed from scope. Focus shifted to multi‑file patches, parallel processing, WASM plugin system, and ecosystem distribution.*

## Document Set Overview

| Document | Purpose |
|----------|---------|
| [Vision & Strategic Alignment](#patch-ts-vision--strategic-alignment) | Long‑term direction and success criteria for the stable release. |
| [Business & Stakeholder Requirements (BRS)](#patch-ts-business--stakeholder-requirements-specification-brs) | Business goals, stakeholders, and high‑level needs for v1.0.0. |
| [Software Requirements Specification (SRS)](#patch-ts-software-requirements-specification-srs) | Functional and non‑functional requirements for v1.0.0. |
| [Architecture & Design Specification](#patch-ts-architecture--design-specification) | Architectural decisions, parallel processing, plugin system, and ADRs. |
| [Behavioral Specification & Test Verification Plan](#patch-ts-behavioral-specification--test-verification-plan) | Acceptance criteria, test strategy, and traceability for v1.0.0 features. |

---

# patch‑ts Vision & Strategic Alignment

| Field | Value |
|-------|-------|
| Project | patch‑ts |
| Document | Vision & Strategic Alignment |
| Version | 8.0 (v1.0.0) |
| Date | 2026‑04‑22 |
| Author | patch‑ts team, assisted by spec‑writer |
| Status | Draft |

## 1. Vision Statement

> *patch‑ts is the definitive, production‑ready universal patching tool—trusted by AI agents and developers worldwide to safely and intelligently modify code across all major languages, with unparalleled performance, extensibility, and seamless distribution.*

## 2. Elevator Pitch (Moore Template)

> For **AI coding agents and developers working across 15+ programming languages** who need a reliable, high‑performance patching tool that scales across entire codebases, **patch‑ts** is a **tree‑sitter‑backed CLI** that provides **fuzzy matching, AST‑based repair, multi‑file patches, parallel processing, and a WASM plugin system**. Unlike fragmented toolchains, our product **delivers enterprise‑grade stability, is easily extensible via sandboxed plugins, and is available through standard package managers (crates.io, Homebrew, Scoop)**, making it the ultimate patching companion for any codebase.

## 3. Problem Statement & Business Context

**Problem:** v0.9.0 supports fifteen languages and includes advanced features (config, watch, TUI), but lacks the polish, performance, and ecosystem integration required for widespread production adoption. Multi‑file operations are serial, limiting throughput on large codebases. There is no way to extend the tool with custom repair logic. Distribution is limited to source builds.

**Why now:**
- AI‑assisted development is mainstream; a stable, universal patching tool is essential infrastructure.
- Performance and extensibility are key differentiators for professional users.
- crates.io and package managers dramatically lower the barrier to entry.

**Business drivers:**
- Establish patch‑ts as the industry‑standard patching utility.
- Drive adoption through ease of installation and high‑performance multi‑file operations.
- Build a community around a plugin ecosystem.

## 4. Target Users / Customers

| Segment | Description |
|---------|-------------|
| **AI coding agents** | Require a stable, performant, and scriptable patching backend. |
| **Professional developers** | Use patch‑ts daily for refactoring and applying AI‑generated patches. |
| **DevOps / Platform engineers** | Need multi‑file patching and parallel processing for large‑scale refactoring. |
| **Plugin authors** | Want to extend patch‑ts with custom language support or repair strategies. |
| **Open‑source contributors** | Value a well‑documented, extensible codebase. |

**Explicitly NOT targeting (v1.0.0):**
- Language Server Protocol (LSP) integration (deferred to v1.1.0).
- Additional languages beyond the fifteen already supported.
- Semantic repairs beyond delimiters and basic syntax (plugin territory).

## 5. User Needs & Value Proposition

| Need | patch‑ts v1.0.0 Value |
|------|------------------------|
| "I need to patch multiple files at once." | Multi‑file patch support (`--files` glob patterns). |
| "I need faster processing on large codebases." | Parallel processing with `rayon` for multi‑file operations. |
| "I want to add custom repair logic for my company's codebase." | WASM plugin system allows custom repair strategies. |
| "I want to install patch‑ts easily." | crates.io, Homebrew, Scoop packages. |
| "I need comprehensive documentation." | Full guides for AI agents, plugin authors, and users. |

**Differentiator:** patch‑ts is the only universal patching tool with a plugin system, parallel processing, and multi‑file support, distributed through standard package ecosystems.

## 6. Desired Outcomes & Success Metrics

### Business Outcomes (v1.0.0)

| ID | Outcome | Key Result / Target |
|----|---------|---------------------|
| G‑1 | Production readiness | Zero critical bugs reported within 3 months of release. |
| G‑2 | Ecosystem adoption | 1,000+ downloads on crates.io within 6 months. |
| G‑3 | Community growth | 5+ external plugins published within 6 months. |
| G‑4 | Easy installation | `cargo install patch‑ts`, `brew install patch‑ts`, `scoop install patch‑ts` work without manual intervention. |

### Product Outcomes (v1.0.0)

| ID | Outcome | Metric |
|----|---------|--------|
| P‑1 | Multi‑file patches work correctly | 100% of multi‑file test cases pass. |
| P‑2 | Parallel processing improves performance | ≥50% speedup on 4+ cores for multi‑file balance. |
| P‑3 | Plugin system is functional | Sample WASM plugin can be loaded and executed. |
| P‑4 | Easy installation | All three installation methods verified in CI. |

## 7. Strategic Constraints

| Constraint | Description |
|------------|-------------|
| **Backward compatibility** | v1.0.0 CLI must accept all v0.9.0 flags and produce equivalent behavior. |
| **Stability** | No breaking changes to the public API after v1.0.0 (semver). |
| **Performance** | Parallel mode must not degrade single‑threaded performance. |
| **Security** | WASM plugins must run in a sandboxed environment with limited capabilities. |

## 8. Goals and Non‑Goals (v1.0.0)

### Goals

- Multi‑file patch application with glob support.
- Parallel processing for multi‑file operations (`rayon`).
- WASM‑based plugin system for custom repair strategies.
- Publish to crates.io.
- Create Homebrew and Scoop formulas.
- Comprehensive documentation (user guide, AI agent guide, plugin guide).

### Non‑Goals (explicitly excluded from v1.0.0)

- Language Server Protocol (LSP) integration (deferred to v1.1.0).
- New languages (deferred to v1.1.0+).
- Semantic repairs in core (delegated to plugins).
- Cloud‑based patch service.

## 9. Operational Concept & High‑Level Scenarios

### Concept of Operations

`patch‑ts` is now a complete ecosystem: a high‑performance CLI for batch operations and a plugin system for extensibility. Users install via their preferred package manager. Multi‑file patches are applied with parallel processing for speed. Custom repair logic can be loaded via WASM plugins.

### High‑Level Scenarios (v1.0.0)

1. **Multi‑file patch**  
   `patch‑ts patch --files "src/**/*.rs" --line 10 --old "foo" --new "bar"`  
   → Applies the patch to all matching Rust files in parallel.

2. **Parallel balance on a large codebase**  
   `patch‑ts balance --files "**/*.ts" --apply`  
   → Processes all TypeScript files concurrently, leveraging all CPU cores.

3. **WASM plugin**  
   User places `my_plugin.wasm` in `~/.config/patch‑ts/plugins/`; plugin provides custom repair for company‑specific DSL.

4. **Install via Homebrew**  
   `brew install patch‑ts` → binary available in PATH.

## 10. Stakeholders, Sponsorship & Governance

| Role | Name / Org | Responsibility |
|------|------------|----------------|
| **Executive Sponsor** | (Project maintainer) | Approves strategic direction. |
| **Product Owner** | (Project maintainer) | Prioritizes features, manages roadmap. |
| **Engineering Lead** | (Core contributor) | Oversees technical implementation. |

## 11. Risks, Assumptions & Open Questions

### Assumptions

- `rayon` can be used safely with thread‑local `Language` instances (tree‑sitter parsers are thread‑safe when each thread owns its own instance).
- `wasmtime` provides adequate sandboxing for plugins.
- `glob` crate handles cross‑platform pattern expansion.

### Risks

| Risk | Likelihood | Impact | Mitigation |
|------|------------|--------|------------|
| WASM plugin complexity | Medium | High | Start with a minimal plugin API; iterate. |
| Parallel processing introduces data races | Low | High | Use thread‑local parsers; test with ThreadSanitizer. |
| Homebrew/Scoop formula maintenance burden | Low | Medium | Use automated tools (`formulaic`, `sprinkles‑rs`). |

### Open Questions

- Should plugins have access to the file system? (No, sandboxed.)

---

# patch‑ts Business & Stakeholder Requirements Specification (BRS)

| Field | Value |
|-------|-------|
| Project | patch‑ts |
| Document | Business & Stakeholder Requirements Specification |
| Version | 8.0 (v1.0.0) |
| Date | 2026‑04‑22 |
| Author | patch‑ts team, assisted by spec‑writer |
| Status | Draft |
| References | Vision v8.0 |

## 1. Business Context

### 1.1 Purpose

This BRS defines business‑level requirements for patch‑ts v1.0.0, focusing on production readiness, ecosystem integration, and advanced features (multi‑file, parallel, plugins).

### 1.2 Business Problem / Opportunity

v0.9.0 is feature‑rich but lacks the polish, distribution, and performance expected of a professional tool. AI agents and developers need a stable, universally accessible patching utility that scales across large codebases.

### 1.3 Scope Boundaries

**In scope:**
- Multi‑file patch application.
- Parallel processing.
- WASM plugin system.
- crates.io, Homebrew, Scoop distribution.
- Comprehensive documentation.

**Out of scope:**
- LSP integration (deferred).
- New languages.
- Semantic repairs in core.

## 2. Business Goals, Objectives & Success Metrics

| ID | Business Goal | Success Metric (Fit Criterion) |
|----|---------------|-------------------------------|
| BR‑001 | Production readiness | Zero critical bugs in first 3 months. |
| BR‑002 | Multi‑file support | Multi‑file test suite passes 100%. |
| BR‑003 | Parallel performance | ≥50% speedup on 4+ cores. |
| BR‑004 | Plugin system availability | At least one third‑party plugin published within 3 months. |
| BR‑005 | Distribution | crates.io, Homebrew, Scoop packages available. |

## 3. Business Model & Processes

patch‑ts remains open‑source. Distribution through standard channels drives adoption. A plugin ecosystem encourages community contributions.

## 4. Business Rules & Policies

| ID | Rule | Source |
|----|------|--------|
| BR‑R1 | Semantic versioning applies from v1.0.0. | Industry standard. |
| BR‑R2 | Plugins must not access the network or file system (sandboxed). | Security. |

## 5. Stakeholders & User Classes

| Stakeholder / User Class | Description | Primary Goals |
|--------------------------|-------------|---------------|
| **AI Agent** | LLM‑based coding assistant. | Reliable, performant multi‑file patching. |
| **Developer** | Daily user. | Multi‑file operations, easy install. |
| **Plugin Author** | Extends patch‑ts. | Simple, powerful plugin API. |
| **DevOps Engineer** | CI/CD pipelines. | Multi‑file, parallel operations. |

## 6. Glossary / Ubiquitous Language

| Term | Definition |
|------|------------|
| **WASM** | WebAssembly; used for sandboxed plugins. |
| **Multi‑file patch** | Applying the same change to multiple files. |
| **Glob** | Pattern matching for file paths (e.g., `**/*.rs`). |

## 7. Conceptual Domain Model

**Core entities (extended):**
- `Plugin` trait for WASM extensions.
- `MultiFilePatch` orchestrates parallel application.

## 8. Stakeholder Needs & User Requirements

| ID | Stakeholder Need | User Class |
|----|------------------|------------|
| SN‑001 | As a developer, I want to patch multiple files at once. | Developer |
| SN‑002 | As a DevOps engineer, I want faster multi‑file processing. | DevOps |
| SN‑003 | As a plugin author, I want to add custom repair logic. | Plugin Author |
| SN‑004 | As a new user, I want to install patch‑ts easily. | Developer |

## 9. System‑in‑Context & Operational Concept

`patch‑ts` CLI supports glob patterns for multi‑file operations, processed in parallel. Plugins extend repair capabilities.

## 10. Stakeholder‑Level Constraints & Quality Expectations

| ID | Constraint / Quality Expectation |
|----|----------------------------------|
| C‑001 | CLI must remain responsive during parallel operations. |
| C‑002 | Plugins must not crash the main process. |

## 11. Risks, Assumptions & Open Issues

### Assumptions
- `rayon` and `wasmtime` are stable.

### Risks
| Risk | Mitigation |
|------|------------|
| WASM plugin API is too limiting | Gather feedback from early adopters. |

### Open Issues
- Which WASM runtime? (`wasmtime` recommended.)

## 12. Traceability Mapping to Vision

| Vision Goal | BRS Goal | Stakeholder Need |
|-------------|----------|------------------|
| G‑1 | BR‑001 | All |
| G‑2 | BR‑005 | SN‑004 |
| G‑3 | BR‑004 | SN‑003 |
| G‑4 | BR‑005 | SN‑004 |
| P‑1 | BR‑002 | SN‑001 |
| P‑2 | BR‑003 | SN‑002 |
| P‑3 | BR‑004 | SN‑003 |
| P‑4 | BR‑005 | SN‑004 |

---

# patch‑ts Software Requirements Specification (SRS)

| Field | Value |
|-------|-------|
| Project | patch‑ts |
| Document | Software Requirements Specification |
| Version | 8.0 (v1.0.0) |
| Date | 2026‑04‑22 |
| Author | patch‑ts team, assisted by spec‑writer |
| Status | Draft |
| References | BRS v8.0, Vision v8.0 |

## 1. Introduction & Scope

This SRS defines functional and non‑functional requirements for patch‑ts v1.0.0.

### 1.1 Scope

- Multi‑file patch application.
- Parallel processing with `rayon`.
- WASM plugin system.
- Distribution packages (crates.io, Homebrew, Scoop).
- Comprehensive documentation.

### 1.2 Out of Scope

- LSP server.
- New languages.
- Semantic repairs in core.

## 2. System Context & Overview

**Context Diagram (C1):** Extended to include plugin system.

## 3. Functional Capabilities & Behavior

### Feature: Multi‑File Patches

| ID | Requirement | Priority | Acceptance Criteria |
|----|-------------|----------|---------------------|
| FR‑MULTI‑001 | `--files` flag accepts glob patterns. | Must | Glob expands correctly. |
| FR‑MULTI‑002 | Patch applied to all matching files. | Must | All files modified. |
| FR‑MULTI‑003 | Dry‑run shows diff for each file. | Must | Output lists each file and change. |

### Feature: Parallel Processing

| ID | Requirement | Priority | Acceptance Criteria |
|----|-------------|----------|---------------------|
| FR‑PAR‑001 | Multi‑file operations use `rayon` parallel iterators. | Must | Files processed concurrently. |
| FR‑PAR‑002 | Each thread creates its own `Language` instance. | Must | No data races. |
| FR‑PAR‑003 | `--serial` flag disables parallelism. | Should | Serial execution available. |

### Feature: WASM Plugin System

| ID | Requirement | Priority | Acceptance Criteria |
|----|-------------|----------|---------------------|
| FR‑PLUG‑001 | Plugins loaded from `~/.config/patch‑ts/plugins/`. | Must | Plugins discovered and loaded. |
| FR‑PLUG‑002 | Plugins implement `RepairPlugin` trait (WASM interface). | Must | Plugin can provide custom repairs. |
| FR‑PLUG‑003 | Plugins run in sandboxed WASM runtime (`wasmtime`). | Must | No file/network access. |
| FR‑PLUG‑004 | `--plugin` flag enables specific plugin for session. | Should | Plugin activated. |

### Feature: Distribution

| ID | Requirement | Priority | Acceptance Criteria |
|----|-------------|----------|---------------------|
| FR‑DIST‑001 | Published on crates.io. | Must | `cargo install patch‑ts` works. |
| FR‑DIST‑002 | Homebrew formula available. | Must | `brew install patch‑ts` works. |
| FR‑DIST‑003 | Scoop manifest available. | Should | `scoop install patch‑ts` works. |

## 4. Quality & Non‑Functional Requirements

| ID | Category | Requirement | Fit Criterion |
|----|----------|-------------|---------------|
| NFR‑PERF‑001 | Performance | Multi‑file balance on 10 files (10k lines each) ≤ 5s on 4 cores. | Benchmarked. |
| NFR‑SEC‑001 | Security | WASM plugins cannot access host system. | Sandbox test. |
| NFR‑DOC‑001 | Documentation | User guide covers all CLI flags; plugin guide includes example. | Review. |
| NFR‑COMPAT‑001 | Compatibility | All v0.9.0 tests pass without modification. | CI regression. |

## 5. External Interfaces & Data Contracts

**CLI Interface:** New flags: `--files`, `--serial`, `--plugin`.

**Plugin Interface (WIT):**
```wit
package patch‑ts:plugin;

interface repair {
    record Span {
        start‑byte: u32,
        end‑byte: u32,
    }

    variant DelimiterError {
        extra(char, Span),
        missing(char, Span),
    }

    repair: func(errors: list<DelimiterError>, source: string) -> string;
}
```

## 6. Constraints, Assumptions & Dependencies

| Type | Description |
|------|-------------|
| Constraint | Add `rayon`, `wasmtime`, `glob`, `wit‑bindgen`. |
| Assumption | WASM plugins are trusted by the user (sandbox limits risk). |
| Dependency | `tree‑sitter` 0.26. |

## 7. TBD Log

| ID | Item | Owner | Due |
|----|------|-------|-----|
| TBD‑001 | Finalize plugin WIT interface. | Engineering | Before implementation. |

## 8. Requirements Attributes & Traceability Model

**ID Scheme:**  
- Functional: `FR‑MULTI‑###`, `FR‑PAR‑###`, `FR‑PLUG‑###`, `FR‑DIST‑###`  
- NFR: `NFR‑CAT‑###`

**Traceability Matrix (excerpt):**

| SRS ID | BRS ID | Verification Method |
|--------|--------|---------------------|
| FR‑MULTI‑001 | BR‑002 | Test |
| FR‑PAR‑001 | BR‑003 | Test + Bench |
| FR‑PLUG‑001 | BR‑004 | Test |
| FR‑DIST‑001 | BR‑005 | Manual |

---

# patch‑ts Architecture & Design Specification

| Field | Value |
|-------|-------|
| Project | patch‑ts |
| Document | Architecture & Design Specification |
| Version | 8.0 (v1.0.0) |
| Date | 2026‑04‑22 |
| Author | patch‑ts team, assisted by spec‑writer |
| Status | Draft |
| References | SRS v8.0, BRS v8.0 |

## 1. Context & Scope

This document describes architectural changes for v1.0.0: multi‑file support, parallel processing, and plugin system.

## 2. Goals & Non‑Goals

### Goals

- Implement `--files` glob expansion for multi‑file operations.
- Integrate `rayon` for parallel processing.
- Build WASM plugin host using `wasmtime`.
- Create distribution packages.

### Non‑Goals

- LSP server (deferred).
- New languages.
- Semantic repairs in core.

## 3. Architecturally Significant Requirements (ASRs)

| ASR ID | Description | Source |
|--------|-------------|--------|
| ASR‑001 | Multi‑file operations must support glob patterns. | FR‑MULTI‑001 |
| ASR‑002 | Parallel processing must be thread‑safe. | FR‑PAR‑002 |
| ASR‑003 | Plugins must run sandboxed. | FR‑PLUG‑003 |

## 4. The Design

### 4.1 System Overview (C4 Level 2)

```
[User/Agent] → (CLI) → [patch‑ts Binary]
                           ├── cli.rs (glob expansion, parallel dispatch)
                           ├── plugin.rs (WASM host, wasmtime)
                           ├── ast.rs (Language trait + 15 implementors)
                           ├── repair.rs (AST‑based repair)
                           └── file.rs (atomic write)
```

### 4.2 Key Design Changes

**4.2.1 Multi‑File Support**

Use `glob` crate to expand patterns:

```rust
use glob::glob;

let paths: Vec<PathBuf> = glob("src/**/*.rs")?
    .filter_map(Result::ok)
    .collect();
```

**4.2.2 Parallel Processing**

Use `rayon` parallel iterators with thread‑local `Language` instances:

```rust
use rayon::prelude::*;

paths.par_iter().for_each(|path| {
    let mut lang = detect_language(path)?;
    balance_file(path, None, dry_run, &mut lang)
});
```

Each thread creates its own `Language` instance, ensuring thread safety (tree‑sitter parsers are not `Sync` but can be created per thread).

**4.2.3 WASM Plugin System**

Use `wasmtime` as the runtime and `wit‑bindgen` for interface generation. Plugins are loaded from `~/.config/patch‑ts/plugins/` and must implement the `RepairPlugin` interface.

```rust
// Host side
let engine = Engine::default();
let module = Module::from_file(&engine, plugin_path)?;
let mut store = Store::new(&engine, ());
let instance = Instance::new(&mut store, &module, &[])?;
let repair_fn = instance.get_typed_func::<(String,), (String,)>(&mut store, "repair")?;
```

**4.2.4 Distribution**

- crates.io: `cargo publish` with trusted publishing (OIDC).
- Homebrew: Use `formulaic` to generate formula from GitHub releases.
- Scoop: Use `sprinkles‑rs` or manual manifest with SHA256 checksums.

### 4.3 Data Model

New structs: `PluginHost`, `GlobPattern`.

### 4.4 Security Architecture

WASM plugins run in a sandbox with no file system or network access (unless explicitly granted via WASI, which is not enabled by default).

## 5. Architecture Decision Records (ADRs)

### ADR‑018: Use `rayon` for parallel multi‑file operations

**Context:** Multi‑file operations are CPU‑bound and independent.

**Decision:** Use `rayon` parallel iterators with thread‑local `Language` instances.

**Alternatives:** Manual threading. Rejected for complexity.

**Consequences:** Simple, safe parallelism; each thread creates its own parser.

### ADR‑019: Use `wasmtime` for plugin sandbox

**Context:** Need a secure, portable plugin runtime.

**Decision:** Use `wasmtime` with WIT interface definitions.

**Alternatives:** `wasmer`, `extism`. `wasmtime` is the most mature and actively maintained.

**Consequences:** Sandboxed execution; plugins are portable across platforms.

### ADR‑020: Use `formulaic` for Homebrew formula generation

**Context:** Maintaining Homebrew formulas manually is error‑prone.

**Decision:** Use `formulaic` to generate formulas from GitHub releases.

**Alternatives:** Manual formula. Rejected for maintenance burden.

**Consequences:** Automated, consistent formula updates.

## 6. API & Interface Contracts

CLI extended with `--files`, `--serial`, `--plugin`. Plugin interface defined in WIT.

## 7. Cross‑cutting Concerns

| Concern | Approach |
|---------|----------|
| **Testing** | Multi‑file integration tests; plugin sandbox tests. |
| **Performance** | Benchmark parallel vs serial. |
| **Security** | Sandbox tests for WASM plugins. |

## 8. Alternatives Considered

| Alternative | Why Rejected |
|-------------|--------------|
| Manual threading | `rayon` is simpler and safer. |
| Dynamic library plugins (`.so`/`.dylib`) | Not portable; security risks. |
| Manual Homebrew formula | High maintenance burden. |

## 9. Traceability

| ASR | ADR | Component |
|-----|-----|-----------|
| ASR‑001 | – | cli.rs |
| ASR‑002 | ADR‑018 | repair.rs |
| ASR‑003 | ADR‑019 | plugin.rs |

---

# patch‑ts Behavioral Specification & Test Verification Plan

| Field | Value |
|-------|-------|
| Project | patch‑ts |
| Document | Behavioral Specification & Test Verification Plan |
| Version | 8.0 (v1.0.0) |
| Date | 2026‑04‑22 |
| Author | patch‑ts team, assisted by spec‑writer |
| Status | Draft |
| References | SRS v8.0, Architecture v8.0 |

## 1. Behavioral Specifications (Specification by Example)

### Feature: Multi‑File Patches

```gherkin
Feature: Multi‑file patch application
  Scenario: Apply patch to all Rust files in directory
    Given a directory "src" with files:
      | a.rs | fn a() {} |
      | b.rs | fn b() {} |
    When I run `patch‑ts patch --files "src/**/*.rs" --line 1 --old "fn a() {}" --new "fn a_new() {}"`
    Then the file "src/a.rs" contains "fn a_new() {}"
    And the file "src/b.rs" is unchanged
```

### Feature: Parallel Processing

```gherkin
Feature: Parallel multi‑file processing
  Scenario: Balance multiple files in parallel
    Given 10 Rust files with delimiter errors
    When I run `patch‑ts balance --files "*.rs" --apply`
    Then all files are processed correctly
    And the operation completes faster than serial execution
```

### Feature: WASM Plugin

```gherkin
Feature: WASM plugin system
  Scenario: Load and execute a custom repair plugin
    Given a WASM plugin at "~/.config/patch‑ts/plugins/custom.wasm"
    And a file with a custom delimiter error
    When I run `patch‑ts balance --file test.rs --plugin custom`
    Then the plugin's repair logic is applied
```

## 2. Test Strategy & Plan

### 2.1 Test Pyramid

| Level | Scope | Tools |
|-------|-------|-------|
| Unit | Glob expansion, plugin loading | Rust `#[test]` |
| Integration | Multi‑file commands, parallel processing | `assert_cmd` |
| Regression | v0.9.0 test suite | Cargo test |
| Performance | Parallel vs serial benchmark | Criterion |
| Security | WASM sandbox escape tests | Custom harness |

### 2.2 Risk‑Based Prioritization

| Risk | Test Focus |
|------|------------|
| Parallel data races | Test with ThreadSanitizer. |
| WASM plugin sandbox escape | Attempt file system access from plugin. |
| Glob pattern edge cases | Test with nested directories, symlinks. |

## 3. Test Case Specifications

### TC‑MULTI‑001: Glob expansion

- **Requirement:** FR‑MULTI‑001
- **Preconditions:** Directory with multiple `.rs` files.
- **Steps:** Run `patch‑ts patch --files "**/*.rs" ...`.
- **Expected:** All matching files processed.
- **Automated:** Yes.

### TC‑PAR‑001: Parallel performance

- **Requirement:** FR‑PAR‑001
- **Preconditions:** 10 files, 10k lines each.
- **Steps:** Run `balance --files` with and without `--serial`.
- **Expected:** Parallel ≥50% faster on 4+ cores.
- **Automated:** Yes (benchmark).

### TC‑PLUG‑001: Plugin loading

- **Requirement:** FR‑PLUG‑001
- **Preconditions:** Valid WASM plugin in plugins directory.
- **Steps:** Run `patch‑ts balance --plugin test`.
- **Expected:** Plugin loaded and executed.
- **Automated:** Yes.

### TC‑DIST‑001: crates.io installation

- **Requirement:** FR‑DIST‑001
- **Preconditions:** Clean Rust environment.
- **Steps:** `cargo install patch‑ts`.
- **Expected:** Binary installed and functional.
- **Automated:** Manual (CI verification).

## 4. NFR Verification Plans

### NFR‑PERF‑001 (Performance)

- **Method:** Criterion benchmark on 10 files (10k lines each).
- **Threshold:** ≤ 5s on 4 cores.

### NFR‑SEC‑001 (Security)

- **Method:** Attempt file system access from WASM plugin.
- **Threshold:** Access denied; plugin terminated.

### NFR‑COMPAT‑001 (Compatibility)

- **Method:** Run v0.9.0 test suite.
- **Threshold:** 100% pass.

## 5. Requirements Traceability Matrix (RTM)

| SRS ID | Test Case(s) | Verification Method |
|--------|--------------|---------------------|
| FR‑MULTI‑001 | TC‑MULTI‑001 | Test |
| FR‑PAR‑001 | TC‑PAR‑001 | Test + Bench |
| FR‑PLUG‑001 | TC‑PLUG‑001 | Test |
| FR‑DIST‑001 | TC‑DIST‑001 | Manual |
| NFR‑PERF‑001 | TC‑PAR‑001 | Bench |
| NFR‑SEC‑001 | TC‑PLUG‑001 | Security Test |

---

This completes the revised specification suite for patch‑ts v1.0.0. The scope has been refined to focus on multi‑file patches, parallel processing, the WASM plugin system, and ecosystem distribution—all features that deliver immediate value and solidify patch‑ts as a production‑ready tool. LSP integration is deferred to a future release (v1.1.0).
