use regex::Regex;

use crate::ast::Language;

#[derive(Debug, Clone, serde::Serialize)]
pub struct CompilerError {
    pub error_code: String,
    pub file: String,
    pub line: usize,
    pub column: usize,
    pub message: String,
    pub language: String,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct PatchSuggestion {
    pub old: String,
    pub new: String,
    pub file: String,
    pub line: usize,
    pub confidence: f64,
    pub explanation: String,
}

/// Parse a compiler error from output text.
pub fn parse_compiler_error(output: &str) -> Option<CompilerError> {
    // Rust: error[E0308]: src/main.rs:10:5
    let rust_re = Regex::new(r"error\[(.*?)\]: (.*):(\d+):(\d+)").unwrap();
    if let Some(caps) = rust_re.captures(output) {
        return Some(CompilerError {
            error_code: caps[1].to_string(),
            file: caps[2].to_string(),
            line: caps[3].parse().unwrap_or(0),
            column: caps[4].parse().unwrap_or(0),
            message: output.lines().next().unwrap_or("").to_string(),
            language: "rs".to_string(),
        });
    }
    // TypeScript: src/app.ts(10,5): error TS2322
    let ts_re = Regex::new(r"(.+)\((\d+),(\d+)\):\s+error\s+(\w+): (.+)").unwrap();
    if let Some(caps) = ts_re.captures(output) {
        return Some(CompilerError {
            error_code: caps[4].to_string(),
            file: caps[1].to_string(),
            line: caps[2].parse().unwrap_or(0),
            column: caps[3].parse().unwrap_or(0),
            message: format!("{}: {}", &caps[4], &caps[5]),
            language: "ts".to_string(),
        });
    }
    // Python (before generic JS) – SyntaxError with line number
    let py_re = Regex::new(r"line (\d+)").unwrap();
    if let Some(caps) = py_re.captures(output) {
        return Some(CompilerError {
            error_code: "SyntaxError".to_string(),
            file: "".to_string(),
            line: caps[1].parse().unwrap_or(0),
            column: 0,
            message: output.lines().next().unwrap_or("").to_string(),
            language: "py".to_string(),
        });
    }
    // JavaScript: SyntaxError: Unexpected token '...'
    let js_re = Regex::new(r"SyntaxError: Unexpected token").unwrap();
    if js_re.is_match(output) {
        return Some(CompilerError {
            error_code: "SyntaxError".to_string(),
            file: "".to_string(),
            line: 0,
            column: 0,
            message: output.lines().next().unwrap_or("").to_string(),
            language: "js".to_string(),
        });
    }
    None
}

/// Suggest a fix for a given compiler error by looking at syntax issues.
pub fn suggest_fix(error: &CompilerError, file_content: &str, lang: &mut dyn Language) -> Option<PatchSuggestion> {
    if let Some(fixed) = crate::repair::quick_balance(file_content, lang) {
        let original_lines: Vec<&str> = file_content.lines().collect();
        let fixed_lines: Vec<&str> = fixed.lines().collect();
        for (i, (orig, fix)) in original_lines.iter().zip(&fixed_lines).enumerate() {
            if orig != fix {
                return Some(PatchSuggestion {
                    old: orig.to_string(),
                    new: fix.to_string(),
                    file: error.file.clone(),
                    line: i + 1,
                    confidence: 0.85,
                    explanation: "Auto‑repair applied to fix syntax error.".to_string(),
                });
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_rust_error() {
        let output = "error[E0308]: src/main.rs:10:5: mismatched types";
        let err = parse_compiler_error(output).unwrap();
        assert_eq!(err.error_code, "E0308");
        assert_eq!(err.file, "src/main.rs");
        assert_eq!(err.line, 10);
        assert_eq!(err.column, 5);
    }

    #[test]
    fn test_parse_ts_error() {
        let output = "src/app.ts(10,5): error TS2322: Type 'string' is not assignable to type 'number'.";
        let err = parse_compiler_error(output).unwrap();
        assert_eq!(err.error_code, "TS2322");
        assert_eq!(err.line, 10);
    }

    #[test]
    fn test_parse_python_error() {
        let output = "  File \"script.py\", line 10\n    print(\"hello\"\nSyntaxError: invalid syntax";
        let err = parse_compiler_error(output).unwrap();
        assert_eq!(err.error_code, "SyntaxError");
        assert_eq!(err.line, 10);
    }

    #[test]
    fn test_parse_js_error() {
        let output = "SyntaxError: Unexpected token '}'";
        let err = parse_compiler_error(output).unwrap();
        assert_eq!(err.error_code, "SyntaxError");
        assert_eq!(err.language, "js");
    }

    #[test]
    fn test_unrecognized_error() {
        let output = "Something went wrong but I don't know what.";
        assert!(parse_compiler_error(output).is_none());
    }
}
