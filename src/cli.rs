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
    Watch(WatchArgs),
    Undo,
    Redo,
    History,
    Lsp,
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
    #[arg(long, conflicts_with_all = ["line", "diff", "delete", "after", "marker"])]
    pub url: Option<String>,
    #[arg(long, conflicts_with_all = ["line", "diff", "delete", "after", "marker"])]
    pub git_commit: Option<String>,
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
    let result = match cli.command {
        Command::Patch(args) => handle_patch(args),
        Command::Balance(args) => handle_balance(args),
        Command::Explain(args) => handle_explain(args),
        Command::Watch(args) => handle_watch(args),
        Command::Undo => handle_undo(),
        Command::Redo => handle_redo(),
        Command::History => handle_history(),
        Command::Lsp => handle_lsp(),
    };

    if let Err(ref e) = result {
        eprintln!("{}", e);
        std::process::exit(1);
    }
    Ok(())
}

fn expand_files(pattern: &str) -> Result<Vec<PathBuf>> {
    let paths: Vec<PathBuf> = glob(pattern)?.filter_map(|entry| entry.ok()).collect();
    if paths.is_empty() {
        anyhow::bail!("No files matched pattern: {}", pattern);
    }
    Ok(paths)
}

fn detect_language(file_path: &Path) -> Result<Box<dyn Language>> {
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

fn apply_patch_to_file(file_path: &Path, args: &PatchArgs) -> Result<()> {
    let mut lang = detect_language(file_path)?;
    let _manager = FileManager::new(!args.no_backup);
    let options = PatchOptions {
        fuzz_radius: args.fuzz,
        dry_run: args.dry_run,
        force: args.force,
        no_backup: args.no_backup,
        similarity_threshold: 0.9,
        no_auto_repair: args.no_auto_repair,
        marker: args.marker.clone(),
        plugin: args.plugin.clone(),
    };

    let original = std::fs::read_to_string(file_path)?;
    if let Some(url) = &args.url {
        let diff_text = crate::remote::fetch_http(url)?;
        apply_unified_diff(file_path, &diff_text, options)?;
    } else if let Some(commit) = &args.git_commit {
        let diff_text = crate::remote::fetch_git_commit(".", commit)?;
        apply_unified_diff(file_path, &diff_text, options)?;
    } else if args.diff {
        let mut buffer = String::new();
        std::io::stdin().read_to_string(&mut buffer)?;
        apply_unified_diff(file_path, &buffer, options)?;
    } else if let Some(line) = args.delete {
        let expected = args.expect.as_deref().ok_or_else(|| anyhow::anyhow!("--expect required"))?;
        delete_line(file_path, line, expected, options)?;
    } else if let Some(after) = args.after {
        let content = args.content.as_deref().ok_or_else(|| anyhow::anyhow!("--content required"))?;
        insert_lines(file_path, after, content, options)?;
    } else if let (Some(old), Some(new)) = (args.old.as_deref(), args.new.as_deref()) {
        let line = args.line.ok_or_else(|| anyhow::anyhow!("--line required"))?;
        apply_literal_patch(file_path, line, old, new, options, &mut *lang)?;
    } else if let Some(line) = args.line {
        let mut buffer = String::new();
        std::io::stdin().read_to_string(&mut buffer)?;
        let (expected, new) = parse_heredoc(&buffer)?;
        apply_literal_patch(file_path, line, &expected, &new, options, &mut *lang)?;
    } else if let Some(marker) = args.marker.as_deref() {
        let new = args.new.as_deref().or(args.content.as_deref()).ok_or_else(|| anyhow::anyhow!("--new or --content required"))?;
        apply_marker_patch(file_path, marker, new, options)?;
    } else {
        anyhow::bail!("No patch operation specified");
    }

    let patched = std::fs::read_to_string(file_path)?;
    if patched != original {
        let manager = HistoryManager::new();
        manager.save(&file_path.to_string_lossy(), &original, &patched)?;
    }

    if args.json {
        println!("{}", serde_json::to_string(&JsonDiagnostic::success())?);
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
                    if msg.contains("no match found") || msg.contains("ambiguous match") {
                        eprintln!("Skipping {}: {}", path.display(), msg);
                        Ok(())
                    } else { Err(e) }
                }
            }
        };
        if args.serial {
            for path in &paths { process(path)?; }
        } else {
            paths.par_iter().try_for_each(|path| process(path))?;
        }
        if !any_success.load(Ordering::Relaxed) {
            anyhow::bail!("No files were successfully patched");
        }
    } else {
        let file_path = Path::new(args.file.as_deref().unwrap());
        apply_patch_to_file(file_path, &args)?;
    }
    Ok(())
}

fn apply_balance_to_file(file_path: &Path, args: &BalanceArgs) -> Result<()> {
    let mut lang = detect_language(file_path)?;
    let result = balance_file(
        file_path,
        args.function.as_deref(),
        !args.apply,
        &mut *lang,
        args.plugin.as_deref(),
        args.max_cost,
    )?;
    if args.json {
        println!("{}", serde_json::to_string(&result)?);
    }
    Ok(())
}

fn handle_balance(args: BalanceArgs) -> Result<()> {
    if let Some(pattern) = &args.files {
        let paths = expand_files(pattern)?;
        if args.serial {
            for path in paths { apply_balance_to_file(&path, &args)?; }
        } else {
            paths.par_iter().try_for_each(|path| apply_balance_to_file(path, &args))?;
        }
    } else {
        let file_path = Path::new(args.file.as_deref().unwrap());
        apply_balance_to_file(file_path, &args)?;
    }
    Ok(())
}

fn handle_explain(args: ExplainArgs) -> Result<()> {
    let file_path = Path::new(&args.file);
    let mut lang = detect_language(file_path)?;
    let diag = explain_error(file_path, args.line, args.json, &mut *lang)?;
    if let Some(diag) = diag {
        if args.json {
            let json_err = JsonError {
                code: "patch_ts::syntax_error".to_string(),
                message: diag.details.clone(),
                span: crate::diagnostics::JsonSpan {
                    file: args.file.clone(),
                    line: args.line,
                    column: 1,
                },
                context: String::new(),
                suggestion: Some("Run `patch-ts balance` to attempt automatic fix".to_string()),
                best_score: None,
                best_match_line: None,
                candidates: None,
            };
            println!("{}", serde_json::to_string(&JsonDiagnostic::error(json_err))?);
        } else {
            eprintln!("{:?}", miette::Report::new(diag));
        }
    } else if args.json {
        println!("{}", serde_json::to_string(&JsonDiagnostic::success())?);
    }
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

fn handle_undo() -> Result<()> {
    let manager = HistoryManager::new();
    match manager.undo_last()? {
        Some(record) => {
            std::fs::write(&record.file, &record.original_content)?;
            println!("Undo applied: file {} restored.", record.file);
        }
        None => {
            eprintln!("No history to undo.");
        }
    }
    Ok(())
}

fn handle_redo() -> Result<()> {
    eprintln!("Redo not yet implemented.");
    Ok(())
}

fn handle_history() -> Result<()> {
    let manager = HistoryManager::new();
    let records = manager.list()?;
    if records.is_empty() {
        println!("No history found.");
    } else {
        for record in &records {
            println!("{} - {}", record.timestamp, record.file);
        }
    }
    Ok(())
}

fn handle_lsp() -> Result<()> {
    let rt = tokio::runtime::Builder::new_current_thread().enable_all().build()?;
    rt.block_on(crate::lsp::run_lsp())?;
    Ok(())
}

fn parse_heredoc(input: &str) -> Result<(String, String)> {
    let parts: Vec<&str> = input.split("\n---\n").collect();
    if parts.len() != 2 {
        anyhow::bail!("Heredoc must contain '<<<' expected block, then '---', then new block");
    }
    Ok((
        parts[0].trim_start_matches("<<<\n").to_string(),
        parts[1].to_string(),
    ))
}
