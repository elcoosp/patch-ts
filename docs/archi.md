# `patch-ts` Architecture & Tech Stack Document

## Executive Summary

`patch-ts` is a command-line tool designed for AI agents (LLMs) to safely apply and verify line‑based source code patches, with Tree‑sitter integration for syntax validation and structural repair capabilities. This document outlines the technical architecture, crate selection rationale, and implementation strategy for the project.


## System Architecture Overview

The system follows a modular pipeline architecture with five primary stages:

```
┌─────────────────┐     ┌─────────────────┐     ┌─────────────────┐
│   CLI Layer     │ ──► │  Patch Parsing  │ ──► │  AST Validation │
│   (clap)        │     │  & Application  │     │ (tree-sitter)   │
└─────────────────┘     └─────────────────┘     └─────────────────┘
                                                         │
                                                         ▼
┌─────────────────┐     ┌─────────────────┐     ┌─────────────────┐
│  File I/O &     │ ◄── │  Structural     │ ◄── │  Diagnostics &  │
│  Backup         │     │  Repair Engine  │     │  Error Reporting│
└─────────────────┘     └─────────────────┘     └─────────────────┘
```

### Core Modules

| Module | Responsibility |
|--------|---------------|
| **CLI** | Parse arguments, dispatch to subcommands (`patch`, `balance`, `explain`) |
| **Patch Parser** | Parse heredoc blocks, unified diffs, or line‑delete/insert commands |
| **AST Engine** | Parse Rust source with Tree‑sitter; provide node queries and syntax checks |
| **Diff Engine** | Apply patches with fuzzy matching; generate unified diffs for dry‑runs |
| **Repair Engine** | Detect unbalanced delimiters and propose minimal fixes |
| **Diagnostics** | Format human‑readable and JSON‑structured error messages |
| **File Manager** | Read/write files, create backups, handle temporary buffers |


## Technology Stack: Crate Selection

### CLI Argument Parsing — `clap` (Derive API)

**Choice:** `clap` 4.x with `derive` feature

**Rationale:**
- Industry standard; over 1,000 crates depend on it
- Derive API provides declarative, self‑documenting command definitions with automatic help generation
- Subcommand support is essential for `patch`, `balance`, `explain` modes
- Rich validation features (required arguments, conflicts, value parsing) reduce boilerplate

**Alternatives considered:** `argh` (too minimal), `palc` (alpha stage, not production‑ready)

---

### AST Parsing — `tree-sitter` + `tree-sitter-rust`

**Choice:** `tree-sitter` 0.22+ with `tree-sitter-rust` grammar

**Rationale:**
- Incremental parsing: tree can be updated efficiently after edits (millisecond scale)
- Error recovery: produces `ERROR` nodes for malformed syntax, enabling repair diagnostics
- Lossless concrete syntax tree includes all whitespace and comments, essential for precise byte‑range edits
- Mature Rust bindings with stable API

**Implementation notes:**
- Include `tree-sitter-rust` via git dependency or the `tree-sitter-rust` crate
- For fuzz‑tolerant matching, walk the AST to locate nodes near target line numbers

**Alternative approach:** The `rust-sitter` crate provides higher‑level macros for defining grammars, but is designed for creating new parsers rather than consuming existing ones. Not applicable here.

---

### Diff Generation & Patch Application — `flickzeug` (or `diffy`)

**Choice:** `flickzeug` 0.5+

**Rationale:**
- Fork of `diffy` maintained by prefix.dev, used in production with thousands of real‑world patches
- **Fuzzy patch application**: applies hunks even when line numbers drift, using similarity‑based matching — exactly what we need
- Supports unified diff parsing/generation, three‑way merge, and binary content
- Myers' diff algorithm for minimal edit sequences

**Alternative:** `diffy` 0.1.x is the original; `flickzeug` adds fuzzy matching which is critical for our use case.

**Additional patch utility:** `patch-apply` crate provides forgiving unified diff parsing compatible with `git diff` output. May be used as a fallback parser.

---

### Error Reporting & Diagnostics — `miette` + `thiserror`

**Choice:** `miette` 7.x with `fancy` feature

**Rationale:**
- Purpose‑built for diagnostic reporting with source code snippets, line highlighting, and error codes
- Integrates seamlessly with `thiserror` derive macros
- Produces both human‑friendly terminal output (ANSI colors, underlines) and structured data for LLM consumption
- Customizable `ReportHandler` allows JSON output mode for agent‑friendly diagnostics

**Alternative:** `ariadne` offers similar capabilities but `miette` has broader adoption and `thiserror` integration.

**Error type definition pattern:**
```rust
#[derive(Error, Debug, Diagnostic)]
#[error("patch application failed")]
#[diagnostic(code(patch_ts::patch_failed), help("try --fuzz to adjust line tolerance"))]
struct PatchError {
    #[source_code]
    src: NamedSource,
    #[label("expected content not found here")]
    span: SourceSpan,
}
```

---

### Line/Column ↔ Byte Offset Conversion — `line-index`

**Choice:** `line-index` 0.1.x (by matklad, used in rust‑analyzer)

**Rationale:**
- Lightweight, single‑purpose crate mapping `TextSize` offsets to `(line, column)` pairs
- O(log n) conversion via binary search on precomputed line starts
- Battle‑tested in rust‑analyzer, one of the most robust Rust language tools
- Handles UTF‑8 correctly (column = character count, not byte count)

**Alternative:** `sipha-source` offers more features (source map, snippets) but `line-index` is sufficient for our needs and has zero dependencies.

---

### String Similarity for Fuzzy Matching — `strsim`

**Choice:** `strsim` 0.11+

**Rationale:**
- Implements Levenshtein, Damerau‑Levenshtein, Jaro‑Winkler, and other algorithms
- Lightweight with no dependencies
- Used internally by `flickzeug` for its fuzzy patch application; we may need direct access for custom heuristics

**Alternative:** `ruzzy` offers SIMD‑accelerated Levenshtein for ASCII, but `strsim` is more established and sufficient for our scale.

---

### Temporary Files & Safe Writes — `tempfile`

**Choice:** `tempfile` 3.x

**Rationale:**
- Creates temporary files that are automatically deleted when handles close
- Cross‑platform secure file creation with proper permissions
- Used for atomic write pattern: write patched content to temp file, validate AST, then rename over original

---

### Error Handling — `anyhow` + `eyre` (optional)

**Choice:** `anyhow` for application‑level errors; `eyre` if custom reporting needed

**Rationale:**
- `anyhow::Error` provides flexible, trait‑object‑based error handling for CLI applications
- Works with any `std::error::Error` type, including `miette` diagnostics
- `eyre` is a fork of `anyhow` with customizable error reporting, useful if we need to override default formatting

---

### Serialization — `serde` + `serde_json`

**Choice:** `serde` 1.x with `derive` feature

**Rationale:**
- Industry standard for Rust serialization; ubiquitous
- Enable JSON output mode for structured diagnostics consumable by LLM agents
- `serde_json` provides compact, readable output

---

### Regular Expressions — `regex`

**Choice:** `regex` 1.x

**Rationale:**
- May be needed for parsing compiler error messages or extracting spans from rustc output
- Linear‑time execution guarantees with finite automata
- Well‑tested and widely used

---

### Additional Utility Crates

| Crate | Purpose |
|-------|---------|
| `log` + `env_logger` | Structured logging for debugging |
| `walkdir` | Recursive file traversal (future multi‑file support) |
| `pathdiff` | Compute relative paths for diagnostics |
| `colored` | Terminal color output (if not using `miette` fancy mode) |


## Core Module Design

### 1. CLI Module (`src/cli.rs`)

Uses `clap` derive macros to define:

```rust
#[derive(Parser)]
#[command(name = "patch-ts", about = "Tree-sitter-aware patching tool for LLM agents")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Apply a patch to a file
    Patch(PatchArgs),
    /// Detect and fix unbalanced delimiters
    Balance(BalanceArgs),
    /// Explain syntax errors at a given line
    Explain(ExplainArgs),
}
```

**PatchArgs** includes:
- `--file <PATH>`
- `--line <N>` or `--diff`
- `--old` / `--new` (literal blocks) or heredoc reading from stdin
- `--fuzz <N>` (optional search radius)
- `--dry-run`
- `--force` (skip AST validation)

### 2. AST Engine (`src/ast.rs`)

Wraps `tree-sitter`:

- `parse_file(path) -> Tree`
- `node_at_line(tree, line) -> Option<Node>`
- `validate_edit(original, edited) -> Result<(), Vec<SyntaxError>>`
- `find_extra_delimiter(tree) -> Vec<Span>` (for balance command)

### 3. Patch Engine (`src/patch.rs`)

Integrates `flickzeug`:

- `apply_unified_diff(file, diff_text)`
- `apply_literal_patch(file, line, old, new, fuzz)`
- `delete_lines(file, lines, expected_content)`

Fuzzy matching fallback:
- If exact line content doesn't match, search ±N lines using `strsim` similarity
- Accept match if similarity > threshold (e.g., 0.9)

### 4. Repair Engine (`src/repair.rs`)

Algorithm for `balance` command:
1. Parse file with Tree‑sitter
2. Traverse AST; collect all `ERROR` nodes
3. For each `ERROR`, analyze token stream to detect delimiter mismatch
4. Propose minimal fix (delete extra `}`, insert missing `}`)
5. Rank proposals by resulting AST health (fewest `ERROR` nodes)

### 5. Diagnostics Module (`src/diagnostics.rs`)

- Formats errors using `miette` with source snippets
- Provides `--json` flag for structured output:

```json
{
  "success": false,
  "error": {
    "code": "E001",
    "message": "Expected content not found at line 234",
    "span": { "file": "src/state.rs", "line": 234, "column": 17 },
    "context": "...",
    "suggestion": "Use --fuzz 5 to search nearby"
  }
}
```

### 6. File Manager (`src/file.rs`)

- Atomic writes via `tempfile`: write to temp → validate → `persist()`
- Automatic `.bak` backup unless `--no-backup`
- Integration with git (optional): check if file is tracked, warn if dirty


## Data Flow: Patch Operation

```
User Input
    │
    ▼
┌─────────────────────────────────────────────────────────────┐
│ CLI: Parse arguments, read heredoc/diff from stdin          │
└─────────────────────────────────────────────────────────────┘
    │
    ▼
┌─────────────────────────────────────────────────────────────┐
│ File Manager: Read target file, convert line to byte offset │
│               using `line-index`                            │
└─────────────────────────────────────────────────────────────┘
    │
    ▼
┌─────────────────────────────────────────────────────────────┐
│ Patch Engine: Apply edit in-memory, verify expected content │
│               matches (with optional fuzz)                  │
└─────────────────────────────────────────────────────────────┘
    │
    ├── Mismatch ──► Diagnostics → Exit with error
    │
    ▼ (Match)
┌─────────────────────────────────────────────────────────────┐
│ AST Engine: Parse edited buffer, check for syntax errors    │
└─────────────────────────────────────────────────────────────┘
    │
    ├── New errors ──► Diagnostics → Abort (unless --force)
    │
    ▼ (Valid)
┌─────────────────────────────────────────────────────────────┐
│ File Manager: Write to tempfile, validate, persist, backup  │
└─────────────────────────────────────────────────────────────┘
    │
    ▼
Success + diff summary output
```


## Safety & Error Handling Philosophy

1. **Never write without validation** — all patches are applied in‑memory and AST‑checked before touching disk
2. **Atomic writes** — tempfile + rename prevents partial writes
3. **Automatic backups** — `.bak` file created by default
4. **Clear diagnostics** — failures produce actionable messages for both humans and LLMs
5. **Graceful degradation** — if Tree‑sitter parsing fails, fall back to line‑based verification


## Configuration & Extensibility

### Future Language Support

The architecture is language‑agnostic at the patch engine level; only the AST engine needs language‑specific grammars. Adding support for new languages requires:

1. Adding the corresponding `tree-sitter-<lang>` grammar crate
2. Registering language detection by file extension
3. Implementing language‑specific repair heuristics (optional)

### Configuration File

Optional `patch-ts.toml` for project‑wide defaults:
```toml
[fuzz]
default_radius = 5
similarity_threshold = 0.9

[backup]
enabled = true
directory = ".patch-ts-backups"

[languages]
rust = { formatter = "rustfmt" }
```


## Development & Build Considerations

### Minimum Supported Rust Version (MSRV)
Rust 1.75+ (stable)

### Build Dependencies
- `cc` crate — required for compiling Tree‑sitter C grammars
- `pkg-config` — optional for system Tree‑sitter libraries

### Testing Strategy
- Unit tests for patch parsing, line/byte conversion, fuzzy matching
- Integration tests with sample Rust files and known patches
- Snapshot testing for diagnostic output formatting

### CI Pipeline
- `cargo test`, `cargo clippy`, `cargo fmt --check`
- Test against multiple Rust versions (stable, beta, MSRV)


## Conclusion

This architecture provides a robust foundation for `patch-ts`, leveraging battle‑tested Rust crates for each concern. The combination of `tree-sitter` for AST awareness, `flickzeug` for fuzzy patch application, and `miette` for rich diagnostics creates a tool that is both safe for automated use and informative for LLM‑driven iteration. The modular design ensures maintainability and a clear path for future language support.
