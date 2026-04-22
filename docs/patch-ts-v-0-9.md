# patch-ts v0.9.0 — Specification Documents

## Document Set Overview

| Document | Purpose |
|----------|---------|
| [Vision & Strategic Alignment](#patch-ts-vision--strategic-alignment) | Long‑term direction and success criteria for the "awesome" release. |
| [Business & Stakeholder Requirements (BRS)](#patch-ts-business--stakeholder-requirements-specification-brs) | Business goals, stakeholders, and high‑level needs for v0.9.0. |
| [Software Requirements Specification (SRS)](#patch-ts-software-requirements-specification-srs) | Functional and non‑functional requirements for v0.9.0. |
| [Architecture & Design Specification](#patch-ts-architecture--design-specification) | Architectural decisions, TUI integration, parallel processing, and ADRs. |
| [Behavioral Specification & Test Verification Plan](#patch-ts-behavioral-specification--test-verification-plan) | Acceptance criteria, test strategy, and traceability for v0.9.0 features. |

---

# patch-ts Vision & Strategic Alignment

| Field | Value |
|-------|-------|
| Project | patch-ts |
| Document | Vision & Strategic Alignment |
| Version | 7.0 (v0.9.0) |
| Date | 2026-04-22 |
| Author | patch-ts team, assisted by spec-writer |
| Status | Draft |

## 1. Vision Statement

> *patch-ts becomes the definitive, intelligent, and delightful universal patching tool—supporting every major programming language, featuring an interactive TUI, watch mode, configuration, and blazing‑fast parallel processing. It is the ultimate companion for AI agents and developers alike.*

## 2. Elevator Pitch (Moore Template)

> For **AI coding agents and developers working across Rust, TypeScript, JavaScript, Python, Go, Ruby, PHP, HTML, XML, C, C++, Java, C#, Kotlin, Swift, Scala, and Zig** who need a single, reliable, and feature‑rich patching tool, **patch-ts** is a **tree‑sitter‑backed CLI with an optional TUI** that provides **fuzzy matching, AST‑based delimiter repair, batch error resolution, marker‑based targeting, configuration files, watch mode, and parallel processing across sixteen languages**. Unlike fragmented toolchains, our product **auto‑detects language, respects project settings, watches for changes, and leverages all CPU cores**, making it the one patching utility for every codebase and every workflow.

## 3. Problem Statement & Business Context

**Problem:** v0.8.0 supports twelve languages, but Kotlin, Swift, Scala, and Zig developers are still excluded. AI agents and developers also lack advanced workflow features: project‑specific configuration, automatic patch application on file changes (watch mode), and interactive patch review (TUI). Performance on large codebases could be improved with parallelism.

**Why now:**
- Kotlin is the #1 language for Android and a top‑tier backend language; Swift dominates Apple platforms; Scala remains strong in data engineering; Zig is the fastest‑growing systems language. Supporting these four languages completes the "top languages" coverage and includes the hottest emerging language (Zig).
- Advanced features like config files, watch mode, TUI, and parallel processing elevate patch‑ts from a utility to a professional‑grade tool that fits seamlessly into any development workflow.
- The Rust ecosystem provides mature crates for all these features: `ratatui` for TUI, `notify` for watch mode, `serde`/`toml` for configuration, and `rayon` for parallelism.

**Business drivers:**
- Expand user base to Android (Kotlin), Apple (Swift), data engineering (Scala), and cutting‑edge systems (Zig) developers.
- Increase tool adoption by providing features developers expect from modern CLI tools (config, watch, parallel, interactive).
- Solidify patch‑ts as the definitive universal patching utility with no compromises.

## 4. Target Users / Customers

| Segment | Description |
|---------|-------------|
| **Kotlin developers** | Android and backend (Ktor, Spring) developers. |
| **Swift developers** | iOS, macOS, and server‑side Swift developers. |
| **Scala developers** | Data engineering (Spark), backend (Play, Akka) developers. |
| **Zig developers** | Systems programmers seeking a modern C alternative. |
| **AI coding agents** | Require consistent patching across all major languages and programmatic configuration. |
| **CI/CD pipelines** | Benefit from watch mode and parallel processing in automated workflows. |
| **Power users** | Value TUI for interactive patch review and configuration for project‑specific defaults. |

**Explicitly NOT targeting (v0.9.0):**
- Additional languages beyond the sixteen already covered (deferred to v1.0.0+).
- Semantic repairs beyond delimiters.
- Multi‑file operations (already planned for v1.0.0).

## 5. User Needs & Value Proposition

| Need | patch-ts v0.9.0 Value |
|------|------------------------|
| "I need to patch Kotlin files." | Kotlin support via `tree-sitter-kotlin`. |
| "I need to patch Swift files." | Swift support via `tree-sitter-swift`. |
| "I need to patch Scala files." | Scala support via `tree-sitter-scala`. |
| "I need to patch Zig files." | Zig support via `tree-sitter-zig`. |
| "I want project‑specific defaults (fuzz radius, backup location)." | Configuration file (`patch-ts.toml`) read automatically. |
| "I want patches applied automatically when files change." | Watch mode (`--watch`) monitors files and applies queued patches. |
| "I want to review patches interactively before applying." | TUI mode (`--tui`) using `ratatui` for diff review and approval. |
| "I want faster processing on large files or many files." | Parallel processing with `rayon` for batch operations. |

**Differentiator:** patch-ts is the only CLI patching tool supporting sixteen major languages with AST‑aware repair, batch resolution, configuration, watch mode, TUI, and parallel processing.

## 6. Desired Outcomes & Success Metrics

### Business Outcomes (v0.9.0)

| ID | Outcome | Key Result / Target |
|----|---------|---------------------|
| G‑1 | Expand language coverage | Kotlin, Swift, Scala, and Zig support fully integrated; all commands work. |
| G‑2 | Increase user adoption | 100+ GitHub stars; 10+ external contributors within 3 months. |
| G‑3 | Improve performance | Parallel processing reduces multi‑file operation time by ≥50% on 4+ core machines. |
| G‑4 | Enhance workflow integration | Watch mode and configuration file adopted by ≥30% of users (measured via telemetry opt‑in). |

### Product Outcomes (v0.9.0)

| ID | Outcome | Metric |
|----|---------|--------|
| P‑1 | Users can patch Kotlin/Swift/Scala/Zig files | CLI accepts `.kt`, `.swift`, `.scala`, `.zig` with auto‑detection. |
| P‑2 | Users can configure defaults per project | `patch-ts.toml` read and merged with CLI args. |
| P‑3 | Users can watch files for automatic patching | `--watch` mode applies patches when files change. |
| P‑4 | Users can review patches interactively | `--tui` launches a terminal UI for diff review and approval. |
| P‑5 | Users experience faster batch operations | Parallel processing enabled for `balance` on multiple files. |

## 7. Strategic Constraints

| Constraint | Description |
|------------|-------------|
| **Backward compatibility** | v0.9.0 CLI must accept all v0.8.0 flags and produce equivalent behavior. |
| **Dependency footprint** | Add `tree-sitter-kotlin`, `tree-sitter-swift`, `tree-sitter-scala`, `tree-sitter-zig`, `ratatui`, `notify`, `rayon`, `serde`, `toml`. |
| **Performance** | Balance on 10k‑line files ≤ 200 ms (p95) across all sixteen languages; parallel mode must not degrade single‑threaded performance. |
| **Cross‑platform** | TUI and watch mode must work on Linux, macOS, and Windows. |

## 8. Goals and Non‑Goals (v0.9.0)

### Goals

- Implement `KotlinLanguage`, `SwiftLanguage`, `ScalaLanguage`, `ZigLanguage` via macro.
- Auto‑detect `.kt`, `.kts`, `.swift`, `.scala`, `.zig` extensions.
- Implement configuration file loading (`patch-ts.toml`) with precedence: CLI > config > defaults.
- Implement watch mode (`--watch`) using `notify` crate.
- Implement TUI mode (`--tui`) using `ratatui` for interactive patch review.
- Implement parallel processing for multi‑file operations using `rayon`.
- Add integration tests for new languages and features.

### Non‑Goals (explicitly excluded from v0.9.0)

- Semantic repairs (e.g., missing semicolons, type errors).
- Additional languages beyond the sixteen.
- Multi‑file patch application (planned for v1.0.0).
- Plugin system.

## 9. Operational Concept & High‑Level Scenarios

### Concept of Operations

`patch-ts` now supports sixteen languages. Users can create a `patch-ts.toml` file in their project root to set defaults (fuzz radius, backup location, auto‑repair behavior). The `--watch` flag enables continuous monitoring: patch‑ts watches specified files and applies queued patches automatically. The `--tui` flag launches an interactive terminal UI for reviewing and approving patches before application. Multi‑file operations leverage `rayon` for parallel processing.

### High‑Level Scenarios (v0.9.0)

1. **Patch a Kotlin file with project defaults**  
   `patch-ts patch --file Main.kt --line 10 --old "val x = 1" --new "val x = 2"`  
   → Reads `patch-ts.toml` for default fuzz radius, applies patch.

2. **Watch mode for automatic patching**  
   `patch-ts watch --file src/ --queue patches.json`  
   → Monitors `src/` directory; when files change, applies patches from the queue.

3. **Interactive TUI for patch review**  
   `patch-ts patch --file app.rs --tui`  
   → Opens TUI showing diff; user can accept, reject, or edit the patch.

4. **Parallel balance on multiple files**  
   `patch-ts balance --files src/**/*.rs --apply`  
   → Processes all matching Rust files in parallel, fixing delimiter errors.

5. **Configuration file merging**  
   CLI flag `--fuzz 10` overrides `fuzz = 5` in `patch-ts.toml`.

## 10. Stakeholders, Sponsorship & Governance

| Role | Name / Org | Responsibility |
|------|------------|----------------|
| **Executive Sponsor** | (Project maintainer) | Approves strategic direction. |
| **Product Owner** | (Project maintainer) | Prioritizes features, manages roadmap. |
| **Engineering Lead** | (Core contributor) | Oversees technical implementation. |

## 11. Risks, Assumptions & Open Questions

### Assumptions

- `tree-sitter-kotlin`, `tree-sitter-swift`, `tree-sitter-scala`, `tree-sitter-zig` grammars are stable and expose delimiter nodes.
- `ratatui`, `notify`, and `rayon` are cross‑platform compatible.
- Users understand the precedence rules for configuration.

### Risks

| Risk | Likelihood | Impact | Mitigation |
|------|------------|--------|------------|
| TUI complexity increases maintenance burden | Medium | Medium | Keep TUI minimal; focus on core diff review. |
| Watch mode false positives on some filesystems | Low | Low | Use `notify` with debouncing; provide `--watch-delay` option. |
| Parallel processing introduces subtle bugs | Low | Medium | Thorough testing; make parallel mode opt‑in initially. |

### Open Questions

- Should TUI support editing patches inline? (Deferred; start with review/approve.)
- Should watch mode support glob patterns? (Yes, use `glob` crate.)

---

# patch-ts Business & Stakeholder Requirements Specification (BRS)

| Field | Value |
|-------|-------|
| Project | patch-ts |
| Document | Business & Stakeholder Requirements Specification |
| Version | 7.0 (v0.9.0) |
| Date | 2026-04-22 |
| Author | patch-ts team, assisted by spec-writer |
| Status | Draft |
| References | Vision v7.0 |

## 1. Business Context

### 1.1 Purpose

This BRS defines business‑level requirements for patch-ts v0.9.0, adding Kotlin, Swift, Scala, and Zig support, plus configuration, watch mode, TUI, and parallel processing.

### 1.2 Business Problem / Opportunity

v0.8.0 supports twelve languages but misses key mobile (Kotlin/Swift), data engineering (Scala), and emerging systems (Zig) languages. Developers also expect modern CLI features: project‑specific configuration, file watching, interactive UIs, and parallelism. Adding these features makes patch-ts a complete, professional‑grade tool.

### 1.3 Scope Boundaries

**In scope:**
- Kotlin, Swift, Scala, Zig language support.
- Configuration file (`patch-ts.toml`).
- Watch mode (`--watch`).
- TUI mode (`--tui`).
- Parallel processing with `rayon`.
- Integration tests for all new features.

**Out of scope:**
- Other languages.
- Semantic repairs.
- Multi‑file patch application (v1.0.0).

## 2. Business Goals, Objectives & Success Metrics

| ID | Business Goal | Success Metric (Fit Criterion) |
|----|---------------|-------------------------------|
| BR‑001 | Add Kotlin support | 100% of CLI commands work on `.kt` files in test suite. |
| BR‑002 | Add Swift support | 100% of CLI commands work on `.swift` files in test suite. |
| BR‑003 | Add Scala support | 100% of CLI commands work on `.scala` files in test suite. |
| BR‑004 | Add Zig support | 100% of CLI commands work on `.zig` files in test suite. |
| BR‑005 | Implement configuration file | `patch-ts.toml` correctly overrides defaults and is overridden by CLI flags. |
| BR‑006 | Implement watch mode | File changes trigger patch application within 500ms. |
| BR‑007 | Implement TUI | Users can review and approve/reject patches interactively. |
| BR‑008 | Implement parallel processing | Multi‑file operations complete ≥50% faster on 4+ cores. |

## 3. Business Model & Processes

patch-ts remains an open‑source CLI tool. Advanced features drive adoption and community contributions.

## 4. Business Rules & Policies

| ID | Rule | Source |
|----|------|--------|
| BR‑R1 | Configuration precedence: CLI flags > config file > defaults. | Industry standard. |
| BR‑R2 | Watch mode must not apply patches that introduce syntax errors (unless `--force`). | Quality requirement. |
| BR‑R3 | TUI must display clear diff and require explicit user confirmation. | Usability. |

## 5. Stakeholders & User Classes

| Stakeholder / User Class | Description | Primary Goals |
|--------------------------|-------------|---------------|
| **Kotlin Developer** | Android, backend. | Apply patches safely; use project config. |
| **Swift Developer** | iOS, macOS. | Same as above. |
| **Scala Developer** | Data engineering, backend. | Same as above. |
| **Zig Developer** | Systems programming. | Same as above. |
| **Power User** | Uses advanced CLI features. | TUI, watch mode, config. |
| **AI Agent** | Generates patches across languages. | Consistent, reliable patching; programmatic config. |

## 6. Glossary / Ubiquitous Language

| Term | Definition |
|------|------------|
| **TUI** | Text‑based User Interface; interactive terminal application. |
| **Watch mode** | Continuous monitoring of files for changes, triggering actions. |
| **Configuration file** | `patch-ts.toml` containing project‑specific defaults. |
| **Parallel processing** | Using multiple CPU cores to speed up operations. |

## 7. Conceptual Domain Model

**Core entities (extended):**
- `KotlinLanguage`, `SwiftLanguage`, `ScalaLanguage`, `ZigLanguage` implement `Language`.
- `Config` struct holds settings from `patch-ts.toml`.
- `Watcher` struct manages file monitoring.
- `TuiApp` struct manages interactive UI.

## 8. Stakeholder Needs & User Requirements

| ID | Stakeholder Need | User Class |
|----|------------------|------------|
| SN‑001 | As a Kotlin developer, I want to use all patch-ts commands on `.kt` files. | Kotlin Developer |
| SN‑002 | As a Swift developer, I want the same fuzzy patching and auto‑repair. | Swift Developer |
| SN‑003 | As a Scala developer, I want to patch `.scala` files. | Scala Developer |
| SN‑004 | As a Zig developer, I want to patch `.zig` files. | Zig Developer |
| SN‑005 | As a power user, I want project‑specific defaults in a config file. | Power User |
| SN‑006 | As a power user, I want automatic patching when files change. | Power User |
| SN‑007 | As a power user, I want to review patches interactively. | Power User |
| SN‑008 | As a CI user, I want faster multi‑file operations. | CI/CD System |

## 9. System‑in‑Context & Operational Concept

`patch-ts` now supports `.kt`, `.kts`, `.swift`, `.scala`, `.zig`. Configuration is read from `patch-ts.toml` if present. `--watch` starts a file watcher; when files change, queued patches are applied. `--tui` launches an interactive UI. Multi‑file commands use `rayon` for parallelism.

## 10. Stakeholder‑Level Constraints & Quality Expectations

| ID | Constraint / Quality Expectation |
|----|----------------------------------|
| C‑001 | Configuration file must be optional; tool works without it. |
| C‑002 | Watch mode must be responsive (apply within 500ms of file change). |
| C‑003 | TUI must be intuitive and require minimal keystrokes. |

## 11. Risks, Assumptions & Open Issues

### Assumptions
- Grammars are compatible with tree-sitter 0.26.
- `notify` works reliably across platforms.

### Risks
| Risk | Mitigation |
|------|------------|
| TUI development is time‑consuming | Scope to essential features; iterate. |
| Watch mode may trigger on temporary files | Use ignore patterns; debouncing. |

### Open Issues
- Should TUI support editing? (Deferred.)

## 12. Traceability Mapping to Vision

| Vision Goal | BRS Goal | Stakeholder Need |
|-------------|----------|------------------|
| G‑1 | BR‑001..004 | SN‑001..004 |
| G‑2 | BR‑005..008 | SN‑005..008 |
| P‑1..5 | BR‑001..008 | SN‑001..008 |

---

# patch-ts Software Requirements Specification (SRS)

| Field | Value |
|-------|-------|
| Project | patch-ts |
| Document | Software Requirements Specification |
| Version | 7.0 (v0.9.0) |
| Date | 2026-04-22 |
| Author | patch-ts team, assisted by spec-writer |
| Status | Draft |
| References | BRS v7.0, Vision v7.0 |

## 1. Introduction & Scope

This SRS defines functional and non‑functional requirements for patch-ts v0.9.0.

### 1.1 Scope

- Implement Kotlin, Swift, Scala, Zig languages.
- Configuration file loading and merging.
- Watch mode with `notify`.
- TUI mode with `ratatui`.
- Parallel processing with `rayon`.
- Integration tests.

### 1.2 Out of Scope

- Other languages.
- Semantic repairs.
- Multi‑file patch application.

## 2. System Context & Overview

**Context Diagram (C1):** Unchanged. `Language` trait now has sixteen implementors. New modules: `config`, `watch`, `tui`.

## 3. Functional Capabilities & Behavior

### Feature: Kotlin Language Support

| ID | Requirement | Priority | Acceptance Criteria |
|----|-------------|----------|---------------------|
| FR‑KT‑001 | Provide `KotlinLanguage` implementing `Language`. | Must | Trait methods compile and pass tests. |
| FR‑KT‑002 | Use `tree-sitter-kotlin` grammar. | Must | Correct grammar loaded. |
| FR‑KT‑003 | Support `.kt` and `.kts` extensions. | Must | Auto‑detection works. |

### Feature: Swift Language Support

| ID | Requirement | Priority | Acceptance Criteria |
|----|-------------|----------|---------------------|
| FR‑SW‑001 | Provide `SwiftLanguage`. | Must | Compiles and passes tests. |
| FR‑SW‑002 | Use `tree-sitter-swift`. | Must | Correct grammar. |
| FR‑SW‑003 | Support `.swift`. | Must | Auto‑detection works. |

### Feature: Scala Language Support

| ID | Requirement | Priority | Acceptance Criteria |
|----|-------------|----------|---------------------|
| FR‑SC‑001 | Provide `ScalaLanguage`. | Must | Compiles and passes tests. |
| FR‑SC‑002 | Use `tree-sitter-scala`. | Must | Correct grammar. |
| FR‑SC‑003 | Support `.scala`. | Must | Auto‑detection works. |

### Feature: Zig Language Support

| ID | Requirement | Priority | Acceptance Criteria |
|----|-------------|----------|---------------------|
| FR‑ZG‑001 | Provide `ZigLanguage`. | Must | Compiles and passes tests. |
| FR‑ZG‑002 | Use `tree-sitter-zig`. | Must | Correct grammar. |
| FR‑ZG‑003 | Support `.zig`. | Must | Auto‑detection works. |

### Feature: Configuration File

| ID | Requirement | Priority | Acceptance Criteria |
|----|-------------|----------|---------------------|
| FR‑CFG‑001 | Read `patch-ts.toml` from current directory or ancestors. | Must | Config loaded if present. |
| FR‑CFG‑002 | Merge config with CLI args (CLI overrides). | Must | Correct precedence. |
| FR‑CFG‑003 | Support settings: `fuzz`, `backup`, `auto_repair`, `similarity_threshold`. | Must | Settings applied. |

### Feature: Watch Mode

| ID | Requirement | Priority | Acceptance Criteria |
|----|-------------|----------|---------------------|
| FR‑WATCH‑001 | `--watch` flag enables file monitoring. | Must | Watcher starts. |
| FR‑WATCH‑002 | When watched file changes, apply queued patches. | Must | Patches applied. |
| FR‑WATCH‑003 | Support `--watch-delay` for debouncing. | Should | Configurable delay. |

### Feature: TUI Mode

| ID | Requirement | Priority | Acceptance Criteria |
|----|-------------|----------|---------------------|
| FR‑TUI‑001 | `--tui` flag launches interactive terminal UI. | Must | TUI displays. |
| FR‑TUI‑002 | TUI shows diff of proposed changes. | Must | Diff visible. |
| FR‑TUI‑003 | User can accept (apply) or reject (skip) patch. | Must | Actions work. |
| FR‑TUI‑004 | TUI supports keyboard navigation (arrows, Enter, Esc). | Must | Navigation works. |

### Feature: Parallel Processing

| ID | Requirement | Priority | Acceptance Criteria |
|----|-------------|----------|---------------------|
| FR‑PAR‑001 | Multi‑file `balance` uses `rayon` parallel iterators. | Must | Files processed in parallel. |
| FR‑PAR‑002 | Parallel mode is enabled by default; `--no-parallel` disables. | Should | Flag works. |

### Feature: Language Detection Update

| ID | Requirement | Priority | Acceptance Criteria |
|----|-------------|----------|---------------------|
| FR‑LANG‑013 | Detect Kotlin from `.kt`/`.kts`. | Must | Returns `KotlinLanguage`. |
| FR‑LANG‑014 | Detect Swift from `.swift`. | Must | Returns `SwiftLanguage`. |
| FR‑LANG‑015 | Detect Scala from `.scala`. | Must | Returns `ScalaLanguage`. |
| FR‑LANG‑016 | Detect Zig from `.zig`. | Must | Returns `ZigLanguage`. |

## 4. Quality & Non‑Functional Requirements

| ID | Category | Requirement | Fit Criterion |
|----|----------|-------------|---------------|
| NFR‑PERF‑001 | Performance | Watch mode response ≤ 500ms. | Benchmarked. |
| NFR‑PERF‑002 | Performance | Parallel balance ≥50% faster on 4+ cores. | Benchmarked. |
| NFR‑USAB‑001 | Usability | TUI requires ≤3 keystrokes to approve/reject. | User testing. |
| NFR‑COMPAT‑001 | Compatibility | All v0.8.0 tests pass without modification. | CI regression suite. |

## 5. External Interfaces & Data Contracts

**CLI Interface:** New flags: `--watch`, `--watch-delay`, `--tui`, `--no-parallel`.

**Configuration File Schema (`patch-ts.toml`):**
```toml
fuzz = 5
backup = true
auto_repair = true
similarity_threshold = 0.9
```

**JSON Output Schema:** Unchanged.

## 6. Constraints, Assumptions & Dependencies

| Type | Description |
|------|-------------|
| Constraint | Add `tree-sitter-kotlin`, `tree-sitter-swift`, `tree-sitter-scala`, `tree-sitter-zig`, `ratatui`, `crossterm`, `notify`, `rayon`, `serde`, `toml`. |
| Assumption | Grammars expose standard delimiter nodes. |
| Dependency | `tree-sitter` 0.26. |

## 7. TBD Log

| ID | Item | Owner | Due |
|----|------|-------|-----|
| TBD‑001 | Determine exact TUI layout and key bindings. | Engineering | Before implementation. |

## 8. Requirements Attributes & Traceability Model

**ID Scheme:**  
- Functional: `FR‑KT‑###`, `FR‑SW‑###`, `FR‑SC‑###`, `FR‑ZG‑###`, `FR‑CFG‑###`, `FR‑WATCH‑###`, `FR‑TUI‑###`, `FR‑PAR‑###`, `FR‑LANG‑###`  
- NFR: `NFR‑CAT‑###`

**Traceability Matrix (excerpt):**

| SRS ID | BRS ID | Verification Method |
|--------|--------|---------------------|
| FR‑KT‑001 | BR‑001 | Test |
| FR‑SW‑001 | BR‑002 | Test |
| FR‑SC‑001 | BR‑003 | Test |
| FR‑ZG‑001 | BR‑004 | Test |
| FR‑CFG‑001 | BR‑005 | Test |
| FR‑WATCH‑001 | BR‑006 | Test |
| FR‑TUI‑001 | BR‑007 | Test + Demo |
| FR‑PAR‑001 | BR‑008 | Test + Bench |

---

# patch-ts Architecture & Design Specification

| Field | Value |
|-------|-------|
| Project | patch-ts |
| Document | Architecture & Design Specification |
| Version | 7.0 (v0.9.0) |
| Date | 2026-04-22 |
| Author | patch-ts team, assisted by spec-writer |
| Status | Draft |
| References | SRS v7.0, BRS v7.0 |

## 1. Context & Scope

This document describes architectural changes for v0.9.0: adding four languages, configuration, watch mode, TUI, and parallel processing.

## 2. Goals & Non‑Goals

### Goals

- Add `KotlinLanguage`, `SwiftLanguage`, `ScalaLanguage`, `ZigLanguage` via macro.
- Implement `Config` struct and loading from `patch-ts.toml`.
- Implement `Watcher` using `notify`.
- Implement `TuiApp` using `ratatui` and `crossterm`.
- Add `rayon` parallel iterators for multi‑file operations.

### Non‑Goals

- Refactor `Language` trait.
- Add language‑specific semantic repairs.
- Multi‑file patch application.

## 3. Architecturally Significant Requirements (ASRs)

| ASR ID | Description | Source |
|--------|-------------|--------|
| ASR‑001 | Configuration must merge with CLI precedence. | FR‑CFG‑002 |
| ASR‑002 | Watch mode must be responsive and cross‑platform. | FR‑WATCH‑001 |
| ASR‑003 | TUI must be non‑blocking and responsive. | FR‑TUI‑001 |
| ASR‑004 | Parallel processing must not introduce data races. | FR‑PAR‑001 |

## 4. The Design

### 4.1 System Overview (C4 Level 2)

```
[User/Agent] → (CLI) → [patch-ts Binary]
                           ├── cli.rs (language detection, config loading)
                           ├── config.rs (Config struct, TOML parsing)
                           ├── watch.rs (Watcher, file monitoring)
                           ├── tui.rs (TuiApp, ratatui UI)
                           ├── ast.rs (Language trait + 16 implementors)
                           ├── repair.rs (AST‑based repair + rollback)
                           └── file.rs (atomic write)
                                     ↓
              ┌──────────────────┼──────────────────┬──────────────────┬──────────────────┐
              ↓                  ↓                  ↓                  ↓                  ↓
     [tree-sitter-*]   [tree-sitter-kotlin] [tree-sitter-swift] [tree-sitter-scala] [tree-sitter-zig]
```

### 4.2 Key Design Changes

**4.2.1 Language Implementations**

Use the existing macro to generate the four new languages:

```rust
impl_language!(KotlinLanguage, tree_sitter_kotlin::LANGUAGE);
impl_language!(SwiftLanguage, tree_sitter_swift::LANGUAGE);
impl_language!(ScalaLanguage, tree_sitter_scala::LANGUAGE);
impl_language!(ZigLanguage, tree_sitter_zig::LANGUAGE);
```

**4.2.2 Configuration**

`Config` struct with `serde::Deserialize`:
```rust
#[derive(Debug, Deserialize)]
pub struct Config {
    pub fuzz: Option<usize>,
    pub backup: Option<bool>,
    pub auto_repair: Option<bool>,
    pub similarity_threshold: Option<f64>,
}
```
Loaded from `patch-ts.toml` in current directory or ancestors. Merged with `PatchOptions` (CLI overrides).

**4.2.3 Watch Mode**

`Watcher` struct using `notify`:
```rust
pub fn watch<P: AsRef<Path>>(path: P, queue: Vec<Patch>) -> Result<()> {
    let (tx, rx) = channel();
    let mut watcher = RecommendedWatcher::new(tx, Config::default())?;
    watcher.watch(path.as_ref(), RecursiveMode::Recursive)?;
    for event in rx {
        // Debounce, then apply queued patches
    }
}
```

**4.2.4 TUI Mode**

`TuiApp` using `ratatui` and `crossterm`:
```rust
pub fn run_tui(patch: Patch) -> Result<bool> {
    // Setup terminal, run event loop
    // Display diff, handle keyboard
    // Return true if approved
}
```

**4.2.5 Parallel Processing**

Multi‑file `balance` uses `rayon`:
```rust
files.par_iter().for_each(|file| {
    balance_file(file, ...).unwrap();
});
```

### 4.3 Data Model

New structs: `Config`, `Watcher`, `TuiApp`.

### 4.4 Security Architecture

No changes.

## 5. Architecture Decision Records (ADRs)

### ADR‑015: Use `notify` for cross‑platform file watching

**Context:** Need reliable file monitoring across Linux, macOS, Windows.

**Decision:** Use `notify` crate with `RecommendedWatcher`.

**Alternatives:** Platform‑specific APIs. Rejected for complexity.

**Consequences:** Cross‑platform support with minimal code.

### ADR‑016: Use `ratatui` for TUI

**Context:** Need a lightweight, Rust‑native TUI library.

**Decision:** Use `ratatui` with `crossterm` backend.

**Alternatives:** `tui-rs` (deprecated), `cursive`. Rejected for ecosystem maturity.

**Consequences:** Rich, customizable TUI with active maintenance.

### ADR‑017: Use `rayon` for parallel processing

**Context:** Multi‑file operations are CPU‑bound and independent.

**Decision:** Use `rayon` parallel iterators.

**Alternatives:** Manual threading with `std::thread`. Rejected for complexity.

**Consequences:** Simple, safe parallelism with work‑stealing.

## 6. API & Interface Contracts

CLI extended with new flags. Configuration file schema defined.

## 7. Cross‑cutting Concerns

| Concern | Approach |
|---------|----------|
| **Testing** | New integration tests for languages, config, watch, TUI (simulated). |
| **Performance** | Benchmark parallel vs serial; watch mode latency. |

## 8. Alternatives Considered

| Alternative | Why Rejected |
|-------------|--------------|
| Custom file watcher per platform | Too complex. |
| Other TUI libraries | `ratatui` is most mature. |
| Manual threading | `rayon` simpler and safer. |

## 9. Traceability

| ASR | ADR | Component |
|-----|-----|-----------|
| ASR‑001 | – | config.rs |
| ASR‑002 | ADR‑015 | watch.rs |
| ASR‑003 | ADR‑016 | tui.rs |
| ASR‑004 | ADR‑017 | repair.rs, cli.rs |

---

# patch-ts Behavioral Specification & Test Verification Plan

| Field | Value |
|-------|-------|
| Project | patch-ts |
| Document | Behavioral Specification & Test Verification Plan |
| Version | 7.0 (v0.9.0) |
| Date | 2026-04-22 |
| Author | patch-ts team, assisted by spec-writer |
| Status | Draft |
| References | SRS v7.0, Architecture v7.0 |

## 1. Behavioral Specifications (Specification by Example)

### Feature: Kotlin Support

```gherkin
Feature: Kotlin language support
  Scenario: Patch a Kotlin file exactly
    Given a file "Main.kt" with content:
      """
      fun main() {
          println("hello")
      }
      """
    When I run `patch-ts patch --file Main.kt --line 2 --old "    println(\"hello\")" --new "    println(\"world\")"`
    Then the file contains "println(\"world\")"
```

### Feature: Configuration File

```gherkin
Feature: Configuration file
  Scenario: CLI flag overrides config file
    Given a file "patch-ts.toml" with content:
      """
      fuzz = 5
      """
    And a file "main.rs" needing fuzz 10
    When I run `patch-ts patch --file main.rs --line 1 --old "x" --new "y" --fuzz 10`
    Then the CLI fuzz value 10 is used, not 5.
```

### Feature: Watch Mode

```gherkin
Feature: Watch mode
  Scenario: File change triggers patch
    Given a watcher running on "src/"
    And a queued patch for "src/main.rs"
    When I modify and save "src/main.rs"
    Then the patch is applied within 500ms.
```

### Feature: TUI Mode

```gherkin
Feature: TUI mode
  Scenario: Review and approve a patch
    Given a patch for "main.rs"
    When I run `patch-ts patch --file main.rs --tui`
    Then a TUI displays the diff.
    And I can press 'y' to accept or 'n' to reject.
```

### Feature: Parallel Processing

```gherkin
Feature: Parallel processing
  Scenario: Balance multiple files in parallel
    Given 10 Rust files with delimiter errors
    When I run `patch-ts balance --files "*.rs" --apply`
    Then all files are processed correctly.
    And the operation completes faster than serial execution.
```

## 2. Test Strategy & Plan

### 2.1 Test Pyramid

| Level | Scope | Tools |
|-------|-------|-------|
| Unit | Config loading, language detection | Rust `#[test]` |
| Integration | Full commands on new languages, watch mode (simulated) | `assert_cmd` |
| TUI | Simulated input testing | `ratatui` testing utilities |
| Regression | v0.8.0 test suite | Cargo test |
| Performance | Parallel vs serial benchmark | Criterion |

### 2.2 Risk‑Based Prioritization

| Risk | Test Focus |
|------|------------|
| Watch mode false positives | Extensive testing with different filesystems. |
| TUI cross‑platform issues | Test on Linux, macOS, Windows CI. |
| Parallel data races | Use `rayon`'s safety guarantees; test with ThreadSanitizer. |

## 3. Test Case Specifications

### TC‑KT‑001: Kotlin patch exact

- **Requirement:** FR‑KT‑001
- **Preconditions:** `.kt` file.
- **Steps:** Run `patch --line X --old ... --new ...`.
- **Expected:** Patch applied.
- **Automated:** Yes.

### TC‑CFG‑001: Config file loading

- **Requirement:** FR‑CFG‑001
- **Preconditions:** `patch-ts.toml` exists.
- **Steps:** Run command without CLI flags.
- **Expected:** Config values used.
- **Automated:** Yes.

### TC‑WATCH‑001: Watch mode applies patch

- **Requirement:** FR‑WATCH‑001
- **Preconditions:** Watcher running.
- **Steps:** Modify watched file.
- **Expected:** Patch applied.
- **Automated:** Yes (simulated file change).

### TC‑TUI‑001: TUI accept patch

- **Requirement:** FR‑TUI‑003
- **Preconditions:** Patch provided.
- **Steps:** Run `--tui`, press 'y'.
- **Expected:** Patch applied.
- **Automated:** Yes (simulated input).

### TC‑PAR‑001: Parallel balance

- **Requirement:** FR‑PAR‑001
- **Preconditions:** Multiple files.
- **Steps:** Run `balance --files`.
- **Expected:** All files processed.
- **Automated:** Yes.

## 4. NFR Verification Plans

### NFR‑PERF‑001 (Watch latency)

- **Method:** Measure time from file change to patch application.
- **Threshold:** ≤ 500ms.

### NFR‑PERF‑002 (Parallel speedup)

- **Method:** Benchmark 10‑file balance serial vs parallel.
- **Threshold:** ≥50% faster on 4+ cores.

## 5. Requirements Traceability Matrix (RTM)

| SRS ID | Test Case(s) | Verification Method |
|--------|--------------|---------------------|
| FR‑KT‑001 | TC‑KT‑001 | Test |
| FR‑CFG‑001 | TC‑CFG‑001 | Test |
| FR‑WATCH‑001 | TC‑WATCH‑001 | Test |
| FR‑TUI‑003 | TC‑TUI‑001 | Test |
| FR‑PAR‑001 | TC‑PAR‑001 | Test |

---

This completes the specification suite for patch-ts v0.9.0.

---

# patch-ts v0.9.0 Implementation Plan

> **For agentic workers:** REQUIRED: Use superpowers:subagent-driven-development (if subagents available) or superpowers:executing-plans to implement this plan. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add Kotlin, Swift, Scala, Zig support; implement configuration file, watch mode, TUI, and parallel processing.

**Architecture:** Extend language detection, implement four new languages via macro, add `config` module, `watch` module, `tui` module, and integrate `rayon` for parallelism.

**Tech Stack:** Rust, tree-sitter 0.26, tree-sitter-kotlin, tree-sitter-swift, tree-sitter-scala, tree-sitter-zig, ratatui, crossterm, notify, rayon, serde, toml.
