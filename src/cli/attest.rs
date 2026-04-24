use anyhow::Result;
use super::types::*;
pub fn handle_attest(args: AttestArgs) -> Result<()> {
    if args.cra_report {
        let report = crate::attest::generate_cra_report(&args.since, args.output.as_deref().map(|s| std::path::Path::new(s)))?;
        if args.json { println!("{}", serde_json::to_string(&report)?); }
        else { println!("CRA attestation report generated: {}", report["report_id"]); }
    } else {
        let report = crate::attest::generate_attestation(&args.since, args.output.as_deref().map(|s| std::path::Path::new(s)))?;
        if args.json { println!("{}", serde_json::to_string(&report)?); }
        else { println!("Attestation report generated: {}", report.report_id); }
    }
    Ok(())
}
