use anyhow::Result;
use clap::{Parser, Subcommand};
use glob::glob;
use rayon::prelude::*;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};

use crate::ast::{
    CLanguage, CSharpLanguage, CppLanguage, GoLanguage, HtmlLanguage, JavaLanguage,
    JavaScriptLanguage, Language, PHPLanguage, PythonLanguage, RubyLanguage, RustLanguage,
    ScalaLanguage, SwiftLanguage, TypeScriptLanguage, XmlLanguage, ZigLanguage,
};
use crate::diagnostics::{JsonDiagnostic, JsonError};
use crate::file::FileManager;
use crate::history::HistoryManager;
use crate::patch::{
    apply_literal_patch, apply_marker_patch, apply_unified_diff, delete_line, insert_lines,
    PatchOptions,
};
use crate::repair::{balance_file, explain_error};
use crate::semdiff::ChangeType;
use crate::validate;

#[derive(Parser)]
#[command(
    name = "patch-ts",
    about = "Tree-sitter-aware patching tool for LLM agents"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    Patch(PatchArgs),
    Balance(BalanceArgs),
    Explain(ExplainArgs),
    Fix(FixArgs),
    Git(GitArgs),
    SemDiff(SemDiffArgs),
    Provenance(ProvenanceQueryArgs),
    Gate(GateArgs),
    Score(ScoreArgs),
    Index(IndexArgs),
    TraceVerify(TraceArgs),
    Watch(WatchArgs),
    Undo,
    Redo,
    History,
    Lsp,
    Mcp,
    AdaptThreshold,
    AdaptStrategy,
}

#[derive(Parser, Debug)]
pub struct IndexArgs {
    #[arg(long)]
    pub callers: Option<String>,
    #[arg(long)]
    pub rebuild: bool,
    #[arg(long)]
    pub json: bool,
}

#[derive(Parser, Debug)]
pub struct ScoreArgs {
    #[arg(short, long)]
    pub file: String,
    #[arg(long)]
    pub old: String,
    #[arg(long)]
    pub new: String,
    #[arg(long)]
    pub confidence: Option<f64>,
    #[arg(long)]
    pub uniqueness_score: Option<f64>,
    #[arg(long)]
    pub cross_file_impact: Option<usize>,
    #[arg(long)]
    pub json: bool,
}

#[derive(Parser, Debug)]
pub struct GateArgs {
    #[arg(long, default_value = "syntax,compile")]
    pub stages: String,
    #[arg(short, long)]
    pub file: String,
    #[arg(long)]
    pub json: bool,
    #[arg(long, default_value = "true")]
    pub parallel: bool,
    #[arg(long, default_value = "30")]
    pub compile_timeout: u64,
}

#[derive(Parser, Debug)]
pub struct GitArgs {
    #[command(subcommand)]
    pub action: GitAction,
}

#[derive(Subcommand, Debug)]
pub enum GitAction {
    Apply(GitApplyArgs),
    Diff(GitDiffArgs),
}

#[derive(Parser, Debug)]
pub struct GitApplyArgs {
    pub commit: String,
    #[arg(long)]
    pub file: Option<String>,
    #[arg(long)]
    pub dry_run: bool,
    #[arg(long, default_value = "5")]
    pub fuzz: usize,
    #[arg(long)]
    pub force: bool,
    #[arg(long)]
    pub no_backup: bool,
    #[arg(long)]
    pub json: bool,
}

#[derive(Parser, Debug)]
pub struct GitDiffArgs {
    pub target: String,
    #[arg(long)]
    pub file: Option<String>,
    #[arg(long)]
    pub apply: bool,
    #[arg(long)]
    pub dry_run: bool,
    #[arg(long, default_value = "5")]
    pub fuzz: usize,
    #[arg(long)]
    pub force: bool,
    #[arg(long)]
    pub no_backup: bool,
    #[arg(long)]
    pub json: bool,
}

#[derive(Parser, Debug)]
pub struct SemDiffArgs {
    #[arg(long)]
    pub file: Option<String>,
    #[arg(long)]
    pub old: String,
    #[arg(long)]
    pub new: String,
    #[arg(long)]
    pub json: bool,
    #[arg(long)]
    pub tui: bool,
}

#[derive(Parser, Debug)]
pub struct ProvenanceQueryArgs {
    #[arg(long)]
    pub since: Option<String>,
    #[arg(long)]
    pub file: Option<String>,
    #[arg(long)]
    pub output: Option<String>,
    #[arg(long)]
    pub json: bool,
}

#[derive(Parser, Debug)]
pub struct TraceArgs {
    #[arg(long)]
    pub since: Option<String>,
    #[arg(long)]
    pub key: Option<String>,
    #[arg(long)]
    pub json: bool,
}

#[derive(Parser, Debug)]
pub struct PatchArgs {
    #[arg(short, long, required_unless_present = "files")]
    pub file: Option<String>,
    #[arg(long, conflicts_with = "file")]
    pub files: Option<String>,
    #[arg(short, long, required_unless_present_any = ["diff", "delete", "after", "marker", "url", "git_commit"])]
    pub line: Option<usize>,
    #[arg(short = 'z', long, default_value = "5")]
    pub fuzz: usize,
    #[arg(long)]
    pub old: Option<String>,
    #[arg(long)]
    pub new: Option<String>,
    #[arg(long, default_value = "0.9")]
    pub confidence: f64,
    #[arg(long)]
    pub fix_indent: bool,
    #[arg(long, conflicts_with = "line")]
    pub diff: bool,
    #[arg(long, conflicts_with_all = ["line", "diff"])]
    pub delete: Option<usize>,
    #[arg(long, requires = "delete")]
    pub expect: Option<String>,
    #[arg(long, conflicts_with_all = ["line", "diff", "delete"])]
    pub after: Option<usize>,
    #[arg(long, requires = "after")]
    pub content: Option<String>,
    #[arg(long)]
    pub dry_run: bool,
    #[arg(long)]
    pub force: bool,
    #[arg(long)]
    pub no_backup: bool,
    #[arg(long)]
    pub json: bool,
    #[arg(long)]
    pub no_auto_repair: bool,
    #[arg(long, conflicts_with = "line")]
    pub marker: Option<String>,
    #[arg(long)]
    pub serial: bool,
    #[arg(long)]
    pub plugin: Option<String>,
    #[arg(long)]
    pub allow_all_paths: bool,
    #[arg(long, conflicts_with_all = ["line", "diff", "delete", "after", "marker"])]
    pub url: Option<String>,
    #[arg(long, conflicts_with_all = ["line", "diff", "delete", "after", "marker"])]
    pub git_commit: Option<String>,
    #[arg(long)]
    pub no_strip_fence: bool,
    #[arg(long)]
    pub no_compile_check: bool,
    #[arg(long, default_value = "30")]
    pub compile_timeout: u64,
    #[arg(long)]
    pub no_sanitize: bool,
    #[arg(long)]
    pub no_ellipsis: bool,
    #[arg(long, default_value = "0.2")]
    pub uniqueness_weight: f64,
    #[arg(long)]
    pub strict_whitespace: bool,
    #[arg(long)]
    pub cross_file: bool,
    #[arg(long)]
    pub agent: Option<String>,
    #[arg(long)]
    pub model: Option<String>,
    #[arg(long)]
    pub no_provenance: bool,
}

#[derive(Parser, Debug)]
pub struct BalanceArgs {
    #[arg(short, long, required_unless_present = "files")]
    pub file: Option<String>,
    #[arg(long, conflicts_with = "file")]
    pub files: Option<String>,
    #[arg(long)]
    pub function: Option<String>,
    #[arg(long)]
    pub apply: bool,
    #[arg(long)]
    pub no_backup: bool,
    #[arg(long, default_value = "10")]
    pub max_cost: usize,
    #[arg(long)]
    pub json: bool,
    #[arg(long)]
    pub serial: bool,
    #[arg(long)]
    pub plugin: Option<String>,
    #[arg(long)]
    pub allow_all_paths: bool,
}

#[derive(Parser, Debug)]
pub struct FixArgs {
    #[arg(long)]
    pub error_file: Option<String>,
    #[arg(long)]
    pub apply: bool,
    #[arg(long)]
    pub force: bool,
    #[arg(long)]
    pub file: Option<String>,
    #[arg(long)]
    pub json: bool,
}

#[derive(Parser, Debug)]
pub struct WatchArgs {
    #[arg(short, long)]
    pub path: String,
    #[arg(long, default_value = "500")]
    pub delay: u64,
    #[arg(long)]
    pub ignore: Option<Vec<String>>,
    #[arg(long)]
    pub hooks: Option<String>,
}

#[derive(Parser, Debug)]
pub struct ExplainArgs {
    #[arg(short, long)]
    pub file: String,
    #[arg(short, long)]
    pub line: usize,
    #[arg(long)]
    pub json: bool,
}

pub fn run() -> Result<()> {
    let cli = Cli::parse();
    match &cli.command {
        Command::Patch(args) => {
            if let Some(ref file) = args.file { validate::validate_path(file, args.allow_all_paths)?; }
            if let Some(ref old) = args.old { validate::validate_string(old, "--old")?; }
            if let Some(ref new) = args.new { validate::validate_string(new, "--new")?; }
            if let Some(ref expect) = args.expect { validate::validate_string(expect, "--expect")?; }
            if let Some(ref content) = args.content { validate::validate_string(content, "--content")?; }
        }
        Command::Balance(args) => {
            if let Some(ref file) = args.file { validate::validate_path(file, args.allow_all_paths)?; }
        }
        _ => {}
    }
    let result = match cli.command {
        Command::Patch(args) => handle_patch(args),
        Command::Balance(args) => handle_balance(args),
        Command::Explain(args) => handle_explain(args),
        Command::Fix(args) => handle_fix(args),
        Command::Git(git_args) => handle_git(git_args),
        Command::SemDiff(args) => handle_sem_diff(args),
        Command::Provenance(args) => handle_provenance_query(args),
        Command::Gate(args) => handle_gate(args),
        Command::Score(args) => handle_score(args),
        Command::Index(args) => handle_index(args),
        Command::TraceVerify(args) => handle_trace_verify(args),
        Command::Watch(args) => handle_watch(args),
        Command::Undo => handle_undo(),
        Command::Redo => handle_redo(),
        Command::History => handle_history(),
        Command::Lsp => handle_lsp(),
        Command::Mcp => handle_mcp(),
        Command::AdaptThreshold => handle_adapt_threshold(),
        Command::AdaptStrategy => handle_adapt_strategy(),
    };
    if let Err(ref e) = result { eprintln!("{}", e); std::process::exit(1); }
    Ok(())
}

fn expand_files(pattern: &str) -> Result<Vec<PathBuf>> { let paths: Vec<PathBuf> = glob(pattern)?.filter_map(|entry| entry.ok()).collect(); if paths.is_empty() { anyhow::bail!("No files matched pattern: {}", pattern); } Ok(paths) }
pub fn detect_language(file_path: &Path) -> Result<Box<dyn Language>> { match file_path.extension().and_then(|e| e.to_str()) { Some("rs") => Ok(Box::new(RustLanguage::new())), Some("ts")|Some("tsx")|Some("mts")|Some("cts") => Ok(Box::new(TypeScriptLanguage::new())), Some("js")|Some("jsx")|Some("mjs")|Some("cjs") => Ok(Box::new(JavaScriptLanguage::new())), Some("py")|Some("pyi") => Ok(Box::new(PythonLanguage::new())), Some("go") => Ok(Box::new(GoLanguage::new())), Some("rb") => Ok(Box::new(RubyLanguage::new())), Some("php") => Ok(Box::new(PHPLanguage::new())), Some("html")|Some("htm") => Ok(Box::new(HtmlLanguage::new())), Some("xml") => Ok(Box::new(XmlLanguage::new())), Some("c")|Some("h") => Ok(Box::new(CLanguage::new())), Some("cpp")|Some("cc")|Some("cxx")|Some("hpp") => Ok(Box::new(CppLanguage::new())), Some("java") => Ok(Box::new(JavaLanguage::new())), Some("cs") => Ok(Box::new(CSharpLanguage::new())), Some("swift") => Ok(Box::new(SwiftLanguage::new())), Some("scala") => Ok(Box::new(ScalaLanguage::new())), Some("zig") => Ok(Box::new(ZigLanguage::new())), _ => anyhow::bail!("Unsupported file extension.") } }
pub fn apply_patch_to_file(file_path: &Path, args: &PatchArgs) -> Result<()> { let mut lang = detect_language(file_path)?; let _manager = FileManager::new(!args.no_backup); let mut options = PatchOptions { fuzz_radius: args.fuzz, dry_run: args.dry_run, force: args.force, no_backup: args.no_backup, similarity_threshold: 0.9, confidence_threshold: args.confidence, uniqueness_weight: args.uniqueness_weight, strict_whitespace: args.strict_whitespace, no_auto_repair: args.no_auto_repair, marker: args.marker.clone(), plugin: args.plugin.clone(), match_info: None, fix_indent: args.fix_indent }; let original = std::fs::read_to_string(file_path)?; if let Some(url) = &args.url { let diff_text = crate::remote::fetch_http(url)?; apply_unified_diff(file_path, &diff_text, options.clone())?; } else if let Some(commit) = &args.git_commit { let diff_text = crate::remote::fetch_git_commit(".", commit)?; apply_unified_diff(file_path, &diff_text, options.clone())?; } else if args.diff { let mut buffer = String::new(); std::io::stdin().read_to_string(&mut buffer)?; let diff_text = if !args.no_sanitize { if let Some((_, content)) = crate::sanitize::extract_fenced_block(&buffer) { content.to_string() } else { buffer } } else { buffer }; apply_unified_diff(file_path, &diff_text, options.clone())?; } else if let Some(line) = args.delete { let expected = args.expect.as_deref().ok_or_else(|| anyhow::anyhow!("--expect required"))?; delete_line(file_path, line, expected, options.clone())?; } else if let Some(after) = args.after { let content = args.content.as_deref().ok_or_else(|| anyhow::anyhow!("--content required"))?; insert_lines(file_path, after, content, options.clone())?; } else if let (Some(old), Some(new)) = (args.old.as_deref(), args.new.as_deref()) { let line = args.line.ok_or_else(|| anyhow::anyhow!("--line required"))?; apply_literal_patch(file_path, line, old, new, &mut options, &mut *lang)?; } else if let Some(line) = args.line { let mut buffer = String::new(); std::io::stdin().read_to_string(&mut buffer)?; let (expected, new) = parse_heredoc(&buffer, args.no_strip_fence, args.no_sanitize)?; apply_literal_patch(file_path, line, &expected, &new, &mut options, &mut *lang)?; } else if let Some(marker) = args.marker.as_deref() { let new = args.new.as_deref().or(args.content.as_deref()).ok_or_else(|| anyhow::anyhow!("--new or --content required"))?; apply_marker_patch(file_path, marker, new, options.clone())?; } else { anyhow::bail!("No patch operation specified"); } let patched = std::fs::read_to_string(file_path)?; if !args.no_compile_check { let lang_str = file_path.extension().and_then(|e| e.to_str()).unwrap_or(""); let compile_result = crate::compile::compile_check(file_path, lang_str, args.compile_timeout)?; if !compile_result.success { std::fs::write(file_path, &original)?; let error_msg = compile_result.errors.iter().map(|e| format!("{}:{}:{}: {}", e.file, e.line, e.column, e.message)).collect::<Vec<_>>().join("\n"); anyhow::bail!("Patch introduced compilation errors:\n{}", error_msg); } } if patched != original { let manager = HistoryManager::new(); manager.save(&file_path.to_string_lossy(), &original, &patched)?; } if !args.no_provenance { let record = crate::provenance::ProvenanceRecord::new(&file_path.to_string_lossy(), "patch", args.agent.as_deref(), args.model.as_deref(), serde_json::json!({"syntax": true, "compile": true})); crate::provenance::emit_record(&record)?; } let language_name = file_path.extension().and_then(|e| e.to_str()).unwrap_or(""); let missing = crate::identifier::missing_identifiers(&original, &patched, language_name); if !missing.is_empty() { eprintln!("Warning: new identifiers not found in original file: {:?}", missing); } let mut cross_file_warnings = Vec::new(); if args.cross_file { let project_index = crate::crossfile::build_project_index(Path::new(".")); if let (Some(old), Some(new)) = (args.old.as_deref(), args.new.as_deref()) { if old != new { if let Some(old_name) = old.split("fn ").nth(1).and_then(|s| s.split('(').next()) { if let Some(new_name) = new.split("fn ").nth(1).and_then(|s| s.split('(').next()) { if old_name != new_name { let callers = crate::crossfile::find_callers(old_name, &project_index); for caller in callers { cross_file_warnings.push(format!("Function '{}' was renamed to '{}'. Caller found in {} at line {}", old_name, new_name, caller.file, caller.line)); } } } } } } } if args.json { let mut diag = JsonDiagnostic::success(); if let Some(ref info) = options.match_info { diag.confidence = Some(info.confidence); diag.strategy = Some(info.strategy.clone()); } if !cross_file_warnings.is_empty() { diag.warnings = Some(cross_file_warnings); } println!("{}", serde_json::to_string(&diag)?); } else if !cross_file_warnings.is_empty() { eprintln!("Cross‑file warnings:"); for warn in &cross_file_warnings { eprintln!("  {}", warn); } } Ok(()) }
fn handle_patch(args: PatchArgs) -> Result<()> { if let Some(pattern) = &args.files { let paths = expand_files(pattern)?; let any_success = AtomicBool::new(false); let process = |path: &PathBuf| -> Result<(), anyhow::Error> { match apply_patch_to_file(path, &args) { Ok(()) => { any_success.store(true, Ordering::Relaxed); Ok(()) } Err(e) => { let msg = e.to_string(); if msg.contains("no match found") || msg.contains("ambiguous match") { eprintln!("Skipping {}: {}", path.display(), msg); Ok(()) } else { Err(e) } } } }; if args.serial { for path in &paths { process(path)?; } } else { paths.par_iter().try_for_each(|path| process(path))?; } if !any_success.load(Ordering::Relaxed) { anyhow::bail!("No files were successfully patched"); } } else { let file_path = Path::new(args.file.as_deref().unwrap()); apply_patch_to_file(file_path, &args)?; } Ok(()) }
pub fn apply_balance_to_file(file_path: &Path, args: &BalanceArgs) -> Result<()> { let mut lang = detect_language(file_path)?; let result = balance_file(file_path, args.function.as_deref(), !args.apply, &mut *lang, args.plugin.as_deref(), args.max_cost)?; if args.json { println!("{}", serde_json::to_string(&result)?); } Ok(()) }
fn handle_git(git_args: GitArgs) -> Result<()> { match git_args.action { GitAction::Apply(args) => handle_git_apply(args), GitAction::Diff(args) => handle_git_diff(args) } }
fn handle_git_apply(args: GitApplyArgs) -> Result<()> { let repo = git2::Repository::open(".")?; let rev = repo.revparse_single(&args.commit)?; let commit = rev.peel_to_commit()?; let tree = commit.tree()?; let parent = if commit.parent_count() > 0 { commit.parent(0)?.tree()? } else { tree.clone() }; let diff = repo.diff_tree_to_tree(Some(&parent), Some(&tree), None)?; let mut diff_text = Vec::new(); diff.print(git2::DiffFormat::Patch, |_delta, _hunk, line| { diff_text.extend_from_slice(line.content()); true })?; let diff_str = String::from_utf8(diff_text)?; if let Some(file_path) = &args.file { let options = PatchOptions { fuzz_radius: args.fuzz, dry_run: args.dry_run, force: args.force, no_backup: args.no_backup, ..Default::default() }; apply_unified_diff(Path::new(file_path), &diff_str, options)?; } else { let mut files_to_patch = Vec::new(); diff.foreach(&mut |delta, _| { if let Some(path) = delta.new_file().path() { files_to_patch.push(path.to_path_buf()); } true }, None, None, None)?; for file in &files_to_patch { let options = PatchOptions { fuzz_radius: args.fuzz, dry_run: args.dry_run, force: args.force, no_backup: args.no_backup, ..Default::default() }; apply_unified_diff(file, &diff_str, options.clone())?; } } if args.json { println!("{}", serde_json::to_string(&JsonDiagnostic::success())?); } Ok(()) }
fn handle_git_diff(args: GitDiffArgs) -> Result<()> { let repo = git2::Repository::open(".")?; let target_ref = repo.revparse_single(&args.target)?; let target_tree = target_ref.peel_to_tree()?; let head_tree = repo.head()?.peel_to_tree()?; let diff = repo.diff_tree_to_tree(Some(&target_tree), Some(&head_tree), None)?; let mut diff_text = Vec::new(); diff.print(git2::DiffFormat::Patch, |_delta, _hunk, line| { diff_text.extend_from_slice(line.content()); true })?; let diff_str = String::from_utf8(diff_text)?; if args.apply { if let Some(file_path) = &args.file { let options = PatchOptions { fuzz_radius: args.fuzz, dry_run: args.dry_run, force: args.force, no_backup: args.no_backup, ..Default::default() }; apply_unified_diff(Path::new(file_path), &diff_str, options)?; } else { let mut files_to_patch = Vec::new(); diff.foreach(&mut |delta, _| { if let Some(path) = delta.new_file().path() { files_to_patch.push(path.to_path_buf()); } true }, None, None, None)?; for file in &files_to_patch { let options = PatchOptions { fuzz_radius: args.fuzz, dry_run: args.dry_run, force: args.force, no_backup: args.no_backup, ..Default::default() }; apply_unified_diff(file, &diff_str, options.clone())?; } } if args.json { println!("{}", serde_json::to_string(&JsonDiagnostic::success())?); } } else { print!("{}", diff_str); } Ok(()) }
fn handle_sem_diff(args: SemDiffArgs) -> Result<()> { let old_content = std::fs::read_to_string(&args.old)?; let new_content = std::fs::read_to_string(&args.new)?; let lang = if let Some(ref file) = args.file { Path::new(file).extension().and_then(|e| e.to_str()).unwrap_or("rs") } else { Path::new(&args.old).extension().and_then(|e| e.to_str()).unwrap_or("rs") }; let old_entities = crate::semdiff::extract_entities(&old_content, lang).map_err(|e| anyhow::anyhow!("Failed to extract entities from old file: {}", e))?; let new_entities = crate::semdiff::extract_entities(&new_content, lang).map_err(|e| anyhow::anyhow!("Failed to extract entities from new file: {}", e))?; let changes = crate::semdiff::diff_entities(&old_entities, &new_entities); if args.json { println!("{}", serde_json::to_string(&changes)?); } else { for change in &changes { let symbol = match change.change_type { ChangeType::Added => "⊕", ChangeType::Removed => "⊖", ChangeType::Modified => "∆", ChangeType::Moved => "⇢" }; println!("{} {} {}", symbol, change.entity.kind, change.entity.name); } } Ok(()) }
fn handle_provenance_query(args: ProvenanceQueryArgs) -> Result<()> { let records = crate::provenance::query_provenance(args.since.as_deref(), args.file.as_deref())?; if let Some(ref output) = args.output { let jsonl = records.iter().map(|r| serde_json::to_string(r).unwrap()).collect::<Vec<_>>().join("\n"); std::fs::write(output, jsonl)?; } if args.json { println!("{}", serde_json::to_string(&records)?); } else { for record in &records { println!("{} {} {} {} {}", record.timestamp, record.tool, record.file, record.operation, record.agent.as_deref().unwrap_or("unknown")); } } Ok(()) }
fn handle_gate(args: GateArgs) -> Result<()> { let file_path = Path::new(&args.file); let content = std::fs::read_to_string(file_path)?; let stages: Vec<String> = args.stages.split(',').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect(); let result = if args.parallel { crate::gate::run_gate_parallel(&stages, &file_path, &content, &content, args.compile_timeout)? } else { crate::gate::run_gate(&stages, &file_path, &content, &content, args.compile_timeout)? }; if args.json { println!("{}", serde_json::to_string(&result)?); } else { for stage in &result.stages { let status = if stage.passed { "✅" } else { "❌" }; println!("{} {} – {}", status, stage.name, stage.details); } if !result.passed { std::process::exit(1); } } Ok(()) }
fn handle_score(args: ScoreArgs) -> Result<()> { let content = std::fs::read_to_string(&args.file)?; let syntax_valid = { let mut lang = detect_language(Path::new(&args.file))?; let parse_result = lang.parse(&content); lang.is_valid(&parse_result) }; let compile_success = { let result = crate::compile::compile_check(Path::new(&args.file), "rs", 30)?; result.success }; let confidence = args.confidence.unwrap_or(0.9); let uniqueness_score = args.uniqueness_score.unwrap_or(0.5); let cross_file_impact = args.cross_file_impact.unwrap_or(0); let historical_success_rate = crate::score::historical_success_rate(); let ctx = crate::score::ScoreContext { syntax_valid, compile_success, confidence, uniqueness_score, cross_file_impact, historical_success_rate }; let score = crate::score::calculate_score(&ctx); if args.json { println!("{}", serde_json::to_string(&score)?); } else { println!("Overall reliability score: {}/100", score.overall); for (dim, val) in &score.dimensions { println!("  {}: {}/100", dim, val); } } Ok(()) }
fn handle_index(args: IndexArgs) -> Result<()> { let kg = crate::knowledge::build_project_index(Path::new(".")); if let Some(ref symbol) = args.callers { let callers = kg.callers_of(symbol); if args.json { println!("{}", serde_json::to_string(&callers)?); } else { if callers.is_empty() { println!("No callers found for '{}'", symbol); } else { for caller in &callers { println!("{}:{}", caller.caller_file, caller.caller_line); } } } } else { println!("Knowledge graph built with {} symbols and {} call edges.", kg.symbols.len(), kg.call_edges.len()); if args.json { println!("{}", serde_json::to_string(&kg)?); } } Ok(()) }
fn handle_fix(args: FixArgs) -> Result<()> { use std::fs; let error_text = if let Some(ref path) = args.error_file { fs::read_to_string(path)? } else { let mut buf = String::new(); std::io::stdin().read_to_string(&mut buf)?; buf }; let error = crate::fix::parse_compiler_error(&error_text).ok_or_else(|| anyhow::anyhow!("No fix pattern recognized"))?; let target_file = args.file.as_deref().unwrap_or(&error.file); let content = if Path::new(target_file).exists() { fs::read_to_string(target_file)? } else { anyhow::bail!("File '{}' not found", target_file) }; let mut lang = detect_language(Path::new(target_file))?; let suggestion = crate::fix::suggest_fix(&error, &content, &mut *lang).ok_or_else(|| anyhow::anyhow!("Could not auto‑suggest fix"))?; if args.json { println!("{}", serde_json::to_string(&suggestion)?); } else { println!("Suggested fix for {}:{}:{}", suggestion.file, suggestion.line, error.error_code); println!("  Replace: `{}`", suggestion.old); println!("  With:    `{}`", suggestion.new); } if args.apply { let patch_args = PatchArgs { file: Some(target_file.to_string()), files: None, line: Some(suggestion.line), fuzz: 5, old: Some(suggestion.old), new: Some(suggestion.new), confidence: 0.9, fix_indent: false, diff: false, delete: None, expect: None, after: None, content: None, dry_run: false, force: args.force, no_backup: false, json: false, no_auto_repair: false, marker: None, serial: false, plugin: None, allow_all_paths: false, url: None, git_commit: None, no_strip_fence: false, no_compile_check: false, compile_timeout: 30, no_sanitize: false, no_ellipsis: false, uniqueness_weight: 0.2, strict_whitespace: false, cross_file: false, agent: None, model: None, no_provenance: true }; apply_patch_to_file(Path::new(target_file), &patch_args)?; println!("Fix applied to {}", target_file); } Ok(()) }
fn handle_balance(args: BalanceArgs) -> Result<()> { if let Some(pattern) = &args.files { let paths = expand_files(pattern)?; if args.serial { for path in paths { apply_balance_to_file(&path, &args)?; } } else { paths.par_iter().try_for_each(|path| apply_balance_to_file(path, &args))?; } } else { let file_path = Path::new(args.file.as_deref().unwrap()); apply_balance_to_file(file_path, &args)?; } Ok(()) }
fn handle_explain(args: ExplainArgs) -> Result<()> { let file_path = Path::new(&args.file); let mut lang = detect_language(file_path)?; let diag = explain_error(file_path, args.line, args.json, &mut *lang)?; if let Some(diag) = diag { if args.json { let json_err = JsonError { code: "patch_ts::syntax_error".to_string(), message: diag.details.clone(), span: crate::diagnostics::JsonSpan { file: args.file.clone(), line: args.line, column: 1 }, context: String::new(), suggestion: Some("Run `patch-ts balance` to attempt automatic fix".to_string()), best_score: None, best_match_line: None, candidates: None, error_code: Some("E005".to_string()), retry_prompt: Some(diag.details.clone()) }; println!("{}", serde_json::to_string(&JsonDiagnostic::error(json_err))?); } else { eprintln!("{:?}", miette::Report::new(diag)); } } else if args.json { println!("{}", serde_json::to_string(&JsonDiagnostic::success())?); } Ok(()) }
fn handle_watch(args: WatchArgs) -> Result<()> { use crate::watch::FileWatcher; use std::time::Duration; let ignore_patterns = args.ignore.unwrap_or_default(); let delay = Duration::from_millis(args.delay); let hooks = args.hooks.as_deref(); let mut watcher = FileWatcher::new(delay, ignore_patterns, hooks)?; watcher.watch(&args.path)?; let event = watcher.wait_for_change()?; println!("Change detected: {:?}", event.paths); Ok(()) }
fn handle_mcp() -> Result<()> { crate::mcp::run_mcp()?; Ok(()) }
fn handle_undo() -> Result<()> { let manager = HistoryManager::new(); match manager.undo_last()? { Some(record) => { std::fs::write(&record.file, &record.original_content)?; println!("Undo applied: file {} restored.", record.file); } None => { eprintln!("No history to undo."); } } Ok(()) }
fn handle_redo() -> Result<()> { eprintln!("Redo not yet implemented."); Ok(()) }
fn handle_history() -> Result<()> { let manager = HistoryManager::new(); let records = manager.list()?; if records.is_empty() { println!("No history found."); } else { for record in &records { println!("{} - {}", record.timestamp, record.file); } } Ok(()) }
fn handle_lsp() -> Result<()> { let rt = tokio::runtime::Builder::new_current_thread().enable_all().build()?; rt.block_on(crate::lsp::run_lsp())?; Ok(()) }
fn handle_adapt_threshold() -> Result<()> { let threshold = crate::history_adaptive::suggest_threshold(Path::new(".patch‑ts"))?; println!("{:.2}", threshold); Ok(()) }
fn handle_adapt_strategy() -> Result<()> { println!("exact,anchor,anchor_pair,ellipsis,similarity,fuzzy"); Ok(()) }
fn handle_trace_verify(args: TraceArgs) -> Result<()> { let log_path = Path::new(".patch-ts/provenance.jsonl"); let tampered = crate::scitt::verify_log(log_path, None)?; if args.json { println!("{}", serde_json::to_string(&tampered)?); } else { if tampered.is_empty() { println!("All records verified."); } else { println!("Tampered records detected:"); for t in &tampered { println!("  {}", t); } } } Ok(()) }
fn parse_heredoc(input: &str, no_strip_fence: bool, no_sanitize: bool) -> Result<(String, String)> { let input = input.trim(); let content = if !no_sanitize { if let Some((_, extracted)) = crate::sanitize::extract_fenced_block(input) { extracted.to_string() } else { input.to_string() } } else { input.to_string() }; let content = if !no_strip_fence && content.starts_with("```") && content.ends_with("```") { &content[3..content.len()-3] } else { &content }; let parts: Vec<&str> = content.split("\n---\n").collect(); if parts.len() != 2 { anyhow::bail!("Heredoc must contain '<<<' expected block, then '---', then new block"); } Ok((parts[0].trim_start_matches("<<<\n").to_string(), parts[1].to_string())) }
