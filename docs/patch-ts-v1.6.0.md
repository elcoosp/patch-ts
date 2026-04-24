# patch-ts v1.6.0 — Intelligent & Integrated Patching Specification Suite

Below are the five core specification documents for **patch‑ts v1.6.0**, which transforms the tool from a passive applier of patches into an intelligent teammate that understands compiler errors, works natively with Git, integrates seamlessly into CI/CD pipelines, and validates changes across entire projects — all while maintaining the safety and resilience built in previous versions.

---

## 1. Vision & Strategic Alignment

```markdown
# Product Vision & Strategic Alignment — patch‑ts v1.6.0

| Field | Value |
|-------|-------|
| Project | patch‑ts |
| Document | Vision & Strategic Alignment |
| Version | 1.0 |
| Date | 2026‑04‑24 |
| Author | patch‑ts team, assisted by spec‑writer |
| Status | Draft |

## 1. Vision Statement

> **patch‑ts becomes the world’s first intelligent patching companion — one that not only applies changes safely, but understands compiler errors, volunteers fixes, works natively with Git, and guards every pull request with automated, spec‑aware validation.**

## 2. Elevator Pitch (Moore Template)

> For **development teams and AI agents who want to close the loop between compiler errors, Git history, and CI/CD pipelines**, patch‑ts is a **tree‑sitter‑backed CLI that now proposes fixes from compiler output, applies patches directly to commits, powers GitHub Actions, and validates changes across the entire codebase**. Unlike other tools that wait for you to figure out the fix, our product **reads the compiler error, suggests the patch, lets you review it in Git context, and then ensures nothing broke — automatically.**

## 3. Problem Statement & Business Context

**Problem:** Even with LLM‑resilient patching, the workflow is still fragmented:
- The developer (or AI agent) must manually copy compiler errors, craft a patch, and run it.  
- There’s no built‑in connection between `git diff`, `git show`, and patching.  
- CI/CD pipelines don’t use patch‑ts to validate PRs or auto‑fix common mistakes.  
- There’s no learning from past patches; every fix is a one‑off.  
- Cross‑file impacts are invisible — a change in one file might break another.

**Why now:**
- The “patching engine” is mature; it’s time to wrap it with **intelligence and integration**.  
- The compiler‑error‑to‑fix pattern is the most requested feature from early adopters.  
- GitHub Actions and GitLab CI are standard; with a single action, patch‑ts can gate every PR.  
- The `.patch‑ts/history.jsonl` file is already collecting data — it’s time to make it useful.

**Business drivers:**  
- Eliminate the manual “compiler error → copy/paste → run patch‑ts” step for hundreds of developers.  
- Become a standard CI quality gate for AI‑generated code.  
- Increase patch‑ts’s value per installation by making it an everyday Git companion.

## 4. Target Users / Customers

| Segment | Description |
|---------|-------------|
| **Individual developers** | Want to type `patch‑ts fix` and have the tool read the compiler error and propose a fix. |
| **Team leads & maintainers** | Want to enforce that all PRs pass patch‑ts validation, and that common issues are auto‑fixed. |
| **DevOps / CI engineers** | Need a pre‑built GitHub Action that adds lint‑style comments on PRs. |
| **AI agent frameworks** | Continue to benefit from a single, standard patching backend that now understands compiler output and Git context. |

**Explicitly NOT targeting (v1.6.0):**  
- Full ML‑based repair (still research).  
- Web playground.  
- Real‑time collaborative editing.  
- Smart contract or formal verification.

## 5. User Needs & Value Proposition

| Need | patch‑ts v1.6.0 Value |
|------|-----------------------|
| “I got a compiler error — just fix it.” | `patch‑ts fix` parses the error and proposes a patch, with confidence scoring. |
| “I want to apply a patch from a specific commit.” | `patch‑ts git apply <commit>` extracts the diff and applies it safely. |
| “I want to see how a patch changes between my branch and main.” | `patch‑ts git diff <branch>` shows the patch and lets you apply it. |
| “I want my CI to block PRs that introduce delimiter errors.” | Pre‑built GitHub Action and GitLab CI configuration that run patch‑ts validation. |
| “I changed a function signature — did I break callers?” | Cross‑file semantic analysis warns when a patch affects other files. |
| “I want patch‑ts to learn my project’s patterns and get smarter.” | History‑based suggestion of confidence thresholds, fuzz radii, and strategies. |

**Differentiator:** No other patching tool integrates with the compiler, Git history, CI/CD, and cross‑file analysis in a single open‑source binary.

## 6. Desired Outcomes & Success Metrics

### Business Outcomes (v1.6.0)

| ID | Outcome | Key Result / Target |
|----|---------|---------------------|
| G‑1 | Reduce time from compiler error to applied fix | `patch‑ts fix` produces a correct patch in ≥70% of common Rust/TS/JS error cases. |
| G‑2 | Increase CI adoption | At least 200 repositories use the GitHub Action within 3 months of release. |
| G‑3 | Increase daily active users | ≥25% growth in installations (measured via crates.io download stats). |

### Product Outcomes (v1.6.0)

| ID | Outcome | Metric |
|----|---------|--------|
| P‑1 | Compiler‑error‑to‑patch works | 100% of a curated set of 50 common compiler errors produce a valid patch suggestion. |
| P‑2 | Git integration works | `patch‑ts git apply <ref>` applies the exact diff from the commit. |
| P‑3 | GitHub Action passes | The action is published on the GitHub Marketplace and passes all integration tests. |
| P‑4 | Cross‑file warnings are actionable | False‑positive rate <10% on a test corpus of 20 multi‑file Rust projects. |

## 7. Strategic Constraints

| Constraint | Description |
|------------|-------------|
| **Backward compatibility** | All v1.5.0 CLI flags, JSON schemas, MCP interface, and WASM plugin interfaces remain unchanged. |
| **Performance** | `patch‑ts fix` must complete in under 2 seconds for typical files; cross‑file analysis under 5 seconds per project. |
| **Cross‑platform** | All new features must work on Linux, macOS, and Windows. |
| **Dependencies** | No new heavy dependencies; compiler error parsing uses existing `regex` and tree‑sitter; Git integration uses existing `git2` crate. |

## 8. Goals and Non‑Goals (v1.6.0)

### Goals

- Implement `patch‑ts fix` subcommand that parses compiler errors and proposes patches.
- Add `patch‑ts git apply <ref>` and `patch‑ts git diff <branch>` subcommands.
- Build a GitHub Action that runs patch‑ts validation on PRs.
- Implement cross‑file semantic analysis (warnings when a patch affects symbols used in other files).
- Add history‑based suggestion of confidence thresholds and strategies.
- Update documentation, README, and integration guides.

### Non‑Goals (explicitly excluded)

- Full ML‑based repair.
- Web‑based playground.
- Real‑time collaborative editing.
- Formal verification.
- Decentralized patch sharing.

## 9. Operational Concept & High‑Level Scenarios

### Concept of Operations

A developer (or AI agent) compiles their code and gets an error. Instead of copying the error and pasting it into an LLM, they run `patch‑ts fix`. patch‑ts reads the compiler error (from stdin or a log file), maps it to an AST‑aware repair, and proposes a patch with confidence scoring. The user reviews and accepts with `--apply`.

In a Git workflow, the developer can apply a patch from a specific commit (`patch‑ts git apply HEAD~1`), diff their branch against main (`patch‑ts git diff main`), or gate pull requests with a pre‑built GitHub Action that posts validation results as PR comments.

When a patch changes a function signature, patch‑ts warns if other files in the project call that function and might break.

### High‑Level Scenarios (v1.6.0)

1. **Compiler‑error‑driven fix**  
   `$ cargo build 2>&1 | patch‑ts fix --apply`  
   patch‑ts parses the error, finds the offending line, and inserts the missing closing brace. The file is fixed and ready to compile.

2. **Git‑based patch application**  
   `$ patch‑ts git apply abc123 --file src/main.rs`  
   The diff introduced by commit `abc123` is extracted and applied to `src/main.rs` with full validation.

3. **Cross‑file semantic warning**  
   `$ patch‑ts patch --file src/lib.rs --line 10 --old "fn get_user(id: i32)" --new "fn get_user(id: u64)"`  
   patch‑ts applies the patch but warns: “Function `get_user` is called in `src/main.rs` at line 25 with an `i32` argument. This change may break the caller.”

4. **GitHub Action in action**  
   A PR is opened on GitHub. The patch‑ts action runs automatically, validating all changed files. If a file has an unbalanced delimiter, the action posts a comment with the suggested fix and a link to the relevant patch‑ts command.

## 10. Stakeholders, Sponsorship & Governance

| Role | Name / Org | Responsibility |
|------|------------|----------------|
| **Executive Sponsor** | Project maintainer | Approves strategic direction. |
| **Product Owner** | Project maintainer | Prioritizes features, manages scope. |
| **Engineering Lead** | Core contributor(s) | Oversees technical implementation. |
| **Community** | Open‑source contributors | Review PRs, test pre‑releases. |

## 11. Risks, Assumptions & Open Questions

### Assumptions

- The `git2` crate can handle all required Git operations (diff extraction, commit history) reliably.  
- Compiler error messages are well‑formed enough for regex‑based parsing.  
- The GitHub Actions marketplace will accept the action if it’s well‑documented.

### Risks

| Risk | Likelihood | Impact | Mitigation |
|------|------------|--------|------------|
| Compiler error parsing may be brittle across compiler versions. | Medium | High | Use a test corpus of real compiler outputs; fall back to a generic error message if parsing fails. |
| Cross‑file analysis may produce false positives. | Medium | Medium | Make warnings non‑blocking; allow project‑specific ignore lists. |
| GitHub Action may be rejected or take time to publish. | Low | Low | Provide manual CI integration instructions as fallback. |

### Open Questions

- Should `patch‑ts fix` be interactive (TUI) or purely command‑line (CLI)? (Both: CLI for scripts, `--tui` for review.)  
- How deep should cross‑file analysis go? (Start with direct callsite matching; expand incrementally.)  
- Should the GitHub Action be a separate repository or in‑tree? (In‑tree under `.github/actions/patch‑ts`.)

---

*This vision document anchors the intelligent and integrated patching strategy for patch‑ts v1.6.0.*
```

---

## 2. Business & Stakeholder Requirements Specification (BRS)

```markdown
# Business & Stakeholder Requirements Specification — patch‑ts v1.6.0

| Field | Value |
|-------|-------|
| Project | patch‑ts |
| Document | Business & Stakeholder Requirements Specification |
| Version | 1.0 |
| Date | 2026‑04‑24 |
| Author | patch‑ts team, assisted by spec‑writer |
| Status | Draft |
| References | Vision v1.6.0 |

## 1. Business Context

### 1.1 Purpose

This BRS defines the business‑level requirements for patch‑ts v1.6.0, which adds compiler‑error‑driven fix suggestions, Git‑native patching commands, a GitHub Action for CI/CD, cross‑file semantic analysis, and history‑based adaptive tuning.

### 1.2 Business Problem / Opportunity

v1.5.0 hardened patch‑ts against malformed LLM output, but the tool is still passive — it waits for a patch to be given. Developers still spend time manually bridging the gap between compiler errors, Git history, and CI pipelines. The business opportunity is to make patch‑ts **proactive** and **integrated**, reducing the time from “something broke” to “something is fixed” by an order of magnitude.

### 1.3 Scope Boundaries

**In Scope:**
- `patch‑ts fix` subcommand (compiler error → patch suggestion).
- `patch‑ts git apply` and `patch‑ts git diff` subcommands.
- Pre‑built GitHub Action and CI integration templates.
- Cross‑file semantic analysis (call‑site warnings).
- History‑based adaptive threshold and strategy suggestions.
- Documentation updates for all new features.

**Out of Scope:**
- Full ML‑based repair.
- Web‑based playground.
- Real‑time collaborative editing.
- Decentralized patch sharing.

## 2. Business Goals, Objectives & Success Metrics

| ID | Goal | Fit Criterion |
|----|------|---------------|
| BR‑001 | Reduce time from compiler error to fix | `patch‑ts fix` produces correct patch in ≥70% of common error cases in benchmark. |
| BR‑002 | Increase CI adoption | At least 200 repositories use the GitHub Action within 3 months. |
| BR‑003 | Increase daily active users | ≥25% growth in crates.io download stats within 3 months. |

*(Traceability: BR‑001…003 ← Vision G‑1…G‑3)*

## 3. Business Model & Processes

patch‑ts remains open‑source. Intelligent features and tight CI integration make it indispensable for professional teams, driving enterprise adoption and community contributions.

## 4. Business Rules & Policies

| ID | Rule | Source |
|----|------|--------|
| BR‑R1 | `patch‑ts fix` must never apply a patch without explicit `--apply` (dry‑run by default). | Safety |
| BR‑R2 | Cross‑file warnings must be non‑blocking (informational only). | Developer trust |
| BR‑R3 | Git operations must respect the current repository’s `.gitignore` and never modify the Git index unless explicitly requested. | Data integrity |

## 5–12. Additional Sections

*(Follow the same pattern as previous BRS documents: Glossary, Conceptual Domain Model, Stakeholder Needs, System‑in‑Context, Constraints, Risks, Traceability.)*

Key Stakeholder Needs:
- SN‑001: As a developer, I want to run `patch‑ts fix` after a compiler error and get a ready‑to‑apply patch.
- SN‑002: As a developer, I want to apply a patch from a specific Git commit without manually copying diffs.
- SN‑003: As a team lead, I want a GitHub Action that validates every PR for syntax and delimiter issues.
- SN‑004: As a developer, I want to be warned when my patch changes a function that is used elsewhere.

---

*This BRS establishes the business foundation for v1.6.0.*
```

---

## 3. Software Requirements Specification (SRS)

```markdown
# Software Requirements Specification — patch‑ts v1.6.0

| Field | Value |
|-------|-------|
| Project | patch‑ts |
| Document | Software Requirements Specification |
| Version | 1.0 |
| Date | 2026‑04‑24 |
| Author | patch‑ts team, assisted by spec‑writer |
| Status | Draft |
| References | BRS v1.6.0, Vision v1.6.0 |

## 1. Introduction & Scope

This SRS defines the functional and non‑functional requirements for patch‑ts v1.6.0, which adds compiler‑error‑driven fixes, Git integration, CI/CD actions, cross‑file analysis, and adaptive tuning.

### 1.1 Scope

- `patch‑ts fix` subcommand with compiler‑error parsing.
- `patch‑ts git apply` and `patch‑ts git diff` subcommands.
- Pre‑built GitHub Action and GitLab CI template.
- Cross‑file semantic analysis module.
- History‑based adaptive suggestion module.
- Integration tests and documentation.

### 1.2 Out of Scope

- ML‑based repair.
- Web playground.
- Real‑time collaboration.
- Decentralized sharing.

## 2. System Context & Overview

Same C1 context as before. New internal modules: `fix.rs` (compiler error parsing), `crossfile.rs` (cross‑file analysis), `history_adaptive.rs` (adaptive tuning). Enhancements to `mcp.rs` (expose new tools). New CI artifacts: `.github/actions/patch‑ts/action.yml` and `gitlab‑ci/patch‑ts‑validation.yml`.

## 3. Functional Capabilities & Behavior

### Feature: Compiler‑Error‑Driven Fix (`patch‑ts fix`)

| ID | Requirement | Priority | Acceptance Criteria |
|----|-------------|----------|---------------------|
| FR‑FIX‑001 | `patch‑ts fix` shall accept compiler error output via stdin or `--error‑file <PATH>`. | Must | Error text is read and parsed. |
| FR‑FIX‑002 | The tool shall parse common Rust (`error[E...]`), TypeScript (`error TS...`), JavaScript (`SyntaxError`), and Python (`SyntaxError`) error formats. | Must | Recognized errors yield a patch suggestion. |
| FR‑FIX‑003 | For each recognized error, the tool shall propose an AST‑aware patch (e.g., insert missing delimiter, replace mismatched type). | Must | Patch is syntactically valid. |
| FR‑FIX‑004 | The proposed patch shall include a confidence score and explanation. | Must | JSON output includes `confidence` and `explanation`. |
| FR‑FIX‑005 | `--apply` flag shall apply the suggested patch after user confirmation (or with `--force`). | Must | Patch is applied correctly. |

### Feature: Git‑Native Patching

| ID | Requirement | Priority | Acceptance Criteria |
|----|-------------|----------|---------------------|
| FR‑GIT‑001 | `patch‑ts git apply <commit>` shall extract the diff introduced by a commit and apply it to the specified file. | Must | The diff is applied and validated. |
| FR‑GIT‑002 | `patch‑ts git diff <branch>` shall compare the current branch with `<branch>` and produce a patch that can be reviewed or applied. | Must | The diff is generated correctly. |
| FR‑GIT‑003 | Both subcommands shall support `--dry‑run` and `--fuzz`. | Must | Flags honored. |

### Feature: CI/CD Integration

| ID | Requirement | Priority | Acceptance Criteria |
|----|-------------|----------|---------------------|
| FR‑CI‑001 | A GitHub Action shall be provided that runs `patch‑ts balance --files "**/*.rs" --json` on all changed Rust files in a PR. | Must | Action runs on `pull_request` and posts comments. |
| FR‑CI‑002 | A GitLab CI template shall be provided for the same purpose. | Should | Template can be included from a GitLab CI pipeline. |
| FR‑CI‑003 | The CI action shall fail the check if any file has unresolved delimiter errors (unless `--force` is configured). | Must | Check fails when errors are present. |

### Feature: Cross‑File Semantic Analysis

| ID | Requirement | Priority | Acceptance Criteria |
|----|-------------|----------|---------------------|
| FR‑CROSS‑001 | After a successful patch, the tool shall scan other files in the project for callsites of any changed function or method. | Must | Warnings emitted for affected callers. |
| FR‑CROSS‑002 | Cross‑file analysis shall be opt‑in via `--cross‑file` flag (to avoid performance impact by default). | Should | Flag honored. |
| FR‑CROSS‑003 | The analysis shall build a project‑wide symbol index using tree‑sitter queries, cached between invocations. | Should | Cache invalidated when files change. |

### Feature: History‑Based Adaptive Tuning

| ID | Requirement | Priority | Acceptance Criteria |
|----|-------------|----------|---------------------|
| FR‑ADAPT‑002 | `patch‑ts adapt‑threshold` shall read `.patch‑ts/history.jsonl` and suggest an optimal confidence threshold. | Should | Suggestion is based on success/failure rates. |
| FR‑ADAPT‑003 | `patch‑ts adapt‑strategy` shall suggest which matching strategies to prioritize or deprioritize. | Could | Suggestion is based on strategy success rates. |

## 4. Quality & Non‑Functional Requirements

| ID | Category | Requirement | Fit Criterion |
|----|----------|-------------|---------------|
| NFR‑PERF‑001 | Performance | `patch‑ts fix` must complete in ≤2s for a typical file. | Benchmarked. |
| NFR‑PERF‑002 | Performance | Cross‑file analysis must complete in ≤5s for a project with up to 100 files. | Benchmarked. |
| NFR‑SEC‑001 | Security | The GitHub Action must not require write access beyond pull request comments (unless configured). | Audit. |
| NFR‑COMPAT‑001 | Compatibility | All v1.5.0 tests pass without modification. | CI regression suite. |
| NFR‑DOC‑001 | Documentation | New commands and CI integrations are documented in README and inline help. | Review checklist. |

## 5. External Interfaces & Data Contracts

### CLI New/Modified Flags

- `fix` subcommand: `--error-file <PATH>`, `--apply`, `--force`.
- `git` subcommand with `apply` and `diff` sub‑subcommands.
- `--cross‑file` flag on `patch`, `balance`, and `git apply`.
- `adapt‑threshold` and `adapt‑strategy` subcommands.

### JSON Output Schema (Additions)

```json
{
  "fix": {
    "error_type": "E0308",
    "file": "src/main.rs",
    "line": 42,
    "column": 5,
    "suggested_patch": {
      "old": "timeout",
      "new": "Duration::from_secs(timeout)",
      "confidence": 0.85,
      "explanation": "Mismatched types: expected `Duration`, found `u64`"
    }
  },
  "cross_file_warnings": [
    {
      "file": "src/main.rs",
      "line": 25,
      "message": "Function `get_user` changed signature; caller may break."
    }
  ]
}
```

## 6–8. Constraints, Assumptions, TBD

*(Summarized: must maintain backward compatibility, `git2` stable, compiler error formats well‑defined.)*

---

*This SRS defines the complete behavioral contract for v1.6.0.*
```

---

## 4. Architecture & Design Specification

```markdown
# Architecture & Design Specification — patch‑ts v1.6.0

| Field | Value |
|-------|-------|
| Project | patch‑ts |
| Document | Architecture & Design Specification |
| Version | 1.0 |
| Date | 2026‑04‑24 |
| Author | patch‑ts team, assisted by spec‑writer |
| Status | Draft |
| References | SRS v1.6.0, BRS v1.6.0 |

## 1. Context & Scope

This document describes the architectural design for the intelligent and integrated features in v1.6.0: compiler‑error parsing, Git commands, CI actions, cross‑file analysis, and adaptive tuning.

## 2. Goals & Non‑Goals

**Goals:**
- Introduce `fix.rs` for parsing compiler errors and generating patch suggestions.
- Introduce `crossfile.rs` for project‑wide symbol index and callsite analysis.
- Introduce `history_adaptive.rs` for reading history and computing suggestions.
- Extend CLI with `fix`, `git apply`, `git diff`, `adapt‑threshold`, `adapt‑strategy`.
- Create CI templates in `.github/actions/` and `gitlab‑ci/`.
- Keep all changes backward‑compatible.

**Non‑Goals:**
- Rewrite existing Git integration; use `git2` crate for all operations.
- Introduce a full language server for cross‑file analysis; use tree‑sitter queries.

## 3. Architecturally Significant Requirements (ASRs)

| ASR ID | Description | Source |
|--------|-------------|--------|
| ASR‑001 | Compiler error parsing must support multiple languages and compilers. | FR‑FIX‑002 |
| ASR‑002 | Cross‑file analysis must be fast and cacheable. | FR‑CROSS‑003 |
| ASR‑003 | Git commands must never corrupt the repository state. | BR‑R3 |
| ASR‑004 | CI action must run self‑contained without extra dependencies. | FR‑CI‑001 |

## 4. The Design

### 4.1 System Overview (C4 Level 2)

```
[User/CI] → [CLI] → [fix.rs] [git commands] [crossfile.rs] [adaptive.rs]
                        ↓            ↓             ↓              ↓
                   [Compiler Output]  [git2]   [Symbol Index] [History]
```

### 4.2 Key Design Changes

**Compiler Error Parsing (`fix.rs`)**
- `parse_error(output: &str) -> Option<CompilerError>` uses regex patterns for each supported compiler.
- `suggest_fix(error: &CompilerError, file_content: &str, lang: &str) -> Option<PatchSuggestion>` uses tree‑sitter to locate the error node and propose a fix (e.g., insert missing delimiter, replace type).

**Git Commands**
- `git apply <commit> --file <path>` uses `git2::Repository::revparse_single` and `diff_tree_to_tree` to extract the relevant patch.
- `git diff <branch>` uses `git2` to diff the current HEAD against the branch and outputs a unified diff.

**Cross‑File Analysis (`crossfile.rs`)**
- `build_project_index(root: &Path) -> HashMap<String, SymbolIndex>` walks all source files in the project, builds a symbol index per file, and caches it in `.patch‑ts/cache/`.
- `find_callers(func_name: &str, project_index: &HashMap<String, SymbolIndex>) -> Vec<CallSite>` searches all files for calls to the given function and returns locations.

**CI Templates**
- `.github/actions/patch‑ts/action.yml` uses the `patch‑ts` binary installed in the runner. It runs `patch‑ts balance --files "**/*.rs" --json` and uses `github‑script` to post a PR comment if errors are found.
- `gitlab‑ci/patch‑ts‑validation.yml` contains a job definition that can be included in a `.gitlab‑ci.yml`.

**Adaptive Tuning (`history_adaptive.rs`)**
- Reads `.patch‑ts/history.jsonl`, computes a recommended confidence threshold by finding the point that maximizes F‑score of past successes, and outputs it.

### 4.3 Data Model

- `CompilerError { error_code, file, line, column, message, language }`
- `PatchSuggestion { old, new, file, line, confidence, explanation }`
- `CallSite { file, line, column, func_name }`

## 5. Architecture Decision Records (ADRs)

### ADR‑034: Use regex for compiler error parsing

**Context:** Compiler errors have well‑known formats but vary across compilers.  
**Decision:** Use `regex` patterns per compiler; fall back to generic parser if pattern fails.  
**Alternatives:** Use tree‑sitter to parse error messages (over‑engineered).  
**Consequences:** Brittle if compiler formats change; mitigated by test corpus.

### ADR‑035: Use `git2` for all Git operations

**Context:** Need to extract diffs and diff branches without shelling out.  
**Decision:** Use `git2` crate (already a dependency).  
**Alternatives:** Shell out to `git` command (less portable).  
**Consequences:** Requires libgit2; already shipped; stable.

### ADR‑036: Cache symbol index per project

**Context:** Cross‑file analysis requires scanning all files; too slow to re‑parse every time.  
**Decision:** Cache `SymbolIndex` per file in `.patch‑ts/cache/` with file hash as key.  
**Alternatives:** Build on‑the‑fly (slow).  
**Consequences:** Cache invalidation needed on file change; simple file‑hash check.

## 6. API & Interface Contracts

CLI extended with new subcommands. MCP server exposes `fix` and `git` tools. JSON schema updated with new fields.

## 7. Cross‑cutting Concerns

- **Error Handling**: Compiler error parsing failures are non‑fatal; user is prompted to provide a manual patch.
- **Testing**: Integration tests with real compiler outputs (stored in `tests/fixtures/compiler_errors/`).
- **Performance**: Symbol index is cached; cross‑file analysis runs only when `--cross‑file` is passed.

## 8. Alternatives Considered

| Alternative | Why Rejected |
|-------------|--------------|
| Use LSP for cross‑file analysis | Too heavy; tree‑sitter is sufficient. |
| Use `git` CLI instead of `git2` | Less portable; `git2` is already a dependency. |
| Full ML‑based fix suggestion | Too complex for v1.6; research‑only. |

## 9. Traceability

| ASR | ADR | Component |
|-----|-----|-----------|
| ASR‑001 | ADR‑034 | fix.rs |
| ASR‑002 | ADR‑036 | crossfile.rs |
| ASR‑003 | ADR‑035 | cli.rs (git commands) |
| ASR‑004 | – | .github/actions/ |

---

*This architecture specification provides the blueprint for implementing v1.6.0.*
```

---

## 5. Behavioral Specification & Test Verification Plan

```markdown
# Behavioral Specification & Test Verification Plan — patch‑ts v1.6.0

| Field | Value |
|-------|-------|
| Project | patch‑ts |
| Document | Behavioral Specification & Test Verification Plan |
| Version | 1.0 |
| Date | 2026‑04‑24 |
| Author | patch‑ts team, assisted by spec‑writer |
| Status | Draft |
| References | SRS v1.6.0, Architecture v1.6.0 |

## 1. Behavioral Specifications (Specification by Example)

### Feature: `patch‑ts fix`

```gherkin
Feature: Compiler‑error‑driven fix
  Scenario: Fix missing closing brace from rustc error
    Given a file "src/main.rs" with a missing '}'
    When I run `cargo build 2>&1 | patch‑ts fix --apply`
    Then the missing '}' is inserted at the correct location
    And the file compiles successfully
    And the JSON output includes `"confidence"` > 0.8

  Scenario: Handle unrecognized error gracefully
    Given a compiler error output that is not recognized
    When I run `patch‑ts fix --error‑file unknown.txt`
    Then the tool reports "No fix pattern recognized"
    And exits with code 1
```

### Feature: Git‑Native Patching

```gherkin
Feature: Git integration
  Scenario: Apply patch from a specific commit
    Given a git repository with commit "abc123" that changed "main.rs"
    When I run `patch‑ts git apply abc123 --file main.rs`
    Then the changes from commit "abc123" are applied to "main.rs"
    And the file is validated

  Scenario: Diff against another branch
    Given a branch "feature" with changes to "main.rs"
    When I run `patch‑ts git diff main --file main.rs`
    Then the output shows a unified diff between "feature" and "main" for that file
```

### Feature: Cross‑File Analysis

```gherkin
Feature: Cross‑file semantic warnings
  Scenario: Warn when a changed function is called elsewhere
    Given a project with "lib.rs" defining `get_user` and "main.rs" calling it
    When I patch `get_user`'s signature in "lib.rs" with `--cross‑file`
    Then the JSON output includes a warning in `cross_file_warnings`
    And the warning references "main.rs" line 25

  Scenario: No warning for local‑only changes
    Given a patch that only affects a local variable
    When I apply the patch with `--cross‑file`
    Then no cross‑file warnings are emitted
```

### Feature: GitHub Action

```gherkin
Feature: GitHub Action for PR validation
  Scenario: Action runs on pull request
    Given a GitHub repository with the patch‑ts action installed
    When a PR is opened that introduces an extra brace in a Rust file
    Then the action fails the check
    And a comment is posted on the PR with the suggested fix
    And the comment includes the exact `patch‑ts` command to run
```

## 2. Test Strategy & Plan

### 2.1 Test Pyramid

| Level | Scope | Tools |
|-------|-------|-------|
| Unit | Error parser, fix suggester, cross‑file index, adaptive calculator | Rust `#[test]` |
| Integration | CLI `fix`, `git`, `adapt` commands with real files and git repos | `assert_cmd`, `tempfile`, `git2` |
| CI | GitHub Action end‑to‑end (manual or via `act`) | `act` or manual testing in a sandbox repo |
| Property | Fuzzing compiler error output with random noise | `proptest` |

### 2.2 Risk‑Based Prioritization

| Risk | Test Focus |
|------|------------|
| Fix suggester produces incorrect patches | Extensive corpus of real compiler errors. |
| Cross‑file analysis false positives | Test with known multi‑file projects. |
| Git commands corrupt index | Run in isolated temp repos. |

## 3. Test Case Specifications (Excerpt)

| TC‑ID | Requirement | Steps | Expected |
|-------|-------------|-------|----------|
| TC‑FIX‑001 | FR‑FIX‑002 | Pipe `rustc` error output to `patch‑ts fix` | Fix suggestion returned |
| TC‑GIT‑001 | FR‑GIT‑001 | Create commit, run `git apply <sha>` | Patch applied correctly |
| TC‑CROSS‑001 | FR‑CROSS‑001 | Patch `lib.rs`, run with `--cross‑file` | Warning for caller in `main.rs` |

## 4. NFR Verification

| NFR | Verification Method |
|-----|---------------------|
| NFR‑PERF‑001 | Benchmark `fix` on 50‑line to 2000‑line files. |
| NFR‑PERF‑002 | Benchmark cross‑file on a 100‑file Rust project. |
| NFR‑SEC‑001 | Review action permissions; ensure only PR comment write. |
| NFR‑COMPAT‑001 | Run v1.5.0 test suite unchanged. |
| NFR‑DOC‑001 | Review README, inline help, and CI templates. |

## 5. Requirements Traceability Matrix (RTM)

*(Table mapping Vision/BRS objectives → SRS requirements → Test cases.)*

---

*This verification plan ensures complete coverage of all v1.6.0 intelligent and integrated features.*
