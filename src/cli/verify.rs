use anyhow::Result;
use super::types::*;
pub fn handle_verify(args: VerifyArgs) -> Result<()> {
    let content = std::fs::read_to_string(&args.file)?;
    let invariants = crate::invariant::extract_invariants(&content);
    let violations = crate::invariant::verify_invariants(&content, &content);
    if args.json { println!("{}", serde_json::to_string(&serde_json::json!({"invariants": invariants, "violations": violations}))?); }
    else {
        println!("Invariants: {}", invariants.len());
        for inv in &invariants { println!("  {}: {} at line {}", inv.kind, inv.expression, inv.line); }
        if !violations.is_empty() { println!("Violations:"); for v in &violations { println!("  {} at line {}", v.invariant.expression, v.line); } }
    }
    Ok(())
}
