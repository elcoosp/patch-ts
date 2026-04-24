use anyhow::Result;
use super::types::*;
pub fn handle_impact(args: ImpactArgs) -> Result<()> {
    let graph = crate::crossfile::build_project_call_graph(std::path::Path::new("."));
    let edges = if args.recursive {
        graph.all_callers_recursive(&args.symbol)
    } else {
        graph.callers_of(&args.symbol)
    };
    if args.json {
        let vec: Vec<_> = edges.iter().map(|e| serde_json::json!({
            "file": e.caller_file,
            "line": e.caller_line,
            "column": e.caller_column
        })).collect();
        println!("{}", serde_json::to_string(&vec)?);
    } else {
        if edges.is_empty() {
            println!("No callers found for '{}'", args.symbol);
        } else {
            for edge in edges {
                println!("{}:{}:{}", edge.caller_file, edge.caller_line, edge.caller_column);
            }
        }
    }
    Ok(())
}
