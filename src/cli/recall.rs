use anyhow::Result;
use super::types::RecallArgs;
use crate::recall::generate_recall_context;

pub fn handle_recall(args: RecallArgs) -> Result<()> {
    let file_path = std::path::Path::new(&args.file);
    let mut lang = crate::ast::detect_language(file_path)?;
    let context = generate_recall_context(
        file_path,
        args.line,
        &args.old,
        &args.new,
        &args.error_code,
        args.error_message.as_deref(),
        args.context_lines,
        &mut *lang,
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
        println!("### Surrounding context ({} lines around target):", args.context_lines);
        for line in &context.context.surrounding_lines {
            println!("      {}", line);
        }
        if let Some(ref symbol) = context.context.symbol {
            println!("\n### Containing symbol: `{}`", symbol);
        }
        println!("\n### Suggested approach:");
        for strategy in &context.strategies {
            println!("  - **{}**: {}", strategy.name, strategy.description);
        }
        println!("\nGenerate ONLY the corrected patch. Do not repeat the file content.");
    } else {
        println!("Recall id: {}", context.recall_id);
        println!("File: {}", args.file);
        println!("Error: {} - {}", context.error.code, context.error.message);
        println!("Suggested strategies:");
        for s in &context.strategies {
            println!("  - {}: {}", s.name, s.description);
        }
    }

    Ok(())
}
