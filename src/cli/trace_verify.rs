use super::types::TraceArgs;
use anyhow::Result;
///
/// # Errors
///
/// Returns an error if the cryptographic operation fails.
pub fn handle_trace_verify(args: &TraceArgs) -> Result<()> {
    let log_path = std::path::Path::new(".patch-ts/provenance.jsonl");
    let tampered = crate::scitt::verify_log(log_path, None)?;
    if args.json {
        println!("{}", serde_json::to_string(&tampered)?);
    } else if tampered.is_empty() {
        println!("All records verified.");
    } else {
        println!("Tampered records detected:");
        for t in &tampered {
            println!("  {t}");
        }
    }
    Ok(())
}
