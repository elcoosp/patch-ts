use super::types::AttestArgs;
use anyhow::Result;
///
/// # Errors
///
/// Returns an error if the cryptographic operation fails.
pub fn handle_attest(args: &AttestArgs) -> Result<()> {
    if args.cra_report {
        let report = crate::attest::generate_cra_report(
            &args.since,
            args.output.as_deref().map(std::path::Path::new),
        )?;
        if args.json {
            println!("{}", serde_json::to_string(&report)?);
        } else {
            println!("CRA attestation report generated: {}", report["report_id"]);
        }
    } else {
        let report = crate::attest::generate_attestation(
            &args.since,
            args.output.as_deref().map(std::path::Path::new),
        )?;
        if args.json {
            println!("{}", serde_json::to_string(&report)?);
        } else {
            println!("Attestation report generated: {}", report.report_id);
        }
    }
    Ok(())
}
