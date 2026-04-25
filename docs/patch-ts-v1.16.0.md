# patch‑ts v1.16.0 — Specification  
**LSP‑Integrated Gate & Agentic Security Hardening**

## 1. Executive Summary  

Building on v1.15.0’s `heal` command, adversarial gate, and pre‑patch validation, **v1.16.0** closes the validation gap by integrating **Language Server Protocol** (LSP) diagnostics into the gate pipeline and hardens patch‑ts against **agentic security threats**.  

The release introduces:  

- **`gate --stages lsp`** — connects to real language servers to catch type errors, cross‑file semantic issues, and compiler warnings that tree‑sitter alone cannot detect.  
- **Shadow Editor** — diffs LSP diagnostics before and after a patch, ensuring no new errors are introduced.  
- **OWASP‑aligned security gates** — `owasp‑tool‑poisoning`, `owasp‑prompt‑injection`, `owasp‑supply‑chain` stages for enterprise compliance.  
- **MCP Safety Wrapper** — a policy enforcement layer between MCP clients and tool execution.  

The result: **AI‑generated patches that are validated as rigorously as human code review**, and **enterprise‑ready security hardening** for AI agent workflows.

---

## 2. Design Goals  

| ID | Goal | Measurement |
|----|------|-------------|
| G1 | LSP diagnostic gate | `gate --stages lsp` reports real language‑server errors/warnings for Rust, TypeScript, Python, Go, JavaScript. |
| G2 | Shadow Editor validation | After a patch, LSP diagnostics are compared; any new errors cause gate failure. |
| G3 | OWASP compliance | `gate --stages owasp‑tool‑poisoning` detects inline instructions in tool outputs; `owasp‑prompt‑injection` scans for adversarial patterns. |
| G4 | MCP Safety Wrapper | Policy enforcement layer blocks unauthorised tool calls based on configurable rules. |
| G5 | Backward compatibility | Existing gates unchanged; LSP/OWASP stages are additive. |
| G6 | No new daemon dependencies | LSP communication uses stdio or TCP sockets; no persistent daemon required. |

---

## 3. New Features  

### 3.1 `gate --stages lsp` — LSP Diagnostic Validation  

**Command:**  
```bash
patch-ts gate --file <FILE> --stages lsp [--lsp-command <CMD>] [--lsp-args <ARGS>]
```

**Behaviour:**  
1. Spawns the appropriate language server for the file’s language (e.g., `rust‑analyzer` for Rust, `pyright` for Python, `typescript‑language‑server` for TypeScript).  
2. Sends `textDocument/didOpen` with the file content, waits for diagnostics.  
3. Collects all published diagnostics (errors, warnings, hints).  
4. Reports them as a `StageResult`.  
5. Kills the language server process.  

**Supported languages (initial):**  

| Language | Default LSP | Fallback |
|----------|------------|----------|
| Rust | `rust‑analyzer` | `rls` |
| TypeScript | `typescript‑language‑server` | `ts‑ls` |
| JavaScript | `typescript‑language‑server` | `tsserver` |
| Python | `pyright‑langserver` | `pylsp` |
| Go | `gopls` | – |

**Detection:**  
- Uses `which` to find the LSP binary; if not found, the stage reports a warning and skips.  
- `--lsp-command` and `--lsp-args` allow explicit override.  

**Example output:**  
```
❌ lsp – 3 errors, 1 warning: src/main.rs:42 type mismatch, src/main.rs:58 unused variable
```

### 3.2 Shadow Editor — Pre/Post LSP Diff  

**Integrated into `patch‑ts patch --validate‑first` and `gate --stages lsp`:**  

- Before applying the patch, snapshot LSP diagnostics for the original file.  
- After applying the patch (in memory, before writing), snapshot diagnostics for the patched file.  
- If any **new** errors appear in the patched diagnostics (compared to original), the gate fails.  

**This prevents patches that "fix one thing but break another" from being applied.**

### 3.3 OWASP‑Aligned Security Gates  

Three new gate stages:

#### `owasp‑tool‑poisoning`  

Scans the **new content** for patterns that indicate hidden instructions or tool poisoning (e.g., `ignore_above:`, `noqa`, `# pragma:`, `eval(`, `exec(`, `<!-- --!>` bypasses). Uses a curated regex set based on OWASP Agentic Top 10 (2026).  

#### `owasp‑prompt‑injection`  

Scans the **full file** after patching for potential prompt injection vectors:  
- Comments containing `system:`, `instructions:`, `ignore previous`, `new instruction`.  
- String literals with suspicious command patterns.  
- Hidden Unicode bidirectional characters (Trojan Source attacks).  

#### `owasp‑supply‑chain`  

- Checks if the patch introduces new dependencies (e.g., new `use` statements, new `import` lines).  
- If new dependencies are detected, runs `cargo audit` (Rust), `pip‑audit` (Python), `npm audit` (JavaScript) if available.  
- Verifies that the provenance of the patch matches the expected agent/model.  

**Usage:**  
```bash
patch-ts gate --file src/main.rs --stages owasp-tool-poisoning,owasp-prompt-injection,owasp-supply-chain
```

### 3.4 MCP Safety Wrapper  

A new command `patch‑ts mcp‑gateway` that wraps the MCP server with policy enforcement:  

```bash
patch-ts mcp-gateway --port 9090 --policy policy.toml
```

**Policy file (TOML):**  
```toml
[policy]
max_patch_size = 5000         # bytes
allowed_tools = ["patch", "balance", "explain", "recall"]
allowed_files = ["src/**/*.rs", "tests/**/*.rs"]
blocked_patterns = ["rm -rf", "DROP TABLE"]

[policy.owasp]
tool_poisoning_check = true
prompt_injection_check = true
supply_chain_check = false
```

The gateway intercepts every MCP tool call, validates it against the policy, and either forwards or rejects it with an error response.

---

## 4. Technical Design  

### 4.1 LSP Client  

A lightweight LSP client is implemented in `src/lsp_client.rs`.  

**Key design choices:**  
- Uses `std::process::Command` for spawning, stdin/stdout JSON‑RPC communication.  
- Implements only the essential LSP methods: `initialize`, `textDocument/didOpen`, `textDocument/diagnostic`.  
- Uses `serde_json` for message serialisation.  
- No tokio dependency for the client itself (keeps it synchronous and simple).  
- The client is spawned, used, and killed within a single `gate` invocation.  

### 4.2 Security Pattern Database  

A new module `src/security.rs` contains the curated regex patterns for OWASP stages.  

```rust
pub struct SecurityPattern {
    pub category: OwasThreat,
    pub pattern: Regex,
    pub description: &'static str,
}

pub enum OwasThreat {
    ToolPoisoning,
    PromptInjection,
    SupplyChain,
}
```

### 4.3 MCP Gateway  

A new module `src/mcp_gateway.rs` implements the policy enforcement layer:  

- Reads policy from a TOML file.  
- Wraps the existing `mcp_http` server.  
- On each incoming `tools/call` request, validates:  
  - Tool name is in `allowed_tools`.  
  - Target file matches `allowed_files` glob.  
  - Patch content does not exceed `max_patch_size`.  
  - No `blocked_patterns` are present.  
  - If OWASP checks are enabled, runs the corresponding gate stage.  

---

## 5. Implementation Plan (4 Sprints)  

### Sprint 1 – LSP Client & `gate --stages lsp`  
- Implement `src/lsp_client.rs` with basic JSON‑RPC support.  
- Add `gate_lsp` stage that spawns a language server and collects diagnostics.  
- Support Rust, TypeScript, Python, Go (4 languages).  

### Sprint 2 – Shadow Editor & OWASP Gates  
- Implement LSP diagnostic diff in `gate` and `patch --validate-first`.  
- Create `src/security.rs` with pattern database.  
- Add `owasp‑tool‑poisoning` and `owasp‑prompt‑injection` stages.  

### Sprint 3 – MCP Gateway & Supply Chain Gate  
- Implement `mcp‑gateway` command with policy enforcement.  
- Add `owasp‑supply‑chain` stage with dependency auditing.  
- Wire OWASP checks into the gateway.  

### Sprint 4 – Documentation, Polish & Release  
- Update README, CHANGELOG, `docs/lsp-gate.md`, `docs/owasp-security.md`.  
- Cross‑platform testing.  
- Bump version, tag, release.  

---

## 6. Quality Gates  

- All existing tests pass.  
- `gate --stages lsp` correctly reports diagnostics for Rust, TypeScript, Python, Go on sample files with known errors.  
- Shadow Editor catches new errors introduced by patches.  
- OWASP stages correctly flag known dangerous patterns (unit tests with sample payloads).  
- MCP gateway blocks unauthorised tool calls as specified in policy.  
- No regression in existing gate performance (LSP stage gracefully degrades if no language server is found).  

---

## 7. Documentation Checklist  

- [ ] `README.md` updated with LSP gate, OWASP stages, MCP gateway.  
- [ ] `CHANGELOG.md` entry for v1.16.0.  
- [ ] `docs/lsp-gate.md` – how to use LSP validation and Shadow Editor.  
- [ ] `docs/owasp-security.md` – reference for all OWASP gates and MCP gateway configuration.  

---

*Specification approved for v1.16.0 development.*
