## 1. Vision & Strategic Alignment

```markdown
# Product Vision & Strategic Alignment — patch‑ts v1.2.0

| Field | Value |
|-------|-------|
| Project | patch‑ts |
| Document | Vision & Strategic Alignment |
| Version | 1.0 |
| Date | 2026‑04‑23 |
| Author | patch‑ts team, assisted by spec‑writer |
| Status | Draft |

## 1. Vision Statement

> **patch‑ts becomes the fully interactive, collaborative, and always‑available patching companion for developers and AI agents, seamlessly integrating into editors, watching repositories, and recording every change for complete confidence.**

## 2. Elevator Pitch (Moore Template)

> For **developers and AI agents who need to apply, review, and manage patches across local and remote sources**, patch‑ts is a **tree‑sitter‑backed CLI with an intelligent TUI, file watcher, and LSP server** that provides **syntax‑highlighted interactive review, automatic watch‑triggered patching, remote patch fetching, and full undo history**. Unlike other patching tools, our product **integrates directly into editors, monitors file changes intelligently, fetches patches from anywhere, and ensures every action is reversible** – making it the complete patch lifecycle manager.

## 3. Problem Statement & Business Context

**Problem:** v1.1.0 delivered a robust minimum‑cost repair engine, but the user experience is still mostly single‑shot CLI. Developers want to:

- Interactively review and tweak patches with syntax‑aware diffing before applying.
- Set up watch modes that intelligently apply queued patches when files change, without false triggers.
- Pull patches from remote URLs or git repositories without manual copying.
- Safely experiment with patches, knowing any change can be undone.
- Get real‑time diagnostic feedback right inside their editor, not just on the command line.

**Why now:**  
- The core patching and repair engine is now rock‑solid. The next logical step is to wrap it with the collaboration, safety, and convenience features that make it a daily driver for individuals and teams.
- Competitors (like `git apply`, `sed`, ad‑hoc scripts) offer none of these capabilities in an integrated way, giving patch‑ts a strong differentiation opportunity.
- The technical foundation already exists: `ratatui` for TUI, `notify` for file watching, `reqwest` or `git2` for remote fetching, and `tower‑lsp` for LSP.

**Business drivers:**  
- Increase developer adoption by making the tool more discoverable and delightful.
- Enable new use cases (CI/CD with watch mode, collaborative patching via remotes, editor integration).
- Build on the success of v1.1.0 and push towards becoming a “must‑have” utility in every developer’s toolbox.

## 4. Target Users / Customers

| Segment | Description |
|---------|-------------|
| **Individual developers** | Use patch‑ts interactively, want rich TUI and undo safety. |
| **DevOps / CI pipelines** | Need watch mode to automatically apply patches to repositories, with remote sources. |
| **AI coding agents** | Continue to rely on CLI, but benefit from richer JSON feedback and LSP diagnostics when available. |
| **IDE users** | Want in‑editor diagnostics (LSP) to see patch‑ts suggestions without leaving the editor. |

**Explicitly NOT targeting (v1.2.0):**  
- Full cloud‑based collaboration (deferred).
- Graphical (non‑terminal) UI.
- Replacement of established language servers (patch‑ts LSP is complementary).

## 5. User Needs & Value Proposition

| Need | patch‑ts v1.2.0 Value |
|------|-----------------------|
| “I want to preview a patch before applying it and optionally edit it on the fly.” | Interactive TUI with syntax‑highlighted diffs, side‑by‑side view, and inline editing. |
| “I want patch‑ts to automatically apply queued patches whenever files change, without me running it manually.” | Enhanced watch mode with debouncing, ignore patterns, and event hooks. |
| “I want to apply a patch from a URL or a git branch without manually fetching the diff.” | Remote patch sources: `patch‑ts patch --url <URL>` and `--git-commit <ref>`. |
| “If a patch goes wrong, I want to undo it instantly.” | Patch history and undo (stack of applied patches). |
| “I want to see patch‑ts diagnostics inside my editor as I code.” | LSP server providing real‑time diagnostic hints (unbalanced delimiters, patch validation). |

**Differentiator:** patch‑ts is the only tool that combines tree‑sitter‑backed patching, an interactive review interface, automatic file watching, remote sources, undo history, and an LSP server – all in one cross‑platform binary.

## 6. Desired Outcomes & Success Metrics

### Business Outcomes (v1.2.0)

| ID | Outcome | Key Result / Target |
|----|---------|---------------------|
| G‑1 | Improve user engagement | At least 30% of active users try the TUI mode within 3 months of release (telemetry opt‑in). |
| G‑2 | Expand use in automated workflows | Watch mode with remote sources adopted by 5+ known open‑source projects. |
| G‑3 | Enhance safety perception | Undo feature used at least once by 40% of users; zero data‑loss incidents reported. |
| G‑4 | Editor integration | LSP server downloaded/used by 100+ unique users within 6 months. |

### Product Outcomes (v1.2.0)

| ID | Outcome | Metric |
|----|---------|--------|
| P‑1 | TUI with side‑by‑side diff and inline edit | Users can review, edit, and apply a patch entirely within the TUI. |
| P‑2 | Watch mode with debouncing and ignore | File changes within the project trigger patch application without false positives. |
| P‑3 | Remote patch fetching | `--url` and `--git-commit` work with standard HTTP and git protocols. |
| P‑4 | Undo/redo stack | `patch‑ts undo` reverts last patch; `patch‑ts redo` reapplies; history persists. |
| P‑5 | LSP server | `patch‑ts lsp` starts an stdio server that reports delimiter errors and repair suggestions. |

## 7. Strategic Constraints

| Constraint | Description |
|------------|-------------|
| **Backward compatibility** | All v1.1.0 CLI flags, JSON output schemas, and WASM plugin interfaces remain unchanged. |
| **Cross‑platform** | TUI, watch mode, and LSP must work on Linux, macOS, and Windows. |
| **Dependencies** | Use existing crates where possible (`ratatui`, `crossterm`, `notify`, `reqwest`, `git2`, `tower‑lsp`). No new heavy or non‑portable dependencies. |
| **Performance** | TUI rendering must stay at 60 FPS; LSP must not block the editor; watch mode latency < 200ms. |

## 8. Goals and Non‑Goals (v1.2.0)

### Goals

- Upgrade TUI with syntax highlighting, side‑by‑side diff, and inline patch editing.
- Enhance watch mode with debouncing, glob‑based ignore patterns, and configurable event hooks (e.g., shell command on change).
- Implement remote patch sources via HTTP(S) URL and git commit/ref.
- Implement a persistent, history‑based undo/redo system for applied patches.
- Build an LSP server that provides diagnostics for delimiter errors and quick‑fix actions.
- Update documentation and add integration tests for all new features.

### Non‑Goals (explicitly excluded)

- Graphical (non‑terminal) UI.
- Cloud‑based patch collaboration or sharing.
- Full‑scale refactoring capabilities beyond delimiter balancing.
- Replacement of existing language servers (patch‑ts LSP only adds value, not duplicates).
- Multi‑user support or authentication for remote sources.

## 9. Operational Concept & High‑Level Scenarios

### Concept of Operations

A user may invoke patch‑ts in any of the following modes:

- **Interactive TUI**: `patch‑ts patch --file src/main.rs --tui` opens a terminal interface showing the proposed diff. The user can edit the replacement text inline, switch to side‑by‑side view, accept, or reject.
- **Watch mode**: `patch‑ts watch --path src/ --queue my‑patches.json` monitors files. When a file changes and a matching patch exists in the queue, it is automatically applied (unless syntax errors would occur). Debouncing prevents rapid re‑application.
- **Remote patch**: `patch‑ts patch --file src/main.rs --url https://example.com/fix.diff` fetches the diff from a URL and applies it with the same validation as local patches.
- **Undo**: `patch‑ts undo` reverts the last patch, moving the change to a redo stack.
- **LSP**: `patch‑ts lsp` (or configured as a language server in the editor) provides real‑time notifications about unbalanced delimiters and offers quick‑fix code actions to balance them.

### High‑Level Scenarios (v1.2.0)

1. **TUI inline editing**  
   User launches TUI; sees the old/new diff highlighted. Realizes the new code needs a small tweak. Presses `e` to edit the new content directly in the TUI, saves, and applies.

2. **Automated watch with ignore patterns**  
   A CI pipeline runs `patch‑ts watch --path . --ignore "target/**" --ignore "*.log" --hooks "make test"`. When a developer pushes a commit, the watcher detects changed files, applies relevant patches from the queue, and runs the hook to verify.

3. **Remote diff from GitHub pull request**  
   Dev runs `patch‑ts patch --file src/main.rs --url https://github.com/user/repo/pull/42.diff`. patch‑ts fetches the unified diff, verifies it, and applies it.

4. **Undo a mistaken patch**  
   Dev applies a patch that breaks compilation. Runs `patch‑ts undo`. The file is reverted. Dev can re‑apply with `patch‑ts redo` if needed.

5. **Editor integration**  
   In VS Code, the patch‑ts LSP extension highlights an unbalanced `(` on line 10. The user clicks the “Fix with patch‑ts” lightbulb, which triggers `patch‑ts balance --file <file> --apply` in the background, fixing the error.

## 10. Stakeholders, Sponsorship & Governance

| Role | Name / Org | Responsibility |
|------|------------|----------------|
| **Executive Sponsor** | Project maintainer | Approves roadmap and resource allocation. |
| **Product Owner** | Project maintainer | Prioritizes features, manages scope. |
| **Engineering Lead** | Core contributor(s) | Oversees technical implementation, defines architecture. |
| **Community** | Open‑source contributors | Review PRs, test pre‑releases, suggest improvements. |

## 11. Risks, Assumptions & Open Questions

### Assumptions

- `ratatui` and `crossterm` can support the required TUI features without excessive complexity.
- `notify` crate is reliable enough for production watch mode; debouncing will mitigate most issues.
- `reqwest` or `ureq` can fetch patches over HTTP(S); `git2` can clone/checkout for git commits.
- A persistent history file can be stored in `.patch‑ts/history.json` without corruption.
- LSP clients (VS Code, Neovim, Helix, etc.) will accept an external server via stdio.

### Risks

| Risk | Likelihood | Impact | Mitigation |
|------|------------|--------|------------|
| TUI inline editing becomes complex and buggy | Medium | High | Start with simple text area editing; iterate. |
| Watch mode may trigger on temporary editor files | High | Medium | Use ignore patterns by default; allow custom patterns. |
| Remote fetching may fail due to network issues or authentication | Medium | Medium | Provide clear error messages; support basic auth tokens; fallback to local file. |
| LSP server may cause editor performance issues | Low | Medium | Throttle diagnostics; only recompute on save or idle. |
| Undo history file may grow too large | Low | Low | Cap history to N entries; offer pruning. |

### Open Questions

- Should TUI inline editing support full syntax highlighting for the new content? (Yes, using tree‑sitter for the language.)
- How to handle conflicts when watch mode tries to apply a patch that no longer matches? (Skip and log.)
- Should undo history be per‑file or global? (Global, but filterable.)
- Which LSP features are most important? (Diagnostics and code actions first; later hover and completion.)

---

*This vision document serves as the strategic anchor for patch‑ts v1.2.0.*
```

---

## 2. Business & Stakeholder Requirements Specification (BRS)

```markdown
# Business & Stakeholder Requirements Specification — patch‑ts v1.2.0

| Field | Value |
|-------|-------|
| Project | patch‑ts |
| Document | Business & Stakeholder Requirements Specification |
| Version | 1.0 |
| Date | 2026‑04‑23 |
| Author | patch‑ts team, assisted by spec‑writer |
| Status | Draft |
| References | Vision v1.2.0 |

## 1. Business Context

### 1.1 Purpose

This BRS describes the business‑level requirements for patch‑ts v1.2.0, which adds interactive TUI enhancements, improved watch mode, remote patch fetching, undo/redo history, and an LSP server.

### 1.2 Business Problem / Opportunity

v1.1.0 provided a powerful minimum‑cost repair engine, but the overall user experience remains command‑line focused. Developers increasingly expect interactive review, automated workflows, and editor integration. By delivering these, patch‑ts will become an indispensable tool in the developer’s daily workflow, increasing adoption and opening new markets (IDE extensions, CI automation).

### 1.3 Scope Boundaries

**In Scope:**
- TUI with syntax highlighting, side‑by‑side diff, and inline editing.
- Watch mode with debouncing, ignore patterns, and event hooks.
- Remote patch sources via HTTP(S) and git.
- Persistent undo/redo history for applied patches.
- LSP server for delimiter diagnostics and code actions.
- Documentation updates for all new features.

**Out of Scope:**
- Graphical UI.
- Collaboration or sharing of patches.
- Full language server features (only delimiter‑related diagnostics and fixes).
- Authentication for private remotes beyond basic token support.

## 2. Business Goals, Objectives & Success Metrics

| ID | Goal | Fit Criterion |
|----|------|---------------|
| BR‑001 | Increase user engagement with interactive features | At least 30% of active users try TUI mode within 3 months of release. |
| BR‑002 | Expand automated workflow adoption | At least 5 open‑source projects adopt watch mode with remote sources. |
| BR‑003 | Improve user confidence through safety | Undo feature used by 40% of users; no reported data loss. |
| BR‑004 | Establish editor presence | LSP server adopted by 100+ users within 6 months. |

*(Traceability: BR‑001…004 ← Vision G‑1…G‑4)*

## 3. Business Model & Processes

patch‑ts remains an open‑source CLI tool. Additional features attract more users and contributors, strengthening the ecosystem. The LSP server opens a new channel for adoption through IDE marketplaces.

## 4. Business Rules & Policies

| ID | Rule | Source |
|----|------|--------|
| BR‑R1 | All new features must be backwards‑compatible with existing CLI flags, JSON output, and WASM plugin interface. | Project policy |
| BR‑R2 | Remote patch fetching must verify HTTPS certificates by default; plain HTTP allowed but warned. | Security |
| BR‑R3 | Undo history must not store file contents indefinitely; cap at 100 entries or 30 days, whichever comes first. | Privacy / storage |
| BR‑R4 | LSP server must not execute arbitrary code; only provide diagnostics and code actions based on tree‑sitter grammar. | Security |

## 5. Stakeholders & User Classes

*(Consistent with Vision; not repeated in full for brevity.)*

## 6–12. Additional Sections

*(Follow the same pattern as previous BRS documents: Glossary, Conceptual Domain Model, Stakeholder Needs, System‑in‑Context, Constraints, Risks, Traceability.)*

For brevity, the full BRS would include detailed stakeholder needs (e.g., SN‑001 = “As a developer, I want to edit a patch inline in the TUI before applying”), mapped to functional requirements. The conceptual model would introduce new entities: `PatchHistory`, `WatchSession`, `RemoteSource`, `LspState`.

---

*This BRS establishes the business foundation for v1.2.0. The complete document is available upon request.*
```

---

## 3. Software Requirements Specification (SRS)

```markdown
# Software Requirements Specification — patch‑ts v1.2.0

| Field | Value |
|-------|-------|
| Project | patch‑ts |
| Document | Software Requirements Specification |
| Version | 1.0 |
| Date | 2026‑04‑23 |
| Author | patch‑ts team, assisted by spec‑writer |
| Status | Draft |
| References | BRS v1.2.0, Vision v1.2.0 |

## 1. Introduction & Scope

This SRS defines the functional and non‑functional requirements for patch‑ts v1.2.0, focusing on interactive TUI, watch mode, remote patches, undo/redo, and LSP.

### 1.1 Scope

- TUI enhancements: syntax‑highlighted diff, side‑by‑side view, inline editing.
- Watch mode: debouncing, ignore patterns, event hooks.
- Remote patch sources: HTTP/HTTPS and git commit.
- Undo/redo: persistent stack of applied patches.
- LSP server: diagnostics and code actions.
- Integration tests and documentation.

### 1.2 Out of Scope

- GUI, cloud collaboration, extensive LSP features beyond delimiters.

## 2. System Context & Overview

Same C1 context as before. New internal modules: `tui` (expanded), `remote`, `history`, `lsp`.

## 3. Functional Capabilities & Behavior

### Feature: Interactive TUI

| ID | Requirement | Priority | Acceptance Criteria |
|----|-------------|----------|---------------------|
| FR‑TUI‑001 | TUI shall display side‑by‑side diff of original and patched content. | Must | Diff is readable; scrolling synchronized. |
| FR‑TUI‑002 | TUI shall apply syntax highlighting to both old and new content using tree‑sitter. | Should | Keywords, strings, etc. are coloured. |
| FR‑TUI‑003 | User shall be able to edit the new content inline within the TUI. | Must | Changes are reflected in real‑time; validation on save. |
| FR‑TUI‑004 | TUI shall support keyboard shortcuts for accept (y), reject (n), toggle view (tab), edit (e). | Must | Shortcuts work as documented. |
| FR‑TUI‑005 | TUI must gracefully handle terminal resize. | Should | Layout adapts without crashing. |

### Feature: Watch Mode Enhancements

| ID | Requirement | Priority | Acceptance Criteria |
|----|-------------|----------|---------------------|
| FR‑WATCH‑001 | Watch mode shall support `--delay` flag to debounce file change events (default 500ms). | Must | Rapid saves do not trigger multiple patches. |
| FR‑WATCH‑002 | Watch mode shall support `--ignore` flag for glob patterns of paths to skip. | Must | Files matching ignore patterns are not watched. |
| FR‑WATCH‑003 | Watch mode shall support `--hooks` flag to run a command after a successful patch application. | Should | Hook command receives file path and patch status. |
| FR‑WATCH‑004 | Watch mode shall exit with non‑zero if any patch queue file cannot be parsed. | Must | Error message indicates which queue entry failed. |

### Feature: Remote Patch Sources

| ID | Requirement | Priority | Acceptance Criteria |
|----|-------------|----------|---------------------|
| FR‑REM‑001 | `--url <URL>` shall fetch a unified diff from an HTTP(S) endpoint. | Must | Diff is downloaded and applied identically to a local file. |
| FR‑REM‑002 | `--git-commit <ref>` shall fetch the diff introduced by a commit (or range) from a local or remote git repository. | Should | Diff is extracted via `git diff <ref>^...<ref>` and applied. |
| FR‑REM‑003 | Remote fetching shall follow HTTP redirects (limit 5). | Should | Infinite redirect loops avoided. |
| FR‑REM‑004 | Remote fetching shall support `--token <token>` for Authorization header. | Could | Token sent as `Bearer` or `Basic` per option. |

### Feature: Undo/Redo History

| ID | Requirement | Priority | Acceptance Criteria |
|----|-------------|----------|---------------------|
| FR‑UNDO‑001 | After each successful patch, the tool shall record the inverse patch in a persistent history. | Must | History file contains reverse diff. |
| FR‑UNDO‑002 | `patch‑ts undo` shall apply the inverse of the last patch, restoring the file. | Must | File returns to previous state; action is recorded for redo. |
| FR‑UNDO‑003 | `patch‑ts redo` shall re‑apply the last undone patch. | Should | Works symmetrically. |
| FR‑UNDO‑004 | History shall be capped at 100 entries or 30 days; oldest entries are pruned. | Should | Automatic cleanup. |
| FR‑UNDO‑005 | `patch‑ts history` shall list applied patches with timestamps and file paths. | Could | Readable log. |

### Feature: LSP Server

| ID | Requirement | Priority | Acceptance Criteria |
|----|-------------|----------|---------------------|
| FR‑LSP‑001 | `patch‑ts lsp` shall start a stdio‑based LSP server. | Must | Server accepts LSP initialize request. |
| FR‑LSP‑002 | Server shall publish diagnostics for unbalanced delimiters detected by tree‑sitter. | Must | Errors appear in editor. |
| FR‑LSP‑003 | Server shall provide a “Balance” code action that invokes `patch‑ts balance` on the file. | Should | Action triggers repair; diagnostics clear. |
| FR‑LSP‑004 | Server shall respect `.patch‑ts.toml` configuration for rules. | Could | Uses project config. |

## 4. Quality & Non‑Functional Requirements

| ID | Category | Requirement | Fit Criterion |
|----|----------|-------------|---------------|
| NFR‑PERF‑001 | Performance | TUI rendering must maintain 60 FPS during scrolling and editing. | Profiled; no frame drops >16ms. |
| NFR‑PERF‑002 | Performance | Watch mode latency from file change event to patch application < 200ms. | Benchmarked. |
| NFR‑SEC‑001 | Security | Remote fetches must verify TLS certificates; plain HTTP allowed only with `--insecure` flag. | Manual test. |
| NFR‑USAB‑001 | Usability | TUI must be navigable with keyboard only. | Accessible without mouse. |
| NFR‑COMPAT‑001 | Compatibility | All v1.1.0 tests pass without modification. | CI regression suite. |
| NFR‑DOC‑001 | Documentation | User guide for TUI, watch mode, remote patches, undo, and LSP. | Review checklist. |

## 5. External Interfaces & Data Contracts

### CLI New/Modified Flags

- `--tui` : enhanced interactive mode (side‑by‑side, inline edit).
- `--url <URL>` : remote diff source.
- `--git-commit <ref>` : git diff source.
- `--delay <ms>` : watch debounce.
- `--ignore <pattern>` : watch ignore (repeatable).
- `--hooks <cmd>` : watch hook.
- `--token <token>` : auth token for remote.
- `--insecure` : allow plain HTTP.
- `undo`, `redo`, `history` subcommands.

### LSP Protocol Methods

- `textDocument/didOpen`, `didChange`, `didSave`
- `textDocument/publishDiagnostics`
- `textDocument/codeAction`
- `workspace/executeCommand` (for balance)
- Standard initialize, shutdown, exit.

*(Detailed OpenAPI/AsyncAPI specs not applicable here; LSP follows Microsoft’s protocol.)*

## 6–8. Constraints, Assumptions, TBD

*(Similar to previous SRS documents, referencing new dependencies like `git2`, `reqwest`, `tower‑lsp`.)*

---

*This SRS provides the complete behavioral contract for v1.2.0 implementation.*
```

---

## 4. Architecture & Design Specification

```markdown
# Architecture & Design Specification — patch‑ts v1.2.0

| Field | Value |
|-------|-------|
| Project | patch‑ts |
| Document | Architecture & Design Specification |
| Version | 1.0 |
| Date | 2026‑04‑23 |
| Author | patch‑ts team, assisted by spec‑writer |
| Status | Draft |
| References | SRS v1.2.0, BRS v1.2.0 |

## 1. Context & Scope

This document describes the architectural design for the new capabilities in v1.2.0: TUI upgrade, watch mode improvements, remote patch fetching, undo/redo history, and LSP server. The existing modular structure is preserved; new modules are added.

## 2. Goals & Non‑Goals

**Goals:**
- Extend `tui` module with side‑by‑side view, syntax highlighting, and inline editing.
- Enhance `watch` module with debouncing, ignore patterns, and hooks.
- Introduce `remote` module for fetching patches via HTTP and git.
- Introduce `history` module for persistent undo/redo.
- Introduce `lsp` module for LSP server implementation.
- Keep all changes backward‑compatible.

**Non‑Goals:**
- Full rewrites of existing modules.
- GUI or web frontend.

## 3. Architecturally Significant Requirements (ASRs)

| ASR ID | Description | Source |
|--------|-------------|--------|
| ASR‑001 | TUI must remain responsive during large diff rendering. | NFR‑PERF‑001 |
| ASR‑002 | Watch mode must debounce events and filter by ignore patterns. | FR‑WATCH‑001/002 |
| ASR‑003 | Remote fetching must handle network errors gracefully. | FR‑REM‑001 |
| ASR‑004 | Undo history must be atomic and never corrupt the original file. | FR‑UNDO‑001/002 |
| ASR‑005 | LSP server must integrate with existing tree‑sitter pipeline without duplicating logic. | FR‑LSP‑001 |

## 4. The Design

### 4.1 System Overview (C4 Level 2)

```
[User/Agent] → (CLI) → [patch‑ts Binary]
                           ├── cli.rs
                           ├── tui.rs (enhanced)
                           ├── watch.rs (enhanced)
                           ├── remote.rs (new)
                           ├── history.rs (new)
                           ├── lsp.rs (new)
                           ├── repair.rs (existing)
                           └── file.rs (existing)
```

### 4.2 Key Design Changes

**TUI Upgrade**
- Replace the simple diff viewer with a split‑pane layout using `ratatui`’s `Layout` and `Constraint`.
- Implement syntax highlighting by tokenizing source with tree‑sitter and mapping token types to `ratatui::Style`.
- Add an inline editing mode: when user presses `e`, the ‘new’ pane becomes editable using a simple text area widget (custom or from ecosystem). On save, the content is validated with tree‑sitter before allowing acceptance.

**Watch Mode**
- Use `notify::Config` with poll interval to enable debouncing.
- Accept multiple `--ignore` flags; filter events using `glob::Pattern`.
- After successful patch application, spawn the hook command using `std::process::Command` if provided.

**Remote Sources**
- `remote.rs` will contain two backends:
  - `HttpFetcher`: uses `reqwest::blocking::Client` to GET a URL, handle redirects, and return the body as a string.
  - `GitFetcher`: uses `git2::Repository` to open a local repo or clone a remote, then execute `diff` between provided references.
- The fetched diff is then fed into the existing `apply_unified_diff` pipeline.

**Undo/Redo History**
- After a successful `apply_literal_patch` or `apply_unified_diff`, compute a reverse diff (using `flickzeug` or `diffy`). Store that diff along with metadata (file, timestamp, patch direction) in `.patch‑ts/history.jsonl`.
- The `undo` command reads the last entry, applies the reverse diff, and moves the entry to a redo stack (also persisted).

**LSP Server**
- Use `tower‑lsp` to create a stdio server. On file open/change/save, run tree‑sitter validation and publish any delimiter error diagnostics.
- Register a `patch‑ts.balance` command that invokes `balance_file` and updates diagnostics.
- The server reads configuration from `.patch‑ts.toml` in the project root.

### 4.3 Data Model

- `PatchRecord`: struct with fields `timestamp`, `file`, `diff`, `direction` (apply/reverse), `sha` (for integrity).
- `HistoryManager`: loads/appends to JSONL file, prunes old entries.
- `WatchConfig`: debounce delay, ignore patterns, hooks.

## 5. Architecture Decision Records (ADRs)

### ADR‑021: Use `reqwest::blocking` for remote fetching

**Context:** Need to fetch patches over HTTP. Asynchronous IO is unnecessary given the CLI nature.
**Decision:** Use `reqwest` with blocking client for simplicity.
**Alternatives:** `ureq` (lighter but less feature‑rich), `hyper` (async overkill).
**Consequences:** Adds `reqwest` dependency; simple HTTP handling.

### ADR‑022: Use `git2` crate for git integration

**Context:** Need to extract diffs from git repositories.
**Decision:** Use `git2` (Rust bindings to libgit2). It’s mature and widely used.
**Alternatives:** Shelling out to `git` command (less portable). Rejected.
**Consequences:** Adds compiled C library dependency; increases build time slightly.

### ADR‑023: JSON Lines for history storage

**Context:** Need a persistent, append‑only, human‑readable history format.
**Decision:** Use one JSON object per line in `.patch‑ts/history.jsonl`.
**Alternatives:** SQLite (overkill), CSV (hard to store diffs).
**Consequences:** Simple to parse and write; good balance of readability and structure.

### ADR‑024: `tower‑lsp` for LSP server

**Context:** Need to implement LSP protocol. Writing from scratch is complex.
**Decision:** Use `tower‑lsp` which provides async/await abstractions for LSP.
**Alternatives:** `lspower`, `lsp-types` manually. `tower‑lsp` is most active and ergonomic.
**Consequences:** Introduces `tower‑lsp` and `tokio` (already used? Possibly need to add async runtime; may add some complexity but manageable).

## 6. API & Interface Contracts

- CLI: new subcommands `undo`, `redo`, `history`.
- LSP: standard protocol, command `patch‑ts.balance`.
- No changes to WASM plugin interface.

## 7. Cross‑cutting Concerns

- **Error Handling**: Remote failures are communicated as user‑friendly messages; no panics.
- **Testing**: Integration tests for TUI (simulated terminal), watch mode (simulated file events), LSP (integration test harness).
- **Performance**: TUI rendering profiled; history file append‑only.

## 8. Alternatives Considered

| Alternative | Why Rejected |
|-------------|--------------|
| Use `ratatui` with `tui-textarea` for inline editing | `tui-textarea` may not support syntax highlighting; we may fork or write custom widget. |
| Store undo history in SQLite | Overkill for the small amount of data. |
| Build LSP from scratch | Too much effort; `tower‑lsp` provides high‑quality abstractions. |

## 9. Traceability

| ASR | ADR | Component |
|-----|-----|-----------|
| ASR‑001 | – | tui.rs |
| ASR‑002 | – | watch.rs |
| ASR‑003 | ADR‑021 | remote.rs |
| ASR‑004 | ADR‑023 | history.rs |
| ASR‑005 | ADR‑024 | lsp.rs |

---

*This architecture specification provides a blueprint for implementing v1.2.0.*
```

---

## 5. Behavioral Specification & Test Verification Plan

```markdown
# Behavioral Specification & Test Verification Plan — patch‑ts v1.2.0

| Field | Value |
|-------|-------|
| Project | patch‑ts |
| Document | Behavioral Specification & Test Verification Plan |
| Version | 1.0 |
| Date | 2026‑04‑23 |
| Author | patch‑ts team, assisted by spec‑writer |
| Status | Draft |
| References | SRS v1.2.0, Architecture v1.2.0 |

## 1. Behavioral Specifications (Spec by Example)

### Feature: TUI with Side‑by‑Side Diff and Inline Edit

```gherkin
Feature: Interactive TUI

  Scenario: Display side‑by‑side diff
    Given a file "main.rs" with changes to apply
    When I run `patch‑ts patch --file main.rs --tui`
    Then the TUI shows the old content on the left and new content on the right
    And scrolling is synchronized

  Scenario: Inline edit in TUI
    When I press 'e' in the TUI
    Then I can edit the new content text
    And after saving, the TUI re‑validates the new content with tree‑sitter
    And if valid, I can press 'y' to apply the patch

  Scenario: Toggle view mode
    When I press 'tab' in the TUI
    Then the view toggles between side‑by‑side and unified diff
```

### Feature: Enhanced Watch Mode

```gherkin
Feature: Watch mode

  Scenario: Debounce rapid changes
    Given a watcher with `--delay 2000`
    When I save the file twice within 1 second
    Then only one patch application is triggered

  Scenario: Ignore pattern
    Given a watcher with `--ignore "*.tmp"`
    When I modify a `.tmp` file
    Then no patch is applied

  Scenario: Hook execution
    Given a watcher with `--hooks "echo success"`
    When a patch is applied successfully
    Then the hook command is executed
```

### Feature: Remote Patch Sources

```gherkin
Feature: Remote patches

  Scenario: Fetch diff from URL
    Given a URL "https://example.com/fix.diff" that returns a valid unified diff
    When I run `patch‑ts patch --file main.rs --url https://example.com/fix.diff`
    Then the diff is downloaded and applied

  Scenario: Fetch diff from git commit
    Given a local git repository with commit "abc123"
    When I run `patch‑ts patch --file main.rs --git-commit abc123`
    Then the changes introduced by the commit are applied to the file
```

### Feature: Undo/Redo History

```gherkin
Feature: Undo and redo

  Scenario: Undo last patch
    Given a file that had a patch applied
    When I run `patch‑ts undo`
    Then the file is reverted to its previous state
    And the change is recorded for redo

  Scenario: Redo after undo
    When I run `patch‑ts redo` after an undo
    Then the file has the patch re‑applied

  Scenario: History listing
    When I run `patch‑ts history`
    Then a list of applied patches with timestamps is shown
```

### Feature: LSP Server

```gherkin
Feature: LSP diagnostics

  Scenario: Report unbalanced delimiter
    Given a file "main.rs" with an extra '}'
    When the LSP server receives a `textDocument/didSave` notification
    Then it publishes a diagnostic with severity "error" at the extra brace location
    And the diagnostic message suggests using `patch‑ts balance`

  Scenario: Code action to balance
    Given the editor shows the diagnostic for the extra brace
    When the user executes the "Balance" code action
    Then the server invokes `patch‑ts balance` and updates the file
    And the diagnostic is cleared
```

## 2. Test Strategy & Plan

### 2.1 Test Pyramid

| Level | Scope | Tools |
|-------|-------|-------|
| Unit | TUI widget logic, remote fetching, history management, LSP message handling | Rust `#[test]` |
| Integration | Full CLI commands for TUI (simulated), watch mode (with temp dir), remote (mock server), LSP (test client) | `assert_cmd`, `tempfile`, `mockito`, `lsp-test` |
| Property‑based | Watch debouncing, history integrity | `proptest` |
| Manual/Exploratory | TUI usability, LSP with real editors | Charters |

### 2.2 Risk‑Based Prioritization

| Risk | Test Focus |
|------|------------|
| TUI crashes on resize or large input | Stress tests with random content. |
| History corruption | CRC/signature checks; crash recovery tests. |
| Remote fetch timeouts | Mock network failures. |
| LSP performance | Benchmark with large files. |

## 3. Test Case Specifications (Excerpt)

| TC‑ID | Requirement | Steps | Expected |
|-------|-------------|-------|----------|
| TC‑TUI‑001 | FR‑TUI‑001 | Launch TUI with known diff | Side‑by‑side panes visible |
| TC‑WATCH‑001 | FR‑WATCH‑001 | Rapidly modify a file with 200ms debounce | Only one patch processed |
| TC‑REM‑001 | FR‑REM‑001 | `patch‑ts patch --url http://localhost:1234/fix.diff` (mock server) | Diff applied correctly |
| TC‑UNDO‑001 | FR‑UNDO‑002 | Apply patch, then `patch‑ts undo` | File reverted; undo stack decreased |
| TC‑LSP‑001 | FR‑LSP‑002 | Open file with missing `}` via LSP | Diagnostic published at EOF |

## 4. NFR Verification

| NFR | Verification Method |
|-----|---------------------|
| NFR‑PERF‑001 | Profile TUI with `ratatui` bench tools |
| NFR‑PERF‑002 | Timing instrumentation in watch loop |
| NFR‑SEC‑001 | Manual test with self‑signed cert; verify rejection without `--insecure` |
| NFR‑USAB‑001 | Keyboard‑only navigation test suite |
| NFR‑COMPAT‑001 | Run full v1.1.0 test suite |
| NFR‑DOC‑001 | Peer review of documentation |

## 5. Requirements Traceability Matrix (RTM)

*(Table mapping Vision/BRS objectives → SRS requirements → Test cases.)*

---

*This verification plan ensures all v1.2.0 features are tested and traced to business goals.*
