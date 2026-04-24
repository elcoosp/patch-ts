use anyhow::Result;
use std::io::Read;
use std::path::PathBuf;
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use glob::glob;
use rayon::prelude::*;
use super::types::*;
use crate::diagnostics::JsonDiagnostic;
use crate::file::FileManager;
use crate::history::HistoryManager;
use crate::patch::{
    apply_literal_patch, apply_marker_patch, apply_unified_diff, delete_line, insert_lines,
    PatchOptions, apply_symbol_patch,
};

pub fn expand_files(pattern: &str) -> Result<Vec<PathBuf>> {
    let paths: Vec<PathBuf> = glob(pattern)?.filter_map(|entry| entry.ok()).collect();
    if paths.is_empty() { anyhow::bail!("No files matched pattern: {}", pattern); }
    Ok(paths)
}

pub fn apply_patch_to_file(file_path: &Path, args: &PatchArgs) -> Result<()> {
    let mut lang = match crate::ast::detect_language(file_path) {
        Ok(lang) => lang,
        Err(_) => {
            let ext = file_path.extension().and_then(|e| e.to_str()).unwrap_or("");
            if ext == "toml" || ext == "json" {
                let old = args.old.as_deref().ok_or_else(|| anyhow::anyhow!("--old required for .{}", ext))?;
                let new = args.new.as_deref().ok_or_else(|| anyhow::anyhow!("--new required for .{}", ext))?;
                return crate::patch::full_file_replace(file_path, old, new, args.dry_run);
            }
            anyhow::bail!("Unsupported file extension .{}", ext);
        }
    };
    let _manager = FileManager::new(!args.no_backup);
    let mut options = PatchOptions {
        fuzz_radius: args.fuzz,
        dry_run: args.dry_run,
        force: args.force,
        no_backup: args.no_backup,
        similarity_threshold: 0.9,
        confidence_threshold: args.confidence,
        uniqueness_weight: args.uniqueness_weight,
        strict_whitespace: args.strict_whitespace,
        no_auto_repair: args.no_auto_repair,
        marker: args.marker.clone(),
        plugin: args.plugin.clone(),
        match_info: None,
        fix_indent: args.fix_indent,
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
            if let Some((_, content)) = crate::sanitize::extract_fenced_block(&buffer) {
                content.to_string()
            } else { buffer }
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
        let (expected, new) = super::heredoc::parse_heredoc(&buffer, args.no_strip_fence, args.no_sanitize)?;
        apply_literal_patch(file_path, line, &expected, &new, &mut options, &mut *lang)?;
    } else if let Some(symbol) = args.symbol.as_deref() {
        let new = args.new.as_deref().ok_or_else(|| anyhow::anyhow!("--new required"))?;
        apply_symbol_patch(file_path, symbol, new, &options, &mut *lang)?;
    } else if let Some(marker) = args.marker.as_deref() {
        let new = args.new.as_deref().or(args.content.as_deref()).ok_or_else(|| anyhow::anyhow!("--new or --content required"))?;
        apply_marker_patch(file_path, marker, new, options.clone())?;
    } else {
        anyhow::bail!("No patch operation specified");
    }
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
    if !args.no_provenance {
        let record = crate::provenance::ProvenanceRecord::new(
            &file_path.to_string_lossy(), "patch", args.agent.as_deref(), args.model.as_deref(),
            serde_json::json!({"syntax": true, "compile": true})
        );
        crate::provenance::emit_record(&record)?;
    }
    let mut cross_file_warnings = Vec::new();
    if args.cross_file {
        let project_index = crate::crossfile::build_project_index(Path::new("."));
        if let (Some(old), Some(new)) = (args.old.as_deref(), args.new.as_deref()) {
            if old != new {
                if let Some(old_name) = old.split("fn ").nth(1).and_then(|s| s.split('(').next()) {
                    if let Some(new_name) = new.split("fn ").nth(1).and_then(|s| s.split('(').next()) {
                        if old_name != new_name {
                            let callers = crate::crossfile::find_callers(old_name, &project_index);
                            for caller in callers {
                                cross_file_warnings.push(format!(
                                    "Function '{}' was renamed to '{}'. Caller found in {} at line {}",
                                    old_name, new_name, caller.file, caller.line
                                ));
                            }
                        }
                    }
                }
            }
        }
    }
    let language_name = file_path.extension().and_then(|e| e.to_str()).unwrap_or("");
    let missing = crate::identifier::missing_identifiers(&original, &patched, language_name);
    if !missing.is_empty() {
        eprintln!("Warning: new identifiers not found in original file: {:?}", missing);
    }
    if args.json {
        let mut diag = JsonDiagnostic::success();
        if let Some(ref info) = options.match_info {
            diag.confidence = Some(info.confidence);
            diag.strategy = Some(info.strategy.clone());
        }
        if !cross_file_warnings.is_empty() {
            diag.warnings = Some(cross_file_warnings);
        }
        println!("{}", serde_json::to_string(&diag)?);
    } else if !cross_file_warnings.is_empty() {
        eprintln!("Cross‑file warnings:");
        for warn in &cross_file_warnings {
            eprintln!("  {}", warn);
        }
    }
    Ok(())
}

pub fn handle_patch(args: PatchArgs) -> Result<()> {
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

use crate::word_diff::{word_diff, colorize_word_changes};

fn print_colored_diff(original: &str, patched: &str) {
    let old_lines: Vec<&str> = original.lines().collect();
    let new_lines: Vec<&str> = patched.lines().collect();

    for i in 0..std::cmp::max(old_lines.len(), new_lines.len()) {
        let old = old_lines.get(i).copied().unwrap_or("");
        let new = new_lines.get(i).copied().unwrap_or("");
        if old == new {
            println!("  {}", old);
        } else if old.is_empty() {
            // Inserted line
            println!("+ {}", colorize_word_changes(&word_diff("", new).1));
        } else if new.is_empty() {
            // Deleted line
            println!("- {}", colorize_word_changes(&word_diff(old, "").0));
        } else {
            // Modified line: show old removed words and new added words
            let (old_changes, new_changes) = word_diff(old, new);
            print!("~ ");
            for w in &old_changes {
                if w.change_type == crate::word_diff::ChangeType::Removed {
                    print!("\x1b[41m{}\x1b[0m", w.text);
                }
            }
            for w in &new_changes {
                if w.change_type == crate::word_diff::ChangeType::Added {
                    print!("\x1b[42m{}\x1b[0m", w.text);
                }
            }
            println!();
        }
    }
}
