# patch‑ts v1.13.0 – Recall Mode: Token‑Efficient Patch Retry

## 1. Research Synthesis

The proposed feature is well‑grounded in current AI‑agent research and industry practice. Below is a synthesis of the key findings:

### 1.1. The Problem

AI coding agents are **token‑hungry monsters** when editing code. To fix a single line, they often:
1. Read the **entire file** (even 200‑line files for a 1‑line change)
2. Generate **old code** to indicate where the edit goes (94% of output tokens wasted on location, per FastEdit benchmarks)  
3. On failure, **re‑read the entire file** and **re‑generate the full edit**  
4. Accumulate **noisy error traces** across retries, corrupting context

The result: agents burn thousands of tokens on failed attempts, get stuck in **loops repeating the same fix**, and waste significant API costs. Users on the OpenAI developer community report agents taking "2-3 minutes" and burning tokens even for trivial edits.

### 1.2. Key Research Findings

**Amazon CodeStruct (ACL 2026): Structured Action Spaces**
Reframes the codebase so agents operate on **named AST entities** rather than text spans. Provides `readCode` (retrieves complete syntactic units) and `editCode` (applies syntax‑validated transformations). Results: Pass@1 accuracy +1.2–5.0%, token consumption **-12–38%** across six LLMs. Models prone to empty‑patch failures improved by 20.8%. Our existing `entity` and `--symbol` commands already align with this paradigm.

**FastEdit: AST‑Aware Editing**
Uses tree‑sitter to find targets by name – the agent writes **only the change**, not old code. Benchmarks show **44–54% output token savings** across GPT‑5.4, Opus 4.6/4.7, and Grok 4.20. The key insight: *you pay double when the model repeats old lines just to say "find this."*

**trustcall: Patch‑Based Retry**
When validation fails, instead of regenerating the full output, the LLM generates a **concise patch** to fix the specific error. This is both more reliable than naive retry and **cheaper** since only a subset is regenerated. The pattern directly applies to code editing: generate only the delta, not the whole file.

**L‑ICL: Localized Error Correction (arXiv:2602.00276)**
Identifies the **first constraint‑violating step** in a failed trace and injects a **minimal corrective input‑output example** at that precise step. Prevents context corruption from accumulating noisy error traces across retries. Result: valid plan rate raised from 59% to 89%.

**REFINE: Context‑Aware Patch Refinement (arXiv 2510.03588)**
Transforms partially correct "Draft Patches" into correct ones by disambiguating context, diversifying candidates, and aggregating partial fixes. Improved AutoCodeRover by 14.67% on SWE‑Bench Lite, achieving 51.67%.

**ExpeRepair: Dual‑Memory Enhanced Repair (arXiv 2506.10484)**
Organises historical repair experiences into episodic memory (concrete demonstrations) and semantic memory (abstract insights). At inference, retrieves relevant demonstrations and recalls repair insights, replacing static prompts with experience‑driven ones. Achieved 49.3% Pass@1 on SWE‑Bench Lite with Claude 3.7 Sonnet.

**The Recursive Agent Pattern**
An autonomous loop: observe error state → reason about fix → apply fix → re‑evaluate. Captures stderr, parses errors into structured context, generates patches, writes them back, and retries until clean or a retry ceiling is hit.

**Agent Convergence Tracking**
Agents frequently get stuck repeating the same failures – the "agent loop problem." Tracking validation failure signatures can detect when an agent is stuck and signal it to try something different or escalate to human review.

**Diff‑Based Token Management**
PR‑Agent's diff processing pipeline transforms raw diffs into token‑optimised, context‑enriched patches. In testing, diff‑based changes reduced output tokens and inference speed by up to 90%.

### 1.3. Synthesis: The Recall Mode

The proposed **Recall Mode** combines these proven patterns into a single, cohesive feature:

| Research Pattern | How Recall Mode Applies It |
|---|---|
| **CodeStruct** – structured action space | Leverages existing `--symbol` / `entity` targeting; operates on named entities |
| **FastEdit** – zero location tokens | Agent writes only the new code; patch‑ts finds the target |
| **trustcall** – patch‑based retry | On failure, agent generates only the delta, not the full file |
| **L‑ICL** – minimal corrective signal | Injects structured error context at failure point, not full error traces |
| **REFINE** – draft‑to‑correct refinement | Iterative refinement loop with structured feedback |
| **ExpeRepair** – experiential recall | Stores repair experiences for future reuse |
| **Recursive Agent Pattern** – observe‑fix‑verify | Autonomous retry loop with intelligent feedback |

The name **"Recall"** is chosen because it:
- "Recalls" the previous attempt and its error context
- "Recalls" relevant repair strategies from history
- Contrasts with "re‑inserting" (sending the full file again)
- Evokes cognitive memory retrieval – pulling exactly what's needed

---

## 2. Feature Specification

### 2.1. Core Concept

`patch‑ts recall` is a new subcommand that, when a patch fails, **generates a minimal, structured retry context** that the LLM agent can use to fix the error without re‑reading the entire file.

### 2.2. New Subcommand: `patch‑ts recall`

```bash
patch-ts recall \
    --file <FILE> \
    --line <N> \
    --old "<ATTEMPTED_OLD>" \
    --new "<ATTEMPTED_NEW>" \
    --error-code <CODE> \
    [--error-message "<MSG>"] \
    [--context-lines <N>] \
    [--json] \
    [--prompt]
```

**Arguments:**

| Flag | Description |
|---|---|
| `--file` | The file that was being patched |
| `--line` | Target line number of the attempted patch |
| `--old` | The old content the agent tried to match |
| `--new` | The new content the agent tried to insert |
| `--error-code` | Error code from the original failure (E001-E006) |
| `--error-message` | Full error message from the original failure |
| `--context-lines` | Number of surrounding context lines to include (default: 5) |
| `--json` | Output structured JSON for agent consumption |
| `--prompt` | Output a ready‑to‑use prompt for the LLM |
| `--no‑provenance` | Skip provenance recording for the recall |
| `--agent` | Agent name for provenance |
| `--model` | Model name for provenance |

### 2.3. Recall Output Format

The `recall` command produces a **minimal structured context** containing exactly what the LLM needs:

#### JSON Output (`--json`)

```json
{
  "recall_id": "recall-2026-04-24-130000",
  "file": "src/main.rs",
  "attempt": {
    "line": 42,
    "old": "let port = 3000;",
    "new": "let port = 8080;",
    "strategy_used": "exact",
    "confidence": 0.85
  },
  "error": {
    "code": "E002",
    "category": "content_mismatch",
    "message": "expected line 42 to contain 'let port = 3000;' but found 'let port = 3000'",
    "retry_prompt": "The content at line 42 did not match. Expected: 'let port = 3000;'. Found: 'let port = 3000' (missing semicolon). Try matching with --fuzz 2 or anchor on nearby unique lines."
  },
  "context": {
    "surrounding_lines": [
      "    // server config",
      "    let host = \"0.0.0.0\";",
      "    let port = 3000;",     // target
      "    let timeout = 30;",
      "    server.start(host, port);"
    ],
    "symbol": "fn start_server",
    "language": "rs"
  },
  "strategies": [
    {
      "name": "increase_fuzz",
      "description": "Increase --fuzz radius to 3 to handle minor formatting differences"
    },
    {
      "name": "use_anchor",
      "description": "Match on 'let host = \"0.0.0.0\";' and 'let timeout = 30;' as anchors"
    },
    {
      "name": "use_symbol",
      "description": "Use --symbol start_server to target the function directly"
    }
  ],
  "history": {
    "similar_failures": 0,
    "suggested_approach": "anchor"
  }
}
```

#### Prompt Output (`--prompt`)

```
## Patch Retry Context

Your previous patch to `src/main.rs` failed.

### What you attempted:
  Line: 42
  Old: `let port = 3000;`
  New: `let port = 8080;`

### What went wrong:
  Error E002 (content_mismatch):
  "expected line 42 to contain 'let port = 3000;' but found 'let port = 3000'"

### Surrounding context (5 lines around target):
      // server config
      let host = "0.0.0.0";
  →   let port = 3000;
      let timeout = 30;
      server.start(host, port);

### Suggested approach:
  The target line may have formatting differences. Try:
  1. Use `--fuzz 3` to allow fuzzy matching
  2. Use anchor matching on the unique lines `let host = "0.0.0.0"` and `let timeout = 30`
  3. Use `--symbol start_server` to target the function by name

Generate ONLY the corrected patch. Do not repeat the file content.
```

### 2.4. Recall Strategy Database

A new module `src/recall.rs` maintains a **repair strategy database** that maps error patterns to suggested retry approaches:

```rust
pub struct RecallStrategy {
    pub error_code: &'static str,
    pub pattern: &'static str,      // substring match in error message
    pub strategies: Vec<RetryStrategy>,
    pub priority: u8,               // 0 = highest
}

pub enum RetryStrategy {
    IncreaseFuzz { suggested: usize },
    UseAnchor { lines: Vec<String> },
    UseSymbol { name: String },
    UseDiff,
    ForceRepair,
    ExplainFirst,
    BalanceFirst,
}
```

Default strategies:

| Error Code | Pattern | Suggested Strategy |
|---|---|---|
| E001 | file not found | Check path, suggest `--files` glob |
| E002 | content_mismatch | Increase `--fuzz`, use anchor/symbol, check whitespace |
| E003 | confidence_below_threshold | Provide more unique context, use `--symbol` |
| E004 | ambiguous_match | Narrow context, add surrounding lines |
| E005 | syntax_error | Run `patch‑ts balance`, then retry; use `--force` |
| E006 | compilation_error | Show compiler output; suggest `--no-compile-check` for iterative fixing |
| E000 | unknown | Run `patch‑ts explain`, then retry with more specific context |

### 2.5. Recall History

The `.patch‑ts/recall.jsonl` file stores recall records for **experiential learning** (inspired by ExpeRepair):

```json
{
  "recall_id": "recall-2026-04-24-130000",
  "original_attempt": { "file": "...", "line": 42, "old": "...", "new": "..." },
  "error": { "code": "E002", "message": "..." },
  "retry_successful": true,
  "retry_strategy": "anchor",
  "token_saved": 4500,
  "agent": "Claude Code",
  "model": "sonnet",
  "timestamp": "2026-04-24T13:00:00+00:00"
}
```

### 2.6. Integration with Existing Commands

The recall mode can be triggered **automatically** by other commands when a failure occurs:

- `patch‑ts patch --recall-on-failure` – if the patch fails, automatically output a recall context
- `patch‑ts gate --stages syntax,compile --recall-on-failure` – on gate failure, output recall context
- MCP server: new `recall` tool that agents can call after a failed `patch` or `entity_replace` call

### 2.7. MCP Tool

A new MCP tool `recall` is exposed:

```json
{
  "name": "recall",
  "description": "Generate a minimal retry context when a patch fails. Saves tokens by not requiring the file to be re-read.",
  "inputSchema": {
    "type": "object",
    "properties": {
      "file": { "type": "string", "description": "File that was being patched" },
      "line": { "type": "integer" },
      "old": { "type": "string" },
      "new": { "type": "string" },
      "error_code": { "type": "string" },
      "error_message": { "type": "string" },
      "context_lines": { "type": "integer", "default": 5 }
    },
    "required": ["file", "line", "old", "new", "error_code"]
  }
}
```

### 2.8. Token Savings Estimate

Based on FastEdit benchmarks and CodeStruct data:

| Scenario | Without Recall | With Recall | Savings |
|---|---|---|---|
| 200‑line file, 1‑line change, first failure | ~3500 tokens (full file re‑read + re‑generation) | ~500 tokens (recall context only) | **~86%** |
| 500‑line file, 3‑line change, second retry | ~8000 tokens | ~800 tokens | **~90%** |
| Multi‑file change, 2 files failed | ~12000 tokens | ~1500 tokens | **~88%** |

These estimates align with FastEdit's 44‑54% savings for successful first attempts and extreme token compression techniques showing 96% savings for TOON‑based compression. Since recall avoids re‑reading the entire file and only outputs the delta, savings for retries are even higher than for first attempts.

---

## 3. Implementation Plan

### 3.1. New Module: `src/recall.rs`

```
src/recall.rs          # Core recall logic, strategy database, context generation
src/cli/recall.rs      # CLI handler for `patch‑ts recall`
```

Register in `src/cli/mod.rs` and `src/cli/types.rs`.

### 3.2. New Argument Struct

```rust
#[derive(Parser, Debug)]
pub struct RecallArgs {
    #[arg(short, long)]
    pub file: String,
    #[arg(short, long)]
    pub line: usize,
    #[arg(long)]
    pub old: String,
    #[arg(long)]
    pub new: String,
    #[arg(long)]
    pub error_code: String,
    #[arg(long)]
    pub error_message: Option<String>,
    #[arg(long, default_value = "5")]
    pub context_lines: usize,
    #[arg(long)]
    pub json: bool,
    #[arg(long)]
    pub prompt: bool,
    #[arg(long)]
    pub no_provenance: bool,
    #[arg(long)]
    pub agent: Option<String>,
    #[arg(long)]
    pub model: Option<String>,
}
```

### 3.3. Recall Context Generator

```rust
pub struct RecallContext {
    pub recall_id: String,
    pub file: String,
    pub attempt: AttemptInfo,
    pub error: ErrorInfo,
    pub context: ContextInfo,
    pub strategies: Vec<StrategyInfo>,
    pub history: HistoryInfo,
}

pub fn generate_recall_context(
    file_path: &Path,
    line: usize,
    old_content: &str,
    new_content: &str,
    error_code: &str,
    error_message: Option<&str>,
    context_lines: usize,
    language: &mut dyn Language,
) -> Result<RecallContext> {
    // Read file, extract surrounding context
    // Identify containing symbol (function/class)
    // Match error code to strategy database
    // Query recall history for similar failures
    // Generate structured recall context
}
```

### 3.4. Strategy Database

```rust
static RECALL_STRATEGIES: Lazy<Vec<RecallStrategy>> = Lazy::new(|| {
    vec![
        RecallStrategy {
            error_code: "E002",
            pattern: "content_mismatch",
            strategies: vec![
                RetryStrategy::IncreaseFuzz { suggested: 3 },
                RetryStrategy::UseAnchor { lines: vec![] },
                RetryStrategy::UseSymbol { name: String::new() },
            ],
            priority: 0,
        },
        // ... more strategies
    ]
});
```

### 3.5. CLI Integration

- `Command::Recall(args) => recall::handle_recall(args)` added to dispatch
- `PatchArgs` gains `--recall-on-failure` flag
- `GateArgs` gains `--recall-on-failure` flag

### 3.6. MCP Integration

- New `recall` tool added to `tools/list`
- New `recall` handler function that calls `generate_recall_context`

---

## 4. Quality Gates

- All existing tests pass
- New unit tests for strategy matching
- Integration test: simulate a failed patch, verify recall output
- Token count comparison (manual verification)
- JSON output schema validation
- Prompt output human‑readable test
- MCP recall tool returns valid JSON

---

## 5. Timeline

| Sprint | Deliverable |
|---|---|
| Sprint 1 | `src/recall.rs` core logic, strategy database, JSON output |
| Sprint 2 | CLI handler, `--prompt` output, `--recall-on-failure` flag |
| Sprint 3 | MCP integration, recall history, provenance |
| Sprint 4 | Documentation, testing, polish, release |

**Estimated total:** ~4 weeks (1 developer)

---

## 6. Competitive Advantage

No existing tool provides a dedicated **recall/retry context generator**:

- **Claude Code** / **Cursor**: Retry by resending full files; agents waste tokens on loops
- **Aider**: Uses diff‑based editing but retries are naive
- **SWE‑agent**: No recall mechanism; full context re‑reads
- **OpenHands**: Has retry but without structured error context
- **trustcall**: Only for JSON patch operations, not code editing

**patch‑ts recall** would be the **first tool** to combine AST‑aware retry context, experiential recall, and structured error injection into a single command for AI code agents.

---

*Specification ready for implementation.*
