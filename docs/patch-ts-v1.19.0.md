# patch-ts `--preserve-comments` Feature Specification

| Field | Value |
|-------|-------|
| Project | patch-ts (existing tree‑sitter‑backed patching CLI) |
| Feature | Comment‑Preserving Patch (`--preserve-comments`) |
| Version | 0.1 (Draft) |
| Date | 2026-04-27 |
| Author | Feature Team, assisted by AI |
| Status | Draft — Pending Review |

---

## 1. Vision & Strategic Alignment

**Vision:** Make every LLM‑generated patch *non‑destructive* to existing code comments, so that automated patching never silently erases documentation.

**Problem:** LLMs producing patches often omit comments that existed in the original code—doc strings, inline explanations, TODO markers. This results in patches that break the code’s documentation integrity, forcing manual review and repair.

**Goal (G‑CP‑01):** Deliver a `--preserve-comments` flag that re‑attaches all original comments to the patched output, preserving documentation, inline notes, and structural comments while keeping the patch’s intended code change.

**Non‑goals:**
- Do not modify the comment content; they are re‑inserted verbatim.
- Do not alter the semantic placement of comments (e.g., moving a function‑level doc comment to a different function).
- The feature does **not** guarantee comment preservation when the original entity (function, struct) is completely removed—such comments may be dropped (or optionally appended as orphaned comments).

**Strategic fit:** Complement patch‑ts’s existing capabilities (tree‑sitter validation, fuzzy matching, balance) to make the tool the most AI‑friendly patching solution. Builds on existing infrastructure (`Language` trait, `semdiff`, `indent`).

---

## 2. Business & Stakeholder Requirements (BRS/StRS)

### Business Goals & Success Metrics

| ID | Goal | Fit Criterion |
|----|------|---------------|
| G‑CP‑01 | Prevent comment loss during LLM‑driven patching | ≥95% of comments from the original file preserved after patch (measured by a test suite containing 100 common real‑world comment‑heavy Rust/TypeScript files) |
| G‑CP‑02 | Maintain patch‑ts usability and performance | The comment‑preservation pass adds ≤50ms overhead for files <2000 lines, ≤200ms for larger files |
| G‑CP‑03 | Zero regression of existing patch correctness | No existing test fails due to the new feature; code‑only changes are unaffected when flag is off |

### Stakeholder Needs

**LLM agent developers (primary):**
- PA‑01: I need to apply a patch that changes only the intended code, and automatically keeps all original comments intact.
- PA‑02: I want to trust that the tool will handle re‑indentation so comments stay aligned.

**Patch‑ts maintainers (secondary):**
- PA‑03: The new feature must integrate with the existing `Language` trait and support all languages already supported.
- PA‑04: The implementation must be testable and reference the existing tree‑sitter infrastructure.

### Business Rules

| ID | Rule |
|----|------|
| BR‑CP‑01 | Comments are re‑attached *after* the patch is applied and *before* the final syntax validation pass. |
| BR‑CP‑02 | The comment preservation step must not change the semantics of the patched code (except for re‑inserting comments). |
| BR‑CP‑03 | Inline comments are reattached to the same line of the nearest surviving statement; doc comments (`///`, `/**`) are placed before the entity they document. |
| BR‑CP‑04 | If the entity a comment was attached to no longer exists, the comment is either dropped (default) or appended as a block of orphaned comments (with a warning). |

---

## 3. System Requirements (SRS)

### Functional Requirements

| ID | Requirement | Acceptance Criteria |
|----|-------------|---------------------|
| REQ‑F‑CP‑001 | The `patch` subcommand shall accept a `--preserve-comments` boolean flag (optional, default false). | Invoking `patch --help` shows the flag. |
| REQ‑F‑CP‑002 | When `--preserve-comments` is used with any patch mode (literal, unified diff, symbol, marker, content‑based, heredoc), the system shall apply the patch first, then re‑attach comments from the original file to the patched content. | Verified by automated test: (a) original file with comments, (b) apply a patch that changes code but has no comments, (c) result contains all original comments at their correct positions. |
| REQ‑F‑CP‑003 | The comment re‑attachment algorithm shall extract comments from the original AST (tree‑sitter `comment`, `line_comment`, `block_comment` nodes) and map each to an anchor entity (function, struct, etc.) using the `Language` trait. | Given a Rust source with a function `fn foo()`, the system must associate a comment immediately preceding that function with the `foo` entity. |
| REQ‑F‑CP‑004 | For each comment, the system shall locate the corresponding anchor entity in the patched AST (by name or, if not uniquely named, by structural position) and compute a new byte offset for insertion. | Test: original has two functions with same name? Use structural index. |
| REQ‑F‑CP‑005 | The system shall insert each comment at its new position, adjusting leading whitespace to match the indentation of the insertion line (using the `indent` module). | Original indented comment `// old code` inside a 4‑space block remains indented correctly after re‑attachment. |
| REQ‑F‑CP‑006 | Inline comments (same line as code) shall be re‑attached to the nearest surviving statement in the patched version, preserving trailing position. | Test: `let x = 1; // initialize` after a patch that changes `1` to `2` still shows `let x = 2; // initialize`. |
| REQ‑F‑CP‑007 | Doc comments (`///`, `/**`) shall be placed immediately before their associated entity in the patched file. | A doc comment preceding `fn foo()` remains right before `fn foo()` after patch. |
| REQ‑F‑CP‑008 | Orphan comments (whose anchor entity was removed) shall be omitted by default, with an optional configuration to append them at end of file as a block with a warning indicator. | A test that deletes a function shows its preceding comment is dropped (or appended when `--preserve-orphan-comments` is used). |
| REQ‑F‑CP‑009 | The comment preservation shall be applied before final syntax validation and compilation checks, so that the patched+commented code is validated. | Ensure the post‑preserve validation pass catches syntax errors introduced by comment re‑insertion (ideally none). |
| REQ‑F‑CP‑010 | The feature shall support all languages currently supported by `detect_language` (Rust, TS, JS, Python, Go, etc.). | Tests for at least Rust, TypeScript, Python to ensure cross‑language comment extraction. |

### Non‑Functional Requirements

| ID | Requirement | Fit Criterion |
|----|-------------|---------------|
| NFR‑CP‑001 | Performance: comment re‑attachment must not exceed 50ms for files up to 2000 lines, measured with a 2000‑line Rust file with 100 comments. | Benchmark test: median time ≤50ms on a standard CI machine. |
| NFR‑CP‑002 | Accuracy: ≥95% of comments (by count) in a set of 50 representative source files shall be correctly re‑attached. | Automated test suite checks preservation rate. |
| NFR‑CP‑003 | Backward compatibility: when `--preserve-comments` is not set, patching behavior is unchanged. | All existing tests pass without modification. |

---

## 4. Architecture & Design Specification

### Context & Scope

We are adding a new module `src/comment_preserve.rs` and a new flag `--preserve-comments` to the existing `patch` command. The feature uses the existing `ast::Language` trait and tree‑sitter parsing to extract comments, map them to entities, locate those entities in the patched AST, and re‑insert comments with indentation corrections.

### Architecturally Significant Requirements

- ASR‑1 (Performance): Re‑attachment overhead must be minimal (≤50ms for typical files). This drives a design that reuses existing parse results, avoiding re‑parsing of the original file (the original parse result already exists in `apply_patch_to_file`).
- ASR‑2 (Cross‑language): The algorithm should work for all `Language` implementations. The `Language` trait must be extended with methods to support comment extraction and entity mapping for all languages, or a default fallback using AST traversal.
- ASR‑3 (Placement accuracy): Comments must be placed intelligently (doc comments before entity, inline after statement). The algorithm must differentiate comment types.

### Design Decisions (ADRs)

**ADR‑CP‑001: Use post‑patch re‑attachment rather than pre‑patching comment preservation**

- **Context:** Comment preservation could be done by pre‑scanning and trying to avoid deleting comment lines, but that would be fragile and dependent on patch semantics.
- **Decision:** Apply the patch normally (which may strip comments), then re‑attach all original comments using an anchor‑relocation strategy.
- **Consequences:** Simpler integration with all patch modes; however, if the patch moves entities significantly, relocation may be imprecise; acceptable.

**ADR‑CP‑002: Extend `Language` trait with `extract_comments` and `find_enclosing_entity` methods**

- **Context:** Currently, `Language` has methods like `find_symbol_node` and `entity_body_range`, but no generic comment extraction. We need to identify comments and their parent entities.
- **Decision:** Add two new optional methods with default implementations:
  - `fn extract_comments(&self, result: &ParseResult) -> Vec<CommentNode>` – traverses AST and returns comment nodes with text, byte range, and parent entity info.
  - `fn find_enclosing_entity(&self, node: Node) -> Option<(usize, usize, String)>` – returns start, end bytes and entity name (if any) of enclosing function/struct.
- **Consequences:** Existing language implementations (Rust, TS, etc.) will need to override these if the default tree‑sitter method is insufficient; Rust and TS have clear `comment` node types and entity names, so override will be straightforward.

**ADR‑CP‑003: Indentation normalization using existing `indent` module**

- **Context:** Comments must match the indentation of the line where they are inserted.
- **Decision:** Use `indent::detect_indent` on the target line in the patched content and `indent::apply_indent` to adjust comment lines.
- **Consequences:** Works well for indentation‑based semantics (Python) but may need special handling for tab vs space; existing module handles both.

### Component Interaction

1. `cli/patch.rs` (or `apply_patch_to_file`) after the patch is written to the file but before compile‑check, if `--preserve-comments` is true:
   - Read original content (kept before patch) and new patched content.
   - Call `preserve_comments(original, patched, lang)` which returns updated patched content.
   - Write the new content to the file (overwrites).
2. `comment_preserve.rs`:
   - `preserve_comments` orchestrates parse of original and patched, extracts comments with entity anchors, relocates anchors in patched AST, inserts comments, adjusts indentation.
   - Helper functions: `extract_comments_with_anchors`, `relocate_anchors`, `insert_comments`.
3. Dependencies: `ast.rs` (Language trait, ParseResult), `semdiff.rs` (optional for entity movement detection), `indent.rs`.

### API/Interface Changes

- No external API changes except the CLI flag.
- The `Language` trait gains two new methods (described above).

### C4 Model (text description)

- **System Context:** patch‑ts CLI enhanced with `--preserve-comments` interacts with source files and tree‑sitter parsers.
- **Container:** The `patch` subcommand module now invokes `comment_preserve` after the core patch logic.
- **Component (new):** `comment_preserve.rs` encapsulates comment extraction, anchor relocation, and insertion; it depends on the `Language` trait implementations and `indent` module.

---

## 5. Behavioral Specification & Test Verification

### 5.1 Behavioral Specifications (BDD Scenarios)

**Feature: Comment‑Preserving Patch**

```gherkin
Scenario: Patch code without stripping comments (literal patch)
  Given a Rust file with content:
    """
    // This is the main function
    fn main() {
        println!("Hello"); // print greeting
    }
    """
  And a patch command with --preserve-comments that changes the line `        println!("Hello");` to `        println!("Hi");`
  When the patch is applied
  Then the resulting file contains:
    """
    // This is the main function
    fn main() {
        println!("Hi"); // print greeting
    }
    """

Scenario: Doc comment preserved before function after patch
  Given a Rust file with:
    """
    /// Adds two numbers
    fn add(a: i32, b: i32) -> i32 {
        a + b
    }
    """
  And a patch that changes the function body to `a * b`
  When applied with --preserve-comments
  Then the file still has the `/// Adds two numbers` line immediately before the function.

Scenario: Inline comment moved with statement
  Given a TypeScript file with:
    """
    let speed = 10; // m/s
    """
  And a patch that changes `10` to `20`
  When applied with --preserve-comments
  Then the line reads `let speed = 20; // m/s`

Scenario: Orphan comment when function removed (default behavior)
  Given a Rust file with:
    """
    // old helper
    fn helper() {}
    """
  And a unified diff that removes `fn helper() {}` entirely
  When applied with --preserve-comments
  Then the comment "// old helper" is not present in the output.
```

### 5.2 Test Strategy

- **Unit tests:**
  - `comment_preserve::extract_comments` returns correct comment nodes for a known source.
  - Anchor relocation for simple rename, deletion, structural index.
  - Indentation adjustment with tab and space.
- **Integration tests:**
  - Apply literal patch with `--preserve-comments` on a Rust/TS file and verify comment preservation.
  - Test with all supported patch modes (diff, symbol, content‑based).
  - Test with multi‑file patches (should preserve per‑file).
- **Performance tests:** Benchmark `preserve_comments` on a 2000‑line Rust file with many comments; ensure under 50ms.
- **Regression tests:** All existing tests run without failure; comment‑preservation is off by default.
- **Cross‑language tests:** At minimum Rust, TypeScript, Python.

### 5.3 Requirements Traceability Matrix

| Business Goal | Stakeholder Need | System Requirement | Test Case/Scenario |
|---------------|------------------|-------------------|-------------------|
| G‑CP‑01 | PA‑01, PA‑02 | REQ‑F‑CP‑002 – REQ‑F‑CP‑010 | BDD scenarios above, integration tests |
| G‑CP‑02 | PA‑04 | NFR‑CP‑001 | Benchmark test |
| G‑CP‑03 | PA‑03 | NFR‑CP‑003 | Full existing test suite |

---

## Summary

The specification outlines a self‑contained enhancement to `patch-ts` that leverages the existing tree‑sitter and `Language` infrastructure to add a `--preserve-comments` flag. The feature re‑attaches all original comments after the patch, using anchor‑based relocation and indentation correction. The design is modular (`comment_preserve.rs`), minimally invasive, and thoroughly testable. This enhancement will make `patch-ts` the first tool to explicitly solve comment loss from AI‑generated patches, strengthening its value as an LLM‑friendly patching CLI.
