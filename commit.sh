#!/usr/bin/env bash
# ==============================================================================
# Bash 3.2 compatible macOS script to commit each modified file individually
# with a conventional commit message based on the analysis of the provided diff.
# ==============================================================================
set -euo pipefail

# Ensure we are in a git repository
if ! git rev-parse --is-inside-work-tree >/dev/null 2>&1; then
    echo "Error: Not a git repository." >&2
    exit 1
fi

# Global variables for commit message
COMMIT_SUBJECT=""
COMMIT_BODY=""

# Function to determine commit message based on file path
get_commit_info() {
    local file="$1"
    COMMIT_SUBJECT=""
    COMMIT_BODY=""

    case "$file" in
        "src/api.rs")
            COMMIT_SUBJECT="feat(api): add in-memory library API"
            COMMIT_BODY="- Expose core engines (search/replace, balance, validate) over plain strings
- Allow embedders to avoid round-tripping through temporary files
- Add comprehensive unit tests for the new API surface"
            ;;
        "src/compile.rs")
            COMMIT_SUBJECT="fix(compile): use rustc for single-file checks to avoid cargo lock"
            COMMIT_BODY="- Prefer lock-free single-file rustc check for isolated snippets
- Fall back to cargo check with explicit manifest path for workspace files
- Drain stdout/stderr on helper threads to prevent pipe deadlocks
- Add robust working directory resolution and timeout handling"
            ;;
        "src/ast.rs")
            COMMIT_SUBJECT="refactor(ast): improve delimiter error detection and function lookup"
            COMMIT_BODY="- Harvest tree-sitter MISSING nodes for precise repair suggestions
- Fallback to ERROR node traversal for unbalanced function bodies
- Replace raw 'as' casts with safe crate::num saturating conversions
- Apply pedantic clippy lints and #[must_use] annotations"
            ;;
        "src/patch/mod.rs")
            COMMIT_SUBJECT="refactor(patch): split options into RuntimeFlags and MatchFlags"
            COMMIT_BODY="- Decouple execution toggles (dry_run, force) from matching behavior
- Extract syntax gate enforcement and match targeting into dedicated helpers
- Ensure multi-line expected blocks consume the correct span
- Apply strict clippy pedantic formatting"
            ;;
        "src/repair/search.rs")
            COMMIT_SUBJECT="fix(repair): seed BFS frontier with precomputed delimiter errors"
            COMMIT_BODY="- Avoid redundant initial parse by seeding the search queue directly
- Enforce max_cost budget strictly before accepting solutions
- Refactor visited-state tracking into a closure for clarity"
            ;;
        "src/cli/patch.rs")
            COMMIT_SUBJECT="refactor(cli): modularize patch command and improve delta semantics"
            COMMIT_BODY="- Extract diff application, content-based matching, and verification into helpers
- Only reject patches that *introduce* compilation errors (delta semantics)
- Pass arguments by reference to avoid massive struct clones
- Enforce pedantic clippy lints across all CLI handlers"
            ;;
        "src/cli/types.rs")
            COMMIT_SUBJECT="refactor(cli): group boolean flags into flattened toggle structs"
            COMMIT_BODY="- Introduce RunToggles, BalanceToggles, RecallToggles to reduce clap boilerplate
- Box PatchArgs in the Command enum to reduce variant size
- Apply consistent formatting and documentation comments"
            ;;
        "src/num.rs")
            COMMIT_SUBJECT="feat(num): add saturating numeric conversion helpers"
            COMMIT_BODY="- Provide safe alternatives to 'as' casts for clippy pedantic compliance
- Handle usize/u32/isize/f64 conversions with explicit saturation boundaries
- Include comprehensive unit tests for edge cases and NaN handling"
            ;;
        "src/mcp.rs")
            COMMIT_SUBJECT="refactor(mcp): modularize tool dispatch and resource routing"
            COMMIT_BODY="- Extract tool definitions and resource list into dedicated functions
- Route tools/call through a centralized dispatch_tool matcher
- Update JSON-RPC handlers to use safe numeric conversions
- Add exhaustive integration tests for all MCP tools"
            ;;
        "src/recall.rs")
            COMMIT_SUBJECT="refactor(recall): encapsulate query parameters in RecallQuery struct"
            COMMIT_BODY="- Replace long parameter lists with a structured RecallQuery object
- Migrate from once_cell::Lazy to std::sync::LazyLock
- Improve entropy calculation and strategy selection logic"
            ;;
        "src/verify.rs")
            COMMIT_SUBJECT="fix(verify): implement delta semantics for compilation checks"
            COMMIT_BODY="- Only fail the compile stage if the patch introduces new errors
- Compare against baseline content compilation to avoid false positives
- Improve stage verdict reporting and error details"
            ;;
        "src/scitt.rs")
            COMMIT_SUBJECT="fix(scitt): improve hash chain verification and signature checks"
            COMMIT_BODY="- Fix hash chain break detection logic
- Add tests for bad signature detection
- Ensure robust parsing of provenance records"
            ;;
        "src/heal.rs")
            COMMIT_SUBJECT="refactor(heal): modularize heuristic repair strategies"
            COMMIT_BODY="- Separate cost-weighted and history-guided heuristics
- Improve error description formatting for history lookup
- Add comprehensive tests for all healing paths"
            ;;
        "src/gate.rs")
            COMMIT_SUBJECT="refactor(gate): convert stage runners to return StageResult directly"
            COMMIT_BODY="- Remove unnecessary Result wrapping for pure stage functions
- Add new OWASP and SWE-bench gate stages
- Improve parallel execution and error reporting"
            ;;
        "src/security.rs")
            COMMIT_SUBJECT="refactor(security): migrate OWASP patterns to std::sync::LazyLock"
            COMMIT_BODY="- Replace once_cell::Lazy with standard library LazyLock
- Improve pattern matching and category filtering
- Add tests for tool poisoning and prompt injection detection"
            ;;
        "src/tui.rs")
            COMMIT_SUBJECT="refactor(tui): improve key event handling and view modes"
            COMMIT_BODY="- Consolidate key event matching logic
- Improve comment collection and view mode toggling
- Apply rustfmt and clippy pedantic fixes"
            ;;
        "src/lsp.rs")
            COMMIT_SUBJECT="refactor(lsp): update LSP diagnostics and command handling"
            COMMIT_BODY="- Improve diagnostic publishing and range calculation
- Use safe numeric conversions for LSP positions
- Streamline balance command execution"
            ;;
        "Cargo.toml"|"Cargo.lock")
            COMMIT_SUBJECT="chore(deps): bump workspace dependencies and tree-sitter grammars"
            COMMIT_BODY="- Upgrade tree-sitter core and language bindings to latest versions
- Bump ratatui, crossterm, wasmtime, and wit-bindgen
- Update dev-dependencies (criterion, proptest, jsonschema)
- Enable pedantic clippy lints workspace-wide"
            ;;
        ".github/workflows/"*|".github/dependabot.yml"|"cliff.toml"|"justfile")
            COMMIT_SUBJECT="ci: modernize CI/CD pipelines and add release automation"
            COMMIT_BODY="- Add coverage gate with cargo-llvm-cov and configurable threshold
- Configure git-cliff for automated conventional changelog generation
- Add multi-target release workflow with cross-compilation
- Update dependabot to group minor/patch Cargo updates"
            ;;
        "README.md"|"CONTRIBUTING.md")
            COMMIT_SUBJECT="docs: document library API and CI/CD workflows"
            COMMIT_BODY="- Add section explaining the new in-memory patch_ts::api module
- Document coverage enforcement and conventional commit requirements
- Update CLI command references and architecture diagrams"
            ;;
        "benches/"*)
            COMMIT_SUBJECT="perf(bench): update criterion benchmarks for new API"
            COMMIT_BODY="- Adapt benchmark harness to pass PatchOptions by mutable reference
- Add preserve-comments benchmark for large files
- Use std::fmt::Write for efficient string building"
            ;;
        "crates/"*)
            COMMIT_SUBJECT="refactor(crates): update workspace crate re-exports"
            COMMIT_BODY="- Format module declarations and re-exports
- Apply rustfmt and clippy pedantic fixes"
            ;;
        ".patch-ts/"*)
            COMMIT_SUBJECT="chore(history): update provenance and history logs"
            COMMIT_BODY="- Append new patch execution records
- Maintain SCITT-compliant hash chain"
            ;;
        *)
            # Smart fallback based on directory structure
            local dir="${file%/*}"
            local base="${file##*/}"

            if [[ "$dir" == "src/cli" ]]; then
                COMMIT_SUBJECT="refactor(cli): apply pedantic lints to ${base}"
                COMMIT_BODY="- Pass CLI arguments by reference to avoid cloning
- Add #[must_use] and error documentation
- Format with rustfmt and resolve clippy pedantic warnings"
            elif [[ "$dir" == "src/patch" || "$dir" == "src/repair" ]]; then
                local mod="${dir#src/}"
                COMMIT_SUBJECT="refactor(${mod}): apply pedantic lints and safe casts"
                COMMIT_BODY="- Replace implicit 'as' casts with crate::num helpers
- Add #[must_use] annotations and '# Errors' doc sections
- Improve error handling and formatting"
            elif [[ "$dir" == "src" ]]; then
                COMMIT_SUBJECT="refactor(${base%.rs}): apply pedantic lints and safe casts"
                COMMIT_BODY="- Replace implicit 'as' casts with crate::num helpers
- Add #[must_use] annotations and '# Errors' doc sections
- Improve error handling and formatting"
            elif [[ "$dir" == "tests" ]]; then
                local tname="${base%.rs}"
                COMMIT_SUBJECT="test(${tname}): update integration tests and remove ignores"
                COMMIT_BODY="- Re-enable previously ignored tests now supported by the new repair engine
- Update assertions to match new JSON diagnostic shapes
- Apply rustfmt and clippy pedantic fixes"
            else
                COMMIT_SUBJECT="style: format and apply pedantic lints to ${file}"
                COMMIT_BODY="- Run rustfmt and resolve clippy pedantic warnings
- Ensure consistent code style across the workspace"
            fi
            ;;
    esac
}

# Create a temporary file to hold the list of changed files
TMP_FILE=$(mktemp)
trap 'rm -f "$TMP_FILE"' EXIT

# Get all modified, deleted, and untracked files
# Using -z for null-terminated output to safely handle spaces/special characters
git ls-files --modified --deleted -z > "$TMP_FILE"
git ls-files --others --exclude-standard -z >> "$TMP_FILE"

# Check if there are any changes
if [[ ! -s "$TMP_FILE" ]]; then
    echo "No changes detected in the working tree."
    exit 0
fi

echo "Starting individual file commits..."

# Process each file
while IFS= read -r -d '' file; do
    # Skip if file is empty (safeguard)
    [[ -z "$file" ]] && continue

    # Get commit message
    get_commit_info "$file"

    echo "Committing: $file"
    echo "  Subject: $COMMIT_SUBJECT"

    # Stage the file (handle deletions gracefully)
    if [[ -e "$file" ]]; then
        git add -- "$file"
    else
        git rm -- "$file" >/dev/null 2>&1 || true
    fi

    # Commit with subject and body
    # Using -m twice creates a commit with a subject and a body paragraph
    git commit -m "$COMMIT_SUBJECT" -m "$COMMIT_BODY" --quiet

done < "$TMP_FILE"

echo "All files committed successfully!"
