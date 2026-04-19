use clap::{Parser, Subcommand};
use anyhow::Result;
use std::path::Path;
use std::io::Read;

use crate::ast::RustLanguage;
use crate::diagnostics::JsonDiagnostic;
use crate::file::FileManager;
use crate::patch::{apply_literal_patch, apply_unified_diff, delete_line, insert_lines, PatchOptions};
use crate::repair::{balance_file, explain_error};

#[derive(Parser)]
#[command(name = "patch-ts", about = "Tree-sitter-aware patching tool for LLM agents")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Apply a patch to a file
    Patch(PatchArgs),
    /// Detect and fix unbalanced delimiters
    Balance(BalanceArgs),
    /// Explain syntax errors at a given line
    Explain(ExplainArgs),
}

#[derive(Parser, Debug)]
pub struct PatchArgs {
    /// Path to the source file
    #[arg(short, long)]
    pub file: String,

    /// Target line number (1-indexed)
    #[arg(short, long, required_unless_present = "diff")]
    pub line: Option<usize>,

    /// Search radius for fuzzy line matching
    #[arg(short = 'z', long, default_value = "5")]
    pub fuzz: usize,

    /// Expected content (single line; alternative to heredoc)
    #[arg(long, requires = "new")]
    pub old: Option<String>,

    /// New content (single line; alternative to heredoc)
    #[arg(long, requires = "old")]
    pub new: Option<String>,

    /// Read patch from stdin as unified diff
    #[arg(long, conflicts_with = "line")]
    pub diff: bool,

    /// Delete a line after verifying content
    #[arg(long, conflicts_with_all = ["line", "diff"])]
    pub delete: Option<usize>,

    /// Expected content for delete operation
    #[arg(long, requires = "delete")]
    pub expect: Option<String>,

    /// Insert content after this line
    #[arg(long, conflicts_with_all = ["line", "diff", "delete"])]
    pub after: Option<usize>,

    /// Content to insert (single line or heredoc)
    #[arg(long, requires = "after")]
    pub content: Option<String>,

    /// Preview changes without modifying file
    #[arg(long)]
    pub dry_run: bool,

    /// Skip AST validation
    #[arg(long)]
    pub force: bool,

    /// Do not create backup file
    #[arg(long)]
    pub no_backup: bool,

    /// Output JSON diagnostics instead of human-readable
    #[arg(long)]
    pub json: bool,
}

#[derive(Parser, Debug)]
pub struct BalanceArgs {
    /// Path to the source file
    #[arg(short, long)]
    pub file: String,

    /// Limit balancing to specific function
    #[arg(long)]
    pub function: Option<String>,

    /// Apply the fix (default is dry-run)
    #[arg(long)]
    pub apply: bool,

    /// Do not create backup file
    #[arg(long)]
    pub no_backup: bool,

    /// Output JSON diagnostics instead of human-readable
    #[arg(long)]
    pub json: bool,
}

#[derive(Parser, Debug)]
pub struct ExplainArgs {
    /// Path to the source file
    #[arg(short, long)]
    pub file: String,

    /// Line number to explain
    #[arg(short, long)]
    pub line: usize,

    /// Output JSON diagnostics instead of human-readable
    #[arg(long)]
    pub json: bool,
}

pub fn run() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::Patch(args) => handle_patch(args),
        Command::Balance(args) => handle_balance(args),
        Command::Explain(args) => handle_explain(args),
    }
}

fn handle_patch(args: PatchArgs) -> Result<()> {
    let mut lang = RustLanguage::new();
    let _manager = FileManager::new(!args.no_backup);
    let options = PatchOptions {
        fuzz_radius: args.fuzz,
        dry_run: args.dry_run,
        force: args.force,
        no_backup: args.no_backup,
        similarity_threshold: 0.9,
    };

    let file_path = Path::new(&args.file);

    if args.diff {
        let mut buffer = String::new();
        std::io::stdin().read_to_string(&mut buffer)?;
        apply_unified_diff(file_path, &buffer, options)?;
    } else if let Some(line) = args.delete {
        let expected = args.expect.as_deref().ok_or_else(|| anyhow::anyhow!("--expect required with --delete"))?;
        delete_line(file_path, line, expected, options)?;
    } else if let Some(after) = args.after {
        let content = args.content.as_deref().ok_or_else(|| anyhow::anyhow!("--content required with --after"))?;
        insert_lines(file_path, after, content, options)?;
    } else if let (Some(old), Some(new)) = (args.old.as_deref(), args.new.as_deref()) {
        let line = args.line.ok_or_else(|| anyhow::anyhow!("--line required"))?;
        apply_literal_patch(file_path, line, old, new, options, &mut lang)?;
    } else if let Some(line) = args.line {
        let mut buffer = String::new();
        std::io::stdin().read_to_string(&mut buffer)?;
        let (expected, new) = parse_heredoc(&buffer)?;
        apply_literal_patch(file_path, line, &expected, &new, options, &mut lang)?;
    } else {
        anyhow::bail!("No patch operation specified");
    }

    if args.json {
        println!("{}", serde_json::to_string(&JsonDiagnostic::success())?);
    }
    Ok(())
}

fn handle_balance(args: BalanceArgs) -> Result<()> {
    let lang = RustLanguage::new();
    let file_path = Path::new(&args.file);
    balance_file(file_path, args.function.as_deref(), !args.apply, &mut lang)?;
    if args.json {
        println!("{}", serde_json::to_string(&JsonDiagnostic::success())?);
    }
    Ok(())
}

fn handle_explain(args: ExplainArgs) -> Result<()> {
    let lang = RustLanguage::new();
    let file_path = Path::new(&args.file);
    let diag = explain_error(file_path, args.line, args.json, &mut lang)?;
    if let Some(diag) = diag {
        if args.json {
            let json_err = crate::diagnostics::JsonError {
                code: "E0001".to_string(),
                message: diag.to_string(),
                span: crate::diagnostics::JsonSpan {
                    file: args.file.clone(),
                    line: args.line,
                    column: 1,
                },
                context: diag.details.clone(),
                suggestion: Some("Run `patch-ts balance` to attempt automatic fix".to_string()),
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

fn parse_heredoc(input: &str) -> Result<(String, String)> {
    let parts: Vec<&str> = input.split("\n---\n").collect();
    if parts.len() != 2 {
        anyhow::bail!("Heredoc must contain '<<<' expected block, then '---', then new block");
    }
    let expected = parts[0].trim_start_matches("<<<\n").to_string();
    let new = parts[1].to_string();
    Ok((expected, new))
}
