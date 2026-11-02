use anyhow::{Context, Result};
use serde::Deserialize;
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize)]
pub struct Policy {
    pub policy: PolicySection,
}

#[derive(Debug, Deserialize)]
pub struct PolicySection {
    pub max_patch_size: Option<usize>,
    pub allowed_tools: Option<Vec<String>>,
    pub allowed_files: Option<Vec<String>>,
    pub blocked_patterns: Option<Vec<String>>,
    pub owasp: Option<OWASPSection>,
}

#[derive(Debug, Deserialize)]
pub struct OWASPSection {
    pub tool_poisoning_check: Option<bool>,
    pub prompt_injection_check: Option<bool>,
    pub supply_chain_check: Option<bool>,
}

/// Load a policy from a TOML file.
///
/// # Errors
///
/// Returns an error if the file or record cannot be read.
pub fn load_policy(path: &Path) -> Result<Policy> {
    let content = fs::read_to_string(path)
        .with_context(|| format!("Failed to read policy file: {}", path.display()))?;
    let policy: Policy = toml::from_str(&content)
        .with_context(|| format!("Failed to parse policy file: {}", path.display()))?;
    Ok(policy)
}

/// Validate a tool call against the policy.
///
/// # Errors
///
/// Returns an error if validation fails.
pub fn validate_tool_call(
    tool_name: &str,
    file_path: Option<&str>,
    patch_size: usize,
    policy: &Policy,
) -> Result<Option<String>> {
    // Check allowed tools
    if let Some(allowed) = &policy.policy.allowed_tools
        && !allowed.iter().any(|t| t == tool_name)
    {
        return Ok(Some(format!("Tool '{tool_name}' is not allowed by policy")));
    }

    // Check blocked patterns
    if let Some(blocked) = &policy.policy.blocked_patterns {
        for pattern in blocked {
            if tool_name.contains(pattern) {
                return Ok(Some(format!(
                    "Tool '{tool_name}' matches blocked pattern '{pattern}'"
                )));
            }
        }
    }

    // Check file path
    if let Some(file) = file_path
        && let Some(allowed_files) = &policy.policy.allowed_files
    {
        let is_allowed = allowed_files.iter().any(|glob| {
            let regex_pattern = glob_to_regex(glob);
            regex::Regex::new(&regex_pattern).is_ok_and(|re| re.is_match(file))
        });
        if !is_allowed {
            return Ok(Some(format!("File '{file}' is not in allowed paths")));
        }
    }

    // Check max patch size
    if let Some(max_size) = policy.policy.max_patch_size
        && patch_size > max_size
    {
        return Ok(Some(format!(
            "Patch size {patch_size} exceeds maximum allowed {max_size} bytes"
        )));
    }

    Ok(None) // No violation
}

/// Simple glob to regex conversion.
fn glob_to_regex(glob: &str) -> String {
    let mut re = String::from("^");
    for ch in glob.chars() {
        match ch {
            '*' => re.push_str(".*"),
            '?' => re.push('.'),
            '.' | '+' | '(' | ')' | '|' | '^' | '$' | '{' | '}' | '[' | ']' | '\\' => {
                re.push('\\');
                re.push(ch);
            }
            _ => re.push(ch),
        }
    }
    re.push('$');
    re
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    fn write_policy(dir: &std::path::Path, extra: &str) -> std::path::PathBuf {
        let path = dir.join("policy.toml");
        fs::write(&path, extra).unwrap();
        path
    }

    #[test]
    fn load_policy_parses_toml() {
        let dir = tempdir().unwrap();
        let path = write_policy(
            dir.path(),
            "[policy]\nallowed_tools = [\"patch\"]\nmax_patch_size = 100\n",
        );
        let policy = load_policy(&path).unwrap();
        assert_eq!(
            policy.policy.allowed_tools.as_deref(),
            Some(&["patch".to_string()][..])
        );
        assert_eq!(policy.policy.max_patch_size, Some(100));
    }

    #[test]
    fn load_policy_missing_file_errors() {
        let dir = tempdir().unwrap();
        assert!(load_policy(&dir.path().join("nope.toml")).is_err());
    }

    #[test]
    fn validate_tool_call_allows_permitted_tool() {
        let dir = tempdir().unwrap();
        let path = write_policy(dir.path(), "[policy]\nallowed_tools = [\"patch\"]\n");
        let policy = load_policy(&path).unwrap();
        assert_eq!(
            validate_tool_call("patch", None, 10, &policy).unwrap(),
            None
        );
        let denied = validate_tool_call("delete-all", None, 10, &policy).unwrap();
        assert!(denied.is_some());
    }

    #[test]
    fn validate_tool_call_blocks_patterns() {
        let dir = tempdir().unwrap();
        let path = write_policy(dir.path(), "[policy]\nblocked_patterns = [\"danger\"]\n");
        let policy = load_policy(&path).unwrap();
        let violation = validate_tool_call("very-danger-tool", None, 0, &policy).unwrap();
        assert!(violation.unwrap().contains("blocked pattern"));
    }

    #[test]
    fn validate_tool_call_enforces_allowed_files_glob() {
        let dir = tempdir().unwrap();
        let path = write_policy(dir.path(), "[policy]\nallowed_files = [\"src/**\"]\n");
        let policy = load_policy(&path).unwrap();
        assert_eq!(
            validate_tool_call("patch", Some("src/main.rs"), 0, &policy).unwrap(),
            None
        );
        let violation = validate_tool_call("patch", Some("/etc/passwd"), 0, &policy).unwrap();
        assert!(violation.unwrap().contains("allowed paths"));
    }

    #[test]
    fn validate_tool_call_enforces_max_patch_size() {
        let dir = tempdir().unwrap();
        let path = write_policy(dir.path(), "[policy]\nmax_patch_size = 10\n");
        let policy = load_policy(&path).unwrap();
        let violation = validate_tool_call("patch", None, 500, &policy).unwrap();
        assert!(violation.unwrap().contains("exceeds maximum"));
        assert_eq!(validate_tool_call("patch", None, 5, &policy).unwrap(), None);
    }
}
