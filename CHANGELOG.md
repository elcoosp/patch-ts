
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
