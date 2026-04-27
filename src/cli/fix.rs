use std::io::Read;
use std::path::Path;
use super::types::*;
pub fn handle_fix(args: FixArgs) -> anyhow::Result<()> {
    use std::fs;
    let error_text = if let Some(ref path) = args.error_file { fs::read_to_string(path)? }
    else { let mut buf = String::new(); std::io::stdin().read_to_string(&mut buf)?; buf };
    let error = crate::fix::parse_compiler_error(&error_text).ok_or_else(|| anyhow::anyhow!("No fix pattern recognized"))?;
    let target_file = args.file.as_deref().unwrap_or(&error.file);
    let content = if Path::new(target_file).exists() { fs::read_to_string(target_file)? }
    else { anyhow::bail!("File '{}' not found", target_file) };
    let mut lang = crate::ast::detect_language(Path::new(target_file))?;
    let suggestion = crate::fix::suggest_fix(&error, &content, &mut *lang)
        .ok_or_else(|| anyhow::anyhow!("Could not auto‑suggest fix"))?;
    if args.json { println!("{}", serde_json::to_string(&suggestion)?); }
    else {
        println!("Suggested fix for {}:{}:{}", suggestion.file, suggestion.line, error.error_code);
        println!("  Replace: `{}`", suggestion.old);
        println!("  With:    `{}`", suggestion.new);
    }
    if args.apply {
        let patch_args = PatchArgs {
            file: Some(target_file.to_string()), files: None, line: Some(suggestion.line),
            fuzz: 5, old: Some(suggestion.old), new: Some(suggestion.new), confidence: 0.9,
            fix_indent: false, diff: false, delete: None, expect: None, after: None, content: None,
            dry_run: false, force: args.force, no_backup: false, json: false, no_auto_repair: false,
            marker: None, serial: false, plugin: None, allow_all_paths: false,
            symbol: None, url: None, git_commit: None, no_strip_fence: false,
            no_compile_check: false, compile_timeout: 30, no_sanitize: false, no_ellipsis: false,
            uniqueness_weight: 0.2, strict_whitespace: false, cross_file: false, agent: None, model: None,
            no_provenance: true,
            fix_headers: false,
            verify: false,
            verify_test: None,
            validate_first: false,
            entity_body: false,
        };
        super::patch::apply_patch_to_file(Path::new(target_file), &patch_args)?;
        println!("Fix applied to {}", target_file);
    }
    Ok(())
}
