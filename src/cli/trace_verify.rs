use anyhow::Result;
use super::types::*;
pub fn handle_trace_verify(args: TraceArgs) -> Result<()> {
    let log_path = std::path::Path::new(".patch-ts/provenance.jsonl");
    let tampered = crate::scitt::verify_log(log_path, None)?;
    if args.json { println!("{}", serde_json::to_string(&tampered)?); }
    else { if tampered.is_empty() { println!("All records verified."); } else { println!("Tampered records detected:"); for t in &tampered { println!("  {}", t); } } }
    Ok(())
}
