use anyhow::Result;
use super::types::*;
pub fn handle_review(args: ReviewArgs) -> Result<()> {
    let file_path = std::path::Path::new(&args.file);
    let old = std::fs::read_to_string(file_path)?;
    let new = if args.new == "-" { old.clone() } else { args.new.clone() };
    let report = crate::review::run_review(file_path, &old, &new);
    if args.json { println!("{}", serde_json::to_string(&report)?); }
    else {
        println!("Code Review Report:");
        for finding in &report.findings {
            let icon = if finding.severity == "pass" { "✅" } else { "❌" };
            println!("  {} {} – {}", icon, finding.agent, finding.details);
        }
        println!("Action: {}", report.action);
    }
    Ok(())
}
