use anyhow::{bail, Result};

/// Check that a string does not contain control characters (except \n and \t).
pub fn validate_string(value: &str, field_name: &str) -> Result<()> {
    for (i, ch) in value.char_indices() {
        if ch.is_control() && ch != '\n' && ch != '\t' {
            bail!(
                "Control character 0x{:02x} found in argument '{}' at position {}",
                ch as u32,
                field_name,
                i
            );
        }
    }
    Ok(())
}

/// Check a file path for path traversal patterns.
pub fn validate_path(path: &str, allow_all: bool) -> Result<()> {
    if allow_all {
        return Ok(());
    }
    if path.contains("..") {
        bail!("Path traversal detected: {}", path);
    }
    Ok(())
}
