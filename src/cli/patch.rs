use super::types::PatchArgs;
use crate::ast::Language;
use crate::diagnostics::JsonDiagnostic;
use crate::file::FileManager;
use crate::history::HistoryManager;
use crate::patch::{
    MatchFlags, PatchOptions, RuntimeFlags, apply_literal_patch, apply_marker_patch,
    apply_symbol_patch, apply_unified_diff, delete_line, insert_lines,
};
use anyhow::Result;
use glob::glob;
use rayon::prelude::*;
use std::io::Read;
use std::path::Path;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};

///
/// # Errors
///
/// Returns an error if the glob pattern matches no files.
pub fn expand_files(pattern: &str) -> Result<Vec<PathBuf>> {
    let paths: Vec<PathBuf> = glob(pattern)?.filter_map(std::result::Result::ok).collect();
    if paths.is_empty() {
        anyhow::bail!("No files matched pattern: {pattern}");
    }
    Ok(paths)
}

///
/// # Errors
///
/// Returns an error if the operation fails.
///
/// # Panics
///
/// Panics when the documented invariant is violated.
pub fn apply_patch_to_file(file_path: &Path, args: &PatchArgs) -> Result<()> {
    let Ok(mut lang) = crate::ast::detect_language(file_path) else {
        let ext = file_path.extension().and_then(|e| e.to_str()).unwrap_or("");
        if ext == "toml" || ext == "json" {
            let old = args
                .old
                .as_deref()
                .ok_or_else(|| anyhow::anyhow!("--old required for .{ext}"))?;
            let new = args
                .new
                .as_deref()
                .ok_or_else(|| anyhow::anyhow!("--new required for .{ext}"))?;
            return crate::patch::full_file_replace(file_path, old, new, args.dry_run);
        }
        anyhow::bail!("Unsupported file extension .{ext}");
    };
    let _manager = FileManager::new(!args.no_backup);
    let mut options = PatchOptions {
        fuzz_radius: args.fuzz,
        confidence_threshold: args.confidence,
        marker: args.marker.clone(),
        plugin: args.plugin.clone(),
        run: RuntimeFlags {
            dry_run: args.dry_run,
            force: args.force,
            no_backup: args.no_backup,
        },
        match_behavior: MatchFlags {
            strict_whitespace: args.strict_whitespace,
            no_auto_repair: args.no_auto_repair,
            fix_indent: args.fix_indent,
        },
        ..Default::default()
    };
    let original = std::fs::read_to_string(file_path)?;
    perform_patch_operation(file_path, args, &mut options, &mut *lang)?;
    let patched = std::fs::read_to_string(file_path)?;
    if !args.no_compile_check {
        let lang_str = file_path.extension().and_then(|e| e.to_str()).unwrap_or("");
        let compile_result =
            crate::compile::compile_check(file_path, lang_str, args.compile_timeout)?;
        if !compile_result.success {
            // Delta semantics: a patch is only rejected when it *introduces*
            // errors. If the original file already failed to compile (e.g. a
            // code fragment), keep the patch.
            let baseline =
                crate::compile::compile_check_content(&original, lang_str, args.compile_timeout)?;
            if baseline.success {
                std::fs::write(file_path, &original)?;
                let error_msg = compile_result
                    .errors
                    .iter()
                    .map(|e| format!("{}:{}:{}: {}", e.file, e.line, e.column, e.message))
                    .collect::<Vec<_>>()
                    .join("\n");
                anyhow::bail!("Patch introduced compilation errors:\n{error_msg}");
            }
        }
    }
    let cross_file_warnings = if args.cross_file {
        collect_cross_file_warnings(args)
    } else {
        Vec::new()
    };
    let language_name = file_path.extension().and_then(|e| e.to_str()).unwrap_or("");
    let missing = crate::identifier::missing_identifiers(&original, &patched, language_name);
    if !missing.is_empty() {
        eprintln!("Warning: new identifiers not found in original file: {missing:?}");
    }
    if patched != original {
        if args.verify {
            enforce_post_verification(file_path, &original, &patched, args, &mut *lang)?;
        }
        let manager = HistoryManager::new();
        manager.save(&file_path.to_string_lossy(), &original, &patched)?;
    }
    emit_provenance_record(file_path, args)?;
    if args.preserve_comments {
        preserve_comments_after_patch(file_path, &original, args, &mut *lang);
    }
    report_patch_outcome(args, &options, cross_file_warnings)?;
    Ok(())
}

/// Dispatch the requested patch operation based on which CLI arguments are set.
///
/// # Errors
///
/// Returns an error if the operation fails or no operation was specified.
fn perform_patch_operation(
    file_path: &Path,
    args: &PatchArgs,
    options: &mut PatchOptions,
    lang: &mut dyn Language,
) -> Result<()> {
    if let Some(url) = &args.url {
        let diff_text = crate::remote::fetch_http(url)?;
        apply_fixed_diff(file_path, &diff_text, args, options)
    } else if let Some(commit) = &args.git_commit {
        let diff_text = crate::remote::fetch_git_commit(".", commit)?;
        apply_fixed_diff(file_path, &diff_text, args, options)
    } else if args.diff {
        let mut buffer = String::new();
        std::io::stdin().read_to_string(&mut buffer)?;
        let diff_text = if args.no_sanitize {
            buffer
        } else if let Some((_, content)) = crate::sanitize::extract_fenced_block(&buffer) {
            content.to_string()
        } else {
            buffer
        };
        apply_fixed_diff(file_path, &diff_text, args, options)
    } else if let Some(line) = args.delete {
        let expected = args
            .expect
            .as_deref()
            .ok_or_else(|| anyhow::anyhow!("--expect required"))?;
        delete_line(file_path, line, expected, options)
    } else if let Some(after) = args.after {
        let content = args
            .content
            .as_deref()
            .ok_or_else(|| anyhow::anyhow!("--content required"))?;
        insert_lines(file_path, after, content, options)
    } else if let (None, Some(old), Some(new)) =
        (args.line, args.old.as_deref(), args.new.as_deref())
    {
        patch_content_based(file_path, old, new, args)
    } else if let (Some(line), Some(old), Some(new)) =
        (args.line, args.old.as_deref(), args.new.as_deref())
    {
        apply_literal_patch(file_path, line, old, new, options, lang)
    } else if let Some(line) = args.line {
        let mut buffer = String::new();
        std::io::stdin().read_to_string(&mut buffer)?;
        // Detect SEARCH/REPLACE block input vs standard heredoc
        let (expected, new) = if buffer.trim().starts_with("<<< SEARCH") {
            super::heritage::parse_search_replace_block(&buffer)?
        } else {
            super::heredoc::parse_heredoc(&buffer, args.no_strip_fence, args.no_sanitize)?
        };
        apply_literal_patch(file_path, line, &expected, &new, options, lang)
    } else if let Some(symbol) = args.symbol.as_deref() {
        let new = args
            .new
            .as_deref()
            .ok_or_else(|| anyhow::anyhow!("--new required"))?;
        if args.entity_body {
            crate::patch::apply_symbol_body_patch(file_path, symbol, new, options, lang)
        } else {
            apply_symbol_patch(file_path, symbol, new, options, lang)
        }
    } else if let Some(marker) = args.marker.as_deref() {
        let new = args
            .new
            .as_deref()
            .or(args.content.as_deref())
            .ok_or_else(|| anyhow::anyhow!("--new or --content required"))?;
        apply_marker_patch(file_path, marker, new, options)
    } else {
        anyhow::bail!("No patch operation specified");
    }
}

/// Fetch-independent diff application: fixes hunk headers when requested and
/// applies the unified diff.
///
/// # Errors
///
/// Returns an error if the diff cannot be applied to the file.
fn apply_fixed_diff(
    file_path: &Path,
    diff_text: &str,
    args: &PatchArgs,
    options: &PatchOptions,
) -> Result<()> {
    let diff_text = if args.fix_headers {
        let original = std::fs::read_to_string(file_path).unwrap_or_default();
        crate::patch::hunk_fix::fix_hunk_headers(diff_text, &original, args.fuzz)
            .unwrap_or_else(|_| diff_text.to_string())
    } else {
        diff_text.to_string()
    };
    apply_unified_diff(file_path, &diff_text, options)
}

/// Content-based replacement without a line hint: finds `old` anywhere in the
/// file (exact first, then relaxed) and replaces it with `new`.
///
/// # Errors
///
/// Returns an error if the match cannot be found or the patch would introduce
/// a syntax error.
fn patch_content_based(file_path: &Path, old: &str, new: &str, args: &PatchArgs) -> Result<()> {
    let file_content = std::fs::read_to_string(file_path)?;
    let (byte_range, relaxed) = match crate::matching_flex::find_content_block(&file_content, old) {
        Some(r) => (r, false),
        None => match crate::matching_flex::find_content_block_relaxed(&file_content, old) {
            Some(r) => (r, true),
            None => anyhow::bail!(
                "Could not find a match for the provided old content. \
                     Use a more unique search block or try --symbol."
            ),
        },
    };
    if relaxed {
        eprintln!("Warning: exact match not found; used relaxed matching.");
    }
    let mut patched = file_content.clone();
    crate::patch::replace_byte_range(&mut patched, byte_range, new);
    if !args.dry_run {
        std::fs::write(file_path, &patched)?;
    }
    if !args.force {
        let mut lang = crate::ast::detect_language(file_path)?;
        let parse_result = lang.parse(&patched);
        if !lang.is_valid(&parse_result) {
            std::fs::write(file_path, &file_content)?;
            anyhow::bail!(
                "Content\u{2011}based patch introduced syntax error. \
                 Use --force to bypass or correct the replacement content."
            );
        }
    }
    Ok(())
}

/// Post-patch verification: rolls the patch back when any verification stage
/// fails, reporting the recall id for later learning.
///
/// # Errors
///
/// Returns an error if verification fails or the rollback cannot be written.
///
/// # Panics
///
/// Panics if an internal `unwrap` fails (invariant violation).
fn enforce_post_verification(
    file_path: &Path,
    original: &str,
    patched: &str,
    args: &PatchArgs,
    lang: &mut dyn Language,
) -> Result<()> {
    let lang_str = file_path.extension().and_then(|e| e.to_str()).unwrap_or("");
    let verification = crate::verify::verify_patch(
        file_path,
        patched,
        original,
        lang_str,
        args.compile_timeout,
        args.verify_test.as_deref(),
    )
    .unwrap_or(crate::verify::VerificationResult {
        syntax: None,
        lsp: None,
        compile: None,
        tests: None,
        all_passed: false,
    });
    if verification.all_passed {
        return Ok(());
    }
    std::fs::write(file_path, original)?;
    let mut query = crate::recall::RecallQuery::new(
        file_path,
        args.line.unwrap_or(1),
        args.old.as_deref().unwrap_or(""),
        args.new.as_deref().unwrap_or(""),
        "E006",
    );
    query.error_message = Some("Verification failed");
    let recall_ctx = crate::recall::generate_recall_context(&query, lang)
        .unwrap_or_else(|e| panic!("Failed to generate recall: {e}"));
    if args.json {
        let output = serde_json::json!({
            "patch_applied": false,
            "verification": verification,
            "recall_id": recall_ctx.recall_id
        });
        println!("{}", serde_json::to_string_pretty(&output)?);
    } else {
        eprintln!("Patch verification failed. Rollback applied.");
        eprintln!("Recall ID: {}", recall_ctx.recall_id);
    }
    anyhow::bail!("Post‑patch verification failed")
}

/// Record a provenance entry for the applied patch unless suppressed.
///
/// # Errors
///
/// Returns an error if the record cannot be written.
fn emit_provenance_record(file_path: &Path, args: &PatchArgs) -> Result<()> {
    if args.no_provenance {
        return Ok(());
    }
    let record = crate::provenance::ProvenanceRecord::new(
        &file_path.to_string_lossy(),
        "patch",
        args.agent.as_deref(),
        args.model.as_deref(),
        serde_json::json!({"syntax": true, "compile": true}),
    );
    crate::provenance::emit_record(&record)
}

/// Re-inject comments that the patch dropped from the file.
fn preserve_comments_after_patch(
    file_path: &Path,
    original: &str,
    args: &PatchArgs,
    lang: &mut dyn Language,
) {
    let Ok(current) = std::fs::read_to_string(file_path) else {
        return;
    };
    match crate::comment_preserve::preserve_comments(original, &current, lang) {
        Ok(preserved) => {
            if !args.dry_run
                && let Err(e) = std::fs::write(file_path, &preserved)
            {
                eprintln!("Warning: failed to write preserved comments: {e}");
            }
        }
        Err(e) => eprintln!("Warning: failed to preserve comments: {e}"),
    }
}

/// Detect call sites of functions renamed by the patch.
fn collect_cross_file_warnings(args: &PatchArgs) -> Vec<String> {
    let Some(old) = args.old.as_deref() else {
        return Vec::new();
    };
    let Some(new) = args.new.as_deref() else {
        return Vec::new();
    };
    if old == new {
        return Vec::new();
    }
    let Some(old_name) = old.split("fn ").nth(1).and_then(|s| s.split('(').next()) else {
        return Vec::new();
    };
    let Some(new_name) = new.split("fn ").nth(1).and_then(|s| s.split('(').next()) else {
        return Vec::new();
    };
    if old_name == new_name {
        return Vec::new();
    }
    let project_index = crate::crossfile::build_project_index(Path::new("."));
    crate::crossfile::find_callers(old_name, &project_index)
        .into_iter()
        .map(|caller| {
            format!(
                "Function '{old_name}' was renamed to '{new_name}'. Caller found in {} at line {}",
                caller.file, caller.line
            )
        })
        .collect()
}

/// Emit the patch outcome as JSON diagnostics or as stderr warnings.
///
/// # Errors
///
/// Returns an error if the JSON report cannot be serialized.
fn report_patch_outcome(
    args: &PatchArgs,
    options: &PatchOptions,
    cross_file_warnings: Vec<String>,
) -> Result<()> {
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
            eprintln!("  {warn}");
        }
    }
    Ok(())
}

/// Apply one or more patches as requested by the CLI arguments.
///
/// # Errors
///
/// Returns an error if the operation fails.
pub fn handle_patch(args: &PatchArgs) -> Result<()> {
    if let Some(pattern) = &args.files {
        let paths = expand_files(pattern)?;
        let any_success = AtomicBool::new(false);
        let process = |path: &PathBuf| -> Result<(), anyhow::Error> {
            match apply_patch_to_file(path, args) {
                Ok(()) => {
                    any_success.store(true, Ordering::Relaxed);
                    Ok(())
                }
                Err(e) => {
                    let msg = e.to_string();
                    if msg.contains("no match found") || msg.contains("ambiguous match") {
                        eprintln!("Skipping {}: {}", path.display(), msg);
                        Ok(())
                    } else {
                        Err(e)
                    }
                }
            }
        };
        if args.serial {
            for path in &paths {
                process(path)?;
            }
        } else {
            paths.par_iter().try_for_each(process)?;
        }
        if !any_success.load(Ordering::Relaxed) {
            anyhow::bail!("No files were successfully patched");
        }
    } else {
        let Some(file) = args.file.as_deref() else {
            anyhow::bail!("--file is required when --files is not given");
        };
        apply_patch_to_file(Path::new(file), args)?;
    }
    Ok(())
}
