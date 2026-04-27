## v1.18.0 (2026-04-27)

### Added
- **Content‑based replacement**: Apply patches without a `--line` number; the tool locates and replaces the old content anywhere in the file.  
  - Automatic relaxation cascade: if exact match fails, the tool retries with whitespace normalization, comment stripping, and Jaccard similarity.
- **SEARCH/REPLACE blocks**: Use the `<<< SEARCH … --- …` heredoc format for reliable multi‑line patch input without line numbers.
- **Entity body replacement**: New `--entity-body` flag replaces only the body of a named function, class, or method, keeping the signature intact.
- **Hunk‑fuzzy diff application**: Unified diffs are now applied by matching context lines; hallucinated line numbers are ignored (`fix_all_hunk_headers` / `match_hunk_by_context`).
- **Byte‑span primitive**: All patching operations internally use `replace_byte_range` for robust, line‑agnostic replacements.
- **MCP node‑targeted tools**: Three new MCP tools – `replace_node`, `delete_node`, `insert_before_node` – accept tree‑sitter queries for precise AST‑level edits.
- **Enhanced recall**: `get_best_strategy` function uses historical success rates to suggest the most effective retry approach; `recall` output includes strategy history.

### Changed
- Refactored `cascade_match` and `apply_literal_patch` to accept a `content` string so matches can return byte ranges.
- Made `--line` optional in `patch` command; when omitted, content‑based or entity replacement modes are used.
- RustLanguage `find_symbol_node` and `find_all_entities` are now proper trait implementations, enabling correct `entity_body_range` resolution.
- Expanded `cli/patch.rs` with content‑based replacement branch and SEARCH/REPLACE heredoc detection.

### Fixed
- Missing `;;` closing brace in `apply_unified_diff` after hunk‑fuzzy integration.
- `heritage` module declaration and missing `entity_body` field in `PatchArgs` across multiple files.
- Compilation errors from `StreamingIterator` import in `node_patch.rs` and improper `required_unless_present_any` constraint.
- Byte‑span test ranges corrected.
- Several unused‑variable and import warnings resolved.

### Infrastructure
- Added integration tests covering content‑based patches, SEARCH/REPLACE blocks, entity body replacement, and hunk‑fuzzy diffs.
- Updated module structure: `src/patchex.rs`, `src/matching_flex.rs`, `src/cli/heritage.rs`, `src/patch/node_patch.rs`.

## v1.13.0 (2026-04-24)

### Added
- **Recall Mode**: Token‑efficient patch retry context generator (`recall` command).
  - Structured JSON and human‑readable prompt outputs.
  - Strategy database mapping error codes to suggested retry approaches.
  - Recall history tracking (`.patch-ts/recall.jsonl`) for experiential learning.
  - MCP `recall` tool for agent workflows.
  - Token savings of ~86‑90% compared to full file re‑reads on retry.

### Changed
- Minor warning cleanups across the codebase.

### Fixed
- Compilation issues related to MCP tool handler generation.
- Unused variable warnings in recall and incremental parsing modules.

## v1.14.0 (2026-04-25)

### Added
- **WASM Build Target**: `crates/patch-ts-wasm` scaffold for compiling core patching to `wasm32-wasi`.
- **FileSystem Trait**: Abstraction over file I/O with `StdFileSystem` and `VirtualFileSystem` implementations.
- **Entropy‑based Recall**: `recall --entropy` uses Shannon entropy to select the most information‑dense context lines.
- **Predictive Pre‑fetch**: `recall --pre-fetch` automatically includes callers/callees and containing function body.
- **Minimal Recall Mode**: `recall --minimal` outputs a compact context (error, line, best strategy) for extreme token savings.
- **Session Memory**: `recall --session <ID>` tracks retry attempts and re‑ranks strategies based on past success.
- **Token Budget Mode**: `recall --max-tokens <N>` enforces a hard token limit with intelligent truncation.

### Changed
- `RecallArgs` CLI struct updated with all new flags.
- MCP `RecallParams` extended to support entropy, pre‑fetch, and budget options.
- Unit tests expanded to cover entropy selection and new recall features.

### Fixed
- Borrow‑checker issues in `find_containing_symbol_rich`.
- Duplicate field definitions in recall structs.

## v1.14.0 (2026-04-25)

### Added
- **WASM Build Target**: `crates/patch-ts-wasm` scaffold for compiling core patching to `wasm32-wasi`.
- **FileSystem Trait**: Abstraction over file I/O with `StdFileSystem` and `VirtualFileSystem` implementations.
- **Entropy‑based Recall**: `recall --entropy` uses Shannon entropy to select the most information‑dense context lines.
- **Predictive Pre‑fetch**: `recall --pre-fetch` automatically includes callers/callees and containing function body.
- **Minimal Recall Mode**: `recall --minimal` outputs a compact context (error, line, best strategy) for extreme token savings.
- **Session Memory**: `recall --session <ID>` tracks retry attempts and re‑ranks strategies based on past success.
- **Token Budget Mode**: `recall --max-tokens <N>` enforces a hard token limit with intelligent truncation.

### Changed
- `RecallArgs` CLI struct updated with all new flags.
- MCP `RecallParams` extended to support entropy, pre‑fetch, and budget options.
- Unit tests expanded to cover entropy selection and new recall features.

### Fixed
- Borrow‑checker issues in `find_containing_symbol_rich`.
- Duplicate field definitions in recall structs.
