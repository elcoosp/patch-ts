# Product Vision & Strategic Alignment

| Field | Value |
|-------|-------|
| Project | `patch-ts` |
| Document | Vision & Strategic Alignment |
| Version | 1.0 |
| Date | 2026-04-19 |
| Author | AI-assisted (spec-writer) |
| Status | Draft |

---

## 1. Vision Statement

> To make AI‑assisted code repair safe and iterative—where every patch is verified by the language itself, and failures become actionable feedback instead of silent corruption.

---

## 2. Elevator Pitch

> **For** developers who use LLMs to fix compiler errors and apply patches,
> **who are** frustrated by brittle `sed` commands that fail silently or corrupt syntax,
> **our product is a CLI tool** that applies line‑based edits with Tree‑sitter validation and structural repair.
> **Unlike** raw `sed` or manual copy‑pasting,
> **our product** verifies expected content, checks syntax after the edit, and explains failures with AST‑level context—enabling a tight feedback loop between human, LLM, and code.

---

## 3. Problem Statement & Business Context

### 3.1 The Current Workflow Is Brittle

Developers increasingly use LLMs (via web chat or IDE) to generate fixes for compiler errors. The LLM outputs a command—often `sed`—using line numbers from the error message. The developer copies and runs the command.

**The failure modes are numerous:**

- Line numbers have shifted since the error was produced (the developer edited the file).
- The expected content no longer matches (whitespace, different code).
- The `sed` regex is incorrectly escaped.
- The patch introduces a new syntax error (e.g., unbalanced braces) that goes undetected.
- The developer must manually re‑run the compiler, copy the new error, and paste it back to the LLM—a slow, high‑friction loop.

### 3.2 Why Now?

- **AI coding assistants are mainstream.** Developers are already pasting compiler errors into ChatGPT, Claude, etc.
- **Tooling for the feedback loop is missing.** There is no standard, safe way to apply LLM‑generated line‑based patches.
- **Tree‑sitter is mature and fast.** It provides incremental, error‑tolerant parsing ideal for validation and repair.

---

## 4. Target Users & Customers

### Primary Users

| User Class | Description |
|------------|-------------|
| **Developer using AI assistance** | Writes code, gets compiler errors, asks an LLM for a fix, runs a command, compiles again. Wants safety and speed. |
| **AI agent / LLM** (indirect) | The tool is designed for the LLM to output commands in a predictable format. The LLM benefits from rich diagnostics when patches fail. |

### Explicitly NOT Targeted (Non‑goals)

- **Non‑Rust languages** (initial release).  
- **IDE plugin integration** (CLI only, at least initially).  
- **Automatic patch generation** (the LLM generates the patch; this tool applies it safely).  
- **Replacing `rust-analyzer` or `cargo fix`** (this is for applying LLM‑generated fixes, not for automated refactoring).

---

## 5. User Needs & Value Proposition

### Top User Needs

1. **I want to apply an LLM‑suggested patch without corrupting my file.**  
   → `patch-ts` verifies the target content matches expectation before changing anything.

2. **I want to know *why* a patch failed, so I can give better context to the LLM.**  
   → Diagnostic output includes the actual line content and AST context.

3. **When a bad patch leaves my code with unbalanced braces, I want a quick way to fix it.**  
   → `patch-ts balance` detects and repairs delimiter mismatches automatically.

### Value Proposition & Differentiators

| Attribute | `patch-ts` | `sed` / manual |
|-----------|------------|----------------|
| **Pre‑change verification** | ✅ Checks expected content matches | ❌ Blind replacement |
| **Syntax validation** | ✅ Tree‑sitter AST check before write | ❌ Silent corruption |
| **Fuzz tolerance** | ✅ Searches ±N lines for expected content | ❌ Exact line only |
| **Structural repair** | ✅ `balance` command fixes braces/parens | ❌ Manual debugging |
| **LLM‑friendly diagnostics** | ✅ Source snippets and suggestions | ❌ Only compiler error |

---

## 6. Desired Outcomes & Success Metrics

### Business / Adoption Outcomes (G‑1)

| ID | Objective | Key Result / Target |
|----|-----------|---------------------|
| G‑1.1 | Become a trusted tool in the Rust AI‑assisted workflow | 1,000+ GitHub stars within 12 months of public release |
| G‑1.2 | Demonstrate value through real‑world usage | At least 3 documented case studies of teams using `patch-ts` to accelerate development |

### Product Outcomes (G‑2)

| ID | Objective | Metric |
|----|-----------|--------|
| G‑2.1 | Reduce patch‑related syntax errors | >95% of successful `patch-ts` applications result in valid AST (no new `ERROR` nodes) |
| G‑2.2 | Reduce time spent recovering from bad patches | Users report saving at least 5 minutes per patch failure that would otherwise require manual debugging |
| G‑2.3 | Provide actionable diagnostics | >90% of failed patch attempts produce a diagnostic that the user rates as "helpful" in surveys |

---

## 7. Strategic Constraints

| Constraint | Description |
|------------|-------------|
| **Language** | Rust only (initial release). Tree‑sitter‑rust grammar is stable and well‑maintained. |
| **Distribution** | CLI tool distributed via `cargo install` and pre‑built binaries (GitHub Releases). |
| **License** | MIT or Apache 2.0 (open source). |
| **Performance** | Must parse and validate a typical Rust file (<2000 lines) in under 100ms to feel instantaneous. |
| **Team** | Solo developer / small open‑source team. Tooling and maintenance overhead must be minimal. |

---

## 8. Goals and Non‑goals

### Goals (In‑Scope)

- **G‑1** Provide a CLI for line‑based patch application with content verification.
- **G‑2** Validate patched files with Tree‑sitter before writing.
- **G‑3** Support fuzzy line matching (`--fuzz N`).
- **G‑4** Provide `balance` command to fix unbalanced delimiters.
- **G‑5** Provide `explain` command to show AST context for an error line.
- **G‑6** Output LLM‑consumable diagnostics (plain text and JSON).
- **G‑7** Atomic writes with automatic `.bak` backups.

### Non‑goals (Explicitly Out of Scope for v1)

- **NG‑1** Multi‑file patches (focus on single‑file edits).
- **NG‑2** Automatic patch generation from compiler errors (LLM does that).
- **NG‑3** Integration with specific LLM providers or IDE extensions.
- **NG‑4** Support for languages other than Rust (though architecture should allow future extension).
- **NG‑5** Full refactoring capabilities (rename, extract method, etc.)—use `rust-analyzer` for that.
- **NG‑6** GUI or web interface.
- **NG‑7** Perfect recovery from any syntax error—`balance` is a best‑effort heuristic.

---

## 9. Operational Concept & High‑Level Scenarios

### Concept of Operations

A developer encounters a Rust compiler error. They copy the error output and paste it to an LLM (ChatGPT, Claude, etc.). The LLM responds with a `patch-ts` command (or a diff). The developer runs the command in their terminal.

- If the patch applies successfully, the file is updated, and the developer recompiles.
- If the patch fails (content mismatch, syntax error), `patch-ts` prints a diagnostic. The developer copies that diagnostic back to the LLM, which now has exact context to adjust its command.

### High‑Level Scenarios

1. **Successful type annotation fix (single‑line replace)**  
   - Compiler error points to line 234.  
   - LLM outputs: `patch-ts --file src/state.rs --line 234 <<'EOF' ...`  
   - `patch-ts` verifies content, applies change, validates AST, writes file.

2. **Line number shifted—fuzzy match saves the day**  
   - Developer had added a comment above, moving the target line to 236.  
   - With `--fuzz 3`, `patch-ts` finds the expected content at line 236 and applies the patch.

3. **Mismatched brace—`balance` repair**  
   - A previous patch left an extra `}`. Compiler error points to line 566.  
   - Developer runs `patch-ts explain --line 566` to understand the issue.  
   - Then runs `patch-ts balance --apply` to auto‑remove the extra brace.

4. **LLM iterates on failure**  
   - `patch-ts` fails because expected content doesn't match.  
   - Diagnostic shows actual line content.  
   - Developer pastes diagnostic to LLM; LLM outputs corrected command with updated expected block.

---

## 10. Stakeholders, Sponsorship, and Governance

| Role | Responsibility |
|------|----------------|
| **Project Maintainer** | Owns vision, roadmap, and technical direction. Approves major changes. |
| **Contributors** | Submit PRs for features and bug fixes. |
| **Early Adopters** | Provide feedback on usability and diagnostic quality. |
| **LLM Ecosystem** (indirect) | The tool's command format should be easy for LLMs to generate reliably. |

### Decision‑Making

- The project maintainer has final say on scope and prioritization.
- Feature requests that align with the vision and non‑goals will be considered.
- Breaking changes to the CLI interface will follow semantic versioning and be clearly communicated.

---

## 11. Traceability & Alignment Notes

This document establishes the following traceability anchors for downstream requirements:

| ID Prefix | Scope | Example |
|-----------|-------|---------|
| `G‑` | Business / Product Goal | `G‑2.1` |
| `NG‑` | Non‑goal (explicit exclusion) | `NG‑1` |

Downstream documents (BRS, SRS, Architecture, Test) will reference these IDs to maintain alignment.

---

## 12. Risks, Assumptions, and Open Questions

### Top Assumptions

1. **LLMs can reliably generate `patch-ts` commands** given examples and a system prompt.  
2. **Tree‑sitter‑rust parsing is fast enough** for interactive use (target <100ms for typical files).  
3. **Developers will adopt a new CLI tool** if it demonstrably saves time and prevents frustration.

### Major Risks & Mitigations

| Risk | Likelihood | Impact | Mitigation |
|------|------------|--------|------------|
| LLM outputs malformed `patch-ts` commands | Medium | High | Provide clear examples and error messages; consider a "validate‑command" dry‑run mode. |
| Tree‑sitter fails to parse some valid Rust (edge cases) | Low | Medium | Fall back to line‑based verification without AST check; log errors for later fixes. |
| Adoption friction (yet another tool) | Medium | Medium | Focus on clear value proposition; provide one‑liner install; integrate with common workflows (e.g., shell aliases). |
| Performance bottlenecks with large files | Low | Low | Tree‑sitter is incremental and fast; profile early. |

### Open Questions

1. **What is the best default fuzz radius?** (Propose `--fuzz 5` based on typical drift; gather feedback.)  
2. **Should `balance` operate on the whole file or only a specified function?** (Initial: whole file, with `--function` option later.)  
3. **How to handle patches that span multiple files?** (Defer to v2; consider `patch-ts workspace` command.)

---

*This vision document serves as the strategic anchor for `patch-ts`. It will be referenced by all subsequent specifications to ensure alignment.*
