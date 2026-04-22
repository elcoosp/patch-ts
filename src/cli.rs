use clap::{Parser, Subcommand};
use anyhow::Result;
use std::path::Path;
use std::io::Read;

use crate::ast::RustLanguage;
use crate::diagnostics::{JsonDiagnostic, JsonError, anyhow_to_json};
use crate::file::FileManager;
use crate::patch::{apply_literal_patch, apply_unified_diff, delete_line, insert_lines, apply_marker_patch, PatchOptions};
use crate::repair::{balance_file, explain_error};

#[derive(Parser)]
#[command(
    name = "patch-ts",
    about = "Tree-sitter-aware patching tool for LLM agents",
    after_help = "EXAMPLES:\n  patch-ts patch --file src/lib.rs --line 10 <<'EOF'\n  <<<\n  old line\n  ---\n  new line\n  EOF\n\n  patch-ts balance --file src/lib.rs --apply\n\n  patch-ts explain --file src/lib.rs --line 42"
)]
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
    #[arg(short, long, required_unless_present_any = ["diff", "delete", "after", "marker"])]
    pub line: Option<usize>,

    /// Search radius for fuzzy line matching
    #[arg(short = 'z', long, default_value = "5")]
    pub fuzz: usize,

    /// Expected content (single line; alternative to heredoc)
    #[arg(long)]
    pub old: Option<String>,

    /// New content (single line; alternative to heredoc)
    #[arg(long)]
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

    #[arg(long)]
    pub no_auto_repair: bool,

    /// Target a patch using a marker comment (e.g., // PATCH-ME: id)
    #[arg(long, conflicts_with = "line")]
    pub marker: Option<String>,
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

    // Capture json flag and file before moving command
    let (json, file) = match &cli.command {
        Command::Patch(args) => (args.json, args.file.clone()),
        Command::Balance(args) => (args.json, args.file.clone()),
        Command::Explain(args) => (args.json, args.file.clone()),
    };

    let result = match cli.command {
        Command::Patch(args) => handle_patch(args),
        Command::Balance(args) => handle_balance(args),
        Command::Explain(args) => handle_explain(args),
    };

    if let Err(ref e) = result {
        if json {
            let json_err = anyhow_to_json(e, &file);
            println!("{}", serde_json::to_string(&JsonDiagnostic::error(json_err))?);
            std::process::exit(1);
        }
    }

    result
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
        no_auto_repair: args.no_auto_repair,
        marker: args.marker.clone(),
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
        // Heredoc: read expected and new from stdin
        let mut buffer = String::new();
        std::io::stdin().read_to_string(&mut buffer)?;
        let (expected, new) = parse_heredoc(&buffer)?;
        apply_literal_patch(file_path, line, &expected, &new, options, &mut lang)?;
    } else if let Some(marker) = args.marker {
        let new = args.new.as_deref().or(args.content.as_deref()).ok_or_else(|| anyhow::anyhow!("--new or --content required with --marker"))?;
        apply_marker_patch(file_path, &marker, new, options)?;
    } else {
        anyhow::bail!("No patch operation specified");
    }

    if args.json {
        println!("{}", serde_json::to_string(&JsonDiagnostic::success())?);
    }
    Ok(())
}

fn handle_balance(args: BalanceArgs) -> Result<()> {
    let mut lang = RustLanguage::new();
    let file_path = Path::new(&args.file);
    let result = balance_file(file_path, args.function.as_deref(), !args.apply, &mut lang)?;
    if args.json {
        // If the operation was successful and no JSON was printed internally, print the result
        if result.success {
            println!("{}", serde_json::to_string(&result)?);
        } else {
            // Error already captured in result.error, but we still print the full result
            println!("{}", serde_json::to_string(&result)?);
            // Exit with error code
            if let Some(err) = result.error {
                anyhow::bail!("{}", err.message);
            }
        }
    }
    Ok(())
}

fn handle_explain(args: ExplainArgs) -> Result<()> {
    let mut lang = RustLanguage::new();
    let file_path = Path::new(&args.file);
    let diag = explain_error(file_path, args.line, args.json, &mut lang)?;
    if let Some(diag) = diag {
        if args.json {
            let json_err = JsonError {
                code: "patch_ts::syntax_error".to_string(),
                message: diag.to_string(),
                span: crate::diagnostics::JsonSpan {
                    file: args.file.clone(),
                    line: args.line,
                    column: 1,
                },
                context: diag.details.clone(),
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

fn parse_heredoc(input: &str) -> Result<(String, String)> {
    let parts: Vec<&str> = input.split("\n---\n").collect();
    if parts.len() != 2 {
        anyhow::bail!("Heredoc must contain '<<<' expected block, then '---', then new block");
    }
    let expected = parts[0].trim_start_matches("<<<\n").to_string();
    let new = parts[1].to_string();
    Ok((expected, new))
}
