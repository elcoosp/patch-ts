use regex::Regex;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Invariant {
    pub kind: String,
    pub expression: String,
    pub line: usize,
}

#[derive(Debug, Clone, Serialize)]
pub struct Violation {
    pub invariant: Invariant,
    pub line: usize,
    pub expression: String,
}

/// Extract invariants from source code via specially‑formatted comments.
pub fn extract_invariants(source: &str) -> Vec<Invariant> {
    let mut invariants = Vec::new();
    let re = Regex::new(r"//@\s*(invariant|requires|ensures)\s+(.+)").unwrap();
    for (i, line) in source.lines().enumerate() {
        if let Some(caps) = re.captures(line) {
            invariants.push(Invariant {
                kind: caps[1].to_string(),
                expression: caps[2].to_string(),
                line: i + 1,
            });
        }
    }
    invariants
}

/// Verify that invariants still hold in the new code.
pub fn verify_invariants(original: &str, patched: &str) -> Vec<Violation> {
    let invariants = extract_invariants(original);
    let mut violations = Vec::new();
    for inv in &invariants {
        // Simple check: see if the invariant expression still appears in the patched code
        if !patched.contains(&inv.expression) {
            violations.push(Violation {
                invariant: inv.clone(),
                line: inv.line,
                expression: inv.expression.clone(),
            });
        }
    }
    violations
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_invariants() {
        let source = "fn main() {\n    //@ invariant x > 0\n    let x = 1;\n}";
        let invariants = extract_invariants(source);
        assert_eq!(invariants.len(), 1);
        assert_eq!(invariants[0].kind, "invariant");
        assert_eq!(invariants[0].expression, "x > 0");
    }

    #[test]
    fn test_verify_no_violation() {
        let original = "//@ invariant x > 0\nlet x = 1;";
        let patched = "//@ invariant x > 0\nlet x = 2;";
        let violations = verify_invariants(original, patched);
        assert!(violations.is_empty());
    }

    #[test]
    fn test_verify_violation() {
        let original = "//@ invariant x > 0\nlet x = 1;";
        let patched = "let x = -1;";
        let violations = verify_invariants(original, patched);
        assert!(!violations.is_empty());
    }
}
