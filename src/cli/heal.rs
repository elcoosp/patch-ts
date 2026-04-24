use anyhow::Result;
use super::types::HealArgs;
use crate::heal::{HealOptions, Heuristic, heal_file};

pub fn handle_heal(args: HealArgs) -> Result<()> {
    let file_path = std::path::Path::new(&args.file);
    let heuristic = match args.heuristic.as_str() {
        "language-aware" => Heuristic::LanguageAware,
        "cost-weighted" => Heuristic::CostWeighted,
        "history-guided" => Heuristic::HistoryGuided,
        "balanced" => Heuristic::Balanced,
        _ => anyhow::bail!("Unknown heuristic: {}", args.heuristic),
    };

    let mut lang = crate::ast::detect_language(file_path)?;
    let options = HealOptions {
        max_cost: args.max_cost,
        heuristic,
        apply: args.apply,
        incremental: true,
    };

    let result = heal_file(file_path, &options, &mut *lang)?;

    if args.json {
        println!("{}", serde_json::to_string_pretty(&result)?);
    } else {
        if result.success {
            println!("✅ Heal successful ({} actions, cost {})", result.actions.len(), result.total_cost);
            for action in &result.actions {
                match action {
                    crate::repair::search::RepairAction::Insert { ch, pos } => {
                        println!("  Insert '{}' at byte {}", ch, pos);
                    }
                    crate::repair::search::RepairAction::Delete { start, end: _ } => {
                        println!("  Delete at byte {}", start);
                    }
                }
            }
        } else {
            println!("❌ Heal failed: could not repair within max cost {}", args.max_cost);
        }
    }

    Ok(())
}
