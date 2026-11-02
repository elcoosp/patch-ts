use super::types::ProvenanceQueryArgs;
use anyhow::Result;
///
/// # Errors
///
/// Returns an error if the operation fails.
///
/// # Panics
///
/// Panics if an internal `unwrap` fails (invariant violation).
pub fn handle_provenance_query(args: &ProvenanceQueryArgs) -> Result<()> {
    let records = crate::provenance::query_provenance(args.since.as_deref(), args.file.as_deref())?;
    if let Some(ref output) = args.output {
        std::fs::write(
            output,
            records
                .iter()
                .map(|r| serde_json::to_string(r).unwrap())
                .collect::<Vec<_>>()
                .join("\n"),
        )?;
    }
    if args.json {
        println!("{}", serde_json::to_string(&records)?);
    } else {
        for record in &records {
            println!(
                "{} {} {} {} {}",
                record.timestamp,
                record.tool,
                record.file,
                record.operation,
                record.agent.as_deref().unwrap_or("unknown")
            );
        }
    }
    Ok(())
}
