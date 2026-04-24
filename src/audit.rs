use serde::Serialize;
use regex::Regex;

#[derive(Debug, Clone, Serialize)]
pub struct AuditFinding {
    pub severity: String,
    pub pattern: String,
    pub line: usize,
    pub description: String,
}

pub fn audit_patch(patch_content: &str) -> Vec<AuditFinding> {
    let mut findings = Vec::new();
    let patterns = vec![
        (Regex::new(r"(?i)SELECT\s+.*\s+WHERE\s+.*=.*\+").unwrap(), "SQL injection", "critical"),
        (Regex::new(r"os\.system\s*\(").unwrap(), "Command injection", "critical"),
        (Regex::new(r"\beval\s*\(").unwrap(), "Code injection", "critical"),
        (Regex::new(r"subprocess\.call\s*\(").unwrap(), "Command injection", "high"),
        (Regex::new(r"\.\./").unwrap(), "Path traversal", "high"),
        (Regex::new(r"requests\.post\s*\(.*read\s*\(").unwrap(), "Data exfiltration", "medium"),
    ];
    for (line_idx, line) in patch_content.lines().enumerate() {
        for (re, name, severity) in &patterns {
            if re.is_match(line) {
                findings.push(AuditFinding {
                    severity: severity.to_string(),
                    pattern: name.to_string(),
                    line: line_idx + 1,
                    description: format!("{} pattern detected", name),
                });
            }
        }
    }
    findings
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_sql_injection() {
        let patch = "SELECT * FROM users WHERE name = 'test' + input";
        let findings = audit_patch(patch);
        assert!(findings.iter().any(|f| f.pattern == "SQL injection"));
    }

    #[test]
    fn test_detect_command_injection() {
        let patch = "os.system(user_input)";
        let findings = audit_patch(patch);
        assert!(findings.iter().any(|f| f.pattern == "Command injection"));
    }

    #[test]
    fn test_clean_code_passes() {
        let patch = "fn main() { println!(\"hello\"); }";
        let findings = audit_patch(patch);
        assert!(findings.is_empty());
    }
}
