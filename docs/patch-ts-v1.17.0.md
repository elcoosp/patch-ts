# patch‑ts v1.17.0 — Specification  
**Professional Workspace Architecture & Modular Decoupling**

## 1. Executive Summary

**patch‑ts v1.17.0** transforms the monolithic codebase into a **professional, modular Cargo workspace** of five focused crates. It replaces the flat `src/` tree with **domain‑aligned sub‑crates** (`patch‑ts‑core`, `patch‑ts‑gate`, `patch‑ts‑cli`, `patch‑ts‑mcp`, `patch‑ts‑wasm`), extracts **traits** for key abstractions (file system, LSP client, history store), and splits large single‑file modules into focused, single‑responsibility modules.  

The release delivers zero user‑visible changes but dramatically improves **build speed**, **compile‑time validation**, **testability**, and **developer onboarding**.

---

## 2. Design Goals

| ID | Goal | Measurement |
|----|------|-------------|
| G1 | Workspace compilation | `cargo build --workspace` succeeds; incremental build time reduced by ≥30%. |
| G2 | Clean crate boundaries | Each crate has a well‑defined public API; no circular dependencies. |
| G3 | Trait‑based decoupling | `FileSystem`, `LspClient`, and `HistoryStore` traits enable mock implementations and WASM builds. |
| G4 | Large module decomposition | Files with >800 lines are split into sub‑modules; maximum file length ≤500 lines. |
| G5 | Backward compatibility | All existing CLI commands, MCP tools, and public APIs remain unchanged. |
| G6 | No dead code | `cargo clippy --workspace -- -D warnings` passes with zero errors. |
| G7 | Documentation | All public items have doc comments; workspace structure documented in README and CONTRIBUTING. |

---

## 3. New Architecture  

### 3.1 Workspace Structure  

```
patch-ts/                           # repository root
├── Cargo.toml                      # virtual workspace manifest (no [package])
├── Cargo.lock
├── README.md
├── justfile
├── .github/
├── crates/
│   ├── patch-ts-core/              # Domain logic: AST, matching, repair, semdiff, file system traits
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── ast/
│   │       │   ├── mod.rs
│   │       │   ├── span.rs
│   │       │   ├── language.rs     # Language trait
│   │       │   ├── entity.rs       # Entity struct, find_all_entities
│   │       │   └── languages/      # One file per supported language
│   │       │       ├── mod.rs
│   │       │       ├── rust.rs
│   │       │       ├── typescript.rs
│   │       │       ├── python.rs
│   │       │       └── ...
│   │       ├── patch/
│   │       │   ├── mod.rs
│   │       │   ├── literal.rs      # apply_literal_patch
│   │       │   ├── diff.rs         # apply_unified_diff
│   │       │   ├── marker.rs       # marker‑based patching
│   │       │   ├── symbol.rs       # symbol‑level patching
│   │       │   └── options.rs      # PatchOptions
│   │       ├── matching/
│   │       │   ├── mod.rs
│   │       │   ├── strategies.rs   # exact, anchor, ellipsis, similarity, fuzzy
│   │       │   └── blocks.rs       # block matching
│   │       ├── repair/
│   │       │   ├── mod.rs
│   │       │   ├── search.rs       # minimum_cost_repair
│   │       │   ├── heal.rs         # heuristic repair engine
│   │       │   └── history.rs      # repair history storage
│   │       ├── semdiff.rs
│   │       ├── crossfile.rs        # call graph & impact analysis
│   │       ├── identifiers.rs      # (renamed from identifier.rs)
│   │       ├── indent.rs
│   │       ├── fs.rs               # FileSystem trait + Std/Virtual impls
│   │       ├── diagnostics.rs      # JSON error types
│   │       └── provenance.rs       # SCITT provenance (moved from root)
│   ├── patch-ts-gate/              # Validation pipeline
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── gate.rs             # stage registry + runners
│   │       ├── stages/
│   │       │   ├── mod.rs
│   │       │   ├── syntax.rs
│   │       │   ├── compile.rs
│   │       │   ├── lsp.rs
│   │       │   ├── semdiff.rs
│   │       │   ├── adversarial.rs
│   │       │   ├── owasp.rs
│   │       │   └── sweep_bench.rs
│   │       ├── score.rs
│   │       ├── review.rs
│   │       ├── evolve.rs
│   │       └── security.rs         # OWASP pattern database
│   ├── patch-ts-cli/               # CLI, TUI, LSP editor integration
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── cli/
│   │       │   ├── mod.rs
│   │       │   ├── types.rs
│   │       │   ├── patch.rs
│   │       │   ├── balance.rs
│   │       │   ├── heal.rs
│   │       │   ├── explain.rs
│   │       │   ├── gate.rs
│   │       │   ├── score.rs
│   │       │   ├── recall.rs
│   │       │   ├── entity.rs
│   │       │   ├── key.rs
│   │       │   ├── impact.rs
│   │       │   ├── mcp.rs
│   │       │   └── ...
│   │       ├── tui.rs
│   │       └── lsp.rs
│   ├── patch-ts-mcp/               # MCP server (stdio + HTTP + gateway)
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── server.rs           # stdio + HTTP server
│   │       ├── handlers.rs         # tool handlers
│   │       ├── resources.rs        # resource handlers
│   │       └── gateway.rs          # policy‑enforced gateway
│   └── patch-ts-wasm/              # WASM bindings (preview)
│       ├── Cargo.toml
│       └── src/
│           └── lib.rs
├── tests/                          # cross‑crate integration tests
│   ├── adversarial_input_tests.rs
│   ├── cli_tests.rs
│   ├── e2e_tests.rs
│   └── fixtures/
├── benches/
│   └── patch_benchmark.rs
└── docs/
    ├── heal-mode.md
    ├── lsp-gate.md
    ├── owasp-security.md
    ├── recall-mode.md
    └── wasm.md
```

### 3.2 Crate Descriptions  

| Crate | Purpose | Dependencies |
|-------|---------|-------------|
| `patch-ts-core` | Domain logic: AST, matching, patching, repair, semantic diff, file system abstraction, SCITT. | No I/O beyond `std::fs`; `FileSystem` trait decouples from concrete implementations. |
| `patch-ts-gate` | Validation pipeline: gate stages, scoring, review, OWASP patterns, adversarial checks. | `patch-ts-core` |
| `patch-ts-cli` | CLI argument parsing, TUI, LSP editor server, command handlers. | `patch-ts-core`, `patch-ts-gate`, `patch-ts-mcp` |
| `patch-ts-mcp` | MCP server (stdio + HTTP), MCP gateway with policy enforcement. | `patch-ts-core`, `patch-ts-cli` (for argument types) |
| `patch-ts-wasm` | WASM bindings for browser‑based usage. | `patch-ts-core` (with `FileSystem` virtual implementation) |

**Circular dependency avoidance:** `patch-ts-mcp` depends on `patch-ts-cli` for argument types – this is intentional because MCP tool handlers reuse the same argument structs. We replace the current `use crate::cli::types::*` with a re‑export path via `patch-ts-cli`.

### 3.3 Trait‑Based Abstractions  

**`FileSystem` (existing, moved to `patch-ts-core`)**  

```rust
pub trait FileSystem: Send + Sync {
    fn read_to_string(&self, path: &Path) -> Result<String>;
    fn write(&self, path: &Path, content: &str) -> Result<()>;
    fn exists(&self, path: &Path) -> bool;
    fn create_dir_all(&self, path: &Path) -> Result<()>;
    fn read_dir(&self, path: &Path) -> Result<Vec<PathBuf>>;
}

pub struct StdFileSystem;     // delegates to std::fs
pub struct VirtualFileSystem; // in‑memory HashMap for WASM
```

All file operations in `patch-ts-core` that currently call `std::fs` directly are updated to accept `&dyn FileSystem` (or a generic parameter). `patch-ts-cli` injects `StdFileSystem` at startup.

**`LspClient` (extracted from current concrete type)**  

```rust
pub trait LspClient {
    fn diagnostics(&mut self, uri: &str, content: &str, language_id: &str) -> Result<Vec<lsp::Diagnostic>>;
    fn shutdown(self) -> Result<()>;
}
```

**`HistoryStore` (new, for recall and repair history)**  

```rust
pub trait HistoryStore {
    fn record(&self, entry: &HistoryEntry) -> Result<()>;
    fn query(&self, error_code: &str, file: &str, limit: usize) -> Result<Vec<HistoryEntry>>;
    fn prune(&self, older_than: chrono::Duration) -> Result<()>;
}
```

Default implementation writes to JSONL files (`.patch‑ts/recall.jsonl`, `.patch‑ts/repair‑history.jsonl`). WASM builds can inject an in‑memory store.

---

## 4. Implementation Plan  

### Phase 1 – Workspace Scaffolding (Sprint 1, 1 week)  

- Create virtual root `Cargo.toml` with `[workspace]` and shared dependencies.  
- Create crate sub‑directories with `Cargo.toml` manifests that inherit from the workspace.  
- Move source files from `src/` into their target crates.  
- Update all `use` paths: `crate::ast` → `patch_ts_core::ast`, etc.  
- Ensure `cargo build --workspace` and `cargo test --workspace` pass.  

### Phase 2 – Module Decomposition (Sprint 2, 1 week)  

- Split `ast.rs` into `ast/span.rs`, `ast/language.rs`, `ast/entity.rs`, and `ast/languages/`.  
- Split `patch.rs` into `patch/literal.rs`, `patch/diff.rs`, `patch/marker.rs`, `patch/symbol.rs`, `patch/options.rs`.  
- Split `matching.rs` into `matching/strategies.rs` and `matching/blocks.rs`.  
- Split `gate.rs` into `gate/stages/` with one file per stage.  
- Extract `mcp.rs` into `mcp/server.rs`, `mcp/handlers.rs`, `mcp/resources.rs`.  

### Phase 3 – Trait Interfaces (Sprint 3, 1 week)  

- Extract `LspClient` trait and provide `StdLspClient` implementation.  
- Extract `HistoryStore` trait and provide `JsonlHistoryStore` implementation.  
- Integrate `FileSystem` trait across all crates; replace direct `std::fs` calls.  
- Add doc comments to all public items.  

### Phase 4 – Testing, Polish & Release (Sprint 4, 1 week)  

- Move integration tests to `tests/` (cross‑crate).  
- Update CI to test `--workspace`.  
- Update `README.md` with architecture diagram and new structure.  
- Bump version to `1.17.0`, tag, and release.  

---

## 5. Quality Gates  

- `cargo build --workspace` clean, zero compiler errors, zero warnings.  
- `cargo test --workspace` all pass.  
- `cargo clippy --workspace -- -D warnings` zero errors.  
- Incremental build time (after a single file change) reduced by ≥30% compared to v1.16.0.  
- All public APIs backward compatible.  
- Documentation comments on all public items in `patch-ts-core`.  

---

## 6. Risks & Mitigation  

| Risk | Likelihood | Impact | Mitigation |
|------|------------|--------|------------|
| Import path changes break compilation | High | High | Use `rust-analyzer` to find/update all paths; commit crate by crate. |
| Circular dependency between MCP and CLI crates | Medium | High | Restructure argument types into a shared `patch‑ts‑types` crate if necessary. |
| Splitting large modules loses git history | Medium | Low | Commit moves and splits separately; use `git mv` for individual file moves. |
| CI pipeline times increase with workspace overhead | Low | Medium | Keep integration tests unified; use `cargo test --workspace`. |

---

## 7. Documentation Checklist  

- [ ] Update `README.md` with workspace architecture diagram and crate descriptions.  
- [ ] Update `CONTRIBUTING.md` with workspace layout and development instructions.  
- [ ] Add `ARCHITECTURE.md` explaining the layered design and trait‑based decoupling.  
- [ ] Update `CHANGELOG.md` with v1.17.0 changes.  

---

*Specification approved for v1.17.0 development.*
