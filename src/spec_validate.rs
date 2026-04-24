use anyhow::Result;
use std::path::Path;

pub fn validate_spec(spec_file: &Path, code_file: &Path) -> Result<Vec<String>> {
    let spec_content = std::fs::read_to_string(spec_file)?;
    let code_content = std::fs::read_to_string(code_file)?;
    let mut warnings = Vec::new();

    // Simple check: extract function names from spec (lines with `fn <name>` in markdown)
    for line in spec_content.lines() {
        if line.contains("fn ") && line.contains('(') {
            let func_name = line
                .split("fn ")
                .nth(1)
                .and_then(|s| s.split('(').next())
                .unwrap_or("");
            if !code_content.contains(&format!("fn {}", func_name)) {
                warnings.push(format!(
                    "Function '{}' defined in spec but not found in code",
                    func_name
                ));
            }
        }
    }
    Ok(warnings)
}
