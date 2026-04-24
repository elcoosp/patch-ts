use anyhow::Result;
use super::types::*;
pub fn handle_index(args: IndexArgs) -> Result<()> {
    let kg = crate::knowledge::build_project_index(std::path::Path::new("."));
    if let Some(ref symbol) = args.callers {
        let callers = kg.callers_of(symbol);
        if args.json { println!("{}", serde_json::to_string(&callers)?); }
        else { if callers.is_empty() { println!("No callers found for '{}'", symbol); } else { for caller in &callers { println!("{}:{}", caller.caller_file, caller.caller_line); } } }
    } else { println!("Knowledge graph built with {} symbols and {} call edges.", kg.symbols.len(), kg.call_edges.len()); if args.json { println!("{}", serde_json::to_string(&kg)?); } }
    Ok(())
}
