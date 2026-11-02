use super::types::RecallArgs;
use crate::recall::{build_minimal_context, generate_recall_context_with_session};
use anyhow::Result;

///
/// # Errors
///
/// Returns an error if the file or record cannot be read.
pub fn handle_recall(args: &RecallArgs) -> Result<()> {
    let file_path = std::path::Path::new(&args.file);

    if args.minimal {
        let file_content = std::fs::read_to_string(file_path)?;
        let ctx = build_minimal_context(
            file_path,
            args.line,
            &args.error_code,
            args.error_message.as_deref(),
            &file_content,
        );
        if args.json {
            println!("{}", serde_json::to_string_pretty(&ctx)?);
        } else {
            println!(
                "Error: {} - {}\nLine: {}\nBest strategy: {}",
                ctx.error, ctx.message, ctx.line, ctx.best_strategy
            );
        }
        return Ok(());
    }

    let mut lang = crate::ast::detect_language(file_path)?;
    let query = crate::recall::RecallQuery {
        file_path,
        line: args.line,
        old_content: &args.old,
        new_content: &args.new,
        error_code: &args.error_code,
        error_message: args.error_message.as_deref(),
        context_lines: args.context_lines,
        use_entropy: args.toggles.entropy,
        entropy_threshold: args.entropy_threshold,
        pre_fetch: args.toggles.pre_fetch,
        session_id: args.session.as_deref(),
        max_tokens: args.max_tokens,
    };
    let context = generate_recall_context_with_session(&query, &mut *lang)?;

    if args.json {
        println!("{}", serde_json::to_string_pretty(&context)?);
    } else if args.toggles.prompt {
        println!("## Patch Retry Context\n");
        println!("Your previous patch to `{}` failed.\n", args.file);
        println!("### What you attempted:");
        println!("  Line: {}", args.line);
        println!("  Old: `{}`", args.old);
        println!("  New: `{}`", args.new);
        println!();
        println!("### What went wrong:");
        println!(
            "  Error {} ({}):",
            context.error.code, context.error.category
        );
        println!("  \"{}\"", context.error.message);
        println!();
        println!("### Surrounding context:");
        for line in &context.context.surrounding_lines {
            println!("      {line}");
        }
        if let Some(ref symbol) = context.context.symbol {
            println!("\n### Containing symbol: `{symbol}`");
        }
        if let Some(ref body) = context.context.containing_body {
            println!("\n### Function body:\n{body}");
        }
        println!("\n### Suggested approach:");
        for s in &context.strategies {
            println!("  - **{}**: {}", s.name, s.description);
        }
        println!("\nGenerate ONLY the corrected patch. Do not repeat the file content.");
    } else {
        println!("Recall id: {}", context.recall_id);
        println!("File: {}", args.file);
        println!("Error: {} - {}", context.error.code, context.error.message);
        if let Some(best) = crate::recall::get_best_strategy(&args.error_code) {
            println!("Best historical strategy for {}: {}", args.error_code, best);
        }
        println!("Suggested strategies:");
        for s in &context.strategies {
            println!("  - {}: {}", s.name, s.description);
        }
    }

    Ok(())
}
