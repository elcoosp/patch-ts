# Software Requirements Specification

| Field | Value |
|-------|-------|
| Project | `patch-ts` |
| Document | Software Requirements Specification (SRS) |
| Version | 1.0 |
| Date | 2026-04-19 |
| Author | AI-assisted (spec-writer) |
| Status | Draft — Pending Review |

---

## 1. Introduction and Scope

### 1.1 Purpose

This Software Requirements Specification (SRS) defines the functional and non‑functional requirements for `patch-ts` version 1.0. It describes what the software must do, how it must perform, and the constraints under which it must operate. This document serves as the authoritative behavioral contract for implementation and testing.

### 1.2 Scope

`patch-ts` is a command‑line tool that:

- Accepts line‑based patch commands (replace, delete, insert) with expected‑content verification.
- Applies unified diffs with fuzzy context matching.
- Validates patched Rust source files using Tree‑sitter syntax parsing.
- Provides structural repair commands (`balance`, `explain`).
- Outputs human‑readable and machine‑readable diagnostics.

**Out of Scope (this release):**

- Multi‑file or workspace‑wide patch operations.
- Support for languages other than Rust.
- Graphical user interface or IDE integration.
- Automatic patch generation from compiler errors.

### 1.3 Document Conventions

- **"Shall"** indicates a mandatory requirement.
- **"Should"** indicates a recommended but not mandatory requirement.
- **"May"** indicates an optional feature.

Requirements are uniquely identified with prefixes:
- `REQ-FUNC-###` – Functional requirements.
- `REQ-NFR-###` – Non‑functional requirements.
- `REQ-IF-###` – Interface requirements.
- `REQ-CONST-###` – Constraints.

### 1.4 References

- **Vision Document:** `patch-ts-vision.md` v1.0
- **Business & Stakeholder Requirements Specification:** `patch-ts-brs.md` v1.0
- **ISO/IEC/IEEE 29148:2018** – Systems and software engineering — Requirements engineering
- **IEEE 830‑1998** – Recommended Practice for Software Requirements Specifications

---

## 2. System Context and Overview

### 2.1 System Boundary and Context

`patch-ts` operates as a standalone CLI executable invoked by a user or script. It interacts with:

- **File System:** Reads source files, writes modified files, creates backup files.
- **Standard Input/Output:** Receives patch content (heredoc, diff), emits diagnostics and progress information.
- **Standard Error:** Emits error messages and structured diagnostic data.

It does **not** directly interact with the Rust compiler or any language server; it uses the `tree-sitter-rust` grammar library for parsing.

### 2.2 High‑Level Capabilities

| Capability | Description |
|------------|-------------|
| **Line‑Based Patch** | Replace, delete, or insert lines with expected‑content verification. |
| **Unified Diff Application** | Apply patches in `git diff` format with fuzzy matching. |
| **AST Validation** | Check that patched code parses without new syntax errors. |
| **Fuzzy Line Matching** | Locate expected content within a tolerance of line numbers. |
| **Structural Repair** | Automatically fix unbalanced delimiters; explain syntax errors. |
| **Dry‑Run & Backup** | Preview changes and preserve original files. |

### 2.3 Actors

| Actor | Description |
|-------|-------------|
| **Human User** | Developer invoking `patch-ts` directly from a terminal. |
| **Automation Script** | Shell script or CI pipeline invoking `patch-ts` non‑interactively. |
| **LLM / AI Agent** | (Indirect) Generates `patch-ts` commands; consumes diagnostic output. |

---

## 3. Functional Capabilities and Behavior

Functional requirements are grouped by capability. Each requirement includes a unique ID, a statement using EARS syntax where applicable, priority (MoSCoW), and traceability to stakeholder needs (SN‑xxx from BRS).

### 3.1 Command‑Line Interface

**REQ-FUNC-001 – Subcommand Structure**  
**Priority:** Must  
**Traceability:** SN‑001, SN‑002, SN‑009, SN‑010  

The system shall support the following subcommands:

- `patch` – Apply line‑based patches or unified diffs (default if no subcommand).
- `balance` – Detect and fix unbalanced delimiters.
- `explain` – Provide AST‑based explanation of a syntax error at a given line.

Each subcommand shall have its own set of options and arguments.

**REQ-FUNC-002 – Help and Version**  
**Priority:** Must  
**Traceability:** General usability  

The system shall provide `--help` and `--version` flags that display usage information and version number respectively. Help output shall include examples for each subcommand.

### 3.2 Line‑Based Patch Operations

**REQ-FUNC-010 – Replace Block (Heredoc)**  
**Priority:** Must  
**Traceability:** SN‑001  

When the user invokes `patch-ts --file <PATH> --line <N> [--fuzz <R>]` with a heredoc containing `<<<` (expected) and `---` (new) blocks, the system shall:

1. Read the file at `<PATH>`.
2. Attempt to locate the expected block starting at line `<N>`.
3. If `--fuzz <R>` is provided and the expected block is not found at line `<N>`, search within ±`<R>` lines for a match.
4. If the expected block is found, replace it with the new block.
5. If the expected block is not found, abort and emit a diagnostic.

**REQ-FUNC-011 – Replace Block (Arguments)**  
**Priority:** Should  
**Traceability:** SN‑001  

The system shall support an alternative syntax using `--old <STRING>` and `--new <STRING>` arguments for single‑line replacements, without requiring a heredoc.

**REQ-FUNC-012 – Delete Line**  
**Priority:** Must  
**Traceability:** SN‑002  

When the user invokes `patch-ts --file <PATH> --delete <N> --expect <STRING>`, the system shall:

1. Read the file and examine line `<N>`.
2. Verify that the trimmed content of line `<N>` equals `<STRING>`.
3. If it matches, delete line `<N>`.
4. If it does not match, abort and emit a diagnostic showing actual vs expected.

**REQ-FUNC-013 – Insert Lines**  
**Priority:** Should  
**Traceability:** SN‑003  

When the user invokes `patch-ts --file <PATH> --after <N> --content <STRING>` (or heredoc for multi‑line), the system shall insert the provided content as new lines immediately following line `<N>`.

### 3.3 Unified Diff Application

**REQ-FUNC-020 – Apply Unified Diff**  
**Priority:** Could  
**Traceability:** SN‑004  

When the user invokes `patch-ts --diff` with a unified diff supplied via heredoc or file, the system shall:

1. Parse the diff and attempt to apply each hunk to the target file.
2. Use fuzzy matching (similarity‑based) to locate hunk contexts if exact line numbers do not match.
3. If any hunk fails to apply, abort all changes and emit a diagnostic indicating which hunk failed and why.

### 3.4 AST Validation

**REQ-FUNC-030 – Syntax Validation Before Write**  
**Priority:** Must  
**Traceability:** SN‑006  

Before writing any modified file to disk, the system shall parse the proposed content using Tree‑sitter for Rust. If the parse tree contains new `ERROR` nodes that were not present in the original file (or if the original had no `ERROR` nodes and the new one does), the system shall abort the patch and emit a diagnostic unless the `--force` flag is provided.

**REQ-FUNC-031 – Force Flag Override**  
**Priority:** Should  
**Traceability:** SN‑007  

When the `--force` flag is provided, the system shall skip AST validation and write the file regardless of syntax errors.

### 3.5 Dry‑Run and Backup

**REQ-FUNC-040 – Dry‑Run Mode**  
**Priority:** Should  
**Traceability:** SN‑008  

When the `--dry-run` flag is provided, the system shall perform all verification steps but shall not modify the file. It shall output a unified diff of the changes that would be made.

**REQ-FUNC-041 – Automatic Backup**  
**Priority:** Must  
**Traceability:** SN‑013  

By default, before modifying a file, the system shall create a backup copy with the extension `.bak` (e.g., `file.rs.bak`). If a backup file already exists, it shall be overwritten. The `--no-backup` flag shall disable this behavior.

### 3.6 Structural Repair Commands

**REQ-FUNC-050 – Balance Delimiters**  
**Priority:** Must  
**Traceability:** SN‑009  

When the user invokes `patch-ts balance --file <PATH>`, the system shall:

1. Parse the file with Tree‑sitter.
2. Identify unbalanced delimiters (braces `{}`, parentheses `()`, brackets `[]`).
3. Propose a minimal fix (delete extra delimiters, insert missing ones) that results in a valid AST.
4. If `--apply` is provided, apply the fix; otherwise, output the proposed change as a diff (dry‑run default).

**REQ-FUNC-051 – Balance Specific Function**  
**Priority:** Could  
**Traceability:** SN‑009 (extended)  

When the `--function <NAME>` option is provided with `balance`, the system shall restrict analysis and repair to the body of the named function.

**REQ-FUNC-052 – Explain Syntax Error**  
**Priority:** Should  
**Traceability:** SN‑010  

When the user invokes `patch-ts explain --file <PATH> --line <N>`, the system shall:

1. Parse the file with Tree‑sitter.
2. Locate the AST node containing or nearest to line `<N>`.
3. If an `ERROR` node is present, provide a diagnostic describing the likely cause (e.g., extra closing brace, missing delimiter) and the surrounding context.
4. If no error is present at that line, state that the line parses successfully.

### 3.7 Diagnostic Output

**REQ-FUNC-060 – Human‑Readable Diagnostic Format**  
**Priority:** Must  
**Traceability:** SN‑011  

When a patch fails or an error occurs, the system shall output a diagnostic to stderr that includes:

- The file path and line number involved.
- The expected content vs. actual content (if applicable).
- A snippet of the source code with context (±3 lines).
- A suggested action (e.g., "Try using --fuzz 5 to search nearby lines").

**REQ-FUNC-061 – JSON Diagnostic Format**  
**Priority:** Could  
**Traceability:** SN‑012  

When the `--json` flag is provided, the system shall output diagnostic information as a single JSON object to stdout, containing structured fields (`error_code`, `message`, `file`, `line`, `column`, `context`, `suggestion`). Successful operations shall output `{"success": true}`.

**REQ-FUNC-062 – Exit Codes**  
**Priority:** Should  
**Traceability:** SN‑014  

The system shall exit with a non‑zero exit code when a patch fails, an error occurs, or validation fails. Exit code `0` shall indicate success. Specific exit codes shall be defined for common failure modes (e.g., content mismatch, syntax error, file not found).

---

## 4. Quality and Non‑functional Requirements

Non‑functional requirements are organized by ISO/IEC 25010:2023 quality characteristics. Each includes a measurable fit criterion.

### 4.1 Performance Efficiency

**REQ-NFR-001 – Patch Application Latency**  
**Priority:** Must  
**Fit Criterion:** For a Rust source file of up to 2,000 lines, the `patch` command (including file read, content verification, AST validation, and write) shall complete in ≤100 milliseconds on a typical developer machine (as defined in test environment specification).  
**Traceability:** CQ‑001

**REQ-NFR-002 – Balance Command Latency**  
**Priority:** Should  
**Fit Criterion:** For a file of up to 2,000 lines, the `balance` command shall complete in ≤200 milliseconds.  
**Traceability:** CQ‑001

### 4.2 Reliability

**REQ-NFR-010 – Data Integrity**  
**Priority:** Must  
**Fit Criterion:** The system shall never leave a file in a partially written state. File writes shall be atomic (write to temp file, then rename). Verification shall be performed via integration tests.  
**Traceability:** CQ‑002

**REQ-NFR-011 – Backup Creation Reliability**  
**Priority:** Must  
**Fit Criterion:** When backup is enabled, the system shall successfully create a `.bak` file with identical content to the original before modification, in 100% of test cases.  
**Traceability:** SN‑013

### 4.3 Security

**REQ-NFR-020 – No Execution of Patched Code**  
**Priority:** Must  
**Fit Criterion:** The system shall not execute, compile, or interpret the patched source code. It shall only read, parse (AST), and write text files. Verified by design review.  
**Traceability:** General safety

**REQ-NFR-021 – Secure File Handling**  
**Priority:** Should  
**Fit Criterion:** Temporary files shall be created with restrictive permissions (owner read/write only). File renames shall not follow symlinks. Verified by code inspection and test cases.

### 4.4 Compatibility

**REQ-NFR-030 – Operating System Support**  
**Priority:** Should  
**Fit Criterion:** The tool shall compile and run correctly on Linux (x86_64, aarch64), macOS (x86_64, aarch64), and Windows (x86_64, MSVC). Test suite shall pass on all target platforms.  
**Traceability:** CQ‑005

**REQ-NFR-031 – Rust Edition Support**  
**Priority:** Must  
**Fit Criterion:** The Tree‑sitter grammar shall correctly parse valid Rust code from Edition 2015, 2018, 2021, and 2024. Test suite shall include samples from each edition.

### 4.5 Usability (Interaction Capability)

**REQ-NFR-040 – Command Predictability**  
**Priority:** Must  
**Fit Criterion:** At least 90% of test participants shall be able to construct a correct `patch-ts` command after reading the `--help` output and one example.  
**Traceability:** CQ‑004

**REQ-NFR-041 – Diagnostic Clarity**  
**Priority:** Must  
**Fit Criterion:** In user testing, at least 80% of participants shall rate diagnostic messages as "clear and actionable" on a Likert scale survey.  
**Traceability:** CQ‑003

### 4.6 Maintainability

**REQ-NFR-050 – Code Modularity**  
**Priority:** Should  
**Fit Criterion:** The codebase shall be organized into distinct modules (CLI, patch engine, AST engine, diagnostics, file I/O) with clear interfaces. Cyclomatic complexity per function shall be ≤10 as measured by `clippy`.  
**Traceability:** Contributor experience

**REQ-NFR-051 – Test Coverage**  
**Priority:** Should  
**Fit Criterion:** Unit test coverage shall be ≥80% as measured by `tarpaulin` or equivalent. Integration tests shall cover all major workflows.

### 4.7 Flexibility (Portability)

**REQ-NFR-060 – Language Extensibility**  
**Priority:** Could  
**Fit Criterion:** The architecture shall separate language‑specific AST logic to allow future addition of other Tree‑sitter grammars without rewriting core patch logic. Verified by design review.  
**Traceability:** Future roadmap (NG‑4 from Vision)

---

## 5. External Interfaces and Data Contracts

### 5.1 User Interface (CLI)

**REQ-IF-001 – Command Syntax**  
The system shall follow standard POSIX command‑line conventions:

- Options shall use `--long-opt` and `-s` short forms where appropriate.
- Positional arguments shall be clearly documented.
- Heredoc input shall be accepted from stdin.

**REQ-IF-002 – Input Formats**

| Format | Description | Example |
|--------|-------------|---------|
| Heredoc replace | `<<'EOF'` followed by `<<<` expected block, `---` separator, new block, `EOF` | See REQ‑FUNC‑010 |
| Unified diff | Standard `git diff` format with `---` and `+++` headers and `@@` hunks | `patch-ts --diff <<'EOF' ...` |
| Inline strings | `--old "..." --new "..."` | For single‑line replacements |

**REQ-IF-003 – Output Formats**

| Format | Trigger | Destination | Schema |
|--------|---------|-------------|--------|
| Human‑readable diagnostic | Default (stderr) | stderr | Plain text with ANSI colors (optional) |
| JSON diagnostic | `--json` | stdout | `{ "success": bool, "error": { ... } }` |
| Dry‑run diff | `--dry-run` | stdout | Unified diff format |

### 5.2 File System Interface

**REQ-IF-010 – File Reading**  
The system shall read source files as UTF‑8 text. If a file contains invalid UTF‑8 sequences, the system shall abort with an appropriate error message.

**REQ-IF-011 – File Writing**  
The system shall write files atomically: write patched content to a temporary file in the same directory, then rename the temporary file over the original. File permissions shall be preserved.

**REQ-IF-012 – Backup File Naming**  
Backup files shall be created by appending `.bak` to the original filename. If `file.rs.bak` already exists, it shall be overwritten.

### 5.3 Tree‑sitter Grammar Interface

**REQ-IF-020 – Grammar Dependency**  
The system shall embed the `tree-sitter-rust` grammar at a specific, versioned commit. The grammar version shall be recorded in the `--version` output.

---

## 6. Constraints, Assumptions, and Dependencies

### 6.1 Constraints

| ID | Constraint | Rationale |
|----|------------|-----------|
| **REQ-CONST-001** | Implementation language: Rust (Edition 2021). | Ecosystem compatibility, performance, safety. |
| **REQ-CONST-002** | Must be distributable via `cargo install`. | Standard Rust distribution channel. |
| **REQ-CONST-003** | Tree‑sitter parsing must be synchronous (no async runtime). | Keep complexity low; files are small. |
| **REQ-CONST-004** | No network access required. | Tool operates entirely offline. |

### 6.2 Assumptions

| ID | Assumption |
|----|------------|
| **AS‑001** | The `tree-sitter-rust` grammar accurately parses all valid Rust syntax. |
| **AS‑002** | Target files are encoded in UTF‑8. |
| **AS‑003** | Users have write permission to the target file's directory. |

### 6.3 Dependencies

| Dependency | Version | Purpose |
|------------|---------|---------|
| `clap` | 4.x | CLI argument parsing |
| `tree-sitter` | 0.22+ | Incremental parsing framework |
| `tree-sitter-rust` | Latest | Rust grammar |
| `line-index` | 0.1.x | Line/column ↔ byte offset conversion |
| `flickzeug` (or `diffy`) | 0.5+ | Unified diff parsing and fuzzy application |
| `tempfile` | 3.x | Atomic file writes |
| `miette` | 7.x | Diagnostic formatting |
| `serde` / `serde_json` | 1.x | JSON output |
| `strsim` | 0.11+ | String similarity for fuzzy matching |

---

## 7. TBD Log

The following items are intentionally left as placeholders to be resolved during design and implementation.

| ID | Description | Owner | Due |
|----|-------------|-------|-----|
| **TBD‑001** | Exact exit code values for each failure mode. | Maintainer | Pre‑release |
| **TBD‑002** | Default fuzz radius for `--fuzz` when not specified (proposed: 5). | Maintainer | v1 design |
| **TBD‑003** | Similarity threshold for fuzzy diff hunk application (proposed: 0.9). | Maintainer | v1 design |
| **TBD‑004** | ANSI color support detection and `--color` flag behavior. | Maintainer | v1 design |

---

## 8. Requirements Attributes and Traceability Model

### 8.1 Requirement Attributes

Each requirement shall be maintained with the following attributes in the requirements management tool (or source markdown):

| Attribute | Description |
|-----------|-------------|
| ID | Unique identifier (e.g., `REQ-FUNC-010`) |
| Type | Functional / NFR / Interface / Constraint |
| Priority | Must / Should / Could (MoSCoW) |
| Status | Proposed / Approved / Implemented / Verified |
| Source | Stakeholder Need ID (e.g., SN‑001) or derived |
| Rationale | Why this requirement exists |
| Verification Method | Inspection / Analysis / Demonstration / Test |

### 8.2 Traceability Matrix (Excerpt)

| Stakeholder Need (BRS) | System Requirement (SRS) |
|------------------------|-------------------------|
| SN‑001 | REQ-FUNC-010, REQ-FUNC-011 |
| SN‑002 | REQ-FUNC-012 |
| SN‑003 | REQ-FUNC-013 |
| SN‑004 | REQ-FUNC-020 |
| SN‑005 | REQ-FUNC-010 (fuzz option) |
| SN‑006 | REQ-FUNC-030 |
| SN‑007 | REQ-FUNC-031 |
| SN‑008 | REQ-FUNC-040 |
| SN‑009 | REQ-FUNC-050, REQ-FUNC-051 |
| SN‑010 | REQ-FUNC-052 |
| SN‑011 | REQ-FUNC-060 |
| SN‑012 | REQ-FUNC-061 |
| SN‑013 | REQ-FUNC-041 |
| SN‑014 | REQ-FUNC-062 |
| CQ‑001 | REQ-NFR-001, REQ-NFR-002 |
| CQ‑002 | REQ-NFR-010 |
| CQ‑003 | REQ-NFR-041 |
| CQ‑004 | REQ-NFR-040 |
| CQ‑005 | REQ-NFR-030 |

---

## 9. Appendices

### 9.1 Glossary

*(Inherited from BRS Section 6. Additional SRS‑specific terms below.)*

| Term | Definition |
|------|------------|
| **Hunk** | A section of a unified diff describing changes to a contiguous block of lines. |
| **AST** | Abstract Syntax Tree – a structured representation of source code. |
| **ERROR node** | A special node in a Tree‑sitter parse tree indicating a syntax error. |

### 9.2 EARS Pattern Reference

This SRS uses the following EARS patterns for functional requirements:

- **Event‑driven:** "When `<trigger>`, the system shall `<response>`."
- **Unwanted behavior:** "If `<undesired event>`, then the system shall `<response>`."

---

*This SRS defines the complete behavioral contract for `patch-ts` v1.0. It will serve as the foundation for architecture design, implementation, and test planning.*
