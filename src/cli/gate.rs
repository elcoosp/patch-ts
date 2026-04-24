use anyhow::Result;
use std::path::Path;
use super::types::*;
pub fn handle_gate(args: GateArgs) -> Result<()> {
    let file_path = Path::new(&args.file);
    let content = std::fs::read_to_string(file_path)?;
    let stages: Vec<String> = args.stages.split(',').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect();
    let result = if args.parallel { crate::gate::run_gate_parallel(&stages, &file_path, &content, &content, args.compile_timeout)? }
    else { crate::gate::run_gate(&stages, &file_path, &content, &content, args.compile_timeout)? };
    if args.json { println!("{}", serde_json::to_string(&result)?); }
    else { for stage in &result.stages { let status = if stage.passed { "✅" } else { "❌" }; println!("{} {} – {}", status, stage.name, stage.details); } if !result.passed { std::process::exit(1); } }
    Ok(())
}
