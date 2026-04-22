# patch-ts v1.0.0 Implementation Plan

> **For agentic workers:** REQUIRED: Use superpowers:subagent-driven-development (if subagents available) or superpowers:executing-plans to implement this plan. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Upgrade patch-ts to a production-ready v1.0.0 with multi-file patch support, parallel processing, a WASM plugin system, and official ecosystem distribution (crates.io, Homebrew, Scoop).

**Architecture:** Extend CLI to accept glob patterns, use `rayon` for parallel multi-file operations with thread-local parsers, integrate `wasmtime` as a sandboxed plugin host with a WIT-defined interface, and set up automated distribution workflows for crates.io, Homebrew, and Scoop.

**Tech Stack:** Rust, tree-sitter 0.26, rayon, glob, wasmtime, wit-bindgen, formulaic, sprinkles-rs.

---

## Chunk 1: Multi-File Support with Glob Patterns

### Task 1.1: Add `glob` dependency and CLI flags

**Files:**
- Modify: `Cargo.toml`
- Modify: `src/cli.rs`

- [ ] **Step 1: Add `glob` to dependencies**

```toml
glob = "0.3"
```

- [ ] **Step 2: Add `--files` and `--serial` flags to `PatchArgs` and `BalanceArgs`**

```rust
// src/cli.rs - in PatchArgs and BalanceArgs
#[arg(long, conflicts_with = "file")]
pub files: Option<String>,

#[arg(long)]
pub serial: bool,
```

- [ ] **Step 3: Write unit test for glob expansion**

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use glob::glob;
    use tempfile::tempdir;

    #[test]
    fn test_glob_expansion() {
        let dir = tempdir().unwrap();
        fs::write(dir.path().join("a.rs"), "").unwrap();
        fs::write(dir.path().join("b.rs"), "").unwrap();
        let pattern = dir.path().join("*.rs").to_str().unwrap().to_string();
        let paths: Vec<_> = glob(&pattern).unwrap().filter_map(Result::ok).collect();
        assert_eq!(paths.len(), 2);
    }
}
```

- [ ] **Step 4: Run build to verify**

```bash
cargo build
```

- [ ] **Step 5: Commit**

```bash
git add Cargo.toml src/cli.rs
git commit -m "feat(cli): add --files glob flag and --serial flag for multi-file operations"
```

---

### Task 1.2: Implement multi-file dispatch in command handlers

**Files:**
- Modify: `src/cli.rs`

- [ ] **Step 1: Refactor `handle_patch` to support `--files`**

```rust
fn handle_patch(args: PatchArgs) -> Result<()> {
    if let Some(pattern) = args.files {
        let paths: Vec<PathBuf> = glob(&pattern)?.filter_map(Result::ok).collect();
        if paths.is_empty() {
            anyhow::bail!("No files matched pattern: {}", pattern);
        }
        for path in paths {
            apply_patch_to_file(&path, &args)?;
        }
    } else {
        let file_path = Path::new(&args.file);
        apply_patch_to_file(file_path, &args)?;
    }
    Ok(())
}

fn apply_patch_to_file(file_path: &Path, args: &PatchArgs) -> Result<()> {
    // existing single-file logic
}
```

- [ ] **Step 2: Similarly refactor `handle_balance`**

- [ ] **Step 3: Add integration test for multi-file patch**

```rust
// tests/multi_file_tests.rs
use assert_cmd::Command;
use tempfile::tempdir;

#[test]
fn test_multi_file_patch() {
    let dir = tempdir().unwrap();
    fs::write(dir.path().join("a.rs"), "fn a() {}\n").unwrap();
    fs::write(dir.path().join("b.rs"), "fn b() {}\n").unwrap();

    let pattern = dir.path().join("*.rs").to_str().unwrap();
    let mut cmd = Command::cargo_bin("patch-ts").unwrap();
    cmd.arg("patch")
        .arg("--files").arg(pattern)
        .arg("--line").arg("1")
        .arg("--old").arg("fn a() {}")
        .arg("--new").arg("fn a_new() {}")
        .assert()
        .success();

    assert!(fs::read_to_string(dir.path().join("a.rs")).unwrap().contains("fn a_new()"));
    assert!(!fs::read_to_string(dir.path().join("b.rs")).unwrap().contains("fn a_new()"));
}
```

- [ ] **Step 4: Run tests**

```bash
cargo test --test multi_file_tests
```

- [ ] **Step 5: Commit**

```bash
git add src/cli.rs tests/multi_file_tests.rs
git commit -m "feat(cli): implement multi-file patch and balance with glob patterns"
```

---

## Chunk 2: Parallel Processing with Rayon

### Task 2.1: Add `rayon` dependency and thread-local parsers

**Files:**
- Modify: `Cargo.toml`
- Modify: `src/cli.rs`
- Modify: `src/repair.rs`

- [ ] **Step 1: Add `rayon` to dependencies**

```toml
rayon = "1.10"
```

- [ ] **Step 2: Modify multi-file dispatch to use parallel iterators**

```rust
// src/cli.rs
use rayon::prelude::*;

if args.serial {
    for path in paths {
        apply_patch_to_file(&path, &args)?;
    }
} else {
    paths.par_iter().try_for_each(|path| {
        apply_patch_to_file(path, &args)
    })?;
}
```

- [ ] **Step 3: Ensure `apply_patch_to_file` creates a new `Language` instance per thread**

```rust
fn apply_patch_to_file(file_path: &Path, args: &PatchArgs) -> Result<()> {
    let mut lang = detect_language(file_path)?; // thread-local parser
    // ...
}
```

- [ ] **Step 4: Write benchmark for parallel vs serial**

```rust
// benches/parallel_benchmark.rs
use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
use patch_ts::*;
use tempfile::tempdir;
use std::fs;

fn bench_parallel_balance(c: &mut Criterion) {
    let dir = tempdir().unwrap();
    for i in 0..10 {
        fs::write(dir.path().join(format!("{}.rs", i)), "fn main() {}\n}\n").unwrap();
    }
    let pattern = dir.path().join("*.rs").to_str().unwrap().to_string();

    c.bench_with_input(BenchmarkId::new("parallel", &pattern), &pattern, |b, pattern| {
        b.iter(|| {
            // run balance with --files pattern
        });
    });
}
```

- [ ] **Step 5: Run tests and benchmarks**

```bash
cargo test
cargo bench --bench parallel_benchmark
```

- [ ] **Step 6: Commit**

```bash
git add Cargo.toml src/cli.rs benches/parallel_benchmark.rs
git commit -m "feat(parallel): add rayon support for multi-file operations"
```

---

## Chunk 3: WASM Plugin System

### Task 3.1: Set up plugin infrastructure with `wasmtime` and `wit-bindgen`

**Files:**
- Create: `crates/plugin-interface/wit/repair.wit`
- Create: `src/plugin.rs`
- Modify: `Cargo.toml`

- [ ] **Step 1: Add `wasmtime` and `wit-bindgen` dependencies**

```toml
wasmtime = "35.0"
wit-bindgen = "0.41"
```

- [ ] **Step 2: Define WIT interface for plugins**

```wit
// crates/plugin-interface/wit/repair.wit
package patch-ts:plugin;

interface repair {
    record span {
        start-byte: u32,
        end-byte: u32,
    }

    variant delimiter-error {
        extra(char, span),
        missing(char, span),
    }

    repair: func(errors: list<delimiter-error>, source: string) -> string;
}

world plugin {
    export repair;
}
```

- [ ] **Step 3: Generate Rust bindings using `wit-bindgen`**

```bash
wit-bindgen rust --out-dir src/plugin_bindings crates/plugin-interface/wit
```

- [ ] **Step 4: Implement plugin host in `src/plugin.rs`**

```rust
use wasmtime::*;
use anyhow::Result;

pub struct PluginHost {
    engine: Engine,
    module: Module,
}

impl PluginHost {
    pub fn load(path: &Path) -> Result<Self> {
        let engine = Engine::default();
        let module = Module::from_file(&engine, path)?;
        Ok(Self { engine, module })
    }

    pub fn repair(&self, errors: &[DelimiterError], source: &str) -> Result<String> {
        let mut store = Store::new(&self.engine, ());
        let instance = Instance::new(&mut store, &self.module, &[])?;
        let repair_fn = instance.get_typed_func::<(String,), (String,)>(&mut store, "repair")?;
        let input = serde_json::to_string(&(errors, source))?;
        let (output,) = repair_fn.call(&mut store, (input,))?;
        Ok(output)
    }
}
```

- [ ] **Step 5: Write a sample plugin in Rust (for testing)**

```rust
// examples/sample_plugin.rs
#[no_mangle]
pub extern "C" fn repair(errors_json: *const u8, len: usize) -> *mut u8 {
    // parse JSON, apply custom repair, return JSON string
}
```

- [ ] **Step 6: Add integration test for plugin loading**

```rust
#[test]
fn test_plugin_loading() {
    let plugin_path = Path::new("tests/fixtures/sample_plugin.wasm");
    let host = PluginHost::load(plugin_path).unwrap();
    let result = host.repair(&[], "source").unwrap();
    assert_eq!(result, "repaired");
}
```

- [ ] **Step 7: Run tests**

```bash
cargo test --test plugin_tests
```

- [ ] **Step 8: Commit**

```bash
git add crates/ src/plugin.rs Cargo.toml tests/plugin_tests.rs examples/
git commit -m "feat(plugin): add WASM plugin system with wasmtime and WIT interface"
```

---

### Task 3.2: Integrate plugins into repair flow

**Files:**
- Modify: `src/repair.rs`
- Modify: `src/cli.rs`

- [ ] **Step 1: Add `--plugin` flag to CLI**

```rust
#[arg(long)]
pub plugin: Option<String>,
```

- [ ] **Step 2: Load specified plugin in `balance_file` and apply its repair logic before built‑in repairs**

```rust
if let Some(plugin_name) = plugin {
    let plugin = load_plugin(&plugin_name)?;
    current_content = plugin.repair(&errors, &current_content)?;
}
```

- [ ] **Step 3: Add test that plugin repair is applied**

- [ ] **Step 4: Run tests**

```bash
cargo test
```

- [ ] **Step 5: Commit**

```bash
git add src/cli.rs src/repair.rs tests/
git commit -m "feat(plugin): integrate WASM plugins into repair flow"
```

---

## Chunk 4: Distribution and Documentation

### Task 4.1: crates.io publication setup

**Files:**
- Modify: `Cargo.toml` (add metadata)
- Create: `.github/workflows/publish.yml`

- [ ] **Step 1: Update `Cargo.toml` with complete metadata**

```toml
[package]
description = "Tree-sitter-backed universal patching CLI for AI agents"
license = "MIT"
repository = "https://github.com/elcoosp/patch-ts"
readme = "README.md"
keywords = ["patching", "tree-sitter", "cli", "ai", "refactoring"]
categories = ["command-line-utilities", "development-tools"]
```

- [ ] **Step 2: Create GitHub Actions workflow for trusted publishing**

```yaml
name: Publish to crates.io
on:
  release:
    types: [published]
jobs:
  publish:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: actions-rs/toolchain@v1
      - run: cargo publish --token ${{ secrets.CARGO_REGISTRY_TOKEN }}
```

- [ ] **Step 3: Commit**

```bash
git add Cargo.toml .github/workflows/publish.yml
git commit -m "chore: prepare for crates.io publication"
```

---

### Task 4.2: Homebrew formula generation with `formulaic`

**Files:**
- Create: `scripts/generate_homebrew_formula.rs`

- [ ] **Step 1: Add `formulaic` as a dev dependency or use it in CI**

```toml
[dev-dependencies]
formulaic = "0.1"
```

- [ ] **Step 2: Write script to generate formula**

```rust
// scripts/generate_homebrew_formula.rs
fn main() {
    let formula = formulaic::Formula::new("patch-ts")
        .description("Universal patching CLI for AI agents")
        .homepage("https://github.com/elcoosp/patch-ts")
        .url("https://github.com/elcoosp/patch-ts/archive/refs/tags/v1.0.0.tar.gz")
        .sha256("...")
        .bin("patch-ts")
        .build();
    std::fs::write("patch-ts.rb", formula.to_string()).unwrap();
}
```

- [ ] **Step 3: Add CI step to generate and push formula to homebrew tap**

- [ ] **Step 4: Commit**

```bash
git add scripts/ .github/workflows/
git commit -m "feat(dist): add Homebrew formula generation"
```

---

### Task 4.3: Scoop manifest generation

**Files:**
- Create: `scripts/generate_scoop_manifest.rs`

- [ ] **Step 1: Use `sprinkles-rs` or manual template**

```rust
// scripts/generate_scoop_manifest.rs
fn main() {
    let manifest = serde_json::json!({
        "version": "1.0.0",
        "url": "https://github.com/elcoosp/patch-ts/releases/download/v1.0.0/patch-ts-x86_64-pc-windows-msvc.zip",
        "bin": "patch-ts.exe",
        "hash": "..."
    });
    std::fs::write("patch-ts.json", manifest.to_string()).unwrap();
}
```

- [ ] **Step 2: Add CI step to push manifest to Scoop bucket**

- [ ] **Step 3: Commit**

```bash
git add scripts/ .github/workflows/
git commit -m "feat(dist): add Scoop manifest generation"
```

---

### Task 4.4: Comprehensive documentation update

**Files:**
- Modify: `README.md`
- Create: `docs/plugin-guide.md`
- Create: `docs/ai-agent-guide.md`

- [ ] **Step 1: Update README with v1.0.0 features (multi‑file, parallel, plugins, installation)**

- [ ] **Step 2: Write plugin development guide**

- [ ] **Step 3: Write AI agent integration guide (prompt examples, JSON output)**

- [ ] **Step 4: Commit**

```bash
git add README.md docs/
git commit -m "docs: update documentation for v1.0.0"
```

---

## Chunk 5: Final Verification and Release

### Task 5.1: Run full test suite and benchmarks

- [ ] **Step 1: `cargo test`**
- [ ] **Step 2: `cargo bench`**
- [ ] **Step 3: `cargo build --release`**
- [ ] **Step 4: Manual smoke test: multi‑file patch, plugin loading**

### Task 5.2: Tag and release

- [ ] **Step 1: Update version in `Cargo.toml` to `1.0.0`**
- [ ] **Step 2: Commit and tag**

```bash
git add Cargo.toml
git commit -m "release: bump version to 1.0.0"
git tag v1.0.0
git push origin main --tags
```

- [ ] **Step 3: Create GitHub Release with binaries attached**

---

**Plan complete.** Each chunk will be delivered as a self-contained bash script using `cat` for file updates and `just test` for verification before committing.
