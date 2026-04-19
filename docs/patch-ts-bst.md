# Behavioral Specification & Test Verification Plan

| Field | Value |
|-------|-------|
| Project | `patch-ts` |
| Document | Behavioral Specification & Test Verification Plan |
| Version | 1.0 |
| Date | 2026-04-19 |
| Author | AI-assisted (spec-writer) |
| Status | Draft — Pending Review |

---

## 1. Introduction

This document provides the behavioral specifications (Level 4) and test verification plan (Level 5) for `patch-ts` v1.0. It translates the SRS requirements into concrete, testable scenarios using Specification by Example / BDD patterns and defines the overall test strategy, test cases, and traceability matrix to ensure complete verification coverage.

---

## 2. Behavioral Specifications (Specification by Example)

This section defines executable acceptance criteria for key functional requirements using Given‑When‑Then scenarios, decision tables, and state models.

### 2.1 Patch Application Scenarios

#### Feature: Apply Single‑Line Replacement

**Traceability:** REQ-FUNC-010, REQ-FUNC-011

```gherkin
Feature: Apply single-line replacement with verification

  Background:
    Given a file "src/lib.rs" with content:
      """
      fn main() {
          println!("Hello, world!");
      }
      """

  Scenario: Successful replacement when expected content matches exactly
    When the user runs:
      """
      patch-ts --file src/lib.rs --line 2 <<'EOF'
      <<<
          println!("Hello, world!");
      ---
          println!("Hello, patch-ts!");
      EOF
      """
    Then the command exits with status 0
    And the file "src/lib.rs" now contains:
      """
      fn main() {
          println!("Hello, patch-ts!");
      }
      """
    And a backup file "src/lib.rs.bak" exists with original content

  Scenario: Replacement fails when expected content does not match
    When the user runs:
      """
      patch-ts --file src/lib.rs --line 2 <<'EOF'
      <<<
          println!("Goodbye, world!");
      ---
          println!("Hello, patch-ts!");
      EOF
      """
    Then the command exits with non-zero status
    And the file "src/lib.rs" remains unchanged
    And stderr contains "Expected content not found at line 2"
    And stderr contains a diff showing actual vs expected

  Scenario: Fuzzy match locates shifted line
    Given the file "src/lib.rs" has an extra comment line at the top, shifting the target to line 3
    When the user runs with `--fuzz 3` and expected content targeting original line 2
    Then the patch is applied successfully at line 3
    And the command reports "Found expected content at line 3 (shifted by +1)"
```

#### Feature: Delete Line with Verification

**Traceability:** REQ-FUNC-012

```gherkin
Feature: Delete a line after verifying its content

  Background:
    Given a file "src/lib.rs" with content:
      """
      fn main() {
          // TODO: remove this
          println!("Hello");
      }
      """

  Scenario: Successful deletion when line matches expected content
    When the user runs:
      """
      patch-ts --file src/lib.rs --delete 2 --expect "// TODO: remove this"
      """
    Then the command exits with status 0
    And line 2 is removed from the file

  Scenario: Deletion fails when line content does not match
    When the user runs:
      """
      patch-ts --file src/lib.rs --delete 2 --expect "// FIXME: remove this"
      """
    Then the command exits with non-zero status
    And the file remains unchanged
    And stderr indicates expected content mismatch
```

#### Feature: Insert Lines

**Traceability:** REQ-FUNC-013

```gherkin
Feature: Insert new lines after a specified line

  Scenario: Insert single line after existing line
    Given a file with 3 lines
    When the user runs:
      """
      patch-ts --file src/lib.rs --after 2 --content "    // New comment"
      """
    Then the new line is inserted as line 3
    And the original line 3 becomes line 4
```

#### Feature: Apply Unified Diff

**Traceability:** REQ-FUNC-020

```gherkin
Feature: Apply patch in unified diff format

  Scenario: Apply a simple unified diff successfully
    Given a file with original content
    When the user runs `patch-ts --diff` with a valid unified diff on stdin
    Then the changes are applied
    And the command exits with status 0

  Scenario: Diff hunk fails due to mismatched context
    When the diff context lines do not match the current file
    Then the command fails with a diagnostic indicating which hunk failed
    And the file remains unchanged
```

### 2.2 AST Validation Scenarios

**Traceability:** REQ-FUNC-030, REQ-FUNC-031

```gherkin
Feature: Prevent patches that introduce syntax errors

  Scenario: Patch introduces unbalanced brace and is rejected
    Given a syntactically valid Rust file
    When the user attempts to apply a patch that adds an extra `}` without matching `{`
    Then the command exits with non-zero status
    And the file is not modified
    And stderr contains "Patch would introduce syntax error"
    And stderr shows the location of the extra delimiter

  Scenario: Force flag overrides syntax validation
    When the user applies the same invalid patch with `--force`
    Then the file is modified despite the syntax error
    And the command exits with status 0
```

### 2.3 Dry‑Run and Backup Scenarios

**Traceability:** REQ-FUNC-040, REQ-FUNC-041

```gherkin
Feature: Dry-run and backup behavior

  Scenario: Dry-run shows diff but does not modify file
    When the user runs a valid patch with `--dry-run`
    Then the command outputs a unified diff of the changes
    And the original file is unchanged

  Scenario: Backup is created before modification
    When a patch is applied successfully without `--no-backup`
    Then a file with `.bak` extension is created with original content

  Scenario: No backup with --no-backup flag
    When a patch is applied with `--no-backup`
    Then no `.bak` file is created
```

### 2.4 Structural Repair Scenarios

**Traceability:** REQ-FUNC-050, REQ-FUNC-051, REQ-FUNC-052

```gherkin
Feature: Balance command fixes unbalanced delimiters

  Scenario: Extra closing brace is detected and removed
    Given a file with an extra `}` at line 42
    When the user runs `patch-ts balance --file src/lib.rs --dry-run`
    Then the command outputs a diff showing the removal of line 42
    And suggests running with `--apply` to fix

  Scenario: Apply balance fix
    When the user runs `patch-ts balance --file src/lib.rs --apply`
    Then the extra `}` is removed
    And the resulting file parses without syntax errors

  Scenario: Balance specific function only
    When the user runs `patch-ts balance --file src/lib.rs --function "navigate" --apply`
    Then only delimiters within the function body are balanced
    And code outside the function is unchanged

Feature: Explain command provides AST context

  Scenario: Explain pinpoints extra delimiter
    Given a file with an unexpected `}` at line 566
    When the user runs `patch-ts explain --file src/lib.rs --line 566`
    Then the output states that line 566 is an extra closing brace
    And suggests running `patch-ts balance`
    And shows the likely matching opening brace location

  Scenario: Explain on valid syntax line
    When the user runs `patch-ts explain` on a line with no syntax error
    Then the output states "No syntax error detected at this line"
```

### 2.5 Diagnostic Output Scenarios

**Traceability:** REQ-FUNC-060, REQ-FUNC-061, REQ-FUNC-062

```gherkin
Feature: Diagnostic output formats

  Scenario: Human-readable diagnostic on failure
    When a patch fails due to content mismatch
    Then stderr contains a formatted message with:
      - File path and line number
      - Expected vs actual content
      - Source snippet with context (±3 lines)
      - Suggestion (e.g., "Try --fuzz 5")

  Scenario: JSON output on failure
    When a patch fails and `--json` is provided
    Then stdout contains a JSON object with fields:
      - `success: false`
      - `error.code`
      - `error.message`
      - `error.span.file`
      - `error.span.line`
      - `error.context`
      - `error.suggestion`

  Scenario: JSON output on success
    When a patch succeeds and `--json` is provided
    Then stdout contains `{"success": true}`

  Scenario: Exit codes reflect outcome
    When a patch succeeds, exit code is 0
    When a patch fails due to content mismatch, exit code is 2
    When a patch fails due to syntax error, exit code is 3
    When a file is not found, exit code is 1
```

### 2.6 Edge Cases and Unwanted Behavior

**Decision Table: Fuzz Matching Edge Cases**

| Condition | Match Found? | Action |
|-----------|--------------|--------|
| Exact line matches expected content | Yes (exact) | Apply patch |
| Expected content found within fuzz radius | Yes (fuzzy) | Apply patch, report shift |
| Multiple matches within fuzz radius | Ambiguous | Abort, list candidate lines |
| No match within fuzz radius | No | Abort, suggest increasing fuzz |
| Fuzz radius = 0 | N/A | Exact match only |

**State Model: File Modification Safety**

```
States: Unmodified, PatchedInMemory, Validated, Written

Events:
- ParsePatch → Unmodified → PatchedInMemory
- ValidateAST → PatchedInMemory → Validated (if valid)
- ValidateAST → PatchedInMemory → Unmodified (if invalid, abort)
- WriteFile → Validated → Written
```

---

## 3. Test Strategy and Plan

### 3.1 Overall Test Strategy

`patch-ts` follows a **test trophy** approach with emphasis on integration and acceptance tests due to the CLI nature and file system interactions.

| Test Level | Scope | Tools | Goal |
|------------|-------|-------|------|
| **Unit Tests** | Individual modules (parser, fuzzy matcher, AST wrapper) | Rust `#[test]`, `rstest` | Validate logic in isolation. |
| **Integration Tests** | CLI invocation with real files | `assert_cmd`, `tempfile` | Verify end‑to‑end behavior. |
| **Acceptance Tests** | BDD scenarios (Gherkin) | `cucumber` crate | Executable specifications. |
| **Performance Tests** | Latency benchmarks | `criterion` | Ensure ASR‑001 compliance. |
| **Property‑Based Tests** | Fuzzy matching and diff application | `proptest` | Discover edge cases. |

### 3.2 Test Environments

| Environment | Description |
|-------------|-------------|
| **CI (Linux)** | GitHub Actions `ubuntu-latest` |
| **CI (macOS)** | GitHub Actions `macos-latest` |
| **CI (Windows)** | GitHub Actions `windows-latest` |
| **Local dev** | Developer's machine |

### 3.3 Test Data Strategy

- **Sample Rust files:** A corpus of valid and invalid Rust snippets (various editions) stored in `tests/fixtures/`.
- **Generated inputs:** Use `proptest` to generate random line edits and verify properties (e.g., file unchanged on failure).
- **Real‑world samples:** Include example files from actual Rust projects to test realistic scenarios.

### 3.4 Entry/Exit Criteria

| Gate | Criteria |
|------|----------|
| **PR merge** | All unit and integration tests pass; coverage ≥80%. |
| **Release** | All acceptance scenarios pass on all target platforms; performance benchmarks within thresholds. |

### 3.5 Risk‑Based Test Prioritization

| Risk Area | Priority | Testing Focus |
|-----------|----------|---------------|
| Data corruption (atomic write) | High | Integration tests with simulated crashes |
| AST false negatives (invalid code accepted) | High | Corpus of malformed Rust; fuzz testing |
| Fuzzy match false positives | Medium | Property‑based tests on similarity matching |
| Performance regression | Medium | CI benchmarks with `criterion` comparisons |

---

## 4. Test Case Specifications

Representative test cases mapped to SRS requirements.

### 4.1 Functional Test Cases

| ID | Requirement | Title | Preconditions | Steps | Expected Result |
|----|-------------|-------|---------------|-------|-----------------|
| TC‑F‑010‑1 | REQ-FUNC-010 | Replace block with exact match | File with known content | Run heredoc replace command | File updated, exit 0 |
| TC‑F‑010‑2 | REQ-FUNC-010 | Replace block with fuzz match | File with content shifted by +2 | Run with `--fuzz 3` | Patch applied at correct line, exit 0 |
| TC‑F‑010‑3 | REQ-FUNC-010 | Replace block with no match | File with different content | Run replace command | File unchanged, diagnostic emitted, exit 2 |
| TC‑F‑012‑1 | REQ-FUNC-012 | Delete line with correct expect | Line matches | Run delete command | Line removed, exit 0 |
| TC‑F‑012‑2 | REQ-FUNC-012 | Delete line with wrong expect | Line differs | Run delete command | File unchanged, diagnostic, exit 2 |
| TC‑F‑030‑1 | REQ-FUNC-030 | Reject syntax‑breaking patch | Valid Rust file | Apply patch adding extra `}` | File unchanged, syntax error diagnostic, exit 3 |
| TC‑F‑031‑1 | REQ-FUNC-031 | Force apply syntax‑breaking patch | Same as above | Apply with `--force` | File modified, exit 0 |
| TC‑F‑050‑1 | REQ-FUNC-050 | Balance removes extra brace | File with extra `}` | Run `balance --apply` | Extra brace removed, file parses, exit 0 |
| TC‑F‑052‑1 | REQ-FUNC-052 | Explain on error line | File with extra `}` at L566 | Run `explain --line 566` | Diagnostic identifies extra brace, suggests balance |

### 4.2 Non‑functional Test Cases

| ID | Requirement | Title | Measurement | Threshold |
|----|-------------|-------|-------------|-----------|
| TC‑N‑001‑1 | REQ-NFR-001 | Patch latency on 2000‑line file | `criterion` benchmark | p99 ≤ 100ms |
| TC‑N‑002‑1 | REQ-NFR-002 | Balance latency on 2000‑line file | `criterion` benchmark | p99 ≤ 200ms |
| TC‑N‑010‑1 | REQ-NFR-010 | Atomic write on power loss | Simulated crash during write | File remains intact or fully updated, no corruption |
| TC‑N‑030‑1 | REQ-NFR-030 | Cross‑platform execution | Run test suite on Linux/macOS/Windows | All tests pass |

---

## 5. NFR Verification Plans

### 5.1 Performance Verification

Performance SLOs will be enforced via `criterion` benchmarks in CI. A benchmark suite measures:

- `patch_small_file`: 100‑line file, single‑line replace.
- `patch_large_file`: 2000‑line file, single‑line replace.
- `balance_large_file`: 2000‑line file with one extra brace.

CI will fail the build if benchmark regressions exceed 20% or if absolute thresholds (REQ-NFR-001, REQ-NFR-002) are violated.

### 5.2 Reliability Verification

Atomic write behavior is tested via integration tests that:

1. Create a large file.
2. Spawn `patch-ts` and kill the process during the write phase (using `ptrace` or similar on Linux).
3. Verify the original file is intact and no temporary file remains.

### 5.3 Security Verification

- **Code audit:** Review for unsafe blocks, proper file permissions (`tempfile` defaults to 0o600).
- **Dependency audit:** `cargo audit` in CI.
- **No shell execution:** Ensure no `Command::new("sh")` is used.

### 5.4 Compatibility Verification

CI matrix includes:
- `ubuntu-latest`, `macos-latest`, `windows-latest`
- Rust editions: tests include samples from 2015, 2018, 2021.

### 5.5 Usability Verification

- **Command predictability:** User testing with sample tasks; measure success rate in constructing correct commands.
- **Diagnostic clarity:** Survey users after encountering a failure; Likert scale rating.

---

## 6. Requirements Traceability Matrix (RTM)

| SRS Requirement | BDD Scenario(s) | Test Case(s) | Verification Method |
|-----------------|-----------------|--------------|---------------------|
| REQ-FUNC-010 | Apply Single‑Line Replacement (all) | TC‑F‑010‑1, TC‑F‑010‑2, TC‑F‑010‑3 | Test (automated) |
| REQ-FUNC-011 | (implicit in above) | TC‑F‑010‑1 (with `--old`/`--new`) | Test |
| REQ-FUNC-012 | Delete Line with Verification | TC‑F‑012‑1, TC‑F‑012‑2 | Test |
| REQ-FUNC-013 | Insert Lines | TC‑F‑013‑1 | Test |
| REQ-FUNC-020 | Apply Unified Diff | TC‑F‑020‑1, TC‑F‑020‑2 | Test |
| REQ-FUNC-030 | AST Validation | TC‑F‑030‑1 | Test |
| REQ-FUNC-031 | Force Flag Override | TC‑F‑031‑1 | Test |
| REQ-FUNC-040 | Dry‑Run and Backup | TC‑F‑040‑1 | Test |
| REQ-FUNC-041 | Automatic Backup | TC‑F‑041‑1 | Test |
| REQ-FUNC-050 | Balance Command | TC‑F‑050‑1 | Test |
| REQ-FUNC-051 | Balance Specific Function | TC‑F‑051‑1 | Test |
| REQ-FUNC-052 | Explain Command | TC‑F‑052‑1 | Test |
| REQ-FUNC-060 | Human‑Readable Diagnostic | (all failure scenarios) | Inspection + Test |
| REQ-FUNC-061 | JSON Diagnostic | TC‑F‑061‑1 | Test |
| REQ-FUNC-062 | Exit Codes | (all scenarios) | Test |
| REQ-NFR-001 | Patch Latency | TC‑N‑001‑1 | Analysis + Test |
| REQ-NFR-002 | Balance Latency | TC‑N‑002‑1 | Analysis + Test |
| REQ-NFR-010 | Data Integrity | TC‑N‑010‑1 | Test |
| REQ-NFR-030 | OS Support | CI matrix | Test |
| REQ-NFR-040 | Command Predictability | Usability study | Demonstration |
| REQ-NFR-041 | Diagnostic Clarity | Survey | Demonstration |

---

## 7. Living Documentation Strategy

- **Executable Specifications:** Gherkin feature files stored in `tests/features/` and executed via `cucumber` in CI.
- **Rendered Reports:** CI publishes Cucumber HTML reports as artifacts.
- **RTM Maintenance:** Traceability matrix stored in markdown; updated manually with each requirement change.
- **Performance Reports:** `criterion` benchmarks generate HTML reports stored as CI artifacts.

---

*This verification plan ensures complete coverage of all SRS requirements and provides a repeatable, automated test suite that serves as both acceptance criteria and living documentation.*
