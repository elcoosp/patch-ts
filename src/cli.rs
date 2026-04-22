use clap::{Parser, Subcommand};
use anyhow::Result;
use std::path::Path;
use std::io::Read;

use crate::ast::{
    Language, RustLanguage, TypeScriptLanguage, JavaScriptLanguage,
    PythonLanguage, GoLanguage, RubyLanguage, PHPLanguage, HtmlLanguage, XmlLanguage,
    CLanguage, CppLanguage, JavaLanguage, CSharpLanguage,
    KotlinLanguage, SwiftLanguage, ScalaLanguage, ZigLanguage,
};
use crate::diagnostics::{JsonDiagnostic, JsonError, anyhow_to_json};
use crate::file::FileManager;
use crate::patch::{apply_literal_patch, apply_unified_diff, delete_line, insert_lines, apply_marker_patch, PatchOptions};
use crate::repair::{balance_file, explain_error};

#[derive(Parser)]
#[command(name = "patch-ts", about = "Tree-sitter-aware patching tool for LLM agents")]
pub struct Cli { #[command(subcommand)] pub command: Command }

#[derive(Subcommand)]
pub enum Command { Patch(PatchArgs), Balance(BalanceArgs), Explain(ExplainArgs) }

#[derive(Parser, Debug)]
pub struct PatchArgs {
    #[arg(short, long)] pub file: String,
    #[arg(short, long, required_unless_present_any = ["diff", "delete", "after", "marker"])] pub line: Option<usize>,
    #[arg(short = 'z', long, default_value = "5")] pub fuzz: usize,
    #[arg(long)] pub old: Option<String>,
    #[arg(long)] pub new: Option<String>,
    #[arg(long, conflicts_with = "line")] pub diff: bool,
    #[arg(long, conflicts_with_all = ["line", "diff"])] pub delete: Option<usize>,
    #[arg(long, requires = "delete")] pub expect: Option<String>,
    #[arg(long, conflicts_with_all = ["line", "diff", "delete"])] pub after: Option<usize>,
    #[arg(long, requires = "after")] pub content: Option<String>,
    #[arg(long)] pub dry_run: bool,
    #[arg(long)] pub force: bool,
    #[arg(long)] pub no_backup: bool,
    #[arg(long)] pub json: bool,
    #[arg(long)] pub no_auto_repair: bool,
    #[arg(long, conflicts_with = "line")] pub marker: Option<String>,
}

#[derive(Parser, Debug)]
pub struct BalanceArgs {
    #[arg(short, long)] pub file: String,
    #[arg(long)] pub function: Option<String>,
    #[arg(long)] pub apply: bool,
    #[arg(long)] pub no_backup: bool,
    #[arg(long)] pub json: bool,
}

#[derive(Parser, Debug)]
pub struct ExplainArgs {
    #[arg(short, long)] pub file: String,
    #[arg(short, long)] pub line: usize,
    #[arg(long)] pub json: bool,
}

pub fn run() -> Result<()> {
    let cli = Cli::parse();
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
        Some("kt") | Some("kts") => Ok(Box::new(KotlinLanguage::new())),
        Some("swift") => Ok(Box::new(SwiftLanguage::new())),
        Some("scala") => Ok(Box::new(ScalaLanguage::new())),
        Some("zig") => Ok(Box::new(ZigLanguage::new())),
        _ => anyhow::bail!("Unsupported file extension. Supported: .rs, .ts, .tsx, .js, .jsx, .py, .pyi, .go, .rb, .php, .html, .htm, .xml, .c, .h, .cpp, .cc, .cxx, .hpp, .java, .cs, .kt, .kts, .swift, .scala, .zig"),
    }
}

fn handle_patch(args: PatchArgs) -> Result<()> {
    let file_path = Path::new(&args.file);
    let mut lang = detect_language(file_path)?;
    let _manager = FileManager::new(!args.no_backup);
    let options = PatchOptions {
        fuzz_radius: args.fuzz, dry_run: args.dry_run, force: args.force,
        no_backup: args.no_backup, similarity_threshold: 0.9,
        no_auto_repair: args.no_auto_repair, marker: args.marker.clone(),
    };
    if args.diff {
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
    } else if let Some(marker) = args.marker {
        let new = args.new.as_deref().or(args.content.as_deref()).ok_or_else(|| anyhow::anyhow!("--new or --content required"))?;
        apply_marker_patch(file_path, &marker, new, options)?;
    } else { anyhow::bail!("No patch operation specified"); }
    if args.json { println!("{}", serde_json::to_string(&JsonDiagnostic::success())?); }
    Ok(())
}

fn handle_balance(args: BalanceArgs) -> Result<()> {
    let file_path = Path::new(&args.file);
    let mut lang = detect_language(file_path)?;
    let result = balance_file(file_path, args.function.as_deref(), !args.apply, &mut *lang)?;
    if args.json { println!("{}", serde_json::to_string(&result)?); }
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
                span: crate::diagnostics::JsonSpan { file: args.file.clone(), line: args.line, column: 1 },
                context: String::new(),
                suggestion: Some("Run `patch-ts balance` to attempt automatic fix".to_string()),
                best_score: None, best_match_line: None, candidates: None,
            };
            println!("{}", serde_json::to_string(&JsonDiagnostic::error(json_err))?);
        } else { eprintln!("{:?}", miette::Report::new(diag)); }
    } else if args.json { println!("{}", serde_json::to_string(&JsonDiagnostic::success())?); }
    Ok(())
}

fn parse_heredoc(input: &str) -> Result<(String, String)> {
    let parts: Vec<&str> = input.split("\n---\n").collect();
    if parts.len() != 2 { anyhow::bail!("Heredoc must contain '<<<' expected block, then '---', then new block"); }
    Ok((parts[0].trim_start_matches("<<<\n").to_string(), parts[1].to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;
    use crate::ast::{
        RustLanguage, TypeScriptLanguage, JavaScriptLanguage, PythonLanguage, GoLanguage,
        RubyLanguage, PHPLanguage, HtmlLanguage, XmlLanguage, CLanguage, CppLanguage,
        JavaLanguage, CSharpLanguage, KotlinLanguage, SwiftLanguage, ScalaLanguage, ZigLanguage
    };

    #[test] fn test_detect_language_rust() { let mut lang = detect_language(Path::new("main.rs")).unwrap(); assert!(lang.as_any_mut().is::<RustLanguage>()); }
    #[test] fn test_detect_language_typescript() { let mut lang = detect_language(Path::new("app.ts")).unwrap(); assert!(lang.as_any_mut().is::<TypeScriptLanguage>()); }
    #[test] fn test_detect_language_javascript() { let mut lang = detect_language(Path::new("script.js")).unwrap(); assert!(lang.as_any_mut().is::<JavaScriptLanguage>()); }
    #[test] fn test_detect_language_python() { let mut lang = detect_language(Path::new("script.py")).unwrap(); assert!(lang.as_any_mut().is::<PythonLanguage>()); }
    #[test] fn test_detect_language_go() { let mut lang = detect_language(Path::new("main.go")).unwrap(); assert!(lang.as_any_mut().is::<GoLanguage>()); }
    #[test] fn test_detect_language_ruby() { let mut lang = detect_language(Path::new("app.rb")).unwrap(); assert!(lang.as_any_mut().is::<RubyLanguage>()); }
    #[test] fn test_detect_language_php() { let mut lang = detect_language(Path::new("index.php")).unwrap(); assert!(lang.as_any_mut().is::<PHPLanguage>()); }
    #[test] fn test_detect_language_html() { let mut lang = detect_language(Path::new("page.html")).unwrap(); assert!(lang.as_any_mut().is::<HtmlLanguage>()); }
    #[test] fn test_detect_language_xml() { let mut lang = detect_language(Path::new("data.xml")).unwrap(); assert!(lang.as_any_mut().is::<XmlLanguage>()); }
    #[test] fn test_detect_language_c() { let mut lang = detect_language(Path::new("main.c")).unwrap(); assert!(lang.as_any_mut().is::<CLanguage>()); }
    #[test] fn test_detect_language_cpp() { let mut lang = detect_language(Path::new("main.cpp")).unwrap(); assert!(lang.as_any_mut().is::<CppLanguage>()); }
    #[test] fn test_detect_language_java() { let mut lang = detect_language(Path::new("Main.java")).unwrap(); assert!(lang.as_any_mut().is::<JavaLanguage>()); }
    #[test] fn test_detect_language_cs() { let mut lang = detect_language(Path::new("Program.cs")).unwrap(); assert!(lang.as_any_mut().is::<CSharpLanguage>()); }
    #[test] fn test_detect_language_kotlin() { let mut lang = detect_language(Path::new("Main.kt")).unwrap(); assert!(lang.as_any_mut().is::<KotlinLanguage>()); }
    #[test] fn test_detect_language_swift() { let mut lang = detect_language(Path::new("main.swift")).unwrap(); assert!(lang.as_any_mut().is::<SwiftLanguage>()); }
    #[test] fn test_detect_language_scala() { let mut lang = detect_language(Path::new("Main.scala")).unwrap(); assert!(lang.as_any_mut().is::<ScalaLanguage>()); }
    #[test] fn test_detect_language_zig() { let mut lang = detect_language(Path::new("main.zig")).unwrap(); assert!(lang.as_any_mut().is::<ZigLanguage>()); }
    #[test] fn test_detect_language_unknown() { assert!(detect_language(Path::new("file.txt")).is_err()); }
}
