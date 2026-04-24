
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
