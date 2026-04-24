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
    /// Git-related operations
    Git(GitArgs),
    Watch(WatchArgs),
    Undo,
    Redo,
    History,
    Lsp,
    Mcp,
}

#[derive(Parser, Debug)]
pub struct GitArgs {
    #[command(subcommand)]
    pub action: GitAction,
}

#[derive(Subcommand, Debug)]
pub enum GitAction {
    /// Apply the diff from a specific commit (or commit range) to a file
    Apply(GitApplyArgs),
    /// Generate a diff between the current branch and a target branch
    Diff(GitDiffArgs),
}

#[derive(Parser, Debug)]
pub struct GitApplyArgs {
    /// Commit reference (e.g. abc123, HEAD~1, HEAD~1..HEAD)
    pub commit: String,
    /// Target file path (if omitted, the whole diff is applied to all files)
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
    /// Target branch or commit to diff against (e.g. main, origin/main)
    pub target: String,
    /// Optional specific file to diff
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
        Command::Watch(args) => handle_watch(args),
        Command::Undo => handle_undo(),
        Command::Redo => handle_redo(),
        Command::History => handle_history(),
        Command::Lsp => handle_lsp(),
        Command::Mcp => handle_mcp(),
    };

    if let Err(ref e) = result {
        eprintln!("{}", e);
        std::process::exit(1);
    }
    Ok(())
}

fn expand_files(pattern: &str) -> Result<Vec<PathBuf>> {
    let paths: Vec<PathBuf> = glob(pattern)?.filter_map(|entry| entry.ok()).collect();
    if paths.is_empty() { anyhow::bail!("No files matched pattern: {}", pattern); }
    Ok(paths)
}

pub fn detect_language(file_path: &Path) -> Result<Box<dyn Language>> {
    match file_path.extension().and_then(|e| e.to_str()) {
        Some("rs") => Ok(Box::new(RustLanguage::new())),
        Some("ts") | Some("tsx") | Some("mts") | Some("cts") => Ok(Box::new(TypeScriptLanguage::new())),
        Some("js") | Some("jsx") | Some("mjs") | Some("cjs") => Ok(Box::new(JavaScriptLanguage::new())),
        Some("py") | Some("pyi") => Ok(Box::new(PythonLanguage::new())),
        Some("go") => Ok(Box::new(GoLanguage::new())),
        Some("rb") => Ok(Box::new(RubyLanguage::new())),
        Some("php") => Ok(Box::new(PHPLanguage::new())),
        Some("html") | Some("htm") => Ok(Box::new(HtmlLanguage::new())),
        Some("xml") => Ok(Box::new(XmlLanguage::new())),
        Some("c") | Some("h") => Ok(Box::new(CLanguage::new())),
        Some("cpp") | Some("cc") | Some("cxx") | Some("hpp") => Ok(Box::new(CppLanguage::new())),
        Some("java") => Ok(Box::new(JavaLanguage::new())),
        Some("cs") => Ok(Box::new(CSharpLanguage::new())),
        Some("swift") => Ok(Box::new(SwiftLanguage::new())),
        Some("scala") => Ok(Box::new(ScalaLanguage::new())),
        Some("zig") => Ok(Box::new(ZigLanguage::new())),
        _ => anyhow::bail!("Unsupported file extension."),
    }
}

pub fn apply_patch_to_file(file_path: &Path, args: &PatchArgs) -> Result<()> {
    let mut lang = detect_language(file_path)?;
    let _manager = FileManager::new(!args.no_backup);
    let mut options = PatchOptions {
        fuzz_radius: args.fuzz, dry_run: args.dry_run, force: args.force,
        no_backup: args.no_backup, similarity_threshold: 0.9,
        confidence_threshold: args.confidence, uniqueness_weight: args.uniqueness_weight,
        strict_whitespace: args.strict_whitespace, no_auto_repair: args.no_auto_repair,
        marker: args.marker.clone(), plugin: args.plugin.clone(),
        match_info: None, fix_indent: args.fix_indent,
    };

    let original = std::fs::read_to_string(file_path)?;

    if let Some(url) = &args.url {
        let diff_text = crate::remote::fetch_http(url)?;
        apply_unified_diff(file_path, &diff_text, options.clone())?;
    } else if let Some(commit) = &args.git_commit {
        let diff_text = crate::remote::fetch_git_commit(".", commit)?;
        apply_unified_diff(file_path, &diff_text, options.clone())?;
    } else if args.diff {
        let mut buffer = String::new();
        std::io::stdin().read_to_string(&mut buffer)?;
        let diff_text = if !args.no_sanitize {
            if let Some((_, content)) = crate::sanitize::extract_fenced_block(&buffer) { content.to_string() } else { buffer }
        } else { buffer };
        apply_unified_diff(file_path, &diff_text, options.clone())?;
    } else if let Some(line) = args.delete {
        let expected = args.expect.as_deref().ok_or_else(|| anyhow::anyhow!("--expect required"))?;
        delete_line(file_path, line, expected, options.clone())?;
    } else if let Some(after) = args.after {
        let content = args.content.as_deref().ok_or_else(|| anyhow::anyhow!("--content required"))?;
        insert_lines(file_path, after, content, options.clone())?;
    } else if let (Some(old), Some(new)) = (args.old.as_deref(), args.new.as_deref()) {
        let line = args.line.ok_or_else(|| anyhow::anyhow!("--line required"))?;
        apply_literal_patch(file_path, line, old, new, &mut options, &mut *lang)?;
    } else if let Some(line) = args.line {
        let mut buffer = String::new();
        std::io::stdin().read_to_string(&mut buffer)?;
        let (expected, new) = parse_heredoc(&buffer, args.no_strip_fence, args.no_sanitize)?;
        apply_literal_patch(file_path, line, &expected, &new, &mut options, &mut *lang)?;
    } else if let Some(marker) = args.marker.as_deref() {
        let new = args.new.as_deref().or(args.content.as_deref()).ok_or_else(|| anyhow::anyhow!("--new or --content required"))?;
        apply_marker_patch(file_path, marker, new, options.clone())?;
    } else { anyhow::bail!("No patch operation specified"); }

    let patched = std::fs::read_to_string(file_path)?;
    if !args.no_compile_check {
        let lang_str = file_path.extension().and_then(|e| e.to_str()).unwrap_or("");
        let compile_result = crate::compile::compile_check(file_path, lang_str, args.compile_timeout)?;
        if !compile_result.success {
            std::fs::write(file_path, &original)?;
            let error_msg = compile_result.errors.iter().map(|e| format!("{}:{}:{}: {}", e.file, e.line, e.column, e.message)).collect::<Vec<_>>().join("\n");
            anyhow::bail!("Patch introduced compilation errors:\n{}", error_msg);
        }
    }
    if patched != original {
        let manager = HistoryManager::new();
        manager.save(&file_path.to_string_lossy(), &original, &patched)?;
    }
    let language_name = file_path.extension().and_then(|e| e.to_str()).unwrap_or("");
    let missing = crate::identifier::missing_identifiers(&original, &patched, language_name);
    if !missing.is_empty() { eprintln!("Warning: new identifiers not found in original file: {:?}", missing); }
    if args.json {
        let mut diag = JsonDiagnostic::success();
        if let Some(ref info) = options.match_info { diag.confidence = Some(info.confidence); diag.strategy = Some(info.strategy.clone()); }
        println!("{}", serde_json::to_string(&diag)?);
    }
    Ok(())
}

fn handle_patch(args: PatchArgs) -> Result<()> {
    if let Some(pattern) = &args.files {
        let paths = expand_files(pattern)?;
        let any_success = AtomicBool::new(false);
        let process = |path: &PathBuf| -> Result<(), anyhow::Error> {
            match apply_patch_to_file(path, &args) {
                Ok(()) => { any_success.store(true, Ordering::Relaxed); Ok(()) }
                Err(e) => {
                    let msg = e.to_string();
                    if msg.contains("no match found") || msg.contains("ambiguous match") { eprintln!("Skipping {}: {}", path.display(), msg); Ok(()) } else { Err(e) }
                }
            }
        };
        if args.serial { for path in &paths { process(path)?; } } else { paths.par_iter().try_for_each(|path| process(path))?; }
        if !any_success.load(Ordering::Relaxed) { anyhow::bail!("No files were successfully patched"); }
    } else { let file_path = Path::new(args.file.as_deref().unwrap()); apply_patch_to_file(file_path, &args)?; }
    Ok(())
}

pub fn apply_balance_to_file(file_path: &Path, args: &BalanceArgs) -> Result<()> {
    let mut lang = detect_language(file_path)?;
    let result = balance_file(file_path, args.function.as_deref(), !args.apply, &mut *lang, args.plugin.as_deref(), args.max_cost)?;
    if args.json { println!("{}", serde_json::to_string(&result)?); }
    Ok(())
}

fn handle_git(git_args: GitArgs) -> Result<()> {
    match git_args.action {
        GitAction::Apply(args) => handle_git_apply(args),
        GitAction::Diff(args) => handle_git_diff(args),
    }
}

fn handle_git_apply(args: GitApplyArgs) -> Result<()> {
    let repo = git2::Repository::open(".")?;
    let rev = repo.revparse_single(&args.commit)?;
    let commit = rev.peel_to_commit()?;
    let tree = commit.tree()?;

    let parent = if commit.parent_count() > 0 {
        commit.parent(0)?.tree()?
    } else {
        tree.clone()
    };

    let diff = repo.diff_tree_to_tree(Some(&parent), Some(&tree), None)?;
    let mut diff_text = Vec::new();
    diff.print(git2::DiffFormat::Patch, |_delta, _hunk, line| {
        diff_text.extend_from_slice(line.content());
        true
    })?;
    let diff_str = String::from_utf8(diff_text)?;

    if let Some(file_path) = &args.file {
        let options = PatchOptions {
            fuzz_radius: args.fuzz, dry_run: args.dry_run, force: args.force,
            no_backup: args.no_backup, ..Default::default()
        };
        apply_unified_diff(Path::new(file_path), &diff_str, options)?;
    } else {
        let mut files_to_patch = Vec::new();
        diff.foreach(&mut |delta, _| {
            if let Some(path) = delta.new_file().path() { files_to_patch.push(path.to_path_buf()); }
            true
        }, None, None, None)?;
        for file in &files_to_patch {
            let options = PatchOptions {
                fuzz_radius: args.fuzz, dry_run: args.dry_run, force: args.force,
                no_backup: args.no_backup, ..Default::default()
            };
            apply_unified_diff(file, &diff_str, options.clone())?;
        }
    }
    if args.json { println!("{}", serde_json::to_string(&JsonDiagnostic::success())?); }
    Ok(())
}

fn handle_git_diff(args: GitDiffArgs) -> Result<()> {
    let repo = git2::Repository::open(".")?;
    let target_ref = repo.revparse_single(&args.target)?;
    let target_tree = target_ref.peel_to_tree()?;
    let head_tree = repo.head()?.peel_to_tree()?;

    let diff = repo.diff_tree_to_tree(Some(&target_tree), Some(&head_tree), None)?;
    let mut diff_text = Vec::new();
    diff.print(git2::DiffFormat::Patch, |_delta, _hunk, line| {
        diff_text.extend_from_slice(line.content());
        true
    })?;
    let diff_str = String::from_utf8(diff_text)?;

    if args.apply {
        if let Some(file_path) = &args.file {
            let options = PatchOptions {
                fuzz_radius: args.fuzz, dry_run: args.dry_run, force: args.force,
                no_backup: args.no_backup, ..Default::default()
            };
            apply_unified_diff(Path::new(file_path), &diff_str, options)?;
        } else {
            let mut files_to_patch = Vec::new();
            diff.foreach(&mut |delta, _| {
                if let Some(path) = delta.new_file().path() { files_to_patch.push(path.to_path_buf()); }
                true
            }, None, None, None)?;
            for file in &files_to_patch {
                let options = PatchOptions {
                    fuzz_radius: args.fuzz, dry_run: args.dry_run, force: args.force,
                    no_backup: args.no_backup, ..Default::default()
                };
                apply_unified_diff(file, &diff_str, options.clone())?;
            }
        }
        if args.json { println!("{}", serde_json::to_string(&JsonDiagnostic::success())?); }
    } else {
        print!("{}", diff_str);
    }
    Ok(())
}

fn handle_fix(args: FixArgs) -> Result<()> {
    use std::fs;
    let error_text = if let Some(ref path) = args.error_file { fs::read_to_string(path)? } else {
        let mut buf = String::new(); std::io::stdin().read_to_string(&mut buf)?; buf
    };

    let error = crate::fix::parse_compiler_error(&error_text)
        .ok_or_else(|| anyhow::anyhow!("No fix pattern recognized in the provided error output."))?;

    let target_file = args.file.as_deref().unwrap_or(&error.file);
    let content = if std::path::Path::new(target_file).exists() { fs::read_to_string(target_file)? }
        else { anyhow::bail!("File '{}' not found. Specify --file if different from error output.", target_file) };
    let mut lang = detect_language(Path::new(target_file))?;
    let suggestion = crate::fix::suggest_fix(&error, &content, &mut *lang)
        .ok_or_else(|| anyhow::anyhow!("Could not automatically suggest a fix for this error."))?;

    if args.json { println!("{}", serde_json::to_string(&suggestion)?); }
    else {
        println!("Suggested fix for {}:{}:{}", suggestion.file, suggestion.line, error.error_code);
        println!("  Replace: `{}`", suggestion.old);
        println!("  With:    `{}`", suggestion.new);
        println!("  Confidence: {:.2}", suggestion.confidence);
    }

    if args.apply {
        let patch_args = PatchArgs {
            file: Some(target_file.to_string()), files: None, line: Some(suggestion.line),
            fuzz: 5, old: Some(suggestion.old), new: Some(suggestion.new),
            confidence: 0.9, fix_indent: false, diff: false, delete: None, expect: None,
            after: None, content: None, dry_run: false, force: args.force, no_backup: false,
            json: false, no_auto_repair: false, marker: None, serial: false, plugin: None,
            allow_all_paths: false, url: None, git_commit: None, no_strip_fence: false,
            no_compile_check: false, compile_timeout: 30, no_sanitize: false, no_ellipsis: false,
            uniqueness_weight: 0.2, strict_whitespace: false,
        };
        apply_patch_to_file(Path::new(target_file), &patch_args)?;
        println!("Fix applied to {}", target_file);
    }
    Ok(())
}

fn handle_balance(args: BalanceArgs) -> Result<()> {
    if let Some(pattern) = &args.files {
        let paths = expand_files(pattern)?;
        if args.serial { for path in paths { apply_balance_to_file(&path, &args)?; } } else { paths.par_iter().try_for_each(|path| apply_balance_to_file(path, &args))?; }
    } else { let file_path = Path::new(args.file.as_deref().unwrap()); apply_balance_to_file(file_path, &args)?; }
    Ok(())
}

fn handle_explain(args: ExplainArgs) -> Result<()> {
    let file_path = Path::new(&args.file);
    let mut lang = detect_language(file_path)?;
    let diag = explain_error(file_path, args.line, args.json, &mut *lang)?;
    if let Some(diag) = diag {
        if args.json {
            let json_err = JsonError {
                code: "patch_ts::syntax_error".to_string(), message: diag.details.clone(),
                span: crate::diagnostics::JsonSpan { file: args.file.clone(), line: args.line, column: 1 },
                context: String::new(), suggestion: Some("Run `patch-ts balance` to attempt automatic fix".to_string()),
                best_score: None, best_match_line: None, candidates: None,
                error_code: Some("E005".to_string()), retry_prompt: Some(diag.details.clone()),
            };
            println!("{}", serde_json::to_string(&JsonDiagnostic::error(json_err))?);
        } else { eprintln!("{:?}", miette::Report::new(diag)); }
    } else if args.json { println!("{}", serde_json::to_string(&JsonDiagnostic::success())?); }
    Ok(())
}

fn handle_watch(args: WatchArgs) -> Result<()> {
    use crate::watch::FileWatcher;
    use std::time::Duration;
    let ignore_patterns = args.ignore.unwrap_or_default();
    let delay = Duration::from_millis(args.delay);
    let hooks = args.hooks.as_deref();
    let mut watcher = FileWatcher::new(delay, ignore_patterns, hooks)?;
    watcher.watch(&args.path)?;
    let event = watcher.wait_for_change()?;
    println!("Change detected: {:?}", event.paths);
    Ok(())
}

fn handle_mcp() -> Result<()> { crate::mcp::run_mcp()?; Ok(()) }
fn handle_undo() -> Result<()> {
    let manager = HistoryManager::new();
    match manager.undo_last()? {
        Some(record) => { std::fs::write(&record.file, &record.original_content)?; println!("Undo applied: file {} restored.", record.file); }
        None => { eprintln!("No history to undo."); }
    }
    Ok(())
}
fn handle_redo() -> Result<()> { eprintln!("Redo not yet implemented."); Ok(()) }
fn handle_history() -> Result<()> {
    let manager = HistoryManager::new();
    let records = manager.list()?;
    if records.is_empty() { println!("No history found."); } else { for record in &records { println!("{} - {}", record.timestamp, record.file); } }
    Ok(())
}
fn handle_lsp() -> Result<()> {
    let rt = tokio::runtime::Builder::new_current_thread().enable_all().build()?;
    rt.block_on(crate::lsp::run_lsp())?;
    Ok(())
}

fn parse_heredoc(input: &str, no_strip_fence: bool, no_sanitize: bool) -> Result<(String, String)> {
    let input = input.trim();
    let content = if !no_sanitize {
        if let Some((_, extracted)) = crate::sanitize::extract_fenced_block(input) { extracted.to_string() } else { input.to_string() }
    } else { input.to_string() };
    let content = if !no_strip_fence && content.starts_with("```") && content.ends_with("```") { &content[3..content.len()-3] } else { &content };
    let parts: Vec<&str> = content.split("\n---\n").collect();
    if parts.len() != 2 { anyhow::bail!("Heredoc must contain '<<<' expected block, then '---', then new block"); }
    Ok((parts[0].trim_start_matches("<<<\n").to_string(), parts[1].to_string()))
}
