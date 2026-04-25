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
pub fn load_policy(path: &Path) -> Result<Policy> {
    let content = fs::read_to_string(path)
        .with_context(|| format!("Failed to read policy file: {}", path.display()))?;
    let policy: Policy = toml::from_str(&content)
        .with_context(|| format!("Failed to parse policy file: {}", path.display()))?;
    Ok(policy)
}

/// Validate a tool call against the policy.
pub fn validate_tool_call(
    tool_name: &str,
    file_path: Option<&str>,
    patch_size: usize,
    policy: &Policy,
) -> Result<Option<String>> {
    // Check allowed tools
    if let Some(allowed) = &policy.policy.allowed_tools {
        if !allowed.iter().any(|t| t == tool_name) {
            return Ok(Some(format!("Tool '{}' is not allowed by policy", tool_name)));
        }
    }

    // Check blocked patterns
    if let Some(blocked) = &policy.policy.blocked_patterns {
        for pattern in blocked {
            if tool_name.contains(pattern) {
                return Ok(Some(format!("Tool '{}' matches blocked pattern '{}'", tool_name, pattern)));
            }
        }
    }

    // Check file path
    if let Some(file) = file_path {
        if let Some(allowed_files) = &policy.policy.allowed_files {
            let is_allowed = allowed_files.iter().any(|glob| {
                let regex_pattern = glob_to_regex(glob);
                regex::Regex::new(&regex_pattern).map(|re| re.is_match(file)).unwrap_or(false)
            });
            if !is_allowed {
                return Ok(Some(format!("File '{}' is not in allowed paths", file)));
            }
        }
    }

    // Check max patch size
    if let Some(max_size) = policy.policy.max_patch_size {
        if patch_size > max_size {
            return Ok(Some(format!(
                "Patch size {} exceeds maximum allowed {} bytes",
                patch_size, max_size
            )));
        }
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
