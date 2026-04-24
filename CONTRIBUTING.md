# Contributing to patch‑ts

Thank you for your interest in contributing to **patch‑ts**! We’re excited to have you join the community. This document outlines the guidelines, workflows, and best practices to make contributing as smooth as possible.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Setup](#development-setup)
- [Project Structure](#project-structure)
- [Workflow](#workflow)
- [Code Style](#code-style)
- [Testing](#testing)
- [Pull Requests](#pull-requests)
- [Reporting Bugs](#reporting-bugs)
- [Feature Requests](#feature-requests)
- [License](#license)

---

## Code of Conduct

We follow the [Rust Code of Conduct](https://www.rust-lang.org/policies/code-of-conduct).  
Be kind, respectful, and constructive. Harassment of any kind will not be tolerated.

---

## Getting Started

1. **Fork** the repository on [GitHub](https://github.com/elcoosp/patch-ts).
2. **Clone** your fork locally:
   ```bash
   git clone https://github.com/your-username/patch-ts.git
   cd patch-ts
   ```
3. **Install dependencies** (you need Rust and `just`):
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   brew install just       # macOS
   # or: cargo install just
   ```
4. **Build** the project:
   ```bash
   cargo build
   ```

---

## Development Setup

We use a [`justfile`](justfile) for common tasks. Run `just` to see all available recipes:

- `just build` – Compile the project.
- `just test` – Run the full test suite.
- `just test-verbose` – Run tests with output (`--nocapture`).
- `just fmt` – Format code with `rustfmt`.
- `just lint` – Lint with `clippy` (`-D warnings`).
- `just bench` – Run benchmarks (requires nightly toolchain).
- `just run -- <args>` – Run the binary with arguments.
- `just install` – Install the binary locally via `cargo install --path .`.

We recommend setting up a pre‑commit hook that runs `just fmt && just lint` to keep the codebase clean.

---

## Project Structure

```
.
├── src/
│   ├── ast.rs              # AST traversal, Language trait, entity definitions
│   ├── patch.rs            # Core patching logic (literal, diff, marker, symbol)
│   ├── matching.rs         # Cascade matching strategies
│   ├── repair/             # Delimiter balancing and search
│   ├── gate.rs             # Validation gate pipeline
│   ├── cli/                # CLI argument parsing and command handlers
│   │   ├── types.rs        # All argument structs
│   │   ├── mod.rs          # Dispatch table
│   │   ├── entity.rs       # Entity subcommand
│   │   ├── key.rs          # Key management subcommand
│   │   └── ...
│   ├── mcp.rs              # MCP server (stdio + HTTP)
│   ├── tui.rs              # Terminal UI with side‑by‑side view
│   ├── scitt.rs            # SCITT provenance and signing
│   ├── semdiff.rs          # Semantic diff engine
│   ├── highlight.rs        # Syntax highlighting via syntect
│   ├── word_diff.rs        # Word‑level diff
│   ├── scorecard.rs        # Terminal scorecard rendering
│   ├── render_md.rs        # Markdown report generation
│   ├── parse_cache.rs      # Incremental parsing cache (experimental)
│   └── ...
├── tests/                  # Integration and unit tests
├── benches/                # Criterion benchmarks
├── justfile                # Task runner recipes
├── Cargo.toml              # Dependencies and metadata
└── README.md               # Project overview
```

---

## Workflow

1. Create a **branch** from `main` for your work:
   ```bash
   git checkout -b feat/my-feature
   ```
2. Make your changes, following the [Code Style](#code-style) and [Testing](#testing) guidelines.
3. **Commit** early and often with descriptive messages.
4. When ready, push your branch and open a **pull request** against `main`.

We squash‑merge PRs to keep a linear history. Please ensure your branch is up to date with `main` before opening the PR.

---

## Code Style

- We use `rustfmt` with the default settings. **Run `just fmt` before committing.**
- Clippy warnings are treated as errors. **Run `just lint` and fix any issues.**
- Prefer `anyhow::Result` for fallible functions; use `thiserror` for library errors.
- Keep functions small and focused. A function should do one thing well.
- Use `serde` derives for serializable types.
- When adding a new CLI subcommand, place it in `src/cli/` and register it in `types.rs` and `mod.rs`.
- Write doc comments (`///`) for public API items.
- Use `#[arg]` attributes for clap argument descriptions.

---

## Testing

All changes must pass the full test suite. We use:

- **Unit tests** – inline `#[cfg(test)]` modules in source files.
- **Integration tests** – under `tests/` using `assert_cmd` and `tempfile`.
- **Property‑based tests** – `proptest` in `tests/proptest_repair.rs`.
- **Benchmarks** – `criterion` in `benches/patch_benchmark.rs`.

### Run Tests

```bash
just test
```

If you add a new feature, **add tests that cover the happy path, error conditions, and edge cases**. For CLI commands, use `assert_cmd::Command::cargo_bin("patch-ts")` to invoke the binary.

---

## Pull Requests

- Provide a **clear title** and **description** of what the PR does and why.
- Reference any related issues (`Fixes #123`).
- Ensure **all tests pass** and **clippy is clean** (`just lint`).
- If your change adds a new dependency, justify it in the PR description.
- Keep PRs focused – one feature or fix per PR. Large changes can be split into smaller, reviewable chunks.
- A maintainer will review your PR. Address feedback promptly and keep the discussion constructive.

---

## Reporting Bugs

Open a [GitHub Issue](https://github.com/elcoosp/patch-ts/issues) with:

- A clear, descriptive title.
- Steps to reproduce the bug.
- Expected vs actual behaviour.
- Your OS, Rust version (`rustc --version`), and patch‑ts version (`patch-ts --version`).
- Any relevant error messages or logs.

---

## Feature Requests

We love ideas! Before opening a feature request, check existing issues to see if it’s already being discussed.  
Describe the problem you’re trying to solve and, if possible, suggest an approach.  
For larger features, consider opening a **discussion** first to get feedback from maintainers and the community.

---

## License

By contributing, you agree that your contributions will be licensed under the MIT License (the same as the project).  
See [LICENSE](LICENSE) for details.

---

*Thank you for helping make patch‑ts better!*
