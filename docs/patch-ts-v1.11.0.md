# patch‑ts v1.11.0 Specification — Human‑Readable Output & Visual Diff

---

## 1. Executive Summary

**patch‑ts v1.10.0** delivered structural capabilities: symbol‑level patching, impact analysis, semantic diffs, and expanded MCP. **v1.11.0** turns outward—toward the agent or developer reading the output. The release makes every command produce **syntax‑highlighted, colour‑optimised, visually structured output** in both CLI and TUI modes, matching or exceeding the readability of tools like `bat`, `delta`, and `difftastic`, but unified under one binary.

The guiding principle: *a human (or an AI agent scanning terminal output) should understand the nature, location, and impact of a code change in under 2 seconds.*

---

## 2. Market Landscape & Technology Trends

| Tool | Strength | Gap patch‑ts can fill |
|------|----------|-----------------------|
| **delta** | Best‑in‑class git diff pager with word‑level highlights, side‑by‑side views, syntax‑highlighting themes | Does not do semantic diffs; does not understand AST structure; no TUI review mode |
| **bat** | Syntax highlighting for `cat` output; 80+ languages; Git change annotations in gutter | Static file viewer; no diff logic; no agent integration |
| **difftastic** | Structural (AST‑based) diff using tree‑sitter; ignores formatting noise; ~35 languages supported | Only a diff viewer; no patching capability; no TUI; no MCP; no gate/score pipeline |
| **hunk** | Modern TUI for reviewing agent‑authored changesets; multi‑file sidebar; inline AI annotations; mouse support | Written in TypeScript (not Rust); no patching; no semantic understanding; limited language coverage |
| **shuire** | Vim‑like TUI for git diff reviewing; syntax highlighting via syntect; inline comments pipeable to AI | Purely a diff viewer; no patching; no gate pipeline; no MCP |
| **inkjet** | Tree‑sitter‑based highlighting library for Rust; batteries‑included; pluggable formatters (HTML, terminal); 70+ languages | Library, not a tool; no diff rendering; no patching |
| **syntect** | Mature, widely‑used Sublime Text syntax engine for Rust; used by `bat`, `shuire`, and many editors | No tree‑sitter integration; only works with Sublime syntax definitions |
| **syntect‑tui** | Translation layer between syntect styles and ratatui `Line`/`Span` types | Bridge library; depends on syntect + ratatui |
| **termimad** | Rich Markdown rendering in terminal; tables, code blocks, wrapping, scrolling | Perfect for rendering patch‑ts dashboards and gate reports |

**Key Insight:** No tool in the Rust ecosystem combines **tree‑sitter‑backed syntax highlighting** with **word‑level diff visualization**, **side‑by‑side views**, and **structured gate/score output** in a single CLI binary. patch‑ts v1.11.0 occupies that position.

---

## 3. Design Goals

| ID | Goal | Measurement |
|----|------|-------------|
| G1 | Syntax‑highlighted output on all CLI commands that display source code | Every `patch‑ts patch`, `explain`, `gate`, `score`, and `review` command outputs colour‑highlighted code |
| G2 | Word‑level diff highlighting within changed lines | Inline diffs show exactly which tokens/words changed, not just whole lines |
| G3 | Side‑by‑side diff view in TUI | `patch‑ts review --tui` shows old and new panes with aligned line numbers |
| G4 | Theme support (light/dark/colourblind) | `--theme` flag with at least 4 bundled themes; use `syntect` or `inkjet` theme infrastructure |
| G5 | Structured Markdown gate reports | `patch‑ts gate --json` also produces a `--markdown` summary suitable for PR comments |
| G6 | Human‑readable score cards | `patch‑ts score` outputs a colour‑coded scorecard with per‑dimension bars |
| G7 | Gutter indicators (added/modified/removed) | CLI and TUI diff views show `+`, `-`, `~` in a left gutter column |

---

## 4. New Features

### 4.1 Syntax Highlighting Engine

**Integration:**  
Add the `syntect` crate (v5.x) or `inkjet` as the highlight engine. `syntect` is recommended based on prior integrations like `syntect‑tui` being actively maintained and used by `shuire`.

**Which commands are affected:**

| Command | Before | After |
|---------|--------|-------|
| `patch‑ts patch` | Plain text diff output only with `--json` | Colour‑highlighted code in terminal; old/new with syntax colours |
| `patch‑ts explain --line N` | Error context in plain text | Syntax‑highlighted source excerpt with error location in red |
| `patch‑ts gate` | Plain text stage results | Colour‑coded pass/fail icons; syntax‑highlighted error snippets |
| `patch‑ts score` | Plain text numeric scores | Colour‑gradient score bars (red→yellow→green) per dimension |
| `patch‑ts review` | Plain text findings | Markdown‑rendered report with colour‑coded agent findings |
| `patch‑ts tui` (existing) | Basic text display, limited highlighting | Full syntax highlighting in both panes; word‑level diff |
| `patch‑ts impact` | Plain text caller list | Syntax‑highlighted file paths with line‑number links |

**Fallback behaviour:**  
When syntect cannot identify a file type, output remains plain text (no regression).

**Theme support:**  
- `--theme` flag accepts: `dark` (default), `light`, `deuteranopia`, `highcontrast`
- Uses syntect's theme loading; bundled themes include `base16-ocean.dark`, `Solarized (light)`, `base16-eighties.dark`, `Monokai Extended`
- Respects `NO_COLOR` and `CLICOLOR=0` environment variables

---

### 4.2 Word‑Level Diff Highlighting

**Problem:** Today, `patch‑ts` shows entire lines as changed (green/red). For long lines, the user cannot see which tokens differ.

**Solution:** Implement a Levenshtein‑based word‑level diff on changed lines, inspired by delta's algorithm.

**How it works:**
1. After identifying changed lines, tokenize each line into words (split on whitespace, punctuation, language tokens via tree‑sitter leaf nodes).
2. Compute minimal edit distance between the token sequences.
3. In output, highlight only the *changed tokens* in each line:
   - Removed words: red background
   - Added words: green background
   - Unchanged words: no background

**Performance:**  
Only runs on changed lines (not the entire diff). For files with hundreds of changes, caching tokenization results keeps overhead under 5ms per diff.

---

### 4.3 Side‑by‑Side Diff View (TUI)

**Current state:** `patch‑ts tui` shows original and patched in two stacked panes.

**v1.11.0 enhancement:**  
Add a **side‑by‑side mode** (toggle with `s` key) where:
- Left pane: original file with line numbers
- Right pane: patched file with line numbers
- Changed lines are vertically aligned
- Word‑level highlights appear within each line
- Scrolling is synchronised

**Implementation:**  
Use ratatui's `Layout::Direction::Horizontal` split with two `Paragraph` widgets. Line numbering uses `LineNumbers` widget. Scroll sync achieved by sharing a `ScrollState` between both panes.

---

### 4.4 Structured Markdown Gate Reports

**New flag:** `--markdown` on `gate`, `review`, and `score` commands.

**Output:** A Markdown document suitable for:
- Pasting into a GitHub PR comment
- Rendering with `termimad` in terminal
- Storing in a file for agent consumption

**Format:**
```
## 🔧 patch‑ts Gate Report

| Stage | Status | Details |
|-------|--------|---------|
| syntax | ✅ Passed | AST valid |
| compile | ❌ Failed | 2 errors |
| semdiff | ✅ Passed | No semantic changes |
| cross‑file | ⚠️ Warning | 3 callers affected |

### Errors
```rust
// syntax‑highlighted error context
```
```

---

### 4.5 Colour‑Coded Score Cards

**Current:** `patch‑ts score` prints plain text numeric scores.

**v1.11.0 enhancement:**  
Output a terminal‑rendered scorecard with:
- A horizontal bar chart for each dimension (syntax, compile, confidence, uniqueness, cross‑file, historical)
- Bar colour gradient: red (<40) → yellow (40‑70) → green (>70)
- Overall score prominently displayed with a large number and colour
- ASCII/Unicode bar characters (`█` or `━`)

**JSON mode unchanged** (backward compatible).

---

### 4.6 Gutter Indicators

**Add a left gutter column** in CLI diff output showing:

| Symbol | Meaning |
|--------|---------|
| `+` | Added line |
| `-` | Removed line |
| `~` | Modified line |
| ` ` | Unchanged context line |

This matches the visual style of GitHub PR diffs. Implementation uses ANSI colour codes directly on the terminal.

---

## 5. Technical Design

### 5.1 Dependency Additions

| Crate | Version | Purpose |
|-------|---------|---------|
| `syntect` | 5.x | Syntax highlighting engine (Sublime Text grammars) |
| `syntect-tui` | 3.x | Syntect‑to‑ratatui style translation |
| `termimad` | 0.20 | Markdown rendering in terminal (for gate reports) |
| `ansi_term` or `yansi` | 0.12 | ANSI colour helpers (already have crossterm; may reuse) |
| `diffs` | 0.5 | Diff algorithm for word‑level change detection |

### 5.2 New Modules

| Module | Purpose |
|--------|---------|
| `src/highlight.rs` | Syntax highlighting engine wrapper; language detection → syntect theme → coloured output |
| `src/word_diff.rs` | Word‑level diff computation using Levenshtein or Myers algorithm |
| `src/render_md.rs` | Markdown rendering for gate reports using termimad |
| `src/scorecard.rs` | Terminal scorecard renderer with colour bars |

### 5.3 Performance Budget

- Syntax highlighting: <10ms per 500‑line file (one‑time parse + highlight)
- Word‑level diff: <5ms per changed line
- Markdown rendering: <2ms per report
- TUI side‑by‑side: no added cost over current TUI
- Memory: syntect grammar set ~2MB loaded once at startup

---

## 6. Changes from v1.10.0

### 6.1 CLI Flag Additions

| Flag | Applies To | Purpose |
|------|-----------|---------|
| `--theme <THEME>` | All commands | Select highlight theme: `dark`, `light`, `deuteranopia`, `highcontrast` |
| `--markdown` | `gate`, `review`, `score` | Output structured Markdown instead of plain text |
| `--no-color` | All | Disable all colour output |
| `--word-diff` | `patch` (TUI), `review` | Enable word‑level diff (default: on) |

### 6.2 Backward Compatibility

- All existing JSON output unchanged
- Plain text output preserved with `--no-color` flag
- Existing CLI flags remain identical
- TUI key bindings unchanged; side‑by‑side is a toggle (`s`), default remains stacked

---

## 7. Implementation Plan (4 Sprints, ~4 Weeks)

### Sprint 1: Syntax Highlighting Foundation
- Integrate `syntect` and load default theme set
- Implement `src/highlight.rs` with `highlight_code(code: &str, lang: &str, theme: &Theme) -> Vec<ColoredSpan>`
- Wire into `patch‑ts explain` and `patch‑ts patch` output
- Add `--theme` flag parsing

### Sprint 2: Word‑Level Diff & Gutter
- Implement Levenshtein token‑level diff in `src/word_diff.rs`
- Add gutter indicators to CLI output
- Integrate word‑level diff into `apply_literal_patch` and `apply_unified_diff` output paths

### Sprint 3: TUI Side‑by‑Side & Scorecards
- Implement side‑by‑side mode in `src/tui.rs`
- Synchronise scrolling between panes
- Build `src/scorecard.rs` with colour‑gradient bars
- Wire `--markdown` flag for gate reports

### Sprint 4: Markdown Reports & Final Polish
- Integrate `termimad` for terminal Markdown rendering
- Build gate report Markdown template
- Documentation: update README, add visual examples
- CI: test with `--no-color` to ensure plain‑text fallback

---

## 8. Quality Gates

- All existing tests pass (no regression)
- `--no-color` output matches plain text baseline
- Syntax highlighting covers all 16 patch‑ts supported languages
- Word‑level diff correctly highlights tokens; tested with 100+ sample diffs
- TUI side‑by‑side scrolls synchronously on files up to 500 lines
- Markdown gate report renders correctly when piped to `termimad` and GitHub
- No warnings, no clippy errors
- Performance benchmarks: highlighting overhead <10ms for 500‑line file

---

## 9. Risks & Mitigation

| Risk | Likelihood | Mitigation |
|------|------------|------------|
| `syntect` grammar loading is slow at startup | Medium | Load grammar set lazily on first use; cache per‑file grammars |
| Word‑diff performance on very large files | Low | Only compute on visible diff hunks; limit to 50 changed lines per pass |
| Terminal colour support varies across terminals | Medium | Detect true‑colour support; fall back to 256‑colour; honour `NO_COLOR` |
| `termimad` rendering clashes with existing TUI style | Low | Use alternate screen; apply separately from ratatui |

---

*Specification approved for v1.11.0 development.*
