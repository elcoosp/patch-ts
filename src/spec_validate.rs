use anyhow::Result;
use std::path::Path;

///
/// # Errors
///
/// Returns an error if validation fails.
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
            if !code_content.contains(&format!("fn {func_name}")) {
                warnings.push(format!(
                    "Function '{func_name}' defined in spec but not found in code"
                ));
            }
        }
    }
    Ok(warnings)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn spec_functions_present_in_code_produce_no_warnings() {
        let dir = tempdir().unwrap();
        let spec = dir.path().join("SPEC.md");
        let code = dir.path().join("lib.rs");
        fs::write(&spec, "- fn compute(x) — does math\n").unwrap();
        fs::write(&code, "fn compute(x: u32) -> u32 { x }\n").unwrap();
        let warnings = validate_spec(&spec, &code).unwrap();
        assert_eq!(warnings.len(), 0);
    }

    #[test]
    fn missing_functions_are_reported() {
        let dir = tempdir().unwrap();
        let spec = dir.path().join("SPEC.md");
        let code = dir.path().join("lib.rs");
        fs::write(&spec, "- fn missing_fn(x)\n- fn present_fn()\n").unwrap();
        fs::write(&code, "fn present_fn() {}\n").unwrap();
        let warnings = validate_spec(&spec, &code).unwrap();
        assert_eq!(warnings.len(), 1);
        assert!(warnings[0].contains("missing_fn"));
    }
}
