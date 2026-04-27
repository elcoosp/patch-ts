use anyhow::Result;
use super::types::RecallArgs;
use crate::recall::{generate_recall_context_with_session, build_minimal_context};

pub fn handle_recall(args: RecallArgs) -> Result<()> {
    let file_path = std::path::Path::new(&args.file);

    if args.minimal {
        let file_content = std::fs::read_to_string(&file_path)?;
        let ctx = build_minimal_context(&file_path, args.line, &args.error_code, args.error_message.as_deref(), &file_content);
        if args.json { println!("{}", serde_json::to_string_pretty(&ctx)?); }
        else { println!("Error: {} - {}\nLine: {}\nBest strategy: {}", ctx.error, ctx.message, ctx.line, ctx.best_strategy); }
        return Ok(());
    }

    let mut lang = crate::ast::detect_language(file_path)?;
    let context = generate_recall_context_with_session(
        file_path,
        args.line,
        &args.old,
        &args.new,
        &args.error_code,
        args.error_message.as_deref(),
        args.context_lines,
        &mut *lang,
        args.entropy,
        args.entropy_threshold,
        args.pre_fetch,
        args.session.as_deref(),
        args.max_tokens,
    )?;

    if args.json {
        println!("{}", serde_json::to_string_pretty(&context)?);
    } else if args.prompt {
        println!("## Patch Retry Context\n");
        println!("Your previous patch to `{}` failed.\n", args.file);
        println!("### What you attempted:");
        println!("  Line: {}", args.line);
        println!("  Old: `{}`", args.old);
        println!("  New: `{}`", args.new);
        println!();
        println!("### What went wrong:");
        println!("  Error {} ({}):", context.error.code, context.error.category);
        println!("  \"{}\"", context.error.message);
        println!();
        println!("### Surrounding context:");
        for line in &context.context.surrounding_lines { println!("      {}", line); }
        if let Some(ref symbol) = context.context.symbol { println!("\n### Containing symbol: `{}`", symbol); }
        if let Some(ref body) = context.context.containing_body { println!("\n### Function body:\n{}", body); }
        println!("\n### Suggested approach:");
        for s in &context.strategies { println!("  - **{}**: {}", s.name, s.description); }
        println!("\nGenerate ONLY the corrected patch. Do not repeat the file content.");
    } else {
        println!("Recall id: {}", context.recall_id);
        println!("File: {}", args.file);
        println!("Error: {} - {}", context.error.code, context.error.message);
        if let Some(best) = crate::recall::get_best_strategy(&args.error_code) {
            println!("Best historical strategy for {}: {}", args.error_code, best);
        }
        println!("Suggested strategies:");
        for s in &context.strategies { println!("  - {}: {}", s.name, s.description); }
    }

    Ok(())
}
